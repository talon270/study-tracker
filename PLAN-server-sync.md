# Server-side sync — plan

Written 2026-08-22, against `index.html` as of the current commit
(444654e, `schemaVersion: 3`, single key `studyTracker.v1`).
Method: read `index.html` lines 1796–2013 (STATE/STORAGE block) in full to
confirm the save/load shape before designing a sync layer on top of it.

**Implemented 2026-08-22.** Four deviations from what is written below, each
made during the build and each with a reason:

| Plan said | Built instead | Why |
|---|---|---|
| B3 · Supabase JS client from `esm.sh` | Four plain `fetch` calls | ~100 lines against the REST and auth endpoints is less code than a loader, and it keeps the file zero-dependency so the service worker still delivers a genuinely offline app. The CDN import was the one thing in this plan that broke "works offline and forever" |
| A6 · `activeSession` merges by `updatedAt` | `activeSession` never syncs at all | A running timer belongs to the machine running it. Uploading it lets a second device "recover" a block it never sat through and credit itself the minutes |
| A6 · badges resolved by `updatedAt` | badges union, keeping the **earlier** timestamp | A badge earned on either device is earned, and it was earned when it was first earned |
| B2 · every `save()` stamps `updatedAt` | a `saveLocal()` split: the badge backfill and the timer heartbeat write without stamping | Stamping on a heartbeat would fire a network write every 15s of every block carrying nothing new, and would let a device that merely had the app open outrank one where you actually changed a setting |

Verified by driving the real UI against an intercepted Supabase: 46 checks,
0 failures — covering first upload, two-device merge, offline failure, token
expiry, sign-out, auto-push, and the v1→v4 migration. Local-only behaviour is
unchanged before and after (0 console errors, 120/120 sessions survive).
Setup instructions are in `SETUP-sync.md`.

Answered up front, from your choices: this is for **multi-device sync**
(same log seen from phone and laptop), backed by a **managed BaaS** (no
server to run yourself), for **one user only** (you) — so this needs a
login secret, not a full account system.

---

## Part A — design decisions and their tradeoffs

### A1 · House-rule break, named explicitly

`context.md`'s house style says "no backend, no accounts, no CDN, no
telemetry" for every project here. Multi-device sync cannot be built
without a backend and *some* form of login, so this plan breaks that rule
on purpose. Named here rather than silently — this is the one project
where that tradeoff is being made.

### A2 · Backend: Supabase, not a hand-rolled API

Supabase gives a hosted Postgres table, row-level security, and email/
password auth on a free tier, with no server process for you to run or
patch. The alternative (Cloudflare Worker + KV) would mean writing and
hosting the API yourself for no real benefit at one-user scale — more to
maintain for the same result. Supabase is the smallest thing that
satisfies "sync across my own devices."

### A3 · One row, whole-blob sync — not per-session rows

`index.html`'s state is already one JSON object (`S`) with a single
`save()` that writes it whole. Matching that shape server-side — one
Postgres row, `data jsonb` holding the exact same object `JSON.stringify`
already produces — means no new serialization logic and no per-session
table to design. A normalized `sessions` table would be more "correct"
relationally but buys nothing here: nothing server-side ever queries
individual sessions, only the whole blob is ever read or written.

### A4 · Auth: Supabase email/password, one account

A single Supabase user (your email), gated by Postgres RLS
(`user_id = auth.uid()`). No magic-link email flow, no OAuth — one
password, entered once per device, session persisted by Supabase's own
client. This is the minimum that makes the anon key (which is public in
the page source) safe: without RLS+auth, anyone with the URL could read or
overwrite your log.

### A5 · Local-first, not server-first

`localStorage` stays as the write target for every action — starting a
timer, logging a session, changing a setting all still call today's
`save()` first, instantly, offline or not. A sync layer pushes to Supabase
*after* the local write, debounced, and pulls on load/sign-in/reconnect.
This preserves the one guarantee that must never break: **a session you
just finished is never lost**, even mid-flight to a server that might be
unreachable. Server-first would mean a dropped connection loses the
session that was just logged.

### A6 · Conflict resolution: merge, not last-write-wins

Two devices can both log sessions offline before either syncs. Plain
last-write-wins on the whole blob would silently drop every session
logged on the losing device — exactly the "destructive-by-surprise"
failure class this house avoids. Instead, on pull:

- `sessions`: union by `id`, so a session present in either copy survives.
- `settings`, `badges`, `activeSession`: newer `updatedAt` wins whole.

This needs a new top-level `updatedAt` (ms epoch) on the state object,
bumped on every local `save()`. That's a schema change — `SCHEMA_VERSION`
3 → 4, migration backfills `updatedAt: 0` on any older blob, so a blob
that predates sync always loses that field to the first real pull rather
than fighting it.

### A7 · Hosting of the page itself is unchanged

This plan moves *data*, not the page. Wherever `index.html` is opened from
today (`file://` or a static host) is untouched — Supabase is reached by
the page's own JS over HTTPS, same as any other API call. Out of scope
unless you want the page itself put on a public URL too.

---

## Part B — the build

1. **Create the Supabase project.** One table `study_state(user_id uuid
   primary key references auth.users, data jsonb not null, updated_at
   timestamptz not null default now())`. RLS policy: select/insert/update
   where `auth.uid() = user_id`. Create your one user (email/password) in
   the Supabase dashboard.

2. **Bump `SCHEMA_VERSION` to 4** in `index.html`. Add `updatedAt: 0` to
   `blank()` and to `migrate()`'s backfill. `save()` sets
   `S.updatedAt = Date.now()` before writing to `localStorage`.

3. **Add the Supabase JS client.** Loaded from `esm.sh` as an ES module
   `<script type="module">` — this is the CDN dependency named in A1. No
   bundler, still opens from a single file, but this one script tag needs
   network access on first load (browsers cache the module after that).

4. **Settings → new "Cloud sync" section**: email/password fields, Sign
   in / Sign out, and a status line — "Synced 2 min ago" / "Not signed
   in" / "Sync failed, retrying" — same stale-data-banner pattern as
   `Stonks`' scrape-age banner. Never silently hide a failed sync.

5. **Sync module**: `pushState()` (debounced ~2s after any `save()`,
   upserts the row), `pullState()` (on sign-in and on reconnect —
   `window.addEventListener('online', …)`), and the A6 merge function
   run before `pullState()` overwrites local `S`.

6. **Session persistence.** Supabase's client keeps the login in
   `localStorage` itself (its own key, not `studyTracker.v1`) so you're
   not re-entering the password every visit.

---

## Out of scope

- Multiple accounts or sharing the log with anyone else — A4 assumes you
  forever.
- Real-time push between open tabs/devices (sockets) — pull-on-load and
  pull-on-reconnect is enough for a study log; nobody needs sub-second
  cross-device updates here.
- Hosting the static page itself somewhere new (A7).
- Removing `localStorage` — it stays as the offline-first write target
  permanently, not just during a migration window.
- Import/export UI changes — existing backup/restore, if any exists
  today, is untouched by this plan.
