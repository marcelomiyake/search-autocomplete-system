# Query Analytics Service

Rust/Axum bounded context that records completed searches and owns the `query_analytics` PostgreSQL schema.

> Documentation: [project index](../docs/README.md) · [repository overview](../README.md)

## Contract

- `POST /api/v1/query-events`: accepts `{ "query": "rust web", "idempotency_key": "..." }`. It normalizes to lowercase with collapsed spaces, accepts only `a-z` and spaces up to 50 characters, and increments frequency only for a new idempotency key.
- `GET /internal/v1/frequencies`: returns the monotonic source revision and current `{term, frequency}` aggregates for the index builder.
- `GET /healthz`: checks PostgreSQL connectivity.

The event insert, aggregate increment, and revision increment use one database transaction. Run sample seeding only with `SEED_SAMPLE_DATA=true`; fixed keys make local seed retries idempotent and the sample terms are synthetic. The local demo includes example prefixes `ru` and `the sky is` so the UI is useful before anyone submits a search.

## Run and verify

```sh
DATABASE_URL='postgres://...' cargo run -p query-analytics-service
cargo test -p query-analytics-service
```

For the real PostgreSQL integration test set `TEST_DATABASE_URL` to an isolated test database. See the root README and `docs/system-design.md` for the local Kind workflow and detailed data contracts.

## Component ownership, prerequisites, and lifecycle

- **Owner:** `search-autocomplete-system` / `query-analytics-service`.
- **API, event, and data contract owners/producers/consumers:** see the [contract catalog](../docs/contracts/README.md) for each authoritative interface.
- **Parent architecture:** [System Design](../docs/system-design.md).

### Build prerequisites

Use this component’s pinned toolchain and lockfile/wrapper. The supported versions and complete local build environment are listed in the [root README](../README.md).

### Use prerequisites

This component is used as part of the parent system. Start its required local dependencies and use the supported local access path described in the [root README](../README.md).

### Build, verify, deploy, undeploy, and use

Build, run, and verification commands for this component are documented above. There is no independent release lifecycle for this component.
The parent Helm release owns deployment and removal; follow the [chart guide](../deploy/helm/search-autocomplete/README.md) and [root deployment lifecycle](../README.md). Uninstall removes application workloads while retained PVCs keep their data; deleting the namespace or claims purges persistent data.

[Documentation index](../docs/README.md)


## AI development disclaimer

> **AI development disclaimer:** This project was built entirely with GPT-6 Luna at Max effort as a proof of concept exploring how low-cost AI plans can be useful when paired with disciplined harness and loop engineering. This is project-owner attribution; repository contents do not independently verify runtime model metadata. Review AI-generated design and code before relying on them.
