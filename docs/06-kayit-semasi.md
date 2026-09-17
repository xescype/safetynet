# 06 — Kayıt Şeması

Local SQLite + 1TB HDD (evdeki disk). Silme yok, kademeli saklama var.

## persons
id, isim, tip (aile/misafir/bilinmeyen/kara-liste), whitelist, yüz embedding x3-5, body embedding, ses yolu, host (kimin misafiri), ilk/son görülme, not (LLM: "mavi montlu"), ziyaret_sayısı.

- Bilinmeyen: `Ziyaretçi-YYYY-AA-GG-001` otomatik. Tabletten isim/host/kara-liste verilir.
- Tekrar gelen embedding benzerliğiyle aynı id'ye düşer → cross-reference buradan: "bu kişi 3 ay önce X ile gelmiş".
- Aile + onaylı misafir silinmez. Onaysız yabancı da silinmez, pasife düşer.

## visits
id, person_id, zaman, kamera, karar (oto_aç/soruldu/red/eşlikli), skorlar (yüz/liveness/presence), grup_id (birlikte gelenler), foto (crop+thumb), klip (5sn), onaylayan, ifade (Faz 3).

- Eşlikli: bilgi bildirimi. Kara-liste: yüksek öncelik.
- Thumb telefona, full evde kalır.

## Saklama (1TB planı)
- Hot (laptop SSD, 30 gün): full klip + foto, hızlı arama.
- Cold (1TB HDD): 30 günü geçenler buraya taşınır — thumb + embedding + metadata her zaman, full klip sadece bayraklı olaylarda (red / kara-liste / onaylı). Günde 20 ziyaret = yılda ~22GB, 1TB ~40+ yıl yeter.
- Cross-reference: kişi kartında tüm ziyaret zaman çizelgesi + birlikte geldikleri (grup_id). Vektör arama için FAISS/sqlite-vec, 100 bin kayıtta bile <200ms.
- 03:00 job'u silmez, taşır + index optimize eder. HDD SMART kontrolü eklenir.

## Uyarı
Uzun saklama KVKK'da risk: tabela + amaç ("apartman güvenliği") + yönetim izni şart. Silme talebi gelirse tek kişiyi silme fonksiyonu olacak. HDD çalınmaya karşı şifreli (LUKS/SQLCipher), yetkisiz bakmaya karşı erişim logu.
