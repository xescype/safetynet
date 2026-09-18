# 01 — Fikir ve Akış

Durum: Fikir aşaması. Kullanıcı memlekette değil, dönünce kurulum.

## Ana fikir
Aile apartmanı girişi: 3 gizli kamera → ev laptopu tanır + takip eder → senin klon sesinle karşılar / kovar. Rust + Python, local-first, bulut yok.

Tuğlalar:
- `RuView-main/` → tespit, tracking, body ReID, presence
- `VoiceStudio-main/` → local voice clone + TTS

## Ön kayıt
Aile: fotoğraflar + presence (yürüyüş/body embedding) + sesler önceden girili.

## Akış
A. Aile ise → yüz + presence + liveness tutarlıysa otomatik aç + karşılama + log.

B. Yabancı ise:
1. Foto + klip, 30 gün saklanır.
2. Telefon/tablete bildirim: foto + saat + skor.
3. Zil evdekilere düşer, röle varsa tek tuşla açma.
4. Onaylanırsa yüz + presence kaydedilir ama whitelist'e otomatik alınmaz.

Zil bütçesi min 10 sn: 0-3 tanıma, 3-4 karar, 4-9 TTS, 9-10 çal + bildirim paralel.

## Eşlikli giriş (annen + misafir)
Birlikte geldilerse (aynı anda varış + yan yana yürüme) → sormadan aç, yabancıyı "X'in misafiri" logla, bilgi bildirimi at, sorgulama yapma.
Sonradan sızma (kapı açıldıktan 5 sn sonra belirme) → tailgating uyarısı.
