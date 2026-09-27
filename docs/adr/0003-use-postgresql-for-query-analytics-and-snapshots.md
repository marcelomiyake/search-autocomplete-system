# ADR-0003: Use PostgreSQL for Query Analytics and Snapshots

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

The analytics service owns committed-query records and aggregate data. The index builder reads aggregates and stores revisioned snapshots; the suggestion service loads the latest snapshot into process memory. PostgreSQL is the local durable source, not the online prefix lookup engine.

The scope of this decision is PostgreSQL as durable storage for query events, aggregates, and revisioned suggestion snapshots. The source confirms the implementation; its historical selection rationale and original option set are not recorded.

## Decision drivers

- Persist accepted query events and aggregation state across service restarts.
- Make snapshot creation repeatable by source revision.
- Keep the hot suggestion path small while retaining a durable rebuild source.

## Options considered

### PostgreSQL

- **Benefits:** Relational migrations and durable transactions support the current event, aggregate, and snapshot state.
- **Costs and risks:** The local chart uses a single database instance and snapshot history has an explicit retention gap.

### Redis as the authoritative store

- **Benefits:** Could provide low-latency structures for counters and lookups.
- **Costs and risks:** Would make durability, rebuild, and revision semantics depend on a new stateful service.

### In-memory state only

- **Benefits:** Would reduce local infrastructure for a disposable demonstration.
- **Costs and risks:** Would lose accepted analytics and snapshot state across process restarts.

## Decision outcome

Keep PostgreSQL as the durable source for analytics and revisioned snapshots; retain the in-memory prefix index as a serving cache that can reload persisted snapshots.

## Consequences

### Positive

- A service restart can reload durable analytics and snapshots.
- Snapshot revisions provide a basis for idempotent rebuilds and freshness inspection.

### Negative and risks

- The local database is a single replica and is not a high-availability deployment.
- Without retention or compaction, persisted snapshots can accumulate.

## Evidence and realization

- [0001_init.sql](../../query-analytics-service/migrations/0001_init.sql)
- [0001_init.sql](../../index-builder-worker/migrations/0001_init.sql)
- [values.yaml](../../deploy/helm/search-autocomplete/values.yaml)
- [database-model.md](../database-model.md)
- [system-design.md](../system-design.md)

## Review triggers

- Reconsider if event volume, aggregation latency, or snapshot history requires a different persistence model.
- Define snapshot retention and recovery before treating this profile as a long-running production service.

## References

- [search-autocomplete-system README](../../README.md)
- [System Design](../system-design.md)
- [ADR practices](https://adr.github.io/ad-practices/)
- [ADR template guidance](https://adr.github.io/adr-templates/)
