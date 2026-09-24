# Search Autocomplete Helm chart

This chart deploys the suggestion service, query analytics service, index builder, browser frontend, and PostgreSQL to the local `search-autocomplete` namespace.

> Documentation: [project index](../../../docs/README.md) · [repository overview](../../../README.md)

## Contents

- [Build and use prerequisites](#build-and-use-prerequisites)
- [Prerequisites and deploy](#prerequisites-and-deploy)
- [Resource budgets and persistence](#resource-budgets-and-persistence)
- [Undeploy and use](#undeploy-and-use)
- [AI development disclaimer](#ai-development-disclaimer)

## Build and use prerequisites

- **Build/deploy:** use the root project README for source-image build commands and the chart instructions below for the Helm release. Required tools are Docker, the supported local Kubernetes cluster/context, kubectl, and Helm.
- **Use:** the cluster release must be ready; use the loopback browser/port-forward instructions in the root README. See the resource table for each pod container budget.


## Prerequisites and deploy

Use the existing Kind cluster with context `kind-kind`, Docker, Kind, kubectl, Helm, and the pinned pnpm toolchain. The root [README](../../../README.md) documents build prerequisites and the complete deploy/use flow. Run from the repository root:

```sh
./scripts/deploy-kind.sh
```

## Resource budgets and persistence

Every workload container has CPU and memory requests and limits. Exact per-container values are listed in the [Kubernetes resource budget](../../../docs/kubernetes-resources.md) and configured in [`values.yaml`](values.yaml). PostgreSQL uses a 2Gi PVC and the StatefulSet retains its claim on deletion or scale-down. Removing the chart does not back up or delete persisted data; deleting the namespace or PVC does.

## Undeploy and use

```sh
helm uninstall search-autocomplete --namespace search-autocomplete
```

Port-forward the frontend as shown in the root README and open `http://127.0.0.1:4173`. See the [System Design](../../../docs/system-design.md), [contracts](../../../docs/contracts/README.md), and [documentation index](../../../docs/README.md).


## AI development disclaimer

> **AI development disclaimer:** This project was built entirely with GPT-6 Luna at Max effort as a proof of concept exploring how low-cost AI plans can be useful when paired with disciplined harness and loop engineering. This is project-owner attribution; repository contents do not independently verify runtime model metadata. Review AI-generated design and code before relying on them.
