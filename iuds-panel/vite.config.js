import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      // Panelden /api/... istekleri Rust core'a (3901) gider. Tarayıcı tek adres görür.
      '/api': 'http://127.0.0.1:3901',
    },
  },
})
