# lazypock

A small, fast, secure backend-as-a-service on top of PostgreSQL, written in Rust.

Define collections in a web admin UI, and lazypock turns them into real Postgres tables and a ready-to-use REST API, all from a single self-contained binary.

> **Status: early development.** The project is being rebuilt from scratch. Phase 1 covers the dynamic API and the Studio (admin UI). Everything marked _planned_ below does not exist yet.

---

## Why lazypock

- **Fast** — async Rust (tokio + axum), a thin database layer, no ORM, no runtime reflection beyond a cached schema.
- **Small** — one binary with the admin UI embedded. No Node.js or JS runtime needed in production.
- **Secure by default** — parameterized SQL only, whitelisted identifiers, Postgres Row Level Security (RLS) for authorization, fail-closed behavior.
- **Postgres-native** — collections are real tables. Use SQL, extensions and tooling you already know.
- **Familiar API** — a PocketBase-style REST envelope and filter syntax.

## Features

### Phase 1 — in progress

- [ ] Create, edit and delete collections and fields from the Studio; real `CREATE/ALTER TABLE` under the hood
- [ ] Dynamic records API: list, view, create, update, delete
- [ ] Filtering, sorting, pagination and field selection
- [ ] Schema cache that reloads automatically when the schema changes (`LISTEN/NOTIFY`)
- [ ] Studio: server-side rendered admin UI for collections and records
- [ ] Single-binary build with embedded assets

### Planned

- [ ] Authentication: password login, rotating sessions, OAuth2, email verification and password reset
- [ ] Authorization through Postgres RLS generated from collection rules
- [ ] Realtime subscriptions over Server-Sent Events (SSE), filtered per subscriber, with `Last-Event-ID` resume
- [ ] File uploads with validation, image presets, and local / S3-compatible storage
- [ ] Server-side hooks (sandboxed JavaScript/TypeScript)
- [ ] Background jobs and cron
- [ ] Mail, audit logs, settings import/export, backup and restore
- [ ] Multi-tenant deployment (one database per tenant)
- [ ] Official TypeScript SDK

## Architecture

lazypock is a Cargo workspace with one-way dependencies:

```
core ← sql ← schema ← db ← api, studio ← app
```

```
lazypock/
├─ migrations/          # system SQL: _collections, _fields, _settings
├─ crates/
│  ├─ core/             # config, errors, ids, telemetry
│  ├─ sql/              # filter lexer/parser, AST, column whitelist, SQL generation
│  ├─ schema/           # collection/field model, DDL, schema cache, change notifications
│  ├─ db/               # connection pool, migrations, records CRUD, request context
│  ├─ api/              # REST routes under /api/
│  ├─ studio/           # server-rendered admin UI under /_/
│  └─ app/              # binary and CLI
└─ tests/               # integration tests (require PostgreSQL)
```

| Crate | Responsibility |
|---|---|
| `core` | Configuration from environment, error types, ids, tracing |
| `sql` | Parses filter expressions into an AST, validates them against the schema, emits parameterized SQL. Pure and I/O-free, so it is easy to test and fuzz |
| `schema` | Describes collections and fields, generates DDL, keeps an in-memory cache and reloads it on change |
| `db` | Pooling, migrations, record operations, and the request context used for RLS |
| `api` | Dynamic REST endpoints and the response envelope |
| `studio` | Admin UI that calls `schema` and `db` directly rather than going through the HTTP API |
| `app` | Wires everything together, CLI, graceful shutdown |

Planned crates (`rls`, `auth`, `realtime`, `files`, `hooks`, `jobs`, `mail`) are listed in `crates/README.md`.

## Tech stack

| Concern | Choice |
|---|---|
| Runtime / HTTP | `tokio`, `axum`, `tower-http` |
| PostgreSQL | `tokio-postgres`, `deadpool-postgres`, `rustls` |
| SQL generation | `sea-query` (bind parameters, identifier quoting) |
| Studio | `askama` templates, HTMX, Alpine.js, embedded with `rust-embed` |
| Realtime (planned) | Server-Sent Events, with Postgres `LISTEN/NOTIFY` as the event bus |
| Hooks (planned) | QuickJS via `rquickjs` |

Heavy UI widgets (rich text editor, rule editor) are planned as small JS islands loaded only on the pages that need them.

## Security principles

- All user input reaches PostgreSQL as bind parameters; identifiers come from the schema whitelist and are always quoted.
- Authorization lives in the database through Row Level Security, not only in application code.
- The server refuses to start in an unsafe state (for example, if the RLS role cannot be applied) instead of continuing without protection.
- Global request timeouts and body-size limits are part of the router from the start.
- `#![forbid(unsafe_code)]` in every crate; `clippy -D warnings`, `cargo deny` and `cargo audit` in CI.
- The filter parser and policy compiler are designed to be fuzzed.

## Getting started

Requirements (planned): Rust (stable) and PostgreSQL 14+.

Target workflow once phase 1 lands:

```sh
# 1. Point lazypock at a database
export DATABASE_URL=postgres://user:pass@localhost:5432/lazypock

# 2. Apply system migrations
lazypock migrate

# 3. Create a superuser for the Studio
lazypock superuser create admin@example.com

# 4. Run
lazypock serve --http 0.0.0.0:8090
```

- Studio: `http://localhost:8090/_/`
- API: `http://localhost:8090/api/`

> These commands describe the intended interface and may change before the first release.

## API sketch

The records API follows PocketBase conventions:

```
GET    /api/collections/{collection}/records?filter=...&sort=-created&page=1&perPage=30
GET    /api/collections/{collection}/records/{id}
POST   /api/collections/{collection}/records
PATCH  /api/collections/{collection}/records/{id}
DELETE /api/collections/{collection}/records/{id}
```

Example filter:

```
title ~ 'hello' && views > 100
```

Filter values are always sent to PostgreSQL as bind parameters.

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace            # integration tests need a running PostgreSQL
cargo build --release
```

Release profile: `lto = "fat"`, `codegen-units = 1`, `strip = true`.

## Roadmap

1. **Phase 1** — Workspace, `sql` crate, schema engine, records API, Studio.
2. **Phase 2** — Authentication and RLS-based authorization.
3. **Phase 3** — Realtime over SSE, files, hooks, jobs.
4. **Phase 4** — Backup/restore, multi-tenant deployment, SDK.

## Contributing

The project is in an early stage and the structure may still change. Issues and discussions are welcome; please open an issue before starting a large change.

## License

To be decided.
