use chrono::Local;
use rusqlite::{Connection,Result,params};
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
pub fn today_prefix()->String{
    Local::now().format("%Y-%m-%d").to_string()
}
pub fn next_unknown_id(conn:&Connection) -> Result<String>{
    let day = today_prefix();
    let n : i64 = conn.query_row(
        "SELECT COUNT(*) FROM persons WHERE id LIKE ?1",
        params![format!("Ziyaretci-{day}-%")],
        |r| r.get(0)
    )?;
        Ok(format!("Ziyaretci-{day}-{:03}",n+1))
}
    
pub fn ensure_person(conn: &Connection, id: &str, name: &str) -> Result<()>{
    conn.execute(
        "INSERT OR IGNORE INTO persons 
        (
            id, name, tip, whitelist, host, visit_count,
            first_seen, last_seen
        )
        VALUES(?1, ?2,'bilinmeyen', 0, 0, 0, ?3, ?3)
        ",
        params![id,name,now_str()]
    )?;
    Ok(())}
    pub fn is_whitelisted(conn: &Connection, id: &str) -> Result<bool>{
        let w:i64 = conn.query_row(
            "SELECT whitelist FROM persons WHERE id= ?1",
        params![id],
        |r| r.get(0),       
        )?;
    Ok(w ==1)
    }
    pub fn bump_visit_counter(conn: &Connection, person_id: &str) -> Result<()>{
        let now = now_str();
        conn.execute(
            "UPDATE persons 
            SET visit_count = visit_count + 1, last_seen = ?1 
            WHERE id = ?2",
            params![now, person_id],
        )?;
    Ok(())
    }
#[allow(clippy::too_many_arguments)]
pub fn insert_visit(
    conn:&Connection,
    visit_id: &str,
    person_id: &str,
    camera:    &str,
    karar: &str,
    yuz: f64,
    presence:f64,
    liveness:f64,
    group_id: Option<&str>
    )
    -> Result<()> {
        conn.execute(
        "INSERT INTO visits (id,person_id,time,camera,karar,yuz,presence,liveness,group_id) 
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        params![
            visit_id,person_id,now_str(),camera,karar,yuz,presence,liveness,group_id,    
        ])?;
        bump_visit_counter(conn,person_id)?;
        Ok(())
}
pub fn log_command(conn: &Connection,action: &str, visit_id: &str,ok: bool,detail:&str)->Result<()>{
    conn.execute(
        "INSERT INTO command_log (time,action,visit_id,ok,detail) VALUES(?1,?2,?3,?4,?5)",
        params![now_str(),action,visit_id,ok as i32 , detail ],
    )?;
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn unknown_id_artar() -> Result<()>{
     let conn = open_memory()?;
     let a = next_unknown_id(&conn)?;
     ensure_person(&conn, &a, &a )?;
     let b = next_unknown_id(&conn)?;
     assert_ne!(a,b);
     assert!(b.starts_with("Ziyaretci-"));
     Ok(())
    }
    #[test]
    fn visit_ekle_sayaci_artirir() -> Result<()>{
     let conn = open_memory()?;
     ensure_person(&conn,"p1","Anne")?;
     insert_visit(&conn, "v1" , "p1" ,"cam0","oto_ac",0.9, 0.9, 0.9, None)?;
     let n:i64=conn.query_row(
     "SELECT visit_count FROM persons WHERE id= 'p1'", [],
     |r| r.get(0),
    )?;   
    assert_eq!(n,1);
    Ok(())
    }
    #[test]
    fn command_log_append_only() -> Result<()>{
        let conn = open_memory()?;
        log_command(&conn, "open","v-1",true,"test")?;
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM command_log", [], |r| r.get(0),)?;   
        assert_eq!(n,1);
        Ok(())
    }
    #[test]
    fn whitelist_dbden_okunur() -> Result<()>{
        let conn = open_memory()?;
        ensure_person(&conn,"p1","Anne")?;
        assert!(!is_whitelisted(&conn, "p1")?);
        conn.execute("UPDATE persons SET whitelist=1 WHERE id='p1'", [])?;
        assert!(is_whitelisted(&conn, "p1")?);
        Ok(())
    }   
}
