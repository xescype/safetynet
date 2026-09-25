# SafetyNet (IUDS) — Elle Yazım İlerleme Takibi (progress-handwritten.md)

Bu dosya, projeyi Rust dilini sıfırdan öğrenerek elle yazma sürecimizin güncel durumunu, tamamlanan adımları ve sıradaki görevleri takip eder.

---

## 🎯 Proje Bileşenleri & Yol Haritası

`iuds-core` (Rust Çekirdeği) 5 modül + 1 manifest dosyasıdır. AI'nin yazdığı referans sürüm `nothandwrittencode/iuds-core/src/`, elle yazdığım sürüm `handwritten/iuds-core/src/` altında.

| # | Dosya | Elle yazım durumu | Ne iş yapar |
| :--- | :--- | :--- | :--- |
| 1 | `src/decisions.rs` | ✅ **Bitti — testler 3/3 geçiyor** | Karar motoru: AND kuralı, eşikler (`ESIK_YUZ`, `ESIK_PRESENCE`, `ESIK_LIVENESS`, `MIN_TRACK_SURESI`), `VisionScores`, `Decision` enum'ı, `validate`, `decide` + birim testleri. |
| 2 | `src/db.rs` | ✅ **Bitti — testler 4/4 geçiyor** | SQLite tek-yazar katmanı. `SCHEMA`, `open`, `open_memory`, `now_str`, `today_prefix`, `next_unknown_id`, `ensure_person`, `is_whitelisted` (B3), `bump_visit_counter`, `insert_visit`, `log_command` + 4 birim testi. |
| 3 | `src/presence.rs` | ✅ **Bitti — testler 5/5 geçiyor** | RuView MQTT + LD2410 presence önbelleği; bayat veri = 0 puan (fail-closed), kopmaz Tokio MQTT istemcisi, JSON/metin ayrıştırıcı. |
| 4 | `src/tts.rs` | 🟡 **SIRADAKİ HEDEF** (dosya henüz yok) | VoiceStudio yerel TTS istemcisi; karşılama metni + `greetings/<visit_id>.wav`, path traversal koruması. |
| 5 | `src/main.rs` | ⏳ Sırada (şimdilik sadece mod bildirimleri) | Axum HTTP sunucusu (`127.0.0.1:3901`), 11 API ucu, `AppState`, entegrasyon testleri. |
| 6 | `Cargo.toml` | 🟡 Kısmen | `rusqlite`, `chrono`, `tokio`, `serde`, `serde_json`, `rumqttc` var. Sırada: `reqwest` (TTS için), `axum` (main için). |

**Ortam doğrulaması:** `cargo 1.98.1` / `rustc 1.98.1`, `edition = "2024"`.
`cargo test` çıktısı: **12 passed; 0 failed** (3 `decisions.rs` + 4 `db.rs` + 5 `presence.rs`).

---

## 📍 Nerede Kaldık? (`src/presence.rs` BİTTİ! 🎉 Sıradaki: `src/tts.rs`)

`src/presence.rs` (152 satır) ve testleri eksiksiz tamamlandı, 12 testin hepsi başarıyla geçiyor.

### ✅ presence.rs İçinde Tamamlananlar (Hepsi elle yazıldı)
1. `STALE_SECS: i64 = 15` — 15 sn veri yoksa varlık skoru 0 kabul edilir (Fail-Closed).
2. `PresenceState` struct'ı — `score`, `source`, `updated_at`.
3. `impl Default for PresenceState` — 1 saat öncesine damgalı güvenli başlangıç durumu.
4. `SharedPresence` type alias — `Arc<RwLock<PresenceState>>`.
5. `fresh(&PresenceState) -> bool` — 15 sn tazelik denetimi.
6. `current_score(&SharedPresence) -> f32` — Async kilit okuma ve Fail-Closed skorlama.
7. `clamp(n: f32) -> f32` — 0.0-1.0 arasına hapsetme.
8. `num(&Value) -> Option<f32>` — JSON sayı okuyucu.
9. `bool_val(&Value) -> Option<bool>` — `"ON"`/`"OFF"`, `"true"`/`"false"` çevirici.
10. `parse_payload(&str, &[u8]) -> Option<(f32, String)>` — JSON ve düz metin MQTT paket çözücü.
11. `spawn_subscriber(...)` — Kopunca 5 sn arayla yeniden bağlanan Tokio arka plan MQTT görevi.
12. `#[cfg(test)] mod tests` — 5/5 birim testi:
    - `ruvview_presence_score_yuzde` ✅
    - `json_edge_vitals` ✅
    - `binary_on_off` ✅
    - `cop_tutmaz` ✅
    - `bayat_sifir_sayar` (Async `#[tokio::test]`) ✅

---

## 🎙️ Sıradaki Modül: `src/tts.rs` (Sesli Karşılama & VoiceStudio Katmanı)

