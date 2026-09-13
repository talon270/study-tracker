#!/bin/sh
# Install the built Study Tracker binary into the user's local prefix so it
# launches from the app menu like any installed app. No AppImage bundle needed:
# this machine already has webkit2gtk-4.1 system-wide, which the binary links
# against, so the raw release binary IS the install.
#
# Re-run this after every `cargo tauri build` (or plain `cargo build --release`)
# to update the installed copy.
set -e
ROOT="$(CDPATH= cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/src-tauri/target/release/study-tracker"
ICONS="$ROOT/src-tauri/icons"

[ -x "$BIN" ] || { echo "Build first: cd src-tauri && cargo build --release"; exit 1; }

BINDIR="$HOME/.local/bin"
APPS="$HOME/.local/share/applications"
ICONBASE="$HOME/.local/share/icons/hicolor"

mkdir -p "$BINDIR" "$APPS" "$ICONBASE/128x128/apps" "$ICONBASE/256x256/apps"

install -m755 "$BIN" "$BINDIR/study-tracker"
install -m644 "$ICONS/128x128.png" "$ICONBASE/128x128/apps/study-tracker.png"
install -m644 "$ICONS/icon.png"     "$ICONBASE/256x256/apps/study-tracker.png"

cat > "$APPS/study-tracker.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Study Tracker
Comment=Pomodoro timer, study heatmap and streaks
Exec=$BINDIR/study-tracker
Icon=study-tracker
Terminal=false
Categories=Utility;Education;
StartupWMClass=Study Tracker
EOF

# Refresh the menu/icon caches so it appears without a re-login.
update-desktop-database "$APPS" 2>/dev/null || true
gtk-update-icon-cache -f -t "$ICONBASE" 2>/dev/null || true

echo "Installed: $BINDIR/study-tracker  (launch 'Study Tracker' from your app menu)"
