# Staj Projesi Önerisi — Intruder-Unaware Defence Systems (IUDS)

> Komisyon başlığı (Düzce EEE): "FMCW Radar ve Çoklu Görsel Sensör Füzyonuna Dayalı, Donanım İzolasyonlu Uç Birim (Edge) Akıllı Erişim Denetleyicisi"

> 1 sayfa. Şirket mentörüne + okula verilir. Köşeli parantezleri doldur.

**Öğrenci:** [Ad Soyad] — EEE [sınıf], [Üniversite]
**Dönem:** [gg.aa.yyyy – gg.aa.yyyy], [hafta sayısı] hafta
**Yer:** [Şirket adı] — Danışman: [Mentör adı]

## 1. Problem
Apartman girişlerinde tanımadık girişi ve kuyruklanma (tailgating) riski var. Klasik kameralar sadece kaydeder, tepki vermez. Aile bireyleri misafirle gelince sürtüşmesiz giriş ister, yabancıda ise onay gerekir.

## 2. Amaç
3 gizli kamera + edge laptop ile geleni tanıyan, takip eden, sesli karşılayan/uyaran ve kapı kontrolüne onay üreten local-first bir giriş güvenlik sistemi prototipi. Bulut yok, tüm kararlar cihazda.

## 3. Kapsam (stajda bitecek)
- Donanım/gömülü: 1x Pi Zero 2W + Camera Module 3 (merkez) + 2x XIAO ESP32S3 Sense (yanlar, mic'li), LD2410 presence radarı, kapı röle arayüzü (şema + test).
- Edge yazılım: Rust ingest + kural motoru (fail-closed), Python vision (YOLOv8n + ByteTrack + InsightFace + anti-spoof), SQLite kişi/ziyaret DB.
- Arayüzler: React ziyaret paneli (Rust Axum serve eder), Kotlin Android app'te FCM push + kilit ekranı zil + imzalı Aç/Red.
- Ses: kullanıcının klon sesiyle karşılama (local TTS), 10 sn zil bütçesi içinde.
- Dokümantasyon: mimari, test tablosu, kurulum kılavuzu (bu repo `docs/`).

## 4. Teslimatlar (haftalık demo)
1. Hafta: Rust API sözleşmesi (`/visits`, `/door/open|deny`, `/persons/tag`) + mock'lu React panel.
2. Hafta: Python vision tek kamerayla yüz + track + liveness iskeleti, metrikli test (10 senaryo).
3. Hafta: 3 kamera + radar füzyonu, eşlikli/tailgating kuralı, DB + 1TB soğuk saklama.
4. Hafta: TTS karşılama + Android FCM zil + imzalı komut, gece özeti (LLM).
5. Hafta: Uçtan uca demo + saldırı testleri (foto/telefon/maske) + staj raporu.

## 5. Başarı kriterleri
- Aile gündüz %95+ doğru tanınıp bekletilmeden geçer.
- Foto/telefonla 10 sahte denemede 0 kapı açılışı.
- Zil → sesli tepki ≤10 sn (p50).
- İnternet kesikken kapı kararı local çalışır.

## 6. Kaynaklar
Edge: mevcut laptop (12GB, GTX 960M), 1TB HDD. Beyin: RTX 3060 makine (analiz). Donanım bütçesi ~6.500 TL (hibrit). Lisanslar: MIT + AGPL (ev/prototip kullanımı).

## 7. Risk + önlem
Eski laptop ısınması → sıralı pipeline + soğutucu. WiFi takılması → ayrı SSID + snapshot taktiği. Yanlış tanıma → ilk hafta sessiz mod (sadece kaydet). İzin/tabela deployment öncesi alınır.
