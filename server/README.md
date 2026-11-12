# Fretwise server

A small Rust web server that runs Fretwise on your computer and keeps your songs in a SQLite database file.

- **Axum** web server, **SQLite** (bundled, nothing to install) through `rusqlite`.
- Listens on `127.0.0.1:8080` by default, so only this computer can reach it.
- Serves the app (`../index.html`, read on every request: edit it and reload) and a JSON API.

## Run

You need Rust (https://rustup.rs).

```sh
cd server
cargo run --release
```

Open http://127.0.0.1:8080. In the Song Builder the status under the title box turns green: *Saved in the database on your computer*. Songs are stored in `server/fretwise.db`.

| Variable | Default | Meaning |
|---|---|---|
| `FRETWISE_ADDR` | `127.0.0.1:8080` | Address and port. Use `0.0.0.0:8080` to reach it from your phone on the same Wi-Fi (no login: anyone on the network can then read and change your songs). |
| `FRETWISE_DB` | `fretwise.db` | Database file. |
| `FRETWISE_INDEX` | `../index.html` | The app page to serve. |
| `RUST_LOG` | `fretwise_server=info` | Log level. |

Back up your songs by copying the `.db` file (stop the server first), or with **My songs → Export all (JSON)** in the app.

## API

All bodies are JSON. Errors are `{"error": "message"}` with status 400 or 404.

| Method | Path | Body | Returns |
|---|---|---|---|
| GET | `/api/health` | | `{ok, version, storage}` |
| GET | `/api/songs` | | songs, newest first |
| POST | `/api/songs` | song input | the new song, 201 |
| GET | `/api/songs/{id}` | | one song |
| PUT | `/api/songs/{id}` | song input | the updated song |
| DELETE | `/api/songs/{id}` | | 204 |
| GET | `/api/settings` | | `{settings, updated_at}` or 404 |
| PUT | `/api/settings` | any JSON object (≤ 256 KB) | `{ok, updated_at}` |

Song input:

```json
{
  "title": "Sunday riff",
  "chords": "Am ~Am F C@x32010 G",
  "bpm": 96,
  "time_sig": "3/4",
  "style": "travis",
  "legato": true,
  "key": {"t": 9, "mode": "minor"},
  "notes": ""
}
```

Only `title` and `chords` are required. `bpm` 30–300 (default 90), `time_sig` one of 4/4 3/4 6/8 12/8 2/4, `style` is the Song Builder playing style, `key` is the key chosen by hand (null = detected). Stored songs also have `id`, `created_at`, `updated_at` (UTC, ISO 8601).

## Code

| File | |
|---|---|
| `src/main.rs` | Reads the environment, opens the database, starts the server. |
| `src/lib.rs` | Routes and handlers, JSON errors. |
| `src/db.rs` | Schema, validation, queries. |
| `tests/api.rs` | End-to-end tests of every route on an in-memory database. |

```sh
cargo test      # unit + API tests
cargo clippy    # lints
```

The schema is created on start (`CREATE TABLE IF NOT EXISTS`), so a new database needs no setup.

## Without the server

The app still works when opened as a plain file or from GitHub Pages: songs are then saved in the browser's local storage. **Export / Import JSON** in *My songs* moves them between the browser and the database.
