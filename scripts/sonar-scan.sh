#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -z "${SONAR_TOKEN:-}" ]]; then
  printf 'SONAR_TOKEN is not set. Supply the configured SonarQube analysis token in the environment.\n' >&2
  exit 2
fi

if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  printf 'cargo-llvm-cov is required to generate SonarQube Rust coverage reports. Install it with: cargo install cargo-llvm-cov\n' >&2
  exit 2
fi

cd "$ROOT"
printf 'Generating Vue/TypeScript LCOV coverage\n'
(cd web-frontend && corepack pnpm@10.33.2 test:coverage)

for project in suggestion-service query-analytics-service index-builder-worker; do
  printf 'Generating Rust LCOV coverage for %s\n' "$project"
  mkdir -p "$project/coverage"
  cargo llvm-cov --package "$project" --ignore-filename-regex 'src/main\.rs$' \
    --fail-under-lines 80 --lcov --output-path "$ROOT/$project/coverage/lcov.info"
  python3 - "$ROOT/$project" "$ROOT/$project/coverage/lcov.info" <<'PY'
from pathlib import Path
import sys

project_root = Path(sys.argv[1]).resolve()
report_path = Path(sys.argv[2])
rewritten = []
for line in report_path.read_text().splitlines():
    if line.startswith('SF:'):
        source = Path(line[3:])
        if not source.is_absolute():
            source = project_root / source
        relative = source.resolve().relative_to(project_root)
        line = f'SF:{relative.as_posix()}'
    rewritten.append(line)
report_path.write_text('\n'.join(rewritten) + '\n')
PY
  printf 'Checking Rust lints for %s\n' "$project"
  cargo clippy --package "$project" --all-targets --message-format=json -- -D warnings \
    > "$ROOT/$project/coverage/clippy.json"
done

SCANNER_IMAGE="sonarsource/sonar-scanner-cli@sha256:a3f4215076706c95a17a68c19322ee916e40a3acd081a8c1a1e839e0194afa57"
for project in web-frontend suggestion-service query-analytics-service index-builder-worker; do
  printf 'Analyzing %s\n' "$project"
  docker run --rm --network=host \
    -e SONAR_TOKEN -e SONAR_HOST_URL=http://127.0.0.1:9000 \
    -v "$ROOT/$project:/usr/src" -w /usr/src "$SCANNER_IMAGE"
done
