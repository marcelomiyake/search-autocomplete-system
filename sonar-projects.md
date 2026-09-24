# SonarQube projects

The local SonarQube instance has four independent project keys, one per application service.

> Project documentation index: [Documentation index](docs/README.md)

| Project key | Base directory | Sources | Tests |
|---|---|---|---|
| `search-autocomplete-web-frontend` | `web-frontend` | `src` | `src` (Vitest) |
| `search-autocomplete-suggestion-service` | `suggestion-service` | `src` | `src`, `tests` |
| `search-autocomplete-query-analytics-service` | `query-analytics-service` | `src` | `src`, `tests` |
| `search-autocomplete-index-builder-worker` | `index-builder-worker` | `src` | `src`, `tests` |

Create these projects in the configured local SonarQube and analyze them individually with the configured token. Record the real scan and quality gate results in `docs/verification/sonarqube.md`. Do not invent quality gate results if scanner submission is denied.
