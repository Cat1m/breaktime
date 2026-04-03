import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export function useIdleStatus() {
  const [isIdle, setIsIdle] = useState(false);

  useEffect(() => {
    // Listen to "idle:changed" event
    const unlisten = listen<{ is_idle: boolean }>("idle:changed", (event) => {
      setIsIdle(event.payload.is_idle);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return { isIdle };
}
