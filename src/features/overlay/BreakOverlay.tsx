import { useEffect, useCallback } from "react";
import { useBreakEvents } from "./useBreakEvents";
import { useTimerContext } from "../../contexts/TimerContext";
import { useSettingsContext } from "../../contexts/SettingsContext";
import { useLocale } from "../../contexts/LocaleContext";
import { useAdaptiveColor } from "./useAdaptiveColor";
import { CountdownRing } from "../../shared/components/CountdownRing";
import styles from "./BreakOverlay.module.css";

export function BreakOverlay() {
  const breakState = useBreakEvents();
  const { skipBreak } = useTimerContext();
  const { settings } = useSettingsContext();
  const { t } = useLocale();
  const { cssVars } = useAdaptiveColor(breakState.imageBase64);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        skipBreak();
      }
    },
    [skipBreak]
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown]);

  if (!breakState.active) {
    return (
      <div className={styles.overlay}>
        <div className={styles.center}>
          <div className={styles.breatheIcon}>🧘</div>
          <p className={styles.message}>{t("overlay.preparing")}</p>
        </div>
        <span className={styles.escHint}>{t("overlay.escHint")}</span>
      </div>
    );
  }

  const isStrictMode = settings?.strict_mode ?? false;
  const isAttendance = breakState.breakType === "attendance";
  const hasImage = !!breakState.imageBase64;

  const breakTypeLabel = breakState.breakType === "mini"
    ? t("overlay.miniBreak")
    : breakState.breakType === "long"
    ? t("overlay.longBreak")
    : breakState.breakType === "attendance"
    ? t("overlay.attendance")
    : "";

  return (
    <div className={styles.overlay} style={cssVars as React.CSSProperties}>
      {hasImage && (
        <img className={styles.bgImage} src={`data:image/jpeg;base64,${breakState.imageBase64}`} alt="" />
      )}
      {hasImage && <div className={styles.scrim} />}

      <div className={styles.ui}>
        {breakState.breakType && (
          <span className={styles.breakType}>
            {breakTypeLabel}
          </span>
        )}

        <div className={styles.center}>
          <div className={styles.ringWrapper}>
            <CountdownRing remainingSecs={breakState.remainingSecs} totalSecs={breakState.totalDuration} size={180} />
          </div>
          <p className={styles.message}>{breakState.message}</p>

          {!hasImage && (
            <div className={styles.defaultVisual}>
              <div className={styles.breatheIcon}>🌿</div>
              <span className={styles.breatheHint}>{t("overlay.breathe")}</span>
            </div>
          )}
        </div>

        {(!isStrictMode || isAttendance) ? (
          <button className={styles.skipButton} onClick={skipBreak}>
            {t("overlay.skip")}
          </button>
        ) : (
          <span className={styles.escHint}>{t("overlay.escEmergency")}</span>
        )}
      </div>
    </div>
  );
}
