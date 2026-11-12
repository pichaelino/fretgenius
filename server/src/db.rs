//! SQLite storage for songs and settings.

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MAX_TITLE: usize = 120;
pub const MAX_CHORDS: usize = 4000;
pub const MAX_NOTES: usize = 4000;
pub const MAX_SETTINGS: usize = 256 * 1024;

/// A saved song. `chords` is the Song Builder text, e.g. `"Am ~Am F C@x32010 G"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Song {
    pub id: i64,
    pub title: String,
    pub chords: String,
    pub bpm: i64,
    pub time_sig: String,
    pub style: String,
    pub legato: bool,
    /// Key chosen by hand in the app, as JSON (`{"t":9,"mode":"minor"}`), or null when detected.
    pub key: Option<serde_json::Value>,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

/// What the client sends to create or update a song.
#[derive(Debug, Clone, Deserialize)]
pub struct SongInput {
    pub title: String,
    pub chords: String,
    #[serde(default = "default_bpm")]
    pub bpm: i64,
    #[serde(default = "default_time")]
    pub time_sig: String,
    #[serde(default = "default_style")]
    pub style: String,
    #[serde(default)]
    pub legato: bool,
    #[serde(default)]
    pub key: Option<serde_json::Value>,
    #[serde(default)]
    pub notes: String,
}

fn default_bpm() -> i64 {
    90
}
fn default_time() -> String {
    "4/4".into()
}
fn default_style() -> String {
    "strum".into()
}

const TIME_SIGS: [&str; 5] = ["4/4", "3/4", "6/8", "12/8", "2/4"];

impl SongInput {
    /// Trims text and checks limits. Returns a message for the client on failure.
    pub fn validate(mut self) -> Result<Self, String> {
        self.title = self.title.trim().to_string();
        self.chords = self.chords.split_whitespace().collect::<Vec<_>>().join(" ");
        if self.title.is_empty() {
            return Err("title is empty".into());
        }
        if self.title.chars().count() > MAX_TITLE {
            return Err(format!("title is longer than {MAX_TITLE} characters"));
        }
        if self.chords.len() > MAX_CHORDS {
            return Err(format!("chords are longer than {MAX_CHORDS} characters"));
        }
        if self.notes.len() > MAX_NOTES {
            return Err(format!("notes are longer than {MAX_NOTES} characters"));
        }
        if !(30..=300).contains(&self.bpm) {
            return Err("bpm must be between 30 and 300".into());
        }
        if !TIME_SIGS.contains(&self.time_sig.as_str()) {
            return Err(format!("time_sig must be one of {}", TIME_SIGS.join(", ")));
        }
        if self.style.is_empty() || self.style.len() > 20 {
            return Err("style is not valid".into());
        }
        Ok(self)
    }
}

pub struct Db {
    conn: Connection,
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS songs (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT    NOT NULL,
    chords      TEXT    NOT NULL,
    bpm         INTEGER NOT NULL DEFAULT 90,
    time_sig    TEXT    NOT NULL DEFAULT '4/4',
    style       TEXT    NOT NULL DEFAULT 'strum',
    legato      INTEGER NOT NULL DEFAULT 0,
    key_json    TEXT,
    notes       TEXT    NOT NULL DEFAULT '',
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
    updated_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX IF NOT EXISTS songs_updated ON songs(updated_at DESC);
CREATE TABLE IF NOT EXISTS settings (
    id          INTEGER PRIMARY KEY CHECK (id = 1),
    json        TEXT    NOT NULL,
    updated_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
";

const SONG_COLS: &str =
    "id, title, chords, bpm, time_sig, style, legato, key_json, notes, created_at, updated_at";

fn song_from_row(r: &Row) -> rusqlite::Result<Song> {
    let key: Option<String> = r.get(7)?;
    Ok(Song {
        id: r.get(0)?,
        title: r.get(1)?,
        chords: r.get(2)?,
        bpm: r.get(3)?,
        time_sig: r.get(4)?,
        style: r.get(5)?,
        legato: r.get::<_, i64>(6)? != 0,
        key: key.and_then(|k| serde_json::from_str(&k).ok()),
        notes: r.get(8)?,
        created_at: r.get(9)?,
        updated_at: r.get(10)?,
    })
}

fn key_json(k: &Option<serde_json::Value>) -> Option<String> {
    k.as_ref().filter(|v| !v.is_null()).map(|v| v.to_string())
}

impl Db {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        Self::init(Connection::open(path)?)
    }

    pub fn open_in_memory() -> rusqlite::Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> rusqlite::Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Db { conn })
    }

