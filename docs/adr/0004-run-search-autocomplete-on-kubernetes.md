# ADR-0004: Run Search Autocomplete on Kubernetes

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

The system is deployed as Kubernetes Deployments, Services, and a PostgreSQL StatefulSet in a local Kind cluster. The chart runs multiple application replicas; PostgreSQL is a single local replica. The original deployment platform selection record was not found.

The scope of this decision is Kubernetes on local Kind for the frontend, backend services, and PostgreSQL. The source confirms the implementation; its historical selection rationale and original option set are not recorded.

## Decision drivers

- Exercise service discovery and multiple stateless application replicas.
- Keep PostgreSQL state and application workloads represented as distinct Kubernetes resources.
- Set CPU and memory requests and limits for pod containers.

## Options considered

### Kubernetes on local Kind

- **Benefits:** Matches current workload templates and the local deployment verification workflow.
- **Costs and risks:** Requires a local cluster and storage; the single database and cluster do not provide HA.

### Docker Compose

- **Benefits:** Simpler for local startup and teardown.
- **Costs and risks:** Would not exercise current Services, Deployments, StatefulSet, or claim behavior.

### Direct host processes

- **Benefits:** Minimizes orchestration setup.
- **Costs and risks:** Would not reproduce the network, replica, and persistent-volume configuration.

## Decision outcome

Retain Kubernetes as the local application deployment model, using Kind for development. This is a functional local profile, not proof of production scaling or availability.

## Consequences

### Positive

- Application replicas and persistent database state have explicit workload boundaries.
- Readiness and resource configuration can be checked with the deployed resources.

### Negative and risks

- Kind shares host resources and has a constrained failure domain.
- Kubernetes introduces local setup and storage troubleshooting costs.

## Evidence and realization

- [templates](../../deploy/helm/search-autocomplete/templates)
- [kubernetes-resources.md](../kubernetes-resources.md)
- [system-design.md](../system-design.md)
- [README.md](../../README.md)

## Review triggers

- Reconsider if the project adopts a different target platform or removes Kubernetes-specific deployment requirements.
- Before remote deployment, define security, backup, storage, and capacity controls.

## References

- [search-autocomplete-system README](../../README.md)
- [System Design](../system-design.md)
- [ADR practices](https://adr.github.io/ad-practices/)
- [ADR template guidance](https://adr.github.io/adr-templates/)
