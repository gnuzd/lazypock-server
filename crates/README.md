# Crates

Dependencies flow one way: `core <- sql <- schema <- db <- api, studio <- app`.

Planned crates (not created yet; add them to the workspace when work starts):

| Crate | Purpose |
|---|---|
| `rls` | Row Level Security policy generation and request context |
| `auth` | Passwords, sessions, tokens, OAuth2, verification flows |
| `realtime` | Change events over Server-Sent Events, per-subscriber filtering, resume |
| `files` | Upload validation, image presets, local and S3-compatible storage |
| `hooks` | Sandboxed server-side scripting |
| `jobs` | Background jobs and cron |
| `mail` | Outgoing mail and templates |
