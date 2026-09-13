# Study Tracker

A pomodoro timer, study heatmap and streak tracker that runs from a single HTML
file. Every session you log lives in your own browser's `localStorage` under one
key. Optional cloud sync keeps two devices in step through a Supabase project
you own — off until you set it up, and the app is byte-for-byte as offline as it
ever was until you do. Twenty-six colour palettes, twenty of them ported from my
terminal schemes.

**Live: <https://talon270.github.io/study-tracker/>**

## Running it

Nothing to install, nothing to build.

```sh
git clone https://github.com/talon270/study-tracker.git
cd study-tracker
xdg-open index.html          # or just double-click it
```

Opening `index.html` straight off the filesystem works completely — `file://` is
a first-class target, not a fallback. The only thing it costs is the service
worker, which browsers refuse to register on `file://`. Serve the folder over
HTTP (`python3 -m http.server`) or use the live link if you want offline caching
and installable-app behaviour.

## What's in it

| Feature | What it does |
|---|---|
| Pomodoro timer | Wall-clock countdown inside a depleting ring. Configurable focus, short break, long break, and rounds before a long break |
| Crash recovery | A tab that dies mid-block is credited on reopen, capped at one focus block, with a banner offering to delete it |
| Manual entry | Log time you studied without a timer running. Crossing midnight is handled |
| Streaks | Any logged minute keeps the day. Configurable grace days per month |
| Streak at risk | In the last 6 hours of a day with nothing logged, the tile says what is about to be lost — or that a grace day covers it |
| Five-minute block | A second start button that runs one 5-minute block and queues no break |
| Daily reminder | Off by default. A banner on next open, plus a notification while a tab is open |
| Next badge | The closest unearned badge of the 26, printed on Today |
| Daily and weekly goals | Off by default, and both kept separate from the streak |
| Heatmap | 364 days, shaded relative to your own history, filterable by subject |
| Week strip | Seven Monday-start bars with the daily-goal line drawn across them |
| Subject split | Where the time actually went, over 30 days, 90 days, or all time |
| Levels and badges | 26 badges in five families. One XP is one focused minute |
| 26 palettes | 5 light, 21 dark. Mode and palette are separate choices |
| Backup and export | Full JSON backup, CSV of every session, restore, and an undo on every destructive action |
| Cloud sync (optional) | Phone and laptop share one log through your own Supabase project. Off by default — see [`SETUP-sync.md`](SETUP-sync.md) |

## The things most study trackers get wrong

**Paused time is not study time.** The timer counts from the wall clock rather
than decrementing a number, so a backgrounded and throttled tab never drifts
minutes behind — but it only accumulates while actually running. Pause for
twenty minutes inside a 25-minute block and you are credited the minutes you
sat there, not 25. A block left paused for over 30 minutes closes itself and
keeps what it earned, because a session paused overnight otherwise sits in
storage and reappears as a phantom the next morning.

**A recovered session is capped at one focus block.** If the tab dies at 14:05
and you reopen it at 22:00, the naive fix credits eight hours. This one credits
at most one configured focus block, writes `Recovered after the app closed
mid-session` into the note, and shows a banner with a Delete button. A tracker
that can invent time is worse than one that loses it.

**The streak tile reads the clock, and refuses to overstate the stake.** Every
other number in the app describes time already logged, which means none of them
can affect whether a block happens. In the last 6 hours before the day boundary,
with a run going and nothing logged, the tile turns and names the stake and the
cost in that order: *"3 days · Ends in 3h 00m · One logged minute keeps it."*
The refusal is the other half. With a grace day left the run **holds** whatever
you do tonight, so it says that instead — *"a grace day bridges tonight, 1 left
this month"* — and drops the alarm styling. Frightening you with a loss that
isn't coming would work exactly once.

