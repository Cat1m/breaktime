use tauri::{AppHandle, Emitter, Manager};
use chrono::{Local, Timelike, Datelike, Weekday, NaiveTime};
use crate::core::state::{AppState, TimerStatus, BreakType};
use crate::core::events::*;

fn format_duration(secs: u64) -> String {
    let m = secs / 60;
    let s = secs % 60;
    if m > 0 { format!("{}m {:02}s", m, s) } else { format!("{}s", s) }
}

/// Bat dau timer loop - goi 1 lan khi app start
/// Chay trong tokio::spawn, loop moi giay
pub async fn start_timer_loop(app: AppHandle, state: AppState) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let mut s = state.lock().await;

        // 1. Kiem tra idle — pause timer khi idle, resume khi active
        if s.settings.idle_pause {
            match crate::features::idle::service::get_idle_seconds() {
                Ok(idle) => {
                    s.is_idle = idle >= s.settings.idle_threshold_secs;
                    if s.is_idle {
                        // Idle → don't increment counters, just skip this tick
                        continue;
                    }
                }
                Err(_) => {} // ignore idle detection errors
            }
        }

        // 2. Kiem tra DND
        if s.settings.dnd_pause {
            drop(s); // release lock truoc khi blocking call
            let dnd = tokio::task::spawn_blocking(|| {
                crate::features::dnd::service::is_dnd_active().unwrap_or(false)
            })
            .await
            .unwrap_or(false);
            s = state.lock().await;
            if dnd {
                continue;
            }
        }

        // 3. Neu dang paused hoac on_break -> skip
        if s.timer_status != TimerStatus::Running {
            continue;
        }

        // 3b. Attendance reminder check (once per minute)
        if s.settings.attendance_reminder_enabled {
            let now = Local::now();
            if now.second() == 0 && now.weekday() != Weekday::Sun {
                let app_clone = app.clone();
                let state_clone = state.clone();
                drop(s);
                check_attendance_reminder(&app_clone, &state_clone).await;
                s = state.lock().await;
                // If attendance triggered a break, skip the rest of this tick
                if s.timer_status == TimerStatus::OnBreak {
                    continue;
                }
            }
        }

        // 4. Tang elapsed counters
        s.elapsed_since_last_mini += 1;
        s.elapsed_since_last_long += 1;

        // 4b. Emit timer:tick event + update tray tooltip
        let secs_until_mini = s.settings.mini_break_interval.saturating_sub(s.elapsed_since_last_mini);
        let secs_until_long = s.settings.long_break_interval.saturating_sub(s.elapsed_since_last_long);
        let tick_payload = TimerTickPayload {
            status: "running".into(),
            secs_until_mini,
            secs_until_long,
            mini_break_interval: s.settings.mini_break_interval,
            long_break_interval: s.settings.long_break_interval,
        };
        app.emit(TIMER_TICK, tick_payload).ok();

        // Update tray tooltip
        let lang = &s.settings.language;
        let tooltip = format!(
            "Sipping — {}: {} | {}: {}",
            crate::core::l10n::t(lang, "tooltip.mini"),
            format_duration(secs_until_mini),
            crate::core::l10n::t(lang, "tooltip.long"),
            format_duration(secs_until_long),
        );
        if let Some(tray) = app.tray_by_id("default") {
            tray.set_tooltip(Some(&tooltip)).ok();
        }

        // 5. Kiem tra long break truoc (uu tien cao hon)
        if s.elapsed_since_last_long >= s.settings.long_break_interval {
            let app_clone = app.clone();
            let state_clone = state.clone();
            drop(s);
            trigger_break_standalone(&app_clone, &state_clone, BreakType::Long).await;
            continue;
        }

        // 6. Kiem tra mini break
        if s.elapsed_since_last_mini >= s.settings.mini_break_interval {
            let app_clone = app.clone();
            let state_clone = state.clone();
            drop(s);
            trigger_break_standalone(&app_clone, &state_clone, BreakType::Mini).await;
        }
    }
}

