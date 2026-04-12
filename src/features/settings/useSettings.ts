import { useSettingsContext } from "../../contexts/SettingsContext";
import { Settings } from "../../shared/types";

export function useSettings() {
  const { settings, saveSettings, loading, error } = useSettingsContext();

  // updateField: cap nhat mot field cu the trong settings
  const updateField = <K extends keyof Settings>(key: K, value: Settings[K]) => {
    if (!settings) return;
    const newSettings: Settings = { ...settings, [key]: value };
    saveSettings(newSettings);
  };

  // pickImageFile: mo file dialog de chon image
  // Note: Tauri dialog plugin can duoc cai them neu muon dung open()
  // Hien tai dung input[type=file] alternative
  const pickImageFile = async () => {
    if (!settings) return;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "raf"],
          },
        ],
      });
      if (selected) {
        // open() returns string | string[] | null
        const path = Array.isArray(selected) ? selected[0] : selected;
        if (path) {
          updateField("custom_image_path", path);
        }
      }
    } catch (e) {
      console.warn("Dialog not available:", e);
    }
  };

  const clearImage = () => {
    if (!settings) return;
    updateField("custom_image_path", null);
  };

  const pickSoundFile = async () => {
    if (!settings) return;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Audio",
            extensions: ["ogg", "wav", "flac", "mp3"],
          },
        ],
      });
      if (selected) {
        const path = Array.isArray(selected) ? selected[0] : selected;
        if (path) {
          updateField("custom_sound_path", path);
        }
      }
    } catch (e) {
      console.warn("Dialog not available:", e);
    }
  };

  const clearSound = () => {
    if (!settings) return;
    updateField("custom_sound_path", null);
  };

  const previewSound = async (): Promise<boolean> => {
    if (!settings?.custom_sound_path) return false;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<boolean>("preview_sound", { path: settings.custom_sound_path });
    } catch (e) {
      console.warn("Preview sound failed:", e);
      return false;
    }
  };

  const addCustomText = (text: string) => {
    if (!settings) return;
    updateField("custom_texts", [...settings.custom_texts, text]);
  };

  const removeCustomText = (index: number) => {
    if (!settings) return;
    const newTexts = settings.custom_texts.filter((_, i) => i !== index);
    updateField("custom_texts", newTexts);
  };

  const updateCustomText = (index: number, text: string) => {
    if (!settings) return;
    const newTexts = settings.custom_texts.map((t, i) => (i === index ? text : t));
    updateField("custom_texts", newTexts);
  };

  return {
    settings,
    updateField,
    pickImageFile,
    clearImage,
    pickSoundFile,
    clearSound,
    previewSound,
    loading,
    error,
    addCustomText,
    removeCustomText,
    updateCustomText,
  };
}
