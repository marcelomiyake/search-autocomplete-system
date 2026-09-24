.PHONY: test test-rust test-frontend build-frontend lint-helm deploy-kind

test: test-rust test-frontend

test-rust:
	cargo test --workspace

test-frontend:
	cd web-frontend && corepack pnpm@10.33.2 test && corepack pnpm@10.33.2 build

build-frontend:
	cd web-frontend && corepack pnpm@10.33.2 build

lint-helm:
	helm lint deploy/helm/search-autocomplete

deploy-kind:
	./scripts/deploy-kind.sh
