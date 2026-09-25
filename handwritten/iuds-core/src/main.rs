mod db;
mod decisions;
mod presence;
mod tts;
use serde:: {Deserialize , Serialize};
#[derive(Deserialize)]
struct VisionEventReq{
person_id: Option<String>,
camera: Option<String>,
yuz:f32,
presence:Option<f32>,
liveness: f32,
track_suresi: f32,
whitelist: Option<bool>,
birlikte_mi: bool,
group_id: Option<String>
}
#[derive(Serialize)]
struct VisionEventRes{
    ok: bool,
    visit_id: String,
    person_id:String,
    karar: &'static str,
    neden: &'static str,
}
fn main() {
    println!("Hello, world!");
}
