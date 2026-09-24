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
