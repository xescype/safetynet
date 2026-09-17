"""Yüz eşleştirme: InsightFace buffalo_s + gallery/ klasörü.

gallery/<person_id>/*.jpg → ortalama embedding. match(crop) -> (id|None, skor).
Eşik altı = (None, en iyi skor) — core bunu "sor" diye loglar.
Liveness YOK (Faz 3); pipeline liveness=0.0 basar, karar fail-closed kalır.
"""

import glob
import os

import numpy as np

THR = 0.45  # buffalo_s kosinüs eşiği (ailede düşük tut, şüphede core reddeder)


def cosine(a, b):
    a = np.asarray(a, dtype=np.float64)
    b = np.asarray(b, dtype=np.float64)
    n = np.linalg.norm(a) * np.linalg.norm(b)
    return float(a @ b / n) if n > 0 else 0.0


class FaceDB:
    def __init__(self, gallery="gallery", thr=THR, app=None):
        from insightface.app import FaceAnalysis

        self.thr = thr
        self.app = app or self._default_app()
        self.known = {}  # person_id -> embedding
        gdir = os.path.join(os.path.dirname(os.path.abspath(__file__)), gallery)
        for person in sorted(os.listdir(gdir)) if os.path.isdir(gdir) else []:
            embs = []
            for img in glob.glob(os.path.join(gdir, person, "*.jpg")):
                try:
                    import cv2

                    faces = self.app.get(cv2.imread(img))
                except Exception:
                    continue
                if faces:
                    embs.append(faces[0].normed_embedding)
            if embs:
                self.known[person] = np.mean(embs, axis=0)

    @staticmethod
    def _default_app():
        from insightface.app import FaceAnalysis

        app = FaceAnalysis(name="buffalo_s")
        app.prepare(ctx_id=-1)  # CPU; Faz 2'de GPU id
        return app

    def match(self, crop):
        try:
            faces = self.app.get(crop)
        except Exception:
            return None, 0.0
        if not faces:
            return None, 0.0
        emb = faces[0].normed_embedding
        best, best_id = 0.0, None
        for pid, ref in self.known.items():
            s = cosine(emb, ref)
            if s > best:
                best, best_id = s, pid
        if best_id is not None and best >= self.thr:
            return best_id, best
        return None, best
