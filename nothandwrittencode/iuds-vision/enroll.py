"""Aile yüzü kaydı:  python enroll.py --name anne foto1.jpg foto2.jpg
gallery/anne/ altına kopyalar. En az 3 farklı açı önerilir.
"""

import argparse
import os
import shutil

BASE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "gallery")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--name", required=True)
    ap.add_argument("photos", nargs="+")
    args = ap.parse_args()
    dest = os.path.join(BASE, args.name)
    os.makedirs(dest, exist_ok=True)
    n = 0
    for p in args.photos:
        if not os.path.isfile(p):
            print(f"yok: {p}")
            continue
        shutil.copy(p, os.path.join(dest, os.path.basename(p)))
        n += 1
    print(f"{args.name}: {n} foto -> {dest}")


if __name__ == "__main__":
    main()
