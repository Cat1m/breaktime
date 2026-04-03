use crate::core::error::{AppError, AppResult};
use base64::Engine;
use std::path::Path;
use std::sync::LazyLock;

static DEFAULT_BG_BYTES: &[u8] = include_bytes!("../../../resources/default_bg.jpg");
static DEFAULT_BG_BASE64: LazyLock<String> = LazyLock::new(|| {
    base64::engine::general_purpose::STANDARD.encode(DEFAULT_BG_BYTES)
});

pub fn get_default_bg_base64() -> String {
    DEFAULT_BG_BASE64.clone()
}

/// Load image, resize to overlay-friendly size, encode as JPEG base64
pub fn load_image_as_base64(path: &str) -> AppResult<String> {
    let path = Path::new(path);
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let jpeg_bytes = match extension.as_str() {
        "raf" => load_raf(path)?,
        _ => load_standard_image(path)?,
    };

    Ok(base64::engine::general_purpose::STANDARD.encode(&jpeg_bytes))
}

fn load_standard_image(path: &Path) -> AppResult<Vec<u8>> {
    let img = image::open(path).map_err(|e| AppError::Image(e.to_string()))?;

    // Resize for fullscreen overlay (max 1920x1080, covers most displays)
    let img = if img.width() > 1920 || img.height() > 1080 {
        img.resize(1920, 1080, image::imageops::FilterType::Triangle)
    } else {
        img
    };

    // Encode as JPEG quality 85 (good quality, much smaller than PNG)
    let mut buffer = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 85);
    img.write_with_encoder(encoder)
        .map_err(|e| AppError::Image(e.to_string()))?;

    Ok(buffer)
}

#[cfg(feature = "raf_support")]
fn load_raf(path: &Path) -> AppResult<Vec<u8>> {
    // Dung rsraw crate de decode RAF
    // Convert to image::DynamicImage
    // Encode to PNG
    let raw = rsraw::RawImage::decode_file(path)
        .map_err(|e| AppError::Image(e.to_string()))?;
    let img = raw.to_dynamic_image()
        .map_err(|e| AppError::Image(e.to_string()))?;
    let mut buffer = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageFormat::Png,
    )
    .map_err(|e| AppError::Image(e.to_string()))?;
    Ok(buffer)
}

#[cfg(not(feature = "raf_support"))]
fn load_raf(_path: &Path) -> AppResult<Vec<u8>> {
    Err(AppError::Image(
        "RAF support not compiled. Enable 'raf_support' feature.".into(),
    ))
}