**The five-minute block has its own cap, in one place, for a reason.** Four
separate things cap against the length of a work block: the ring, the credit at
finish, the credit on Stop, and the `plannedMinutes` written into
`activeSession` for crash recovery. Read any of them from `settings.workMin` and
a 5-minute block that died mid-run comes back as 25 — twenty minutes you did not
study, invented by the one feature meant to lower the bar. `workCapMin()` is the
only answer to "how long is this block", and the verification asserts a quick
block backdated by 30 minutes credits exactly 5 while a normal one credits 25.
It logs as an ordinary `pomodoro` session, but it does not advance the round
counter and queues no break: five minutes has not earned one.

**The reminder ships as two named halves because they are not equally
reliable.** A page with no push server cannot wake a closed tab, and this one
has no server. So the settings text says which half is which rather than
promising a notification that may never arrive: the **banner on next open** is
fully offline and always works, the **notification** only fires while a tab is
open or backgrounded. Both go quiet the moment the day has a logged minute — a
reminder to do what you already did is noise — and it fires at most once per
day. The time it fires at belongs to the *logical* day, so with the boundary at
05:00 a 19:00 reminder is seven hours overdue at 02:00, not seventeen hours
early.

**Whether this device already nudged you is not news your phone wants.**
`nudgedOn` sits beside `activeSession` on the list of things stripped from every
upload. Sync it and dismissing the banner on a laptop silently swallows the
reminder on a phone that never showed one. It is written with `saveLocal()` for
the same reason the heartbeat is — it is not a decision you made, so it must not
restamp the timestamp that decides whose settings win a merge.

**A session id from outside this browser is untrusted input.** Ids are
interpolated straight into HTML attributes by the log table — `data-id`,
`data-edit`, `data-del` — and two paths carry ids in from elsewhere: restoring a
backup file, and pulling a row from Supabase. An id of
`s_1' onfocus='…' autofocus x='` broke out of that attribute and ran. The fix is
one regex at `migrate()`, the single door every external blob comes through,
rather than three `esc()` calls at the call sites — that way a fourth
interpolation added later is covered too. A failing id is **regenerated, not
dropped**, so a hand-edited backup still restores every session it contains, and
real ids (`s_<millis>_<base36>`) always pass, which matters because sync unions
sessions by id.

**The day can roll over underneath an open tab, and everything on Today is keyed
to that day.** Leave the app open past midnight and the session list, the day
total, the goal ring and the sidebar all still described yesterday — and the
manual-entry date still defaulted to it, which silently logs to the wrong day.
That is the exact failure this app exists to not have. The minute tick now
compares `todayKey()` against the last one it saw and forces a full repaint when
it changes. The manual date is only reset if it was still sitting on the old
default: someone who deliberately set it to a past day keeps that choice.

**The streak walked the whole log once per day of the streak.** `doneOn()` is a
full scan of every session, and the streak loop called it once for every day in
the run. At two years of daily study — 2,190 sessions, a 730-day streak — that
was 730 scans, measured at 31ms, and `streakInfo()` runs several times per
render plus once a minute. Building the day map once locally instead took it to
**1.5ms**; `renderAll` went 88.5ms → 12.8ms and Progress 172ms → 41ms. The map
is local to the call rather than cached, so there is no invalidation to get
wrong and a stale streak is not a failure this can have.

**The heatmap is a listbox, because what you do with it is pick one day.** 371
cells with a click handler and no `tabindex` meant a keyboard user could not
select a day at all, and the claim that "every cell prints its real minutes on
click" was true for mice only. Arrow keys now move a roving tab stop — up/down a
day, left/right a week, matching what the column layout looks like it should do —
Enter selects, and focus survives the re-render. One tab stop, not 371, because
tabbing through a year of squares to reach the log below is its own defect.

**Picking a day on the heatmap changes two cells and one line, so that is
all it touches.** It used to call the whole History render, which rewrote the
371-cell grid, the 50-row session table with its 100 inputs, the subject select
and the stats tiles — producing markup identical to what was already on screen.
The rewrites themselves were cheap (1.3ms and 3.1ms); the layout they forced was
not (7.4ms and 21.8ms). Measured before and after on the same script: **89ms →
1ms** per click. The subject filter still rebuilds everything, because it changes
the shading, and that genuinely is every cell.

