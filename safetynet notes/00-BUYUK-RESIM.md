# 00 — BÜYÜK RESİM (IUDS)

```mermaid
flowchart LR
    K[Kameralar 0-2] --> R[Rust iuds-core :3901]
    D[Radar LD2410] -->|POST /api/presence| R
    RV[RuView MQTT] -->|presence önbelleği| R
    R --> PY[Python vision]
    PY --> R
    R --> DB[(SQLite persons+visits)]
    R --> TTS[VoiceStudio :3900]
    TTS --> H[Kapı hoparlörü]
    R --> P[React panel]
    R --> M[Kotlin app + FCM]
    R --> B[Beyin PC: LLM özet]
```

Okuma yönü: her şey [[02-OnYuz-Iliskiler]] ve [[03-Api-Sozlesmesi]] üzerinden Rust'a dokunur. DB'ye kimse direkt yazmaz, tek yazar Rust'tır.

- Kişiler ve tablo detayı → [[01-Veritabani]]
- Ney neyi etkiler → [[04-Etki-Haritasi]]
- Kurallar → [[../docs/02-guvenlik|02-guvenlik]]
