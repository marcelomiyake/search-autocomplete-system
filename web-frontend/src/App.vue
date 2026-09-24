<script setup lang="ts">
import SearchAutocomplete from './components/SearchAutocomplete.vue'
</script>

<template>
  <main class="page-shell">
    <header class="topbar">
      <a class="brand" href="/" aria-label="Findly home">
        <span class="brand-mark" aria-hidden="true"><span></span><span></span><span></span></span>
        <span>findly<span class="brand-period">.</span></span>
      </a>
      <div class="topbar-note"><span class="status-dot"></span> Local demo environment</div>
    </header>

    <section class="hero" aria-labelledby="hero-title">
      <p class="eyebrow"><span class="eyebrow-line"></span> SEARCH, MADE SIMPLE</p>
      <h1 id="hero-title">Find what you’re<br /><span>looking for.</span></h1>
      <p class="hero-copy">A little less searching. A lot more finding.</p>

      <SearchAutocomplete />

      <div class="search-hint"><kbd>↑</kbd><kbd>↓</kbd> to navigate <span class="hint-divider">·</span> <kbd>↵</kbd> to search</div>
      <p class="sample-hint">Try “ru” or “the sky is” to explore demo suggestions.</p>
    </section>

    <section class="proof-row" aria-label="How suggestions work">
      <div class="proof-card">
        <span class="proof-icon icon-sparkle" aria-hidden="true">✳</span>
        <div><strong>Helpful suggestions</strong><span>Ranked by what people search</span></div>
      </div>
      <div class="proof-card">
        <span class="proof-icon icon-speed" aria-hidden="true">↗</span>
        <div><strong>Fast by design</strong><span>Built for a 100 ms target</span></div>
      </div>
      <div class="proof-card">
        <span class="proof-icon icon-privacy" aria-hidden="true">⌑</span>
        <div><strong>Local and private</strong><span>Demo data stays on this machine</span></div>
      </div>
    </section>

    <footer class="page-footer">
      <span>Thoughtful search starts here.</span>
      <span><span class="footer-sparkle">✳</span> A system design learning project</span>
    </footer>
  </main>
</template>
