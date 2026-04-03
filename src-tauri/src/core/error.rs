// Unified error type cho toan bo app
// Impl serde::Serialize de co the tra ve qua IPC

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Image error: {0}")]
    Image(String),

    #[error("Timer error: {0}")]
    Timer(String),

    #[error("{0}")]
    General(String),
}

// QUAN TRONG: Tauri yeu cau Serialize cho error type tra ve tu commands
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

pub type AppResult<T> = Result<T, AppError>;
