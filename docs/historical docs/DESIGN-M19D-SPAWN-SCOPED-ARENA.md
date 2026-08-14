# M19d Design: spawn-scoped / multi-weave arenas (T-RX precondition)

**Status:** Accepted implementable design for package **0.32**  
**Date:** 2026-08-04  
**Decision record:** [ADR-035](ADR-035-m19d-spawn-scoped-arena.md)  
**Supersedes (direction):** option **3** in [DESIGN-M19D-RESOURCEFUL-SPAWN-PRECONDITION.md](DESIGN-M19D-RESOURCEFUL-SPAWN-PRECONDITION.md)  
**Depends on:** M2 arenas, M7 nurseries, M19a `release`, M19b Policy A  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`, `SEC-INPUT-001`

---

## 1. Purpose

Policy B cancel-destroy is meaningless until spawn callees can own M2 resources.
This design admits **self-owned arenas in total non-main weaves** so spawn
callees may allocate buffers/tables without borrowing the parent's `access`
Arena, without free-on-raise, and without ambient allocation.

## 2. Product claim

> A **total** weave may declare **at most one** root `bind name <- arena N`
> (1 ≤ N ≤ 1_000_000). A program may declare **multiple** such arenas across
> weaves. The AETH header arena capacity is the **sum** of every declared `N`
> (must remain ≤ 1_000_000). Spawn callees may use M2/M6 resource forms when
> they are total and do not take Arena/Buffer/table/`access` parameters.
> Parent Policy A rules for live owners across `together` remain; exclusive
> `access` loans still cannot cross nursery sites. Seed≡bootstrap for the
> documented multi-arena / resourceful-spawn corpus.

## 3. Decisions

### D1 — Arena per total weave (not main-only)

| Rule | Detail |
| --- | --- |
| Max per weave | Exactly one `arena N` root bind |
| Which weaves | **Total** weaves only (`Error[Whole]` still cannot declare resource forms) |
| Non-main | Allowed (M2 main-only restriction lifted) |
| Nested blocks | Arena bind remains root-only (unchanged) |

### D2 — Capacity accounting (shared pool)

- AETH still carries **one** `u32` capacity field (v6–v11 header).  
- Capacity = **sum** of all arena literals in the program.  
- Runtime keeps a **single** shared pool (`ArenaState`) as today; each `OP_ARENA`
  pushes an Arena capability into that pool.  
- Mid-invocation capacity reclaim is **out of scope** (M2 deliberate limit).  
- Sequential resourceful spawns must fit in the summed static budget.

### D3 — Spawn policy (Policy A → A+)

| Rule | Policy A (0.26) | Policy A+ (0.32) |
| --- | --- | --- |
| Parent owns resources + nursery | yes | yes |
| Spawn args resource | **no** | **no** |
| Spawn callee resource forms | **no** | **yes** if total and no resource parameters |
| Access loan live at `together` | **no** | **no** |

Ordinary `call`/`handle`/`forward` to a total weave that owns its own arena
remains legal (symmetric with spawn). Resource **parameters** (`access Arena`,
owned Buffer, tables) on spawn callees stay forbidden.

### D4 — Logical end of spawn-local owners

Under cooperative M7:

1. **Unstarted cancel** — spawn frame never created → no live owners → destroy
   is a no-op (Policy B precondition satisfied vacuously).  
2. **Started total resourceful spawn** — runs to return; locals drop at frame
   end (logical end of owners).  
3. **No mid-frame cancel** of started spawns (unchanged M7).  
4. **No free-on-raise** for parent owners (M19-INV-004).

Explicit `release` remains available inside total weaves for early logical end.

### D5 — AETH / seed

- No new opcode.  
- Verifier: allow `OP_ARENA` outside main; require `arena_declarations ≥ 1` iff
  capacity > 0 and resource ops appear; capacity must match source sum.  
- Seed: parse non-main arenas; **accumulate** capacity into header field
  (today last-wins — must change to sum).  
- Dual-compare required for mix corpus.

### D6 — Diagnostics

| Code | Message family |
| --- | --- |
| AE-RESOURCE-001 | >1 arena per weave; capacity sum overflow; zero/oversized N |
| AE-TASK-003 | resource spawn **arguments**; erroring resourceful spawn callees |
| AE-EFFECT-003 | erroring weave declares resource forms (unchanged) |

## 4. Non-goals (this package)

- Nested nurseries  
- Mid-frame cancel of started spawns  
- Early arena capacity reclaim / free-list  
- Buffer/table results across weaves  
- Free-on-raise  
- Policy B marketing beyond proven cooperative cancel (see ADR-036)  

## 5. Example surface

```aether
world spawn_arena

weave worker [] -> Whole:
  bind memory <- arena 32
  bind mutable values <- buffer Whole
  bind mutable observed <- 0
  choose allocate access memory move values 1 into values:
    choose append move values 7 into values:
      choose at borrow values 0 into observed:
        yield observed
      otherwise:
        yield -3
    otherwise:
      yield -2
  otherwise:
    yield -1

weave main [] -> Whole:
  bind parent <- arena 16
  bind mutable out <- 0
  together:
    spawn call worker into out
  yield out
```

Header capacity = 32 + 16 = 48. Exit 7.

## 6. Package / claims

- Package **0.32.0**  
- CLM: multi-weave arena + resourceful total spawn callees proven; Policy B
  mid-frame cancel still not claimed.

---

*End of DESIGN-M19D-SPAWN-SCOPED-ARENA.md*
