use tauri::State;
use crate::core::state::AppState;
use crate::core::error::AppError;
use crate::core::events::SETTINGS_CHANGED;
use super::model::Settings;
use super::service;
use tauri::Emitter;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, AppError> {
    let state = state.lock().await;
    Ok(state.settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    new_settings: Settings,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    // 1. Ghi ra file
    service::save_settings(&new_settings)?;
    // 2. Cap nhat AppState + preload image
    let mut state = state.lock().await;
    let boot_changed = state.settings.start_on_boot != new_settings.start_on_boot;
    let image_changed = state.settings.custom_image_path != new_settings.custom_image_path;
    let sound_changed = state.settings.custom_sound_path != new_settings.custom_sound_path;
    state.settings = new_settings.clone();
    if image_changed {
        state.invalidate_image_cache();
        // Preload image in background (cache it now so break starts instantly)
        state.get_image_base64();
    }
    if sound_changed {
        state.invalidate_sound_cache();
        // Preload sound bytes
        state.get_sound_bytes();
    }
    drop(state);
    // 3. Sync autostart with OS
    if boot_changed {
        use tauri_plugin_autostart::ManagerExt;
        let autostart = app.autolaunch();
        if new_settings.start_on_boot {
            autostart.enable().map_err(|e| AppError::General(format!("Failed to enable autostart: {}", e)))?;
        } else {
            autostart.disable().map_err(|e| AppError::General(format!("Failed to disable autostart: {}", e)))?;
        }
    }
    // 4. Emit SETTINGS_CHANGED event
    app.emit(SETTINGS_CHANGED, ()).ok();
    Ok(())
}
