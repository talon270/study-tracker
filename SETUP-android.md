# Study Tracker on Android

The phone app is the same `index.html` in a Tauri v2 Android shell, built from
the same `src-tauri/` project as the desktop app. What the phone adds is
alarms that ring with the app closed: the end of a block, the end of a break,
and the daily reminder. Every one of them is handed to Android when it's
planned, not fired by a page timer.

**Drafted 2026-09-29, before the first install.** A signed APK has been built
and its certificate checked. B3 ran on the Nothing 2a on 2026-09-29; B7 has not.
Any step or behaviour here that hasn't been seen on a device is marked
**UNCONFIRMED**. Those marks come off as B7 reports.

## What you need

| | |
|---|---|
| Toolchain | JDK 17, Android SDK in `~/Android/Sdk` (platform 36, build-tools 35.0.0, NDK 27.2.12479018), the four Rust Android targets, `tauri-cli` 2.11.4. All installed by Helth's B1. `~/.config/fish/conf.d/android.fish` sets `JAVA_HOME`, `ANDROID_HOME` and `NDK_HOME` |
| Signing key | `~/.android-keys/wellness-hub.jks` and `keystore.properties`, the same key Wellness Hub uses. Outside the repo and outside `SyncedWork` |
| A phone | Android 10 or later, on the same Wi-Fi as this machine for wireless debugging |

**Back up `~/.android-keys/` somewhere that isn't this machine or
`SyncedWork`.** `adb install -r` only updates an app signed with the same key.
With a different key it's an uninstall and reinstall, which deletes the
phone's `localStorage`. Sync keeps a copy, but anything the phone logged and
hadn't uploaded yet is gone. That key now signs two apps.

## Build

From the project root, in fish:

```sh
cargo tauri android build --apk --target aarch64
```

`beforeBuildCommand` runs `src-tauri/copy-assets.sh` first, so the APK carries
whatever `index.html` is on disk. The output is:

```
src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

Check it was signed with your key and not Gradle's debug key:

```sh
$ANDROID_HOME/build-tools/35.0.0/apksigner verify --print-certs \
  src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

The SHA-256 must be `ed654669…1427a22c`, `CN=Wellness Hub, O=Talon, C=IN`.
If `keystore.properties` is missing, `build.gradle.kts` skips the signing
block and the APK comes out unsigned, which `adb install` refuses.

A full release build writes a lot. Send it to a file and search that:
`cargo tauri android build --apk --target aarch64 > /tmp/st-build.log 2>&1`.

## Pair (once per phone, and again after a reboot)

The Nothing Phone (2a)'s USB port carries no data, so everything goes over
Wi-Fi.

1. On the phone: Settings → About phone → tap **Build number** 7 times.
2. Developer options → **Wireless debugging** on → **Pair device with pairing
   code**. It shows an IP:port and a six-digit code.
3. Here:

   ```sh
   adb pair 192.168.1.23:41235 123456
   adb connect 192.168.1.23:37011
   adb devices
   ```

   The `connect` port is the one on the Wireless debugging screen itself, not
   the one in the pairing dialog. They differ.

Android turns wireless debugging off after a reboot or a Wi-Fi change. Turn it
back on and `adb connect` to the new port. Pairing is remembered.

## Install and update

```sh
adb install -r src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

Updating is the same command after a new build. `-r` keeps the app's data as
long as the key matches. There's no in-app updater on the phone, so the
desktop "Check for updates" card is hidden there (A8).

## First run on the phone

**The app starts empty.** Its WebView is a new origin, so it can't see what
the phone's browser or PWA stored.

1. Sign in to sync (`SETUP-sync.md`). Your history arrives from the cloud.
   Credentials are typed on the phone and never pass through a build.
2. The first-run card suggests timer alerts and the daily reminder, each
   ticked, each with its reason. Untick what you don't want, then allow
   notifications at Android's prompt.
3. Battery: set Study Tracker to **Unrestricted** (Android 15+: **Allow
   background usage**). On a Samsung, also add it to **Never sleeping apps**.
   Settings → This phone links to the right screen and shows what's granted.

If the PWA on the phone has sessions that never synced, sign it into sync
first, or export JSON from the browser, where downloads work. Then uninstall
the PWA. **Export from inside the app saves nothing.** B3 clicked both
buttons: the app said "Downloaded …" and `/sdcard/Download` held 73 files
before and 73 after. Don't treat that message as a backup. Sync, or export
from the laptop.

## Reading the phone's console

```sh
adb shell cat /proc/net/unix | grep webview_devtools_remote
adb forward tcp:9222 localabstract:webview_devtools_remote_<pid>
```

Then open `chrome://inspect` on this machine, or connect Playwright to
`http://localhost:9222`. For native crashes, `adb logcat -d > /tmp/st.log`
and search the file.

## What the phone does and doesn't promise

These limits come from Wellness Hub's tests on the 2a (2026-09-23). The same
shell and plugin fork are in this app. Study Tracker's own on-device results
are B7's, still to run.

| Limit | What you see |
|---|---|
| **After a force-stop, nothing rings until you open the app again** | Android wipes every alarm an app owns on force-stop. Opening the app re-arms them. Swiping it out of recents is not a force-stop |
| **After a reboot, alarms return only once you've unlocked the phone** | Before the first unlock, nothing rings |
| **The countdown isn't on the AOD** | It's on the lock screen and in the shade. The 2a's firmware doesn't allow promoted notifications. The flag stays set, so it becomes a Live Update if an update allows it |
| **Reminders run 7 days ahead** | Each launch re-arms the next 7. Leave the app unopened for a week and they stop. The last one says "Open Study Tracker to keep reminders coming." |
| **Only blocks started on the phone ring the phone** | A block started on the laptop never leaves the laptop. Alerting it here would need a push server |
| **The focus face is not a real AOD** | While it counts, the screen is on and draws more than Nothing's AOD would. The phone doesn't auto-lock. Power turns it off, and the face comes back over the lock screen at the next wake |
| **A tap that launches the app from closed: UNCONFIRMED** | Helth only tested taps with the app already running. B7 tries it |
| **Ndot digits on the face: UNCONFIRMED off the 2a** | The face borrows the phone's own Ndot through `local()`. Nothing's fonts are proprietary and never ship in this repo. Every other phone gets Doto (OFL) |
| **The header sits under the status bar** | B3's screenshot shows the clock and camera cutout over the title. The WebView does report the inset (`env(safe-area-inset-top)` = 48px), so the fix is one CSS line, not yet applied |

Quiet hours apply to alarms too. A block that ends inside them gets its
countdown and no alert, as `cue()` already behaved on desktop.
