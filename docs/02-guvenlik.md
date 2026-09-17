# 02 — Güvenlik Prensibi

Öncelik: ailenin güvenliği. Şüphede fail-closed.

## Kural
- Presence tetiklenirse her zaman şart.
- Sadece yüzle hiçbir şeye izin yok.
- Foto / video ile kandırılamaz.

```
KARAR = yüz eşleşti VE beden orada VE canlı VE track tutarlı
```

## Anti-spoof
1. Presence: WiFi CSI kütle + nefes + yaklaşma var mı?
2. Pasif: ekran parlaması / kağıt dokusu / moiré / telefon çerçevesi / tutan parmak.
3. Aktif challenge (kapı açma): "başını sola çevir / göz kırp". Kağıt rigid döner, gerçek kafa non-rigid + paralaks.
4. Süreklilik: aynı Track ID 3-5 sn yürümeli. Tek kare %99 yetmez.
5. Canlı tutan saldırısı (beden + elinde foto): stereo/multi-view derinlik + rPPG nabız + kenarlık tespiti. Düzlemsel = reddet.

## Kapı
- Aile: hepsi geçerse oto-aç, biri eksikse "yüzünü göster" + ev halkına sor.
- Body tek başına açamaz.
- Röle: diafon kontağına paralel, elektrik kesilince kilitli (fail-secure).
- Log: kim, saat, skor, 5 sn klip, kim onayladı.
- Yönetim izni + tabela şart.
