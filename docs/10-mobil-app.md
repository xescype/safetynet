# 10 — Mobil App (Android öncelikli)

Karar: native app olacak. Arkaplan zil + kilit ekranı bildirimi web/PWA ile olmaz.

## Platform
- Önce Kotlin Android (Java da olur ama Kotlin modern + coroutines). iOS sonra düşünülür.
- Dağıtım: Play Store şart değil, APK sideload + gerekirse Play.

## Bildirim mimarisi
- Tetik: Rust core → FCM high-priority data message (sadece internet çıkışı, inbound port yok).
- Telefonda Tailscale always-on: bildirime dokununca detay (foto) VPN üzerinden çekilir, full video gitmez.
- Ev içi fallback: internet yoksa evdeki tablet local WebSocket/MQTT ile çalar, kapı çalışmaya devam eder.
- Zil UX'i: normal notification değil, full-screen intent (WhatsApp çağrısı gibi) + özel zil sesi + Aç/Red butonları kilit ekranında.

## Komut güvenliği
- Aç/Red: imzalı + tek kullanımlık + 30 sn pencereli token, VPN üzerinden Rust API'ye. Tekrar oynatma çalışmaz.
- Yanlışlıkla açmaya karşı Aç butonunda basılı tut / kaydırma onayı.

## İzinler
POST_NOTIFICATIONS, USE_FULL_SCREEN_INTENT, FOREGROUND_SERVICE, Tailscale companion.

## API sözleşmesi (Rust tarafı app için açacak)
- `GET /api/visits/today` → bugünkü liste (thumb'lı)
- `POST /api/door/open {visit_id, token}` → imzalı açma
- `POST /api/door/deny {visit_id}`
- `GET /api/persons/{id}` → kişi kartı (ziyaret geçmişi + birlikte gelenler)
- `POST /api/persons/{id}/tag {name, host?, tip?, whitelist?}` → etiketleme
- `POST /api/greet {visit_id?, karar, name?}` → karşılama sesi
