# Motivation — plan

Written 2026-09-04, against `index.html` as of 4139 lines (commit `ff98ce2`).
Implemented the same day; see the status line below.

Method: line-level read of the Today view markup (1362–1511), `renderToday`,
`companionMood`, `streakInfo`, `badgeProgress`, `awardBadges`, `cue`, `inQuiet`,
`DEFAULTS`, and the settings bind block (3929–3998). No code was run for this
document — everything below is read off the source, and the two places where
that is a weaker claim than a measurement are labelled.

**Status: B1, B2, B3 and B4 are implemented and verified.** 66 checks, 0 failures,
against a Playwright harness driving real clicks. A6 is confirmed, not unconfirmed:
boundary 5 at 23:30 reports 5.5 hours left and fires the warning. The out-of-scope
list below still stands.

---

## Part A — findings, ranked

None of these are correctness bugs. The app does what it says. The finding is
that what it says is entirely about the past.

### A1 · MODEL GAP (high): every incentive in the app is retrospective

Streak, XP, badges, heatmap, week strip, subject split, companion mood — all six
render a fact about time already logged. `renderToday()` produces an identical
Today view at 09:00 with fourteen free hours ahead and at 23:40 with a 30-day
streak twenty minutes from dying. Nothing in the file reads the clock against
what is still possible.

The concrete failure: a tracker whose entire feedback loop fires *after* the
block cannot influence whether the block happens. It measures motivation, it
does not supply any.

**Fix:** three of the four items in Part B, each reading state that already
exists. No new stored data except one reminder time and one "already nudged
today" key. Smallest correct fix because `streakInfo()` and `badgeProgress()`
already compute every number needed — what is missing is a *when*, not a *what*.

### A2 · DESIGN RISK (high): the app's own worst-outcome rule is not applied to the streak

`memory.md § Product judgement` says destructive-by-surprise is the worst
failure class and names "resetting a 100-day streak" as the example. The app
guards this on the write side (grace days, the trailing-grace trim, undo on
delete) and not at all on the *do-nothing* side. Letting a 30-day run expire
because the evening got away from you is the same loss, and the streak tile at
20:00 on an unlogged day is byte-identical to the one at 20:00 on a logged one:
`renderToday` branches only on `st.current` and `st.graceUsed`, never on
`doneOn(todayKey())` or the hour.

**Fix:** A1's first item. One branch in the existing tile — no new card, no new
state, no notification required for it to work.

### A3 · MODEL GAP (medium): the nearest badge is computed and then hidden

`badgeProgress()` returns the exact distance to all 26 badges on every render.
The Progress view spends it on a 26-cell grid where "4 more sessions" and
"430 more hours" are the same size. The one number with any pull — the closest
unearned badge — is never surfaced on the view you actually sit on.

**Fix:** derive `min(need - have)` over unearned badges and print one line on
Today. ~10 lines, zero new state, no new badge.

### A4 · MODEL GAP (medium): the app cannot ask for anything

`Notification` is wired (permission flow at 3974–3988, `cue()` at 2709–2715) but
only ever fires *at the end of a block that is already running*. There is no
path by which the app says anything at a time you chose. Opening it is entirely
on you, and a tracker you forget to open motivates nothing.

**Ceiling, stated because it changes what is worth building:** a page with no
push server cannot notify from a fully closed tab. A scheduled `setTimeout` runs
only while a tab is open (backgrounded is fine, installed-PWA-backgrounded is
fine, closed is not). Anything promising otherwise would be the app lying about
its own confidence, which is the one thing this codebase is built not to do.
So the reminder ships in two halves and the setting says which is which.

**Fix:** B2, one reminder time, both halves.

### A5 · COSMETIC (low): the companion never asks for anything

Four moods, all descriptive. `dozing` after two unlogged days is well-judged and
should not become nagging — but `alert` ("Hopping about the branch") is a wasted
slot on a day with nothing logged and hours left.

**Fix:** none proposed on its own. B1's copy lands next to it and does the job;
adding a fifth mood to carry a nudge duplicates the streak tile.

### A6 · Unconfirmed

