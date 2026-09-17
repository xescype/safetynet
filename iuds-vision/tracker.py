"""ByteTrack sarmalayıcı (ultralytics yerleşik). Aynı model Detector ile paylaşılır.

-> [{id, bbox, conf, duration}] — duration: aynı id ilk görüleli geçen sn.
docs/02: tek kareye güvenilmez, karar icin duration >= 3.0 gerekir.
"""

import time


class Tracker:
    def __init__(self, model, prune_after=30.0):
        self.model = model
        self.born = {}  # id -> ilk görülme (sn)
        self.seen = {}  # id -> son görülme (sn)
        self.prune_after = prune_after

    def update(self, frame, t=None):
        t = time.time() if t is None else t
        res = self.model.track(frame, persist=True, verbose=False, tracker="bytetrack.yaml")[0]
        out = []
        for b in res.boxes or []:
            if b.id is None:
                continue
            tid = int(b.id[0])
            self.born.setdefault(tid, t)
            self.seen[tid] = t
            out.append(
                {
                    "id": tid,
                    "bbox": [float(x) for x in b.xyxy[0].tolist()],
                    "conf": float(b.conf[0]),
                    "duration": t - self.born[tid],
                }
            )
        # Ölü id'leri buda (bellek sızmasın).
        dead = [i for i, s in self.seen.items() if t - s > self.prune_after]
        for i in dead:
            self.born.pop(i, None)
            self.seen.pop(i, None)
        return out
