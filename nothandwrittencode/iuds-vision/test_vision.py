"""Saf testler (ağırlıksız) + boş-kare dedektör testi. Python 3.14:
  python -m unittest test_vision -v
"""

import unittest

import numpy as np

from face import cosine
from pipeline import build_events


class TestPure(unittest.TestCase):
    def test_cosine(self):
        self.assertAlmostEqual(cosine([1, 0], [1, 0]), 1.0)
        self.assertAlmostEqual(cosine([1, 0], [0, 1]), 0.0)

    def test_kisa_track_elenir(self):
        tracks = [{"duration": 1.2, "whitelisted": True}]
        self.assertEqual(build_events(tracks, lambda tr: ("anne", 0.9)), [])

    def test_eslikli_uretilir(self):
        tracks = [
            {"duration": 4.0, "whitelisted": True},
            {"duration": 5.0, "whitelisted": False},
        ]
        got = build_events(tracks, lambda tr: (None, 0.3) if not tr["whitelisted"] else ("anne", 0.9))
        by = {e["person_id"]: e for e in got}
        self.assertTrue(by[None]["birlikte_mi"])
        self.assertEqual(by[None]["liveness"], 0.0)
        self.assertIsNone(by[None]["presence"])
        self.assertIsNotNone(by[None]["group_id"])

    def test_b2_fam_present_matchten_sonra_hesaplanir(self):
        # B2 regresyonu: gerçek akışta track dict'lerinde whitelisted YOKTUR;
        # match_fn set eder. fam_present match'lerden önce hesaplanırsa
        # birlikte_mi hep False kalırdı (eşlikli giriş ölürdü).
        tracks = [{"duration": 4.0}, {"duration": 5.0}]

        def match_fn(tr):
            tr["whitelisted"] = tr is tracks[0]  # ilk track aile
            return ("anne", 0.9) if tr["whitelisted"] else (None, 0.3)

        got = build_events(tracks, match_fn)
        by = {e["person_id"]: e for e in got}
        self.assertTrue(by[None]["birlikte_mi"])


class _StubTracker:
    def __init__(self, tracks):
        self._tracks = tracks

    def update(self, frame, t=None):
        return [dict(tr) for tr in self._tracks]


class _StubFaces:
    def __init__(self):
        self.calls = 0

    def match(self, crop_img):
        self.calls += 1
        return ("anne", 0.9)


class TestPipelineDedup(unittest.TestCase):
    def _pipe(self):
        from pipeline import Pipeline

        tracks = [{"id": 1, "bbox": [0, 0, 10, 10], "conf": 0.9, "duration": 4.0}]
        faces = _StubFaces()
        # 39998: bağlanılamaz port — core_whitelist hızlı False döner.
        pipe = Pipeline(_StubTracker(tracks), faces, core="http://127.0.0.1:39998")
        return pipe, faces

    def test_b1_ayni_track_cooldown_icinde_tekrar_basmaz(self):
        pipe, _ = self._pipe()
        frame = np.zeros((10, 10, 3), dtype=np.uint8)
        events, _ = pipe.process(frame, t=100.0, post=False)
        self.assertEqual(len(events), 1)
        self.assertEqual(len(pipe._filter_fresh(events, 100.0, mark=True)), 1)
        self.assertEqual(pipe._filter_fresh(events, 110.0, mark=True), [])  # cooldown
        self.assertEqual(len(pipe._filter_fresh(events, 200.0, mark=True)), 1)  # süre geçti

    def test_match_onbellek_kare_basina_insightface_cagirmaz(self):
        pipe, faces = self._pipe()
        frame = np.zeros((10, 10, 3), dtype=np.uint8)
        pipe.process(frame, t=100.0, post=False)
        pipe.process(frame, t=101.0, post=False)
        self.assertEqual(faces.calls, 1)


class TestDetectorBlank(unittest.TestCase):
    def test_bos_karede_kimse_yok(self):
        from detector import Detector

        det = Detector()
        blank = np.zeros((480, 640, 3), dtype=np.uint8)
        self.assertEqual(det.detect(blank), [])


if __name__ == "__main__":
    unittest.main()
