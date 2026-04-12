# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Sipping** is a break reminder / hydration app built with **Tauri v2** (Rust backend + React/TypeScript frontend). It shows fullscreen overlay windows on all monitors when break timers fire, with a system tray icon for control.

## Build & Run Commands

```bash
# Development (starts Vite dev server + Tauri app)
npm run tauri dev

# Production build (outputs installer to src-tauri/target/release/bundle/)
npm run tauri build

# Frontend only (Vite dev server on port 1420)
npm run dev

# Type-check + build frontend
npm run build

# Rust checks only
cd src-tauri && cargo check
cd src-tauri && cargo clippy
cd src-tauri && cargo test
```

## Architecture

### Two-process Tauri v2 app

- **Rust backend** (`src-tauri/src/`): Timer loop, system tray, settings persistence, platform-specific APIs (idle detection, DND, audio)
- **React frontend** (`src/`): Settings panel UI and break overlay UI, served via Vite

### Window routing

A single `index.html` entry point serves both window types. `App.tsx` reads `?window=overlay` from URL params to switch between `<SettingsPanel />` and `<BreakOverlay />`. The Rust backend creates overlay windows dynamically (one per monitor) with this URL param.

### Rust backend structure (`src-tauri/src/`)

- `core/` — Shared infrastructure: `state.rs` (AppState = `Arc<Mutex<AppStateInner>>` with timer counters + settings), `events.rs` (event name constants + payload structs), `l10n.rs` (hardcoded i18n via match), `error.rs`
- `features/` — Feature modules, each with `mod.rs`, `service.rs`, `commands.rs`:
  - `timer/` — 1-second tick loop (`start_timer_loop`), break triggering, overlay window creation/destruction
  - `settings/` — JSON persistence to `dirs::config_dir()/sipping/sipping-settings.json`, `model.rs` defines `Settings` struct
  - `audio/` — Sound playback via `rodio`
  - `idle/` — Platform-specific idle time detection (Windows `GetLastInputInfo`, macOS `CoreGraphics`, Linux `XScreenSaver`)
  - `dnd/` — Do Not Disturb detection (Windows Focus Assist registry, macOS `defaults read`, Linux stub)
  - `image_loader/` — Custom background image loading + base64 encoding with caching

### Frontend structure (`src/`)

- `contexts/` — React Context providers: `SettingsContext`, `LocaleContext`, `TimerContext`
- `features/overlay/` — Break overlay with frosted glass effect, countdown ring, adaptive text color
- `features/settings/` — Settings panel UI
- `features/tray/` — Tray menu hook
- `shared/` — Reusable components (`Button`, `Toggle`, `NumberInput`, `CountdownRing`), hooks (`useAudio`, `useIdleStatus`), `types.ts` (shared TypeScript interfaces mirroring Rust structs)
- `locales/` — i18n JSON files (`en.json`, `vi.json`)

### Event-driven communication (Rust ↔ Frontend)

Events defined in `core/events.rs`: `break:start`, `break:end`, `break:tick`, `timer:tick`, `timer:status-changed`, `idle:changed`, `settings:changed`. Frontend listens via `@tauri-apps/api`. The overlay window fetches `current_break_payload` from state on mount via the `get_active_break` command as a fallback if it misses the event.

### Platform-specific code

Windows-specific dependencies are gated with `#[cfg(windows)]` / `[target.'cfg(windows)'.dependencies]` in Cargo.toml. Similar gates exist for macOS (`core-graphics`) and Linux (`x11`). The primary development target is Windows.

## Key Conventions

- Settings model must stay in sync between `src-tauri/src/features/settings/model.rs` (Rust) and `src/shared/types.ts` (TypeScript)
- Rust l10n for tray/tooltip is hardcoded in `core/l10n.rs`; frontend l10n uses JSON files in `src/locales/`
- CSS Modules (`.module.css`) are used throughout the frontend — no global CSS framework
- Tauri capabilities for window permissions are in `src-tauri/capabilities/default.json`; overlay windows use wildcard label `overlay-*`
