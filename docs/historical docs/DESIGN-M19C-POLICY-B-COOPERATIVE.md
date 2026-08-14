# M19c Design: Policy B cooperative (product-bounded)

**Status:** Accepted with ADR-036 after M19d precondition  
**Date:** 2026-08-04  
**Depends on:** ADR-035 multi-weave arenas  

## Claim

> For total resourceful spawn callees under cooperative nurseries, cancel of
> unstarted spawns is destroy-clean by construction; started spawns complete and
> drop locals at return.

## Non-goals

Mid-frame cancel, erroring resourceful spawns, free-on-raise, capacity reclaim.

## Relationship to ADR-032

ADR-032 correctly blocked **theater**. This document ships only the cooperative
vacuous/return-end destroy story once M19d admits resourceful total callees.

---

*End of DESIGN-M19C-POLICY-B-COOPERATIVE.md*
