# H.U.D Manager Overlay

Always-on-top desktop overlay for Star Citizen. Floats H.U.D Manager over the game in a draggable widget.

## Features

- **Draggable pill** — place it anywhere on screen; drag the grip on the left edge
- **Auto-fade** — fades to 28% opacity + passes clicks through to the game when idle
- **Quick-launch buttons** — ⛟ Trade · ⌖ Starmap · ◈ AI Assist directly from the pill
- **Global hotkey** — `Ctrl+Shift+H` toggles from anywhere, even while the game has focus
- **System tray** — lives in the tray; right-click to toggle or quit

## Install

Download `hud-manager-overlay_*_x64-setup.exe` from [Releases](../../releases) and run it.

## Dev Setup

Requires: [Rust](https://rustup.rs/) + [Node 20+](https://nodejs.org/) + [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
npm install
npm run dev        # local dev with hot-reload
npm run build      # produces installer in src-tauri/target/release/bundle/
```

## Release

Push a tag like `v1.0.1` → GitHub Actions builds and publishes the `.exe` installer automatically.

```bash
git tag v1.0.1 && git push origin v1.0.1
```
