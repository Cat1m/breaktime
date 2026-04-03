#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod features;

use tauri::{Listener, Manager};
use crate::core::state::create_app_state;
use crate::core::l10n::t;
use crate::features::settings::service::load_settings;

fn main() {
    env_logger::init();

    let settings = load_settings().unwrap_or_default();
    let app_state = create_app_state(settings);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("settings") {
                window.show().ok();
                window.set_focus().ok();
            }
        }))
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            features::settings::commands::get_settings,
            features::settings::commands::save_settings,
            features::audio::commands::play_sound,
            features::audio::commands::set_volume,
            features::timer::commands::pause_timer,
            features::timer::commands::resume_timer,
            features::timer::commands::skip_break,
            features::timer::commands::get_timer_status,
            features::timer::commands::get_active_break,
            features::image_loader::commands::load_image,
            features::image_loader::commands::get_default_bg,
            features::dnd::commands::is_dnd_active,
        ])
        .setup(move |app| {
            println!("[Sipping] App setup starting...");

            let initial_lang = {
                let s = app_state.blocking_lock();
                s.settings.language.clone()
            };

            // Setup system tray
            if let Err(e) = setup_tray(app, &initial_lang) {
                eprintln!("[Sipping] Failed to setup tray: {}", e);
            } else {
                println!("[Sipping] Tray icon created successfully");
            }

            // Listen for settings changes → rebuild tray menu
            let app_handle = app.handle().clone();
            let state_for_listener = app_state.clone();
            app.listen("settings:changed", move |_| {
                let app = app_handle.clone();
                let state = state_for_listener.clone();
                tauri::async_runtime::spawn(async move {
                    let s = state.lock().await;
                    let lang = &s.settings.language;
                    rebuild_tray_menu(&app, lang);
                });
            });

            // Start timer loop
            let app_handle2 = app.handle().clone();
            let state2 = app_state.clone();
            tauri::async_runtime::spawn(async move {
                features::timer::service::start_timer_loop(app_handle2, state2).await;
            });

            // Attendance reminder startup check (2s delay to let UI load)
            let app_handle3 = app.handle().clone();
            let state3 = app_state.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                features::timer::service::check_attendance_on_startup(&app_handle3, &state3).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "settings" {
                    api.prevent_close();
                    window.hide().ok();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &tauri::App, lang: &crate::features::settings::model::Language) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

    let menu = build_tray_menu(app, lang)?;

    let tray_builder = TrayIconBuilder::with_id("default")
        .tooltip(t(lang, "tray.tooltip"))
        .menu(&menu)
        .show_menu_on_left_click(false);

    let tray_builder = if let Some(icon) = app.default_window_icon().cloned() {
        tray_builder.icon(icon)
    } else {
        let png_data = include_bytes!("../icons/icon.png");
        let img = image::load_from_memory(png_data).expect("Failed to decode icon");
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        tray_builder.icon(tauri::image::Image::new_owned(rgba.into_raw(), w, h))
    };

    let _tray = tray_builder
        .on_menu_event(|app, event| match event.id.as_ref() {
            "skip" => {
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(state) = app_clone.try_state::<crate::core::state::AppState>() {
                        features::timer::service::end_break(&app_clone, &state).await;
                    }
                });
            }
            "pause" => {
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(state) = app_clone.try_state::<crate::core::state::AppState>() {
                        let mut s = state.lock().await;
                        let was_running = s.timer_status == crate::core::state::TimerStatus::Running;
                        s.timer_status = if was_running {
                            crate::core::state::TimerStatus::Paused
                        } else {
                            crate::core::state::TimerStatus::Running
                        };
                        let lang = s.settings.language.clone();
                        drop(s);
                        use tauri::Emitter;
                        let status = if was_running { "paused" } else { "running" };
                        app_clone.emit(crate::core::events::TIMER_STATUS_CHANGED,
                            crate::core::events::TimerStatusPayload { status: status.into() }).ok();
                        features::timer::commands::update_tray_pause_label(&app_clone, &lang, was_running);
                    }
                });
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("settings") {
                    window.show().ok();
                    window.set_focus().ok();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("settings") {
                    window.show().ok();
                    window.set_focus().ok();
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn build_tray_menu<R: tauri::Runtime, M: Manager<R>>(app: &M, lang: &crate::features::settings::model::Language) -> Result<tauri::menu::Menu<R>, Box<dyn std::error::Error>> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};

    let skip_item = MenuItemBuilder::with_id("skip", t(lang, "tray.skip")).build(app)?;
    let pause_item = MenuItemBuilder::with_id("pause", t(lang, "tray.pause")).build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", t(lang, "tray.settings")).build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", t(lang, "tray.quit")).build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&skip_item)
        .item(&pause_item)
        .item(&settings_item)
        .item(&separator)
        .item(&quit_item)
        .build()?;

    Ok(menu)
}

fn rebuild_tray_menu(app: &tauri::AppHandle, lang: &crate::features::settings::model::Language) {
    if let Some(tray) = app.tray_by_id("default") {
        if let Ok(menu) = build_tray_menu(app, lang) {
            tray.set_menu(Some(menu)).ok();
            tray.set_tooltip(Some(t(lang, "tray.tooltip"))).ok();
        }
    }
}
