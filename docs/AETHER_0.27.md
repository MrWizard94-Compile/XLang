# Aether 0.27 Toolchain Contract (M20b stdlib layer 1)

**Status:** Historical package 0.27 — language **0.11** / AETH **v11** + stdlib layer 1; current package is [AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** ADR-029, M20, M11  

## Surface

Offline pure modules under `stdlib/`:

| Unit | Exports |
| --- | --- |
| `whole.ae` | `double`, `inc`, `is_zero`, `dec`, `abs`, `max`, `min`, `clamp_nonneg` |
| `truth.ae` | `invert`, `both` |
| `text.ae` | `text_len`, `text_empty` |

Demo: `stdlib/main.ae` → `call wh.double 21` → exit **42**.

## Product path honesty

| Path | Stdlib |
| --- | --- |
| `project build stdlib/…` | Seed elaborates + dual-compares units |
| Host I/O stdlib | **Not** product |

## Non-goals

Registry distribution, grant-backed I/O helpers, resource libs.

*End of AETHER_0.27.md*
