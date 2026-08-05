# M19b Design: nursery × resource (Policy A)

**Status:** Accepted design for ADR-028 — implemented in package 0.26.0  
**Date:** 2026-08-04  
**Depends on:** M7 nurseries, M2 resources, M19a `release` (ADR-027), ADR-023  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`

---

## 1. Purpose

M19a proved explicit `release` so abortive sites can follow cleanup. M7 still
banned **any** co-occurrence of nursery control and M2 resources in one weave
(`AE-TASK-003`), which blocked legitimate total parents that hold an arena while
running pure spawns.

**M19b** admits a **fail-closed Policy A** mix:

> A weave may own M2/M6 resources **and** use `together` when every spawn
> callee is resource-free and spawn arguments remain non-resource. Live resource
> owners may remain across the nursery; exclusive `access` loans may not.
> Cancel of unstarted spawns never orphans spawn-local resource owners (none
> exist under this policy). Abortive sites still require M19a clean boundary.

## 2. Policy choice

| Option | Summary | Decision |
| --- | --- | --- |
| **A** | Forbid resource ops in spawn callees; parent may hold resources | **Adopted** |
| **B** | Cancel runs deterministic destroy of spawn-local owners | Deferred (needs richer cancel model) |

Policy A needs no new opcodes and preserves M19-INV-001 without free-on-raise.

## 3. Rules

### R1 — Parent may mix

Remove the weave-level ban `weave_uses_resource ∧ weave_uses_nursery`.

### R2 — Spawn callees resource-free

For each `spawn call W …`, weave `W` must not use M2/M6 resource forms
(`weave_uses_resource(W)` is false). Host weaves remain non-resource guest
bodies.

### R3 — Spawn arguments (unchanged)

No Arena/Buffer/access/table arguments across spawn (existing `AE-TASK-003`).

### R4 — Nursery site boundary

At `together`, reject live **exclusive access loans** only (same family as M16
handle boundary). Live unique owners and resource owners may remain.

Abortive `raise`/`forward` keep the full clean boundary (M4/M19a).

### R5 — Seed / dual-compare

No seed surface change required (seed already emits nurseries and resources
independently). Bootstrap validation is the gate; product `compile_with_seed`
still validates via bootstrap AST before forge. Dual-compare the mix corpus.

## 4. Non-goals

- Nested nurseries  
- Resource-carrying spawn callees or free-on-cancel (Policy B)  
- Abortive raise without `release` of live owners  
- FFI  

## 5. Package / claims

- Package **0.26.0** (language surface still 0.11 forms + tooling; AETH **v11**).  
- CORE_CLAIMS CLM-033: Proven now, bounded M19b Policy A.

---

*End of DESIGN-M19B-NURSERY-RESOURCE.md*
