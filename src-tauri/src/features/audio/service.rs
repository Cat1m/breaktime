use rodio::{Decoder, OutputStream, Sink};
use crate::core::error::{AppError, AppResult};
use std::io::Cursor;

// Embedded sound file (include_bytes!)
const BREAK_SOUND: &[u8] = include_bytes!("../../../sounds/break_start.ogg");

pub struct AudioPlayer {
    _stream: OutputStream, // giu song de khong bi drop
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> AppResult<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::Audio(e.to_string()))?;
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AppError::Audio(e.to_string()))?;
        Ok(Self {
            _stream: stream,
            sink,
        })
    }

    pub fn play_break_sound(&self) -> AppResult<()> {
        let cursor = Cursor::new(BREAK_SOUND);
        let source = Decoder::new(cursor)
            .map_err(|e| AppError::Audio(e.to_string()))?;
        self.sink.append(source);
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        // volume: 0.0 - 1.0
        self.sink.set_volume(volume);
    }
}

// NOTE: AudioPlayer KHONG Send vi OutputStream.
// Can tao moi hoac dung std::thread cho playback.
// Giai phap: spawn std::thread::spawn cho moi lan play.

pub fn play_sound_blocking(volume: f32) -> AppResult<()> {
    // Tao OutputStream + Sink trong thread hien tai
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| AppError::Audio(e.to_string()))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.set_volume(volume);

    let cursor = Cursor::new(BREAK_SOUND);
    let source = Decoder::new(cursor)
        .map_err(|e| AppError::Audio(e.to_string()))?;
    sink.append(source);

    // Doi den khi xong
    sink.sleep_until_end();
    Ok(())
}
