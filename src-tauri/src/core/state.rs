use std::sync::Arc;
use tokio::sync::Mutex;
use crate::features::settings::model::Settings;
use crate::core::events::BreakStartPayload;

// Timer state machine
#[derive(Debug, Clone, PartialEq)]
pub enum TimerStatus {
    Running,
    Paused,
    OnBreak,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BreakType {
    Mini,
    Long,
}

// Inner state (wrapped trong Mutex)
#[derive(Debug)]
pub struct AppStateInner {
    pub settings: Settings,
    pub timer_status: TimerStatus,
    pub is_idle: bool,
    pub elapsed_since_last_mini: u64,  // seconds
    pub elapsed_since_last_long: u64,  // seconds
    pub mini_breaks_since_long: u32,
    pub current_break_type: Option<BreakType>,
    // Active break payload — overlay windows fetch this on mount
    pub current_break_payload: Option<BreakStartPayload>,
    // Cached image: (path, base64_data)
    pub cached_image: Option<(String, String)>,
}

impl AppStateInner {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            timer_status: TimerStatus::Running,
            is_idle: false,
            elapsed_since_last_mini: 0,
            elapsed_since_last_long: 0,
            mini_breaks_since_long: 0,
            current_break_type: None,
            current_break_payload: None,
            cached_image: None,
        }
    }

    /// Get cached image base64, or load + cache if path changed.
    /// Falls back to embedded default bg when no custom image is set.
    pub fn get_image_base64(&mut self) -> Option<String> {
        match &self.settings.custom_image_path {
            Some(path) => {
                // Return cache if same path
                if let Some((cached_path, cached_data)) = &self.cached_image {
                    if cached_path == path {
                        return Some(cached_data.clone());
                    }
                }
                // Load and cache
                match crate::features::image_loader::service::load_image_as_base64(path) {
                    Ok(data) => {
                        self.cached_image = Some((path.clone(), data.clone()));
                        Some(data)
                    }
                    Err(_) => Some(crate::features::image_loader::service::get_default_bg_base64()),
                }
            }
            None => {
                // No custom image → use embedded default
                Some(crate::features::image_loader::service::get_default_bg_base64())
            }
        }
    }

    /// Invalidate cache (call when user changes image)
    pub fn invalidate_image_cache(&mut self) {
        self.cached_image = None;
    }

    pub fn reset_mini_timer(&mut self) {
        self.elapsed_since_last_mini = 0;
        self.mini_breaks_since_long += 1;
    }

    pub fn reset_long_timer(&mut self) {
        self.elapsed_since_last_long = 0;
        self.elapsed_since_last_mini = 0;
        self.mini_breaks_since_long = 0;
    }

    pub fn reset_all_timers(&mut self) {
        self.elapsed_since_last_mini = 0;
        self.elapsed_since_last_long = 0;
        self.mini_breaks_since_long = 0;
    }
}

// Type alias cho Tauri managed state
pub type AppState = Arc<Mutex<AppStateInner>>;

pub fn create_app_state(settings: Settings) -> AppState {
    Arc::new(Mutex::new(AppStateInner::new(settings)))
}
