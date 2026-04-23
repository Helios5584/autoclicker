#!/bin/bash
# Builds a macOS .app bundle from the compiled binary.
# Usage: ./bundle.sh [release|debug]

set -e
source "$HOME/.cargo/env" 2>/dev/null || true

MODE="${1:-release}"
if [ "$MODE" = "release" ]; then
    cargo build --release
    BINARY="target/release/autoclicker"
else
    cargo build
    BINARY="target/debug/autoclicker"
fi

APP="AutoClicker.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS"
mkdir -p "$APP/Contents/Resources"

cp "$BINARY" "$APP/Contents/MacOS/autoclicker"
cp Info.plist "$APP/Contents/Info.plist"

if [ -f icon.icns ]; then
    cp icon.icns "$APP/Contents/Resources/AppIcon.icns"
fi

# Ad-hoc codesign. Required on Apple Silicon for unsigned binaries to run at all.
# Does NOT bypass Gatekeeper on other machines — users must still strip quarantine.
codesign --force --deep --sign - "$APP"

echo "Built $APP ($MODE)"
echo ""
echo "To run:  open $APP"
echo "To install: mv $APP /Applications/"
echo ""
echo "NOTE: On first run, macOS will ask for Accessibility permissions."
echo "      Go to System Settings > Privacy & Security > Accessibility"
echo "      and grant access to AutoClicker."
echo ""
echo "If sharing the app with others, they will see \"app is damaged\" on first launch."
echo "They must strip the quarantine flag after downloading:"
echo "      xattr -dr com.apple.quarantine /path/to/AutoClicker.app"
