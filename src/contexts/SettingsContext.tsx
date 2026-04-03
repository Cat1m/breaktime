import { createContext, useContext, useReducer, useEffect, ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../shared/types";

// State type
interface SettingsState {
  settings: Settings | null;
  loading: boolean;
  error: string | null;
}

// Actions
type SettingsAction =
  | { type: "LOAD_START" }
  | { type: "LOAD_SUCCESS"; payload: Settings }
  | { type: "LOAD_ERROR"; payload: string }
  | { type: "UPDATE_SETTINGS"; payload: Settings };

function settingsReducer(state: SettingsState, action: SettingsAction): SettingsState {
  switch (action.type) {
    case "LOAD_START":
      return { ...state, loading: true, error: null };
    case "LOAD_SUCCESS":
      return { settings: action.payload, loading: false, error: null };
    case "LOAD_ERROR":
      return { ...state, loading: false, error: action.payload };
    case "UPDATE_SETTINGS":
      return { ...state, settings: action.payload };
    default:
      return state;
  }
}

interface SettingsContextValue {
  settings: Settings | null;
  loading: boolean;
  error: string | null;
  saveSettings: (settings: Settings) => Promise<void>;
}

const SettingsContext = createContext<SettingsContextValue | null>(null);

export function SettingsProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(settingsReducer, {
    settings: null,
    loading: false,
    error: null,
  });

  useEffect(() => {
    dispatch({ type: "LOAD_START" });
    invoke<Settings>("get_settings")
      .then((settings) => dispatch({ type: "LOAD_SUCCESS", payload: settings }))
      .catch((err) =>
        dispatch({ type: "LOAD_ERROR", payload: String(err) })
      );
  }, []);

  const saveSettings = async (newSettings: Settings) => {
    try {
      await invoke("save_settings", { newSettings });
      dispatch({ type: "UPDATE_SETTINGS", payload: newSettings });
    } catch (err) {
      dispatch({ type: "LOAD_ERROR", payload: String(err) });
    }
  };

  return (
    <SettingsContext.Provider
      value={{
        settings: state.settings,
        loading: state.loading,
        error: state.error,
        saveSettings,
      }}
    >
      {children}
    </SettingsContext.Provider>
  );
}

export function useSettingsContext(): SettingsContextValue {
  const ctx = useContext(SettingsContext);
  if (!ctx) {
    throw new Error("useSettingsContext must be used within SettingsProvider");
  }
  return ctx;
}
