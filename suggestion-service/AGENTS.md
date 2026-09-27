# Suggestion Service guidance

> Human guide: [README.md](../README.md) · [Documentation index](../docs/README.md)

- Own only suggestion matching and the in-memory serving index. Do not query analytics tables from this service.
- Keep normalization consistent with the public contract: ASCII English letters/spaces, trim/collapse whitespace, max 50 characters.
- Preserve deterministic top-five ordering: frequency descending, lexical ascending.
- A bad or stale snapshot must never replace the last good index.
- Update this README and `docs/system-design.md` when endpoints or snapshot behavior change.
- Keep ranking, validation, and refresh behavior covered by focused unit tests.
