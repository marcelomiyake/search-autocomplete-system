# Dependency and Node 26.13.1 update verification

**Collected:** 2026-11-15 (America/Sao_Paulo)
**Revision:** `6184675` (branch `chore/node-26.13.1-security-updates`) plus the follow-up commit that adds the Corepack, Dockerfile, unused-dependency and test changes and this record
**Environment:** Linux x86_64; Node.js 26.13.1; pnpm 12.6.0 locally (the Docker image keeps pnpm 10.33.2); `rustc`/`cargo` 1.100.0; Docker

> Documentation: [verification index](README.md) · [project index](../README.md)

## Changes verified

- Web: Node 26.13.1 build image. The image installs Corepack 0.36.0, because Node 25 and later no longer bundle it. Vitest and `@vitest/coverage-v8` 3 → 4.1.12; Vite 7.3.8; `glob`/`minimatch` overrides and the `esbuild` build approval in the new `web-frontend/pnpm-workspace.yaml`, which the Dockerfile now copies.
- Rust: `rustls` 0.23.46, `rustls-webpki` 0.103.17, `webpki-roots` 1.0.12, `tokio` 1.54.1. The unused workspace dependency `thiserror` was removed; Rust 1.100 reports it through `cargo::unused_dependencies`, and CI denies warnings.
- Tests: five component tests for the uncovered paths (ArrowUp wrap, Escape, clearing input, choosing from the list control, a failed event recording, a non-`Error` rejection, Ctrl+K focus). Vitest 4 measures branch coverage differently, which had dropped the result below the 80% gate.

## Observed results

| Area | Command | Observed result |
| --- | --- | --- |
| Rust advisories | `cargo deny check advisories` | `advisories ok`. |
| Rust warnings | `cargo +stable check --workspace --all-targets` | Finished with no warnings. |
| Rust tests | `SQLX_OFFLINE=true cargo test --workspace --locked` | 9 passed, 0 failed. PostgreSQL integration tests were not run against a database. |
| Web dependency audit | `cd web-frontend && pnpm audit` | No known vulnerabilities. |
| Web tests and coverage | `cd web-frontend && pnpm exec vitest run --coverage` | 11 passed. Coverage: 96% statements, 89.56% branches, 99.02% lines. |
| Web image | `docker build web-frontend` | Passed. |

Not run for this change: Playwright acceptance, Kind deployment, Lighthouse and SonarQube Cloud. SonarQube Cloud runs again when this change is pushed.