Bu modül, kapıda bir karar verildiğinde (örn. `"Hoş geldin Anne"`, `"Lütfen bekleyin, onay isteniyor"` gibi) VoiceStudio yerel yapay zeka TTS motoruna HTTP isteği atar, ses dosyasını `greetings/<visit_id>.wav` olarak diske kaydeder ve kapı hoparlöründen çalar.

### 📌 İlk Adımlar:
1. `Cargo.toml` dosyasına `reqwest` HTTP istemcisini eklemek.
2. `main.rs` içine `mod tts;` satırını eklemek.
3. `src/tts.rs` dosyasını oluşturup sabitleri, `greet_text` ve `safe_file_id` (Path Traversal savunması) fonksiyonlarını yazmak.

---

## 🧭 Çalışma Prensibi (nasıl ilerliyoruz?)
- **Kural (`tollms.md`):** Ben istemedikçe AI kodu doğrudan dosyaya yazmaz. AI **anlatır** → **ben elle yazarım**.
- **Döngü:** (1) AI satırı/kelimeyi açıklar → (2) ben editöre yazarım → (3) `cargo check` → (4) hatayı beraber okuruz → (5) `cargo test` yeşilse sıradaki fonksiyon.
- **Hata felsefesi:** Derleyici düşman değil öğretmen. Hata metnini Türkçeye çevirip nedenini anlamadan düzeltme yapmıyoruz.
- **Doğrulama:** Her fonksiyondan sonra test yazarız. Test yoksa "bitti" demiyoruz.

### 🧰 Komut Kutusu (her zaman bu klasörde çalışıyoruz)
```powershell
cd c:\Users\yilma\Desktop\ideas\safetynet\handwritten\iuds-core
cargo check      # sadece "derleniyor mu?" — en hızlı geri bildirim
cargo test       # birim/entegrasyon testlerini koşar
cargo run        # HTTP sunucusunu başlatır (main.rs bitince)
cargo clippy     # stil ve iyi pratik uyarıları (en sonda)
```

---

## 🗺️ Kalan İşin Ders Ders Planı

### A) `src/db.rs` — kalan 7 fonksiyon + testler (ŞU ANKİ DERS)
| Ders | Yazılacak | Öğreneceğim Rust kavramı |
| :--- | :--- | :--- |
| A0 | `main.rs`'e `mod db;` + 3 typo düzeltmesi | `mod` bildirimi, `use` listesi, sözdizimi (syntax) |
| A1 | `today_prefix() -> String` | `format!` makrosu, `&'static str` ↔ `String`, `.to_string()` |
| A2 | `next_unknown_id` | `query_row`, `params!`, closure `\|r\| r.get(0)`, `?` operatörü, `Result<T>` |
| A3 | `ensure_person` | `INSERT OR IGNORE`, `execute` → `usize`, referans parametreler (`&str`) |
| A4 | `is_whitelisted` | `i64` ↔ `bool` çevirimi, tek satır okuma, B3 kuralı |
| A5 | `bump_visit_counter` | `UPDATE`, sayaç mantığı, `last_seen` |
| A6 | `insert_visit` | 8 parametre, `Option<&str>` (SQL NULL), `#[allow(...)]` |
| A7 | `log_command` | Append-only disiplini, `ok as i32` tip dönüşümü |
| A8 | `mod tests` | `#[cfg(test)]`, `#[test]`, testin `Result<()>` dönmesi, `assert_eq!`/`assert_ne!`/`assert!`, `use super::*` |

### B) `src/presence.rs` — yeni dosya (referans: `nothandwrittencode/iuds-core/src/presence.rs`)
- `STALE_SECS = 15`, `PresenceState { score, source, updated_at }`, `impl Default for PresenceState` (trait implementasyonu)
- `type SharedPresence = Arc<RwLock<PresenceState>>` → **type alias**, `Arc` (paylaşılan sahiplik), `RwLock` (kilit)
- `fresh()` ve `async fn current_score()` → bayat veride `0.0` döner (**fail-closed**)
- `parse_payload(topic, payload)` → `serde_json::Value`, `if let` / `match`, `Option` zincirleme, `clamp(0.0, 1.0)`, `bool_val` yardımcısı
- `spawn_subscriber()` → `tokio::spawn(async move { ... })`, `loop`, `rumqttc::AsyncClient`, kopunca 5 sn bekleyip yeniden bağlanma
- Testler: `#[tokio::test]` (async test), `bayat_sifir_sayar`, `binary_on_off`, `cop_tutmaz`
- Cargo.toml'a eklenecek: `tokio`, `serde_json`, `rumqttc`

### C) `src/tts.rs` — yeni dosya (referans: aynı ad)
- `TTS_TIMEOUT` (8 sn), `greet_text(karar, name)` → `match` + `Option::filter` + `unwrap_or("Ziyaretçi")`
- `safe_file_id()` → `chars().all(...)`, `matches!` makrosu, **path traversal** savunması (B5)
- `TtsConfig { base_url, voice, client }` + `from_env()` (`std::env::var`), `testable()` (testler için 2 sn timeout)
- `speak()`, `list_voices()`, `speak_to_file()` → `reqwest::Client`, `serde_json::json!`, `tokio::fs::create_dir_all/write`, `Result<_, String>` + `map_err`
- `#[derive(Serialize, Deserialize)]` → serde'a giriş; backend kapalıysa panik yok (`spoken:false`)
- Cargo.toml'a eklenecek: `reqwest`, `serde`, `serde_json`

