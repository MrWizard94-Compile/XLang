# M19c Design: nursery Policy B (cancel-destroy) — design only

**Status:** Accepted **design direction** — **implementation blocked** pending
preconditions below  
**Date:** 2026-08-04  
**Decision record:** [ADR-032](ADR-032-m19c-policy-b-design.md)  
**Depends on:** M19a `release`, M19b Policy A, M7 nurseries, M2 resources  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`

---

## 1. Purpose

Policy A forbids resourceful spawn callees. Policy B would allow spawn-local
owners and require **deterministic logical destroy** of those owners when a
spawn is cancelled or after first-failure abort of remaining work.

## 2. Why implementation is blocked now (honest)

| Precondition | Current law / product |
| --- | --- |
| Spawn callees can own M2 Arena/Buffer | **No** — M2 requires the single arena declaration in `main` |
| Spawn can pass access Arena | **No** — `AE-TASK-003` bans resource spawn arguments |
| Cancel targets unstarted spawns | Unstarted frames have **no** locals to destroy |
| Started spawns run to completion cooperatively | Their locals end with the spawn; parent does not inherit Buffer ownership |

Under these constraints, “cancel-destroy of spawn-local resources” has **no
admissible program** that differs from Policy A. Implementing free-on-cancel
without resourceful callees would be **fake safety theater**.

## 3. Target claim (when preconditions ship)

> When a nursery cancels an unstarted spawn, no live guest resource owner exists
> for that spawn. When a later ADR admits resourceful spawn callees (or
> spawn-local unique owners that outlive the call), cancel and first-failure
> paths must run the same logical destruction as root `release` (M19a) for every
> still-live owner local in that spawn frame—without free-on-raise for live
> parents (M19-INV-004).

## 4. Candidate future preconditions (each needs its own ADR)

1. **Arena-not-only-main** or **spawn-scoped arenas** (M2 expansion).  
2. **Buffer/table results** across weaves with ownership transfer rules.  
3. Explicit **spawn-frame owner table** in AETH for cancel-time release.  

## 5. Interim product rule (unchanged)

- Keep Policy A (M19b).  
- Keep site-level clean boundary + `release` for abortive effect (M19a).  
- Do **not** implement free-on-raise.

## 6. Package

**No package bump** for design-only M19c.

---

*End of DESIGN-M19C-POLICY-B-CANCEL-DESTROY.md*
