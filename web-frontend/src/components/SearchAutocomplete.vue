<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { registerSuggestionTool, type ToolRegistration } from '../services/webmcp'

type Suggestion = { query: string; frequency: number }
type SuggestionResponse = { prefix: string; suggestions: Suggestion[] }

const input = ref('')
const inputElement = ref<HTMLInputElement>()
const suggestions = ref<Suggestion[]>([])
const loading = ref(false)
const error = ref('')
const eventError = ref('')
const selectedIndex = ref(-1)
const submittedQuery = ref('')
const listOpen = ref(false)
let requestId = 0
let controller: AbortController | undefined
let suppressInputWatcher = false
let webmcpRegistration: ToolRegistration | null = null

const showList = computed(() => listOpen.value && input.value.trim().length > 0)

async function loadSuggestions(prefix: string) {
  controller?.abort()
  const currentRequest = ++requestId
  if (!prefix.trim()) {
    suggestions.value = []
    loading.value = false
    error.value = ''
    selectedIndex.value = -1
    return
  }

  controller = new AbortController()
  loading.value = true
  error.value = ''
  selectedIndex.value = -1
  try {
    const response = await fetch(`/api/v1/suggestions?prefix=${encodeURIComponent(prefix)}`, { signal: controller.signal })
    if (!response.ok) throw new Error('Suggestions are temporarily unavailable. Try again in a moment.')
    const data = await response.json() as SuggestionResponse
    if (currentRequest === requestId) suggestions.value = data.suggestions.slice(0, 5)
  } catch (reason) {
    if (currentRequest === requestId && !(reason instanceof DOMException && reason.name === 'AbortError')) {
      suggestions.value = []
      error.value = reason instanceof Error ? reason.message : 'Something went wrong. Please try again.'
    }
  } finally {
    if (currentRequest === requestId) loading.value = false
  }
}

watch(input, (value) => {
  if (suppressInputWatcher) { suppressInputWatcher = false; return }
  submittedQuery.value = ''
  eventError.value = ''
  listOpen.value = true
  void loadSuggestions(value)
})

function moveSelection(direction: 1 | -1) {
  if (!suggestions.value.length) return
  listOpen.value = true
  const next = selectedIndex.value + direction
  selectedIndex.value = next < 0 ? suggestions.value.length - 1 : next % suggestions.value.length
}

async function submit(query = input.value) {
  const completed = query.trim().replace(/\s+/g, ' ').toLowerCase()
  if (!completed) return
  eventError.value = ''
  try {
    const response = await fetch('/api/v1/query-events', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ query: completed, idempotency_key: crypto.randomUUID() }),
    })
    if (!response.ok) throw new Error('Your search could not be recorded. Please try again.')
    submittedQuery.value = completed
    if (input.value !== completed) {
      suppressInputWatcher = true
      input.value = completed
    }
    suggestions.value = []
    listOpen.value = false
  } catch (reason) {
    eventError.value = reason instanceof Error ? reason.message : 'Your search could not be recorded. Please try again.'
  }
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowDown') { event.preventDefault(); moveSelection(1) }
  if (event.key === 'ArrowUp') { event.preventDefault(); moveSelection(-1) }
  if (event.key === 'Escape') { listOpen.value = false; selectedIndex.value = -1 }
  if (event.key === 'Enter') {
    event.preventDefault()
    const selected = suggestions.value[selectedIndex.value]
    void submit(selected?.query ?? input.value)
  }
}

function pickSuggestion(query: string) { void submit(query) }

function selectSuggestionFromControl(event: Event) {
  const index = Number((event.currentTarget as HTMLSelectElement).value)
  const suggestion = suggestions.value[index]
  if (suggestion) pickSuggestion(suggestion.query)
}

function onGlobalKeydown(event: KeyboardEvent) {
  if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') {
    event.preventDefault()
    inputElement.value?.focus()
  }
}

onMounted(() => {
  window.addEventListener('keydown', onGlobalKeydown)
  webmcpRegistration = registerSuggestionTool(document.modelContext)
})
onBeforeUnmount(() => {
  webmcpRegistration?.unregister()
  controller?.abort()
  window.removeEventListener('keydown', onGlobalKeydown)
})
</script>

<template>
  <div class="search-wrap">
    <label class="sr-only" for="search-input">Search</label>
    <div class="search-box" :class="{ 'search-box-active': showList }">
      <svg class="search-icon" viewBox="0 0 24 24" fill="none" aria-hidden="true"><circle cx="10.9" cy="10.9" r="6.4" stroke="currentColor" stroke-width="1.7"/><path d="m16 16 4.2 4.2" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"/></svg>
      <input
        ref="inputElement"
        id="search-input"
        v-model="input"
        type="search"
        maxlength="50"
        placeholder="Search anything..."
        autocomplete="off"
        role="combobox"
        aria-autocomplete="list"
        :aria-expanded="showList"
        aria-controls="suggestion-list"
        @focus="listOpen = true"
        @keydown="onKeydown"
      />
      <button v-if="input" class="clear-button" type="button" aria-label="Clear search" @click="input = ''">×</button>
      <span class="search-shortcut" aria-hidden="true"><kbd>⌘</kbd><kbd>K</kbd></span>
    </div>

    <div v-if="showList" id="suggestion-list" class="suggestion-panel">
      <p class="suggestion-label">{{ loading ? 'FINDING SUGGESTIONS' : error ? 'SEARCH STATUS' : 'POPULAR SEARCHES' }}</p>
      <output v-if="loading" class="state-message" aria-live="polite"><span class="spinner"></span> Finding suggestions…</output>
      <div v-else-if="error" class="state-message error-message" role="alert">{{ error }}</div>
      <select
        v-else-if="suggestions.length"
        class="suggestion-select"
        aria-label="Search suggestions"
        :size="Math.min(suggestions.length, 5)"
        :value="selectedIndex < 0 ? '' : String(selectedIndex)"
        @change="selectSuggestionFromControl"
      >
        <option value="" disabled hidden>Select a suggestion</option>
        <option v-for="(suggestion, index) in suggestions" :key="suggestion.query" :value="String(index)">
          {{ suggestion.query }} · {{ suggestion.frequency.toLocaleString() }} searches
        </option>
      </select>
      <output v-else class="state-message" aria-live="polite">No popular searches yet. Try another phrase.</output>
    </div>
    <output class="sr-only" aria-live="polite">{{ selectedIndex >= 0 && suggestions[selectedIndex] ? `Suggestion ${selectedIndex + 1} of ${suggestions.length}: ${suggestions[selectedIndex].query}` : '' }}</output>
    <p v-if="eventError" class="event-message" role="alert">{{ eventError }}</p>
    <output v-else-if="submittedQuery" class="event-message success-message">Search recorded for “{{ submittedQuery }}”.</output>
  </div>
</template>
