# SonarQube Cloud projects

The backend services are separate projects in one SonarQube Cloud monorepo. The push-triggered GitHub Actions matrix runs one Rust test, coverage, and analysis job per service. Organization-level auto-import of newly created GitHub repositories is disabled. The full before/after coverage record and verification checks are in [the SonarQube guide](docs/verification/sonarqube.md).

| Service | Source directory | SonarCloud project | Overall coverage (2026-09-27) |
| --- | --- | --- | ---: |
| `suggestion-service` | `suggestion-service` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_suggestion-service) | 100.0% |
| `query-analytics-service` | `query-analytics-service` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_query-analytics-service) | 100.0% |
| `index-builder-worker` | `index-builder-worker` | [project](https://sonarcloud.io/project/overview?id=marcelomiyake_search-autocomplete-system_index-builder-worker) | 99.3% |

Each job uses this GitHub repository's `SONAR_TOKEN` secret and imports that service's LCOV report. The `main` branch has zero open or confirmed issues and zero hotspots awaiting review across the three projects; all three Quality Gates pass. See [the SonarQube verification guide](docs/verification/sonarqube.md) for the prior coverage baseline and validation details.
