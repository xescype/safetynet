"""IUDS Vision — Ders 3: karar kuralı (mock skorlarla).
Calistirma (18:00 sonrasi):  python decision.py
Gercekte skorlar YOLO + InsightFace + LD2410'dan gelecek; kural ayni kalacak.
Bagimlilik yok (saf Python), her makinede kosar.
"""

# Esikler — docs/02-guvenlik'teki AND kuralinin sayilari.
ESIK_YUZ = 0.65       # altinda: "taniyamadim"
ESIK_PRESENCE = 0.50  # altinda: "beden yok" (fotograf olabilir)
ESIK_LIVENESS = 0.60  # altinda: "canli degil"
MIN_TRACK_SURESI = 3.0  # saniye; altinda: "tek kareye guvenme"


def decide(olay):
    """Karar ver. `olay` bir sozluk (dict):
      yuz, presence, liveness: 0-1 arasi skor
      track_suresi: ayni kisinin kac sn takip edildigi
      whitelist: True/False (aile mi?)
      birlikte_mi: aile + misafir yan yana mi geldi?
    Doner: (karar, neden) ikilisi.
      karar: 'oto_ac' | 'eslikli_ac' | 'sor' | 'reddet'
    """
    yuz = olay["yuz"]
    presence = olay["presence"]
    liveness = olay["liveness"]
    sure = olay["track_suresi"]

    # 1) AND kurali: biri bile tutmazsa red. Süphede fail-closed.
    if presence < ESIK_PRESENCE:
        return ("reddet", "beden yok (fotograf olabilir)")
    if liveness < ESIK_LIVENESS:
        return ("reddet", "canlilik yok")
    if sure < MIN_TRACK_SURESI:
        return ("reddet", "takip cok kisa")
    if yuz < ESIK_YUZ:
        # Eslikli giris istisnasi (Rust decision.rs ile ayni):
        # aileyle yan yana gelen misafir sorgulanmadan alinir.
        if olay.get("birlikte_mi"):
            return ("eslikli_ac", "ailenin misafiri, sorgulama yok")
        return ("sor", "yuz tutmadi, ev halkina sor")

    # 2) Buraya gelen: yuz + presence + liveness + track HEPSI gecti.
    if olay.get("birlikte_mi") and not olay.get("whitelist"):
        return ("eslikli_ac", "ailenin misafiri, sorgulama yok")
    if olay.get("whitelist"):
        return ("oto_ac", "aile, tum katmanlar gecti")
    return ("sor", "tanidik degil, onay gerek")


# Demo senaryolar (18:00 sonrasi calisir, cikti verir).
SENARYOLAR = [
    ("anne geliyor", {"yuz": 0.93, "presence": 0.9, "liveness": 0.85, "track_suresi": 4.2, "whitelist": True, "birlikte_mi": False}),
    ("anne + misafir", {"yuz": 0.40, "presence": 0.9, "liveness": 0.8, "track_suresi": 5.0, "whitelist": False, "birlikte_mi": True}),
    ("elde fotograf", {"yuz": 0.91, "presence": 0.2, "liveness": 0.1, "track_suresi": 4.0, "whitelist": False, "birlikte_mi": False}),
    ("tek kare parlamasi", {"yuz": 0.99, "presence": 0.9, "liveness": 0.8, "track_suresi": 0.4, "whitelist": False, "birlikte_mi": False}),
    ("yabanci tek", {"yuz": 0.41, "presence": 0.85, "liveness": 0.75, "track_suresi": 4.5, "whitelist": False, "birlikte_mi": False}),
]

if __name__ == "__main__":
    for ad, olay in SENARYOLAR:
        karar, neden = decide(olay)
        print(f"{ad:18s} -> {karar:10s} ({neden})")
