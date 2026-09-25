# 07 — LLM, Expression, Lisans, Sonraki

## LLM
Sadece karşılama + etiket değil, DB analisti:
- Sorular: "bu adam kaç kez geldi?", "hep kiminle geliyor?", "gece gelen yabancılar kim?", "şüpheli tekrar var mı?"
- Çalışma: LLM SQL'e dokunmaz, Rust özet tablo verir (kişi, sayı, son 5 ziyaret, birlikte geldikleri). LLM bunu yorumlar + önerir.
- Gece özeti: "bugün 8 ziyaret, 2 yeni yüz, biri 3. gelişi" bildirimi.
- Qwen2.5-0.5B Q4 ile başla. Asla kapı açamaz, sadece önerir.

## Güven seviyeleri (sık gelen → oto-aç fikri)
- L0 yabancı: sor + kaydet.
- L1 tanıdık (3+ geliş, olaysız): ismiyle selamla, yine sormadan açma.
- L2 güvenilir: SADECE ailenin tek tuş onayıyla. Şartlı olabilir (gündüz + bildirimli). Asla sayaçla otomatik terfi yok — kurye de sık gelir, kapı ona açılmaz.
- Kural: terfi önerisi LLM'den, karar insandan. Her terfi loglanır, tek tuşla geri alınır.

Not: şu an prototip, ticari amaç yok. Ortak alana takmadan önce yönetim izni + tabela eklenecek.

## Expression (Faz 3, opsiyonel)
Hafif FER (~10ms/yüz). Karara etki etmez, soft sinyal: stres + arkada yabancı = öncelik yükselt.

## Lisans
- RuView MIT: serbest.
- VoiceStudio AGPL-3.0: evde serbest, satarsan kodu açman gerekir. Modeller upstream lisanslı.

## Sonraki
- [ ] 5 dk kontrol listesi (dönünce)
- [ ] Yönetim izni
- [ ] Mikrofon kararı
- [ ] Faz 1 demo
