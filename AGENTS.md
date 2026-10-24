# Repository guidance

> Human guide: [README.md](README.md) · [Documentation index](docs/README.md)

- Keep the system educational and local-first; do not present Kind measurements as production capacity.
- Keep query analytics, index building, suggestions, and the browser UI in their own service directories and data boundaries.
- Prefer small, explicit Rust modules and Vue components. Add abstractions only where the current use case needs them.
- Keep sample queries synthetic. Never add personal or real user query data, credentials, or access tokens.
- Every service folder must keep its README.md and AGENTS.md current when its interface or workflow changes.
- Add and maintain unit, PostgreSQL integration, component, and browser acceptance tests for the behavior they cover.
- Record verification commands, commit revision, environment, and results under docs/verification/.
- Follow Conventional Commits 1.0.0 for commit messages.

## Releases

`.github/workflows/release.yml` runs [release-please](https://github.com/googleapis/release-please) on every push to `main`. From the Conventional Commits merged since the last tag, it keeps one release pull request open, titled `chore: release X.Y.Z`:

- `fix`, `perf` and `revert` make a patch release, `feat` a minor release, and a breaking change (`!`) a major release. The other types release nothing on their own and stay out of the changelog.
- It writes the version to `version.txt` and adds the release to `CHANGELOG.md`.
- Merging it tags `vX.Y.Z` and publishes the GitHub release.
- GitHub runs no workflow on that pull request, because the workflow's own token opened it. The other workflows run on `main` once it merges.

Never change the version or `CHANGELOG.md` by hand. `.release-please-manifest.json` holds the last released version, and `release-please-config.json` the settings. To force a version, add a `Release-As: X.Y.Z` footer to a commit.
