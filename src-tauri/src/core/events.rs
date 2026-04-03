// Event name constants cho Tauri emit/listen

pub const BREAK_START: &str = "break:start";
pub const BREAK_END: &str = "break:end";
pub const BREAK_TICK: &str = "break:tick";
pub const TIMER_STATUS_CHANGED: &str = "timer:status-changed";
pub const IDLE_CHANGED: &str = "idle:changed";
pub const SETTINGS_CHANGED: &str = "settings:changed";
pub const TIMER_TICK: &str = "timer:tick";

// Payload structs (derive Serialize, Clone)

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct BreakStartPayload {
    pub break_type: String,      // "mini" | "long"
    pub duration_secs: u64,
    pub message: String,
    pub image_base64: Option<String>,
}

#[derive(serde::Serialize, Clone)]
pub struct BreakTickPayload {
    pub remaining_secs: u64,
}

#[derive(serde::Serialize, Clone)]
pub struct TimerStatusPayload {
    pub status: String, // "running" | "paused" | "on_break"
}

#[derive(serde::Serialize, Clone)]
pub struct TimerTickPayload {
    pub status: String,           // "running" | "paused" | "on_break"
    pub secs_until_mini: u64,
    pub secs_until_long: u64,
    pub mini_break_interval: u64,
    pub long_break_interval: u64,
}
