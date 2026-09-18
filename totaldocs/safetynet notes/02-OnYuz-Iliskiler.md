# 02 — ÖnYüz İlişkileri (ney nereye bağlı)

## React panel (iuds-panel) ↔ Rust
| ekran | okur | yazar | DB etkisi |
|---|---|---|---|
| Bugünkü ziyaretler | `GET /api/visits/today` | — | yok (salt okur) |
| Aç butonu | — | `POST /api/door/open` | visits.karar=soruldu_açıldı, onaylayan |
| Red butonu | — | `POST /api/door/deny` | visits.karar=red |
| Kişi kartı | `GET /api/persons/:id` | — | yok |
| Etiketle | — | `POST /api/persons/:id/tag` | persons.isim/tip/host/whitelist |
| Seslendir | — | `POST /api/greet` | command_log=greet (TTS kapalıysa spoken:false, metin yine döner) |

Kural: panel DB'ye dokunamaz, her yazı Rust'tan geçer. Rust token boşsa 400 döner → buton hata gösterir.

## Kotlin app ↔ Rust
Aynı 4 endpoint + FCM push (Rust dışarı çağrı yapar, içeri port açılmaz). Kilit ekranı zil → WebView'da bu panelin telefon hali açılır.

## Vision (Python) ↔ Rust
Python karar vermez: skor + embedding verir → Rust kuralı işletir (AND: yüz VE presence VE liveness VE track) → DB'ye yazar → TTS/push tetikler. Bkz [[00-BUYUK-RESIM]].