The at-risk window in B1 depends on `dayBoundaryHour` arithmetic I have read but
not run against a non-zero boundary. Reconstruct with `dayBoundaryHour: 5` and a
23:30 clock before shipping — that is the case where "hours left today" and
"hours left on the calendar" disagree, and the one most likely to be wrong.

---

## Part B — the build

Ranked by payoff per line. Each is independently shippable; B1 alone is worth
the change.

### B1 · The streak tile knows what time it is  (~20 lines, no new state)

In `renderToday`, when `st.current > 0` and `!doneOn(todayKey())` and fewer than
6 hours remain before the day rolls over:

- tile takes `chip--warn`, not a new colour
- copy states the stake and the cost of avoiding it, in that order:
  *"14 days ends in 3h 20m. Five minutes keeps it."*
- if a grace day would bridge tonight, it says **that** instead:
  *"14 days — a grace day covers tonight. 1 left this month."* Overstating the
  risk when `graceLeft > 0` is the tool lying about its confidence.
- streak 0 gets no warning. Nothing is at risk.

Hours left = next occurrence of `dayBoundaryHour`, so it tracks the setting
rather than assuming midnight. Re-render on the existing minute tick.

### B2 · One study reminder  (~45 lines, +2 settings keys)

Settings → a time and an on/off switch. Default **off**, per the never-silently-
configure rule.

Two halves, both listed in the settings help text:

| Half | Works when | What it does |
|---|---|---|
| Live | tab open or backgrounded | fires the existing `cue()` notification path at the set time |
| Catch-up | always, fully offline | on next open, if the time has passed and nothing is logged today, a dismissible banner in `recoveryHost` |

Suppressed when the day already has a logged minute — a reminder to do what you
did is noise. Fires once per `dayKey`, tracked by one stored key.

`// ponytail: one time, every day. Weekday masks if a weekend reminder turns
out to be the thing that gets ignored into disabling the feature.`

### B3 · Nearest badge on Today  (~10 lines, no new state)

One line under the streak/goal tiles: *"4 sessions from Fifty sessions."*
Smallest remaining gap across unearned badges from the existing `badgeProgress()`
call. Hidden at 26/26. Not a new card — a line.

### B4 · A five-minute block  (~15 lines, +1 timer field)

A secondary button beside Start that runs one 5-minute focus block, then stops
rather than starting a break. The streak needs one logged minute and the honest
observation is that starting is the whole problem; a 25-minute commitment at
22:50 gets refused, five minutes does not.

Needs `T.overrideSec` respected in `phaseSeconds`, cleared in `stopAll`. It logs
as `source: "pomodoro"` like any other block — it is a real block, not a
different kind of time, and must not get its own row in the split or its own
badge.

---

## Out of scope

- **Anything social.** No leaderboard, no sharing, no accountability partner.
  The app has no backend by design and this is not the feature that earns one.
- **New badges.** 26 is enough; adding a "5-minute block" badge would reward the
  escape hatch instead of the studying.
- **Streak freezes, XP multipliers, currency.** One XP is one focused minute and
  that readability is worth more than any of them.
- **Coursework integration.** `course_credits.csv` and Bob the Builder's
  timetable could drive "you have ECO354 tomorrow" — real, and a separate plan.
  It couples two apps and needs its own data decision.
- **Generated encouragement copy.** Fixed strings. A tracker that improvises
  praise is a tracker you stop believing.
- **Touching the companion's mood set.** A5 explains why.
- **Schema version bump.** B2's two settings keys land through the existing
  `DEFAULTS` key-fill in `migrate()` (1995–2007), which already backfills any
  missing settings key on every load. Confirm by seeding a v1 blob before
  shipping; if it does not hold, B2 gets a bump and a migration in the same
  change, never later.

---

## Verification, when this is approved

- Backup `index.html` timestamped before the first edit.
- Playwright, clicking real controls: seeded 30-day streak with today empty at a
  faked 21:00 → warn state; same state with `graceLeft: 1` → grace copy; streak 0
  → no warning; `dayBoundaryHour: 5` at 23:30 (A6).
- Old-schema blob restored → both new settings keys present, no data lost.
- Both themes, and at least one palette where the accent is already red
  (Arasaka) — the warn state must not vanish into it.
- Zero console errors, fresh profile and seeded profile.
