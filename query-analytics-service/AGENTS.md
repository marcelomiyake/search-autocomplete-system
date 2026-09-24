# Query Analytics Service guidance

> Human guide: [README.md](../README.md) · [Documentation index](../docs/README.md)

- This bounded context exclusively owns `query_analytics` tables, migrations, and event/frequency mutations.
- Count explicit completed searches only; autocomplete reads are not analytics events.
- Keep idempotency insert, frequency update, and source revision advancement atomic.
- Do not log query values or include raw user search data in verification or model-evaluation evidence.
- Sample queries must remain synthetic and seeded with fixed idempotency keys.
- Update this README and `docs/system-design.md` when contracts or persistence semantics change.
