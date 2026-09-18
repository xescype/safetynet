# 03 — API Sözleşmesi (kaynak: iuds-core/src/main.rs)

| metot | yol | gövde | cevap | hata |
|---|---|---|---|---|
| GET | /api/health | — | `{ok, service}` | — |
| GET | /api/visits/today | — | `[Visit]` | — |
| POST | /api/vision/event | `{person_id?, camera?, yuz, presence, liveness, track_suresi, whitelist?, birlikte_mi?, group_id?}` | `{ok, visit_id, person_id, karar: oto_ac|eslikli_ac|soruldu|red, neden}` | 400 (skor 0-1 dışı) |
| POST | /api/greet | `{visit_id?, karar, name?}` | `{ok, text, spoken, bytes, file?, detail}` | 400 (karar hatalı) |
| POST | /api/door/open | `{visit_id, token}` | `{ok, action:open, visit_id}` | 400 (token/visit boş) |
| POST | /api/door/deny | `{visit_id, token}` | `{ok, action:deny, visit_id}` | 400 |
| GET | /api/persons/{id} | — | `{ok, person, visits[30], companions[30]}` | 404 |
| GET | /api/presence | — | `{score, fresh, source, updated_at}` | — |
| POST | /api/presence | `{score, source?}` | `{score, fresh, source, updated_at}` | 400 (0-1 dışı) |
| GET | /api/voices | — | `{ok, active, voices, detail}` | — (kapalıysa liste boş) |
| POST | /api/voices/select | `{voice}` | `{ok, voice}` | 400 (boş) |
| POST | /api/persons/{id}/tag | `{name, host?, tip?, whitelist?}` | `{ok, id, name, host}` | 400 (ad boş / tip hatalı) |

Değişirse burası + `02-OnYuz-Iliskiler` + panel aynı anda güncellenir. Faz 4'te token → HMAC imzalı olacak, sözleşme aynı kalacak.
