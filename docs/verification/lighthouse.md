# Lighthouse and SEO META Verification

This record audits the built search autocomplete frontend and checks its browser metadata against the published SEO META in 1 CLICK field list. Lighthouse measures this browser run; it is separate from SonarQube analysis and JEV readiness evaluation.

## Contents

- [Scope](#scope)
- [Lighthouse results](#lighthouse-results)
- [Agentic Browsing](#agentic-browsing)
- [SEO META checklist](#seo-meta-checklist)
- [Findings](#findings)
- [Related documentation](#related-documentation)

## Scope

- **Date/time:** 2026-09-25 12:31:45 America/Sao_Paulo.
- **Source:** dirty worktree based on revision 84eb6c7.
- **Environment:** Linux; Google Chrome 154.0.8037.57; Lighthouse CLI 13.5.0; default mobile emulation and simulated throttling.
- **Target:** production Vite preview at http://127.0.0.1:4173/; one app preview ran at a time and was stopped before the next.
- **Commands:** `corepack pnpm@10.33.2 run build`; `npx --yes lighthouse http://127.0.0.1:4173/ --output=json --output-path=/tmp/lighthouse-search.json --only-categories=performance,accessibility,best-practices,seo --chrome-flags='--headless --no-sandbox' --quiet`.
- The suggestion and analytics services were not deployed; this record covers the built browser frontend, not the live suggestion flow.

## Lighthouse results

| Performance | Accessibility | Best practices | SEO |
| ---: | ---: | ---: | ---: |
| 100 | 95 | 100 | 63 |

Scores range from 0 to 100. The SEO score reflects the intentional noindex directive; the accessibility finding is described under Findings.

## Agentic Browsing

Lighthouse 13.5.0 on Chrome 154.0.8037.57 ran `npx --yes lighthouse http://127.0.0.1:4173/ --output=json --output-path=/tmp/lighthouse-search-agentic.json --only-categories=agentic-browsing --chrome-flags='--headless --no-sandbox' --quiet` at 2026-09-25 12:50:58 America/Sao_Paulo. Its experimental fractional result was 0.50; this is not a 0–100 score. The accessibility-tree and layout-stability audits passed. The root `llms.txt` and `ai-catalog.json` discovery checks failed because no public agent catalog is shipped for this local demo. WebMCP form coverage, registered tools, and schema validity were reported as not applicable: this run did not use the WebMCP origin trial, so it did not verify native registration of `get_search_suggestions`.

## SEO META checklist

| Field | Result |
| --- | --- |
| HTML language | English is declared. |
| Title and length | Present: “Findly — search autocomplete” (28 characters). |
| Description and length | Present: “A local search autocomplete system design MVP.” (46 characters). |
| Robots metadata | noindex, nofollow is intentional for this local demo. A valid robots.txt is served with Allow: / so crawlers can read the page directive. |
| Canonical URL | Omitted because the local preview/deployment origin is not a stable public URL. |
| Headings | One H1; no heading-level gaps on the landing page. |
| Images and alt text | No HTML image elements; this check is not applicable. |
| Links | One labeled internal anchor in the default view; one unique target and no external links. |
| Favicon | SVG favicon is declared and served. |
| Open Graph, Twitter, and sitemap | Not provided because the app has no public share URL or public indexing target. |

The SEO META in 1 CLICK extension was not installed in the browser. This is a manual checklist audit against its published fields, not a claim that the extension itself ran.

## Findings

- Lighthouse confirmed the title, description, valid robots.txt, and page metadata. It reports the page as not crawlable by design because of the noindex directive.
- Accessibility scored 95; Lighthouse found low-contrast text on the landing page.
- Best practices scored 100. This frontend-only preview did not validate API-backed suggestions or completed-query recording.
- `corepack pnpm@10.33.2 exec vitest run src/services/webmcp.spec.ts` passed: 2 focused tool tests. Native browser-agent interoperability was not exercised.

## Related documentation

- [Verification index](README.md)
- [Web frontend guide](../../web-frontend/README.md)
- [SonarQube verification](sonarqube.md)
- [JEV readiness assessment](jev-readiness.md)
- [Chrome Lighthouse overview](https://developer.chrome.com/docs/lighthouse/overview)
- [Lighthouse Agentic Browsing scoring](https://developer.chrome.com/docs/lighthouse/agentic-browsing/scoring)
- [SEO META in 1 CLICK listing](https://chromewebstore.google.com/detail/seo-meta-in-1-click/bjogjfinolnhfhkbipphpdlldadpnmhc?hl=en-GB)
