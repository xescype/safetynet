//! IUDS karar motoru — `iuds-vision/decision.py` ile birebir aynı kural.
//! docs/02-guvenlik.md'deki AND kuralının sayısal hali:
//! KARAR = yüz eşleşti VE beden orada VE canlı VE track tutarlı.
//! Şüphede fail-closed: biri bile tutmazsa reddet.

pub const ESIK_YUZ: f32 = 0.65;
pub const ESIK_PRESENCE: f32 = 0.50;
pub const ESIK_LIVENESS: f32 = 0.60;
pub const MIN_TRACK_SURESI: f32 = 3.0;

/// Python tarafındaki `olay` dict'inin Rust karşılığı.
/// Skorlar 0-1 arası beklenir; aralığı `validate` kontrol eder.
#[derive(Debug, Clone)]
pub struct VisionScores {
    pub yuz: f32,
    pub presence: f32,
    pub liveness: f32,
    pub track_suresi: f32,
    pub whitelist: bool,
    pub birlikte_mi: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Aile: tüm katmanlar geçti.
    OtoAc,
    /// Ailenin misafiri: sorgulama yok, bilgi bildirimi.
    EslikliAc,
    /// Tanıdık değil: ev halkına sor.
    Sor,
    /// Spoof / eksik katman: kapı kapalı.
    Reddet,
}

impl Decision {
    /// DB + API'de saklanan kanonik ad (`docs/06` karar kümesi).
    pub fn as_str(self) -> &'static str {
        match self {
            Decision::OtoAc => "oto_ac",
            Decision::EslikliAc => "eslikli_ac",
            Decision::Sor => "soruldu",
            Decision::Reddet => "red",
        }
    }
}

/// Skor aralığını kontrol et. Hatalı skorla karar verilmez (fail-closed).
pub fn validate(s: &VisionScores) -> Result<(), &'static str> {
    for v in [s.yuz, s.presence, s.liveness] {
        if !(0.0..=1.0).contains(&v) || !v.is_finite() {
            return Err("skor 0-1 aralığında olmalı");
        }
    }
    if !s.track_suresi.is_finite() || s.track_suresi < 0.0 {
        return Err("track_suresi negatif olamaz");
    }
    Ok(())
}

pub fn decide(s: &VisionScores) -> (Decision, &'static str) {
    // 1) AND kuralı: biri bile tutmazsa red.
    if s.presence < ESIK_PRESENCE {
        return (Decision::Reddet, "beden yok (fotograf olabilir)");
    }
    if s.liveness < ESIK_LIVENESS {
        return (Decision::Reddet, "canlilik yok");
    }
    if s.track_suresi < MIN_TRACK_SURESI {
        return (Decision::Reddet, "takip cok kisa");
    }
    if s.yuz < ESIK_YUZ {
        // Eşlikli giriş istisnası: decision.py ile aynı — yüz tutmasa bile
        // aileyle yan yana gelen misafir sorgulanmadan alınır, loglanır.
        if s.birlikte_mi {
            return (Decision::EslikliAc, "ailenin misafiri, sorgulama yok");
        }
        return (Decision::Sor, "yuz tutmadi, ev halkina sor");
    }

    // 2) Buraya gelen: yüz + presence + liveness + track HEPSİ geçti.
    if s.birlikte_mi && !s.whitelist {
        return (Decision::EslikliAc, "ailenin misafiri, sorgulama yok");
    }
    if s.whitelist {
        return (Decision::OtoAc, "aile, tum katmanlar gecti");
    }
    (Decision::Sor, "tanidik degil, onay gerek")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn olay(yuz: f32, presence: f32, liveness: f32, sure: f32, wl: bool, birlikte: bool) -> VisionScores {
        VisionScores { yuz, presence, liveness, track_suresi: sure, whitelist: wl, birlikte_mi: birlikte }
    }

    #[test]
    fn anne_oto_ac() {
        let (k, _) = decide(&olay(0.93, 0.9, 0.85, 4.2, true, false));
        assert_eq!(k, Decision::OtoAc);
    }

    #[test]
    fn anne_misafir_eslikli() {
        // decision.py SENARYOLAR ile aynı: yüz düşük ama birlikte_mi=true.
        let (k, _) = decide(&olay(0.40, 0.9, 0.8, 5.0, false, true));
        assert_eq!(k, Decision::EslikliAc);
    }

    #[test]
    fn elde_fotograf_red() {
        let (k, _) = decide(&olay(0.91, 0.2, 0.1, 4.0, false, false));
        assert_eq!(k, Decision::Reddet);
    }

    #[test]
    fn tek_kare_red() {
        let (k, _) = decide(&olay(0.99, 0.9, 0.8, 0.4, false, false));
        assert_eq!(k, Decision::Reddet);
    }

    #[test]
    fn yabanci_sor() {
        let (k, _) = decide(&olay(0.41, 0.85, 0.75, 4.5, false, false));
        assert_eq!(k, Decision::Sor);
    }

    #[test]
    fn bozuk_skor_reddedilir() {
        let s = olay(9.9, 0.9, 0.8, 4.0, true, false);
        assert!(validate(&s).is_err());
    }
}
