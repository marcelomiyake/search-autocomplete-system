import { flushPromises, mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import SearchAutocomplete from './SearchAutocomplete.vue'

const suggestions = [{ query: 'rust', frequency: 12 }, { query: 'rust async', frequency: 3 }]
const jsonResponse = (body: unknown, status = 200) => new Response(JSON.stringify(body), { status, headers: { 'content-type': 'application/json' } })

describe('SearchAutocomplete', () => {
  it('shows top suggestions and submits the highlighted query with an idempotency key', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(jsonResponse({ prefix: 'ru', suggestions }))
      .mockResolvedValueOnce(jsonResponse({ accepted: true }))
    vi.stubGlobal('fetch', fetchMock)
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('ru')
    await flushPromises()
    expect(wrapper.text()).toContain('rust async')
    await wrapper.get('input').trigger('keydown', { key: 'ArrowDown' })
    await wrapper.get('input').trigger('keydown', { key: 'Enter' })
    await flushPromises()
    expect(fetchMock.mock.calls[1][0]).toBe('/api/v1/query-events')
    expect(JSON.parse(fetchMock.mock.calls[1][1].body)).toEqual({ query: 'rust', idempotency_key: 'test-idempotency-key' })
    expect(wrapper.text()).toContain('Search recorded for “rust”.')
  })

  it('shows empty results and a suggestion service failure', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(jsonResponse({ prefix: 'qz', suggestions: [] }))
      .mockResolvedValueOnce(jsonResponse({}, 503))
    vi.stubGlobal('fetch', fetchMock)
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('qz')
    await flushPromises()
    expect(wrapper.text()).toContain('No popular searches yet.')
    await wrapper.get('input').setValue('ru')
    await flushPromises()
    expect(wrapper.text()).toContain('temporarily unavailable')
  })

  it('submits the typed query on Enter when no suggestion is highlighted', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(jsonResponse({ prefix: 'rust', suggestions: [] }))
      .mockResolvedValueOnce(jsonResponse({ accepted: true }))
    vi.stubGlobal('fetch', fetchMock)
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue(' Rust ')
    await flushPromises()
    await wrapper.get('input').trigger('keydown', { key: 'Enter' })
    await flushPromises()
    expect(JSON.parse(fetchMock.mock.calls[1][1].body).query).toBe('rust')
  })
})