**The streak loop is bounded by your data, not by a magic number.** It used to
run `while (guard++ < 3000)` — an arbitrary 8.2 years, past which a longer streak
silently stopped counting and reported a shorter one, with nothing saying so. A
run can never begin before the first day you logged anything, so that is the real
bound. Seeded with 3,200 consecutive days, the old code reported **3000** and the
new one reports **3200**. It is also faster, because it stops when the data runs
out instead of walking empty days.

**Changing view moves focus into the view.** `<main>` already carried
`tabindex="-1"`, which is the affordance for exactly this and was going unused.
Without the move, activating History from the keyboard left focus on the nav
button: nothing announced that the content had changed, and the next Tab
restarted at the top of the nav instead of entering the view you just asked for.

**Grace days bridge a streak; they never start one.** With one grace day a
month, missing Tuesday does not zero a run spanning Monday and Wednesday. But a
run can never *begin* on a day you did not study — the calculation trims
trailing grace days off the end for exactly this reason. Without that trim,
taking two days off and studying once would report a three-day streak.

**The heatmap says what its shading means, and changes it when it can't.**
Shades are percentiles of your own last 90 days, GitHub-style, which means a
shade's meaning drifts as your habits change. Below 15 active days percentiles
are noise, so it switches to fixed cuts at 1x / 2x / 4x your focus length and
the caption underneath names which mode it is in — *"Shading is fixed until you
have 15 active days (6 so far)"*. Every cell also prints its real minutes on
click, so colour is never the only source of truth.

**The streak and the goal answer different questions and are never merged.** The
streak says whether you showed up. The goal says how much. A five-minute day
genuinely is both a kept streak and a missed goal, so it renders as both.
Missing a goal never touches a streak, and the weekly goal never touches either.

**Every date key is local time, derived from one configurable setting.** The day
boundary can be pushed as late as 6am, so studying at 23:00 and logging at 00:20
lands on the day it belongs to. Changing that setting re-derives the `dayKey` on
every stored session in the same operation and offers an undo — leaving them
alone would silently move your history.

**Deleting is instant with an undo, never a confirmation dialog.** Reversing a
mistake beats interrupting every correct action to ask about it. There is not a
single `alert()` or `confirm()` in the file; the only match for that string is
the comment explaining why there isn't one.

**One XP is one focused minute.** No multipliers, no hidden weighting, no streak
bonus. That keeps the number readable as "minutes studied, ever" instead of a
score you have to reverse-engineer. Level *N* needs 300 x N(N-1)/2 XP, and the
remaining amount is printed next to the bar as raw minutes.

**Every palette is contrast-checked against the surface it actually lands on.**
The 20 dark palettes come from Material 3 terminal schemes with a pure `#000000`
ground, which gives cards nothing to sit on — so each one's surface ladder is
lifted off its own `surfaceVariant` and overlay ramps, keeping the palette's hue
instead of going grey. Dim text is then nudged toward the foreground until it
clears 4.5:1 on the card, and the accent until 3:1. 25 of the 26 clear every
threshold. The exception is Paper, whose `#EF9F27` sits at 2.1:1 on its own
card: that is a deliberate keep, and every accent *text* use routes through a
separate `--accent-ink` token at 6.4:1 instead.

**Success is green even when the accent is red.** In Arasaka a met goal is not
another red chip. Palettes whose accent is already green keep it; the rest get a
real green, because semantic clarity beats palette purity for the one colour
that means "you did the thing".

