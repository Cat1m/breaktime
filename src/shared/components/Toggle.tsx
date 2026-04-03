import styles from "./Toggle.module.css";

interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label: string;
  disabled?: boolean;
}

export function Toggle({ checked, onChange, label, disabled = false }: ToggleProps) {
  const handleClick = () => {
    if (!disabled) {
      onChange(!checked);
    }
  };

  return (
    <div
      className={styles.container}
      onClick={handleClick}
      style={{ opacity: disabled ? 0.5 : 1, cursor: disabled ? "not-allowed" : "pointer" }}
    >
      <span className={styles.label}>{label}</span>
      <div
        role="switch"
        aria-checked={checked}
        aria-disabled={disabled}
        tabIndex={0}
        className={[styles.track, checked ? styles.trackChecked : ""].filter(Boolean).join(" ")}
        onKeyDown={(e) => {
          if (e.key === " " || e.key === "Enter") {
            e.preventDefault();
            handleClick();
          }
        }}
      >
        <div
          className={[styles.thumb, checked ? styles.thumbChecked : ""].filter(Boolean).join(" ")}
        />
      </div>
    </div>
  );
}
