use crate::core::error::AppError;

#[tauri::command]
pub async fn load_image(path: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || super::service::load_image_as_base64(&path))
        .await
        .map_err(|e| AppError::Image(e.to_string()))?
}

/// Return embedded default background as base64 JPEG
#[tauri::command]
pub fn get_default_bg() -> String {
    super::service::get_default_bg_base64()
}
