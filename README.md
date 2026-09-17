# SafetyNet (IUDS)

Kod adı: Intruder-Unaware Defence Systems. Klasör adı hâlâ safetynet.

Apartman girişine 3 gizli kamera + bir radar koyup, geleni tanıyan, takip eden, klon sesle karşılayan/uyaran, kapı kararını kendisi veren local-first bir giriş sistemi. Bulut yok, karar hep cihazda verilir. Şu an fikir/prototip aşamasında, fiziksel donanım henüz alınmadı.

## Nasıl çalışıyor

Kural basit: presence (radar/beden var mı) + yüz + canlılık + takip süresi hepsi tutarsa kapı otomatik açılır. Sadece yüzle hiçbir şey açılmaz, foto/video ile kandırılamaz. Aile birlikte misafirle gelirse sorgusuz açılır ve loglanır; kapı açıldıktan birkaç saniye sonra biri sızarsa tailgating uyarısı verir. Yabancıda foto + bildirim + evdekinin onayı istenir.

## Klasörler

- `docs/` — 15 parçaya bölünmüş dokümantasyon: fikir/akış, güvenlik kuralı, mimari, donanım, kamera yerleşimi, kayıt şeması, erişilemezlik/sertleştirme, iki makineli yapı, mobil app, alış listesi, kod incelemesi, çizimler
- `SAFETYNET.md` — docs'a giriş indeksi
- `SAFETYNET-FULL.md` — tüm docs'un tek dosyada kopyası (salt okunur, kopyala-yapıştır için)
- `iuds-core/` — Rust çekirdek: ingest, kural motoru, röle + TTS client, log
- `iuds-vision/` — Python: YOLOv8n + ByteTrack + InsightFace ile tespit/takip/yüz eşleştirme
- `iuds-panel/` — React ziyaret paneli
- `RuView-main/` — WiFi-CSI presence sensörü (üçüncü parti, MIT, dokunulmuyor)
- `VoiceStudio-main/` — local ses klonlama/TTS (üçüncü parti, AGPL-3.0, dokunulmuyor)
- `tools/llmfit.py` — makinenin RAM/VRAM'ine göre hangi LLM'in sığdığını söyler
- `safetynet notes/` — çalışma notları
- `buggedchatslog/` — hata ayıklama sohbet kayıtları

## Durum

Kurulum memlekete dönüşü bekliyor. `docs/14-analiz-ve-oncelikler.md` dosyasında güncel kod incelemesi ve öncelik sırası var — özetle event spam'i (B1) ve eşlikli girişin canlıda çalışmaması (B2) en kritik açık işler.
