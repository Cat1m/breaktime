// Settings type - mirror cua Rust Settings struct
export interface Settings {
  mini_break_interval: number;
  mini_break_duration: number;
  long_break_interval: number;
  long_break_duration: number;
  sound_enabled: boolean;
  sound_volume: number;
  strict_mode: boolean;
  dnd_pause: boolean;
  idle_pause: boolean;
  idle_threshold_secs: number;
  custom_texts: string[];
  custom_image_path: string | null;
  start_on_boot: boolean;
  language: "en" | "vi";
  attendance_reminder_enabled: boolean;
  attendance_times: string[];
}

// Break event payloads
export interface BreakStartPayload {
  break_type: "mini" | "long" | "attendance";
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