async fn trigger_break_standalone(app: &AppHandle, state: &AppState, break_type: BreakType) {
    // 1. Lock state, build payload, store in state, then release lock
    let (play_sound, volume) = {
        let mut s = state.lock().await;
        s.timer_status = TimerStatus::OnBreak;
        s.current_break_type = Some(break_type.clone());

        let duration = match break_type {
            BreakType::Mini => s.settings.mini_break_duration,
            BreakType::Long => s.settings.long_break_duration,
            BreakType::Attendance => 15, // attendance uses its own trigger path
        };

        let message = pick_random_text(&s.settings.custom_texts, &s.settings.language);
        let image_base64 = s.get_image_base64();

        let payload = BreakStartPayload {
            break_type: match break_type {
                BreakType::Mini => "mini",
                BreakType::Long => "long",
                BreakType::Attendance => "attendance",
            }
            .into(),
            duration_secs: duration,
            message,
            image_base64,
        };

        // Store payload in state — overlay windows fetch this on mount
        s.current_break_payload = Some(payload);

        let play_sound = s.settings.sound_enabled;
        let volume = s.settings.sound_volume;
        (play_sound, volume)
    }; // lock released

    // 2. Create overlay windows (one per monitor)
    create_overlay_window(app);

    // 3. Small delay to let windows start loading
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // 4. Emit break:start as well (for any window already listening)
    {
        let s = state.lock().await;
        if let Some(payload) = &s.current_break_payload {
            app.emit(BREAK_START, payload.clone()).ok();
        }
    }

    // 5. Play sound
    if play_sound {
        tokio::task::spawn_blocking(move || {
            crate::features::audio::service::play_sound_blocking(volume).ok();
        });
    }
}

fn create_overlay_window(app: &AppHandle) {
    use tauri::WebviewWindowBuilder;

    // Get all monitors, create one overlay per monitor
    let monitors = match app.available_monitors() {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to get monitors: {}", e);
            // Fallback: single fullscreen overlay
            let _ = WebviewWindowBuilder::new(app, "overlay-0", tauri::WebviewUrl::App("index.html?window=overlay".into()))
                .fullscreen(true)
                .always_on_top(true)
                .decorations(false)
                .skip_taskbar(true)
                .build();
            return;
        }
    };

    if monitors.is_empty() {
        // No monitors detected, fallback
        let _ = WebviewWindowBuilder::new(app, "overlay-0", tauri::WebviewUrl::App("index.html?window=overlay".into()))
            .fullscreen(true)
            .always_on_top(true)
            .decorations(false)
            .skip_taskbar(true)
            .build();
        return;
    }

    for (i, monitor) in monitors.iter().enumerate() {
        let label = format!("overlay-{}", i);
        let pos = monitor.position();
        let size = monitor.size();

        let result = WebviewWindowBuilder::new(
            app,
            &label,
            tauri::WebviewUrl::App("index.html?window=overlay".into()),
        )
        .position(pos.x as f64, pos.y as f64)
        .inner_size(size.width as f64, size.height as f64)
        .always_on_top(true)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(false)
        .build();

        match result {
            Ok(window) => {
                // Force fullscreen after creation for proper coverage
                window.set_fullscreen(true).ok();
                println!("[Sipping] Overlay created on monitor {} ({}x{} at {},{})",
                    i, size.width, size.height, pos.x, pos.y);
            }
            Err(e) => {
                log::error!("Failed to create overlay on monitor {}: {}", i, e);
            }
        }
    }
}

pub fn destroy_overlay_window(app: &AppHandle) {
    // Close all overlay windows (overlay-0, overlay-1, ...)
    for i in 0..16 {
        let label = format!("overlay-{}", i);
        if let Some(window) = app.get_webview_window(&label) {
            window.close().ok();
        } else {
            break; // No more overlay windows
        }
    }
}

fn pick_random_text(texts: &[String], lang: &crate::features::settings::model::Language) -> String {
    if texts.is_empty() {
        return crate::core::l10n::t(lang, "break.default").to_string();
    }
    // Simple pseudo-random selection based on current time
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize)
        % texts.len();
    texts[idx].clone()
}

