mod db;
mod decisions;
mod presence;
mod tts;
use serde::Deserialize;
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
fn main() {
    println!("Hello, world!");
}
