# 13 — Alış Listesi (Satın Alma Planı)

Durum: laptop bazlı çalışma, fiziki materyal yok. Liste 3 aşamada alınır.
Fiyatlar tahmini (docs/04 + piyasa), sipariş öncesi güncellenir.
Kurulum yeri: 3 katlı binanın girişi, 2×2m (alternatif 4×4m) alan.

## Ana parçalar (docs/04'te tanımlı)

| Parça | Görev | ~Fiyat |
|---|---|---|
| Raspberry Pi Zero 2W | Cam0 merkez kamera | ~1.500 TL |
| RPi Camera Module 3 (standart 75°, Wide ALMA) | Merkez lens | ~1.750 TL |
| 2× XIAO ESP32S3 Sense | Cam1/Cam2 yan kameralar (mic dahil) | ~2.450 TL (1.217/adet) |
| HLK-LD2410C | FMCW presence radarı (asıl algılayıcı) | ~400 TL |
| Shelly 1 Mini Gen3 VEYA ESP32+optokuplörlü röle kartı (DIY) | Diafon kontağına paralel tetik | ~950–1.200 TL / ~500 TL |

## Eksik kalemler (docs/04'te yok, şart)

| Parça | Neden şart | ~Fiyat |
|---|---|---|
| Pi Zero kamera adaptör kablosu (22-pin mini → 15-pin) | Cam3 kutusundan çıkan kablo Zero'ya UYMAZ — klasik tuzak | ~100–200 TL |
| microSD 32GB (A2) | Pi Zero diski | ~250–350 TL |
| 5V/2.5A microUSB (Pi) + 2× USB-C 5V (XIAO) besleme | 3 ayrı besleme; telefon şarjıyla idare etme | ~600–900 TL |
| 2× U.FL harici patch anten | docs/05 kuralı: dahili anten duvar/panel arkasında zayıf | ~200–300 TL |
| USB-TTL seri adaptör (CP2102/CH340) | LD2410C'yi ESP32 köprüsüz DİREKT laptopa bağlar → masada presence testi | ~150–250 TL |
| Dupont jumper + mini breadboard | Radar↔ESP32 prototip | ~150–250 TL |
| Hoparlör (USB/3.5mm mini amfi) | Mimari diyagramda var, alış listesinde yoktu | ~300–600 TL |
| Montaj sarfı: mat siyah silikon, 5/8mm matkap ucu, sahte PIR kutusu veya ayna çerçevesi, siyah bant | docs/05 gizli montaj A/B/C | ~200–400 TL |
| 2× USB uzatma / ince kablo (90cm+) | XIAO'lar kapıdan 90cm ötede | ~150–300 TL |

## Radar köprüsü kararı

Mimari "ESP32-S3 radar köprüsü" diyor ama docs/04 listesinde kalem yoktu.
Karar: **3. bir XIAO ESP32S3 (Sense'siz, kamerasız)** ~400–500 TL.
Firmware tek tip kalır. Yan kamera XIAO'larına radar yüklemek YOK (hata ayıklama zorlaşır).

## Sonraya bırakılanlar

- UPS 600VA (~2.000–3.000 TL) → sistem çalışıp kesinti yaşanınca.
- Laptop soğutucu + termal macun (~500–900 TL) → önce sıcaklık ölçülür.
- 1TB HDD → evdeki disk kullanılacak.

## Toplam

- docs/04 hibrit hesabı: ~6.500 TL
- Eksik kalemler + radar köprüsü ile gerçekçi toplam: **~8.500–9.500 TL** (UPS hariç)
- Alternatifler: Full ESP ~4.200 TL (yüz kalitesi düşer, önerilmez), Full Pi ~11.250 TL (gereksiz).

## Satın alma sırası

1. **İlk sipariş (~1.500–2.000 TL):** LD2410C + USB-TTL + jumper.
   Masada presence → `POST /api/presence` uçtan uca test (docs/12 HTTP yolu).
   Kamera tarafı zaten webcam ile `demo.py --source 0` çalışıyor.
2. **İkinci sipariş (~3.000 TL):** 1× XIAO Sense + U.FL anten + Pi Zero 2W + Cam3
   + Zero adaptör kablosu + microSD + beslemeler. Merkez + tek yan masada kurulur.
3. **Son sipariş (~2.500–3.500 TL):** 2. XIAO + röle + hoparlör + montaj sarfı.
   Kapı/diafon fotoğrafı çekilip öyle alınır (docs/04 "dönünce 5 dk kontrol" m.4).
