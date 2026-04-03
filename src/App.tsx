import { SettingsProvider } from "./contexts/SettingsContext";
import { LocaleProvider } from "./contexts/LocaleContext";
import { TimerProvider } from "./contexts/TimerContext";
import { SettingsPanel } from "./features/settings/SettingsPanel";
import { BreakOverlay } from "./features/overlay/BreakOverlay";
import styles from "./App.module.css";

// Detect window type tu URL params
// Overlay window se duoc mo voi URL: index.html?window=overlay
const urlParams = new URLSearchParams(window.location.search);
const windowType = urlParams.get("window") || "settings";

function App() {
  return (
    <div className={styles.app}>
      <SettingsProvider>
        <LocaleProvider>
          <TimerProvider>
            {windowType === "overlay" ? <BreakOverlay /> : <SettingsPanel />}
          </TimerProvider>
        </LocaleProvider>
      </SettingsProvider>
    </div>
  );
}

export default App;
