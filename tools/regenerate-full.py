"""SAFETYNET-FULL.md'yi docs/'dan yeniden üretir (salt okunur kopya).

Kullanim:  python tools/regenerate-full.py
Ayrık docs'lar kaynak, FULL sadece birleşimdir; elle düzenlenmez.
"""
import glob
import io
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEADER = """# Intruder-Unaware Defence Systems (IUDS) — Tek Dosya
> Ayrık docs'ların birleşimi (kopyala-yapıştır için). Ayrıklar duruyor, burası salt okunur kopya.

"""


def main():
    docs = sorted(glob.glob(os.path.join(ROOT, "docs", "*.md")))
    parts = [HEADER.rstrip("\n")]
    for path in docs:
        with io.open(path, encoding="utf-8") as f:
            parts.append(f.read().rstrip("\n"))
    out = "\n\n---\n\n".join(parts) + "\n"
    target = os.path.join(ROOT, "SAFETYNET-FULL.md")
    with io.open(target, "w", encoding="utf-8", newline="") as f:
        f.write(out)
    print("yazildi: {} ({} bolum)".format(target, len(docs)))


if __name__ == "__main__":
    main()
