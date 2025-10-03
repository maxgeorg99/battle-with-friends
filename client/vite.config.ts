import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  base: process.env.VITE_BASE_PATH || '/',
  server: {
    port: 3000,
    open: true,
    headers: {
      'Content-Security-Policy': "script-src 'self' 'wasm-unsafe-eval' 'unsafe-inline' chrome-extension://9a10dc10-8dc2-4363-9dd8-add828fe9f85;",
    },
  },
  build: {
    outDir: 'dist',
  },
});