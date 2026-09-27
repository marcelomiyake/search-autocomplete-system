# API and data contract catalog

This catalog records the suggestion, completed-query, internal snapshot, and shared-schema boundaries. The [System Design](../system-design.md) and service source are authoritative; the repository currently has no separate OpenAPI file for these endpoints.

> Documentation: [project index](../README.md) · [repository overview](../../README.md)

## Contents

- [Contracts](#contracts)
- [Consumer map](#consumer-map)
- [Compatibility and security](#compatibility-and-security)
- [Related documentation](#related-documentation)
- [Document lifecycle](#document-lifecycle)
- [AI development disclaimer](#ai-development-disclaimer)

## Contracts

| Contract | Type and authority | Owner | Producer | Consumers |
| --- | --- | --- | --- | --- |
| Suggestions API (`GET /api/v1/suggestions`) | REST/JSON; `suggestion-service/` and [System Design](../system-design.md#7-apis-and-data-contracts) | `search-autocomplete-system` / `suggestion-service` | `search-autocomplete-system` / `suggestion-service` | `search-autocomplete-system` / `web-frontend`. |
| WebMCP `get_search_suggestions` tool | Browser tool contract; [`web-frontend/src/services/webmcp.ts`](../../web-frontend/src/services/webmcp.ts) plus the Suggestions API | `search-autocomplete-system` / `web-frontend` | `search-autocomplete-system` / `web-frontend` registers the tool in the active document | A browser agent invoking the same-page `document.modelContext`; specific agent products are unknown. No cross-origin exposure is configured. |
| Completed-query event (`POST /api/v1/query-events`) | REST/JSON command/event intake; `query-analytics-service/` and [System Design](../system-design.md#7-apis-and-data-contracts) | `search-autocomplete-system` / `query-analytics-service` | `search-autocomplete-system` / `web-frontend` submits a completed search | `search-autocomplete-system` / `query-analytics-service`, then `search-autocomplete-system` / `index-builder-worker` through analytics data. |
| Frequency export (`GET /internal/v1/frequencies`) | Internal REST/JSON; `query-analytics-service/` and [System Design](../system-design.md#7-apis-and-data-contracts) | `search-autocomplete-system` / `query-analytics-service` | `search-autocomplete-system` / `query-analytics-service` | `search-autocomplete-system` / `index-builder-worker`. |
| Snapshot API (`GET /internal/v1/snapshots/latest`) | Internal REST/JSON; `index-builder-worker/` and [System Design](../system-design.md#7-apis-and-data-contracts) | `search-autocomplete-system` / `index-builder-worker` | `search-autocomplete-system` / `index-builder-worker` | `search-autocomplete-system` / `suggestion-service`. |
| PostgreSQL schemas | Versioned SQL migrations in the analytics and index-builder service folders | Each service owns its schema | `search-autocomplete-system` / `query-analytics-service` and `search-autocomplete-system` / `index-builder-worker` | Each owning service; the other context does not share table ownership. |

## Consumer map

The browser is the only known external-to-service client in this repository. The WebMCP tool is exposed only to an agent using the active page; concrete browser-agent consumers are unknown. Other-repository API consumers are unknown; none is recorded in the tracked repositories. Internal service-to-service consumers are listed explicitly above.

## Compatibility and security

Queries are normalized, length-limited, and idempotent by event key. Internal APIs are cluster-only in the local deployment; production authentication is not implemented. Do not use personal or real query data.

## Related documentation

- [System Design](../system-design.md)
- [Project README](../../README.md)
- [Documentation index](../README.md)

## Document lifecycle

This contract catalog is maintained as Markdown and links to the implementation-owned interface definitions. It has no independent software build, deployment, or undeployment lifecycle. Review it when its linked contracts or consumers change.


## AI development disclaimer

> **AI development disclaimer:** This project was built entirely with GPT-6 Luna at Max effort as a proof of concept exploring how low-cost AI plans can be useful when paired with disciplined harness and loop engineering. This is project-owner attribution; repository contents do not independently verify runtime model metadata. Review AI-generated design and code before relying on them.
