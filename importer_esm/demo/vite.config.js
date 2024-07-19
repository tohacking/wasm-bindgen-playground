import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    fs: {
      allow: [
        // search up for workspace root
        '.',
        // your custom rules
        '../../../',
      ],
    },
  },
  plugins: [react()],
})
