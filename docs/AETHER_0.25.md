# Aether 0.25 Toolchain Contract (M19a explicit `release`)

**Status:** Current package — language **0.11** / AETH **v11** + `RELEASE` (66)  
**Depends on:** ADR-027, M2, M4, M16  

## Surface

```aether
release <name>
```

Logically destroys a live unique owner (Text/Bytes/Record) or resource owner
(Buffer/Table/Arena with no live dependents). Marks the binding moved.

After release, abortive `raise`/`forward` may pass the clean boundary.

## Product path honesty

| Path | `release` |
| --- | --- |
| Bootstrap (`compile --bootstrap`) | **Supported** |
| Seed default `compile` | **Supported** (seed≡bootstrap for `examples/release-raise.ae`) |

## Example

`examples/release-raise.ae` — handled error code 9 after releasing Text.

## Non-goals

Nursery×resource, free-on-raise, user destructors.

*End of AETHER_0.25.md*
