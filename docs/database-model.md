# Search Autocomplete database model

This is the human-readable model of the PostgreSQL schemas used by query analytics and index building. The migrations are authoritative; update this inventory when they change.

- **Owner:** `search-autocomplete-system`; `query-analytics-service` owns query aggregation, and `index-builder-worker` owns persisted snapshots.
- **Known consumers:** `search-autocomplete-system` / `query-analytics-service` writes query events/frequencies and reads its revision; `search-autocomplete-system` / `index-builder-worker` reads frequencies and writes snapshots; `search-autocomplete-system` / `suggestion-service` reads snapshots. No cross-repository database consumer is identified.
- **Database:** PostgreSQL with separate `query_analytics` and `index_builder` schemas.
- **Sources:** [`query-analytics-service/migrations/0001_init.sql`](../query-analytics-service/migrations/0001_init.sql), [`index-builder-worker/migrations/0001_init.sql`](../index-builder-worker/migrations/0001_init.sql).

## Tables

### `query_analytics.query_events`

| Column | Type and rules | Description |
| --- | --- | --- |
| `idempotency_key` | TEXT, primary key | Stable key preventing duplicate recording of the same submitted query event. |
| `normalized_query` | TEXT, required | Normalized completed search text. |
| `recorded_at` | TIMESTAMPTZ, required, default `now()` | Time the event was committed. |

### `query_analytics.query_frequencies`

| Column | Type and rules | Description |
| --- | --- | --- |
| `normalized_query` | TEXT, primary key | Canonical query used as the aggregate key. |
| `frequency` | BIGINT, required, greater than zero | Number of committed submissions represented by the aggregate. |

### `query_analytics.service_state`

| Column | Type and rules | Description |
| --- | --- | --- |
| `singleton` | BOOLEAN, primary key, default `TRUE`, must remain `TRUE` | Ensures the state table contains one revision row. |
| `revision` | BIGINT, required, default `0` | Monotonic aggregate revision used to detect snapshot freshness. |

The migration inserts the singleton row if absent.

### `index_builder.snapshots`

| Column | Type and rules | Description |
| --- | --- | --- |
| `revision` | BIGINT, primary key, non-negative | Analytics revision represented by this immutable snapshot. |
| `snapshot` | JSONB, required | Serialized versioned autocomplete index content. |
| `created_at` | TIMESTAMPTZ, required, default `now()` | Snapshot build time. |

## Related documentation

- [System Design](system-design.md)
- [Contract catalog](contracts/README.md)
- [Documentation index](README.md)
