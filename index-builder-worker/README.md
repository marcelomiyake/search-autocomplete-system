# Index Builder Worker

Rust worker and internal Axum endpoint responsible for turning analytics aggregates into immutable serving snapshots. It owns the `index_builder` PostgreSQL schema.

> Documentation: [project index](../docs/README.md) · [repository overview](../README.md)

## Contract and lifecycle

- Poll `GET /internal/v1/frequencies` on `query-analytics-service`; local default is every 15 seconds (`REFRESH_SECONDS`).
- Store `{revision, terms}` in `index_builder.snapshots`, keyed by source revision. Rebuilding the same revision is a no-op.
- `GET /internal/v1/snapshots/latest`: return the highest persisted revision or `404` before an initial build.
- `GET /healthz`: check PostgreSQL connectivity.

On a source or database failure the worker logs the failure and retries next interval. It never deletes the last successful snapshot.

## Run and verify

```sh
DATABASE_URL='postgres://...' QUERY_ANALYTICS_URL='http://127.0.0.1:8081' cargo run -p index-builder-worker
cargo test -p index-builder-worker
```

For PostgreSQL integration tests set `TEST_DATABASE_URL` to an isolated test database. See the root README and `docs/system-design.md` for the complete Kind flow.

## Component ownership, prerequisites, and lifecycle

- **Owner:** `search-autocomplete-system` / `index-builder-worker`.
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
