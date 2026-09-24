#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLUSTER_CONTEXT="$(kubectl config current-context)"
if [[ "$CLUSTER_CONTEXT" != "kind-kind" ]]; then
  printf 'Expected kubectl context kind-kind, got %s\n' "$CLUSTER_CONTEXT" >&2
  exit 1
fi
if ! kind get clusters | grep -qx 'kind'; then
  printf 'Kind cluster "kind" is not available.\n' >&2
  exit 1
fi

cd "$ROOT"
docker build -f Dockerfile.backends --target suggestion-service -t search-autocomplete/suggestion-service:dev .
docker build -f Dockerfile.backends --target query-analytics-service -t search-autocomplete/query-analytics-service:dev .
docker build -f Dockerfile.backends --target index-builder-worker -t search-autocomplete/index-builder-worker:dev .
docker build -f web-frontend/Dockerfile -t search-autocomplete/web-frontend:dev web-frontend
kind load docker-image search-autocomplete/suggestion-service:dev --name kind
kind load docker-image search-autocomplete/query-analytics-service:dev --name kind
kind load docker-image search-autocomplete/index-builder-worker:dev --name kind
kind load docker-image search-autocomplete/web-frontend:dev --name kind
helm lint deploy/helm/search-autocomplete
helm upgrade --install search-autocomplete deploy/helm/search-autocomplete \
  --kube-context "$CLUSTER_CONTEXT" --namespace search-autocomplete --create-namespace
# The local `dev` tag is mutable; recreate pods to ensure they use the images just loaded into Kind.
kubectl rollout restart --context "$CLUSTER_CONTEXT" -n search-autocomplete \
  deployment/suggestion-service deployment/query-analytics-service \
  deployment/index-builder-worker deployment/web-frontend
kubectl rollout status --context "$CLUSTER_CONTEXT" -n search-autocomplete statefulset/postgres --timeout=180s
for deployment in suggestion-service query-analytics-service index-builder-worker web-frontend; do
  kubectl rollout status --context "$CLUSTER_CONTEXT" -n search-autocomplete "deployment/$deployment" --timeout=180s
done
kubectl get deployments,statefulsets,pods --context "$CLUSTER_CONTEXT" -n search-autocomplete
