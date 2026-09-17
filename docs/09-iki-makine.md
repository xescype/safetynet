# 09 — İki Makineli Yapı (Nöbetçi + Beyin)

llmfit sonucu (güçlü makine): 16GB RAM + RTX 3060 6GB → 7B Q4'e kadar OK, 14B sığmaz.

## Roller
- NÖBETÇİ (memleket laptopu, 960M): kameralar, presence, YOLO+yüz, kapı kararı, röle. İnternet kesilse de çalışır. Üstünde sadece 0.5B LLM.
- BEYİN (bu makine, 3060): gece özetleri, cross-ref analizi (7B), ağır TTS denemeleri, yedek DB, etiketleme.

## Bağlantı
- Tailscale VPN ile iki makine aynı özel ağda. Port açma yok, her şey WireGuard içinden.
- Syncthing ile DB + klipler tek yön (nöbetçi → beyin) akar. Kapı kararı için beyne gidiş gerekmez, gecikme sorun değil.
- Kural: kapı açma komutu asla internetten gelmez. Beyin sadece önerir ("şunu güvenilire al?"), onay yine evdeki cihazdan / VPN'li telefondan.

## Kesinti kuralı
- İki şehir arası aynı WiFi imkansız, hat = internet + Tailscale. Kesinti normal kabul edilir.
- Kapı kararı kesintiden bağımsız: nöbetçi her zaman local çalışır.
- Syncthing kuyruklar, internet gelince devam eder. Bildirim de öyle: ev içi local çalar, uzaktaki birikeni sonra alır.

## tools/llmfit.py
`python tools/llmfit.py` → RAM/VRAM okur, hangi Qwen/Llama sığar söyler. Yeni makinede tekrar çalıştır.
