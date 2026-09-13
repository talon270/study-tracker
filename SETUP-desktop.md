# Study Tracker as a desktop app

The tracker still is one `index.html` you can open from `file://`. The desktop
build wraps that same file in a native window so it lives in your app menu and
system tray — the same Tauri v2 shell as `Helth/src-tauri`, nothing forked.

**Why a native shell at all: reminders only fire while the JS is running.** The
pomodoro-end cue and the daily study reminder come from a `setInterval` tick in
`index.html`. A closed browser tab is a silent app. The shell is **tray-resident**
— closing the window hides it to the tray instead of quitting, the webview keeps
ticking, and the cue arrives as a real OS toast through
`tauri-plugin-notification`. Fully quit it from the tray menu and reminders stop
until you reopen it — that ceiling needs an OS scheduler or a push server, both
out of scope.

## What's in the shell

| File | What it is |
|---|---|
| `src-tauri/tauri.conf.json` | window (1200×900), `frontendDist: dist`, bundle targets |
| `src-tauri/Cargo.toml` | Rust deps: `tauri` + `tauri-plugin-notification`. No app logic |
| `src-tauri/src/main.rs` | the native window, the tray, close-to-tray, `run_update`/`relaunch`. ~125 lines |
| `src-tauri/capabilities/default.json` | window show/hide/focus + notification permissions |
| `src-tauri/copy-assets.sh` | copies `index.html` + manifest + `sw.js` + `icon.svg` into `dist/` |
| `src-tauri/install.sh` | drops the built binary + `.desktop` entry into `~/.local` (Linux) |

The JS side is three branches in `index.html`, all guarded on `window.__TAURI__`
so the browser/PWA is untouched: the notification `cue()` routes to the native
plugin, the Settings notify-toggle grants without a browser prompt, and a
Settings "Check for updates" button reruns the build below without a terminal.

## Updating without a terminal

Settings → Desktop app → **Check for updates** runs `copy-assets.sh`, `cargo
build --release` and `install.sh` in place (via `run_update` in `main.rs`,
plain `std::process::Command` — no shell plugin, no new permission surface)
and offers a **Relaunch now** toast that swaps the running process for the
freshly installed one. Same steps as the terminal commands below, same
~10–60s cargo wait; it needs the Rust toolchain that built the app in the
first place, and looks for `cargo` on `PATH` first, then
`~/.cargo/bin/cargo` (a GUI launch often doesn't inherit a login shell's
`PATH`) — anything more exotic than a stock rustup install still needs the
terminal path below.

## Build and install (Linux)

Icons are generated once from `icon.svg`, not committed:

```sh
rsvg-convert -w 1024 -h 1024 icon.svg -o /tmp/icon-1024.png
cargo tauri icon /tmp/icon-1024.png        # writes src-tauri/icons/
```

Then build and install:

```sh
cd src-tauri
cargo build --release                       # runs copy-assets.sh first
sh install.sh                               # -> ~/.local/bin/study-tracker + app menu
```

Re-run `cargo build --release && sh install.sh` after any edit to `index.html`.

## Windows

`cargo tauri build --bundles nsis` on a Windows box produces `study-tracker.exe`
+ an installer. WebView2 is already part of Win10/11, so no bundled browser.

## What this does not change

- **Data still lives in `localStorage`.** A fresh Tauri origin
  (`tauri://localhost`) starts empty — your history is in the browser's origin.
  Move it once with the Settings backup/restore export→import, or keep using the
  server sync if you have it configured (`SETUP-sync.md`).
- **The PWA still works.** Phone and browser open the same `index.html`; the
  shell is additive.
