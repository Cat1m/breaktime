import { useState, useEffect, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { BreakStartPayload, BreakTickPayload } from "../../shared/types";

interface BreakState {
  active: boolean;
  breakType: "mini" | "long" | "attendance" | null;
  remainingSecs: number;
  totalDuration: number;
  message: string;
  imageBase64: string | null;
}

const EMPTY_STATE: BreakState = {
  active: false,
  breakType: null,
  remainingSecs: 0,
  totalDuration: 0,
  message: "",
  imageBase64: null,
};

export function useBreakEvents(): BreakState {
  const [state, setState] = useState<BreakState>(EMPTY_STATE);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const clearCountdown = useCallback(() => {
    if (intervalRef.current !== null) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const activateBreak = useCallback((payload: BreakStartPayload) => {
    clearCountdown();

    setState({
      active: true,
      breakType: payload.break_type as "mini" | "long" | "attendance",
      remainingSecs: payload.duration_secs,
      totalDuration: payload.duration_secs,
      message: payload.message,
      imageBase64: payload.image_base64,
    });

    // Start local countdown
    intervalRef.current = setInterval(() => {
      setState((prev) => {
        if (prev.remainingSecs <= 1) {
          if (intervalRef.current !== null) {
            clearInterval(intervalRef.current);
            intervalRef.current = null;
          }
          invoke("skip_break").catch(console.error);
          return { ...prev, remainingSecs: 0 };
        }
        return { ...prev, remainingSecs: prev.remainingSecs - 1 };
      });
    }, 1000);
  }, [clearCountdown]);

  useEffect(() => {
    // 1. Signal Rust that this overlay is ready
    emit("overlay:ready").catch(console.error);

    // 2. Fetch current break state (handles late-mounting windows)
    invoke<BreakStartPayload | null>("get_active_break")
      .then((payload) => {
        if (payload) {
          activateBreak(payload);
        }
      })
      .catch(console.error);

    // 3. Listen for break:start event (handles early-mounting windows)
    const unlistenStart = listen<BreakStartPayload>("break:start", (event) => {
      activateBreak(event.payload);
    });

    // 4. Listen to break:tick (server-side tick, override local countdown)
    const unlistenTick = listen<BreakTickPayload>("break:tick", (event) => {
      setState((prev) => ({
        ...prev,
        remainingSecs: event.payload.remaining_secs,
      }));
    });

    // 5. Listen to break:end
    const unlistenEnd = listen("break:end", () => {
      clearCountdown();
      setState(EMPTY_STATE);
    });

    return () => {
      clearCountdown();
      unlistenStart.then((fn) => fn());
      unlistenTick.then((fn) => fn());
      unlistenEnd.then((fn) => fn());
    };
  }, [activateBreak, clearCountdown]);

  return state;
}
