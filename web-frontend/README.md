# Web Frontend

Vue 3 + TypeScript single-page client. The static build is served by Nginx in Kind; its same-origin API routes proxy suggestion reads and completed-query submissions to their owning backends.

> Documentation: [project index](../docs/README.md) · [repository overview](../README.md)

## Interaction contract

- Fetch `GET /api/v1/suggestions?prefix=...` as the user types.
- Show at most five results; Arrow Up/Down moves selection, Enter submits the selected suggestion or typed value, Escape closes the list.
- Show loading, empty, and service-error states.
- Submit a completed search as `POST /api/v1/query-events` with a fresh idempotency key. Typing alone is never recorded.
- The synthetic Kind sample data supports prefixes `ru` and `the sky is`; most other prefixes are empty until matching searches are recorded.

## Run and verify

Use the repository-pinned pnpm version:

```sh
corepack pnpm@10.33.2 install --frozen-lockfile
corepack pnpm@10.33.2 dev
corepack pnpm@10.33.2 test
corepack pnpm@10.33.2 build
```

For local Vite development, forward suggestion to `127.0.0.1:18080` and analytics to `127.0.0.1:18081`, or set `SUGGESTION_SERVICE_PROXY` and `ANALYTICS_SERVICE_PROXY`. For Playwright acceptance, set `E2E_BASE_URL` to the port-forwarded deployed frontend as shown in the root README.

## Browser agent support (WebMCP)

When the browser exposes the experimental WebMCP API, the page registers `get_search_suggestions`. It accepts a bounded prefix, reads the same `GET /api/v1/suggestions` contract as the visible autocomplete, and returns up to five suggestions. It never submits `POST /api/v1/query-events`, so an agent lookup does not record a completed search. Suggestion strings are untrusted content; the tool is read-only, scoped to this page, and unregistered when the component is removed. The ordinary search UI remains available when WebMCP is unsupported or registration fails. The implementation uses `webmcp-types` 0.1.9 for development-time types only; there is no runtime SDK or polyfill.

WebMCP remains browser-dependent and is a progressive enhancement. For local manual checks, follow Chrome's current WebMCP guide to enable the testing flag, relaunch the browser, and inspect the registered tool; production availability follows the current Chrome origin-trial/support requirements. Native browser-agent interoperability has not been verified for this app. Follow the current [Chrome WebMCP setup](https://developer.chrome.com/docs/ai/webmcp), [imperative API](https://developer.chrome.com/docs/ai/webmcp/imperative-api), [best practices](https://developer.chrome.com/docs/ai/webmcp/best-practices), and [security guidance](https://developer.chrome.com/docs/ai/webmcp/secure-tools) before changing the tool. See the [contract catalog](../docs/contracts/README.md#contracts) and [verification record](../docs/verification/lighthouse.md) for ownership and checks.

## Component ownership, prerequisites, and lifecycle

- **Owner:** `search-autocomplete-system` / `web-frontend`.
- **API, event, and data contract owners/producers/consumers:** see the [contract catalog](../docs/contracts/README.md) for each authoritative interface.
- **Parent architecture:** [System Design](../docs/system-design.md).

### Build prerequisites

Use this component’s pinned toolchain and lockfile/wrapper. The supported versions and complete local build environment are listed in the [root README](../README.md).

### Use prerequisites

This component is used as part of the parent system. Start its required local dependencies and use the supported local access path described in the [root README](../README.md).

### Build, verify, deploy, undeploy, and use

Build, run, and verification commands for this component are documented above. There is no independent release lifecycle for this component.
The parent Helm release owns deployment and removal; follow the [chart guide](../deploy/helm/search-autocomplete/README.md) and [root deployment lifecycle](../README.md). Uninstall removes application workloads while retained PVCs keep their data; deleting the namespace or claims purges persistent data.

[Documentation index](../docs/README.md)


## AI development disclaimer

> **AI development disclaimer:** This project was built entirely with GPT-6 Luna at Max effort as a proof of concept exploring how low-cost AI plans can be useful when paired with disciplined harness and loop engineering. This is project-owner attribution; repository contents do not independently verify runtime model metadata. Review AI-generated design and code before relying on them.
