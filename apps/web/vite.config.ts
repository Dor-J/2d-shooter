import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
export default defineConfig({ plugins: [vue()], server: { proxy: { '/ws': { target: 'ws://127.0.0.1:3000', ws: true }, '/health': 'http://127.0.0.1:3000' } }, build: { target: 'es2022' } })
