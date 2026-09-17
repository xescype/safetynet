// IUDS Panel — Kişi kartı: detay + zaman çizelgesi + birlikte gelenler + etiketleme.
// Salt okur kuralı: DB'ye direkt yok, her yazı POST /api/persons/:id/tag üzerinden.

import { useEffect, useState } from 'react'
import { Button } from './ui/button.jsx'
import { Card, TrustBadge } from './ui/card.jsx'

async function api(path, opts) {
  const r = await fetch(path, opts)
  if (!r.ok) throw new Error(path + ' -> ' + r.status)
  return r.json()
}

const TIPS = ['aile', 'misafir', 'bilinmeyen', 'kara-liste']

export default function PersonCard({ personId, onClose, onSelect, onTagged }) {
  const [data, setData] = useState(null)
  const [err, setErr] = useState('')
  const [form, setForm] = useState({ name: '', host: '', tip: 'bilinmeyen', whitelist: false })
  const [saving, setSaving] = useState(false)
  const [saveMsg, setSaveMsg] = useState('')

  const load = () => {
    setErr('')
    setData(null)
    api('/api/persons/' + encodeURIComponent(personId))
      .then((d) => {
        setData(d)
        setForm({
          name: d.person.name || '',
          host: d.person.host || '',
          tip: d.person.tip || 'bilinmeyen',
          whitelist: !!d.person.whitelist,
        })
      })
      .catch(() => setErr('Kişi bulunamadı.'))
  }

  useEffect(load, [personId])

  const save = () => {
    if (!form.name.trim()) {
      setErr('İsim boş olamaz.')
      return
    }
    setSaving(true)
    setErr('')
    setSaveMsg('')
    api('/api/persons/' + encodeURIComponent(personId) + '/tag', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        name: form.name.trim(),
        host: form.host.trim() || null,
        tip: form.tip,
        whitelist: form.whitelist,
      }),
    })
      .then(() => {
        setSaveMsg('Kaydedildi ✓')
        if (onTagged) onTagged()
        load()
      })
      .catch((e) => setErr('Kaydetme red: ' + e.message))
      .finally(() => setSaving(false))
  }

  return (
    <div className="fixed inset-0 z-10 flex items-end justify-center bg-black/40" onClick={onClose}>
      <section
        className="w-full max-w-sm max-h-[85%] overflow-y-auto rounded-t-3xl bg-white p-5 shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="mb-1 flex items-center justify-between">
          <b className="text-lg">Kişi kartı</b>
          <Button variant="ghost" size="sm" onClick={onClose}>Kapat</Button>
        </div>
        {err && <p className="mb-2 rounded-xl bg-red-50 p-3 text-sm text-red-700">{err}</p>}
        {!data && !err && <p className="text-sm text-zinc-400">Yükleniyor…</p>}

        {data && (
          <>
            <Card className="p-4">
              <div className="flex items-center justify-between gap-2">
                <b className="break-all">{data.person.name}</b>
                <TrustBadge level={data.person.trust} />
              </div>
              <p className="mt-1 break-all text-xs text-zinc-500">{data.person.id}</p>
              <dl className="mt-2 grid grid-cols-2 gap-1 text-sm">
                <div><dt className="text-xs text-zinc-400">tip</dt><dd>{data.person.tip}</dd></div>
                <div><dt className="text-xs text-zinc-400">ziyaret</dt><dd>{data.person.visit_count} kez</dd></div>
                <div><dt className="text-xs text-zinc-400">ilk</dt><dd>{data.person.first_seen}</dd></div>
                <div><dt className="text-xs text-zinc-400">son</dt><dd>{data.person.last_seen}</dd></div>
              </dl>
              {data.person.host && <p className="mt-1 text-sm">ev sahibi: <b>{data.person.host}</b></p>}
              {data.person.whitelist
                ? <p className="mt-2 rounded-xl bg-emerald-50 p-2 text-xs text-emerald-700">whitelist açık — oto-aç adayı (karar yine AND kuralında).</p>
                : <p className="mt-2 rounded-xl bg-amber-50 p-2 text-xs text-amber-800">whitelist kapalı — sormadan açılmaz.</p>}
            </Card>

            <h3 className="mb-1 mt-4 font-semibold">Etiketle</h3>
            <div className="space-y-2">
              <input
                className="h-11 w-full rounded-xl border border-zinc-200 px-3 text-base"
                placeholder="İsim"
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
              />
              <input
                className="h-11 w-full rounded-xl border border-zinc-200 px-3 text-base"
                placeholder="Kimin misafiri? (host)"
                value={form.host}
                onChange={(e) => setForm({ ...form, host: e.target.value })}
              />
              <div className="grid grid-cols-2 gap-2">
                <select
                  className="h-11 rounded-xl border border-zinc-200 px-2 text-base"
                  value={form.tip}
                  onChange={(e) => setForm({ ...form, tip: e.target.value })}
                >
                  {TIPS.map((t) => <option key={t} value={t}>{t}</option>)}
                </select>
                <label className="flex items-center gap-2 rounded-xl border border-zinc-200 px-3 text-sm">
                  <input
                    type="checkbox"
                    checked={form.whitelist}
                    onChange={(e) => setForm({ ...form, whitelist: e.target.checked })}
                  />
                  whitelist
                </label>
              </div>
              <Button className="w-full" onClick={save} disabled={saving}>
                {saving ? 'Kaydediliyor…' : 'Kaydet'}
              </Button>
              {saveMsg && <p className="rounded-xl bg-emerald-50 p-2 text-sm text-emerald-700">{saveMsg}</p>}
            </div>

            <h3 className="mb-1 mt-4 font-semibold">Birlikte gelenler ({data.companions.length})</h3>
            {data.companions.length === 0 && <p className="text-sm text-zinc-400">Yalnız gelmiş.</p>}
            <ul className="space-y-1">
              {data.companions.map((c, i) => (
                <li key={i} className="flex items-center justify-between rounded-xl bg-zinc-50 px-3 py-2 text-sm">
                  <button
                    className="font-medium underline"
                    onClick={() => onSelect && onSelect(c.person_id)}
                  >
                    {c.name}
                  </button>
                  <span className="text-xs text-zinc-500">{c.time}</span>
                </li>
              ))}
            </ul>

            <h3 className="mb-1 mt-4 font-semibold">Zaman çizelgesi ({data.visits.length})</h3>
            <ul className="space-y-1 pb-2">
              {data.visits.map((v) => (
                <li key={v.id} className="flex items-center justify-between rounded-xl border border-zinc-100 px-3 py-2 text-sm">
                  <span>{v.time} · {v.karar} · {v.camera}</span>
                  <span className="text-xs text-zinc-500">skor {v.yuz}</span>
                </li>
              ))}
            </ul>
          </>
        )}
      </section>
    </div>
  )
}
