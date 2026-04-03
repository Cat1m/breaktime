use crate::core::error::{AppError, AppResult};
use super::model::Settings;

const CONFIG_FILE: &str = "sipping-settings.json";

/// Lay duong dan config: dirs::config_dir() / "sipping" / CONFIG_FILE
pub fn config_path() -> AppResult<std::path::PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::General("Cannot find config directory".to_string()))?;
    Ok(config_dir.join("sipping").join(CONFIG_FILE))
}

/// Doc settings tu file, tra ve Default neu file chua ton tai
pub fn load_settings() -> AppResult<Settings> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Settings::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let settings: Settings = serde_json::from_str(&content)?;
    Ok(settings)
}

/// Ghi settings ra file JSON (pretty-printed)
/// Tao parent dir neu chua ton tai
pub fn save_settings(settings: &Settings) -> AppResult<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, content)?;
    Ok(())
}
