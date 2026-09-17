# iuds-panel

IUDS kapı paneli: React + Vite, telefon ölçüsünde tek ekran.

- Bugünkü ziyaretler (`GET /api/visits/today`), Aç/Red (`POST /api/door/...`), kişi kartı + etiketleme, Seslendir (`POST /api/greet`), ses seçimi (`GET /api/voices`).
- Geliştirmede `/api` istekleri `vite.config.js`teki proxy ile `http://127.0.0.1:3901`e (iuds-core) gider.

Komutlar: `npm run dev`, `npm run build`, `npm run lint`, `npm run preview`.
