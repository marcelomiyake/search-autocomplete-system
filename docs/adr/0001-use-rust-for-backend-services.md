# ADR-0001: Use Rust for Backend Services

- **Status:** Implemented (retrospective)
- **Recorded:** 2026-09-25
- **Original decision date:** Unknown from repository evidence
- **Decision owner:** Project owner
- **Confirmation:** Current implementation is documented at the project owner's request; historical team approval is not recorded.

> This record captures the Rust backend already present in the repository. The options and rationale below are a retrospective comparison, not a claim that the original project formally evaluated them.

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

The project has three Rust backend services: query analytics records completed searches, an index builder creates revisioned suggestion snapshots, and the suggestion service serves prefix matches from an in-memory index. The browser client is Vue.

The project owner's rationale is that Rust is safer, fast, and can use less CPU and memory. The owner recognizes Rust's deeper learning curve and considers it feasible with AI-assisted code authoring. These are design expectations and the owner's experience, not results from a comparative benchmark in this repository.

## Decision drivers

- Catch memory-safety and concurrency errors through Rust's type and ownership checks.
- Keep the online suggestion path responsive and resource-conscious.
- Support concurrent database and HTTP work in independent services.
- Make Rust development manageable for this proof of concept with AI assistance, compiler feedback, focused tests, and human review.

## Options considered

### Rust for backend services

- **Benefits:** Compiled native services, ownership checks, and low runtime overhead align with a small in-memory serving process and local resource budgets.
- **Costs and risks:** Rust's ownership, lifetime, and async concepts take longer to learn; builds can be slower than lightweight scripting services.

### Go for backend services

- **Benefits:** A comparatively approachable statically typed option with built-in concurrency and a good fit for HTTP services.
- **Costs and risks:** Garbage collection and a different memory/concurrency model; changing languages would not remove the need to design snapshot freshness and database boundaries.

### TypeScript/Node.js for backend services

- **Benefits:** Could reduce language switching with the Vue client.
- **Costs and risks:** Managed runtime overhead and no ownership-based memory checks; whether it meets the same resource targets would require measurement.

No cross-language performance benchmark is available, so the trade-offs are qualitative.

## Decision outcome

Use Rust for query analytics, index building, and suggestion serving. AI-assisted authoring makes the learning curve acceptable for this proof of concept when paired with compiler checks, tests, documented contracts, and human review. Snapshots remain the serving boundary; this ADR does not change the separate decision to use periodic refresh instead of a broker.

## Consequences

### Positive

- Rust's type and ownership checks help prevent memory-safety errors in concurrent service code.
- Native compilation is expected to keep the small online service resource-efficient; actual comparative CPU/memory results are not available.
- The service boundaries remain independently deployable and can be tuned separately.

### Negative and risks

- Contributors need to learn Rust ownership, async execution, and the project's service contracts.
- AI-generated code may preserve syntax while violating normalization, idempotency, or snapshot-revision invariants; those require tests and review.
- The language choice alone does not establish the 100 ms design target; load and latency evidence are still needed.

## Evidence and realization

- Rust crates are defined by [query analytics](../../query-analytics-service/Cargo.toml), [index builder](../../index-builder-worker/Cargo.toml), and [suggestion service](../../suggestion-service/Cargo.toml).
- The [System Design](../system-design.md) documents the in-memory index, revisioned snapshots, polling interval, and acceptance cases.
- The [Kubernetes resource budgets](../kubernetes-resources.md) describe configured local limits, not language benchmarks.

## Review triggers

- Reconsider if measured query latency or resource use misses a documented target and a controlled comparison favors another implementation language.
- Reconsider if Rust's learning or maintenance burden remains material despite AI assistance, compiler feedback, tests, and review.
- Create a superseding ADR before migrating a backend service to a new language or runtime.

## References

- [Search Autocomplete README](../../README.md)
- [System Design](../system-design.md)
- [ADR practices](https://adr.github.io/ad-practices/)
- [ADR template guidance](https://adr.github.io/adr-templates/)
