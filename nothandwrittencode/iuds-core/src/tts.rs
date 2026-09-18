//! IUDS ses katmanı — VoiceStudio local TTS istemcisi.
//! docs/03-mimari: Rust röle + VoiceStudio client. docs/07: LLM sadece
//! karşılama metni üretir, kapı açamaz — LLM gelene kadar şablon metin.
//!
//! API: VoiceStudio OpenAI-uyumlu `POST {base}/v1/audio/speech`
//! `{model, voice, input, response_format}` → wav bytes.
//! Bkz VoiceStudio-main/skills/omnivoice/SKILL.md
//!
//! İlke: TTS asla kapı kararını bloklamaz. Backend kapalıysa
//! `spoken:false` dönülür, karar akışı aynen devam eder.

use std::time::Duration;

/// Zil bütçesi (docs/01): TTS 4-9sn. Üstüne çıkarsa bekleme, sessiz devam.
pub const TTS_TIMEOUT: Duration = Duration::from_secs(8);

/// Karara göre karşılama metni. `{name}` bilinmiyorsa "Ziyaretçi".
/// Not: kovma tonu (red) bilinçli kısa ve serttir.
/// `soruldu_acildi`: insan onayıyla açılan ziyaret (docs/14 B4) — oto_ac tonu.
pub fn greet_text(karar: &str, name: Option<&str>) -> String {
    let name = name.filter(|n| !n.trim().is_empty()).unwrap_or("Ziyaretçi");
    match karar {
        "oto_ac" | "soruldu_acildi" => format!("Hoş geldin {name}, kapı açılıyor."),
        "eslikli_ac" => format!("Hoş geldiniz, ev sahibinizle giriş yapabilirsiniz."),
        "soruldu" => format!("Merhaba {name}, ev halkına haber verdim, lütfen kapıda bekleyin."),
        _ => "Tanınamadınız, lütfen zili kullanıp bekleyin.".to_string(),
    }
}

