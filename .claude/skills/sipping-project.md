---
description: >
  Onboard to the Sipping project — a Tauri v2 desktop break reminder / hydration app
  (Rust backend + React/TypeScript frontend). Use this skill when starting a new session
  to work on Sipping, adding features, fixing bugs, or understanding the codebase.
  Triggers: "sipping", "breaktime", "break reminder app", "tiếp tục project", "continue sipping"
argument-hint: "<optional: specific area to focus on, e.g. 'overlay', 'settings', 'timer'>"
---

# Sipping Project — Quick Onboard

You are continuing work on **Sipping**, a desktop break reminder app.

## First Steps

1. Read `D:\my_project\rust\breaktime\CLAUDE.md` — it has the full architecture, build commands, conventions, and key patterns
2. If the user mentions a specific area, read the relevant files:
   - **Overlay/break screen**: `src/features/overlay/BreakOverlay.tsx`, `useAdaptiveColor.ts`, `useBreakEvents.ts`
   - **Settings UI**: `src/features/settings/SettingsPanel.tsx`, `useSettings.ts`
   - **Timer/backend**: `src-tauri/src/features/timer/service.rs`, `commands.rs`
   - **Tray menu**: `src-tauri/src/main.rs` (setup_tray, rebuild_tray_menu)
   - **Settings model**: `src-tauri/src/features/settings/model.rs` + `src/shared/types.ts` (must stay in sync)
   - **Localization**: `src/locales/en.json`, `vi.json` + `src-tauri/src/core/l10n.rs`
   - **State**: `src-tauri/src/core/state.rs` (AppStateInner)

## Project Location

- **Root**: `D:\my_project\rust\breaktime\` (folder named breaktime, app named Sipping)
- **GitHub**: https://github.com/Cat1m/breaktime

## Quick Build

```bash
cd D:\my_project\rust\breaktime
npm run tauri dev          # dev mode
npm run tauri build        # release → src-tauri/target/release/bundle/nsis/
npx tsc --noEmit           # typecheck frontend
cd src-tauri && cargo check # check rust
```

## Key Patterns to Remember

- Settings changes: update BOTH Rust `model.rs` AND TypeScript `types.ts`
- New l10n strings: add to BOTH `en.json` + `vi.json`, and `l10n.rs` if backend
- Overlay windows: one per monitor, label pattern `overlay-*`
- Tray menu: rebuild via `update_tray_pause_label()` or `rebuild_tray_menu()`
- CSS: all via CSS Modules (`.module.css`), overlay uses CSS variables from `useAdaptiveColor`
- Image: cached in `AppStateInner.cached_image`, preloaded on settings save
- Timer: Rust tokio loop, NOT JavaScript setInterval — events bridge to frontend

## User Preferences

- Vietnamese speaker, communicate in Vietnamese when appropriate
- Prefers plan-before-code for non-trivial features
- Likes minimal, clean UI — frosted glass aesthetic for overlay
- OCD about pixel alignment — check visual details carefully
- Always verify both `cargo check` and `npx tsc --noEmit` before declaring done
