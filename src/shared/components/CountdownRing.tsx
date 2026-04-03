import styles from "./CountdownRing.module.css";

interface CountdownRingProps {
  remainingSecs: number;
  totalSecs: number;
  size?: number;
}

function formatTime(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export function CountdownRing({ remainingSecs, totalSecs, size = 200 }: CountdownRingProps) {
  const strokeWidth = 8;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const progress = totalSecs > 0 ? remainingSecs / totalSecs : 0;
  const dashoffset = circumference * (1 - progress);
  const center = size / 2;

  // Add 2px padding in viewBox to prevent anti-alias clipping at edges
  const vbPad = 2;
  const vbSize = size + vbPad * 2;
  const vbOffset = -vbPad;

  return (
    <div className={styles.container} style={{ width: size, height: size }}>
      <svg
        className={styles.svg}
        width={size}
        height={size}
        viewBox={`${vbOffset} ${vbOffset} ${vbSize} ${vbSize}`}
        overflow="visible"
      >
        <circle
          className={`${styles.track} countdown-ring-track`}
          cx={center}
          cy={center}
          r={radius}
          strokeWidth={strokeWidth}
        />
        <circle
          className={`${styles.progress} countdown-ring-progress`}
          cx={center}
          cy={center}
          r={radius}
          strokeWidth={strokeWidth}
          strokeDasharray={circumference}
          strokeDashoffset={dashoffset}
        />
      </svg>
      <span className={`${styles.time} countdown-ring-time`}>{formatTime(remainingSecs)}</span>
    </div>
  );
}
