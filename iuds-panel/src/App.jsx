// IUDS Panel — Ders 2b: telefon UI (shadcn tarzı, paketsiz).
// Kilit ekranı zili → Kotlin app bu sayfayı WebView'da açacak.

import { useEffect, useState } from 'react'
import { Button } from './components/ui/button.jsx'
import { Card, TrustBadge } from './components/ui/card.jsx'
import PersonCard from './components/PersonCard.jsx'

const TOKEN = 'ders2-mock-token' // Faz 4'te imzalı token.

async function api(path, opts) {
  const r = await fetch(path, opts)
  if (!r.ok) throw new Error(path + ' -> ' + r.status)
  return r.json()
}

export default function App() {
  const [visits, setVisits] = useState([])
  const [err, setErr] = useState('')
  const [msg, setMsg] = useState('')
  const [selected, setSelected] = useState(null)
  const [voices, setVoices] = useState([])
  const [voice, setVoice] = useState('')

  const load = () => {
    setErr('')
    api('/api/visits/today')
      .then(setVisits)
      .catch((e) => setErr('API erişmiyor: iuds-core çalışıyor mu? (' + e.message + ')'))
  }

  const loadVoices = () => {
    api('/api/voices')
      .then((d) => {
        setVoices(d.voices || [])
        setVoice(d.active || '')
      })
      .catch(() => {})
  }

  useEffect(load, [])
  useEffect(loadVoices, [])

  const pickVoice = (v) => {
    setVoice(v)
    if (!v) return
    api('/api/voices/select', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ voice: v }),
    })
      .then((d) => setMsg('Ses: ' + d.voice))
      .catch((e) => setErr('Ses seçimi red: ' + e.message))
  }

  const door = (action, visit_id) => {
    setMsg('')
    api('/api/door/' + action, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ visit_id, token: TOKEN }),
    })
      .then((d) => setMsg(d.action + ' ✓ (' + d.visit_id + ')'))
      .catch((e) => setErr('Komut red: ' + e.message))
  }

  const speak = (v) => {
    setMsg('')
    api('/api/greet', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ visit_id: v.id, karar: v.karar, name: v.person }),
    })
      .then((d) => setMsg(d.spoken ? 'Seslendirildi ✓' : 'TTS kapalı, metin hazır: ' + d.text))
      .catch((e) => setErr('Ses red: ' + e.message))
  }

  return (
    <div className="min-h-screen bg-zinc-100 flex justify-center">
      {/* telefon çerçevesi */}
      <main className="w-full max-w-sm bg-white min-h-screen shadow-xl">
        {/* durum çubuğu */}
        <div className="flex justify-between items-center px-5 pt-4 text-xs text-zinc-500">
          <span>IUDS Kapı</span>
          <span>● canlı</span>
        </div>
        <h1 className="px-5 pt-2 text-2xl font-bold">Bugünkü Ziyaretler</h1>

        <div className="flex gap-2 px-5 py-3">
          <Button variant="ghost" size="sm" onClick={load}>Yenile</Button>
          <select
            className="h-9 flex-1 rounded-xl border border-zinc-200 px-2 text-sm"
            value={voice}
            onChange={(e) => pickVoice(e.target.value)}
            title="Karşılama sesi (VoiceStudio klonu)"
          >
            <option value="">Ses: varsayılan</option>
            {voices.map((v) => (
              <option key={v.id} value={v.id}>{v.name}</option>
            ))}
          </select>
        </div>

        {err && <p className="mx-5 mb-2 rounded-xl bg-red-50 p-3 text-sm text-red-700">{err}</p>}
        {msg && <p className="mx-5 mb-2 rounded-xl bg-emerald-50 p-3 text-sm text-emerald-700">{msg}</p>}

        <ul className="space-y-3 px-5 pb-8">
          {visits.map((v) => (
            <li key={v.id}>
              <Card className="p-4">
                <div className="flex items-center justify-between">
                  <button className="font-bold underline" onClick={() => setSelected(v.person)}>
                    {v.person}
                  </button>
                  <TrustBadge level={v.trust} />
                </div>
                <p className="mt-1 text-sm text-zinc-500">skor {v.score} · {v.time}</p>
                <div className="mt-3 grid grid-cols-2 gap-2">
                  <Button variant="open" onClick={() => door('open', v.id)}>Aç</Button>
                  <Button variant="deny" onClick={() => door('deny', v.id)}>Reddet</Button>
                </div>
                <div className="mt-2">
                  <Button variant="ghost" size="sm" className="w-full" onClick={() => speak(v)}>Seslendir</Button>
                </div>
              </Card>
            </li>
          ))}
        </ul>
        {visits.length === 0 && !err && <p className="px-5 text-zinc-400">Kayıt yok.</p>}
        {selected && (
          <PersonCard
            personId={selected}
            onClose={() => setSelected(null)}
            onSelect={setSelected}
            onTagged={load}
          />
        )}
      </main>
    </div>
  )
}
