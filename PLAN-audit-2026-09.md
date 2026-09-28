# Audit, September — plan

Written 2026-09-28, against `index.html` as of 4744 lines (commit `b31953f`).

Method: Playwright, clicking real controls. A baseline sweep loaded three
seeded profiles (empty, 42 days, 730 days at 2–5 blocks a day) in both
themes at 1920, 1440 and 390px, then clicked through all four views. That is
18 loads and 90 view switches. Console errors, horizontal page overflow and
wide-viewport balance were recorded. Sync was tested with two browser
contexts ("devices") sharing one fake Supabase endpoint served by
`page.route`. That exercises the real `Sync` module, the real debounce and
the real merge, with only the network faked. Line-level reads covered
`migrate`, `Sync`, `streakInfo`, `badgeProgress`, `nearestBadge`,
`renderHistory` and `wireLogHost`.

**Status: A1–A5 implemented 2026-09-28, A3 as option (a).** The sync suite
(same script, run against both files): 2/6 pass before and 6/6 after, run twice.
The two that passed before were the undo and restore checks, which pass
trivially when nothing ever gets deleted. The before run also showed something
this plan missed: the old wipe *uploaded an empty log* (cloud 3 → 0). Only the
other device's copy put it back. Baseline sweep after: 0 console errors across
90 view switches. The heatmap at 390px opens with today visible (`scrollLeft`
537, was 0). `nearestBadge()` on the 42-day seed returns Eight in a day. Backup:
`index.backup-20260928-221656.html`.

**What came back clean:** 0 console errors and 0 horizontal page scroll across
all 90 view switches. The layout is balanced at 1920px. The one page error
seen during testing came from the harness firing a synthetic `change` event.
Real Enter and Tab edits commit cleanly, so it is not listed.

**Not covered:** the timer, crash recovery and the streak-at-risk arithmetic
were not re-run. `PLAN-motivation.md` verified them with 66 checks on
2026-09-04, and nothing in them has changed since. The Tauri shell's Rust side
was not audited.

---

## Part A — findings, ranked

A1–A3 only affect you with sync switched on and two or more devices signed in,
for example the desktop app and the phone. With sync off they cannot happen.

### A1 · BUG (high): a deleted session comes back from any other synced device

`mergeStates` unions sessions by id: "a block survives if either device has
it" (README). Nothing records that a session was deleted, so a deletion looks
exactly like the other device simply not having that session yet.

Reproduced with two devices and 3 sessions:

| Step | Device A | Device B | Cloud |
|---|---|---|---|
| start | 3 | 3 | 3 |
| A deletes one in History | 2 | 3 | 2 |
| B opens the app | 2 | 3 | **3** |
| A opens the app | **3** | 3 | 3 |

The delete is reversed on every device, with no message. The toast said
"Deleted 25 min." and meant it.

**Fix:** tombstones, stored as `S.deleted = { id: deletedAt }`.
- `removeSession` writes the tombstone.
- `mergeStates` unions both sides' tombstones and drops any session whose
  tombstone is newer than that session's last edit (`editedAt`, from A2).
- Undo stamps `editedAt = now` on the restored session, so it outranks its own
  tombstone even if another device already pulled that tombstone.

