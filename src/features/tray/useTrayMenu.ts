// Hook nay KHONG thuc su can thiet cho frontend vi tray menu xu ly o Rust side.
// Tuy nhien, cung cap utility functions de frontend co the tuong tac voi tray state.

import { invoke } from "@tauri-apps/api/core";

export function useTrayMenu() {
  const pauseTimer = async () => invoke("pause_timer");
  const resumeTimer = async () => invoke("resume_timer");
  const skipBreak = async () => invoke("skip_break");

  return { pauseTimer, resumeTimer, skipBreak };
}
