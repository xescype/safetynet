use std::time::Duration;
pub const TTS_TIMEOUT: Duration = Duration::from_secs(8);
pub fn greet_text(karar:&str, name: Option<&str>) -> String {
    let name = name.filter(|n| !n.trim().is_empty()).unwrap_or("Ziyaretçi");
    match karar {
        "oto_ac" | "soruldu_acildi" => format!("Hoşgeldiniz."),
        "eslikli_ac" => format!("Hoş geldiniz,ev sahibiyle giriş yapabilirsiniz."),
        "soruldu" => format!("Merhaba.Ev halkına haber verdim.Lütfen biraz bekle."),
        _=>"Tanınamadınız.Lütfen bekleyin.".to_string()
    }
}
pub fn safe_file_id(s:&str) -> bool{
    !s.is_empty()
    && s.chars()
    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-'| '_' | '.' ))
}