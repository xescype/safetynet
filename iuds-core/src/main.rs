//! IUDS Core — Ders 4: SQLite + karar motoru + vision ingest.
//! Koştur: `cargo run` → http://127.0.0.1:3901/api/health
//! Test: `cargo test`
//!
//! Sözleşme: notes/03-Api-Sozlesmesi.md (değişirse panel + not aynı anda güncellenir).
//! Kural: decision.rs (docs/02). Tek yazar: bu servis (db.rs).

mod db;
mod decision;
mod presence;
mod tts;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::Local;
use decision::{Decision, VisionScores, decide, validate};
use serde::{Deserialize, Serialize};

// ---- Modeller ----

#[derive(Clone)]
struct AppState {
    db: Arc<tokio::sync::Mutex<rusqlite::Connection>>,
    tts: tts::TtsConfig,
    presence: presence::SharedPresence,
    /// Aktif klon ses (env varsayılanı; panelden değiştirilebilir).
    voice: Arc<tokio::sync::RwLock<String>>,
}

/// Env varsayılanı + panel seçimi birleşmiş TTS ayarı.
async fn active_tts(st: &AppState) -> tts::TtsConfig {
    let mut c = st.tts.clone();
    c.voice = st.voice.read().await.clone();
    c
}

#[derive(Serialize)]
struct Health {
    ok: bool,
    service: &'static str,
}

#[derive(Serialize)]
struct Visit {
    id: String,
    person: String,
    trust: String, // L0/L1/L2
    score: f32,
    time: String,
    karar: String, // oto_ac|eslikli_ac|soruldu|red (panel Seslendir için)
}

/// Python vision'dan gelen olay. Skorlar YOLO + InsightFace + LD2410'dan
/// gelecek; şimdilik panel/test manuel basar.
#[derive(Deserialize)]
struct VisionEventReq {
    person_id: Option<String>,
    camera: Option<String>,
    yuz: f32,
    /// None ise presence önbelleğinden (RuView MQTT / LD2410) alınır.
    /// Önbellek bayatsa 0 sayılır.
    presence: Option<f32>,
    liveness: f32,
    track_suresi: f32,
    /// YOK SAYILIR (B3): kapı kararı için whitelist DB'den okunur,
    /// client bayrağına güvenilmez. Sözleşme uyumu için alan kabul edilir.
    whitelist: Option<bool>,
    birlikte_mi: Option<bool>,
    group_id: Option<String>,
}

#[derive(Serialize)]
struct VisionEventResp {
    ok: bool,
    visit_id: String,
    person_id: String,
    karar: &'static str,
    neden: &'static str,
}

#[derive(Deserialize)]
struct GreetReq {
    /// Verilirse ses dosyası `greetings/<visit_id>.wav` yazılır.
    visit_id: Option<String>,
    /// oto_ac | eslikli_ac | soruldu | red
    karar: String,
    name: Option<String>,
}

#[derive(Serialize)]
struct GreetResp {
    ok: bool,
    text: String,
    spoken: bool,
    bytes: usize,
    file: Option<String>,
    detail: String,
}

#[derive(Deserialize)]
struct DoorCmd {
    visit_id: String,
    token: String, // Faz 1'de mock: boş olmamalı. Faz 4'te HMAC olacak.
}

#[derive(Serialize)]
struct DoorResp {
    ok: bool,
    action: &'static str,
    visit_id: String,
}

#[derive(Debug, Serialize)]
struct PersonInfo {
    id: String,
    name: String,
    tip: String,
    whitelist: bool,
    trust: String, // L0/L1/L2
    host: Option<String>,
    visit_count: i64,
    first_seen: String,
    last_seen: String,
    note: Option<String>,
}

