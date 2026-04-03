import { createContext, useContext, useMemo, ReactNode } from "react";
import { useSettingsContext } from "./SettingsContext";
import en from "../locales/en.json";
import vi from "../locales/vi.json";

const locales: Record<string, Record<string, string>> = { en, vi };

interface LocaleContextValue {
  t: (key: string) => string;
  language: "en" | "vi";
}

const LocaleContext = createContext<LocaleContextValue | null>(null);

export function LocaleProvider({ children }: { children: ReactNode }) {
  const { settings } = useSettingsContext();
  const language = settings?.language ?? "en";

  const value = useMemo(() => {
    const messages = locales[language] || locales.en;
    const t = (key: string): string => messages[key] ?? locales.en[key] ?? key;
    return { t, language };
  }, [language]);

  return (
    <LocaleContext.Provider value={value}>
      {children}
    </LocaleContext.Provider>
  );
}

export function useLocale(): LocaleContextValue {
  const ctx = useContext(LocaleContext);
  if (!ctx) {
    throw new Error("useLocale must be used within LocaleProvider");
  }
  return ctx;
}
