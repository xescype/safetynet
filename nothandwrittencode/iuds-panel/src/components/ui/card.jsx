// shadcn tarzı Card + Badge — paketsiz.
export function Card({ className = '', ...props }) {
  return <div className={`rounded-2xl border border-zinc-200 bg-white shadow-sm ${className}`} {...props} />
}

const badgeColors = {
  L0: 'bg-amber-100 text-amber-800',
  L1: 'bg-sky-100 text-sky-800',
  L2: 'bg-emerald-100 text-emerald-800',
}
export function TrustBadge({ level }) {
  return (
    <span className={`rounded-full px-2.5 py-0.5 text-xs font-semibold ${badgeColors[level] || 'bg-zinc-100'}`}>
      {level}
    </span>
  )
}
