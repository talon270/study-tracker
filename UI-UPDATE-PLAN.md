# Study Tracker — UI update plan

Written 2026-09-29, against `index.html` as of 5,836 lines.
Source SHA-256: `16c9c8e680f88ddc86d81c927aedaea48ef52cc3cd45ccd2c7d43854d0aee98f`.

**Direction: restrained neobrutalism.** Warm paper, near-black ink, one muted
yellow accent, bold type, crisp borders and selective hard shadows. The app
already uses one accent for its charts; the largest visual change comes from
geometry, hierarchy and typography.

**Nothing below is implemented — this is the plan.** Writing this document
does not approve implementation. Existing changes in the working tree belong
to the work already in progress and must be preserved.

## Method and evidence

The review covered `README.md`, the palette and component CSS, the four view
templates, and their rendering functions. Playwright opened the local HTML
file in isolated Chromium contexts, then used actual navigation and theme
buttons. Profiles were empty or seeded with 41 synthetic sessions across
28 days, including subject groups, interruptions, and daily and weekly goals.
No personal browser profile or sync credentials were used.

| Viewport | Views checked | Theme |
|---|---|---|
| 390 × 844 | Today, History, Progress, Settings | Light |
| 390 × 844 | Today | Dark |
| 1440 × 1000 | Today, History, Progress, Settings | Light and dark |
| 1920 × 1080 | Today | Light |

The seeded inspection recorded zero page errors and no horizontal overflow
of the whole page. The heatmap and session tables have their own horizontal
scroll regions. Those are usability findings below, rather than page overflow.
Android and the desktop Tauri shell were not run; native behaviour is
unconfirmed by this review.

The review script, screenshots and measurements are in
`/tmp/claude-study-ui/scratchpad/`. They are temporary evidence. The concrete
measurements are recorded here so the plan remains useful after those files
disappear.

## Product constraints

**The timer is the primary action.** Today should help you choose a subject
and start a block quickly. History should make a session easy to find and
correct. Progress should explain what you have earned. Settings should make
the preferences you use regularly easy to reach.

**Logged time must retain its meaning.** Running time, paused time, breaks,
recovered sessions and manual entries keep their existing accounting. A streak
and a daily goal remain separate. Warnings about recovery, past dates, sync
failure and an endangered streak remain visible beside the relevant action.

