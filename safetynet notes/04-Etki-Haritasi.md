# 04 — Etki Haritası (ney neyi etkiler)

- token boş → Rust 400 → panel kırmızı hata, kapı KIPIRDAMAZ (fail-closed).
- trust=L0 yabancı tek → sor + kaydet, oto-aç YOK.
- aile + yabancı BİRLİKTE → soru YOK, aç + "X'in misafiri" logla + bilgi bildirimi.
- yabancı kapı açıldıktan 5sn sonra belirirse → tailgating alarmı.
- whitelist=1 SADECE insan tag'iyle olur → sayaçla otomatik terfi YOK (kurye tuzağı).
- LLM metin üretir, kapı komutu VEREMEZ (yetki Rust'ta).
- internet kesilirse → kapı local çalışır, senkron/bildirim kuyrukta bekler.
- 30 gün dolarsa → silinmez, 1TB'ye taşınır (thumb+embedding hep durur).
- kamera kör + radar "beden var" → "kamera kör" alarmı (lazer/kapatma tespiti).
- Uzun saklama → KVKK: tabela + yönetim izni + tek tuşla silme fonksiyonu şart (deployment öncesi).
