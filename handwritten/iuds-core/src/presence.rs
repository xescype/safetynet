use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime,Local};
use tokio::sync::RwLock;
pub const STALE_SECS: i64 = 15;
#[derive(Debug,Clone)]
pub struct PresenceState{
    pub score :f32,
    pub source : String,
    pub updated_at:DateTime<Local>, 
}
impl Default for PresenceState{
    fn default() -> Self {
        Self{
            score : 0.0,
            source: "yok".into(),
            updated_at: Local::now() - chrono::Duration::hours(1),
        }
    }
}
pub type SharedPresence = Arc<RwLock<PresenceState>>;
pub fn fresh(s:&PresenceState) -> bool{
    (Local::now() - s.updated_at).num_seconds() < STALE_SECS
}
pub async fn current_score(p:&SharedPresence)-> f32{
    let s = p.read().await;
    if fresh(&s){s.score} else{0.0}
}
pub fn parse_payload(topic: &str, payload: &[u8]) -> Option<(f32, String)> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    let t = topic.to_lowercase();

    if let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(n) = map.get("presence_score").and_then(num) {
            return Some((clamp(if n > 1.0 { n / 100.0 } else { n }), "ruview:presence_score".into()));
        }
        if let Some(b) = map.get("presence").and_then(bool_val) {
            return Some((if b { 0.9 } else { 0.05 }, "ruview:presence".into()));
        }
        for k in ["n_persons", "person_count", "count"] {
            if let Some(n) = map.get(k).and_then(num) {
                return Some((if n >= 1.0 { 0.8 } else { 0.05 }, "ruview:count".into()));
            }
        }
        if let Some(n) = map.get("motion").and_then(num) {
            return Some((clamp(if n > 1.0 { n / 100.0 } else { n }), "ruview:motion".into()));
        }
        return None;
    }

    if let Some(b) = bool_val(&serde_json::Value::String(text.into())) {
        return Some((if b { 0.9 } else { 0.05 }, "mqtt:binary".into()));
    }
    if let Ok(n) = text.parse::<f32>() {
        if t.contains("count") || t.contains("person") {
            return Some((if n >= 1.0 { 0.8 } else { 0.05 }, "mqtt:count".into()));
        }
        return Some((clamp(if n > 1.0 { n / 100.0 } else { n }), "mqtt:score".into()));
    }
    None
}

fn clamp(n: f32) -> f32 {
    n.clamp(0.0,1.0)
}
fn num(v:&serde_json::Value) -> Option<f32>{
    v.as_f64().map(|n| (n as f32)) 
}
fn bool_val(v: &serde_json::Value) -> Option<bool>{
    match v{
        serde_json::Value::Bool(b) => Some(*b),
        serde_json::Value::String(s) => match s.trim().to_lowercase().as_str(){
            "on" | "true" | "1" | "home" | "occupied" => Some(true),
            "off" | "false" | "0" | "not_home" | "clear" => Some(false),
            _ => None,
        },
        serde_json::Value::Number(n) => Some(n.as_f64().unwrap_or(0.0) >= 1.0),
        _=>None,
    }
}
pub fn spawn_subscriber(presence:SharedPresence, broker_host: String, broker_port: u16, topic:String){
    tokio::spawn(async move{
        use rumqttc::{AsyncClient,Event,MqttOptions,Packet,QoS};
        loop{
            let mut opts = MqttOptions::new("iuds-core",&broker_host, broker_port);
            opts.set_keep_alive(Duration::from_secs(10));
            let(client,mut evloop) = AsyncClient::new(opts,16);
            if client.subscribe(&topic,QoS::AtMostOnce).await.is_err(){
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }
            loop{
                match evloop.poll().await{
                    Ok(Event::Incoming(Packet::Publish(p))) =>{
                        if let Some((score, source)) = parse_payload(
                            &p.topic, &p.payload
                        ){
                            let mut s = presence.write().await;
                            s.score = score;
                            s.source = source;
                            s.updated_at = Local::now();
                        }
                        
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn ruvview_presence_score_yuzde(){
        let (s,src) = parse_payload("ha/sensor/presence_score/state",b"87").unwrap();
        assert!((s - 0.87).abs()< 1e-6);
    }
        #[test]
    fn json_edge_vitals() {
        let (s, _) =
            parse_payload("ruview/node1", br#"{"presence_score": 62.0, "n_persons": 1}"#).unwrap();
        assert!((s - 0.62).abs() < 1e-6);
        let (s, _) = parse_payload("ruview/node1", br#"{"n_persons": 2}"#).unwrap();
        assert_eq!(s, 0.8);
        let (s, _) = parse_payload("ruview/node1", br#"{"presence": "ON"}"#).unwrap();
        assert_eq!(s, 0.9);
    }

    #[test]
    fn binary_on_off() {
        assert_eq!(parse_payload("t/presence", b"ON").unwrap().0, 0.9);
        assert_eq!(parse_payload("t/presence", b"OFF").unwrap().0, 0.05);
    }
    #[test]
    fn cop_tutmaz() {
        assert!(parse_payload("t/x", b"merhaba").is_none());
        assert!(parse_payload("t/x", b"{bozuk").is_none());
    }
    #[tokio::test]
    async fn bayat_sifir_sayar() {
        let p: SharedPresence = Arc::new(RwLock::new(PresenceState::default()));
        assert_eq!(current_score(&p).await, 0.0);
        {
            let mut s = p.write().await;
            s.score = 0.9;
            s.updated_at = Local::now();
        }
        assert_eq!(current_score(&p).await, 0.9);
    }
}