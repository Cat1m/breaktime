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

  const addAttendanceTime = (time: string) => {
    if (!settings) return;
    if (settings.attendance_times.includes(time)) return;
    const newTimes = [...settings.attendance_times, time].sort();
    updateField("attendance_times", newTimes);
  };

  const removeAttendanceTime = (index: number) => {
    if (!settings) return;
    const newTimes = settings.attendance_times.filter((_, i) => i !== index);
    updateField("attendance_times", newTimes);
  };

  return {
    settings,
    updateField,
    pickImageFile,
    clearImage,
    loading,
    error,
    addCustomText,
    removeCustomText,
    updateCustomText,
    addAttendanceTime,
    removeAttendanceTime,
  };
}
