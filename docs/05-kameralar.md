# 05 — Kameralar ve Gizli Montaj

Şart: hepsi gizli olacak, normal kamera ağırlığı taşınamaz. Alan 2x2m.

- Cam0 merkez: kapıya direkt, yüz hizası. Pi Zero 30-50cm geride, sadece lens 8mm delikten bakar.
- Cam1 sağ: kapıdan 90cm ilerde, XIAO Sense.
- Cam2 sol: kapıdan 90cm ilerde, XIAO Sense.

Merkez adayları (karar verilecek):
- A) Ayna çerçevesi üst kenarı: 8mm delik + siyah silikon
- B) Zil / isimlik paneli içi: 5mm delik
- C) Sahte PIR kutusu: doğal durur

Üçünde de LED kapalı, mat siyah, kablo arkadan.

Not: Module 3 standart 75° — 2m'de ~2.7m görür, 2x2 sığar. Wide alma.

## XIAO kuralları (kilitlenmemesi için)
1. Sürekli RTSP YOK (ısınır, WiFi düşer). Boşta 1-2fps düşük çözünürlük veya sadece radar dinleme. Radar tetikleyince tek kare full snapshot JPEG → laptopa HTTP POST/WebSocket.
2. Anten: dahili seramik anten duvar/metal panel arkasında zayıf kalır → U.FL konnektöre harici yapışkan patch anten, kasa arkasına.
3. Bant: 2 XIAO aynı anda basmaz, tetikleme radara bağlı senkron kare. 2.4GHz tıkanmaz.
Rapor başlığı: "dağıtık mikrodenetleyici mimarisi (ESP32-S3 DMA kamera arayüzü + UART telemetrisi)".
