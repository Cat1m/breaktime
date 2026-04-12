// Settings type - mirror cua Rust Settings struct
export interface Settings {
  mini_break_interval: number;
  mini_break_duration: number;
  long_break_interval: number;
  long_break_duration: number;
  long_break_enabled: boolean;
  sound_enabled: boolean;
  sound_volume: number;
  strict_mode: boolean;
  dnd_pause: boolean;
  idle_pause: boolean;
  idle_threshold_secs: number;
  custom_texts: string[];
  custom_image_path: string | null;
  custom_sound_path: string | null;
  start_on_boot: boolean;
  language: "en" | "vi";
}

// Break event payloads
export interface BreakStartPayload {
  break_type: "mini" | "long";
  duration_secs: number;
  message: string;
  image_base64: string | null;
}

export interface BreakTickPayload {
  remaining_secs: number;
}

export type TimerStatus = "running" | "paused" | "on_break";

export interface TimerTickPayload {
  status: TimerStatus;
  secs_until_mini: number;
  secs_until_long: number;
  mini_break_interval: number;
  long_break_interval: number;
}
