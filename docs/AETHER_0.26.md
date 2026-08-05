# Aether 0.26 Toolchain Contract (M19b nursery × resource)

**Status:** Historical package 0.26 — language **0.11** surface / AETH **v11** + M19b Policy A; current package is [AETHER_0.27.md](AETHER_0.27.md)  
**Depends on:** ADR-028, M7, M2, M19a  

## Surface

No new keywords. Existing `together` / `spawn` may appear in a weave that also
binds M2/M6 resources when:

1. Every spawn callee is resource-free.  
2. Spawn arguments are non-resource (unchanged).  
3. No exclusive `access` loan is live at the `together` site.  

## Product path honesty

| Path | nursery + parent resources |
| --- | --- |
| Bootstrap | **Supported** (Policy A) |
| Seed default `compile` | **Supported** (bootstrap validate + seed emit; dual-compare mix) |

## Example

`examples/nursery-resource.ae` — arena in main, pure spawns, exit 7.

## Non-goals

Policy B cancel-destroy, resourceful spawn callees, free-on-raise, nested nurseries.

*End of AETHER_0.26.md*
