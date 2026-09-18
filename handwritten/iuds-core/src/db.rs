use chrono::Local;
use rusqlite::{Connection,Resultiparams};
pub const SCHEMA: &str = r#"
    CREATE TABLE IF NOT EXISTS persons (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    tip         TEXT NOT NULL DEFAULT 'bilinmeyen',
    whitelist   INTEGER NOT NULL DEFAULT 0,
    host        TEXT,
    visit_count INTEGER NOT NULL DEFAULT 0,
    first_seen  TEXT NOT NULL,
    last_seen   TEXT NOT NULL,
    note        TEXT
);
CREATE TABLE IF NOT EXISTS visits (
    id        TEXT PRIMARY KEY,
    person_id TEXT NOT NULL REFERENCES persons(id),
    time      TEXT NOT NULL,
    camera    TEXT NOT NULL DEFAULT 'cam0',
    karar     TEXT NOT NULL,
    yuz       REAL NOT NULL,
    presence  REAL NOT NULL,
    liveness  REAL NOT NULL,
    group_id  TEXT,
    approved_by TEXT
);
CREATE INDEX IF NOT EXISTS idx_visits_time ON visits(time);
CREATE INDEX IF NOT EXISTS idx_visits_person ON visits(person_id);
CREATE TABLE IF NOT EXISTS command_log (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    time     TEXT NOT NULL,
    action   TEXT NOT NULL,
    visit_id TEXT NOT NULL,
    ok       INTEGER NOT NULL,
    detail   TEXT
);
"#;
pub fn open(path:&str)->Result<Connection>{
    let conn = Connection::open(path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}
pub fn open_memory()->Result<Connection>{
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}
pub fn now_str()->String{
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}