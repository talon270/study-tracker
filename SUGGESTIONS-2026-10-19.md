# Study Tracker — suggestions

Written 2026-10-19, against `index.html` as it sits in the working tree. That
is commit `3eb278f` plus about 1,160 uncommitted lines of Android, ring-driver
and focus-face work.
Method: a read of the uncommitted diff's timer and recovery hunks
(`persistActive`, `workEndAt`, `recoverIfNeeded`, `restoreTimer`, `cue`,
`reminderOn`) and of `load`/`save`. Headless Chromium on `file://` and
`http://`, light and dark, at 390, 1440 and 1920 px, with a 71-click
click-through. A timer suite on a fake clock (2-minute blocks): normal, paused
and resumed, stopped early, paused past 30 minutes, crash mid-block, and crash
then reopen late. It ran against the working tree, against `HEAD`, and against
the working tree with an Android user agent and a stubbed `window.__TAURI__`.

**Three fixes applied in this pass (A1, A1b, A1c). Everything else is a
suggestion and is not implemented.** Backups:
`index.html.backup-20261019-041307` (before A1) and
`index.html.backup-20261019-055243` (before A1c).

Another session added the hairline empty-state figures to `index.html` at
04:24, after A1 landed (its backup: `index.backup-20261019-042359.html`).
Re-checked on the combined file: the A1 probe still keeps the unreadable copy
and shows the banner, and the 71-click smoke run is still at 0 errors.

## Result

Zero page errors and console errors in every run. No sideways scroll at
390 px. The September audit's fixes hold. The new timer code does not regress
the browser path:

| Case | `HEAD` | Working tree (browser) | Working tree (Android stub) |
|---|---|---|---|
| Normal 2-min block | 2 min, 10:00–10:02 | same | same |
| Pause 1 min mid-block | 2 min, ends 10:03 | same | same |
| Stop at 1:30 | "logged 2 min" | same | same |
| Paused past 30 min | 1 min, ends **10:31:01** | 1 min, ends **10:01:00** | 1 min, ends 10:01:00 |
| Close at 1:20, reopen mid-block | 1 min recovered, banner | same | timer restored and running, no banner |
| Close, reopen after the end | 1 min recovered | same | 2 min, "Finished while the app was closed." |

The paused-past-30 row is the intended `workEndAt()` change: the block ended
when you paused it, not when the app noticed.

## Part A — findings, ranked

### A1 · BUG (high): unreadable saved data is overwritten by the first Start press — FIXED

`load()` caught a parse failure and fell through to `migrate(null)`. That gave
an empty profile with no message and no copy. `persistActive()` on the first
Start then wrote the empty profile over the old blob.

Reproduced: seed `studyTracker.v1` with truncated JSON, open, press Start.

| | Original kept anywhere | Message |
|---|---|---|
| Before | no | none |
| After | yes, `studyTracker.v1.unreadable-<ms>` | banner in `#recoveryHost` naming the key |

**Fix (applied):** the same set-aside `vis` does. The raw string is copied to
`studyTracker.v1.unreadable-<ms>` inside the catch, before anything can save,
and `init()` shows a banner. That is 14 lines across `load()` and `init()`.
The ceiling: if storage is too full to hold the copy, nothing is kept and no
banner shows.

### A1b · BUG (high): the service worker deleted every other app's offline cache — FIXED

`sw.js` deleted every cache except `study-tracker-v12` on activate. The
GitHub Pages apps all live on `talon270.github.io`, which is one origin and so
one CacheStorage. So each Study deploy wiped vis's, Docket's, epok's and the
rest's offline copies, and they did the same to Study's. Reproduced: after
another app with this filter activates, `vis` opened offline returns **504
"Offline, and this file was never cached."**
**Fix (applied):** the delete only touches names starting `study-tracker-`,
with a comment saying why. That is one line. Checked on one local origin:
`vis-shell-3` survives Study's activation, a planted `study-tracker-stale-old`
is still deleted, and the app's own cache installs. The same fix went into
9 other apps' workers. `sw.js` was clean in git, so git is the backup.
`CACHE` is bumped to `study-tracker-v13` (with its comment) so installed
copies pick up A1, A1b and A1c instead of serving the cached `index.html`.
Re-checked after the bump: `vis-shell-3` survives and `study-tracker-v13`
installs.

### A1c · BUG (high): a second open tab erases sessions logged in the first — FIXED

Nothing listened for the `storage` event. So a tab left open kept its stale
`S` in memory, and its next save wrote that over everything another tab had
logged since.

Reproduced on a fake clock: tabs A and B open, A finishes a 2-min block, then
B presses Start.

| | Sessions stored after A's block | After B's Start |
|---|---|---|
| Before | 1 | **0** |
| After | 1 | 1 |

**Fix (applied):** a `storage` listener in `init()` adopts the other tab's
copy (`S = migrate(e.newValue)`) and repaints. That is `vis`'s pattern, 10
lines. B's own running timer lives in `T`, not `S`, so it carries on. The
ceiling: two saves within a few milliseconds of each other can still race.
Backup: `index.html.backup-20261019-055243`. The 71-click smoke run is still
at 0 errors.

### A2 · NOISE (low): stopping early rounds half a minute up

`Math.round` on the worked seconds logs 1:30 as 2 min. Measured the same on
`HEAD`, so it is pre-existing. At real block lengths, the most it can invent
is 29 s per session. The README's rule is that a tracker that invents time is
worse than one that loses it. `Math.floor` would honour that, but it would
also log a 59-second stop as 0, which the code then refuses. Your call; not
changed.

### A3 · INCONSISTENCY (low): `PLAN-android.md` still says nothing is implemented

Line 46 reads "Nothing below is implemented. This is the plan." The README in
the working tree documents the Android build as shipped (alarms, focus face,
`studyTracker.device.v1`). `PLAN-audit-2026-09.md` carries a **Status:** line
for this purpose. Add one here before committing. The on-device run (B7) is
the open item, and the README says so.

## Part B — improvements (suggested, not implemented)

- **Commit the Android work**, or split it. About 1,160 lines of
  `index.html`, `main.rs` and `Cargo.toml` are uncommitted, alongside 7
  untracked backups. A crash of the working copy would lose more than any bug
  above.
- **On-device check of B7**: the fake-clock Android stub proves the page
  logic, not Android's alarm delivery or the freeze/kill timing.

## Out of scope

- `src-tauri/` Rust side and the Android `gen/` tree — not audited.
- UI-UPDATE-PLAN.md (restrained neobrutalism) — a design plan, unimplemented,
  not reviewed.
