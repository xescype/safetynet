"""Kare -> core olayi. Python karar VERMEZ (notes/02): skor üretir,
Rust AND kuralı işletir. presence'i basmaz (None) — core önbellekten
(RuView/LD2410) alır. liveness=0.0 (Faz 3 yok) — oto_ac bu yüzden
şimdilik çıkmaz, bu bilinçli (fail-closed).

B1: aynı track için olay bir kez basılır (cooldown sonrası tekrar) —
yoksa kapıda duran biri DB'yi ve TTS'i boğar.
B2: whitelist bayrağı match_fn içinde set edilir; birliktelik (fam_present)
match'lerden SONRA hesaplanır — yoksa hep False kalır (eşlikli giriş ölür).
Yüz eşleşme kare başına değil, track başına MATCH_TTL önbellekle koşar.
"""

import time

import requests

CORE = "http://127.0.0.1:3901"
MIN_DURATION = 3.0  # decision.py MIN_TRACK_SURESI ile aynı
POST_COOLDOWN = 60.0  # aynı track'in tekrar olay basma süresi (sn) (B1)
MATCH_TTL = 5.0  # yüz eşleşme önbelleği (sn) — kare başına InsightFace koşmaz


def crop(frame, bbox, pad=0.15):
    h, w = frame.shape[:2]
    x1, y1, x2, y2 = [int(v) for v in bbox]
    dx, dy = int((x2 - x1) * pad), int((y2 - y1) * pad)
    return frame[max(0, y1 - dy) : min(h, y2 + dy), max(0, x1 - dx) : min(w, x2 + dx)]


def core_whitelist(person_id, timeout=2.0):
    """Kişi core'da whitelist mi? Erişilemezse False (şüphede kapalı).
    Not: bu bayrak sadece birliktelik (birlikte_mi) ipucu içindir; kapı
    kararında core client bayrağına güvenmez, DB'den okur (B3)."""
    try:
        r = requests.get(f"{CORE}/api/persons/{person_id}", timeout=timeout)
        if r.ok:
            return bool(r.json()["person"]["whitelist"])
    except Exception:
        pass
    return False


def build_events(tracks, match_fn, t=None):
    """Nitelikli track'lerden core olay gövdeleri. Saf fonksiyon (test edilir).
    B2: match_fn ÖNCE çağrılır (tr'ye whitelisted bayrağını o yazar),
    fam_present SONRA hesaplanır."""
    t = time.time() if t is None else t
    ready = [tr for tr in tracks if tr["duration"] >= MIN_DURATION]
    if not ready:
        return []
    matched = [(tr, *match_fn(tr)) for tr in ready]
    fam_present = any(tr.get("whitelisted") for tr, _, _ in matched)
    gid = f"g-{int(t)}" if len(ready) > 1 else None
    events = []
    for tr, pid, yuz in matched:
        events.append(
            {
                "track_id": tr.get("id"),  # core'a basılmaz, dedup içindir (B1)
                "person_id": pid,
                "yuz": round(float(yuz), 3),
                "presence": None,  # core önbellekten
                "liveness": 0.0,  # Faz 3'e kadar sabit
                "track_suresi": round(float(tr["duration"]), 2),
                "whitelist": bool(tr.get("whitelisted")),
                "birlikte_mi": bool(pid is None and fam_present),
                "group_id": gid,
            }
        )
    return events


class Pipeline:
    def __init__(self, tracker, faces, camera="cam0", core=CORE,
                 cooldown=POST_COOLDOWN, match_ttl=MATCH_TTL):
        self.tracker = tracker
        self.faces = faces
        self.camera = camera
        self.core = core
        self.cooldown = cooldown
        self.match_ttl = match_ttl
        self.posted = {}  # track_id -> son olay basma zamanı (B1)
        self.match_cache = {}  # track_id -> (t, pid, yuz, whitelisted)

    def _filter_fresh(self, events, t, mark):
        """B1: cooldown içindeki track'leri ele. mark=True ise yenilerini işaretle."""
        fresh = []
        for ev in events:
            tid = ev.get("track_id")
            last = self.posted.get(tid)
            if tid is None or last is None or (t - last) >= self.cooldown:
                fresh.append(ev)
                if mark and tid is not None:
                    self.posted[tid] = t
        return fresh

    def process(self, frame, t=None, post=True):
        t = time.time() if t is None else t
        tracks = self.tracker.update(frame, t)

        def match_fn(tr):
            tid = tr.get("id")
            ent = self.match_cache.get(tid) if tid is not None else None
            if ent is None or (t - ent[0]) >= self.match_ttl:
                pid, yuz = self.faces.match(crop(frame, tr["bbox"]))
                wl = core_whitelist(pid) if pid else False
                ent = (t, pid, yuz, wl)
                if tid is not None:
                    self.match_cache[tid] = ent
            _, pid, yuz, wl = ent
            tr["whitelisted"] = wl
            return pid, yuz

        events = build_events(tracks, match_fn, t)
        fresh = self._filter_fresh(events, t, mark=post)
        resps = []
        if post:
            for ev in fresh:
                body = {k: v for k, v in ev.items() if k != "track_id"}
                body["camera"] = self.camera
                try:
                    r = requests.post(f"{self.core}/api/vision/event", json=body, timeout=5)
                    resps.append((ev, r.status_code, r.json() if r.ok else None))
                except Exception as e:
                    resps.append((ev, -1, str(e)))
        return events, resps
