# SafetyNet (IUDS)

Kod adı: Intruder-Unaware Defence Systems. Klasör adı hâlâ safetynet.

Apartman girişine 3 gizli kamera + bir radar koyup, geleni tanıyan, takip eden, klon sesle karşılayan/uyaran, kapı kararını kendisi veren local-first bir giriş sistemi. Bulut yok, karar hep cihazda verilir. Şu an fikir/prototip aşamasında, fiziksel donanım henüz alınmadı.

## Akış

Biri kapıya yaklaşınca radar (LD2410 FMCW) presence tetikler, laptop uyanır. 0-3sn tanıma (yüz + takip), 3-4sn karar, 4-9sn TTS karşılama, 9-10sn zil + bildirim paralel — toplam 10sn bütçe.

Karar kuralı AND mantığıyla çalışır: presence VAR + yüz eşleşti + canlı + takip en az 3sn tutarlı olmalı. Tek başına yüz hiçbir şey açmaz, foto/telefon ekranıyla kandırılamaz (moiré, kağıt rijitliği, derinlik/rPPG kontrolleriyle). Aile ise hepsi tutarsa kapı otomatik açılır. Aile birlikte bir misafirle geliyorsa (aynı anda varış, yan yana yürüme) sorgusuz açılır, misafir "X'in misafiri" diye loglanır. Kapı açıldıktan birkaç saniye sonra biri arkadan sızarsa tailgating uyarısı verilir. Yabancıda foto+klip kaydedilir, evdekine bildirim gider, tek tuşla açma/red evdekinden gelir.

Aile dışı biri onaylansa bile otomatik whitelist'e girmez — o karar hep insanın.

## Mimari

```
3 kamera (1 merkez Pi + 2 yan XIAO ESP32) + LD2410 radar
  -> ESP32-S3 radar köprüsü (mesafe/hız okur, laptopu UART ile uyandırır)
  -> Laptop (nöbetçi):
       Rust (iuds-core)   : ingest, kural motoru, röle, TTS client, log, API
       Python (iuds-vision): YOLOv8n + ByteTrack + InsightFace + anti-spoof
       LLM (local, küçük)  : sadece karşılama metni + gece özeti üretir, kapı açamaz
  -> Hoparlör + röle (diafona paralel, optokuplörlü) + telefon bildirimi
```

Sistemin iki ayrı makinede çalışması planlanıyor: memlekette kalan eski laptop (960M, 2GB VRAM) "nöbetçi" olarak kamera/kural/kapı kararını verir, internet kesilse de çalışmaya devam eder. Bu makinedeki RTX 3060 ise "beyin" olarak gece özetleri ve ağır analiz için kullanılacak, Tailscale VPN + Syncthing ile bağlanacaklar. Kapı kararı hiçbir zaman internete/beyne bağımlı değil.

RuView (WiFi-CSI presence, MIT lisans) ve VoiceStudio (local ses klonlama, AGPL-3.0) üçüncü parti projeler — koduna dokunulmuyor, sadece MQTT/HTTP üzerinden entegre ediliyor.

## Güvenlik / erişilemezlik

Amaç: dışarıdan kimse sisteme dokunamasın. Kamera ağı ev WiFi'sinden izole ayrı SSID'de, router'da inbound tamamen kapalı, port forwarding yok. Dışarıdan kapı açma sadece VPN (Tailscale/WireGuard) + imzalı tek kullanımlık token (HMAC+nonce, 30sn pencere) ile mümkün — replay saldırısı çalışmaz. Disk LUKS, veritabanı SQLCipher ile şifreli. LLM'in kapı açma yetkisi hiç yok, prompt injection ile "kapıyı aç" dese bile sadece metin üretir, kararı Rust kural motoru verir.

## Klasörler

- `docs/` — 15 parçaya bölünmüş dokümantasyon (fikir/akış, güvenlik kuralı, mimari, donanım, kamera yerleşimi, kayıt şeması, erişilemezlik, iki makineli yapı, mobil app, alış listesi, kod incelemesi, çizimler)
- `SAFETYNET.md` — docs'a giriş indeksi
- `SAFETYNET-FULL.md` — tüm docs'un tek dosyada kopyası (salt okunur, kopyala-yapıştır için)
- `iuds-core/` — Rust çekirdek: ingest, kural motoru, röle + TTS client, log, HTTP API (`/api/visits`, `/api/door/open`, `/api/persons/{id}` vb.)
- `iuds-vision/` — Python: YOLOv8n + ByteTrack + InsightFace ile tespit/takip/yüz eşleştirme, core'a skor POST eder
- `iuds-panel/` — React ziyaret paneli, Rust Axum'un serve ettiği arayüz
- `RuView-main/` — WiFi-CSI presence sensörü (üçüncü parti, MIT)
- `VoiceStudio-main/` — local ses klonlama/TTS (üçüncü parti, AGPL-3.0)
- `tools/llmfit.py` — makinenin RAM/VRAM'ine göre hangi LLM'in sığdığını söyler
- `safetynet notes/` — çalışma notları
- `buggedchatslog/` — hata ayıklama sohbet kayıtları

## Donanım (planlanan, henüz alınmadı)

Pi Zero 2W + Camera Module 3 (merkez kamera), 2x XIAO ESP32S3 Sense (yan kameralar, mikrofonlu), HLK-LD2410C radar, diafona paralel optokuplörlü röle arayüzü. Hibrit kurulum tahmini ~8.500-9.500 TL (UPS hariç). Kurulum yeri: 3 katlı bina girişi, 2x2m alan.

## Durum

Kurulum memlekete dönüşü bekliyor. `docs/14-analiz-ve-oncelikler.md` dosyasında güncel kod incelemesi ve öncelik sırası var — özetle event spam'i (aynı kişi için onlarca ziyaret kaydı açılması) ve eşlikli girişin canlıda çalışmaması en kritik açık işler; whitelist kararının client'tan değil DB'den gelmesi de röle takılmadan önce şart.
