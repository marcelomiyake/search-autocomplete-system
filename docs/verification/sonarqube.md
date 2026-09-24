# SonarQube verification

**Status:** the 2026-09-25 re-analysis submitted all four projects; all four quality gates passed, coverage is at least 86.3%, and SonarQube reports zero open issues for each project.

> Project documentation index: [Documentation index](../README.md)

These measures were read from the local SonarQube API on 2026-09-25 (America/Sao_Paulo), after the latest scan. In SonarQube, open a project and choose **Measures → Coverage**.

| Project key | Overall coverage | Line coverage | Branch coverage | Open issues | Quality gate |
|---|---:|---:|---:|---:|---|
| [`search-autocomplete-web-frontend`](http://127.0.0.1:9000/dashboard?id=search-autocomplete-web-frontend) | **86.3%** | 87.8% (195/222) | 81.2% (56/69) | 0 | Passed (`OK`) |
| [`search-autocomplete-suggestion-service`](http://127.0.0.1:9000/dashboard?id=search-autocomplete-suggestion-service) | **100.0%** | 100.0% (171/171) | Not reported | 0 | Passed (`OK`) |
| [`search-autocomplete-query-analytics-service`](http://127.0.0.1:9000/dashboard?id=search-autocomplete-query-analytics-service) | **100.0%** | 100.0% (166/166) | Not reported | 0 | Passed (`OK`) |
| [`search-autocomplete-index-builder-worker`](http://127.0.0.1:9000/dashboard?id=search-autocomplete-index-builder-worker) | **99.2%** | 99.2% (125/126) | Not reported | 0 | Passed (`OK`) |

The aggregate **Coverage** metric is SonarQube's combined measure. The frontend also reports line and branch measures; the configured Rust LCOV importer reports lines but does not publish a Sonar branch-coverage measure.

| Field | Result |
|---|---|
| SonarQube URL / version | `http://127.0.0.1:9000` / Community Build `26.9.0.129388` |
| Analysis date | 2026-09-25 (America/Sao_Paulo) |
| Analyzed source | Worktree based on `84eb6c7` plus uncommitted source and documentation changes; no commit created |
| Scan command | `TEST_DATABASE_URL='postgres://autocomplete@127.0.0.1:15433/search_autocomplete_test' ./scripts/sonar-scan.sh` (analysis token supplied through the environment) |
| Test database | Temporary PostgreSQL 17 container on loopback port 15433; removed after the scan |
| Coverage reports | Frontend `web-frontend/coverage/lcov.info`; Rust `coverage/lcov.info` in each backend service directory (generated locally, Git-ignored) |
| Gate conditions | All four `OK`; all projects had 0 open issues and 0.0% duplicated lines. |

The Rust `src/main.rs` files are excluded from the coverage denominator because they only wire configuration and start the HTTP process; they are not invoked by the library and integration tests. SonarQube still statically analyzes those entrypoints. The backend library and route logic remain covered by unit tests, PostgreSQL-backed integration tests, and the local Kind acceptance run. No domain or application source modules are excluded. `cargo llvm-cov` enforces at least 80% line coverage for each backend package, and frontend Vitest enforces 80% line and branch coverage.

The latest scan ran the frontend and Rust coverage suites, generated Clippy JSON reports, and submitted all four analyses successfully. The checks passed: frontend 6 tests; suggestion service 3 unit + 1 integration; query analytics 1 unit + 1 PostgreSQL integration; index builder 2 unit + 1 PostgreSQL integration. The scanner reported that no SCM provider was detected because each component directory was mounted separately; coverage and issue measures are current, while blame and new-code attribution may be incomplete. The local Community Build's issue count and ratings are limited to its enabled analyzers; they are not a comprehensive security audit. Generated coverage reports and scanner work files are ignored by Git. Full Kind, E2E, and deployment results are in [`local-kind.md`](local-kind.md).
