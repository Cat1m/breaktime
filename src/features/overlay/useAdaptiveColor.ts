import { useState, useEffect, useRef } from "react";

interface AdaptiveColorResult {
  cssVars: Record<string, string>;
}

// Linearize sRGB channel for luminance calculation
function linearize(c: number): number {
  const s = c / 255;
  return s <= 0.03928 ? s / 12.92 : Math.pow((s + 0.055) / 1.055, 2.4);
}

function relativeLuminance(r: number, g: number, b: number): number {
  return 0.2126 * linearize(r) + 0.7152 * linearize(g) + 0.0722 * linearize(b);
}

function rgbToHsl(r: number, g: number, b: number): [number, number, number] {
  r /= 255; g /= 255; b /= 255;
  const max = Math.max(r, g, b), min = Math.min(r, g, b);
  const l = (max + min) / 2;
  if (max === min) return [0, 0, l];
  const d = max - min;
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = 0;
  if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
  else if (max === g) h = ((b - r) / d + 2) / 6;
  else h = ((r - g) / d + 4) / 6;
  return [h * 360, s, l];
}

// Default scheme for dark backgrounds (current look)
const DARK_BG_SCHEME: Record<string, string> = {
  "--overlay-text-primary": "rgba(255,255,255,0.9)",
  "--overlay-text-secondary": "rgba(255,255,255,0.6)",
  "--overlay-text-muted": "rgba(255,255,255,0.4)",
  "--overlay-text-faint": "rgba(255,255,255,0.3)",
  "--overlay-btn-bg": "rgba(255,255,255,0.15)",
  "--overlay-btn-border": "rgba(255,255,255,0.3)",
  "--overlay-btn-hover-bg": "rgba(255,255,255,0.28)",
  "--overlay-btn-text": "white",
  "--overlay-ring-track": "rgba(255,255,255,0.2)",
  "--overlay-ring-progress": "#818cf8",
  "--overlay-ring-time": "white",
  "--overlay-text-shadow": "0 1px 4px rgba(0,0,0,0.5)",
  "--overlay-scrim-bg": "rgba(0,0,0,0.45)",
};

const LIGHT_BG_SCHEME: Record<string, string> = {
  "--overlay-text-primary": "rgba(15,23,42,0.95)",
  "--overlay-text-secondary": "rgba(15,23,42,0.6)",
  "--overlay-text-muted": "rgba(15,23,42,0.4)",
  "--overlay-text-faint": "rgba(15,23,42,0.3)",
  "--overlay-btn-bg": "rgba(0,0,0,0.12)",
  "--overlay-btn-border": "rgba(0,0,0,0.25)",
  "--overlay-btn-hover-bg": "rgba(0,0,0,0.22)",
  "--overlay-btn-text": "rgba(15,23,42,0.9)",
  "--overlay-ring-track": "rgba(0,0,0,0.15)",
  "--overlay-ring-progress": "#818cf8",
  "--overlay-ring-time": "rgba(15,23,42,0.95)",
  "--overlay-text-shadow": "0 1px 4px rgba(255,255,255,0.3)",
  "--overlay-scrim-bg": "rgba(0,0,0,0.25)",
};

function analyzeImage(img: HTMLImageElement): Record<string, string> {
  const SAMPLE_SIZE = 64;
  const canvas = document.createElement("canvas");
  canvas.width = SAMPLE_SIZE;
  canvas.height = SAMPLE_SIZE;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) return DARK_BG_SCHEME;

  ctx.drawImage(img, 0, 0, SAMPLE_SIZE, SAMPLE_SIZE);

  // Sample center 50% region
  const x0 = Math.floor(SAMPLE_SIZE * 0.25);
  const y0 = Math.floor(SAMPLE_SIZE * 0.25);
  const w = Math.floor(SAMPLE_SIZE * 0.5);
  const h = Math.floor(SAMPLE_SIZE * 0.5);
  const data = ctx.getImageData(x0, y0, w, h).data;
  const pixelCount = w * h;

  let totalLum = 0;
  let hueSin = 0, hueCos = 0, totalSat = 0;

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i], g = data[i + 1], b = data[i + 2];
    totalLum += relativeLuminance(r, g, b);

    const [hue, sat] = rgbToHsl(r, g, b);
    const hueRad = (hue * Math.PI) / 180;
    hueSin += Math.sin(hueRad) * sat; // weight by saturation
    hueCos += Math.cos(hueRad) * sat;
    totalSat += sat;
  }

  const avgLum = totalLum / pixelCount;
  const avgSat = totalSat / pixelCount;

  // Effective luminance after scrim
  // For dark-bg scheme scrim=0.45, for light-bg scrim=0.25
  // Use the dark-bg scrim to test threshold first
  const effLumDarkScrim = avgLum * (1 - 0.45);
  const isLightBg = effLumDarkScrim > 0.45;

  // Pick base scheme
  const scheme = { ...(isLightBg ? LIGHT_BG_SCHEME : DARK_BG_SCHEME) };

  // Compute accent color from average hue
  if (avgSat > 0.1) {
    const avgHue = ((Math.atan2(hueSin, hueCos) * 180) / Math.PI + 360) % 360;
    const [accentSat, accentLight] = isLightBg ? [65, 42] : [70, 72];
    scheme["--overlay-ring-progress"] = `hsl(${Math.round(avgHue)}, ${accentSat}%, ${accentLight}%)`;
  }

  return scheme;
}

export function useAdaptiveColor(imageBase64: string | null): AdaptiveColorResult {
  const [cssVars, setCssVars] = useState<Record<string, string>>(DARK_BG_SCHEME);
  const prevBase64Ref = useRef<string | null>(null);

  useEffect(() => {
    // No image → default dark scheme
    if (!imageBase64) {
      setCssVars(DARK_BG_SCHEME);
      prevBase64Ref.current = null;
      return;
    }

    // Same image → skip recomputation
    if (imageBase64 === prevBase64Ref.current) return;
    prevBase64Ref.current = imageBase64;

    const img = new Image();
    img.onload = () => {
      try {
        setCssVars(analyzeImage(img));
      } catch {
        setCssVars(DARK_BG_SCHEME);
      }
    };
    img.onerror = () => setCssVars(DARK_BG_SCHEME);
    img.src = `data:image/jpeg;base64,${imageBase64}`;
  }, [imageBase64]);

  return { cssVars };
}
