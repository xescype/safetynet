# 04 — Donanım ve Gereksinimler

## Mevcut (tahmini, dönünce doğrulanacak)
- Samsung eski laptop, 12GB DDR3
- GPU: GTX 960M sanılıyor (~2GB VRAM, Maxwell)
- Aşağıda otomatik ışık var
- Zil bütçesi min 10 sn

## Yazılım
- OS: Ubuntu 22.04 LTS önerilir
- NVIDIA 550+ + CUDA 12 + cuDNN
- Docker, Python 3.11 + ONNX Runtime GPU, Rust 1.85+, Ollama
- Modeller: YOLOv8n + ByteTrack, InsightFace buffalo_s + anti-spoof, OSNet, Qwen2.5-0.5B Q4 (sonra 1.5B denenebilir), VoiceStudio local API hafif engine

## Alınacak
- Uç birim firmware: ESP32-S3'ler C/C++ (ESP-IDF veya Arduino framework). UART + interrupt kodları rapora konacak. Python/Rust çekirdek aynen korunur (C++'a baştan yazım YOK).
- Cam0 merkez: Pi Zero 2W (~1.500 TL) + Camera Module 3 standart 75° (~1.750 TL)
- Cam1/Cam2 yan: XIAO ESP32S3 Sense (~1.217 TL/adet, mic dahil)
- Radar: HLK-LD2410C (~403 TL)
- Röle arayüzü: diafon kontağına optokuplörlü (galvanik izolasyon) + ters EMK flyback diyotlu paralel tetikleme. Shelly 1 Mini Gen3 (~950-1.200 TL) veya EEE el yapımı ESP32 röle kartı. Şema + izolasyon testi staj raporuna girecek.
- Soğutucu + temizlik, opsiyonel UPS 600VA

Hibrit toplam ~6.500 TL. Full Pi ~11.250 TL. Full ESP ~4.200 TL.

## Dönünce 5 dk kontrol
1. GPU adı (960M mi?)
2. VRAM (GPU-Z)
3. CPU modeli
4. Diafon/kilit foto + kapı + zil foto
5. OS kararı
