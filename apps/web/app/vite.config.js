import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    wasm()
  ],
  optimizeDeps: {
    exclude: ['ydx-wasm']
  },
  build: {
    target: 'es2020',
    rollupOptions: {
      external: ['ydx-wasm']
    }
  },
  server: {
    fs: {
      allow: ['.', '..']
    },
    proxy: {
      '/api': {
        target: 'http://0.0.0.0:3002',
        changeOrigin: true,
      },
    },
  },
})
