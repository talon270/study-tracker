# Cloud sync — setup

Written 2026-08-22. Ten minutes, once. After this the app syncs on its own and
you never open the Supabase dashboard again.

Sync is **off until you finish this**. Until then the app behaves exactly as it
always has: one browser, one `localStorage` key, no network call of any kind.

---

## 1 · Create the project

[supabase.com](https://supabase.com) → new project. Free tier is enough — this
stores one row. Pick the region nearest you (Mumbai / `ap-south-1`) so a sync is
a few tens of milliseconds rather than a few hundred.

Save the database password it asks you to set. You will not need it for the app,
but you will need it if you ever want to open the database directly.

## 2 · Create the table

Dashboard → **SQL Editor** → paste this whole block → Run.

```sql
create table public.study_state (
  user_id    uuid primary key references auth.users(id) on delete cascade,
  data       jsonb not null,
  updated_at timestamptz not null default now()
);

alter table public.study_state enable row level security;

-- Three policies, not one. The app upserts, and an upsert that hits an existing
-- row is an UPDATE, so a project with only an INSERT policy accepts the first
-- sync and silently rejects every one after it.
create policy "read own row"   on public.study_state
  for select using (auth.uid() = user_id);
create policy "insert own row" on public.study_state
  for insert with check (auth.uid() = user_id);
create policy "update own row" on public.study_state
  for update using (auth.uid() = user_id) with check (auth.uid() = user_id);
```

Row-level security is the whole security model here. The anon key sits in the
page in plain text and is meant to — it identifies the project, it does not
grant access. These three policies are what stop anyone holding that key from
reading your log.

## 3 · Create your one user

Dashboard → **Authentication → Users → Add user**. Enter your email and a
password, and **tick "Auto Confirm User"** — without it Supabase waits for an
email confirmation that will never arrive and every sign-in fails with
`Email not confirmed`.

Then **Authentication → Sign In / Providers → Email**, and turn **"Allow new
users to sign up" off**. One account exists, you just made it; leaving signups
open lets anyone with the anon key create their own account in your project.
Their RLS policies would keep them out of your row, but there is no reason to
host strangers' data on your quota.

## 4 · Connect the app

Settings → **Cloud sync**. Four fields:

| Field | Where it comes from |
|---|---|
| Project URL | Project Settings → Data API → Project URL (`https://xxxx.supabase.co`) |
| Anon key | Project Settings → API Keys → `anon` / `publishable`. **Not** `service_role` |
| Email | the user you made in step 3 |
| Password | the same |

Press **Sign in and sync**.

---

## Getting your existing data up there

**Signing in does it. There is no separate upload button, on purpose.**

The first sync from the device that already has your history reads the cloud
copy, finds it empty, and uploads everything — you will see
`Uploaded 412 session(s) — the cloud copy was empty.` with your real count.
Then sign in on the second device and it pulls the lot down.

**Order does not matter, because sync merges rather than overwrites.** If you
sign in on the empty phone first and the laptop second, nothing is lost: the
laptop's first sync unions its 412 sessions with the phone's 0 and pushes 412
back up. The same holds after either device has been offline for a week — both
sides' sessions survive, matched on session id, and the message tells you how
many came down.

The one thing that is *not* merged is settings: on a conflict the more recently
**edited** copy wins whole, since half-merged settings mean nothing. Syncing is
not editing, so a device that only syncs can never overwrite a change you made
somewhere else.

`activeSession` — a timer running right now — never syncs at all. A running
block belongs to the machine you are sitting at; uploading it would let the
other device "recover" minutes you did not study there.

---

## What it looks like when something is wrong

The chip in the Cloud sync card is the whole status, and it never lies by
omission:

| Chip | Meaning |
|---|---|
| **Off** | Not configured. Local-only, no request made. |
| **Connected** | Signed in, first sync not finished yet. |
| **Syncing…** | In flight. |
| **Synced** | Succeeded, with the session count in the line below. |
| **Offline** | The browser is offline. Everything is still saved locally. |
| **Signed out** | The refresh token expired. Retype the password; the URL, key and email are still in the form. |
| **Sync failed** | The request was rejected. The reason from Supabase is printed verbatim below the chip. |

A failed sync never costs you anything: local storage was written before the
request was attempted, and the next edit, the next reconnect, or **Sync now**
retries.

Common `Sync failed` reasons, and what they actually mean:

| Message | Cause |
|---|---|
| `Invalid login credentials` | Wrong password, or the user was never created in step 3. |
| `Email not confirmed` | Step 3 without "Auto Confirm User". Fix it in the dashboard. |
| `new row violates row-level security policy` | Step 2's policies are missing or partial. |
| `relation "public.study_state" does not exist` | Step 2 was not run, or was run against a different project. |

---

## Still keep a backup

Settings → **Download backup (JSON)** is still the only copy that survives
deleting the Supabase project, losing the password, or the free tier pausing an
idle project. Two live copies that sync to each other are not two independent
backups — a bad merge propagates to both. Download one occasionally.
