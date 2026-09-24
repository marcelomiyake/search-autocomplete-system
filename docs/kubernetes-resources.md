# Kubernetes CPU and memory budgets

The chart applies the service-specific resource values in `values.yaml` to each pod. Each listed Deployment has two replicas; budgets below are per pod container and are local defaults, not measured consumption or production sizing claims.


> Project documentation index: [Documentation index](README.md)


## Workload budgets

| Pod/workload and container | CPU request | Memory request | CPU limit | Memory limit | Source |
| --- | ---: | ---: | ---: | ---: | --- |
| `suggestion-service` / `suggestion-service` | 50m | 64Mi | 500m | 384Mi | [Chart values](../deploy/helm/search-autocomplete/values.yaml) |
| `query-analytics-service` / `query-analytics-service` | 50m | 64Mi | 500m | 384Mi | [Chart values](../deploy/helm/search-autocomplete/values.yaml) |
| `index-builder-worker` / `index-builder-worker` | 50m | 64Mi | 500m | 384Mi | [Chart values](../deploy/helm/search-autocomplete/values.yaml) |
| `web-frontend` / `web-frontend` | 50m | 64Mi | 500m | 384Mi | [Chart values](../deploy/helm/search-autocomplete/values.yaml) |
| `postgres` / `postgres` | 100m | 128Mi | 1 | 512Mi | [Chart values](../deploy/helm/search-autocomplete/values.yaml) |

PostgreSQL is one StatefulSet pod; it requests a 2Gi PVC separately. The chart has no init containers or sidecars.

## Kubernetes resource management practice

Set CPU and memory `requests` and `limits` on every container in every Pod, including init containers and sidecars. Requests guide scheduling and reserve baseline capacity; CPU limits may throttle, and memory limits can trigger OOM termination. Measure representative usage, leave startup/burst headroom, monitor throttling and restarts, and right-size deliberately. Treat the listed values as local development defaults, not production sizing guidance. PVC storage requests are separate from container budgets.

## Build, use, and persistence

The [root README](../README.md) documents build, deploy, use, and undeploy commands with their persistent-data effects. The [Helm chart guide](../deploy/helm/search-autocomplete/README.md) is the deployment owner. This resource inventory is configuration guidance and does not itself deploy workloads.