**Sync merges, it never overwrites.** Two devices can each log offline for days.
Last-write-wins on the whole blob is the obvious implementation and it silently
deletes every session on the losing side — you would find out weeks later, from
a heatmap with a hole in it. Sessions union by id instead, so a block survives if
either device has it. Only settings and badges use a timestamp, and only because
half-merged settings mean nothing; badges take the *earlier* of two timestamps,
because a badge was earned when it was first earned.

**Syncing is not editing.** The timestamp that decides a settings conflict is
stamped by real changes only, never by the sync itself or by the timer's 15-second
heartbeat. Stamping it on sync looks harmless and is not: a laptop that merely had
the app open would outrank a phone where you actually changed a setting an hour
earlier, and quietly revert it. The heartbeat is the same defect in miniature —
left on the syncing save path it would fire a network write every 15 seconds of
every study block, carrying nothing new, because the field it touches is stripped
before upload anyway.

**A running timer never leaves the device it runs on.** `activeSession` is
excluded from the upload. Sync it, and a second device pulls a live block and
"recovers" it on next open — crediting itself minutes you sat through somewhere
else. The one thing this app must never get wrong is logging time you did not
study, and a sync layer is the easiest possible way to introduce exactly that.

**A failed sync says so, in the words the server used.** The status chip has
seven distinct states and prints Supabase's own error text underneath, rather
than a generic "couldn't sync". `Email not confirmed` and
`new row violates row-level security policy` are different setup mistakes with
different fixes, and flattening them into one message costs an hour of guessing.
Local storage is always written before the request is attempted, so a failure
delays when the other device sees a session — it can never lose one.

## Solid vs. assumed

| Solid — measured | Assumed — a choice I made |
|---|---|
| Minutes logged, per session and per day | The 26 badge thresholds. Round numbers, nothing derived |
| Current streak, longest streak, grace used | That one grace day per month is a sensible default |
| Totals, per-subject splits, best day | That a block under 60 seconds is not worth logging |
| Which palette clears which contrast ratio | That focused minutes are the right unit to count at all |
| Recovered time, capped and tagged `interrupted` | That the day boundary belongs at midnight until you move it |
| Hours left in the day, from your own boundary setting | That 6 hours is the right point to start warning |
| Which unearned badge is closest | That a quick block should be 5 minutes, and queue no break |
| Render cost at 2,190 sessions, measured in the browser | That 364 days is the right heatmap window |
| Which sessions exist on each device, matched by id | That on a settings conflict, the more recently edited device is the one you meant |

The heatmap sits between the two columns: the minutes are measured, the *shade*
is a percentile of your own history, and the caption states which of the two
modes produced it.

## Data and privacy

Everything lives in `localStorage` under the single key `studyTracker.v1`, with
an integer `SCHEMA_VERSION` and a migration that runs on every load rather than
only on a version bump. With sync off — the default — nothing is transmitted
anywhere: no analytics, no font CDN, no remote asset, no fetch to any origin.
GitHub Pages serves four static files and never sees a byte of what you log.

With sync on, the single destination is the Supabase project you created and
control. There is no intermediary and no client library: the app talks to your
project over four plain `fetch` calls, which is why the page still has zero
dependencies and still opens offline. Your access tokens are kept in a *separate*
`localStorage` key from the log, deliberately — the log is what gets uploaded and
what a JSON backup dumps, and tokens belong in neither.

Export before you clear site data, switch machines, or do anything else that
takes a browser profile away. Settings → Your data gives you a full JSON backup
that restores exactly, plus a CSV of every session for anything else. **Two
synced devices are not two backups** — they converge, so a bad merge reaches
both. Keep downloading the JSON occasionally regardless.

## Deployment

Deployed from `main` at the repository root — no build step, no bundler, no
generated output. The four files that ship are `index.html`, `sw.js`,
`manifest.webmanifest` and `icon.svg`. Pushing to `main` is the deploy.

The service worker is cache-first over a versioned cache and precaches the
shell, so the site opens with no network once visited. `CACHE` in `sw.js` is
bumped on every release; without that, an installed copy keeps serving the old
shell forever.
