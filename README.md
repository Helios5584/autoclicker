# AutoClicker

Lightweight macOS autoclicker. Runs as menu bar app. Rust + `core-graphics`.

## Features

- Menu bar tray icon (no dock, no window)
- Global hotkey: `⌘⇧Esc` toggles clicking on/off
- Selectable click speed: 1, 5, 10, 20, 50, 100 CPS
- Clicks at current cursor position
- Release binary ~1–2 MB (LTO + `opt-level = "z"`)

## Requirements

- macOS (Apple Silicon or Intel)
- Rust toolchain (`rustup`)
- Accessibility permission — macOS prompts on first run. Grant under **System Settings → Privacy & Security → Accessibility**.

## Build
NOTE: No need to build. .app file already present.
```bash
./bundle.sh           # release build, produces AutoClicker.app
./bundle.sh debug     # debug build
```

Run:

```bash
open AutoClicker.app
```

Install system-wide:

```bash
mv AutoClicker.app /Applications/
```

## Usage

1. Launch app — icon appears in menu bar.
2. Pick speed under **Click Speed** (default 10 CPS).
3. Press `⌘⇧Esc` to start, again to stop. Or use **Start/Stop Clicking** menu item.
4. **Quit** from tray menu to exit.

Clicks fire at wherever the cursor is when toggled on.

## Project layout

| File | Purpose |
|------|---------|
| `src/main.rs` | Tray, hotkey, click loop |
| `Cargo.toml` | Deps: `tao`, `tray-icon`, `global-hotkey`, `core-graphics` |
| `Info.plist` | Bundle metadata, `LSUIElement` hides dock icon |
| `bundle.sh` | Wraps compiled binary into `.app` |

## Regenerate icon

`icon.icns` is committed. To rebuild from `make_icon.swift`:

```bash
swift make_icon.swift icon_1024.png
mkdir -p icon.iconset
for s in 16 32 64 128 256 512 1024; do
    sips -z $s $s icon_1024.png --out icon.iconset/icon_${s}x${s}.png
done
cp icon.iconset/icon_32x32.png   icon.iconset/icon_16x16@2x.png
cp icon.iconset/icon_64x64.png   icon.iconset/icon_32x32@2x.png
cp icon.iconset/icon_256x256.png icon.iconset/icon_128x128@2x.png
cp icon.iconset/icon_512x512.png icon.iconset/icon_256x256@2x.png
cp icon.iconset/icon_1024x1024.png icon.iconset/icon_512x512@2x.png
rm icon.iconset/icon_64x64.png icon.iconset/icon_1024x1024.png
iconutil -c icns icon.iconset -o icon.icns
rm -rf icon.iconset icon_1024.png
```

## TODO

- Configurable hotkey
- Right-click / middle-click options