That one rule ("a session survives a tombstone only if it was edited after
the delete") covers both the delete and its undo. This is the smallest fix
because it adds no second code path.
`SCHEMA_VERSION` 6 → 7, and `migrate()` backfills `deleted: {}`.

`// ponytail: tombstones are never pruned. About 40 bytes each, so 1,000
deletes is 40KB. Prune ones older than 90 days if that ever shows up in a
backup file.`

### A2 · BUG (high): an edit is reverted by a device that was offline

On a same-id collision, `mergeStates` keeps the copy from whichever **whole
blob** has the newer `updatedAt`. Any later change on another device, even an
unrelated setting, makes that device's stale copy of every session "newer".

Reproduced:

| Step | Result |
|---|---|
| A edits a session 25 → 50 min | A `[25, 25, 50]`, cloud `[25, 25, 50]` |
| B, offline, changes its daily goal, then reconnects | B `[25, 25, 25]`, cloud `[25, 25, 25]` |
| A opens the app | A `[25, 25, 25]`. The edit is gone everywhere |

The same happens to a renamed subject or a changed group.

**Fix:** stamp `editedAt = Date.now()` on the session at each of the six sites
that change one: the edits and undos for minutes, label and group in
`wireLogHost` (3667–3708). One `touch(s)` helper keeps a seventh site from
being missed later. `mergeStates` then keeps, per id, the copy with the
larger `editedAt`. A session with no stamp counts as 0, and ties fall back to
today's newer-blob rule, so every pre-v7 session merges exactly as it does
now.

### A3 · DESIGN RISK (medium): "Delete everything" says it worked, then everything comes back

The same mechanism as A1, at full scale. Reproduced: A clicks **Delete
everything** and sees "Everything deleted." with 0 sessions. After one reopen
of each device, both are back to 3.

There are two honest behaviours, and they are genuinely different, so **this
one is your call:**

| Option | What happens | Risk |
|---|---|---|
| **(a) This device only (recommended)** | When signed in, the wipe also signs out. The message says: "Deleted from this browser and signed out. The cloud copy and your other devices still have all 214 sessions." | None. It is the only version where one button can't erase every device |
| (b) Everywhere | Tombstones every id and pushes, so every device empties on its next sync | One click erases every copy. Undo only works in this tab until the toast goes. "Two synced devices are not two backups" becomes literal |

Recommendation: (a). Destructive-by-surprise is the worst failure class, and
(b) turns one button into the most destructive action in the app.

### A4 · MODEL GAP (medium): "Next badge" is stuck on a badge you've made no progress toward

`nearestBadge()` ranks by `need - have` across badges measured in different
units: minutes, sessions, days and one-off events. "Early start" and "Night
owl" need exactly 1, so their gap is 1 until earned. For anyone who never
studies before 08:00 or after midnight, the Today line reads "Early start —
0 / 1" permanently.

Measured on the 42-day seed, the line shows **Early start, 0% done**. Ranked by
fraction done, the real nearest are:

| Badge | Progress | Done |
|---|---|---|
| Eight in a day | 5 / 8 blocks | 63% |
| Twenty-five in one | 920 / 1500 min | 61% |
| Two hundred | 111 / 200 sessions | 56% |

**Fix:** rank by `have / need` instead of `need - have`. That is a one-line
change in `nearestBadge()`. A 0% badge sorts last, and still shows when it is
the only one left.

### A5 · INCONSISTENCY (medium): on a phone, the heatmap opens a year in the past

At 390px the heatmap grid is 861px wide inside a 324px `overflow-x:auto`
wrapper. Nothing sets `scrollLeft`, so it opens on the oldest columns. The
visible range is Oct–Feb of last year, and today sits about 540px off-screen
to the right. With about 6 weeks of real history, a phone shows an entirely
empty heatmap until you scroll.

**Fix:** on `show("history")`, set `.heat-scroll`'s `scrollLeft` to its
maximum. That is one line. It runs on view entry only, so a manual scroll
survives selecting a day.

---

## Part B — the build

Each step can ship on its own. The order is by risk to saved data.

1. **A4 + A5.** Two one-line fixes, no data touched.
2. **A2**, then **A1**, together as schema v7. A1's undo rule depends on
   A2's `editedAt`. Back up `index.html` first.
3. **A3**, whichever option you pick. It is about 5 lines either way.
4. `README.md`: fix "a block survives if either device has it", which stops
   being the whole rule. Add the tombstone rule to "The things most study
   trackers get wrong".
5. Bump `CACHE` in `sw.js`, then rebuild `src-tauri/dist` (`copy-assets.sh`).

## Out of scope

- **Forecasting and the Bob the Builder coursework link.** Both are still
  deferred in earlier plans, and neither is a defect.
- **Real-time sync between open tabs.** Pull-on-open and pull-on-reconnect
  stay. A1–A3 are about correctness, not latency.
- **Pruning tombstones.** See the `ponytail:` note in A1.
- **Re-auditing the timer and the Tauri shell.** See "Not covered" above.

## Verification, when approved

- Backup `index.html` with a timestamp before the first edit.
- Re-run the three reproductions (`audit_sync.py`, the edit variant and the
  wipe variant) against the backup and the new file. Before and after, same
  script. Expected after: the delete stays deleted (cloud 2, both devices 2).
  The edit survives (`[25, 25, 50]` everywhere). The wipe matches the chosen
  option.
- Delete then Undo across two devices: the session survives.
- Load a v6 blob with sync on: every session is still present, `deleted: {}`
  is backfilled, and the merge result is unchanged from today.
- Baseline sweep again: 0 console errors across 90 view switches, both
  themes. Heatmap at 390px shows today without scrolling.
- `nearestBadge()` on the 42-day seed returns "Eight in a day".
