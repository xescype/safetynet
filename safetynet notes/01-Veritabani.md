# 01 — Veritabanı (persons + visits)

Tek yazar: Rust. Okuyan: panel, app, LLM-özet.

## persons — "kim bu?"
| alan | ne | kim doldurur |
|---|---|---|
| id | `Ziyaretçi-YYYY-AA-GG-001` otomatik | Rust ilk görüşte |
| tip | aile / misafir / bilinmeyen / kara-liste | ev halkı (tag) |
| whitelist | 0/1, kapı otomatiği anahtarı | SADECE insan onayı |
| yüz embedding x3-5 | kimlik vektörleri | Python vision |
| host | kimin misafiri | ev halkı |
| ziyaret_sayısı, son_görülme | sayaçlar | Rust her girişte +1 |

## visits — "ne zaman, ne oldu?"
id, person_id, zaman, kamera, karar (oto_aç/soruldu/red/eşlikli), skorlar, grup_id (birlikte gelenler aynı grup), foto, klip (5sn), onaylayan.

## Saklama
Hot (SSD, 30 gün full) → Cold (1TB HDD: thumb+embedding hep, full klip bayraklıda). Silme yok, taşıma var. Bkz [[04-Etki-Haritasi]] (uzun saklama → KVKK notu).
