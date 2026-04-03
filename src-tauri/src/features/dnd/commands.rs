use crate::core::error::AppError;

#[tauri::command]
pub async fn is_dnd_active() -> Result<bool, AppError> {
    // Goi service::is_dnd_active() trong spawn_blocking
    // vi co the co I/O (registry read, etc.)
    tokio::task::spawn_blocking(|| super::service::is_dnd_active())
        .await
        .map_err(|e| AppError::General(e.to_string()))?
}
