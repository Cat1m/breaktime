import { useState, useEffect } from "react";
import { useSettings } from "./useSettings";
import { useTimerContext } from "../../contexts/TimerContext";
import { useLocale } from "../../contexts/LocaleContext";
import { Toggle } from "../../shared/components/Toggle";
import { NumberInput } from "../../shared/components/NumberInput";
import { Button } from "../../shared/components/Button";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-shell";
import qrImage from "../../assets/qr_donate.jpg";
import styles from "./SettingsPanel.module.css";

export function SettingsPanel() {
  const {
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
  } = useSettings();

  const { timerStatus, secsUntilMini, secsUntilLong, pauseTimer, resumeTimer } = useTimerContext();
  const { t } = useLocale();

  const formatTime = (secs: number) => {
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    return m > 0 ? `${m}m ${String(s).padStart(2, "0")}s` : `${s}s`;
  };
  const [newTextInput, setNewTextInput] = useState("");
  const [isDragging, setIsDragging] = useState(false);
  const [isPreviewing, setIsPreviewing] = useState(false);
  const [showQr, setShowQr] = useState(false);

  const IMAGE_EXTENSIONS = ["png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "raf"];

  // Listen for Tauri native file drag-drop
  useEffect(() => {
    const unlisten = getCurrentWindow().onDragDropEvent((event) => {
      const type = event.payload.type;
      if (type === "enter" || type === "over") {
        setIsDragging(true);
      } else if (type === "drop") {
        setIsDragging(false);
        const paths = event.payload.paths;
        if (paths.length > 0) {
          const filePath = paths[0];
          const ext = filePath.split(".").pop()?.toLowerCase() ?? "";
          if (IMAGE_EXTENSIONS.includes(ext)) {
            updateField("custom_image_path", filePath);
          }
        }
      } else if (type === "leave") {
        setIsDragging(false);
      }
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [updateField]);

  if (loading) {
    return (
      <div className={styles.panel}>
        <p>{t("loading")}</p>
      </div>
    );
  }

  if (!settings) {
    return (
      <div className={styles.panel}>
        <p>{t("error.loadFailed")}{error ? `: ${error}` : ""}</p>
      </div>
    );
  }

  const handleAddText = () => {
    const trimmed = newTextInput.trim();
    if (trimmed) {
      addCustomText(trimmed);
      setNewTextInput("");
    }
  };

  return (
    <div className={styles.panel}>
      <h1 className={styles.title}>{t("app.title")}</h1>

      {/* Section 0: Timer Status */}
      <div className={styles.statusSection}>
        <div className={styles.statusHeader}>
          <div className={styles.statusIndicator}>
            <span className={styles.statusDot} data-status={timerStatus} />
            <span className={styles.statusText}>
              {timerStatus === "running"
                ? t("status.running")
                : timerStatus === "paused"
                ? t("status.paused")
                : t("status.onBreak")}
            </span>
          </div>
          <Button
            variant={timerStatus === "running" ? "secondary" : "primary"}
            onClick={timerStatus === "running" ? pauseTimer : resumeTimer}
          >
            {timerStatus === "running" ? t("button.pause") : t("button.start")}
          </Button>
        </div>
        {timerStatus === "running" && (
          <div className={styles.countdownRow}>
            <div className={styles.countdownItem}>
              <span className={styles.countdownLabel}>{t("countdown.mini")}</span>
              <span className={styles.countdownValue}>{formatTime(secsUntilMini)}</span>
            </div>
            {settings.long_break_enabled && (
              <>
                <div className={styles.countdownDivider} />
                <div className={styles.countdownItem}>
                  <span className={styles.countdownLabel}>{t("countdown.long")}</span>
                  <span className={styles.countdownValue}>{formatTime(secsUntilLong)}</span>
                </div>
              </>
            )}
          </div>
        )}
      </div>

      {/* Language selector */}
      <div className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("section.language")}</h2>
        <div className={styles.field}>
          <select
            className={styles.langSelect}
            value={settings.language}
            onChange={(e) => updateField("language", e.target.value as "en" | "vi")}
          >
            <option value="en">English</option>
            <option value="vi">Tiếng Việt</option>
          </select>
        </div>
      </div>

      {/* Break Schedule */}
      <div className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("section.breakSchedule")}</h2>

        <div className={styles.field}>
          <NumberInput label={t("field.miniInterval")} value={Math.round(settings.mini_break_interval / 60)} onChange={(v) => updateField("mini_break_interval", v * 60)} min={1} max={120} step={1} unit={t("unit.minutes")} />
        </div>
        <div className={styles.field}>
          <NumberInput label={t("field.miniDuration")} value={settings.mini_break_duration} onChange={(v) => updateField("mini_break_duration", v)} min={5} max={300} step={5} unit={t("unit.seconds")} />
        </div>
        <div className={styles.field}>
          <Toggle label={t("field.longBreakEnabled")} checked={settings.long_break_enabled} onChange={(v) => updateField("long_break_enabled", v)} />
        </div>
        {settings.long_break_enabled && (
          <>
            <div className={styles.field}>
              <NumberInput label={t("field.longInterval")} value={Math.round(settings.long_break_interval / 60)} onChange={(v) => updateField("long_break_interval", v * 60)} min={1} max={240} step={1} unit={t("unit.minutes")} />
            </div>
            <div className={styles.field}>
              <NumberInput label={t("field.longDuration")} value={Math.round(settings.long_break_duration / 60)} onChange={(v) => updateField("long_break_duration", v * 60)} min={1} max={30} step={1} unit={t("unit.minutes")} />
            </div>
          </>
        )}
      </div>

      {/* Sound */}
      <div className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("section.sound")}</h2>
        <div className={styles.field}>
          <Toggle label={t("field.enableSound")} checked={settings.sound_enabled} onChange={(v) => updateField("sound_enabled", v)} />
        </div>
        <div className={styles.field}>
          <NumberInput label={t("field.volume")} value={Math.round(settings.sound_volume * 100)} onChange={(v) => updateField("sound_volume", v / 100)} min={0} max={100} step={5} unit="%" />
        </div>
        <div className={styles.field}>
          <p style={{ fontSize: "12px", color: "#6b7280", marginBottom: "8px" }}>
            {t("field.customSound")}
          </p>
          <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
            <Button variant="secondary" onClick={pickSoundFile}>
              {t("button.chooseSound")}
            </Button>
            {settings.custom_sound_path && (
              <>
                <Button variant={isPreviewing ? "primary" : "secondary"} onClick={() => {
                  if (isPreviewing) {
                    previewSound();
                    setIsPreviewing(false);
                  } else {
                    setIsPreviewing(true);
                    previewSound().then(() => setIsPreviewing(false));
                  }
                }}>
                  {isPreviewing ? t("button.stopSound") : t("button.previewSound")}
                </Button>
                <Button variant="danger" onClick={() => { if (isPreviewing) { previewSound(); setIsPreviewing(false); } clearSound(); }}>
                  {t("button.clearSound")}
                </Button>
              </>
            )}
          </div>
          {settings.custom_sound_path && (
            <p className={styles.imagePath} style={{ marginTop: "6px" }}>
              {settings.custom_sound_path.split(/[/\\]/).pop()}
            </p>
          )}
        </div>
      </div>

      {/* Behavior */}
      <div className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("section.behavior")}</h2>
        <div className={styles.field}>
          <Toggle label={t("field.strictMode")} checked={settings.strict_mode} onChange={(v) => updateField("strict_mode", v)} />
        </div>
        <div className={styles.field}>
          <Toggle label={t("field.dndPause")} checked={settings.dnd_pause} onChange={(v) => updateField("dnd_pause", v)} />
        </div>
        <div className={styles.field}>
          <Toggle label={t("field.idlePause")} checked={settings.idle_pause} onChange={(v) => updateField("idle_pause", v)} />
        </div>
        <div className={styles.field}>
          <NumberInput label={t("field.idleThreshold")} value={settings.idle_threshold_secs} onChange={(v) => updateField("idle_threshold_secs", v)} min={30} max={600} step={30} unit={t("unit.seconds")} />
        </div>
        <div className={styles.field}>
          <Toggle label={t("field.startOnBoot")} checked={settings.start_on_boot} onChange={(v) => updateField("start_on_boot", v)} />
        </div>
      </div>

      {/* Custom Content */}
      <div className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("section.customContent")}</h2>

        <div className={styles.field}>
          <p style={{ fontSize: "12px", color: "#6b7280", marginBottom: "8px" }}>
            {t("customContent.description")}
          </p>
          <div className={styles.textList}>
            {settings.custom_texts.map((text, i) => (
              <div key={i} className={styles.textItem}>
                <input className={styles.textInput} type="text" value={text} onChange={(e) => updateCustomText(i, e.target.value)} />
                <Button variant="danger" onClick={() => removeCustomText(i)}>
                  {t("button.remove")}
                </Button>
              </div>
            ))}
          </div>
          <div className={styles.textItem} style={{ marginTop: "8px" }}>
            <input className={styles.textInput} type="text" placeholder={t("customContent.placeholder")} value={newTextInput} onChange={(e) => setNewTextInput(e.target.value)} onKeyDown={(e) => { if (e.key === "Enter") handleAddText(); }} />
            <Button variant="primary" onClick={handleAddText}>
              {t("button.add")}
            </Button>
          </div>
        </div>

        <div className={styles.field}>
          <p style={{ fontSize: "12px", color: "#6b7280", marginBottom: "8px" }}>
            {t("customContent.imageLabel")}
          </p>
          <div className={`${styles.dropZone} ${isDragging ? styles.dropZoneActive : ""}`}>
            <div className={styles.dropZoneContent}>
              {isDragging ? (
                <span className={styles.dropZoneActiveText}>{t("dropzone.active")}</span>
              ) : (
                <>
                  <Button variant="secondary" onClick={pickImageFile}>
                    {t("button.chooseImage")}
                  </Button>
                  <span className={styles.dropZoneHint}>{t("dropzone.hint")}</span>
                </>
              )}
            </div>
          </div>
          {settings.custom_image_path && (
            <div className={styles.imageInfo}>
              <p className={styles.imagePath}>{settings.custom_image_path.split(/[/\\]/).pop()}</p>
              <Button variant="danger" onClick={clearImage}>
                {t("button.clearImage")}
              </Button>
            </div>
          )}
        </div>
      </div>

      {/* Support */}
      <div className={styles.supportSection}>
        <p className={styles.aboutText}>{t("support.about")}</p>
        <p className={styles.thankYou}>{t("support.thanks")}</p>
        <div className={styles.donateLinks}>
          <Button variant="primary" onClick={() => open("https://ko-fi.com/minhchienle")}>
            {t("support.kofi")}
          </Button>
          <Button variant="secondary" onClick={() => open("https://paypal.me/ArcaRyze")}>
            {t("support.paypal")}
          </Button>
          <Button variant="secondary" onClick={() => setShowQr(!showQr)}>
            {showQr ? t("support.hideQr") : t("support.qrLabel")}
          </Button>
        </div>
        {showQr && (
          <div className={styles.qrWrapper}>
            <img src={qrImage} alt="QR Donate" className={styles.qrImage} />
          </div>
        )}
      </div>
    </div>
  );
}