/// Dosya adı güvenliği (B5): yalnız [A-Za-z0-9._-] kabul edilir;
/// "/", "\\", ".." gibi path traversal denemeleri reddedilir.
pub fn safe_file_id(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

#[derive(Debug, Clone)]
pub struct TtsConfig {
    pub base_url: String,
    pub voice: String,
    pub client: reqwest::Client,
}

impl TtsConfig {
    /// `IUDS_TTS_URL` (varsayılan http://127.0.0.1:3900),
    /// `IUDS_VOICE` (varsayılan alloy; klon id gelince env'den verilir).
    pub fn from_env() -> Self {
        let base_url =
            std::env::var("IUDS_TTS_URL").unwrap_or_else(|_| "http://127.0.0.1:3900".into());
        let voice = std::env::var("IUDS_VOICE").unwrap_or_else(|_| "alloy".into());
        let client = reqwest::Client::builder()
            .timeout(TTS_TIMEOUT)
            .build()
            .expect("http istemcisi kurulamadi");
        Self { base_url, voice, client }
    }

    pub fn testable(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("http istemcisi kurulamadi");
        Self { base_url: base_url.into(), voice: "alloy".into(), client }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VoiceInfo {
    pub id: String,
    pub name: String,
}

/// Klon sesleri listele (`GET {base}/v1/audio/voices`).
/// Backend kapalıysa Err — çağıran `voices:[]` + gerekçe döner, panik yok.
pub async fn list_voices(cfg: &TtsConfig) -> Result<Vec<VoiceInfo>, String> {
    let url = format!("{}/v1/audio/voices", cfg.base_url.trim_end_matches('/'));
    let resp =
        cfg.client.get(&url).send().await.map_err(|e| format!("tts erisilmiyor: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("tts http {}", resp.status()));
    }
    let v: serde_json::Value = resp.json().await.map_err(|e| format!("json: {e}"))?;
    // Şekil esnek: {voices:[...]} / {data:[...]} / düz liste.
    let arr = v
        .get("voices")
        .or_else(|| v.get("data"))
        .or_else(|| v.get("profiles"))
        .or(Some(&v));
    let mut out = Vec::new();
    if let Some(items) = arr.and_then(|a| a.as_array()) {
        for it in items {
            let id = it
                .get("id")
                .and_then(|x| x.as_str())
                .map(str::to_string)
                .unwrap_or_default();
            if id.is_empty() {
                continue;
            }
            let name = it
                .get("name")
                .and_then(|x| x.as_str())
                .map(str::to_string)
                .unwrap_or_else(|| id.clone());
            out.push(VoiceInfo { id, name });
        }
    }
    Ok(out)
}

#[derive(Debug)]
pub struct SpeakOutcome {
    pub spoken: bool,
    pub bytes: usize,
    pub detail: String,
}

/// Metni sese çevir. Başarısızlık (kapalı backend, timeout, 4xx/5xx)
/// hata fırlatmaz — `spoken:false` + gerekçe döner.
pub async fn speak(cfg: &TtsConfig, text: &str) -> SpeakOutcome {
    if text.trim().is_empty() {
        return SpeakOutcome { spoken: false, bytes: 0, detail: "boş metin".into() };
    }
    let url = format!("{}/v1/audio/speech", cfg.base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": "tts-1",
        "voice": cfg.voice,
        "input": text,
        "response_format": "wav",
    });
    let resp = match cfg.client.post(&url).json(&body).send().await {
        Ok(r) => r,
        Err(e) => {
            return SpeakOutcome {
                spoken: false,
                bytes: 0,
                detail: format!("tts erisilmiyor: {e}"),
            };
        }
    };
    if !resp.status().is_success() {
        return SpeakOutcome {
            spoken: false,
            bytes: 0,
            detail: format!("tts http {}", resp.status()),
        };
    }
    match resp.bytes().await {
        Ok(b) if !b.is_empty() => {
            SpeakOutcome { spoken: true, bytes: b.len(), detail: "ok".into() }
        }
        _ => SpeakOutcome { spoken: false, bytes: 0, detail: "boş ses".into() },
    }
}

/// Wav'i diske yaz: `greetings/<visit_id>.wav`. Hoparlör çalma OS işidir;
///
/// dosya hazır olunca ev otomasyonu / panel indirip çalabilir.
pub async fn speak_to_file(cfg: &TtsConfig, text: &str, visit_id: &str) -> (SpeakOutcome, Option<String>) {
    let url = format!("{}/v1/audio/speech", cfg.base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": "tts-1",
        "voice": cfg.voice,
        "input": text,
        "response_format": "wav",
    });
    let bytes = match cfg.client.post(&url).json(&body).send().await {
        Ok(r) if r.status().is_success() => match r.bytes().await {
            Ok(b) if !b.is_empty() => b,
            _ => return (SpeakOutcome { spoken: false, bytes: 0, detail: "boş ses".into() }, None),
        },
        Ok(r) => {
            return (
                SpeakOutcome { spoken: false, bytes: 0, detail: format!("tts http {}", r.status()) },
                None,
            );
        }
        Err(e) => {
            return (
                SpeakOutcome { spoken: false, bytes: 0, detail: format!("tts erisilmiyor: {e}") },
                None,
            );
        }
    };
    let path = format!("greetings/{visit_id}.wav");
    match tokio::fs::create_dir_all("greetings").await {
        Ok(()) => {}
        Err(e) => {
            return (
                SpeakOutcome { spoken: false, bytes: bytes.len(), detail: format!("klasor: {e}") },
                None,
            );
        }
    }
    match tokio::fs::write(&path, &bytes).await {
        Ok(()) => (
            SpeakOutcome { spoken: true, bytes: bytes.len(), detail: "ok".into() },
            Some(path),
        ),
        Err(e) => (
            SpeakOutcome { spoken: false, bytes: bytes.len(), detail: format!("yazma: {e}") },
            None,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metinler_karara_gore() {
        assert_eq!(greet_text("oto_ac", Some("Anne")), "Hoş geldin Anne, kapı açılıyor.");
        assert!(greet_text("soruldu_acildi", Some("Anne")).contains("Hoş geldin Anne"));
        assert!(greet_text("eslikli_ac", None).contains("ev sahibinizle"));
        assert!(greet_text("soruldu", Some("X")).contains("bekleyin"));
        assert!(greet_text("red", None).contains("zili"));
        // Bilinmeyen karar → red tonu (fail-closed metni).
        assert!(greet_text("???", None).contains("zili"));
    }

    #[test]
    fn dosya_adi_guvenligi() {
        assert!(safe_file_id("v-2026-09-17-001"));
        assert!(safe_file_id("abc_DEF.123"));
        assert!(!safe_file_id("../evil"));
        assert!(!safe_file_id("a/b"));
        assert!(!safe_file_id("a\\b"));
        assert!(!safe_file_id(""));
        assert!(!safe_file_id("boş luk")); // unicode + boşluk reddedilir
    }

    #[tokio::test]
    async fn kapali_backend_sessiz_gecer() {
        // Bağlanılamaz port: panik yok, spoken=false.
        let cfg = TtsConfig::testable("http://127.0.0.1:39998");
        let o = speak(&cfg, "merhaba").await;
        assert!(!o.spoken);
        assert!(o.detail.contains("erisilmiyor"));
    }

    #[tokio::test]
    async fn bos_metin_istek_atmaz() {
        let cfg = TtsConfig::testable("http://127.0.0.1:39998");
        let o = speak(&cfg, "   ").await;
        assert!(!o.spoken);
    }
}
