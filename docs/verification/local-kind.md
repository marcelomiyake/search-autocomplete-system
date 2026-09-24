# Local Kind verification

**Status:** passed for the educational local profile. These results describe the local cluster only; they do not establish production availability or capacity.

> Project documentation index: [Documentation index](../README.md)

| Field | Result |
|---|---|
| Date | 2026-09-25 (America/Sao_Paulo) |
| Git revision | Baseline `4379d57` plus uncommitted implementation, test, and coverage changes; no commit created |
| Kubernetes context / Kind cluster | `kind-kind` / `kind` |
| Namespace | `search-autocomplete` |
| Environment | Linux 7.0.0-34-generic x86_64; Rust/Cargo 1.98.1; Node.js 24.21.0; Corepack pnpm 10.33.2; kubectl 1.37.1; Kind 0.33.0; Helm 4.3.0 |
| Deploy command | `./scripts/deploy-kind.sh` |
| Deployment replicas | `web-frontend`, `suggestion-service`, `query-analytics-service`, and `index-builder-worker` each 2/2 ready; PostgreSQL 1/1 |
| Rust unit and PostgreSQL integration tests | Passed: 6 unit tests and 3 PostgreSQL-backed integration tests |
| Frontend component tests and build | Passed: 4 component tests; TypeScript check and Vite production build passed |
| Playwright E2E | Passed: 2 tests against the running local frontend |
| Helm lint | Passed |
| SonarQube / Clippy | All four Sonar projects above 80% overall coverage, all four gates passed with zero open issues; Rust Clippy passed with warnings denied. See [`sonarqube.md`](sonarqube.md). |
| Suggestion API acceptance result | For synthetic prefix `the sky is`: `the sky is blue` (3), `the sky is beautiful` (2), `the sky is clear` (2), `the sky is falling` (1), `the sky is vast` (1) |
| Frontend screenshot | Captured from the running frontend: [`../assets/search-autocomplete.png`](../assets/search-autocomplete.png) |

## Commands and results

Commands run from the repository root:

```sh
cargo fmt --all
TEST_DATABASE_URL='postgres://autocomplete@127.0.0.1:5433/search_autocomplete_test' cargo test --workspace
(cd web-frontend && corepack pnpm@10.33.2 test)
(cd web-frontend && corepack pnpm@10.33.2 build)
(cd web-frontend && E2E_BASE_URL=http://127.0.0.1:4173 corepack pnpm@10.33.2 test:e2e)
helm lint deploy/helm/search-autocomplete
./scripts/deploy-kind.sh
TEST_DATABASE_URL='postgres://autocomplete@127.0.0.1:5433/search_autocomplete_test' ./scripts/sonar-scan.sh
```

The Rust run passed six unit tests and three database integration tests, covering normalization, trie order and limits, analytics idempotency, snapshot revision handling, snapshot build/load, API routes and their error cases, and snapshot refresh fallback. Frontend tests passed four component cases and the production build succeeded. The two E2E cases cover suggestions, keyboard selection, submitted-query recording, empty results, service failure, and the seeded `the sky is` prefix. Helm lint and deployment completed successfully.

The Kind acceptance run verified all app Deployments at 2/2 and PostgreSQL at 1/1. The app-level query flow returned the expected top five for `the sky is`; committed query events feed analytics aggregates, the worker persists a versioned snapshot, and the suggestion service serves it. The seed and acceptance data are synthetic. The chapter's 100 ms response objective was not benchmarked; scenario traffic numbers in [`../system-design.md`](../system-design.md) are assumptions, not Kind capacity claims.
