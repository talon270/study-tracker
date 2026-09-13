#!/bin/sh
# Assemble the runtime frontend into src-tauri/dist for Tauri to bundle.
#
# The app itself still runs from file:// at the project root, untouched. This
# copy exists only because Tauri's frontendDist must not contain src-tauri/ —
# so we hand it a curated folder with just the runtime assets: the single
# index.html plus the manifest, service worker and icon it references. No
# backups, no PLANs, no README, no Rust. Rebuilt clean on every build; gitignored.
set -e
ROOT="$(CDPATH= cd "$(dirname "$0")/.." && pwd)"
DIST="$ROOT/src-tauri/dist"

rm -rf "$DIST"
mkdir -p "$DIST"
cp \
  "$ROOT/index.html" \
  "$ROOT/manifest.webmanifest" \
  "$ROOT/sw.js" \
  "$ROOT/icon.svg" \
  "$DIST/"
