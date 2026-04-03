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
  const clamp = (val: number) => {
    if (min !== undefined && val < min) return min;
    if (max !== undefined && val > max) return max;
    return val;
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const parsed = parseFloat(e.target.value);
    if (!isNaN(parsed)) {
      onChange(clamp(parsed));
    }
  };

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
          className={styles.input}
          type="number"
          value={value}
          min={min}
          max={max}
          step={step}
          onChange={handleChange}
        />
        <button className={styles.button} onClick={increment} type="button">
          +
        </button>
        {unit && <span className={styles.unit}>{unit}</span>}
      </div>
    </div>
  );
}