/// Goi sau khi break ket thuc (countdown = 0 hoac user skip)
pub async fn end_break(app: &AppHandle, state: &AppState) {
    let mut s = state.lock().await;
    match s.current_break_type.take() {
        Some(BreakType::Mini) => s.reset_mini_timer(),
        Some(BreakType::Long) => s.reset_long_timer(),
        Some(BreakType::Attendance) => {} // no timer reset needed
        None => {}
    }
    s.timer_status = TimerStatus::Running;
    s.current_break_payload = None;
    drop(s);
    destroy_overlay_window(app);
    app.emit(BREAK_END, ()).ok();
}

/// Check if any attendance time is due and trigger overlay
async fn check_attendance_reminder(app: &AppHandle, state: &AppState) {
    let now = Local::now();
    let today_str = now.format("%Y-%m-%d").to_string();
    let current_time = now.time();

    let mut s = state.lock().await;

    // Day rollover → clear reminders
    if s.attendance_reminded_date.as_deref() != Some(&today_str) {
        s.attendance_reminded_today.clear();
        s.attendance_reminded_date = Some(today_str);
    }

    // Already on break → skip
    if s.timer_status == TimerStatus::OnBreak {
        return;
    }

    let times = s.settings.attendance_times.clone();
    let lang = s.settings.language.clone();

    for time_str in &times {
        // Skip already-reminded times
        if s.attendance_reminded_today.contains(time_str) {
            continue;
        }

        // Parse "HH:MM"
        let target = match NaiveTime::parse_from_str(time_str, "%H:%M") {
            Ok(t) => t,
            Err(_) => continue,
        };

        // Check if current_time is within [target, target + 60min)
        let diff = current_time.signed_duration_since(target);
        let diff_secs = diff.num_seconds();
        if diff_secs >= 0 && diff_secs < 3600 {
            // Mark as reminded
            s.attendance_reminded_today.push(time_str.clone());

            // Build message
            let message = build_attendance_message(time_str, &lang);
            let image_base64 = s.get_image_base64();

            let payload = BreakStartPayload {
                break_type: "attendance".into(),
                duration_secs: 15,
                message,
                image_base64,
            };

            s.timer_status = TimerStatus::OnBreak;
            s.current_break_type = Some(BreakType::Attendance);
            s.current_break_payload = Some(payload);

            let play_sound = s.settings.sound_enabled;
            let volume = s.settings.sound_volume;
            drop(s);

            // Create overlay windows
            create_overlay_window(app);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            // Emit break:start
            {
                let s = state.lock().await;
                if let Some(payload) = &s.current_break_payload {
                    app.emit(BREAK_START, payload.clone()).ok();
                }
            }

            // Play sound
            if play_sound {
                tokio::task::spawn_blocking(move || {
                    crate::features::audio::service::play_sound_blocking(volume).ok();
                });
            }

            return; // Only one reminder at a time
        }
    }
}

/// Check attendance on startup (called once with delay)
pub async fn check_attendance_on_startup(app: &AppHandle, state: &AppState) {
    let now = Local::now();

    // Skip Sunday
    if now.weekday() == Weekday::Sun {
        return;
    }

    let s = state.lock().await;
    if !s.settings.attendance_reminder_enabled {
        return;
    }

    let times = s.settings.attendance_times.clone();
    drop(s);

    let current_time = now.time();

    for time_str in &times {
        let target = match NaiveTime::parse_from_str(time_str, "%H:%M") {
            Ok(t) => t,
            Err(_) => continue,
        };

        let diff = current_time.signed_duration_since(target);
        let diff_secs = diff.num_seconds();
        if diff_secs >= 0 && diff_secs < 3600 {
            // Within 1-hour window — trigger reminder
            check_attendance_reminder(app, state).await;
            return;
        }
    }
}

fn build_attendance_message(
    time_str: &str,
    lang: &crate::features::settings::model::Language,
) -> String {
    // Format time for display: "06:55" → "06:55 AM"
    let display_time = match NaiveTime::parse_from_str(time_str, "%H:%M") {
        Ok(t) => t.format("%I:%M %p").to_string(),
        Err(_) => time_str.to_string(),
    };

    match lang {
        crate::features::settings::model::Language::Vi => {
            format!("Chấm công {} chưa?", display_time)
        }
        crate::features::settings::model::Language::En => {
            format!("Time to check in at {}!", display_time)
        }
    }
}
