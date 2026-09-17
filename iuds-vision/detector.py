"""YOLOv8n kisi tespiti. CPU iskelet; Faz 2'de cu'lu torch + imgsz ayari."""

from ultralytics import YOLO


class Detector:
    def __init__(self, weights="weights/yolov8n.pt", conf=0.45, imgsz=640):
        self.model = YOLO(weights)
        self.conf = conf
        self.imgsz = imgsz

    def detect(self, frame):
        """-> [{bbox:[x1,y1,x2,y2], conf}] — sadece insan (class 0)."""
        res = self.model.predict(
            frame, conf=self.conf, imgsz=self.imgsz, classes=[0], verbose=False
        )[0]
        out = []
        for b in res.boxes or []:
            out.append({"bbox": [float(x) for x in b.xyxy[0].tolist()], "conf": float(b.conf[0])})
        return out
