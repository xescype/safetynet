# 16 — Mimari Öneriler ve Teknik Tespitler

Durum: 2026-09 Değerlendirmesi.
Bu doküman, sistemin güvenliği, donanım adaptasyonu ve sensör füzyonu üzerine yapılan teknik önerileri içerir.

---

## 1. Hardcode'suz Mimari ve Güvenlik Çitleri (Guardrails) Önerisi

Sistemde konfigüre edilebilir alanlar artırılırken güvenlik zafiyeti oluşmaması için 3 katmanlı mimari önerilir:

1. **Doğrudan Config'e (`config.toml`) Taşınacaklar:**
   - Ağ ve dinleme portları (`127.0.0.1:3901`, MQTT portu).
   - Veritabanı dosya yolu (`iuds.db`).
   - TTS karşılama cümleleri ve bildirim şablonları.
   - Zaman aşımı (timeout) süreleri.
2. **Config'den Okunacak Fakat Kodda Sert Alt Limitlerle (Guardrails) Korunacaklar:**
   - Eşikler (`yuz`, `presence`, `liveness`, `min_track_suresi`).
   - Saldırgan config dosyasını manipüle etse bile kapının kolayca açılmasını engellemek amacıyla kod içinde aşılamaz alt limitler (`yuz >= 0.50`, `presence >= 0.40`, `liveness >= 0.50`, `track >= 2.0s`) kilitli tutulmalıdır.
3. **Kod İçinde Sabit Kalması Gerekenler:**
   - `AND` karar mantığının kendisi (`yuz && presence && liveness && track`).
   - Whitelist'in yalnızca insan onayıyla verilebilmesi ilkesi.
   - Hata anında kapının kilitli kalması (`fail-closed`).

---

## 2. Donanım Realitesi: Eski Laptop / GT GPU vs CPU Önerisi

- **Durum:** 1050 serisi öncesi mobil GPU'lar (GT 940M, GT 920M vb.) DDR3 bellek bant genişliği darboğazı ve Compute Capability (3.5/5.0) kısıtı nedeniyle modern PyTorch 2.x ve CUDA 12 ile uyumsuzdur; 7/24 inferansta aşırı ısınma ve çökme riski taşır.
- **Öneri:**
  - Nöbetçi laptopta GPU zorlanmamalı, doğrudan **Intel CPU + ONNX Runtime (AVX2)** optimizasyonu kullanılmalıdır.
  - YOLOv8n ve MobileFaceNet/buffalo_s modelleri CPU üzerinde ~150-250ms içinde stabil çalışır; 12GB sistem RAM'i sayesinde VRAM tükenmesi (OOM) yaşanmaz.
  - Ağır işler (gece özetleri, log analizi) masadaki RTX 3060 "Beyin" makinesine bırakılmalıdır.

---

## 3. Sensör Füzyonu: HLK-LD2410 Radar + RuView (Wi-Fi CSI) Önerisi

- **Durum:** Boşta duran Superbox modem, sisteme özel şifreli bir AP olarak izole bir RF ortamı sağlar. Mevcut kodda `presence.rs` içinde tek bir bellek alanı olduğundan RuView ile Radar birbirinin üzerine yazmakta (Last-Write-Wins çatışması) ve bir sensör diğerini ezebilmektedir.
- **Öneri:**
  - **1. Öncelikli Hakem (Ground Truth):** `HLK-LD2410 Radar`. Yön ve mesafe bildiği için kapı açma kararı doğrudan radara bağlı olmalıdır.
  - **2. Öncelikli / Vitrin Sensörü (Showcase & Pre-Trigger):** `RuView Wi-Fi CSI`. Merdiven boşluğundaki hareketi hissedip laptopu uyandırma (wake-up), loglarda çift doğrulama sağlama ve akademik vitrin görevi üstlenmelidir.
  - Basit `max(radar, ruview)` yerine radar öncelikli hiyerarşik füzyon uygulanmalıdır (fotoğraf saldırılarında sahte RF gürültüsüyle kapının açılmaması için).

---

## 4. Çoklu Kamera Görev Dağılımı Önerisi

- Wi-Fi üzerinden mikrosaniyelik senkronizasyon (hardware genlock) imkansız olduğundan 3 kameranın aynı anda 3D derinlik hesaplaması pratikte mümkün değildir.
- **Öneri:**
  - **Cam0 (Merkez Pi Zero 2W):** Tek Yargıç. Yüz tanıma ve kimlik doğrulama sadece bu kameradan yürütülmelidir.
  - **Cam1 & Cam2 (Yan ESP32-S3):** Anti-Spoof ve Tehdit Gözlemcisi. Yüz tanıma çalıştırmadan, yandan bakışla fotoğraf/ekran düzlemselliğini kontrol etmeli ve arkada pusuya yatan yabancıları (tailgating) izlemelidir.

---

## 5. Mevcut Kod Notu

- `iuds-core/src/main.rs` içindeki `DoorResp` ve `GreetResp` struct'larına `#[derive(Debug)]` eklenmesi, `cargo test` derleme hatasını çözecektir.
