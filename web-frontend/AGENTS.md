# Web Frontend guidance

> Human guide: [README.md](../README.md) · [Documentation index](../docs/README.md)

- Keep the interface accessible through a labeled combobox, listbox options, visible request states, and keyboard navigation.
- Record completed selections/submissions only, and include a fresh idempotency key for each search event.
- Do not store, log, or commit a real user's query history. Local seed data is synthetic.
- Keep API errors visible and preserve a usable input when requests fail.
- Use same-origin `/api/v1/...` paths in the browser. Nginx and Vite proxy configuration belong at the edge.
- Keep the WebMCP tool read-only: it may call the Suggestions GET endpoint but must not submit completed-query events. Validate bounded input, mark query text as untrusted output, feature-detect the browser API, and unregister on component teardown.
- Maintain component and Playwright acceptance tests when interaction behavior changes.
- Keep the frontend README, root README, and `docs/system-design.md` aligned.
