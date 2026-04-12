use crate::core::error::AppError;
use crate::core::events::{BreakStartPayload, TimerStatusPayload, TIMER_STATUS_CHANGED};
use crate::core::state::{AppState, TimerStatus};
use tauri::{Emitter, Manager, State};

#[tauri::command]
pub async fn pause_timer(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    let mut s = state.lock().await;
    s.timer_status = TimerStatus::Paused;
    let lang = s.settings.language.clone();
    drop(s);
    // Notify frontend
    app.emit(
        TIMER_STATUS_CHANGED,
        TimerStatusPayload {
            status: "paused".into(),
        },
    )
    .ok();
    // Update tray menu text
    update_tray_pause_label(&app, &lang, true);
    Ok(())
}

#[tauri::command]
pub async fn resume_timer(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    let mut s = state.lock().await;
    s.timer_status = TimerStatus::Running;
    let lang = s.settings.language.clone();
    drop(s);
    app.emit(
        TIMER_STATUS_CHANGED,
        TimerStatusPayload {
            status: "running".into(),
        },
    )
    .ok();
    update_tray_pause_label(&app, &lang, false);
    Ok(())
}

#[tauri::command]
pub async fn skip_break(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), AppError> {
    super::service::end_break(&app, &state).await;
    Ok(())
}

#[tauri::command]
pub async fn get_timer_status(state: State<'_, AppState>) -> Result<String, AppError> {
    let s = state.lock().await;
    Ok(match s.timer_status {
        TimerStatus::Running => "running",
        TimerStatus::Paused => "paused",
        TimerStatus::OnBreak => "on_break",
    }
    .into())
}

#[tauri::command]
pub async fn get_active_break(
    state: State<'_, AppState>,
) -> Result<Option<BreakStartPayload>, AppError> {
    let s = state.lock().await;
    Ok(s.current_break_payload.clone())
}

/// Rebuild tray menu to reflect pause/resume state
pub fn update_tray_pause_label(
    app: &tauri::AppHandle,
    lang: &crate::features::settings::model::Language,
    is_paused: bool,
) {
    use crate::core::l10n::t;
    use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};

    let pause_label = if is_paused {
        t(lang, "tray.resume")
    } else {
        t(lang, "tray.pause")
    };

    // Rebuild menu with updated pause label
    let Ok(skip) = MenuItemBuilder::with_id("skip", t(lang, "tray.skip")).build(app) else {
        return;
    };
    let Ok(pause) = MenuItemBuilder::with_id("pause", pause_label).build(app) else {
        return;
    };
    let Ok(settings) = MenuItemBuilder::with_id("settings", t(lang, "tray.settings")).build(app)
    else {
        return;
    };
    let Ok(sep) = PredefinedMenuItem::separator(app) else {
        return;
    };
    let Ok(quit) = MenuItemBuilder::with_id("quit", t(lang, "tray.quit")).build(app) else {
        return;
    };

    let Ok(menu) = MenuBuilder::new(app)
        .item(&skip)
        .item(&pause)
        .item(&settings)
        .item(&sep)
        .item(&quit)
        .build()
    else {
        return;
    };

    if let Some(tray) = app.tray_by_id("default") {
        tray.set_menu(Some(menu)).ok();
    }
}
