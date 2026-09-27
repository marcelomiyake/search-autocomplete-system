import { vi } from 'vitest'

Object.defineProperty(globalThis, 'crypto', {
  value: { randomUUID: () => 'test-idempotency-key' },
  configurable: true,
})

afterEach(() => vi.restoreAllMocks())
