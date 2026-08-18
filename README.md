# Study Tracker

A pomodoro timer, study heatmap and streak tracker that runs from a single HTML
file. No backend, no account, no network call of any kind — every session you
log lives in your own browser's `localStorage` under one key, and the only copy
that outlives that browser is the backup you export yourself. Twenty-six colour
palettes, twenty of them ported from my terminal schemes.

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
| Daily and weekly goals | Off by default, and both kept separate from the streak |
| Heatmap | 364 days, shaded relative to your own history, filterable by subject |
| Week strip | Seven Monday-start bars with the daily-goal line drawn across them |
| Subject split | Where the time actually went, over 30 days, 90 days, or all time |
| Levels and badges | 26 badges in five families. One XP is one focused minute |
| 26 palettes | 5 light, 21 dark. Mode and palette are separate choices |
| Backup and export | Full JSON backup, CSV of every session, restore, and an undo on every destructive action |

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

## Solid vs. assumed

| Solid — measured | Assumed — a choice I made |
|---|---|
| Minutes logged, per session and per day | The 26 badge thresholds. Round numbers, nothing derived |
| Current streak, longest streak, grace used | That one grace day per month is a sensible default |
| Totals, per-subject splits, best day | That a block under 60 seconds is not worth logging |
| Which palette clears which contrast ratio | That focused minutes are the right unit to count at all |
| Recovered time, capped and tagged `interrupted` | That the day boundary belongs at midnight until you move it |

The heatmap sits between the two columns: the minutes are measured, the *shade*
is a percentile of your own history, and the caption states which of the two
modes produced it.

## Data and privacy

Everything lives in `localStorage` under the single key `studyTracker.v1`, with
an integer `SCHEMA_VERSION` and a migration that runs on every load rather than
only on a version bump. Nothing is transmitted anywhere: no analytics, no font
CDN, no remote asset, no fetch to any origin. GitHub Pages serves four static
files and never sees a byte of what you log.

Export before you clear site data, switch machines, or do anything else that
takes a browser profile away. Settings → Your data gives you a full JSON backup
that restores exactly, plus a CSV of every session for anything else.

## Deployment

Deployed from `main` at the repository root — no build step, no bundler, no
generated output. The four files that ship are `index.html`, `sw.js`,
`manifest.webmanifest` and `icon.svg`. Pushing to `main` is the deploy.

The service worker is cache-first over a versioned cache and precaches the
shell, so the site opens with no network once visited. `CACHE` in `sw.js` is
bumped on every release; without that, an installed copy keeps serving the old
shell forever.