#[derive(Debug, Serialize)]
struct PersonVisit {
    id: String,
    time: String,
    camera: String,
    karar: String,
    yuz: f32,
    group_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct Companion {
    group_id: String,
    time: String,
    person_id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct PersonCard {
    ok: bool,
    person: PersonInfo,
    visits: Vec<PersonVisit>,
    companions: Vec<Companion>,
}

#[derive(Deserialize)]
struct TagCmd {
    name: String,
    host: Option<String>,
    tip: Option<String>,
    /// whitelist SADECE insan onayıyla değişir; None = dokunma.
    whitelist: Option<bool>,
}

// ---- Handlers ----

async fn health() -> Json<Health> {
    Json(Health { ok: true, service: "iuds-core" })
}

fn trust_of(whitelist: i64, visit_count: i64) -> &'static str {
    if whitelist == 1 {
        "L2"
    } else if visit_count >= 3 {
        "L1"
    } else {
        "L0"
    }
}

async fn visits_today(State(st): State<AppState>) -> Result<Json<Vec<Visit>>, StatusCode> {
    let guard = st.db.lock().await;
    let prefix = db::today_prefix();
    let mut stmt = guard
        .prepare(
            "SELECT v.id, v.person_id, v.time, v.yuz, COALESCE(p.whitelist,0), COALESCE(p.visit_count,0), v.karar
             FROM visits v LEFT JOIN persons p ON p.id = v.person_id
             WHERE v.time LIKE ?1 ORDER BY v.time DESC LIMIT 100",
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = stmt
        .query_map([format!("{prefix}%")], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, f32>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, i64>(5)?,
                r.get::<_, String>(6)?,
            ))
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut out = Vec::new();
    for r in rows {
        let (id, person, time, yuz, wl, cnt, karar) =
            r.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // Panel sadece saati gösterir (18:42); DB'de tam tarih var.
        let short = time.get(11..16).unwrap_or(&time).to_string();
        out.push(Visit { id, person, trust: trust_of(wl, cnt).into(), score: yuz, time: short, karar });
    }
    Ok(Json(out))
}

async fn vision_event(
    State(st): State<AppState>,
    Json(req): Json<VisionEventReq>,
) -> Result<Json<VisionEventResp>, StatusCode> {
    let presence = match req.presence {
        Some(p) => p,
        None => presence::current_score(&st.presence).await,
    };
    let _client_whitelist = req.whitelist; // B3: karar için kullanılmaz (DB okunur)
    // Aralık kontrolü önce (validate whitelist'e bakmaz).
    let probe = VisionScores {
        yuz: req.yuz,
        presence,
        liveness: req.liveness,
        track_suresi: req.track_suresi,
        whitelist: false,
        birlikte_mi: false,
    };
    validate(&probe).map_err(|_| StatusCode::BAD_REQUEST)?;

    let guard = st.db.lock().await;
    // Kişi: verildiyse kullan, yoksa bilinmeyen id üret.
    let person_id = match req.person_id {
        Some(p) if !p.trim().is_empty() => p.trim().to_string(),
        _ => db::next_unknown_id(&guard).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };
    db::ensure_person(&guard, &person_id, &person_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // B3: whitelist SADECE DB'den okunur — "SADECE insan onayıyla" ilkesi
    // API seviyesinde de korunur.
    let whitelist =
        db::is_whitelisted(&guard, &person_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let scores = VisionScores {
        whitelist,
        birlikte_mi: req.birlikte_mi.unwrap_or(false),
        ..probe
    };
    let (karar, neden) = decide(&scores);
    let karar_str = karar.as_str();

    // Visit id: gün içi sayaç (v-YYYY-AA-GG-NNN).
    let day = Local::now().format("%Y-%m-%d").to_string();
    let n: i64 = guard
        .query_row("SELECT COUNT(*) FROM visits WHERE time LIKE ?1", [format!("{day}%")], |r| {
            r.get(0)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let visit_id = format!("v-{day}-{:03}", n + 1);
    let camera = req.camera.as_deref().unwrap_or("cam0");

    db::insert_visit(
        &guard,
        &visit_id,
        &person_id,
        camera,
        karar_str,
        scores.yuz,
        scores.presence,
        scores.liveness,
        req.group_id.as_deref(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Kapı kararı da bir komut sayılır: oto/eslikli açılışları logla.
    if matches!(karar, Decision::OtoAc | Decision::EslikliAc) {
        let _ = db::log_command(&guard, karar_str, &visit_id, true, neden);
    }

    // Karşılama sesi: kararı bloklamaz, arka planda best-effort.
    // TTS kapalıysa sessiz geçilir (spoken:false sadece logda).
    let tts_cfg = active_tts(&st).await;
    let db_clone = st.db.clone();
    let greet_vid = visit_id.clone();
    let greet_text = tts::greet_text(karar_str, Some(&person_id));
    let greet_karar = karar_str.to_string();
    tokio::spawn(async move {
        let (out, file) = tts::speak_to_file(&tts_cfg, &greet_text, &greet_vid).await;
        let g = db_clone.lock().await;
        let _ = db::log_command(
            &g,
            "greet",
            &greet_vid,
            out.spoken,
            &format!("{} bytes={} file={:?} {}", greet_karar, out.bytes, file, out.detail),
        );
    });
    Ok(Json(VisionEventResp { ok: true, visit_id, person_id, karar: karar_str, neden }))
}

async fn greet(
    State(st): State<AppState>,
    Json(req): Json<GreetReq>,
) -> Result<Json<GreetResp>, StatusCode> {
    if !["oto_ac", "eslikli_ac", "soruldu", "red", "soruldu_acildi"].contains(&req.karar.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let text = tts::greet_text(&req.karar, req.name.as_deref());
    let tts_cfg = active_tts(&st).await;
    let (out, file) = match req.visit_id.clone() {
        Some(vid) if !vid.trim().is_empty() => {
            let vid = vid.trim();
            // B5: dosya yoluna giden id sanitize edilir (path traversal yok).
            if !tts::safe_file_id(vid) {
                return Err(StatusCode::BAD_REQUEST);
            }
            tts::speak_to_file(&tts_cfg, &text, vid).await
        }
        _ => {
            let o = tts::speak(&tts_cfg, &text).await;
            (o, None)
        }
    };
    let guard = st.db.lock().await;
    let _ = db::log_command(
        &guard,
        "greet",
        req.visit_id.as_deref().unwrap_or("-"),
        out.spoken,
        &format!("{} bytes={} {}", req.karar, out.bytes, out.detail),
    );
    Ok(Json(GreetResp {
        ok: true,
        text,
        spoken: out.spoken,
        bytes: out.bytes,
        file,
        detail: out.detail,
    }))
}

#[derive(Serialize)]
struct PresenceResp {
    score: f32,
    fresh: bool,
    source: String,
    updated_at: String,
}

async fn get_presence(State(st): State<AppState>) -> Json<PresenceResp> {
    let s = st.presence.read().await;
    Json(PresenceResp {
        score: s.score,
        fresh: presence::fresh(&s),
        source: s.source.clone(),
        updated_at: s.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[derive(Deserialize)]
struct PresenceSet {
    score: f32,
    source: Option<String>,
}

/// LD2410 / test enjeksiyonu: presence skorunu direkt yaz.
/// Aralık dışı skor 400 (fail-closed kalsın diye çöp kabul edilmez).
async fn set_presence(
    State(st): State<AppState>,
    Json(req): Json<PresenceSet>,
) -> Result<Json<PresenceResp>, StatusCode> {
    if !(0.0..=1.0).contains(&req.score) || !req.score.is_finite() {
        return Err(StatusCode::BAD_REQUEST);
    }
    {
        let mut s = st.presence.write().await;
        s.score = req.score;
        s.source = req.source.unwrap_or_else(|| "http".into());
        s.updated_at = chrono::Local::now();
    }
    Ok(get_presence(State(st)).await)
}

#[derive(Serialize)]
struct VoicesResp {
    ok: bool,
    active: String,
    voices: Vec<tts::VoiceInfo>,
    detail: String,
}

/// VoiceStudio'daki klon sesler. Backend kapalıysa liste boş döner (200).
async fn list_voices(State(st): State<AppState>) -> Json<VoicesResp> {
    let cfg = active_tts(&st).await;
    match tts::list_voices(&cfg).await {
        Ok(v) => Json(VoicesResp { ok: true, active: cfg.voice, voices: v, detail: "ok".into() }),
        Err(e) => Json(VoicesResp {
            ok: true,
            active: cfg.voice,
            voices: vec![],
            detail: e,
        }),
    }
}

#[derive(Deserialize)]
struct VoiceSelect {
    voice: String,
}

/// Aktif karşılama sesini değiştirir (sadece runtime; kalıcı ayar env).
async fn select_voice(
    State(st): State<AppState>,
    Json(req): Json<VoiceSelect>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if req.voice.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    *st.voice.write().await = req.voice.trim().to_string();
    let guard = st.db.lock().await;
    let _ = db::log_command(&guard, "voice", "-", true, req.voice.trim());
    Ok(Json(serde_json::json!({ "ok": true, "voice": req.voice })))
}

async fn door_open(    State(st): State<AppState>,
    Json(cmd): Json<DoorCmd>,
) -> Result<Json<DoorResp>, StatusCode> {
    if cmd.token.is_empty() || cmd.visit_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST); // imzasız komut reddedilir
    }
    let guard = st.db.lock().await;
    // B4: kapı komutu ziyaretin sonucudur — karar + onaylayan ziyarete işlenir
    // (notes/02: "Aç → visits.karar=soruldu_acildi, onaylayan").
    let n = guard
        .execute(
            "UPDATE visits SET karar = 'soruldu_acildi', approved_by = 'panel' WHERE id = ?1",
            rusqlite::params![cmd.visit_id],
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if n == 0 {
        return Err(StatusCode::NOT_FOUND); // böyle bir ziyaret yok
    }
    let _ = db::log_command(&guard, "open", &cmd.visit_id, true, "manuel acma");
    Ok(Json(DoorResp { ok: true, action: "open", visit_id: cmd.visit_id }))
}

async fn door_deny(
    State(st): State<AppState>,
    Json(cmd): Json<DoorCmd>,
) -> Result<Json<DoorResp>, StatusCode> {
    if cmd.token.is_empty() || cmd.visit_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let guard = st.db.lock().await;
    let n = guard
        .execute(
            "UPDATE visits SET karar = 'red', approved_by = 'panel' WHERE id = ?1",
            rusqlite::params![cmd.visit_id],
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if n == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    let _ = db::log_command(&guard, "deny", &cmd.visit_id, true, "manuel red");
    Ok(Json(DoorResp { ok: true, action: "deny", visit_id: cmd.visit_id }))
}

async fn get_person(
    State(st): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PersonCard>, StatusCode> {
    let guard = st.db.lock().await;
    let (pid, name, tip, wl, host, cnt, first, last, note): (
        String, String, String, i64, Option<String>, i64, String, String, Option<String>,
    ) = guard
        .query_row(
            "SELECT id, name, tip, whitelist, host, visit_count, first_seen, last_seen, note
             FROM persons WHERE id = ?1",
            [&id],
            |r| {
                Ok((
                    r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?,
                    r.get(5)?, r.get(6)?, r.get(7)?, r.get(8)?,
                ))
            },
        )
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut stmt = guard
        .prepare(
            "SELECT id, time, camera, karar, yuz, group_id FROM visits
             WHERE person_id = ?1 ORDER BY time DESC LIMIT 30",
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let visits = stmt
        .query_map([&pid], |r| {
            Ok(PersonVisit {
                id: r.get(0)?,
                time: r.get(1)?,
                camera: r.get(2)?,
                karar: r.get(3)?,
                yuz: r.get(4)?,
                group_id: r.get(5)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Birlikte gelenler: aynı group_id'yi paylaşan başka kişiler (docs/06 cross-reference).
    let mut cstmt = guard
        .prepare(
            "SELECT v.group_id, v.time, v.person_id, COALESCE(p.name, v.person_id)
             FROM visits v LEFT JOIN persons p ON p.id = v.person_id
             WHERE v.group_id IN (SELECT group_id FROM visits WHERE person_id = ?1 AND group_id IS NOT NULL)
               AND v.person_id != ?1
             ORDER BY v.time DESC LIMIT 30",
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let companions = cstmt
        .query_map([&pid], |r| {
            Ok(Companion { group_id: r.get(0)?, time: r.get(1)?, person_id: r.get(2)?, name: r.get(3)? })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PersonCard {
        ok: true,
        person: PersonInfo {
            trust: trust_of(wl, cnt).into(),
            whitelist: wl == 1,
            id: pid, name, tip, host, visit_count: cnt,
            first_seen: first, last_seen: last, note,
        },
        visits,
        companions,
    }))
}

async fn tag_person(
    State(st): State<AppState>,
    Path(id): Path<String>,
    Json(cmd): Json<TagCmd>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if cmd.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if let Some(t) = &cmd.tip {
        if !["aile", "misafir", "bilinmeyen", "kara-liste"].contains(&t.as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    let guard = st.db.lock().await;
    db::ensure_person(&guard, &id, &cmd.name).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    guard
        .execute(
            "UPDATE persons SET name = ?1, host = COALESCE(?2, host) WHERE id = ?3",
            rusqlite::params![cmd.name, cmd.host, id],
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(t) = cmd.tip {
        guard
            .execute("UPDATE persons SET tip = ?1 WHERE id = ?2", rusqlite::params![t, id])
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(wl) = cmd.whitelist {
        // İnsan onayı izi: komut loguna da düşer.
        guard
            .execute(
                "UPDATE persons SET whitelist = ?1 WHERE id = ?2",
                rusqlite::params![wl as i32, id],
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let _ = db::log_command(&guard, "whitelist", &id, true, &cmd.name);
    }
    Ok(Json(serde_json::json!({ "ok": true, "id": id, "name": cmd.name, "host": cmd.host })))
}

fn app(st: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/visits/today", get(visits_today))
        .route("/api/vision/event", post(vision_event))
        .route("/api/presence", get(get_presence).post(set_presence))
        .route("/api/voices", get(list_voices))
        .route("/api/voices/select", post(select_voice))
        .route("/api/greet", post(greet))
        .route("/api/door/open", post(door_open))
        .route("/api/door/deny", post(door_deny))
        .route("/api/persons/{id}", get(get_person))
        .route("/api/persons/{id}/tag", post(tag_person))
        .with_state(st)
}

fn seed_demo(conn: &rusqlite::Connection) {
    // İlk açılışta panel boş kalmasın: 1 aile üyesi. Zararsız, tekrar çalışmaz.
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM persons", [], |r| r.get(0)).unwrap_or(1);
    if n == 0 {
        let t = db::now_str();
        let _ = conn.execute(
            "INSERT INTO persons (id, name, tip, whitelist, visit_count, first_seen, last_seen)
             VALUES ('anne', 'Anne', 'aile', 1, 0, ?1, ?1)",
            [t],
        );
    }
}

#[tokio::main]
async fn main() {
    let conn = db::open("iuds.db").expect("sqlite acilamadi");
    seed_demo(&conn);
    let tts = tts::TtsConfig::from_env();
    println!("tts → {} (voice {})", tts.base_url, tts.voice);
    // RuView MQTT presence: IUDS_MQTT=host:port, IUDS_MQTT_TOPIC=filtre.
    // Broker yoksa abone yeniden dener, kararlar önbellek bayatken fail-closed.
    let (mqtt_host, mqtt_port) = parse_mqtt_env();
    let mqtt_topic =
        std::env::var("IUDS_MQTT_TOPIC").unwrap_or_else(|_| "ruview/#".into());
    let presence: presence::SharedPresence =
        Arc::new(tokio::sync::RwLock::new(presence::PresenceState::default()));
    presence::spawn_subscriber(presence.clone(), mqtt_host.clone(), mqtt_port, mqtt_topic.clone());
    println!("presence ← mqtt {mqtt_host}:{mqtt_port} [{mqtt_topic}]");
    let voice = Arc::new(tokio::sync::RwLock::new(tts.voice.clone()));
    let st = AppState { db: Arc::new(tokio::sync::Mutex::new(conn)), tts, presence, voice };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3901").await.unwrap();
    println!("iuds-core → http://127.0.0.1:3901/api/health (db: iuds.db)");
    axum::serve(listener, app(st)).await.unwrap();
}

fn parse_mqtt_env() -> (String, u16) {
    let raw = std::env::var("IUDS_MQTT").unwrap_or_else(|_| "127.0.0.1:1883".into());
    match raw.rsplit_once(':') {
        Some((h, p)) => (h.to_string(), p.parse().unwrap_or(1883)),
        None => (raw, 1883),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> AppState {
        let conn = db::open_memory().unwrap();
        let tts_cfg = tts::TtsConfig::testable("http://127.0.0.1:39998");
        AppState {
            db: Arc::new(tokio::sync::Mutex::new(conn)),
            tts: tts_cfg.clone(),
            presence: Arc::new(tokio::sync::RwLock::new(presence::PresenceState::default())),
            voice: Arc::new(tokio::sync::RwLock::new(tts_cfg.voice)),
        }
    }

    #[tokio::test]
    async fn health_ok() {
        let h = health().await;
        assert!(h.0.ok);
    }

    #[tokio::test]
    async fn kapi_bos_token_red() {
        let st = test_state();
        let r = door_open(State(st), Json(DoorCmd { visit_id: "v-1".into(), token: "".into() })).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn vision_anne_oto_ac_yazar() {
        let st = test_state();
        {
            // B3: whitelist DB'den okunur — anne'yi DB'de whitelist'li kaydet.
            let g = st.db.lock().await;
            db::ensure_person(&g, "anne", "Anne").unwrap();
            g.execute("UPDATE persons SET whitelist=1 WHERE id='anne'", []).unwrap();
        }
        let req = VisionEventReq {
            person_id: Some("anne".into()),
            camera: Some("cam0".into()),
            yuz: 0.93,
            presence: Some(0.9),
            liveness: 0.85,
            track_suresi: 4.2,
            whitelist: Some(true),
            birlikte_mi: Some(false),
            group_id: None,
        };
        let r = vision_event(State(st.clone()), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "oto_ac");
        let v = visits_today(State(st)).await.unwrap();
        assert_eq!(v.0.len(), 1);
    }

    #[tokio::test]
    async fn vision_client_whitelist_yok_sayilir() {
        // B3: client "whitelist:true" dese bile DB'de whitelist yoksa oto_ac ÇIKMAZ.
        let st = test_state();
        let req = VisionEventReq {
            person_id: Some("salginc".into()),
            camera: None,
            yuz: 0.99,
            presence: Some(0.99),
            liveness: 0.99,
            track_suresi: 9.9,
            whitelist: Some(true), // sahte bayrak
            birlikte_mi: Some(false),
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "soruldu"); // oto_ac DEĞİL
    }

    #[tokio::test]
    async fn vision_db_whitelist_istege_yener() {
        // B3: client "whitelist:false" dese bile DB'de varsa oto_ac çıkar.
        let st = test_state();
        {
            let g = st.db.lock().await;
            db::ensure_person(&g, "anne", "Anne").unwrap();
            g.execute("UPDATE persons SET whitelist=1 WHERE id='anne'", []).unwrap();
        }
        let req = VisionEventReq {
            person_id: Some("anne".into()),
            camera: None,
            yuz: 0.93,
            presence: Some(0.9),
            liveness: 0.85,
            track_suresi: 4.2,
            whitelist: Some(false),
            birlikte_mi: Some(false),
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "oto_ac");
    }

    #[tokio::test]
    async fn vision_fotograf_red() {
        let st = test_state();
        let req = VisionEventReq {
            person_id: None,
            camera: None,
            yuz: 0.91,
            presence: Some(0.2),
            liveness: 0.1,
            track_suresi: 4.0,
            whitelist: Some(false),
            birlikte_mi: Some(false),
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "red");
        assert!(r.0.person_id.starts_with("Ziyaretçi-"));
    }

    #[tokio::test]
    async fn vision_bozuk_skor_400() {
        let st = test_state();
        let req = VisionEventReq {
            person_id: None,
            camera: None,
            yuz: 9.9,
            presence: Some(0.9),
            liveness: 0.9,
            track_suresi: 4.0,
            whitelist: None,
            birlikte_mi: None,
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn tag_kotu_tip_400() {
        let st = test_state();
        let r = tag_person(
            State(st),
            Path("p1".into()),
            Json(TagCmd { name: "X".into(), host: None, tip: Some("uzayli".into()), whitelist: None }),
        )
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn kisi_karti_timeline_ve_yoldas() {
        let st = test_state();
        {
            let g = st.db.lock().await;
            db::ensure_person(&g, "anne", "Anne").unwrap();
            g.execute("UPDATE persons SET tip='aile', whitelist=1 WHERE id='anne'", []).unwrap();
            db::ensure_person(&g, "misafir1", "Misafir").unwrap();
            db::insert_visit(&g, "v-1", "anne", "cam0", "oto_ac", 0.9, 0.9, 0.9, Some("g-1")).unwrap();
            db::insert_visit(&g, "v-2", "misafir1", "cam0", "eslikli_ac", 0.4, 0.9, 0.8, Some("g-1")).unwrap();
        }
        let card = get_person(State(st), Path("misafir1".into())).await.unwrap();
        assert_eq!(card.0.person.id, "misafir1");
        assert_eq!(card.0.visits.len(), 1);
        // misafir1'in yoldaşı anne olmalı (aynı grup).
        assert_eq!(card.0.companions.len(), 1);
        assert_eq!(card.0.companions[0].person_id, "anne");
    }

    #[tokio::test]
    async fn kisi_karti_yoksa_404() {
        let st = test_state();
        let r = get_person(State(st), Path("hayalet".into())).await;
        assert_eq!(r.unwrap_err(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn greet_metin_doner_tts_yoksa_spoken_false() {
        // TTS kapalı (39998): metin yine üretilir, spoken=false, 200.
        let st = test_state();
        let r = greet(
            State(st),
            Json(GreetReq { visit_id: None, karar: "oto_ac".into(), name: Some("Anne".into()) }),
        )
        .await
        .unwrap();
        assert!(r.0.ok);
        assert!(r.0.text.contains("Anne"));
        assert!(!r.0.spoken);
    }

    #[tokio::test]
    async fn greet_kotu_karar_400() {
        let st = test_state();
        let r = greet(
            State(st),
            Json(GreetReq { visit_id: None, karar: "uzayli".into(), name: None }),
        )
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn presence_yoksa_vision_onbellege_duser() {
        // Önbelleğe taze presence yazılınca presence:None olayı onu kullanır.
        let st = test_state();
        set_presence(State(st.clone()), Json(PresenceSet { score: 0.9, source: Some("test".into()) }))
            .await
            .unwrap();
        {
            let g = st.db.lock().await;
            db::ensure_person(&g, "anne", "Anne").unwrap();
            g.execute("UPDATE persons SET whitelist=1 WHERE id='anne'", []).unwrap();
        }
        let req = VisionEventReq {
            person_id: Some("anne".into()),
            camera: None,
            yuz: 0.93,
            presence: None,
            liveness: 0.85,
            track_suresi: 4.2,
            whitelist: Some(true),
            birlikte_mi: None,
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "oto_ac");
    }

    #[tokio::test]
    async fn presence_bayatsa_vision_reddeder() {
        // Önbellek bayat (default) + presence:None → beden yok → red.
        let st = test_state();
        let req = VisionEventReq {
            person_id: None,
            camera: None,
            yuz: 0.95,
            presence: None,
            liveness: 0.9,
            track_suresi: 4.0,
            whitelist: Some(true),
            birlikte_mi: None,
            group_id: None,
        };
        let r = vision_event(State(st), Json(req)).await.unwrap();
        assert_eq!(r.0.karar, "red");
    }

    #[tokio::test]
    async fn presence_aralik_disi_400() {
        let st = test_state();
        let r = set_presence(State(st), Json(PresenceSet { score: 9.9, source: None })).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn ses_secimi_bos_400_dolu_ok() {
        let st = test_state();
        let bad = select_voice(State(st.clone()), Json(VoiceSelect { voice: "  ".into() })).await;
        assert!(bad.is_err());
        let ok = select_voice(State(st.clone()), Json(VoiceSelect { voice: "klon-1".into() }))
            .await
            .unwrap();
        assert_eq!(ok.0["voice"], "klon-1");
        // TTS kapalı: liste boş ama 200 + aktif ses korunur.
        let v = list_voices(State(st)).await;
        assert!(v.0.voices.is_empty());
        assert_eq!(v.0.active, "klon-1");
    }

    #[tokio::test]
    async fn kapi_ac_visit_kararini_gunceller() {
        // B4: Aç → visits.karar=soruldu_acildi + approved_by=panel.
        let st = test_state();
        {
            let g = st.db.lock().await;
            db::ensure_person(&g, "p1", "Misafir").unwrap();
            db::insert_visit(&g, "v-1", "p1", "cam0", "soruldu", 0.4, 0.9, 0.8, None).unwrap();
        }
        let r = door_open(
            State(st.clone()),
            Json(DoorCmd { visit_id: "v-1".into(), token: "t".into() }),
        )
        .await
        .unwrap();
        assert_eq!(r.0.action, "open");
        let g = st.db.lock().await;
        let (karar, onay): (String, String) = g
            .query_row("SELECT karar, approved_by FROM visits WHERE id='v-1'", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!(karar, "soruldu_acildi");
        assert_eq!(onay, "panel");
    }

    #[tokio::test]
    async fn kapi_ac_olmayan_visit_404() {
        let st = test_state();
        let r = door_open(
            State(st),
            Json(DoorCmd { visit_id: "v-hayalet".into(), token: "t".into() }),
        )
        .await;
        assert_eq!(r.unwrap_err(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn kapi_red_visit_kararini_red_yapar() {
        let st = test_state();
        {
            let g = st.db.lock().await;
            db::ensure_person(&g, "p1", "Misafir").unwrap();
            db::insert_visit(&g, "v-1", "p1", "cam0", "soruldu", 0.4, 0.9, 0.8, None).unwrap();
        }
        door_deny(State(st.clone()), Json(DoorCmd { visit_id: "v-1".into(), token: "t".into() }))
            .await
            .unwrap();
        let g = st.db.lock().await;
        let karar: String = g
            .query_row("SELECT karar FROM visits WHERE id='v-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(karar, "red");
    }

    #[tokio::test]
    async fn greet_path_traversal_400() {
        // B5: visit_id dosya yoluna gidiyor — traversal reddedilir.
        let st = test_state();
        let r = greet(
            State(st),
            Json(GreetReq { visit_id: Some("../evil".into()), karar: "oto_ac".into(), name: None }),
        )
        .await;
        assert_eq!(r.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn greet_soruldu_acildi_kabul() {
        // B4 sonrası panelden gelen yeni karar değeri greet'te de geçerli.
        let st = test_state();
        let r = greet(
            State(st),
            Json(GreetReq { visit_id: None, karar: "soruldu_acildi".into(), name: Some("Anne".into()) }),
        )
        .await
        .unwrap();
        assert!(r.0.text.contains("Hoş geldin Anne"));
    }
}
