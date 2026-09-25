// shadcn tarzı Button — yeni paket yok, sade tailwind class'ları.
// Kurulum 18:00 sonrası: npm i -D tailwindcss postcss autoprefixer
export function Button({ variant = 'default', size = 'md', className = '', ...props }) {
  const base = 'inline-flex items-center justify-center font-medium rounded-xl transition active:scale-95 disabled:opacity-50'
  const variants = {
    default: 'bg-zinc-900 text-white',
    open: 'bg-emerald-600 text-white',
    deny: 'bg-zinc-200 text-zinc-900',
    ghost: 'bg-transparent text-zinc-600',
  }
  const sizes = { sm: 'h-9 px-3 text-sm', md: 'h-11 px-4 text-base', lg: 'h-14 px-6 text-lg' }
  return <button className={`${base} ${variants[variant]} ${sizes[size]} ${className}`} {...props} />
}
