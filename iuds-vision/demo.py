"""Kamera/kayıt iskelet demosu. Core çalışıyor olmalı (iuds-core cargo run).

Kullanim (Python 3.14):
  python demo.py --source 0            # webcam
  python demo.py --source kayit.mp4
  python demo.py --source foto.jpg --still
  python demo.py --source 0 --dry      # core'a basmadan skor göster
"""

import argparse
import time

import cv2

from detector import Detector
from face import FaceDB
from pipeline import Pipeline
from tracker import Tracker


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", default="0")
    ap.add_argument("--still", action="store_true")
    ap.add_argument("--dry", action="store_true")
    ap.add_argument("--camera", default="cam0")
    args = ap.parse_args()

    det = Detector()
    tracker = Tracker(det.model)
    faces = FaceDB()
    pipe = Pipeline(tracker, faces, camera=args.camera)
    print(f"gallery: {sorted(faces.known)} | dry={args.dry}")

    if args.still:
        # Tek karede track süresi birikmez; aynı kareyi ~4sn döndürüp
        # track'i olgunlaştır, sonra tek olay bas (kayıt kirlenmez).
        frame = cv2.imread(args.source)
        assert frame is not None, "foto okunamadi"
        t0, events = time.time(), []
        while time.time() - t0 < 6.0:
            events, _ = pipe.process(frame, post=False)
            if events:
                break
            time.sleep(0.5)
        if args.dry or not events:
            for e in events:
                print(e)
            return
        _, resps = pipe.process(frame, post=True)
        for ev, code, body in resps:
            print(code, (body or {}).get("karar", ev), "|", ev)
        return

    src = int(args.source) if args.source.isdigit() else args.source
    cap = cv2.VideoCapture(src)
    assert cap.isOpened(), "kaynak acilmadi"
    try:
        while True:
            ok, frame = cap.read()
            if not ok:
                break
            events, resps = pipe.process(frame, post=not args.dry)
            for ev, code, body in resps or [(e, None, None) for e in events]:
                print(time.strftime("%H:%M:%S"), code, (body or {}).get("karar", ev), "|", ev)
            if args.dry:
                time.sleep(0.5)
    finally:
        cap.release()


if __name__ == "__main__":
    main()
