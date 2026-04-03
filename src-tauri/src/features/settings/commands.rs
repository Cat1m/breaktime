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
    let image_changed = state.settings.custom_image_path != new_settings.custom_image_path;
    state.settings = new_settings;
    if image_changed {
        state.invalidate_image_cache();
        // Preload image in background (cache it now so break starts instantly)
        state.get_image_base64();
    }
    // 3. Emit SETTINGS_CHANGED event
    app.emit(SETTINGS_CHANGED, ()).ok();
    Ok(())
}
