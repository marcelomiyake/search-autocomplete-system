# SonarQube Cloud verification

Each Rust backend service has a dedicated SonarQube Cloud project. The [GitHub Actions workflow](https://github.com/marcelomiyake/search-autocomplete-system/actions/workflows/sonarcloud-microservices.yml) runs one coverage and analysis job per service on pushes to `main`; `workflow_dispatch` supports a manual rerun. Organization-level auto-import of newly created GitHub repositories is disabled, so this workflow owns analysis.

Each job runs `cargo llvm-cov --lcov --output-path target/coverage/lcov.info -- --test-threads=1` from the selected service directory, then imports the LCOV report with `cargo sonar-scanner`. The workflow reads this GitHub repository's `SONAR_TOKEN` secret and provisions PostgreSQL for integration tests. The `src/main.rs` startup entrypoints are excluded from the coverage denominator. No local Sonar scan script is used.

## Projects and coverage

Coverage below is SonarCloud's overall line coverage for `main`, not new-code or local coverage. The baseline is the latest Cloud result before the coverage-test updates; the current column is the latest result after them. Values are from the 2026-09-27 snapshot.

| Microservice | SonarCloud project | Before | Current | Change |
| --- | --- | ---: | ---: | ---: |
| `suggestion-service` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_suggestion-service) | 100.0% | 100.0% | — |
| `query-analytics-service` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_query-analytics-service) | 100.0% | 100.0% | — |
| `index-builder-worker` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_index-builder-worker) | 99.2% | 99.3% | +0.1 pp |

All three projects have passing Quality Gates, zero open or confirmed issues, zero hotspots awaiting review, zero bugs, zero vulnerabilities, zero code smells, and 0.0% duplicated lines. The complete project map is also in [`sonar-projects.md`](../../sonar-projects.md).

## Verification

Confirm that the latest workflow completed for the pushed commit and each project's `main` analysis matches that revision. Review coverage, active issues, security hotspots, duplication, and the Quality Gate. Project links above open the live dashboards; the workflow link shows the run history. Local test results do not replace a completed Cloud analysis.
