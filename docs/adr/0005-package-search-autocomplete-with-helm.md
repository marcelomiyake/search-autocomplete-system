# ADR-0005: Package Search Autocomplete with Helm

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

The chart renders the frontend, backend Deployments and Services, and a PostgreSQL StatefulSet. Its values describe local replicas and resource budgets; scripts use a named Helm release on Kind.

The scope of this decision is Helm as the packaging and release interface for the local Kubernetes application. The source confirms the implementation; its historical selection rationale and original option set are not recorded.

## Decision drivers

- Keep workload manifests and configuration reviewable in one package.
- Make local installation, upgrade, and removal reproducible.
- Avoid maintaining separately copied Kubernetes YAML for the documented profile.

## Options considered

### Helm

- **Benefits:** The existing chart and values provide one release boundary for the local system.
- **Costs and risks:** Template behavior and persistent-claim lifecycle require careful review.

### Kustomize or raw manifests

- **Benefits:** Could avoid templating and make overlays directly visible.
- **Costs and risks:** Would replace the current release and values workflow.

### Docker Compose

- **Benefits:** Could simplify single-machine use.
- **Costs and risks:** Would no longer deploy the selected Kubernetes resources.

## Decision outcome

Retain Helm as the chart and release interface for this Kubernetes deployment. Uninstalling the application and deleting PostgreSQL data remain separate operations.

## Consequences

### Positive

- Replicas and resource budgets are configurable alongside templates.
- The release command path is repeatable for local verification.

### Negative and risks

- Template values can hide rendered behavior unless manifests are reviewed.
- Deleting a namespace or PVC can erase local query and snapshot data.

## Evidence and realization

- [Chart.yaml](../../deploy/helm/search-autocomplete/Chart.yaml)
- [values.yaml](../../deploy/helm/search-autocomplete/values.yaml)
- [templates](../../deploy/helm/search-autocomplete/templates)
- [README.md](../../README.md)

## Review triggers

- Reconsider if the deployment platform or release ownership changes.
- Update the ADR when chart storage retention or release commands change.

## References

- [search-autocomplete-system README](../../README.md)
- [System Design](../system-design.md)
- [ADR practices](https://adr.github.io/ad-practices/)
- [ADR template guidance](https://adr.github.io/adr-templates/)
