use rodio::{Decoder, OutputStream, Sink};
use crate::core::error::{AppError, AppResult};
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// Embedded default sound file
const BREAK_SOUND: &[u8] = include_bytes!("../../../sounds/break_start.ogg");

const SUPPORTED_EXTENSIONS: &[&str] = &["ogg", "wav", "flac", "mp3"];

/// Shared state for preview play/stop toggle.
/// Lives outside AppState Mutex since it's used by blocking threads.
pub struct PreviewState {
    pub active: AtomicBool,
    pub stop: AtomicBool,
}

impl PreviewState {
    pub fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
            stop: AtomicBool::new(false),
        }
    }
}

/// Load sound bytes from a file path on disk.
/// Validates the file extension is a supported audio format.
pub fn load_sound_from_file(path: &str) -> AppResult<Vec<u8>> {
    let p = Path::new(path);
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(AppError::Audio(format!(
            "Unsupported audio format: .{}. Supported: {}",
            ext,
            SUPPORTED_EXTENSIONS.join(", ")
        )));
    }

    if !p.exists() {
        return Err(AppError::Audio(format!("Sound file not found: {}", path)));
    }

    std::fs::read(p).map_err(|e| AppError::Audio(format!("Failed to read sound file: {}", e)))
}

/// Play sound blocking. Uses custom_sound_bytes if provided, otherwise the embedded default.
pub fn play_sound_blocking(volume: f32, custom_sound_bytes: Option<&[u8]>) -> AppResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| AppError::Audio(e.to_string()))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.set_volume(volume);

    let bytes = custom_sound_bytes.unwrap_or(BREAK_SOUND);
    let cursor = Cursor::new(bytes.to_vec());
    let source = Decoder::new(cursor)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.append(source);

    sink.sleep_until_end();
    Ok(())
}

/// Play sound with stop flag — polls every 50ms instead of blocking until end.
/// Used for preview so user can stop playback.
pub fn play_sound_stoppable(volume: f32, bytes: &[u8], preview: &Arc<PreviewState>) -> AppResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| AppError::Audio(e.to_string()))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.set_volume(volume);

    let cursor = Cursor::new(bytes.to_vec());
    let source = Decoder::new(cursor)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.append(source);

    preview.active.store(true, Ordering::Relaxed);
    while !sink.empty() {
        if preview.stop.load(Ordering::Relaxed) {
            sink.stop();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    preview.active.store(false, Ordering::Relaxed);
    preview.stop.store(false, Ordering::Relaxed);
    Ok(())
}
