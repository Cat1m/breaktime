use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::core::error::AppError;
use crate::core::state::AppState;
use super::service::PreviewState;
use tauri::State;

#[tauri::command]
pub async fn play_sound(state: State<'_, AppState>) -> Result<(), AppError> {
    let (volume, custom_sound) = {
        let mut s = state.lock().await;
        if !s.settings.sound_enabled {
            return Ok(());
        }
        let volume = s.settings.sound_volume;
        let custom_sound = s.get_sound_bytes();
        (volume, custom_sound)
    };
    tokio::task::spawn_blocking(move || {
        super::service::play_sound_blocking(volume, custom_sound.as_deref())
    })
    .await
    .map_err(|e| AppError::Audio(e.to_string()))?
}

#[tauri::command]
pub async fn set_volume(volume: f32, state: State<'_, AppState>) -> Result<(), AppError> {
    let mut s = state.lock().await;
    s.settings.sound_volume = volume.clamp(0.0, 1.0);
    Ok(())
}

/// Toggle preview: if playing → stop, if stopped → play.
/// Returns true if preview started, false if stopped.
#[tauri::command]
pub async fn preview_sound(
    path: String,
    state: State<'_, AppState>,
    preview: State<'_, Arc<PreviewState>>,
) -> Result<bool, AppError> {
    // If currently playing, signal stop
    if preview.active.load(Ordering::Relaxed) {
        preview.stop.store(true, Ordering::Relaxed);
        return Ok(false);
    }

    let volume = {
        let s = state.lock().await;
        s.settings.sound_volume
    };
    let bytes = super::service::load_sound_from_file(&path)?;
    let preview = Arc::clone(&preview);
    tokio::task::spawn_blocking(move || {
        super::service::play_sound_stoppable(volume, &bytes, &preview).ok();
    })
    .await
    .map_err(|e| AppError::Audio(e.to_string()))?;
    Ok(true)
}
