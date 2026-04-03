import { useState, useRef } from "react";
import styles from "./NumberInput.module.css";

interface NumberInputProps {
  value: number;
  onChange: (value: number) => void;
  label: string;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
}

export function NumberInput({
  value,
  onChange,
  label,
  min,
  max,
  step = 1,
  unit,
}: NumberInputProps) {
  // Local draft: null = not editing, string = user is typing
  const [draft, setDraft] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const clamp = (val: number) => {
    if (min !== undefined && val < min) return min;
    if (max !== undefined && val > max) return max;
    return val;
  };

  const commitDraft = () => {
    if (draft === null) return;
    const parsed = parseFloat(draft);
    if (!isNaN(parsed)) {
      onChange(clamp(parsed));
    }
    // If NaN (empty/invalid), just revert to current value
    setDraft(null);
  };

  const handleFocus = () => {
    setDraft(String(value));
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Just update local string — no parse, no clamp, no save
    setDraft(e.target.value);
  };

  const handleBlur = () => {
    commitDraft();
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      commitDraft();
      inputRef.current?.blur();
    }
    if (e.key === "Escape") {
      setDraft(null); // revert
      inputRef.current?.blur();
    }
  };

  // Buttons: instant clamp + save (no draft involved)
  const increment = () => onChange(clamp(value + step));
  const decrement = () => onChange(clamp(value - step));

  return (
    <div className={styles.container}>
      <label className={styles.label}>{label}</label>
      <div className={styles.inputRow}>
        <button className={styles.button} onClick={decrement} type="button">
          -
        </button>
        <input
          ref={inputRef}
          className={styles.input}
          type="number"
          value={draft !== null ? draft : value}
          min={min}
          max={max}
          step={step}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
        />
        <button className={styles.button} onClick={increment} type="button">
          +
        </button>
        {unit && <span className={styles.unit}>{unit}</span>}
      </div>
    </div>
  );
}
