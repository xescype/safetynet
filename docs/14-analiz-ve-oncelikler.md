# 14 — Analiz ve Öncelikler (Kod İncelemesi)

Durum: 2026-09 kod incelemesi. Kapsam: iuds-core (Rust), iuds-vision (Python),
iuds-panel (React), docs + notes. Bu dosya takip dokümanıdır; düzelen işaretlenir.

## Gerçek bug'lar (testler yeşilken bile bozuk)

- [ ] **B1 — Event spam'i (en kritik).** `pipeline.py build_events` her karede
  `duration >= 3sn` olan TÜM track'ler için olay üretir, `process` hepsini POST'lar.
  "Bu track yollandı" hafızası yok. Kapıda 30sn duran biri → onlarca visit satırı +
  her biri için ayrı `Ziyaretçi-*` kişisi + TTS spam + `visit_count` şişmesi
  (3+ → sahte L1 rozeti). İlk canlı kamera testinde DB dolar.
- [ ] **B2 — Eşlikli giriş canlıda ölü.** `build_events`'te `fam_present` döngüden
  ÖNCE hesaplanıyor; `whitelisted` bayrağı ise `match_fn` içinde, döngü SIRASINDA
  set ediliyor. Tracker her karede yeni dict ürettiği için `tr.get("whitelisted")`
  hep None → `fam_present` hep False → `birlikte_mi` hep False.
  `test_eslikli_uretilir` hazır dict'le geçtiği için yakalayamaz.
- [ ] **B3 — Whitelist client'tan geliyor.** `vision_event` kararı `req.whitelist`'e
  güvenerek veriyor. Core elindeki DB'den kendisi bakabilir, bakmıyor.
  "Whitelist SADECE insan onayıyla" ilkesi API seviyesinde delik.
  127.0.0.1'de risk düşük, bind değişince kritik.
- [ ] **B4 — door_open ziyareti güncellemiyor.** notes/02: "Aç → visits.karar=
  soruldu_açıldı + onaylayan". Kod sadece command_log'a yazıyor; panelde Aç'a
  basınca listede "soruldu" kalır. Ayrıca "soruldu_açıldı" API sözleşmesindeki
  karar kümesinde yok — sözleşme kendisiyle çelişiyor.
- [ ] **B5 — Path traversal.** `POST /api/greet` `visit_id`'yi sanitize etmeden
  `greetings/{visit_id}.wav` yazar. `../../x` ile klasör dışına yazılır.
- [ ] **B6 — İki kopya eşik.** `decision.rs` ↔ `decision.py` aynı kuralın iki
  yazımı. Testler kilitliyor ama drift riski; tek kaynak düşünülmeli.

## Doküman ↔ kod uyuşmazlıkları (henüz kod yok)

| Vaat (kaynak) | Durum |
|---|---|
| Liveness/anti-spoof (docs/02, Faz 3) | Yok — `liveness: 0.0` sabit, her olay `red` (bilinçli) |
| Tailgating 5sn kuralı (docs/01) | Yok |
| FCM push + Kotlin app (docs/10) | Yok |
| Foto/klip kaydı (docs/06) | DB şemasında kolon bile yok; `approved_by` var ama yazılmıyor |
| Cold storage / 03:00 job (docs/06) | Yok |
| LLM gece özeti / DB analisti (docs/07) | Yok — şablon metin var |
| Yabancı ReID "3 ay önce gelmişti" (docs/06) | Yok — her bilinmeyen olay YENİ kişi açar |
| HMAC imzalı kapı tokenı (docs/08, Faz 4) | Mock: boş olmayan herhangi bir string yeter |
| Röle sürme | `door_open` sadece log yazar |
| Multi-kamera füzyonu | `camera` alanı sadece etiket |
| "Kamera kör ama beden var" alarmı (docs/08) | Yok |

## Sağlam olanlar (dokunma)

- Karar motoru (AND kuralı, validate, fail-closed) — temiz ve testli.
- Presence önbelleği (tazelik 15sn, MQTT reconnect, esnek parser).
- TTS degrade davranışı (kapalıysa spoken:false, akış durmaz).
- SQLite tek-yazar + mutex — race yok, ID üretimi seri.
- Panel ↔ API sözleşmesi uyumlu; Türkçe id'ler encodeURIComponent ile doğru.
- RuView (mqtt feature) ve VoiceStudio (/v1/audio/speech, /v1/audio/voices)
  entegrasyon hedefleri karşı tarafta doğrulandı.

## Öncelik sırası

1. B1 dedup — ilk canlı testi boğar.
2. B2 fam_present — eşlikli girişi canlandırır.
3. B3 whitelist DB'den — röle takılmadan ÖNCE şart.
4. B4 door_open visit güncelleme + karar kümesine "soruldu_acildi" ekle YA DA nottan sil.
5. Bind değiştirmeden önce auth middleware (Faz 4 HMAC'e kadar tek token yeter).
6. B5 greet sanitize.
7. Yabancı ReID — docs/06'nın ana vaadi, DB şişmesini de keser.
8. DB şemasına foto/klip/ifade + migration (PRAGMA user_version).
9. Liveness (Faz 3) — olmadan sistem sadece `red` üretir; %95 aile tanıma hedefi buna bağlı.

## Performans notu

CPU'da InsightFace ~300–600ms/yüz + kare başına HTTP whitelist sorgusu +
3sn min track → 10sn zil bütçesi CPU'da sıkışır. Çare: cu'lu torch + 3060/960M
geçişi (requirements.txt hâlâ CPU torch) + whitelist sorgusunu kare başına değil
track başına cache'leme (B1 ile birlikte düzelir).
