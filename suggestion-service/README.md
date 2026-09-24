# Suggestion Service

Rust/Axum bounded context responsible for validating suggestion prefixes and serving the first five indexed queries by popularity.

> Documentation: [project index](../docs/README.md) · [repository overview](../README.md)

## Contract

- `GET /api/v1/suggestions?prefix=...`: lowercase and collapse English whitespace; accept `a-z` and spaces up to 50 characters. Empty is valid and returns `[]`; invalid text receives `400`.
- `GET /healthz`: readiness and liveness check with the active snapshot revision.
- Refreshes `GET /internal/v1/snapshots/latest` from `index-builder-worker` every five seconds. Only newer snapshots replace the current index; failures leave the last valid index available.

The in-memory index contains up to five terms per exact beginning prefix, ordered by frequency descending and lexical term ascending. No SQL connection belongs to this service.

## Run and verify

```sh
cargo run -p suggestion-service
cargo test -p suggestion-service
```

Set `INDEX_BUILDER_URL` to the index builder's internal base URL when running outside Kubernetes. See the root README for the complete Kind flow and system design.

## Component ownership, prerequisites, and lifecycle

- **Owner:** `search-autocomplete-system` / `suggestion-service`.
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
