# 03 — Mimari

```
[Kam0 merkez Pi] + [Kam1 sağ XIAO] + [Kam2 sol XIAO] + [LD2410 FMCW radar = ASLI algılayıcı]
  → [ESP32-S3 (C/C++, ESP-IDF): radar mesafe/hız profili → UART + interrupt ile laptopu uyandırır]
  → [Laptop: SafetyNet Core]
    - Rust: ingest, kural motoru, röle + VoiceStudio client, log
    - Python: YOLOv8n + ByteTrack + InsightFace + OSNet + anti-spoof + FER (Faz 3)
    - LLM (local, küçük): sadece karşılama metni + etiketleme. Asla kapı açamaz.
  → [Hoparlör] + [Optokuplörlü röle arayüzü] + [Bildirim]
```

- Presence 7/24 hafif, ağır tanıma hareketle uyanır.
- VRAM 2GB: sıralı çalış — tanı → dur → LLM → dur → TTS.
- Işık: apartman otomatiği, entegrasyon yok. İlk 1 sn kareler atılır (patlama).
- Laptop hep açık + uyku kapalı + UPS önerilir.
- Kayıt local şifreli SQLite, 30 gün.
