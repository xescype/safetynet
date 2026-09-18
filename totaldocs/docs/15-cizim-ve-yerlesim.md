# 15 — Çizim ve Yerleşim

Durum: çizimler 3 hedefe gider — staj raporu, kendi kurulum kılavuzu, PCB (opsiyonel).
Kurulum yeri: 3 katlı binanın girişi, 2×2m (alternatif 4×4m) alan.

## Araç haritası (neyi nerede çiz)

| Katman | Ne | Araç | Not |
|---|---|---|---|
| Sistem | Blok, ağ, akış, zamanlama, AND kuralı | **Mermaid** (bu dosyada, docs'ta yaşar) | Rapor için PNG/SVG export |
| Yerleşim keşfi | 2×2/4×4 alana sürükle-bırak obje koyma | **Sweet Home 3D** (ücretsiz, offline) | "Şuraya koysam ne görür" denemeleri; 3D önizleme |
| Yerleşim 2D | Ölçekli plan + görüş konileri | **draw.io desktop** (grid 10cm) | Hızlı varyant karşılaştırma |
| Yerleşim rapor | Ölçekli mühendislik planı | **FreeCAD TechDraw** | Staj raporuna girecek kesin plan |
| Mekanik | Sahte PIR kutusu, lens yuvası → STL | **FreeCAD** | 3D baskıya hazır |
| Elektrik şema | Röle arayüzü, LD2410C↔ESP32-S3 köprü | **KiCad** | docs/04: "şema rapora girecek" |
| Analog doğrulama | Flyback diyot transient simülasyonu | **LTspice** | Rapora "simülasyonla doğrulama" |

Çizim sırası: Sweet Home 3D keşif → Mermaid set (aşağıda) → KiCad şemalar →
LTspice sim → FreeCAD TechDraw kesin plan (kapı ölçüleri gelince).

## Yerleşim rehberi (2×2 / 4×4 alan)

- Referans: kapı merkez (0,0). Cam0 kapıya direkt, yüz hizası (~1.5m), 30–50cm geride.
- Cam1 sağ: kapıdan 90cm ileride. Cam2 sol: simetrik. (docs/05)
- Görüş konileri: Cam3 standart **75°**; XIAO Sense (OV3660) **~65–70°** (doğrulanacak).
  Planda üçgen/sektör olarak çiz, kesişim = kapsanan alan.
- LD2410C: ±60° koni, ~5m. Panel arkasına (plastik/ahşap geçirir, metal GEÇİRMEZ).
- Hoparlör: kapı üstü iç hat, kablo daire içinden.
- Sweet Home 3D yöntemi: duvarları çek (2×2 veya 4×4 oda), kapı ekle,
  objeleri kamera/radar/hoparlör olarak yerleştir, 3D gezintiyle kontrol et.
- draw.io yöntemi: sayfa ölçeği 1cm=10cm, grid 10cm; duvar + kapı + koniler;
  her varyant ayrı katman (layer) → A/B karşılaştırma.

## Mermaid diyagram seti

### Sistem blok

```mermaid
flowchart LR
    subgraph Uc["Uç birimler"]
        C0[Cam0 Pi Zero 2W + Cam3]
        C1[Cam1 XIAO Sense]
        C2[Cam2 XIAO Sense]
        R[LD2410C radar]
        E[ESP32-S3 radar köprüsü]
    end
    subgraph Laptop["Nöbetçi laptop"]
        RS[Rust iuds-core :3901]
        PY[Python iuds-vision]
        DB[(SQLite)]
    end
    C0 & C1 & C2 -->|snapshot JPEG POST| PY
    R -->|UART| E -->|POST /api/presence| RS
    PY -->|skor POST /api/vision/event| RS
    RS <--> DB
    RS -->|TTS| VS[VoiceStudio :3900] --> H[Hoparlör]
    RS --> RL[Röle → diafon]
    RS --> P[React panel]
    RS --> M[Kotlin app FCM]
    RS -.Syncthing/Tailscale.-> B[Beyin PC 3060: LLM özet]
```

### Ağ topolojisi

```mermaid
flowchart TB
    subgraph Ev["Ev ağı"]
        subgraph Izole["Kamera SSID (izole)"]
            C0[Cam0] & C1[Cam1] & C2[Cam2] & E[ESP32 köprü]
        end
        L[Nöbetçi laptop\nport forward YOK, UPnP kapalı]
        T[Ev tableti\nlocal WS/MQTT fallback]
    end
    Izole --> L
    L --- T
    L -->|sadece outbound| FCM[FCM push]
    FCM --> Tel[Telefon]
    Tel -->|Tailscale VPN| L
    L -.Syncthing tek yön.-> B[Beyin PC]
```

### Zil akışı (10 sn bütçe)

```mermaid
sequenceDiagram
    participant Z as Ziyaretçi
    participant R as Radar/ESP32
    participant V as Vision (Python)
    participant C as Core (Rust)
    participant T as TTS/Hoparlör
    participant Tel as Telefon
    R->>C: presence (beden var)
    Note over Z,V: 0–3 sn: tanıma
    V->>C: skorlar (yüz, track, liveness)
    Note over C: 3–4 sn: AND kuralı
    C->>C: karar (oto_ac/eslikli/soruldu/red)
    Note over C,T: 4–9 sn: TTS
    C->>T: karşılama metni
    par 9–10 sn paralel
        C->>Tel: bildirim (foto + Aç/Red)
    end
```

### AND kuralı (docs/02)

```mermaid
flowchart TD
    S[Olay] --> P{presence ≥ 0.50?}
    P -- hayır --> R1[red: beden yok]
    P -- evet --> L{liveness ≥ 0.60?}
    L -- hayır --> R2[red: canlılık yok]
    L -- evet --> T{track ≥ 3sn?}
    T -- hayır --> R3[red: takip kısa]
    T -- evet --> Y{yüz ≥ 0.65?}
    Y -- hayır --> B{birlikte_mi?}
    B -- evet --> E[eslikli_ac]
    B -- hayır --> S1[soruldu]
    Y -- evet --> W{whitelist?}
    W -- evet --> O[oto_ac]
    W -- hayır --> B2{birlikte_mi?}
    B2 -- evet --> E
    B2 -- hayır --> S1
```

## KiCad şema listesi (donanımsız çizilebilir)

1. Röle arayüzü: ESP32 GPIO → optokuplör (PC817) → röle bobini → diafon kontağına
   paralel; flyback diyot (1N4007) bobin uçlarına; galvanik izolasyon notu.
2. Radar köprüsü: LD2410C UART (TX/RX, 5V) → ESP32-S3; OUT pini interrupt.
3. XIAO besleme + U.FL anten notu (şema değil, bağlantı notu).

## LTspice simülasyonu (rapora doğrulama)

- Röle bobini kapatma anı: diyotsuz vs 1N4007'li transient karşılaştırması.
- Optokuplör LED akımı: GPIO 3.3V → seri direnç hesabı (CTR sınırı).
