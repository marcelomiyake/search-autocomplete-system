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

  it('wraps ArrowUp to the last suggestion, closes on Escape, and clears suggestions for an empty input', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(jsonResponse({ prefix: 'ru', suggestions })))
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('ru')
    await flushPromises()
    await wrapper.get('input').trigger('keydown', { key: 'ArrowUp' })
    expect(wrapper.text()).toContain('Suggestion 2 of 2: rust async')
    await wrapper.get('input').trigger('keydown', { key: 'Escape' })
    expect(wrapper.find('#suggestion-list').exists()).toBe(false)
    await wrapper.get('button[aria-label="Clear search"]').trigger('click')
    await flushPromises()
    expect(wrapper.find('#suggestion-list').exists()).toBe(false)
    expect(wrapper.find('button[aria-label="Clear search"]').exists()).toBe(false)
  })

  it('records a suggestion chosen from the list control', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(jsonResponse({ prefix: 'ru', suggestions }))
      .mockResolvedValueOnce(jsonResponse({ accepted: true }))
    vi.stubGlobal('fetch', fetchMock)
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('ru')
    await flushPromises()
    await wrapper.get('select').setValue('1')
    await flushPromises()
    expect(JSON.parse(fetchMock.mock.calls[1][1].body).query).toBe('rust async')
    expect((wrapper.get('input').element as HTMLInputElement).value).toBe('rust async')
  })

  it('shows an error when the search event cannot be recorded', async () => {
    vi.stubGlobal('fetch', vi.fn()
      .mockResolvedValueOnce(jsonResponse({ prefix: 'rust', suggestions: [] }))
      .mockResolvedValueOnce(jsonResponse({}, 500)))
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('rust')
    await flushPromises()
    await wrapper.get('input').trigger('keydown', { key: 'Enter' })
    await flushPromises()
    expect(wrapper.get('.event-message').text()).toBe('Your search could not be recorded. Please try again.')
  })

  it('shows a generic message when the suggestion request rejects with a non-Error value', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValueOnce('offline'))
    const wrapper = mount(SearchAutocomplete)
    await wrapper.get('input').setValue('ru')
    await flushPromises()
    expect(wrapper.get('[role="alert"]').text()).toBe('Something went wrong. Please try again.')
  })

  it('focuses the search input on Ctrl+K', async () => {
    const wrapper = mount(SearchAutocomplete, { attachTo: document.body })
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'K', ctrlKey: true }))
    expect(document.activeElement).toBe(wrapper.get('input').element)
    wrapper.unmount()
  })
})
