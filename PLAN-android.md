# Study Tracker on Android — plan

Written 2026-09-28, against `index.html` as of 4,815 lines (commit `3eb278f`)
and `src-tauri/` (the desktop shell, `b31953f`). The Android side is lifted
from Helth's build: `Helth/PLAN-android.md`, branch `android` at `b3c64d1`.

**Revised 2026-09-28, at your request:** the focus face is now the Today
ring, full screen on black, with its numerals in Nothing's Ndot 57. It
replaces Helth's dot row and is no longer optional. The new text is Part C,
A12 and B8. It was prototyped in Chromium at the 2a's CSS width (412 px) with
Doto standing in for Ndot. Revised again the same day: the ring gets a new
motion driver, shared with Today and measured before and after (Part C, "The
ring driver"). Revised 2026-09-29: the prototype renders in Nothing's own Ndot
55 Caps, from your local copy. The digits are laid out in fixed cells, because
Ndot 55 has no tabular figures (Part C, "Digit cells").

**Method.** A line-level read of Study's code:
- the timer: `persistActive` 3059, `startPhase` 3071, `finishWork` 3104,
  `recoverIfNeeded` 3156, and the Start/Pause/Stop handlers 4397–4441
- cues: `cue` 2987 and `tauriNotify` 3015
- the reminder: `reminderAt` / `reminderDue` / `fireReminder`, 3198–3220
- Settings: the notification switch and the updater card, 4529–4577
- `download` (4153) and service-worker registration (4789)
- `src-tauri/src/main.rs` and `Cargo.toml`

The same read on the Helth side:
- its plan, `src-tauri/src/lib.rs` and `Cargo.toml`
- `js/native.js`, and the permission code in `js/core.js` (1485–1570)
- `MainActivity.kt`, the manifest and the signing block in
  `app/build.gradle.kts`
- the six patch blocks in its vendored notification plugin

The Android toolchain was checked on this machine rather than assumed.

**What was run:** how a running block behaves when the page is frozen and when
it is killed. That was Playwright with real clicks on Start (A1's table). I
tried Chromium's CDP "frozen" lifecycle state first. In headless mode it
doesn't stop timers: 25 of 25 ticks ran during a 5 s "freeze". So the freeze
is simulated with Playwright's fake clock instead. The wall clock jumps and no
timer fires, which is exactly what a frozen page experiences.

Nothing has been run on a phone; `adb devices` lists none. Every device-side
claim is either one of Helth's on-device results (2026-09-23, cited where
used) or marked **UNCONFIRMED**.

**Nothing below is implemented. This is the plan.**

---

## What carries over from Helth

**Most of this project already exists.** Helth built the Android shell,
debugged it on the 2a and wrote down why each piece is shaped the way it is.
This plan reuses those pieces and changes only what Study's single timer and
single reminder need.

| Piece | Helth source | For Study |
|---|---|---|
| Toolchain | JDK 17.0.20, SDK platform 36, build-tools 35.0.0, NDK 27.2.12479018, the four Rust Android targets, `tauri-cli` 2.11.4, `~/.config/fish/conf.d/android.fish` | Nothing to install. Helth's B1 is done |
| Desktop/mobile split | `src-tauri/src/lib.rs`, target-gated `Cargo.toml` | Fewer gates: Study has no single-instance, autostart or `check_update` |
| Notification plugin fork | `src-tauri/plugins/notification` (2.4.0 plus six patches) | Copied verbatim (A5) |
| Android project edits | the `MainActivity.kt` bridge, manifest permissions, signing block, the `gen/android` `.gitignore` lines | Package `com.talon.studytracker` (already Study's identifier). Bridge exposed as `STNative` |
| Alarm planner | `js/native.js`: plan, diff, taps, bridge | A `NATIVE` section in `index.html`. It plans 1 countdown, 2 timer alarms and 7 reminder fires, not hundreds |
| Real permission state | `js/core.js` 1485–1570 | Replaces Study's switch branch at 4535–4539 |
| Phone Settings and first-run card | `js/views/settings.js`, `js/app.js` | One reminder instead of sixteen |
| Pseudo-AOD mechanics | `MainActivity.face()` / `awake()`, and from `js/face.js` the 3 px burn-in drift, the 5 s control fade and the 500 ms hold | Unchanged. The look is Study's own ring (Part C) |
| Nothing's fonts, never shipped | `css/hub.css`: `local("Ndot 57 Aligned")` and `local("Lettera Mono LL")`, both confirmed loading in the 2a's WebView, plus vendored Doto (OFL) as the fallback | The same names, and the same 3.9 KB Doto file (A12) |
| Signing key | `~/.android-keys/` | Reused (A7) |

**Carried over from Helth's interview unchanged:**

| Question | Answer |
|---|---|
| Wrapper | Tauri v2 Android, the same `src-tauri/` project as desktop. No Capacitor, no second shell |
| Sync | The existing Supabase transport, unchanged. The phone signs in, and the merge (tombstones included since `3eb278f`) pulls your history |
| Phones | Nothing Phone (2a) first. The 4a and the Galaxy M13 once they're paired |
| Getting builds on | Wireless debugging on the 2a (its port carries no data), then `adb install -r` of a signed APK |
| Which timers ring the phone | Only blocks started on the phone. This is already true: `activeSession` never leaves the device |
| Alert | A heads-up at the exact second, with sound and vibration. It respects Do Not Disturb |
| Live countdown | For the running block or break only. It shows on the lock screen and in the shade, not the AOD, on the 2a's firmware (Helth A13) |
| Reminder on/off | Per device, not synced. The time stays synced |
| Reminders while closed | Pre-scheduled natively, and re-armed whenever the app runs |
| First run | A suggestion card: each item ticked by default with its reason, and untickable |
| Tapping a notification | Opens Today, and the running countdown opens the focus face. Nothing logs from a notification |
| Work split | Opus orchestrates. `android-mech` (Sonnet, low) and `android-js` (Sonnet, medium) take the mechanical and UI steps |

**Decided for Study. Veto any of these at approval:**

| Question | Decision | Why |
|---|---|---|
| Where the work lives | An `android` branch, merged into `main` only when you say | `main` is the GitHub Pages deploy. The phone code is guarded, but a guard bug would reach the live PWA |
| Reminder horizon | 7 days (Helth uses 24 h) | One alarm a day uses 7 of Android's 500 slots. Helth's interval reminders use dozens a day |
| Signing key | Reuse Helth's keystore | One key to back up instead of two. It signs nothing but your own sideloaded apps |
| Plugin fork | A copy, not a path dependency | See A5 |
| Focus face | The Today ring on a pseudo-AOD, with digits in Ndot 57 (Part C) | You asked for it on 2026-09-28: the same circle as the website and the Linux app, in Nothing's type |
| Face colours | Your dark palette, exactly as Today wears it. No last-minute red | "The same circle." Today has no red state, and red is already the accent in Arasaka and Akira |
| Ring motion | A new ring driver for **both** the face and Today, so the website and the Linux app change too (Part C) | You asked for a more fluid circle. The two circles share one function, so they change together or not at all |

---

## Phase 0 — shape

1. **Who opens this and when?** You, on the 2a, starting a block where the
   laptop isn't the study surface (reading, problem sets on paper), and
   glancing at the lock screen for time left. It's the same log as the laptop
   and the PWA, through sync. *This is assumed from Helth's answers, not
   asked. Correct it if the phone is actually your main timer.*
2. **What must it never get wrong?** It must never lose a block you sat
   through. The app already refuses to credit time you didn't study. On a
   phone the opposite failure becomes the likely one, and A1 reproduces it.
   Second, a block-end alert that never comes looks exactly like a block
   that's still running.
3. **Honest confidence of its central claim?** The claim is "this block
   happened, and this phone will tell you when it ends."
   - A block the app didn't watch finish is logged with a note, and a
     Keep/Delete banner says so.
   - The reminder states how old the phone's knowledge is (A6).
   - Settings shows what the phone can actually back up: notification
     permission, exact alarms, battery exemption, and "alarms armed until Sun
     19:00". It never shows a bare tick.
4. **What happens when the source dies?** A failing sync degrades exactly
   as it does on desktop. `localStorage` stays canonical, and the alarms need
   no network. If the app stays unopened for 7 days, reminders stop. The last
   one says "Open Study Tracker to keep reminders coming", and Settings shows
   the horizon.

---

## Part A — findings and design risks, ranked

### A1 · BUG (high): Android killing the app deletes the block you were studying

Crash recovery records the running block with a 15-second heartbeat:
`persistActive` (3059), run from a `setInterval` (3079). Helth measured on the
2a that Android freezes the page within seconds of the screen going off, so
the heartbeat stops with it. If Android then kills the process (memory
pressure, or a swipe out of recents), `recoverIfNeeded` (3156) credits only
what the last heartbeat saw. Anything under 60 seconds is discarded.

Reproduced with a 25-minute block. Start was clicked, and the phone was next
looked at 40 minutes after Start:

| While locked | What gets logged |
|---|---|
| Page never frozen (desktop today) | 25 min |
| Frozen 30 s in, unlocked at 40 min | 25 min |
| Frozen 30 s in, then killed | **nothing** |
| Frozen 10 min in, then killed | 10 min, flagged interrupted |

The third row is the normal phone flow: press Start, lock the phone, study.
Whether you get the block depends on whether Android needed the memory. Once
A2 is built, the phone would say "Focus block done" at 25 minutes while the
log says nothing happened.

**Back is not the risk.** Wry's handler falls through to `onBackPressed()`.
From Android 12 on, that moves a root launcher activity to the background
instead of finishing it, which means a freeze, not a kill. That's Android 12's
documented behaviour change; B7 checks it on the device.

**Fix: on Android, restore the timer instead of recovering from a crash.**
`persistActive` already runs at every transition: start (3080) and
pause/resume (4423). It gains the fields that let the timer be rebuilt:
`phase`, `endAt`, `paused`, the remaining time at a pause, `lastResume`, the
work banked before the current run, `group` and `quick`. It also persists
breaks, not only work. On open, under Android:

| State at open | What happens |
|---|---|
| Block still running (`now < endAt`) | The timer carries on where it is. Nothing is logged yet |
| Block ended while the app was dead | Logged in full, ending at `endAt`. That's the same credit a frozen page gets. The existing banner is reworded: "Finished while the app was closed. It was never paused, so it counts in full. Delete it if you stopped early." |
| Paused | Restored as paused. The 30-minute abandon rule applies as it does now |
| A break | Resumed if still running. Dropped silently if it's over, since its alarm already fired |

The credit rule is the one the frozen page already applies (row 2), so a
frozen app and a killed app log the same block. Desktop and the browser keep
heartbeat recovery. There, closing the window is something you did, and
otherwise the tray keeps the page alive.

**No `SCHEMA_VERSION` bump.** `activeSession` is transient, device-local and
never synced. A record without `endAt` falls back to today's heartbeat
recovery. B4 verifies this by seeding an old-shape record.

**Two pre-existing defects in the same lines.** Both happen on every platform
today, and a phone makes them common. B4 includes them because it rewrites
this code anyway. Say if you'd rather keep them separate.
- **A recovered block loses its group.** Reproduced: a block on `C - Arrays`
  in group `C` recovers with group `''`, so its minutes leave "C" in the
  split and in the subject badges. `persistActive` never stores the group.
- **A block that ends while the page is asleep is dated to when it woke.** Row
  2 logs 25 min with `endedAt` 40.0 minutes after start. `finishWork` (3123)
  writes `Date.now()`, and it should write `min(now, endAt)`.

### A2 · DESIGN RISK (high): every alert comes from a timer that stops with the screen off

`cue("work")` is called from `finishWork`, which runs from the 200 ms
`T.tick`. The reminder fires from the 60 s slow tick. Both are page timers. In
row 2 of A1's table, the block closed at 40.0 minutes, and that is when "Focus
block done" played: 15 minutes late. If the app is killed, it never plays.

**Fix:** port Helth's `native.js` as a `NATIVE` section of `index.html`. It
keeps a plan of what should be armed, diffs that against what is armed, and
cancels or arms the difference:

| Entry | Id | When |
|---|---|---|
| Countdown for the running block or break | 100 | Posted at start, cancelled at pause or stop. Android removes it at `endAt` by itself (`setTimeoutAfter`) |
| "Focus block done" | 101 | Exact alarm at the block's `endAt` |
| "Break over" | 102 | Exact alarm at the break's `endAt` |
| Daily reminder, next 7 days | 200–206 | `reminderAt()` for each day. Skipped for a day already logged or already nudged |

These carry over from Helth as-is, because each one was found on the 2a:
- **The 2 s minimum lead applies only to new arms.** Helth's first version
  cancelled alarms that were about to fire.
- **Forget-on-launch after a force-stop.** On Helth, 49 alarms were armed
  before a force-stop and 0 after, while the app still believed they were
  armed.
- **A fired id is held for 3 hours**, so re-arming never dismisses a
  notification still sitting in the shade.
- **Two channels.** "alerts" is importance 4. "running" is importance 3 with
  silent posts, because Nothing OS hides Low-importance channels on the lock
  screen.

Two manifest lines make the alarms exact: `SCHEDULE_EXACT_ALARM` (max SDK 32)
and `USE_EXACT_ALARM`. Without them the plugin silently falls back to inexact
alarms, and a block's alert can land 5–10 minutes late (Helth A1).

The diff runs from three call sites that already follow every change it
depends on:
- `renderTimer`, on every timer transition
- `renderAll`, on every data change
- the existing 60 s slow tick

A block that ends inside quiet hours arms no alert, matching `cue()`'s
`!inQuiet()` today. `inQuiet` gains a time argument for this.

### A3 · BUG (high, latent): the notification switch says "on" whether or not Android agreed

At `index.html:4535–4539`, turning notifications on under Tauri fires
`request_permission`, ignores the answer and prints "Notifications on." That's
correct on Linux and Windows, where the plugin always grants. It's wrong on
Android 13+, where `POST_NOTIFICATIONS` is a real prompt. Deny it, and
Settings still says on while every alarm fires into nothing. This is the same
bug as Helth's A3.

**Fix:** lift Helth's code (`core.js` 1485–1570).
- Read `is_permission_granted` on launch and whenever the app returns to the
  foreground.
- On request, report the plugin's actual answer.
- Answer "granted" from the cache before asking. Helth found on the 2a that
  the plugin's `requestPermissions` never resolves when there's nothing left
  to ask.

The desktop branch is unchanged.

### A4 · DESIGN RISK (high): one event, two alerts

With A2 built and the app open, a block ending produces the native heads-up
(sound and vibration). It also produces `cue("work")`'s in-page beep and
`tauriNotify` popup.

**Fix:** on Android, with the permission granted, `cue()` plays nothing for
`work`, `break` and `remind`. The native alarm owns those three events whether
the app is open, frozen or dead. The badge cue has no alarm, so it keeps its
beep. If notifications are denied, the in-page cues remain the only signal
while the app is open. One guard at the top of `cue()` covers every call site.

### A5 · DESIGN RISK (medium): the plugin fork exists only on Helth's unmerged branch

Study needs the same fork. Stock `tauri-plugin-notification` 2.4.0 has these
problems on Android:
- It drops `extra`, so there's no countdown and no tap routing.
- It drops a tap that launches the app from closed.
- It reports every tap as `notification: null`.
- Once taps are fixed, the alarm receiver crashed at the exact moment an alarm
  fired. That was seen on the 2a at 18:20:34 on 2026-09-23.

Six patch blocks fix these.

A path dependency on `../../Helth/src-tauri/plugins/notification` would break
the moment Helth checks out `main`, where that folder doesn't exist. That's
the moved-folder bug again in a new shape.

**Fix:** copy the fork, licences included, into
`Study/src-tauri/plugins/notification` and wire it in through
`[patch.crates-io]`. A fix to either copy has to be made to both.

`// ponytail: two copies of one fork. If a third app needs it, move it to its
own repo and depend on that.`

### A6 · DESIGN RISK (medium): the phone's reminder makes a claim about data it may not have

A reminder armed for 19:00 fires with no JS running, so it can't check
anything. Say you studied on the laptop at 15:00 and haven't opened the phone
since. It still says "Nothing logged today yet", which is false, and is the
reminder's only sentence. The desktop reminder is stale in the same way, but
it at least checks the data it has at fire time.

**Fix: say what the phone actually knows.** The body becomes "Nothing logged
today as of Mon 14:32, when this phone last synced."
- **Opening the app syncs.** A day that then has a session drops out of the
  plan, which cancels its alarm.
- **The on/off is per device** (Helth's A7 rule). It's stored in its own
  `localStorage` key, like the sync credentials, so it's never uploaded and
  never in a backup and can't be merged wrong.
- **Desktop never writes the key,** so desktop behaviour doesn't change.

`// ponytail: the honest sentence, instead of a Kotlin receiver that checks a
mirrored "last logged day" before posting. Add the receiver if the stale nag
is why you switch the reminder off.`

### A7 · DESIGN RISK (medium): losing the signing key wipes the phone's local log

`adb install -r` only updates an APK signed with the same key. Otherwise it's
an uninstall and reinstall, which deletes the app's `localStorage`. Sync keeps
a copy, but unsynced sessions are lost.

**Decision:** sign with the keystore Helth already uses. It lives in
`~/.android-keys/`, outside the repo and outside `SyncedWork`. Helth's B2
asked you to back it up. It's **UNCONFIRMED** that you did, and it now
protects two apps.

### A8 · INCONSISTENCY (medium): the desktop updater card would show on the phone and fail

`index.html:4561–4577` shows "Check for updates" whenever `__TAURI__` exists.
On Android, `run_update` isn't registered, because it rebuilds this machine's
checkout. So the button would print "Update failed: …".

**Fix:** hide the card on Android. The Rust commands move behind
`#[cfg(desktop)]` in B1. Phone updates are `adb install -r` from this machine.

### A9 · UNCONFIRMED (medium impact): export may save nothing, and say it did

`download()` (4153) clicks an `<a download>` on a blob URL and returns `true`
regardless. Android WebView needs a download handler, and nothing outside the
WebView can fetch a blob URL at all. So on the phone this very likely does
nothing, while Settings still prints "Downloaded
study-tracker-backup-2026-09-28.json". A tool claiming a backup it didn't make
is exactly the failure this codebase is built to avoid.

Import (`<input type="file">`) depends on Tauri's WebChromeClient
implementing the file chooser. That's unchecked too.

**B3 checks both.** If export fails, add a `STNative.saveFile(name, mime,
text)` bridge: about 20 lines of Kotlin that write to Downloads through
MediaStore (Android 10+, which all three phones run). It returns whether it
wrote, so the message reports the real result.

### A10 · UNCONFIRMED (low): the header may sit under the status bar

Helth's `MainActivity` calls `enableEdgeToEdge()`. Study's CSS pads only the
bottom, with `env(safe-area-inset-bottom)` at 631, 1207 and 1243. It isn't
confirmed that Android WebView reports `safe-area-inset-top` in an
edge-to-edge app.

**B3 checks it with a screenshot.** If the variable is reported, the fix is
one CSS line. If it isn't, it's about 8 lines of Kotlin that apply the window
insets as WebView padding.

### A11 · INCONSISTENCY (low): service-worker registration throws inside the shell

`index.html:4789` registers `sw.js` on any page not on `file:`. Helth found
that under Tauri Android's `tauri.localhost` scheme this throws "unknown error
fetching the script" on every launch. The assets already ship inside the APK.

**Fix:** also skip registration when `window.__TAURI__` exists. That's one
condition, the same as Helth's `pwa.js`, and it also removes a registration
that can only fail inside the desktop shell.

### A12 · DESIGN RISK (low): Nothing's fonts can't be shipped, so the face borrows the phone's own

Ndot 55, Ndot 57 and NType 82 are proprietary, "All Rights Reserved" to
Nothing Technology (by Colophon Foundry and Seventy). They were never
released for download (source: fontsinuse.com's entry on the Nothing Phone,
and Colophon's own site). Copies extracted onto GitHub carry no licence. None
of them can go into this repo, which is public.

**Fix: Helth's route, already proven on the 2a.** An `@font-face` whose
`src` is `local("Ndot 57 Aligned"), local("Ndot-57-Aligned")`, then the
vendored Doto. Nothing OS keeps its fonts in the phone's `/system/fonts`, so
the WebView uses the copy that's already there. Nothing proprietary is
shipped, and every other device falls back to Doto (SIL OFL 1.1, by Óliver
Lalan). Helth's stylesheet records that both local() names were confirmed
loading in the 2a's WebView. Lettera Mono, the lock-screen text face, loads
the same way. The research subagent found no document confirming that
`local()` reaches OEM fonts on Android. The 2a result is the evidence, and
it's **UNCONFIRMED on the 4a** until B8 asks the WebView directly.

**The laptop prototype uses your local copies, and only there.** Your `qylock`
lock-screen theme bundles Ndot 55 Caps and NType 82 Mono
(`~/qylock/themes/nothing/font/`). The prototype in
`~/SyncedWork/Claude/_scratch/study-android/prototype/` symlinks them, so
the laptop renders Nothing's real dot face. Chromium reports `Ndot55Caps` and
`NType 82 Mono`. Those files never go into this repo or anything published.
The real build has no `url()` to them.

**COSMETIC (low), off Nothing phones only:** Doto's colon at the vendored
weight is two plus-shaped dot clusters, not two dots. Chromium's
`CSS.getPlatformFontsForNode` confirms the glyph is Doto's own ("Doto Black"),
not a fallback. It shows on the M13 and nowhere on the 2a or 4a. Helth's
vendored cut is also square-dotted, where Ndot's dots are round.

### A13 · Stated limits, from Helth's results on the device

- **After a force-stop, nothing rings until you open the app again.**
  Android wipes every alarm the app owns.
- **After a reboot, alarms return only once the phone has been unlocked.**
- **The countdown isn't on the AOD.** It shows on the lock screen and in the
  shade. The 2a's firmware doesn't define `POST_PROMOTED_NOTIFICATIONS`
  (Helth A13). `promoted: true` stays set, so the countdown becomes a Live
  Update once an OS update allows it.
- **A tap that launches the app from closed has never been tried.** Helth
  tested taps only with the app already running. B7 tries it.
- **The focus face is not a real AOD.** While it counts, the screen is on,
  not dozing, so it draws more power than Nothing's AOD. The phone also
  doesn't auto-lock. Pressing power turns the screen off, and the face comes
  back over the lock screen at the next wake.

### A14 · COSMETIC (low): History overflows sideways at 390px, before and after this build

`audit_base.py` flags `OVERFLOW ... 390 history` on every populated profile,
in both `index.html` and the pre-chunk-1 backup. Not something chunk 1
introduced — it only touched `renderTimer`, the timer CSS and the ring
driver, none of which History renders. The widest element differs (heat-grid
before, the session table after), which points at ordinary content growth
in the log table rather than a regression. Not chased further here; it's a
phone-width History layout question for whichever chunk touches that view.

---

## Part B — the build

Each step can ship on its own and ends with a check that was run. The skeleton
(B1–B3) comes before any feature, per `skills.md`.

**Backups first:** timestamped copies of `index.html`,
`src-tauri/src/main.rs`, `src-tauri/Cargo.toml` and `.gitignore`, made in the
same change as the first edit.

| Step | What | Who | Done when |
|---|---|---|---|
| B0 | `git switch -c android`, the backups, and `.claude/agents/android-mech.md` and `android-js.md` copied from Helth and pointed at this plan | Opus | The branch exists and the backups are listed |
| B1 | `lib.rs` with `run()` as `#[cfg_attr(mobile, tauri::mobile_entry_point)]`, called from `main.rs`. The tray, close-to-tray, `run_update`, `relaunch`, `project_root` and its helpers go behind `#[cfg(desktop)]`, and the tests behind `#[cfg(all(test, desktop))]`. `Cargo.toml` gets a `[lib]` section, `tray-icon` gated to desktop and `devtools` on Android (Helth's block) | `android-mech` | `cargo test` passes 3/3, and `cargo build --release` and `sh src-tauri/install.sh` pass on Linux. The installed app opens with your data |
| B2 | Copy the plugin fork and add `[patch.crates-io]` (A5). Run `cargo tauri android init`. Manifest: the two exact-alarm lines, `POST_PROMOTED_NOTIFICATIONS`, and `VIBRATE` (a WebView's `navigator.vibrate` needs it; UNCONFIRMED, checked in B3). Signing block reading `~/.android-keys/keystore.properties` (A7). `MainActivity.kt` bridge as `STNative` (Helth's, minus `face`/`awake`). An `ic_stat_study` icon. `.gitignore` stops ignoring `gen/android` (Helth's diff) | `android-mech` | **Done, verified 2026-09-29.** `cargo tauri android build --apk --target aarch64` produced `gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`. `apksigner verify --print-certs` gives SHA-256 `ed654669...427a22c`, `CN=Wellness Hub, O=Talon, C=IN` — identical to Wellness Hub's own APK, byte for byte. `cargo build --release` (desktop) finished in 11m56s, exit 0. `cargo test` still 3/3 |
| B3 | Skeleton on the 2a. Pair, install, open, sign in to sync, and your history arrives. Read the WebView console through `adb forward` to its devtools socket, driven by Playwright `connect_over_cdp` where that works and `chrome://inspect` by hand where it doesn't. Settle A9 (export JSON and CSV, check with `adb shell ls /sdcard/Download`, restore through the file picker), A10 (`adb exec-out screencap`) and A11 (the console) | `android-mech` + **you** | 0 console errors on open. The phone's History shows the same session count as the laptop. A9, A10 and A11 are each confirmed or refuted, with the output quoted **Partly run 2026-09-29 (wave 2, Nothing 2a, Android 16, WebView 153): install, console, A9-A11 answered below. Sign-in and the History count check are not done: they need you** |
| B4 | Timer and cue core, all JS and testable without a phone. Detect `ANDROID_SHELL` (Helth's test: `__TAURI__` plus an Android user agent). A1's restore-not-recover, plus the two pre-existing fixes unless you pull them. A3, A4, A8, A11, and A9's bridge if B3 calls for it | Opus | A1's script (`_scratch/study-android/harness/freeze_kill.py`, plus `group_loss.py` for the group fix), re-run with an Android user agent against the backup and against the new file. "Killed 30 s in" goes from **nothing** to 25 min ending at 25.0, with the banner. "Killed 10 min in, reopened at 12" shows the timer still running with 13 min left. The desktop rows are unchanged apart from the two pre-existing fixes. An old-shape `activeSession` still recovers. 0 console errors in both themes | **Done, verified 2026-09-29.** A9's bridge was added in wave 3 and verified on the 2a (under "B3 answers" below). The other numbers are under "Progress" below |
| B5 | The native alarms (A2, A6): the `NATIVE` section with plan, diff, channels, and taps that open Today (cold start included). Also the per-device reminder key, the "as of last sync" body and quiet hours | Opus | Playwright with a mocked `__TAURI__.core.invoke`, an Android user agent and real clicks asserts the exact calls. Start → countdown 100 and alarm 101 at `endAt`. Pause → 100 and 101 cancelled. Resume → re-armed, with `endAt` shifted by the pause. Stop → cancelled. Block finishes → 102 at the break's end. A session logged today → 200 cancelled. Reminder moved → 200–206 re-armed. A block ending in quiet hours → countdown only. A desktop user agent → the planner makes zero plugin calls | **Done, verified 2026-09-29 off the phone**, on chunk 5's hunks split out from chunk 6's concurrent edit. The numbers are under "Progress" below |
| B6 | A "This phone" card in Settings: whether notifications are allowed (with a link to settings), exact alarms, battery (Samsung: Never sleeping apps with a confirm tick; Android 15+: "Allow background usage"), "armed until", and the reminder on this phone. The first-run card: timer alerts and the daily reminder, each ticked with its reason, then the permission prompt | `android-js` | Playwright at 390 and 1920 px, in light, dark and Arasaka. The card's ticks land in the device key. A desktop profile running the same flow has no device key. 0 console errors |
| B7 | On the phones: a 1-minute block with the screen locked; `am kill` 5 s after Start (does the storage write survive?) and again mid-block, reopening after the end (A1 on the device); a swipe out of recents mid-block; Back mid-block (the Android 12 claim); force-stop, then reopen; reboot, then unlock; tapping a notification with the app closed; `dumpsys alarm` showing the armed ids. The 2a first. The 4a and M13 in the same pairing session as Helth's still-open B9 | Opus drives `adb`, **you** watch | Each result quoted per phone. Every alert lands within ±5 s of `endAt` (Helth's bar: +1.8 s locked and +1.4 s after `am kill` on the 2a), and the killed block is in the log |
| B8 | The focus face (Part C): the Today ring full screen, the A12 font faces, Helth's `face()`/`awake()` bridge calls, the 3 px drift, the 5 s control fade and the 500 ms hold to start. The face's controls call the same functions as Today's buttons, and it has no timer of its own | `android-js` | In Playwright with an Android user agent, at 390 and 412 px, in Pine and Arasaka, with and without reduced motion: the face's circle has Today's geometry (viewBox 228, r 103, stroke 8) and **the same dash at the same moment** in all four states; the digits stay within 66% of the inner diameter under Doto and under the mono fallback; the block → break → idle cycle runs without leaving the face; 0 console errors. On the phones, `CSS.getPlatformFontsForNode` on the digits (over CDP) reports **Ndot 57 Aligned** on the 2a and the 4a and **Doto** on the M13. On the 2a it shows over the lock screen at 10% brightness, keeps the screen awake while counting and releases it at pause. The ring driver's numbers hold in the real page, measured by `_scratch/study-android/prototype/measure.py` and `zero_and_rm.py` against Today's ring before and after: refills land under 2 px a frame, no dot at 00:00, at most 7 repaints a second in a 25-minute drain, digits on the second, and reduced motion snaps in one frame |
| B9 | Move the phone's data. If the browser or PWA copy on the phone has sessions, sign it into sync, or export JSON from the browser, where downloads do work. Then sign the app in and uninstall the PWA | **you** | The laptop, the app and the cloud show the same session count |
| B10 | `SETUP-android.md` (build, pair, install, and A13's limits), plus README entries for A1, A4 and A6 under "The things most study trackers get wrong" | `android-js` | **Drafted 2026-09-29, before B3.** `SETUP-android.md` marks every step and limit not yet seen on a phone as UNCONFIRMED. Those marks come off as B3 and B7 report. The README has the three entries plus an "Android app" row in the feature table |
| B11 | Merge `android` into `main` and bump `CACHE` in `sw.js`, when you say. The guarded JS reaches the live PWA here, not before | Opus | `git log` shows the merge. The live site loads with 0 console errors |

**B3 answers, 2026-09-29** (APK 19:59 installed with `adb install -r`, Success; console read over `adb forward` and Playwright `connect_over_cdp`).

- **A11 confirmed fixed.** Reload over CDP with listeners attached: `errors: []`, 0 errors and 0 warnings. `__TAURI__` is present, page is `http://tauri.localhost/`.
- **A10 CONFIRMED, and the cheap fix applies.** The screenshot shows the clock and camera cutout drawn over the "Study Tracker" title and the Level pill. The variable is reported: probing `padding-top:env(safe-area-inset-top)` gives `48px` (bottom `24px`, DPR 2.625). So the fix is one CSS line, padding the header by `env(safe-area-inset-top)`; no Kotlin. Header rect is y 0-56, so it needs 48px more.
- **A9 CONFIRMED: export saves nothing and says it did.** Real clicks on "Download backup (JSON)" and "Export sessions (CSV)" printed "Downloaded study-tracker-backup-2026-09-29.json" and "Downloaded study-sessions-2026-09-29.csv (0 rows)". `/sdcard/Download` held 73 files before and 73 after; `find /sdcard -name "study-*2026-09-29*"` found nothing. Wave 3 needs the `STNative.saveFile` bridge.
- **Import chooser works.** Clicking "Restore from backup" fired a Playwright `filechooser` event, so no bridge is needed for restore. Not run: picking a file (needs a file on the phone).
- **Left for you:** type the Supabase URL, key, email and password in Settings > Cloud sync; allow notifications; set Battery to Unrestricted; then compare History's session count with the laptop. The phone currently has 0 sessions (CSV said 0 rows).

**Wave 3, A9's bridge: done and verified on the 2a, 2026-09-29 20:44.**
- **What was built:** `MainActivity.saveFile(name, mime, text)` writes to Downloads through MediaStore and returns whether it wrote. `download()` uses it under `ANDROID_SHELL`, so a false answer or a missing bridge prints "Couldn't write <name> to Downloads.", never "Downloaded".
- **The APK:** `app-universal-release.apk` from 20:14, built from `src-tauri/dist/index.html`. That file is byte-identical (`cmp`) to the current `index.html`. `classes.dex` contains `saveFile`. Signer `CN=Wellness Hub`, SHA-256 `ed654669…1427a22c`. `adb install -r` over the 19:59 build: Success.
- **On the phone:** real clicks over CDP, from `a9_device.py` in the session scratchpad. The page reports `typeof STNative.saveFile === "function"`. JSON, CSV, then JSON again: each printed "Downloaded …", with 0 console errors. `/sdcard/Download` went 73 → 76 files, and a second run took it to 79.
- **The files are real:** the pulled JSON parses (schema 7, 0 sessions), and the pulled CSV is the header line `id,dayKey,code,label,…`. Both are 0-session exports, because the phone still isn't signed in to sync.
- **INCONSISTENCY (low), found here, fixed at 23:03 (below):** a repeat export on the same day is saved as `study-tracker-backup-2026-09-29 (1).json`, because MediaStore picks the name. The message still said "Downloaded study-tracker-backup-2026-09-29.json".

**B3 follow-ups, 2026-09-29 23:03.** Backups `index.html.backup-20260929-225614` and `MainActivity.kt.backup-20260929-225614`.
- **A9 naming fixed.** `saveFile` now returns the `DISPLAY_NAME` MediaStore stored, or `""`. `download()` returns that name, or `""`, and both export messages print it. An APK from before the change answers `true`, which still maps to the asked-for name. The Kotlin and the JS ship together: the old JS reads a string answer as a failure.
  - `a9_export.py` gained two rows, "renamed" and "empty answer". It scores **6/7 on the backup** (renamed fails and prints "Couldn't write…" for a file that was written) and **7/7 on the new file**, with 0 console errors. That also re-runs the bridge fails, no bridge and desktop rows that were skipped at 20:44.
  - The APK (12,324,787 bytes, 23:03, 0 Kotlin errors, `dist` identical to `index.html` by `cmp`, signer `ed654669…1427a22c`) was installed over the 20:53 build. Real clicks over CDP printed "Downloaded study-tracker-backup-2026-09-29 (4).json", "… (5).json" and "Downloaded study-sessions-2026-09-29 (2).csv (0 rows)". `/sdcard/Download` held exactly those names. 0 console errors.
  - Nine test exports (76-byte CSVs, 699–709-byte JSONs, all 0 sessions) were deleted. Downloads is back to its 73 files.
- **The 20:50 `stn()` fix works on the 2a.** The Java bridge matches a method by argument count, so calling a zero-argument read with an undefined argument threw. On the 20:53 build the "This phone" card reads every row: exact alarms "Allowed", battery "restricted", notifications off. None shows "Cannot be read". The before state wasn't reproduced here; that it read "Cannot be read" everywhere comes from the comment on the fix.
- **A10's fix clears the status bar in landscape.** `dumpsys window` gives the status bar as 63 device px, which is 24.0 CSS px at DPR 2.625. The topbar is 80 px tall with 24 px of padding. **Not checked in portrait**, where B3 measured 48 px with the cutout at the top.
- **Sync on the 2a works; the laptop app is the stale copy.** After your sign-in the phone holds **87 sessions, 1,818 min**, last synced 29 Sep 23:21. The Linux app's localStorage holds **70, 1,479 min**, last synced **21 Sep 17:36**. By id: 0 sessions on the laptop only, and 17 on the phone only (16 pomodoro, 1 manual, dated 21–29 Sep). So the phone has everything the laptop has, plus eight days the laptop app hasn't pulled. **Still open:** open the Linux app so it syncs, then it should read 87.

**Launcher icon, 2026-09-29 23:16.** `cargo tauri android init` had left Tauri's default logo in `res/mipmap-*`, a legacy PNG with no adaptive icon. `icons/android/` from `tauri icon` wasn't usable as it stood: its foreground is the whole orange square at full canvas, and a circle mask would cut off the ends of the heatmap row.
- `_scratch/study-android/mkicons.py` renders from `icon.svg` (the PWA's and the Linux app's logo). The foreground is the glyph alone, scaled 0.409 so its farthest point (a cell corner, 80.6 units from centre) lands on the 66 dp safe-zone circle. The background is `@color/ic_launcher_background` `#EF9F27`. It adds `mipmap-anydpi-v26/ic_launcher.xml` and `ic_launcher_round.xml`, plus legacy `ic_launcher.png` and `ic_launcher_round.png` at all five densities. Tauri's originals are in `_scratch/study-android/launcher-icons-backup-20260929-231408/`.
- `aapt2 dump badging` on the new APK gives `icon='res/BW.xml'` (adaptive) at every density, with `color/ic_launcher_background #ffef9f27`. It was installed with `-r` over the 23:03 build, same signer. The 2a's drawer search shows the orange logo with the whole glyph inside the circle.
- **Not done:** no `<monochrome>` layer, so Android's themed-icon mode shows the colour icon. **Wellness Hub still ships Tauri's default icon** (`res/9w.png`); it's the white Tauri-logo tile in the 2a's drawer.

**Chunk 7 on the 2a, 2026-09-29 23:28–23:50** (B7 and B8's phone checks). APK 23:17, whose `dist` is identical to `index.html` (`cmp`). Notifications granted by you through the card's Open settings. Battery left on the default (Optimized, `isIgnoringBatteryOptimizations` false). 1-minute blocks (`workMin` was already 1 on the phone). Timing comes from `adb logcat -b events -v epoch` (`notification_enqueue` / `notification_alert`), read against the page's `T.endAt`, so both sides use the phone's clock. Driven over CDP with real clicks, plus `adb shell input` taps on the face. The tools are in `_scratch/study-android/chunk7/` (`st.py`, `start.py`, `fwd.sh`, `events.log`, `crash-window.log`). The 4a and the M13 weren't paired, so their rows are still open.

| Case | Alarm 101 vs `endAt` | Process | Logged on reopen |
|---|---|---|---|
| Screen off (KEYCODE_SLEEP), quiet hours 22–07 at 23:30 | none armed: countdown 100 only (B5's quiet-hours row, now seen on the device) | alive, and JS ran: the block logged 1 ms after `endAt` | yes |
| Screen off, quiet hours off | enqueue **+46 ms**, sound +261 ms. `dumpsys alarm`: `RTC_WAKEUP origWhen 1790705058391` = `endAt` exactly, tag `TimedNotificationPublisher` | alive | yes |
| Break end (102), same run | enqueue **+14 ms**, sound +218 ms | alive | — |
| App in the background (Home) | **+53 ms**. The page's block → break ran only on resume, 11 s late, so the break's countdown was late too | alive but throttled | yes |
| `am kill` 4.7 s after Start | broadcast restarts the process at +34 ms; enqueue **+117 ms**, sound +331 ms | killed | **yes**, 60 s exactly, with the banner "Finished while the app was closed…". The storage write survives |
| `am kill` 28.8 s in | process +30 ms; enqueue **+94 ms** | killed | yes, one copy (no duplicate) |
| Swiped out of recents, 17.7 s in | `am_kill … remove task`; enqueue **+102 ms** | killed | yes |
| Back, mid-block, face closed | enqueue **+101 ms** | **native crash**, see below | yes |
| Force-stop 5 s in, reopen 6 s later | `dumpsys alarm`: 0 armed after the force-stop, then `origWhen` = `endAt` again after reopening. Enqueue **+33 ms**, sound +241 ms | force-stopped | yes |

Every alert landed well within Helth's ±5 s bar. The worst was +117 ms, against Helth's +1.8 s locked and +1.4 s after `am kill`. Sessions 88–96 are these test blocks. A killed block starts no break when it's restored, so no 102 follows it. That's a limit, not a regression: nothing was running to arm it.

- **BUG (medium): Back crashes the app natively, and A1's "Back is not the risk" is refuted.** Back, mid-block with the face closed, goes to `wm_finish_activity … app-request`: the activity finishes itself, which is not Android 12's move-to-back. 566 ms later the process aborts with `FORTIFY: pthread_mutex_lock called on a destroyed mutex` (on the Rust threads, `RustStdoutStderr`), then `am_proc_died`, leaving a tombstone. Seen 2 of 2 times: once by your own edge swipe with the face open, once from `KEYCODE_BACK`. **Wellness Hub does the same** (`app-request` finish, then the same FORTIFY abort 668 ms later), so this is Tauri/wry's back handling, not Study's code. A1 and the native alarm still cover it: the alarm fired and the block was logged. The cost is a crash and a cold start on every Back press. **Fix:** in `MainActivity`, register an `onBackPressedDispatcher` callback that calls `moveTaskToBack(true)` (the Android 12 behaviour the plan assumed). It's the smallest fix because it avoids the teardown instead of patching Tauri's shutdown. When the face is open, Back should close the face first (B8 left Back unwired). Same patch for Helth. **Not applied**: Kotlin plus a rebuild, waiting for your go.
- **Cold-start tap: confirmed (A13).** Tapping countdown 100 in the shade, 1.1 s after the Back crash, gave `notification_clicked` and then `am_proc_start` 59 ms later. The break was restored with the same `endAt`, and the face opened on it. The fork's held-tap patch works on the device.
- **UNCONFIRMED, seen once: "Focus block done" (101) vanished at unlock.** It was cancelled by the app (reason 8, `APP_CANCEL`) 26 ms after the fingerprint unlock resumed the activity, 12.6 s after it posted. A second run through Home and reopen didn't reproduce it. That run instead cancelled the previous break's 102 as the new 102 was armed, which is the plugin's `trigger()` dismissing any visible notification with the same id. Tracing it failed: wrapping `__TAURI__.core.invoke` recorded nothing, so the object is evidently frozen.
- **B8's phone checks on the 2a: pass.**
  - `CSS.getPlatformFontsForNode(#faceDigits)`: `Ndot 57 Aligned`, `isCustomFont: false`, 5 glyphs, so it's the phone's own font. `document.fonts` shows `ST Ndot` and `ST Lettera` loaded.
  - With the face on, the window reads `sbrt=0.1`, `requestedOrientation=PORTRAIT` and `showWhenLocked=true`.
  - Screen off, then wake: `KeyguardShowing=true, Occluded=true`, and the screenshot shows the face over the keyguard with the bars hidden.
  - Keep-awake: `KEEP_SCREEN_ON` plus a `SCREEN_BRIGHT_WAKE_LOCK` for the app's uid while counting. None at idle. It returned once an 800 ms hold started a block. Gone after Pause (tapped on the face), with 0 alarms armed.
- **The card matches the bridge on Nothing.** Before your grant it read "Off in Android settings" (`POST_NOTIFICATIONS: granted=false`), exact alarms "Allowed" (`USE_EXACT_ALARM: granted=true`), and battery "restricted" (not in `deviceidle whitelist`). Its notification button opened `Settings$AppNotificationSettingsActivity`, and the card read "Allowed" on return.
- **Cleanup.** You deleted test sessions 88–96 and the paused block in History. The phone read 87 sessions afterwards, synced 23:51:42. Block lengths were 1 / 1 / 1 before this chunk started (who set them isn't recorded: settings carry one `updatedAt`, not one per field). They were set back to **25 / 5 / 15** through Settings, and storage read `workMin 25`. **Quiet hours are still 22 → 22 (off), and that's synced.** Put them back to 22–07. The phone dropped off `adb` before that could be done.
- **Still open on the 2a:** that Resume re-arms at the shifted `endAt`; reboot, then unlock; the reminder alarms (200–206) in `dumpsys alarm`; and the Back fix above, once you say go. The 4a and the M13: everything in B7 and B8, plus Helth's B9.


### Running the build in chunks

**The build runs as eight chunks, each in its own session, with this plan as
the handoff.** A session re-sends its whole history with every message. So
the research, the audit and the prototypes behind this plan would otherwise
ride along with every step of the build. Each chunk starts fresh and reads
its row below and the findings it cites, nothing else. Its tools are in
`~/SyncedWork/Claude/_scratch/study-android/`, and that folder's `README.md`
is the index.

| # | Chunk | Steps | Phone? | Model · effort | Reads only | Tools |
|---|---|---|---|---|---|---|
| 1 | New ring motion on Today | Part C, the ring driver | No | Sonnet · medium | `renderTimer`, the timer CSS, and the driver in `prototype/index.html` | `measure.py`, `zero_and_rm.py`, `audit_base.py` |
| 2 | Rust split and Android project | B0–B2 | No | Sonnet · low (`android-mech`) | Study's `main.rs` and `Cargo.toml`, plus Helth's `lib.rs`, manifest and `MainActivity.kt`. The plugin fork is copied, not read | `cargo test`, and `cargo build` output through `tail` |
| 3 | First install on the 2a | B3 | **Yes** | Sonnet · low | Filtered build and `adb` output | Writes the A9–A11 answers back into this plan |
| 4 | Timer and alert logic | B4 | No | Opus · high | The timer and recovery code (`persistActive` to `showRecoveryBanner`, 197 lines), `cue`, the Settings switch and the service-worker line | `freeze_kill.py`, `group_loss.py`, `verify_sync.py`, `audit_base.py` |
| 5 | Native alarms | B5 | No | Opus · high | Helth's `js/native.js` and Study's reminder functions | A mocked-`__TAURI__` harness written in this chunk, plus `verify_sync.py` |
| 6 | Phone settings and focus face | B6, B8 | No | Sonnet · medium (`android-js`) | `prototype/index.html` and `states.html`, the drift, fade and hold parts of Helth's `face.js`, and Study's settings view | `measure.py`, `shot.py`, `audit_base.py` |
| 7 | Tests on the phones | B7, and B8's phone checks | **Yes** | Opus drives `adb`, you watch | Filtered `adb` output | Helth's open B9 on the 4a and M13 runs in the same pairing |
| 8 | Docs and merge | B9–B11 | You, for B9 | Sonnet · low | The findings in Part A | — |

**Order:**
- Chunks 1 and 2 can run at the same time: 1 edits only `index.html`, and 2
  only `src-tauri/`.
- Nothing else runs in parallel on `index.html`. So 4 waits for 1, 5 waits
  for 4, and 6 waits for 1, 4 and 5.
- Chunk 3 needs 2, and 7 needs everything before it. That's two phone
  sessions in all.

**Rules every chunk follows:**
- **Search, then read.** `index.html` is about 59k tokens whole (bytes ÷ 4,
  a rough rule), so read the function by name. The line numbers in this plan
  drift after every edit.
- **Logs go to a file.** Gradle, cargo and `logcat` output is written to a
  file and searched there, or handed to the `log-parser` agent. It never goes
  into the session.
- **Match the effort to the chunk.** Low for chunks 2, 3 and 8, and high only
  for 4 and 5.
- **Numbers before screenshots.** An image is read only when a judgement
  needs eyes.
- **Close out in this file.** A chunk ends by recording what it verified
  under its step, so the next session reads a result instead of re-deriving
  it.

**Progress, 2026-09-29:**
- **Chunk 1: done.** Its close-out is in Part C under "The ring driver", with
  a correction.
- **Chunk 2: done, but it recorded no close-out.** Checked from disk later
  the same day:
  - the `android` branch exists, and `lib.rs` is split out (01:27)
  - the desktop release binary was rebuilt at 08:10, after the split
  - `app-universal-release.apk` (12.3 MB, 08:16) is signed with the same
    certificate as Wellness Hub (SHA-256 `ed654669…1427a22c`)

  **Not run:** `cargo test` (the 3 desktop tests) and `install.sh`. They're
  the first thing chunk 3 does.
- **Chunk 4: done, 2026-09-29, apart from A9's bridge**, which waits for
  chunk 3. The backup is `index.html.backup-20260929-082543`. Every result
  below comes from the same script, run against that backup (copied to a
  `.html` name) and against the edited file.

  `freeze_kill.py` has nine rows now, including reopening while the block
  runs, paused rows, and rows where the screen goes off during the break.
  "Off" is when the screen went off, "open" when the app was next opened,
  both in minutes after Start:

  | Row | Before, Android | After, Android |
  |---|---|---|
  | Off 0.5, killed, open 40 | **nothing logged** | 25 min, ends at 25.0, "Finished while the app was closed" |
  | Off 10, killed, open 40 | 10 min, interrupted | 25 min, ends at 25.0, same banner |
  | Off 10, killed, open 12 | 10 min, interrupted | Nothing logged yet. Timer running, **13.0 min left** |
  | Paused at 5, killed, open 15 | 5 min, interrupted | Nothing logged yet. Restored paused, 20.0 min left |
  | Paused at 5, killed, open 45 | 5 min, interrupted | 5 min, interrupted, ends at 5.0 (the 30-minute rule) |
  | Off 26 (in the break), killed, open 28 | 25 min | 25 min. The break resumes with 2.0 min left |
  | Off 26 (in the break), killed, open 45 | 25 min | 25 min. The break is dropped |
  | Frozen, unlocked at 40 | 25 min, **ends at 40.0** | 25 min, ends at 25.0 |

  Desktop before and after differ in one row only: frozen and unlocked at
  40 now ends at 25.0 instead of 40.0. `group_loss.py` gives group `''`
  before and `'C'` after on desktop, and `'C'` on Android. An old-shape
  `activeSession` with no `endAt`, opened under Android, still recovers
  through the heartbeat: 10 min, interrupted.

  `android_checks.py`, new in this chunk, passes 4/10 before and 10/10 after:
  - A3 denied: the switch is left off, with a message pointing to Android
    Settings.
  - A3 granted: on, answered from the cache, and `request_permission` is
    never called.
  - A4 granted: a block's end sends no "Focus block done" and no block
    beep. The badge keeps its two-tone cue, and Test still plays.
  - A4 denied: the in-page cues stay.
  - A8: the update card is hidden on Android and shown on desktop Tauri.
  - A11 over http: one service-worker registration in a browser, none in
    either shell.

  The other harnesses:
  - `verify_sync.py`: 6/6.
  - `audit_base.py`: 0 console errors. Its only output is A14's
    pre-existing History overflow at 390 px.
  - Android user agent, light and dark, every view: 0 errors.

  **Beyond the plan's text:**
  - `workEndAt()` stamps a paused block with `pausedAt`, not `min(now,
    endAt)`. With `min` alone, a block paused at 10 minutes and closed out
    by the 30-minute rule would be dated 25. The same helper dates Stop,
    so Stop pressed while paused now ends at the pause instead of at the
    press. That last part is a desktop behaviour change as well.
  - The banner says "It was running, not paused, when it ended". The
    plan's "It was never paused" is false for a block that was paused and
    then resumed.
  - `cue()` takes a `test` flag, so Settings' Test button isn't silenced
    by A4.
  - `T.round` is persisted, so a restored break keeps its place in the
    cycle.

  **Left as it was:**
  - A block closed by the 30-minute rule still starts a break, as it does
    on an open page.
  - On Android, denying the permission sets the synced `cues.notify` to
    false, the same thing the browser path already does. That turns the
    laptop's notifications off too. B6's device key is where this should
    move. (Done in chunk 6, below.)
  - A4's guard is live from this chunk, before B5 arms any alarm. So an APK
    built between chunks 4 and 5 is silent at a block's end when
    notifications are allowed. Don't install one.
- **Chunk 5: done, 2026-09-29, verified off the phone only.** The backup
  is `index.html.backup-20260929-112115`.

  **Chunk 6 ran on `index.html` at the same time**, from another session
  that started at 11:21. The order above says 6 waits for 5. The file on
  disk was mid-edit and threw `FACE is not defined`, so testing it proved
  nothing. Chunk 5 was checked on its own instead:
  - `_scratch/study-android/chunk5/site-after/` is the backup plus chunk
    5's 12 hunks only, split out of the shared diff (`mine.diff`).
  - `site-before/` is the backup as it was.
  - Each folder has `sw.js`, the manifest and the icon beside it. Without
    them, `sw.js` fails its precache and A11 reads 0 registrations.

  The combined file still has to pass every harness once chunk 6 has
  finished.

  `native_alarms.py`, new in this chunk, runs on a fake clock at Tue 29 Sep
  10:00 with real clicks. It passes **17/17 after**, with 0 console errors.
  Before, it fails 14 of the first 15 rows, then throws at the tap row:

  | Row | Before | After |
  |---|---|---|
  | Start | nothing | countdown 100 to 10:25, alarm 101 at 10:25 |
  | Pause | nothing | 100 and 101 cancelled |
  | Resume | nothing | both re-armed. `endAt` moved 300.3 s for a 300.3 s pause |
  | Stop | nothing | 100 and 101 cancelled |
  | 1-min block finishes | nothing | 102 at 10:06, and 100 is now "Short break · back at 10:06" |
  | Block ends in quiet hours 10–11 | nothing | countdown only |
  | Notify switched off mid-block | nothing | 100 and 101 cancelled |
  | Reminder at 19:00 | nothing | 7 alarms, Tue 29 → Mon 05. The last one ends "Open Study Tracker to keep reminders coming." |
  | Session logged today | nothing | today's reminder (205) cancelled, nothing else |
  | Reminder moved to 20:00 | nothing | all 7 re-armed at 20:00 |
  | Relaunch 3 min in | nothing | 100 and 101 re-armed, although the record said armed |
  | Permission read never returns | 0 calls | 0 calls. Nothing cancelled |
  | Desktop user agent, a full cycle | — | 0 planner calls. No device key written |

  The regression harnesses, on `site-after`:
  - `verify_sync.py`: 6/6.
  - `audit_base.py`: 0 console errors. Its only output is A14's
    pre-existing overflow.
  - `freeze_kill.py --android`: every row matches chunk 4's table. Without
    `--android`, desktop keeps heartbeat recovery: off at 0.5 and killed
    logs nothing, and off at 10 and killed logs 10 min, interrupted. It
    wasn't re-run on `site-before`, because chunk 5 doesn't touch recovery.
  - `android_checks.py`: 9/10, and the same 9/10 on `site-before`. The
    fail is A3 granted, which chunk 6 rewrote at 11:47 to expect its own
    `notifyOn` device key. Neither file has that key, so the check can't
    pass on either.

  **Beyond the plan's text:**
  - **Reminder ids are 200 + the day's number mod 7, not 200 + days from
    today.** A day keeps its id across launches. A fired reminder's id
    isn't reused for a week, so re-arming never dismisses one still in the
    shade. So "a session logged today cancels 200" reads as "cancels
    today's id" (205 on 29 Sep).
  - **The diff also runs from `save()`.** Moving the reminder or changing
    quiet hours calls only `save()`, not `renderAll`. From renderTimer and
    the slow tick alone, a moved reminder would have waited up to 60 s.
  - **The diff waits until the permission read has answered, and until
    `S` has loaded.** An empty plan would otherwise cancel every alarm the
    phone holds on each launch. The `S` guard came from a console error
    this chunk's first regression run caught: the permission read answers
    before `init()` loads the state. Chunk 6 had already added the same
    guard to `index.html`.
  - **One gate for both sides.** `nativeOwnsAlerts()` decides both
    whether the planner arms and whether `cue()` stays quiet. So switching
    Notifications off brings the page's beep back instead of silencing
    everything. Chunk 6 has since pointed it at its per-device
    `notifyOn()`.
  - **`ic_stat_study.xml` didn't exist.** B2's row lists it, but
    `res/drawable` only had the launcher background. It's added now:
    `icon.svg`'s ring, hand and five cells at 1/8 scale. Part C's "book
    glyph" is wrong, because `icon.svg` has no book. It's unseen on a
    device until B7.
  - **The countdown titles for a break are "Short break" and "Long
    break"**, with the body "break · back at 14:30". Part C only specified
    the focus row.

  **Left as it was:**
  - A block that ends while the app is dead starts no break, so 102 isn't
    armed then. On reopen the block is logged and nothing follows, as in
    chunk 4.
  - A reminder can fire during a running block if nothing is logged yet
    that day. Desktop does the same.
  - The cold-start tap route rests on the plugin fork holding the tap
    until `onAction` registers. That's Helth's patch, untested on the
    device (A13). B7 tries it.
- **Chunk 6: done, 2026-09-29 (B6 and B8), apart from the phone checks below.**
  The backup is `index.html.backup-20260929-112246`. Nothing was built or
  installed. All scripts are in the session scratchpad; the harnesses are
  `_scratch/study-android/harness/`.

  **What was built.**
  - **B6.** A "This phone" card in Settings, and a first-run card on Today.
    Both are hidden off Android.
    - Rows: notifications (with an Open settings action when denied), exact
      alarms, battery, "alarms armed until", and the reminder for this
      phone. On Samsung a "Never sleeping apps" row carries a confirm tick.
      On Android 15+ (API 35) the battery row is titled "Allow background
      usage".
    - `exactAlarms()`, `batteryExempt()`, `maker()` and `sdk()` are read
      through a guarded `stn()` wrapper. A method that is missing shows
      "Cannot be read from here. Check it in Android settings.", never a
      tick. The Samsung list has no API at all, so it is only ever the
      user's own tick.
    - First run: timer alerts and the daily reminder are ticked with their
      reasons and applied only on Apply. Alerts then raise the permission
      prompt. Unticked alerts never prompt.
    - Every tick goes through `setDevice()` (`remindOn`, `notifyOn`,
      `sleepOk`, `firstRunDone`). Nothing reaches the synced blob.
  - **B4's "left as it was" is closed.** On the phone, `cues.notify` is now
    `device().notifyOn` (`notifyOn()` reads it, and follows the synced
    switch until the phone has chosen). A denial no longer switches the
    laptop's notifications off through sync. This was four reads and two
    writes, so it was small enough to do.
  - **B8.** The focus face, as Part C describes it.
    - The ring is `RING`'s second target: one `draw()` writes both arcs, so
      the two can't disagree. Steps are sized off the smaller ring, so the
      bigger face ring doesn't repaint more often.
    - The digits are `renderDigits()`'s own string.
    - Pause and Stop click Today's buttons, and the 500 ms hold clicks
      Start.
    - Entry is a tap on Today's ring while a block or break runs (Android
      only), or the countdown notification's tap.
    - The digits are five fixed cells, fitted to 66% of the inner diameter
      after `document.fonts.load()` on the named faces. `ST Ndot` is
      `local()` x4 and then Doto, embedded as a data URI, so `index.html`
      stays one file and the desktop bundle copies it as before. Doto's
      `OFL.txt` is in `vendor/doto/`. No Ndot or NType file is anywhere in
      `Study/`.
    - In a light mode the face uses Pine's dark values instead of the
      light palette's tokens (dark-on-light on black is unreadable).
    - Idle shows `25:00` (the next block), not the last phase's length.
      Today shows `05:00` after a break, and the face deliberately doesn't.
  - **MainActivity.kt** now has `face()` and `awake()`, ported unchanged
    from Helth. **Not compiled**: chunk 7 builds. Every other bridge method
    the JS reads already existed, so nothing was left out.

  **Verified** (390 and 412 px, Android user agent, mocked `__TAURI__` and
  `STNative`, real clicks, Playwright's fake clock for the cycle):
  - **B6:** 390 and 1920 px in Paper, Pine and Arasaka. Rows render and text
    colour differs from the card's, with no sideways overflow. Ticks land in
    `studyTracker.device.v1`: the synced `cues.notify` and `reminder.on`
    were identical before and after. With no bridge, "Cannot be read" shows
    twice (exact alarms and battery). A denied prompt leaves the synced
    `cues.notify` true. A desktop browser and a desktop-Tauri profile toggle
    the reminder, Notification and Beep switches and run a block, and
    `localStorage` holds only `studyTracker.v1` afterwards. 0 errors.
  - **B8:** 298 checks passed in 9 runs (Pine and Arasaka × 390 and 412 ×
    reduced motion on and off, the mono fallback twice, and Paper in a light
    mode), 0 failed, 0 console errors.
    - The face's circle has Today's geometry (viewBox `0 0 228 228`, r 103,
      stroke 8, round cap).
    - `strokeDashoffset`, opacity and dasharray were equal on both arcs in
      every state and mid-sweep: focus, paused (120 ms after the click),
      resumed, block end (150 ms in), break, break end, idle.
    - The block → break → idle → hold-start → Stop cycle never left the face.
    - Digits: 187.7 / 284.5 = 0.660 at 390 px and 198.4 / 300.5 = 0.660 at
      412 px, under Doto and again with the mono stack forced.
    - Controls fade at 5 s and a touch restores them. The hold starts at
      560 ms and not at 300 ms. Drift stays within ±3 px and changes each
      minute, and is absent under reduced motion. `awake:false` goes out at
      pause.
    - Chromium reports `Doto Black` for the digits here, confirming the data
      URI face loads by name.
  - **Regression:** `android_checks.py` 10/10. Its A3-granted row now reads
    the device key, since the phone's answer no longer lands in synced
    state. `verify_sync.py` 6/6. `audit_base.py` 0 console errors, and only
    A14's History overflow at 390 px.

  **Ring driver on the real Today ring** (real time, 60 s blocks, the
  pre-chunk-1 backup against `index.html`, 390 px):

  | | Before | After | Target |
  |---|---|---|---|
  | Block-end refill, last step | 1.7 px, 510 ms | 0.4 px, 478 ms | under 2 px |
  | Refill after Stop, last step | 4.6 px, 199 ms | 0.5 px, 284 ms | under 2 px |
  | Digits behind the true second | 18 ms | none seen at 60 fps | on the second |
  | 25-min drain, repaints a second (fake clock, 16 ms frames) | n/a | 5.2, max step 0.107 px | at most 7 |
  | Value at 00:00 | not sampled | 0.18 px long, opacity 0.02 (invisible) | no dot |
  | Reduced motion | 1 frame | 1 frame | one frame |

  - The "before" repaint and 00:00 rows aren't re-measured. Chunk 1
    measured them on the prototype (54 a second, and a dot), and a
    real-time run can't drain a 25-minute block.
  - The old page didn't reproduce the prototype's "stops dead at 81 px" on
    the real Today ring: its last step here is 1.7 px. The prototype's
    "before" was harsher than the real page, so the refill claim in Part C
    overstates the improvement. The fix still helps, at 4.6 → 0.5 px after
    Stop, but the 81 px figure shouldn't be quoted.
  - The digit check has a 16 ms floor on the fake clock: two one-frame
    mismatches in 40 s, both within a frame of the true second.
  - One real bug found on the way: `nativeDiff()` could run before `S` had
    loaded. It now returns early, as it already does before the permission
    is known, so an empty plan can't cancel alarms.

  **Deferred to chunk 7 (needs the phone):**
  - `CSS.getPlatformFontsForNode` on the digits over CDP: **Ndot 57
    Aligned** on the 2a and the 4a, **Doto** on the M13. Chromium on the
    laptop has only proved the Doto path.
  - Whether `local("Ndot 57 Aligned")` reaches the OEM font on the 4a, and
    whether `local("Lettera Mono LL")` loads.
  - `face()` over the lock screen at 10% brightness, portrait lock and
    hidden bars, and that `awake()` keeps the screen on while counting and
    releases it at pause. Also the Gradle build of the new Kotlin.
  - That the bridge's real answers match what the card shows: exact alarms,
    battery on Nothing, and the Samsung and Android 15 wording on the M13.
  - The cold-start tap into the face (A13) and a tap while the block runs.
  - The real Android back gesture while the face is open. It isn't wired
    here, so Close is the only exit.
- **The phone is away.** Chunks 3 and 7, B9 and B11 wait for it. Run 4 → 5 →
  6 now, then draft B10. B4's A9 bridge also waits, because it depends on
  chunk 3's export check. Everything else chunk 3 settles (A10, A11) has a fix
  that's additive, so nothing in 4–6 depends on it.

### What runs at the same time, from 2026-09-29 (the phone is back)

**What's left runs in four waves. Everything in a wave can run at the same
time, and each wave waits for the one before it.** Two rules set the waves.
Only one session edits `index.html` at a time. The 2a is one phone, so only
one session drives it. The APK built at 08:16 predates chunks 4–6, so the
first wave rebuilds it.

| Wave | Session or person | What | Model · effort | Touches | Waits for |
|---|---|---|---|---|---|
| 1 | Laptop session | `cargo test` (3/3), `install.sh`, then rebuild the APK. This also compiles chunk 6's `face()`/`awake()` for the first time | Sonnet · low (`android-mech`) | `src-tauri/` only | — |
| 1 | **You** | Pair the 2a (Wireless debugging → pairing code). Confirm that `~/.android-keys/` is backed up outside `SyncedWork`. Sign the phone's browser/PWA copy into sync, or export its JSON (B9, first half). **Don't uninstall the PWA yet** | — | the phone | — |
| 2 | Chunk 3 | B3: install, console, sign-in, A9–A11. You type the credentials and grant the permissions | Sonnet · low | the phone, this plan's B3 row | both of wave 1 |
| 3 | Laptop session | B4's A9 bridge, **only if chunk 3 finds that export saves nothing**. Then rebuild the APK | Opus · high | `index.html`, `MainActivity.kt` | chunk 3's A9 answer |
| 3 | Laptop session | B10: remove the UNCONFIRMED marks that B3 settled | Sonnet · low | `SETUP-android.md`, `README.md` | chunk 3 |
| 3 | **You** | Uninstall the PWA, once the app's History matches the laptop (B9, second half) | — | the phone | chunk 3's count check |
| 4 | Chunk 7 | B7 and B8's phone checks. Helth's open B9 on the 4a and M13 runs in the same pairing | Opus drives `adb`, you watch | all the phones | wave 3's rebuild, if there was one |
| 5 | Chunk 8 | B10's last marks after B7, then B11's merge and `CACHE` bump when you say | Sonnet · low | docs, `sw.js`, git | chunk 7 |

**Wave 1, laptop half: done, 2026-09-29.** `cargo test` 3/3. `install.sh` exit 0; it only copies the release binary (built 08:10, after the last Rust edit at 01:27, so not stale). `cargo tauri android build --apk --target aarch64` exit 0 with 0 Kotlin errors, so `face()`/`awake()` compile. APK: 12,324,339 bytes, 19:59, signer `CN=Wellness Hub`, SHA-256 `ed654669…427a22c`. Not checked: Kotlin warnings; not installed on a phone. The "You" row below is still open.

- **Wave 3's two sessions both close out in this file.** Each writes under
  its own step and nowhere else, so the edits don't collide.
- If export works in B3, wave 3 has no `index.html` edit and no rebuild.
  Chunk 7 then installs wave 1's APK.

---

## Part C — what the phone shows

**Notifications carry one fact each, in a form the lock screen can show.**

| Entry | Title | Body | Lock screen |
|---|---|---|---|
| Countdown | The subject, e.g. `ECO354` | `focus · ends 14:25`, with a chronometer labelled "left" | Readable |
| Block done | `Focus block done` | `25 min on ECO354. Break next.` | Readable |
| Break over | `Break over` | `Back to it.` | Readable |
| Reminder | `Time to study` | `Nothing logged today as of Mon 14:32, when this phone last synced.` | Readable |

The icon is `ic_stat_study`: a monochrome book glyph cut from `icon.svg`,
which Android tints. Without it the plugin falls back to Android's stock "i"
glyph, as Helth found.

**The focus face (B8) is the Today ring, full screen on black.** It's the
same circle drawn by the same rule:
- viewBox 228, radius 103, stroke 8, round caps, starting at 12 o'clock
- the arc is the fraction of the block still remaining
- `--accent` while focusing, `--good` in a break, `--line-strong` when
  paused, and a full ring when idle

The face gets its dash from the same function `renderTimer` uses, so the two
circles can never disagree. Only the type changes: the digits are Nothing's
Ndot 57, and the labels are Lettera Mono.

```
┌──────────────────────────────┐
│            14:07             │   the clock, Lettera Mono, dim
│                              │
│         ╭──────────╮         │
│       ╱              ╲       │   Today's ring, the arc emptying
│      │     18:32      │      │   from 12 o'clock; digits in Ndot 57
│      │  ROUND 2 OF 4  │      │
│       ╲              ╱       │
│         ╰──────────╯         │
│                              │
│     ECO354 · ENDS 14:25      │   the subject and the end time
│         ━━ ━━ ── ──          │   round chips, as on Today
│                              │
│  PAUSE                STOP   │   fade out after 5 s without a touch
└──────────────────────────────┘
```

| State | Ring | Digits | Below the ring | Screen |
|---|---|---|---|---|
| Focus | `--accent` arc, emptying | Time left | `ECO354 · ENDS 14:25`, round chips | Kept awake |
| Paused | `--line-strong` arc | Dimmed | `ECO354 · PAUSED 14:07` | May time out. Power brings it back over the lock screen |
| Break | `--good` arc | Time left | `BACK AT 14:30` | Kept awake |
| Idle | Full ring | `25:00`, dimmed | `HOLD TO START ECO354` | May time out |

**The whole cycle runs inside the face.** When a block ends, the break ring
starts in `--good`. When the break ends, the idle ring waits for a hold.
Closing the face never stops the timer.

| Rule | Value | Why |
|---|---|---|
| Background | `#000` | True black matches the AOD it stands in for, and costs nothing on OLED |
| Colours | Your dark palette's tokens, exactly as Today uses them | "The same circle". Today has no last-minute red, and red is already the accent in Arasaka and Akira |
| Digits | `Ndot 57 Aligned` on Nothing phones, then `Ndot 55 Caps` if that's what the phone has, both through `local()`. Doto everywhere else (A12) | Ndot 57 Aligned is what the 2a's lock screen uses. The Ndot 55 names (`Ndot55Caps`, `Ndot55CapsRegular`) come from your laptop's copy and are **UNCONFIRMED** on the phones |
| Digit cells | One fixed cell per character: every digit gets the widest digit's width, and the colon keeps its own. A narrower digit is offset by whole 0.1 em dot pitches, never half a pitch | Ndot 55 has no tabular figures. At 100 px, `11:11` is 140 px wide where `00:00` is 260 px. In cells, the time held at 197 px through 4 s of changing digits, every dot stayed on one grid, and the `1`'s stem sits on the centre column. On a fixed-width face every offset comes out 0 |
| Weight | Regular, with `font-synthesis: none` | The Nothing faces ship one weight. A requested bold makes Chrome fatten every dot into something Ndot isn't |
| Labels | `Lettera Mono LL`, the face Nothing OS draws its lock screen in, through `local()`. The mono stack elsewhere | The clock, subject and end time in the type the phone uses around them |
| Digit size | Fitted to the ring. Four digit cells plus the colon cell are set to 66% of the inner diameter, measured once the face has been loaded by name. `document.fonts.ready` alone can resolve before a face is used, and the cells would then be measured in the fallback | The Ndot cuts and Doto are different widths, so a fixed size that fits one overflows another. Prototype at 412 px: ring 344 px, inner 299 px, time 197 px wide |
| Clock | Small, at the top | The face covers the lock-screen clock, so it carries one |
| Controls | Pause/Resume, Stop and close. They fade after 5 s without a touch, and any touch brings them back | Helth's `QUIET_MS`. An AOD stand-in shouldn't keep buttons lit |
| Starting from idle | A 500 ms hold, Android's own long-press timeout | A stray tap on a dark desk shouldn't start a block |
| Burn-in | The whole face drifts up to 3 px once a minute | Helth's `DRIFT_PX`. It keeps a static ring from sitting on the same OLED pixels for 25 minutes |
| Motion | The ring driver below, shared with Today | Measured before and after on one clock |
| Entry | Tap the ring on Today while a block or break runs (Android only), or tap the countdown notification | Today's dashboard card is otherwise unchanged |
| Screen (Android) | Shows over the lock screen at 10% brightness, portrait, with the system bars hidden | Helth's `MainActivity.face()`, unchanged. This is the pseudo-AOD part |

**The ring driver: one implementation for Today and the face.** You asked
for the circle to be more fluid (2026-09-28). This was measured in a
before/after prototype: two rings on one simulated timer, driven by real
clicks, with every frame recorded. "Before" is Today's current code, verbatim.

| | Before (Today now) | After |
|---|---|---|
| Refill at a block's end | 197 ms linear, **stops dead at 81 px a frame** | 332 ms on `--slow` / `--ease`, **settles at 1.6 px a frame** |
| Refill after Stop | 200 ms linear, stops at 31 px a frame | 316 ms, settles at 1.3 px a frame |
| At 00:00 | A dot left at 12 o'clock: a round cap on a zero-length dash still draws | Nothing. The last stroke-width of travel fades the arc out |
| Digits at the true second | 17 ms and 67 ms late in two runs, and up to 199 ms depending on when Start was pressed. Frozen at the true 00:00, it still read `00:01` | On the second |
| Repaints during a steady 25-minute drain | 54 a second | 6.5 a second, in 0.10 px steps |
| Reduced motion | Collapsed by the global rule | Sweeps snap in one frame. The drain stays |

**The steady drain was never the problem.** At real speed it moves 0.65 px a
second on the phone, below what reads as motion, and neither version stutters
there. What wasn't fluid was every jump, plus the dot at 00:00. How each is
handled:

- **Drawn from the wall clock.** `stroke-dashoffset` on a fixed dash, from
  the same `remainingSec()` the digits read. That replaces a 0.2 s transition
  chasing the 200 ms tick, and the `.toFixed(1)` rounding.
- **Repaints only when the arc has moved 0.1 CSS px,** which is under one
  device pixel on every screen here. A timeout is scheduled for exactly that
  moment. `requestAnimationFrame` runs only during a sweep, or when the drain
  moves faster than a pixel a frame. `stroke-dashoffset` is a paint property,
  and a round-capped arc of changing length has no transform equivalent. So
  the paint is bounded to the ring and rationed instead.
- **Any jump becomes a sweep.** A jump is anything that arrives with an
  event: a phase change, Stop, the page waking from a freeze, or A1's
  restore. The sweep uses the codebase's own `--slow` (0.34 s) and `--ease`
  tokens, read from the CSS so the curve has one source. It starts from
  wherever the arc is drawn. A second jump mid-sweep restarts from the drawn
  value, so the arc never teleports.
- **The digits change on the true second.** They're scheduled to the
  boundary instead of polled every 200 ms.
- **No glow, no leading dot, no easing on the drain.** A blur filter would
  repaint every frame. The drain is time, and time is linear.

**Chunk 1 verified, 2026-09-29.** `RING` (index.html:3354–3441) and
`renderDigits` (3446–3453) are the ported driver, in `renderTimer`'s only
diff. `measure.py` and `zero_and_rm.py` against `prototype/index.html`:

| | Before | After (this build) |
|---|---|---|
| Refill at block end | 191ms/12 frames, last step 81.5px | 334ms/20 frames, last step 2.1px |
| Refill after Stop | 174ms/11 frames, last step 37.3px | 342ms/20 frames, last step 1.0px |
| At 00:00 | 0.2px arc, opacity 1 (dot) | 0.0px, opacity 0.00 |
| Digit lag past the true second | 186–209ms | 18–32ms |
| Steady drain repaints/s | 37.1 | 6.1 |
| Reduced motion | n/a | 1 frame, 65px snap |

All within the plan's targets (settles under 2px, no dot, repaints under 7/s).
`audit_base.py` against the real `index.html`: 0 console errors in all 18
profile/theme/width runs across 5 views. One `OVERFLOW ... 390 history`
finding survives on both this build and the pre-chunk backup
(`index.backup-20260928-221656.html`) — different widest element each time
(`heat-grid` before, `TABLE#.log` after), same narrow-viewport History table,
present before this chunk touched anything. Unrelated to the ring driver;
carried into A-findings rather than fixed here. A direct Playwright run
against the real Today view (real click on Start, not `evaluate`) confirmed
the same driver runs live: 0 errors, dashoffset advancing in sub-2px steps,
no snapping.

**Correction, checked from another session the same day.** The table above
was measured on `prototype/index.html`, not on the real Today ring. On the
real page, only the error count and the sub-2px steps were checked. The table
also misses two targets:
- the block-end refill lands at 2.1 px a frame, where the target is under 2
- the digits run 18–32 ms late, where the target is "on the second"

Both are below what the eye can see. B8's "Done when" re-measures all of
them against the real Today ring.

---

## What you have to do yourself

| When | What | Why it can't be done from here |
|---|---|---|
| Now | Approve this plan, and veto any decision in "Decided for Study" | Gate 1 |
| Now | Confirm `~/.android-keys/` is backed up somewhere outside `SyncedWork` | A7: it now protects two apps |
| B3, each session | On the 2a: Wireless debugging → Pair device with pairing code, then read me the IP:port and code. Same Wi-Fi as this machine | Physical access. The 2a's port carries no data |
| B3 | Type your Supabase URL, key, email and password into the app | Credentials never pass through this session |
| B3 | Allow notifications. Set Battery → Unrestricted (on Android 15+: Allow background usage). On the M13, also add it to Never sleeping apps | Only you can grant these |
| B7 | Watch the phones during the alarm tests | Only you can see them |
| B9 | Move the PWA's data, then uninstall it | It's your phone |
| Throughout | Approve permission prompts for `adb`, `cargo` and Gradle | Your permission settings |

---

## Out of scope

| Not doing | Why |
|---|---|
| Alerts for blocks started on the laptop | Needs FCM push and a server, which breaks the no-backend rule. Helth made the same call |
| A streak-at-risk alarm | The daily reminder is the one nudge. A second one on the same evening is noise, and the streak tile already says it when the app is open |
| A foreground service keeping the page alive through a block | That's Kotlin, a permanent notification and more permissions. A1's fix makes the page's death harmless instead of preventing it |
| Logging from a notification (a "Done" button) | Writing into webview storage while the page may be dead is a second write path around the merge. Helth's reason |
| Background sync while the app is closed | Sync runs on open and focus, as on desktop. The alarms don't need it, and A6 says how old the reminder's knowledge is |
| Keeping the pomodoro round count across a kill | Pre-existing on desktop, where a quit resets it. At worst the long break comes a round early |
| Widgets, a quick-settings tile, Wear OS | Not asked for |
| Play Store, iOS, an in-app APK updater | Sideload only |
| Glyph SDK, or the real AOD | Not possible on the 2a's firmware (Helth A13). The face is the stand-in |
| Shipping Ndot or Lettera in the repo | Proprietary, "All Rights Reserved" (A12). The phone's own copies are used |
| A round-dot Doto for the M13 | Helth's vendored cut is square-dotted, and Ndot's dots are round. Only worth doing if you actually use the face on the M13 |
| The face on the website or the Linux app | Today's ring already is that circle there. The face is the phone's AOD stand-in |
