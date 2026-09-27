import type { WebMCP } from 'webmcp-types'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { registerSuggestionTool } from './webmcp'

afterEach(() => vi.unstubAllGlobals())

function modelContext() {
  const tools: WebMCP.ModelContextTool[] = []
  let registrationSignal: AbortSignal | undefined
  const registerTool = vi.fn(async (tool: WebMCP.ModelContextTool, options?: WebMCP.ModelContextRegisterToolOptions) => {
    tools.push(tool)
    registrationSignal = options?.signal
  })
  return {
    tools,
    registerTool,
    get registrationSignal() { return registrationSignal },
    context: { registerTool } as unknown as WebMCP.ModelContext,
  }
}

describe('WebMCP suggestion tool', () => {
  it('registers a read-only tool and returns at most five suggestions without submitting a search', async () => {
    const browser = modelContext()
    const fetchMock = vi.fn(async () => new Response(JSON.stringify({
      prefix: 'rust',
      suggestions: Array.from({ length: 7 }, (_, index) => ({ query: `rust ${index}`, frequency: 10 - index })),
    }), { status: 200 }))
    vi.stubGlobal('fetch', fetchMock)

    const registration = registerSuggestionTool(browser.context)
    expect(registration).not.toBeNull()
    await registration?.ready
    expect(browser.tools[0]?.name).toBe('get_search_suggestions')
    expect(browser.tools[0]?.annotations).toEqual({ readOnlyHint: true, untrustedContentHint: true })

    const result = await browser.tools[0]!.execute({ prefix: ' RUST  ' }, { signal: new AbortController().signal }) as {
      content: Array<{ text: string }>
    }
    expect(fetchMock).toHaveBeenCalledWith('/api/v1/suggestions?prefix=rust', expect.any(Object))
    expect(JSON.parse(result.content[0]!.text).suggestions).toHaveLength(5)

    registration?.unregister()
    expect(browser.registrationSignal?.aborted).toBe(true)
  })

  it('rejects invalid prefixes and leaves unsupported browsers untouched', async () => {
    expect(registerSuggestionTool(undefined)).toBeNull()

    const browser = modelContext()
    const fetchMock = vi.fn()
    vi.stubGlobal('fetch', fetchMock)
    const registration = registerSuggestionTool(browser.context)
    await registration?.ready

    await expect(browser.tools[0]!.execute({ prefix: 'café' }, { signal: new AbortController().signal }))
      .rejects.toThrow('English letters and spaces')
    expect(fetchMock).not.toHaveBeenCalled()
  })
})