**Reduced colour still has to communicate state.** Reserve persistent accent
colour for the main action, active navigation and current progress. Success,
warning and error colours remain available when they carry meaning, paired
with words or icons. The heatmap keeps its shading explanation and exact
minutes on selection. See [W3C's use-of-colour guidance](https://www.w3.org/WAI/WCAG22/Understanding/use-of-color).

## Part A — findings, ranked

### A1 · DESIGN RISK (medium): the initial phone layout hides Start

Source: Today's form and timer, `index.html:1510`; `.timer` and
`.timer__actions`; the phone media rules at `index.html:1241`.

At 390 × 844, Start begins at y = 805.6 and is 46px high. The fixed bottom
navigation occupies y = 782–844. The button therefore sits behind the
navigation and below the viewport on initial load. The result is the same
with empty and seeded data. Scrolling reveals it; the finding concerns the
first screen, rather than a permanently inaccessible control.

The introductory paragraph, optional Group field and its explanation consume
space before the timer.

**Fix:** keep Subject, timer and Start focus together on the initial phone
screen. Put Group and its explanation in an expandable More options section,
and condense the introduction. Preserve a visible indication of any active
group when that section is closed. This removes optional material from the
critical path without changing how a block is logged.

### A2 · DESIGN RISK (medium): the desktop heatmap is squeezed into one column

Source: History's `.dash` template, `index.html:1661`; `.heat-scroll` at
`index.html:1117`; `renderHistory()` at `index.html:4763`.

At 1440px, the heatmap needs 861px but receives a 604px scroll viewport.
The statistics rail occupies the other column, so a year of history requires
horizontal scrolling even on this desktop viewport.

**Fix:** give the heatmap a full-width row. Place the session list and compact
statistics beneath it. Keep horizontal heatmap scrolling on narrow screens,
with the current day visible on entry and an evident scroll affordance. This
changes the layout while preserving cell sizing and keyboard navigation.

### A3 · DESIGN RISK (medium): phone session tables conceal information sideways

Source: `logTable()` at `index.html:4660`, `wireLogHost()`, and `.log-scroll`
at `index.html:1168`.

At 390px, Today's log needs 552px inside a 356px region. History's log needs
569px inside a 356px region. Subjects, groups, minutes, source and delete
actions compete for the same narrow row. The desktop History view also gives
subject inputs little room when the statistics rail is present.

**Fix:** retain a table on desktop and use readable session rows on phones.
Show subject and duration on the first line, then date/time, group and source
below. Open editing explicitly from the row. Reuse the existing edit, delete
and undo behaviour; every field remains reachable. This adapts presentation
without introducing a second session model.

### A4 · DESIGN RISK (medium): Today gives supporting modules too much weight

Source: Today's `.dash__side`, `index.html:1575`; `renderToday()` at
`index.html:4391`; `renderCompanion()`, `renderWeek()` and `renderNextBadge()`.

The companion, streak, goal, next badge, week chart and subject split each
occupy separate frames. At 1440px the streak and goal stack vertically because
the right column cannot fit the two 260px minimum-width children. The right
column then extends substantially below the timer and session list.

**Fix:** lead with a compact summary of today's minutes and streak, keeping
their meanings distinct. Reduce the companion to a smaller illustration in
that summary. Group supporting statistics with dividers, and place the week
and subject analysis below the primary workspace. Give recovery and risk
messages priority when present. This changes emphasis using existing data.

### A5 · DESIGN RISK (medium): many labels and explanations are very small

Source: font tokens at `index.html:489`; `.head__eyebrow`, `.field__label`
at `index.html:756`, `.stat__label`, `.help` and `.badge__p`.

Several labels use 10–10.5px type and supporting copy uses 11.5px. Long
explanations at these sizes add density without making the primary controls
easier to identify. The review browser's system-font configuration also makes
the page look predominantly monospaced; that appearance is environment
dependent, rather than an explicit body-font rule in the app.

**Fix:** use a deliberate sans serif for headings and prose, reserving
monospace for time, dates and measurements. Increase useful labels and shorten
repeated explanations. Keep information that explains a metric or prevents a
mistake attached to that metric or action.

### A6 · NOISE (low): the palette library dominates Settings

Source: Settings' Appearance block, `index.html:1778`; `renderSettings()` at
`index.html:5056`; `renderPalettes()` at `index.html:5322`.

The expanded light and dark palette galleries precede Pomodoro, day boundary,
goals and cues. Twenty-six previews make reaching frequent preferences a long
scroll, especially on a phone.

**Fix:** show the current light/dark pair with a Change palette action that
expands the gallery. Group Settings into Study, Appearance and Data, with
in-page navigation. Preserve every existing palette and the distinction
between mode and palette. This improves access without removing choice.

### A7 · COSMETIC (low): the component geometry reads as soft and rounded

Source: radius tokens at `index.html:499`; `.card` at `index.html:675`;
`.btn` at `index.html:707`; `.chip` at `index.html:740`; `.seg` at
`index.html:796`; palette shadow tokens and `body::before` at
`index.html:525`.

Cards use 16px corners, most controls use 11px corners, chips are pills, and
shadows are blurred. Background washes and tinted icons reinforce that
appearance. These are design choices, rather than functional defects.

**Fix:** establish shared border, radius and hard-shadow tokens. Use stronger
framing on the timer and primary actions, simpler frames or dividers for
supporting content, and flat neutral surfaces. Shared tokens make all four
views adopt the same visual language.

## Part B — design specification

### Colour and shape

**Proposed pair: Paper + Ochre and Charcoal + Ochre.** Add them as recommended
choices in the existing palette system. Preserve saved mode and palette
preferences. A fresh profile may use this pair as its defaults; an existing
profile adopts it through the picker.

| Token or treatment | Light | Dark |
|---|---|---|
| Ground | Paper `#F4F1E8` | Charcoal `#191919` |
| Main surface | Off-white `#FCFAF4` | Raised charcoal `#242423` |
| Main text | Ink `#20201E` | Paper `#F4F1E8` |
| Accent | Ochre `#D7B95E` | Ochre `#D7B95E` |
| Primary accent text | Ink `#20201E` | Ink `#20201E` |
| Major borders | 2px ink outline | 2px contrasting neutral outline |
| Corners | 4px for frames and controls | Same geometry |
| Main-action shadow | 4px × 4px, 0 blur | Same geometry, dark neutral shadow |

These colours are proposed values. Contrast has not been verified for the new
pair. Define separate accent-text and outline tokens where necessary, and
check text at 4.5:1 and essential control boundaries at 3:1 on their actual
surfaces before accepting them.

Apply the geometry consistently across existing palettes too. Use neutral
icons by default and accent colour for meaningful selection. Keep heatmap
intensity distinguishable through lightness and retain its numerical detail.

### Type, spacing and interaction

| Element | Proposed treatment |
|---|---|
| Page heading | Bold sans serif, 32–36px desktop and 26–30px phone |
| Timer | Monospace, 72–88px desktop; size to fit the initial phone screen |
| Body and form copy | Sans serif, 14–16px, with comfortable line spacing |
| Labels | 12–13px minimum target; sentence case for ordinary form labels |
| Measurements | Monospace with tabular numerals |
| Spacing | Existing 4px scale; 16–24px between modules |
| Primary action | Clear label, ink text on ochre, strong outline and hard shadow |
| Active navigation | Solid rectangular selection with clear text and icon |
| Pressed action | Move into its shadow while pressed; preserve layout position |
| Keyboard focus | Distinct outline on every interactive control |

Keep motion short and respect reduced-motion preferences. Aim for touch
targets of at least 44 × 44px on phone controls; this is an enhanced target
size, rather than WCAG's minimum criterion. See [W3C's enhanced target-size
guidance](https://www.w3.org/WAI/WCAG22/Understanding/target-size-enhanced).

If a font is bundled, use a licensed WOFF2 asset or the app's existing embedded
font approach. Include its licence and verify loading from `file://`, the PWA
and packaged assets. The app continues to open without a network connection
or a build step.

### View composition

| View | Intended arrangement |
|---|---|
| Today | Compact summary, subject and timer, primary actions, today's sessions, then supporting analysis. The owl is a smaller part of the summary |
| History | Full-width heatmap, then sessions with compact statistics. Add session expands the manual-entry form |
| Progress | Clear level and next milestone, grouped statistics with dividers, then badge families. Earned badges use an outlined stamp with an Earned label |
| Settings | Study, Appearance and Data groups, quick in-page navigation, and an expandable palette library |

For subjects, replace the segmented split and separate legend with labelled
horizontal bars showing name, minutes and percentage together. Preserve the
range selector, grouping rules, totals and the remainder bucket. Long names
must remain readable on a phone.

## Part C — implementation order

1. **Capture the baseline.** Take a timestamped backup of `index.html` before
   editing. Record screenshots and layout measurements against that backup
   with empty and synthetic profiles. Preserve all existing working-tree
   changes; use the current file as the starting point.
2. **Establish shared visual tokens.** Add the recommended palette pair, define
   borders, radii, shadows and typography, then update cards, fields, buttons,
   chips and navigation. Verify these primitives in both themes before
   restructuring the views.
3. **Make Today fit its purpose.** Condense the header, add More options for
   Group, keep timer actions visible on the initial phone screen, and rebuild
   the summary and supporting-module hierarchy. Retain active-group context,
   recovery banners, reminders and streak-risk text.
4. **Recompose History.** Move the heatmap to its own row, compact the stats,
   and make Add session expand the manual form. Preserve intentional dates,
   validation messages, selected-day detail and heatmap keyboard navigation.
5. **Adapt session presentation.** Add phone rows and explicit editing while
   reusing current mutation handlers. Verify edits to subject, group and
   minutes, plus delete and undo, in both Today and History. A session must
   never appear twice because two responsive presentations exist.
6. **Simplify analysis and Progress.** Add labelled subject bars, group the
   statistics, and restyle badge families and the nearest milestone. Keep XP,
   badges, goals, shading cutoffs and subject aggregation calculations intact.
7. **Organise Settings.** Add group navigation and the expandable palette
   library. Preserve current control bindings, saved choices and all device
   permission/status explanations. Expansion state can remain in memory.
8. **Verify and document.** Run the acceptance checks below, capture the same
   screenshots and measurements as the baseline, and update README details
   that describe the revised presentation or palette count. Refresh the
   service-worker cache version and packaged asset list if shipped assets
   change.

## Acceptance checks

**Use the same browser script before and after.** Drive navigation, theme
selection, timer actions, editing and validation with actual clicks and
keyboard input. Seed only isolated profiles. Keep screenshots and scripts in
the session scratchpad.

| Area | Required result |
|---|---|
| Initial phone screen | At 390 × 844 and 412 × 915, Subject, timer and Start focus are visible above the bottom navigation in the ordinary empty and seeded states |
| Smaller screens | At 360 × 800, controls remain reachable and the page has no horizontal overflow |
| Desktop | Check 1180, 1440 and 1920px widths; content stays centred beside the sidebar and the right rail does not dominate Today |
| Desktop heatmap | The full year fits at 1440 and 1920px without horizontal heatmap scrolling; narrow screens retain usable scrolling |
| All views and themes | Empty, one-session and realistic-volume profiles render correctly in light and dark, with zero page errors |
| Palette compatibility | Existing selections survive reload; representative light, dark and red-accent palettes retain readable text, controls and semantic states |
| Phone session rows | Full subject, duration and editing actions are reachable without sideways table scrolling |
| Editing and undo | Subject, group and minute edits persist; deletion and undo retain the correct session in both views |
| Manual entry | Valid entries log correctly; invalid entries show inline messages; deliberate past dates remain selected |
| Timer | Start, quick block, pause, resume and stop work after restructuring; paused time and breaks receive no study credit |
| Important messages | Recovery, past-date, sync-error, permission and streak-risk explanations remain visible and actionable |
| Keyboard | Navigation moves focus into the view; expanded controls have accessible names and state; heatmap arrows and selection still work |
| Data | Loading an older backup preserves sessions and settings; JSON backup/restore and CSV export still round-trip |
| Offline and delivery | The file opens offline; any new font loads locally; PWA caching includes new assets; Tauri asset copying includes them |
| Native focus face | Shared token changes keep digits, ring and controls legible. Browser checks do not establish on-device behaviour |

Check the fixed bottom navigation, viewport height and safe-area padding on
Android before claiming the phone layout is complete. First-run suggestions
or recovery banners may require scrolling; they must remain prominent and
must not overlap the timer actions.

## Out of scope

This plan does not change session accounting, the storage schema, sync merge
rules, streak arithmetic, badge thresholds, XP, goal calculations, reminders
or native alarms. It adds no accounts, backend, framework or build pipeline.

The Android focus face's layout, Ndot/Doto font chain, ring motion, burn-in
drift and fading controls belong to the existing Android work. This update
checks compatibility with shared colour tokens rather than redesigning that
surface.

Existing palettes remain available. Layout expansion and editing state do not
need new persisted settings. Any later change that requires a data shape or
default-setting migration must be proposed and tested explicitly.

Implementation, commits, publishing and native app releases require their
own instruction. This deliverable is the written UI plan.
