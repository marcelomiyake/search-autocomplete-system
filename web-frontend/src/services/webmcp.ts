import type { WebMCP } from 'webmcp-types'

type Suggestion = { query: string; frequency: number }
type SuggestionResponse = { prefix: string; suggestions: Suggestion[] }

const suggestionSchema = {
  type: 'object',
  properties: {
    prefix: {
      type: 'string',
      description: 'A prefix of up to 50 English letters and spaces.',
      minLength: 1,
      maxLength: 50,
    },
  },
  required: ['prefix'],
  additionalProperties: false,
} as const

export interface ToolRegistration {
  ready: Promise<void>
  unregister: () => void
}

function normalizePrefix(value: unknown): string {
  if (typeof value !== 'string') throw new Error('Provide a search prefix.')
  const prefix = value.trim().replace(/\s+/g, ' ').toLowerCase()
  if (prefix.length === 0 || prefix.length > 50 || !/^[a-z ]+$/.test(prefix)) {
    throw new Error('Use 1 to 50 English letters and spaces for the search prefix.')
  }
  return prefix
}

export function registerSuggestionTool(modelContext: WebMCP.ModelContext | undefined): ToolRegistration | null {
  if (!modelContext || typeof modelContext.registerTool !== 'function') return null

  const registration = new AbortController()
  const ready = modelContext.registerTool({
    name: 'get_search_suggestions',
    title: 'Get search suggestions',
    description: 'Returns up to five existing suggestions for a short prefix. This is read-only: it does not submit or record a search.',
    inputSchema: suggestionSchema,
    annotations: {
      readOnlyHint: true,
      untrustedContentHint: true,
    },
    async execute({ prefix }, { signal }) {
      const normalizedPrefix = normalizePrefix(prefix)
      const response = await fetch(`/api/v1/suggestions?prefix=${encodeURIComponent(normalizedPrefix)}`, { signal })
      if (!response.ok) throw new Error('Suggestions are temporarily unavailable.')
      const result = await response.json() as SuggestionResponse
      const suggestions = result.suggestions.slice(0, 5).map(({ query, frequency }) => ({ query, frequency }))
      return {
        content: [{
          type: 'text',
          text: JSON.stringify({ prefix: normalizedPrefix, suggestions }),
        }],
      }
    },
  }, { signal: registration.signal }).catch(() => {
    registration.abort()
  })

  return {
    ready,
    unregister: () => registration.abort(),
  }
}
