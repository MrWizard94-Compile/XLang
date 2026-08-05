# ADR-036: Policy B cooperative cancel for total resourceful spawns

**Status:** Accepted — **bounded product claim after M19d (package 0.32)**  
**Date:** 2026-08-04  
**Related:** ADR-032 (design-only history), ADR-035 (precondition)  

## Context

ADR-032 blocked Policy B product claims because no resourceful spawn programs
existed. ADR-035 admits total resourceful spawn callees with self-owned arenas.

Under **cooperative** M7 (source-order start; first-failure cancels only
**unstarted** spawns; started spawns run to completion):

| Event | Live spawn-local owners | Required destroy |
| --- | --- | --- |
| Cancel unstarted resourceful spawn | None (frame not created) | No-op |
| Complete total resourceful spawn | End at frame return | Locals drop (existing VM) |
| Parent live owners across nursery | Parent-owned | Unchanged Policy A+ / M19a |

## Decision

1. **Product claim (bounded):** cancel of unstarted resourceful total spawns
   never leaves spawn-local M2 owners (none exist). Completed total resourceful
   spawns end owners at return.  
2. **Do not claim** mid-frame cancel, parallel runtimes, or free-on-raise.  
3. Keep ADR-032 historical design text; this ADR is the cooperative product slice.  
4. No new opcodes for this claim.

## Honesty

This is **not** a general free-on-cancel engine. It is the only Policy B surface
admissible under current M7 + M19d total callees.

---

*End of ADR-036.*
