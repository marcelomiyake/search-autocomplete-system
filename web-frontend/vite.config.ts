import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/api/v1/suggestions': process.env.SUGGESTION_SERVICE_PROXY ?? 'http://127.0.0.1:18080',
      '/api/v1/query-events': process.env.ANALYTICS_SERVICE_PROXY ?? 'http://127.0.0.1:18081',
    },
  },
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['src/**/*.spec.ts'],
    setupFiles: ['./src/test-setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'lcov'],
      reportsDirectory: './coverage',
      thresholds: { lines: 80, branches: 80 },
      include: ['src/**/*.{ts,vue}'],
      exclude: ['src/**/*.spec.ts', 'src/main.ts'],
    },
  },
})
