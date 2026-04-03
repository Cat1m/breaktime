import { invoke } from "@tauri-apps/api/core";

export function useAudio() {
  const playBreakSound = async () => {
    try {
      await invoke("play_sound");
    } catch (e) {
      console.warn("Failed to play sound:", e);
    }
  };

  const setVolume = async (volume: number) => {
    await invoke("set_volume", { volume });
  };

  return { playBreakSound, setVolume };
}
