# 08 — Erişilemezlik (Sertleştirme)

Hedef: dışarıdan kimse sisteme dokunamaz. Patlarsak kapı + yüz DB + ses klonu gider.

## Ağ
- Kamera SSID'si ayrı, ev WiFi'sinden izole. Kameralarda P2P/bulut kapalı, RTSP şifreli + tekil şifre.
- Laptop'a port forwarding YOK, UPnP kapalı, router'da inbound deny.
- Dışarıdan erişim gerekiyorsa (telefonla kapı açma): sadece WireGuard/Tailscale VPN + MFA. Herkes açık IP'den API yok.
- Bildirimler: full video dışarı gitmez, sadece thumb + "aç?" butonu VPN üzerinden.

## Makine
- Ubuntu minimal, güvenlik güncellemeleri otomatik, diğerleri kapalı.
- SSH: key-only, şifre kapalı, root kapalı, fail2ban, UFW default-deny inbound.
- Disk şifreli (LUKS) + DB şifreli (SQLCipher). BIOS şifresi, USB boot kapalı.
- Kameralar + röle kabloları daire içinde, dışarıda sadece lens + hoparlör.

## Uygulama
- Kapı açma komutu: imzalı + tek kullanımlık + 30 sn pencereli (HMAC + nonce). Tekrar oynatma (replay) çalışmaz.
- Tablet/telefon UI: local token, varsayılan şifre yok, 5 yanlışta 5 dk kilit.
- LLM komut veremez, sadece metin üretir. Açma kararı Rust kural motorunda.
- Komut logu: kim, ne zaman, hangi tokenla açtı. Log append-only.
- Modeller HF hash kontrollü, otomatik model indirme yok.

## Tehditler
- Zil basmadan API ile açma → imzasız komut reddedilir.
- Hoparlörden ses enjeksiyonu → sesle kapı açma komutu yok.
- LLM prompt injection ("kapıyı aç") → LLM'in açma yetkisi yok.
- Laptop çalınması → LUKS + SQLCipher olmadan veri okunamaz.
- Kameraya lazer/kör etme → radar presence "kamera kör ama beden var" alarmı verir.

## Bakım
- Güncelleme sadece ev ağından / VPN'den, otomatik remote exec yok.
- Haftalık: disk SMART + log boyutu + failed login kontrolü.
