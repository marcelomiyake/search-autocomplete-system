# ADR-0002: Use Vue for the Search Web Client

- **Status:** Implemented (retrospective)
- **Recorded:** 2026-09-25
- **Original decision date:** Unknown from repository evidence
- **Decision owner:** Project owner
- **Confirmation:** Current implementation is documented at the project owner’s request; historical team approval is not recorded.

> This record documents the technology in the current implementation. The options and rationale below are a retrospective comparison, not a claim that the original project formally evaluated them.

## Contents

- [Context and problem statement](#context-and-problem-statement)
- [Decision drivers](#decision-drivers)
- [Options considered](#options-considered)
- [Decision outcome](#decision-outcome)
- [Consequences](#consequences)
- [Evidence and realization](#evidence-and-realization)
- [Review triggers](#review-triggers)
- [References](#references)

## Context and problem statement

The user-facing web client calls the suggestion and query-analytics APIs and renders prefix completions and search state. The original UI-framework selection record was not found.

The scope of this decision is Vue 3 and TypeScript for the browser search and autocomplete interface. The source confirms the implementation; its historical selection rationale and original option set are not recorded.

## Decision drivers

- Keep interactive suggestions responsive as the user types.
- Separate browser presentation from ranking and analytics service ownership.
- Retain type checking and browser-level verification for API interactions.

## Options considered

### Vue 3 with TypeScript

- **Benefits:** The current component UI and Vite tooling support the implemented input, completion list, and query flow.
- **Costs and risks:** Maintains framework-specific conventions and a separate frontend toolchain.

### React with TypeScript

- **Benefits:** Could support the same reactive interface and testing model.
- **Costs and risks:** Would require replacing existing components and tests without an identified Vue limitation.

### Server-rendered pages

- **Benefits:** Could reduce client framework code for static search pages.
- **Costs and risks:** Does not align as directly with the current keystroke-driven autocomplete interaction.

## Decision outcome

Retain Vue 3 and TypeScript for the web client. The decision records current implementation; the comparison is retrospective rather than a claim about historical selection.

## Consequences

### Positive

- The browser remains a presentation and API-consumer boundary.
- Frontend tests can cover suggestion rendering and query submission independently of backend internals.

### Negative and risks

- Client behavior depends on the deployed suggestion API contract and index freshness.
- The repository has no measured framework comparison.

## Evidence and realization

- [package.json](../../web-frontend/package.json)
- [Dockerfile](../../web-frontend/Dockerfile)
- [system-design.md](../system-design.md)
- [README.md](../../README.md)

## Review triggers

- Reconsider if measured browser performance or required platform support shows Vue cannot meet project goals.
- Create a superseding ADR before replacing the UI framework or moving ranking logic into the browser.

## References

- [search-autocomplete-system README](../../README.md)
- [System Design](../system-design.md)
- [ADR practices](https://adr.github.io/ad-practices/)
- [ADR template guidance](https://adr.github.io/adr-templates/)
