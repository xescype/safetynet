# safetynet

aile apartmanının girişi için gizli, çok açılı bir kapı sistemi.
kapıyı iki şey birlikte açar: radar orada biri olduğunu söyler, kameralar yüzü üç ayrı açıdan tanır.
biri eksikse kapı kapalı kalır.
şüpheli bir şey olursa sistem kendi başına karar vermez: ev sahibinin telefonuna haber verir, karar insanda kalır.

*Natura non facit saltus*
Doğa sıçrama yapmaz.

## nasıl çalışır

- **gözler:** 3 × esp32-cam, kapıyı farklı açılardan görür, kareleri wifi ile gönderir.
- **varlık + kilit:** esp32-wroom-32, hlk-ld2410c mmwave radarı okur ve kilidin rölesini sürer.
- **beyin:** bir dizüstü bilgisayar. yüz eşleştirme ruview ile, karar `iuds-core` ile (rust).
- **kural:** radar varlığı **ve** çok açılı yüz eşleşmesi. veri bayatsa puan sıfırdır (fail-closed).

hepsi aynı yerel ağda çalışır.

## klasörler

| klasör | ne var |
| --- | --- |
| `handwritten/iuds-core` | elle yazdığım rust çekirdeği. asıl kod bu. |
| `nothandwrittencode` | yapay zekâya yazdırılmış referans sürüm. karşılaştırmak için duruyor. |
| `totaldocs` | tasarım notları |
| `tools` | yardımcı araçlar |

## durum

| modül | ne yapar | durum |
| --- | --- | --- |
| `decisions.rs` | karar motoru: eşikler, AND kuralı | bitti, testler geçiyor |
| `db.rs` | sqlite, tek yazar | bitti, testler geçiyor |
| `presence.rs` | radar + mqtt önbelleği | bitti, testler geçiyor |
| `tts.rs` | yerel karşılama sesi | sırada |
| `main.rs` | http sunucusu (axum) | sırada |

## çalıştır

```
cd handwritten/iuds-core
cargo test
```

## gizlilik

kameralar yalnızca aile üyelerinin geçtiği ortak girişe bakar.
görüntü ve yüz verisi yerel ağda işlenir.

## lisans

apache-2.0. bkz. [LICENSE](LICENSE).

---

**in short (english).** a hidden, multi-angle door system for a family building's entrance.
the lock opens only when an mmwave radar confirms presence **and** three esp32 cameras match a face from different angles; stale data scores zero (fail-closed).
anything suspicious alerts the owner's phone instead of acting on its own.
the rust core in `handwritten/iuds-core` is written by hand as a learning project; `nothandwrittencode` keeps an ai-written reference for comparison.
licensed under apache-2.0.
