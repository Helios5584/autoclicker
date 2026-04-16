AutoClicker — macOS Menu Bar App
Project: autoclicker/

Files
Cargo.toml — dependencies (tao, tray-icon, global-hotkey, core-graphics)
src/main.rs — event loop, tray icon, menu, hotkey registration
src/clicker.rs — CoreGraphics mouse click simulation
Info.plist — LSUIElement=true (no dock icon)
bundle.sh — builds .app bundle
Features
Menu bar only — no dock icon, just tray icon with dropdown
Toggle — "Start/Stop Clicking" menu item
Speed selection — 1, 5, 10 (default), 20, 50, 100 CPS with checkmarks
Emergency off — Cmd+Shift+Escape kills clicking instantly (one-way off, not toggle)
Background clicker thread — uses CoreGraphics HIDSystemState source to avoid interfering with accessibility
Menu layout

Start Clicking
Click Speed >
  ✓ 10 CPS  (and 5 other options)
────────────
Emergency Off: ⌘⇧Esc  (info, disabled)
────────────
Quit
Running

cd autoclicker
cargo run          # debug
./bundle.sh        # creates AutoClicker.app
open AutoClicker.app
First-run requirement
macOS will ask for Accessibility permissions (System Settings > Privacy & Security > Accessibility). Required for CoreGraphics click simulation.
