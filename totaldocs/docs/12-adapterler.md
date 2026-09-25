# 12 — Üçüncü Parti Adaptörler

RuView ve VoiceStudio'ya dokunmayız, onlar da bize dokunmaz. Aradaki tek yer `iuds-core`.

## RuView → presence (MQTT)

RuView tarafı (`RuView-main/`, MIT):

```sh
cargo run --release -p wifi-densepose-sensing-server --features mqtt -- \
  --source esp32 --mqtt --mqtt-host 127.0.0.1
```

Bizim taraf (`src/presence.rs`): arka plan abonesi, kopunca 5sn arayla dener.

- Env: `IUDS_MQTT` (varsayılan `127.0.0.1:1883`), `IUDS_MQTT_TOPIC` (varsayılan `ruview/#`).
- HA-DISCO state topikleri aynen yenir: `.../presence_score/state` → `87` = 0.87, `.../person_count/state` → `2` = 0.8, `presence` → `ON`/`OFF`.
- JSON da olur: `{"presence_score": 62, "n_persons": 1}`.
- Önbellek 15sn tazedir; bayatsa skor 0 sayılır, kapı açılmaz.
- `GET /api/presence` → o anki skor + tazelik. Panel/debug için.

RuView'da kamera YOLO/ReID yok — WiFi-CSI presence + vitals verir. Yüz/body skorları Faz 2'de Python vision'dan (`iuds-vision/`) gelecek, presence'i bu önbellekten okuyacak.

## iuds-vision → core (Faz 1 iskelet, CANLI)

`iuds-vision/` (Python 3.14, CPU): YOLOv8n tespit + ByteTrack takip + buffalo_s yüz eşleştirme → `POST /api/vision/event`.

```sh
python demo.py --source 0                 # webcam
python demo.py --source foto.jpg --still  # tek foto (track olgunlaşır)
python demo.py --source 0 --dry           # core'a basmadan skor göster
python enroll.py --name anne a.jpg b.jpg  # gallery/anne/ kaydı
python -m unittest test_vision -v
```

Kurallar: Python karar vermez, skor basar. `presence` basılmaz (None) — core önbellekten alır. `liveness` sabit 0.0 (Faz 3 yok) — bu yüzden iskelet `oto_ac` çıkaramaz, yabancı `red`/`soruldu` alır; bu bilinçli. Eşlikli giriş çalışır: whitelist'li track + yabancı track yan yanaysa `birlikte_mi` basılır.

Not: torch CPU (`2.14.0+cu` yok) — YOLOv8n yine de ~100ms/kare. cu'lu torch çıkınca `requirements.txt` güncellenip 3060 GPU'ya geçilecek.

## LD2410 → presence (HTTP, sonra)

ESP32-S3 radar köprüsü kurulunca tek istek yeter, MQTT şart değil:

```sh
curl -X POST http://127.0.0.1:3901/api/presence \
  -H 'Content-Type: application/json' -d '{"score": 0.9, "source": "ld2410"}'
```

Skor 0-1 dışıysa 400. Kaynak adı logda durur.

## VoiceStudio → ses (HTTP)

VoiceStudio tarafı (`VoiceStudio-main/`, AGPL-3.0): local backend `http://127.0.0.1:3900`, OpenAI-uyumlu.

Bizim taraf (`src/tts.rs`):

- `POST /api/greet {visit_id?, karar, name?}` → metin şablonu + TTS. `visit_id` varsa `greetings/<id>.wav` yazar.
- `GET /api/voices` → klon listesi + aktif ses. Backend kapalıysa liste boş, 200.
- `POST /api/voices/select {voice}` → runtime ses değişimi (kalıcı ayar değil, env `IUDS_VOICE` ilk değer).
- Env: `IUDS_TTS_URL` (varsayılan `http://127.0.0.1:3900`), `IUDS_VOICE` (varsayılan `alloy`; klon id gelince buraya).
- TTS kapalıysa `spoken:false` + gerekçe döner, kapı akışı aynen devam eder. TTS asla kapı açmaz, açamaz.