### D) `src/main.rs` — HTTP sunucu (referans: aynı ad, ~1000 satır)
- `#[tokio::main] async fn main()` → runtime, `TcpListener::bind("127.0.0.1:3901")`, `axum::serve`
- `#[derive(Clone)] struct AppState { db: Arc<Mutex<Connection>>, tts, presence, voice }`
- `fn app(st: AppState) -> Router` + 11 `.route(...)` (sözleşme: `totaldocs/safetynet notes/03-Api-Sozlesmesi.md`)
- Handler imzaları: `async fn f(State(st): State<AppState>, Json(req): Json<Req>) -> Result<Json<Resp>, StatusCode>`
- İstek/cevap modelleri: `#[derive(Serialize)]` / `#[derive(Deserialize)]`
- `vision_event` → kalp: `validate` → `person_id` → whitelist **DB'den** (B3) → `decide` → `insert_visit` → `log_command` → **bloklamayan** `tokio::spawn` ile TTS
- `door_open` / `door_deny` → ziyaret kaydını güncelleme (B4), boş token 400
- `get_person` → `prepare` + `query_map` ile 30 ziyaret + yoldaşlar (`group_id`)
- `seed_demo()`, `parse_mqtt_env()`, entegrasyon testleri (handler'ları doğrudan çağırır → ekstra test bağımlılığı gerekmiyor, `#[tokio::test]`)

### E) `Cargo.toml`
`edition = "2024"` korunur. Bağımlılıklar modül yazıldıkça eklenir; `mod db;` dersinde ek bağımlılık gerekmiyor (rusqlite + chrono yeterli).

---

## ❓ Açık Sorular / Vereceğim Kararlar
1. **`Ziyaretci` mi `Ziyaretçi` mi?** Notlarda ve AI referansında `Ziyaretçi-...` (Türkçe `ç`) geçiyor, ben ASCII `Ziyaretci` yazdım. ID ileride dosya adına girebileceği için (`safe_file_id` sadece `[A-Za-z0-9._-]` kabul eder) **ASCII `Ziyaretci`** mantıklı. Karar verirsem `totaldocs` notlarındaki yazımı da güncellemem gerekir.
2. **`decisions.rs` ile referans arasında 1 fark var:** Referansta yüz eşiği geçildikten sonra `if s.birlikte_mi && !s.whitelist { EslikliAc }` dalı var; bende yok → o durumda `Sor` dönüyorum. Yani "yüzü tanınan ama whitelist'te olmayan, ailesiyle gelen kişi" referansta sorgusuz alınırken bende sorgulanıyor. Bilinçli mi karar vereceğim, testini de ona göre ekleyeceğim.
3. **Client'ın gönderdiği `whitelist` alanı** karar için **kullanılmaz**, DB'den okunur (B3). `main.rs` dersinde testle doğrulayacağım.

## 📚 Referans Eşlemesi (AI sürümü ↔ elle yazılan)
| Elle yazılan | AI referansı | Not |
| :--- | :--- | :--- |
| `handwritten/iuds-core/src/decisions.rs` | `nothandwrittencode/iuds-core/src/decision.rs` | Dosya adı farklı (çoğul/tekil), kural aynı. |
| `handwritten/iuds-core/src/db.rs` | `nothandwrittencode/iuds-core/src/db.rs` | `SCHEMA` birebir aynı. |
| (henüz yok) | `nothandwrittencode/iuds-core/src/presence.rs` | Sırada. |
| (henüz yok) | `nothandwrittencode/iuds-core/src/tts.rs` | Sırada. |
| `handwritten/iuds-core/src/main.rs` | `nothandwrittencode/iuds-core/src/main.rs` | En son; tüm parçaları birleştiren dosya. |

---

## ✅ Sıradaki Oturum Kontrol Listesi (Ders A0 + A1 + A2)
- [ ] `main.rs` içine `mod db;` satırını ekle.
- [ ] `db.rs` satır 2 → `use rusqlite::{Connection, Result, params};`
- [ ] `db.rs` satır 51 → `pub fn next_unknown_id(conn: &Connection) -> Result<String> {`
- [ ] `today_prefix()` fonksiyonunu `now_str()`'in hemen altına yaz.
- [ ] `next_unknown_id` içindeki `format!` satırını `Ok(format!("Ziyaretci-{day}-{:03}", n + 1))` yap.
- [ ] `cargo check` → **0 hata** (uyarı olması normal).
- [ ] `cargo test` → 3 test hâlâ geçiyor.
- [ ] Sonra `db.rs` testlerini yaz (Ders A8) → `cargo test` toplam 7 yeşil.
