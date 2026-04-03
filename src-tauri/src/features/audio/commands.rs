use crate::core::error::AppError;
use crate::core::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn play_sound(state: State<'_, AppState>) -> Result<(), AppError> {
    let volume = {
        let s = state.lock().await;
        if !s.settings.sound_enabled {
            return Ok(());
        }
        s.settings.sound_volume
    };
    tokio::task::spawn_blocking(move || super::service::play_sound_blocking(volume))
        .await
        .map_err(|e| AppError::Audio(e.to_string()))?
}

#[tauri::command]
pub async fn set_volume(volume: f32, state: State<'_, AppState>) -> Result<(), AppError> {
    let mut s = state.lock().await;
    s.settings.sound_volume = volume.clamp(0.0, 1.0);
    Ok(())
}
