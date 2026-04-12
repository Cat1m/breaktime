use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "vi")]
    Vi,
}

impl Default for Language {
    fn default() -> Self { Language::En }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Break intervals (seconds)
    pub mini_break_interval: u64,  // default: 600 (10 min)
    pub mini_break_duration: u64,  // default: 20
    pub long_break_interval: u64,  // default: 1800 (30 min)
    pub long_break_duration: u64,  // default: 300 (5 min)

    // Sound
    pub sound_enabled: bool, // default: true
    pub sound_volume: f32,   // default: 0.7 (0.0 - 1.0)

    // Behavior
    pub strict_mode: bool,          // default: false (hide skip button)
    pub dnd_pause: bool,            // default: true
    pub idle_pause: bool,           // default: true
    pub idle_threshold_secs: u64,   // default: 120

    // Long break toggle
    #[serde(default)]
    pub long_break_enabled: bool, // default: false

    // Custom content
    pub custom_texts: Vec<String>,         // default: vec of exercise suggestions
    pub custom_image_path: Option<String>, // default: None
    #[serde(default)]
    pub custom_sound_path: Option<String>, // default: None

    // Startup
    pub start_on_boot: bool, // default: false

    // Language
    #[serde(default)]
    pub language: Language, // default: En
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mini_break_interval: 600,
            mini_break_duration: 20,
            long_break_interval: 1800,
            long_break_duration: 300,
            sound_enabled: true,
            sound_volume: 0.7,
            strict_mode: false,
            dnd_pause: true,
            idle_pause: true,
            idle_threshold_secs: 120,
            long_break_enabled: false,
            custom_texts: vec![
                "Have a sip! Stay hydrated.".to_string(),
                "Grab your water bottle and take a few sips.".to_string(),
                "Time for a sip! Your body will thank you.".to_string(),
                "Hydration check! Take a moment to drink some water.".to_string(),
                "Sip, stretch, and breathe. You deserve this break.".to_string(),
            ],
            custom_image_path: None,
            custom_sound_path: None,
            start_on_boot: false,
            language: Language::default(),
        }
    }
}