    pub fn list_songs(&self) -> rusqlite::Result<Vec<Song>> {
        let mut st = self.conn.prepare(&format!(
            "SELECT {SONG_COLS} FROM songs ORDER BY updated_at DESC, id DESC"
        ))?;
        let rows = st.query_map([], song_from_row)?;
        rows.collect()
    }

    pub fn get_song(&self, id: i64) -> rusqlite::Result<Option<Song>> {
        self.conn
            .query_row(
                &format!("SELECT {SONG_COLS} FROM songs WHERE id = ?1"),
                [id],
                song_from_row,
            )
            .optional()
    }

    pub fn create_song(&self, s: &SongInput) -> rusqlite::Result<Song> {
        self.conn.execute(
            "INSERT INTO songs (title, chords, bpm, time_sig, style, legato, key_json, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![s.title, s.chords, s.bpm, s.time_sig, s.style, s.legato as i64, key_json(&s.key), s.notes],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(self.get_song(id)?.expect("row just inserted"))
    }

    pub fn update_song(&self, id: i64, s: &SongInput) -> rusqlite::Result<Option<Song>> {
        let n = self.conn.execute(
            "UPDATE songs SET title=?1, chords=?2, bpm=?3, time_sig=?4, style=?5, legato=?6, key_json=?7, notes=?8,
             updated_at=strftime('%Y-%m-%dT%H:%M:%SZ','now') WHERE id=?9",
            params![s.title, s.chords, s.bpm, s.time_sig, s.style, s.legato as i64, key_json(&s.key), s.notes, id],
        )?;
        if n == 0 {
            return Ok(None);
        }
        self.get_song(id)
    }

    pub fn delete_song(&self, id: i64) -> rusqlite::Result<bool> {
        Ok(self.conn.execute("DELETE FROM songs WHERE id = ?1", [id])? > 0)
    }

    pub fn get_settings(&self) -> rusqlite::Result<Option<(serde_json::Value, String)>> {
        self.conn
            .query_row("SELECT json, updated_at FROM settings WHERE id = 1", [], |r| {
                let j: String = r.get(0)?;
                Ok((serde_json::from_str(&j).unwrap_or(serde_json::Value::Null), r.get(1)?))
            })
            .optional()
    }

    pub fn put_settings(&self, v: &serde_json::Value) -> rusqlite::Result<String> {
        self.conn.execute(
            "INSERT INTO settings (id, json) VALUES (1, ?1)
             ON CONFLICT(id) DO UPDATE SET json=excluded.json, updated_at=strftime('%Y-%m-%dT%H:%M:%SZ','now')",
            [v.to_string()],
        )?;
        Ok(self.get_settings()?.map(|(_, t)| t).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(title: &str) -> SongInput {
        SongInput {
            title: title.into(),
            chords: "  Am   F C G ".into(),
            bpm: 100,
            time_sig: "3/4".into(),
            style: "pima".into(),
            legato: true,
            key: Some(serde_json::json!({"t": 9, "mode": "minor"})),
            notes: String::new(),
        }
    }

    #[test]
    fn crud_round_trip() {
        let db = Db::open_in_memory().unwrap();
        let s = db.create_song(&input("First").validate().unwrap()).unwrap();
        assert_eq!(s.chords, "Am F C G");
        assert_eq!(s.time_sig, "3/4");
        assert!(s.legato);
        assert_eq!(s.key, Some(serde_json::json!({"t": 9, "mode": "minor"})));

        let mut upd = input("Renamed");
        upd.key = None;
        let u = db.update_song(s.id, &upd.validate().unwrap()).unwrap().unwrap();
        assert_eq!(u.title, "Renamed");
        assert_eq!(u.key, None);

        assert_eq!(db.list_songs().unwrap().len(), 1);
        assert!(db.delete_song(s.id).unwrap());
        assert!(!db.delete_song(s.id).unwrap());
        assert!(db.get_song(s.id).unwrap().is_none());
    }

    #[test]
    fn validation() {
        assert!(input("  ").validate().is_err());
        assert!(input(&"x".repeat(MAX_TITLE + 1)).validate().is_err());
        let mut i = input("ok");
        i.bpm = 10;
        assert!(i.validate().is_err());
        let mut i = input("ok");
        i.time_sig = "5/4".into();
        assert!(i.validate().is_err());
    }

    #[test]
    fn settings_upsert() {
        let db = Db::open_in_memory().unwrap();
        assert!(db.get_settings().unwrap().is_none());
        db.put_settings(&serde_json::json!({"theme": "dark"})).unwrap();
        db.put_settings(&serde_json::json!({"theme": "light"})).unwrap();
        assert_eq!(db.get_settings().unwrap().unwrap().0["theme"], "light");
    }
}
