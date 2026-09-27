# Index Builder guidance

> Human guide: [README.md](../README.md) · [Documentation index](../docs/README.md)

- Read analytics only through its internal frequency API; never query or mutate its tables directly.
- Own snapshot schema, build and latest-snapshot operations only.
- Keep a source revision immutable. A duplicate revision must not create a second row or overwrite the established snapshot.
- Keep the snapshot revision aligned with the frequency response used to build it.
- Retain the last stored snapshot through refresh failures; do not publish partial data.
- Update this README and `docs/system-design.md` when lifecycle or API contracts change.
