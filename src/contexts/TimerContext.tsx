import { createContext, useContext, useState, useEffect, ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { TimerStatus, TimerTickPayload } from "../shared/types";

interface TimerContextValue {
  timerStatus: TimerStatus;
  secsUntilMini: number;
  secsUntilLong: number;
  pauseTimer: () => Promise<void>;
  resumeTimer: () => Promise<void>;
  skipBreak: () => Promise<void>;
}

const TimerContext = createContext<TimerContextValue | null>(null);

export function TimerProvider({ children }: { children: ReactNode }) {
  const [timerStatus, setTimerStatus] = useState<TimerStatus>("running");
  const [secsUntilMini, setSecsUntilMini] = useState(0);
  const [secsUntilLong, setSecsUntilLong] = useState(0);

  useEffect(() => {
    invoke<string>("get_timer_status")
      .then((status) => setTimerStatus(status as TimerStatus))
      .catch(console.error);

    const unlistenStatus = listen<{ status: string }>("timer:status-changed", (event) => {
      setTimerStatus(event.payload.status as TimerStatus);
    });

    const unlistenTick = listen<TimerTickPayload>("timer:tick", (event) => {
      const { status, secs_until_mini, secs_until_long } = event.payload;
      setTimerStatus(status as TimerStatus);
      setSecsUntilMini(secs_until_mini);
      setSecsUntilLong(secs_until_long);
    });

    return () => {
      unlistenStatus.then((fn) => fn());
      unlistenTick.then((fn) => fn());
    };
  }, []);

  const pauseTimer = async () => {
    await invoke("pause_timer");
    setTimerStatus("paused");
  };

  const resumeTimer = async () => {
    await invoke("resume_timer");
    setTimerStatus("running");
  };

  const skipBreak = async () => {
    await invoke("skip_break");
    setTimerStatus("running");
  };

  return (
    <TimerContext.Provider value={{ timerStatus, secsUntilMini, secsUntilLong, pauseTimer, resumeTimer, skipBreak }}>
      {children}
    </TimerContext.Provider>
  );
}

export function useTimerContext(): TimerContextValue {
  const ctx = useContext(TimerContext);
  if (!ctx) {
    throw new Error("useTimerContext must be used within TimerProvider");
  }
  return ctx;
}
