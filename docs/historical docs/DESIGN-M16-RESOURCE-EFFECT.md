# M16 Design: resource ↔ effect interaction (T-RX)

**Status:** Accepted design for ADR-020 — **implemented in package 0.21.0**  
**Date:** 2026-08-04  
**Decision record:** [ADR-020](ADR-020-m16-resource-effect.md)  
**Validation:** [M16 validation matrix](M16-VALIDATION-MATRIX.md)  
**Depends on:** M2 arenas/buffers ([ADR-004](ADR-004-aeth-v6-bounded-resources.md)), M4 effects ([ADR-007](ADR-007-m4-typed-error-effect.md)), M6 tables, M7 nurseries  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) **T-RX**  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`

---

## 1. Purpose and boundary

M4 and M7 deliberately **forbid** mixing M2/M6 resource owners with effect
control or nurseries in the same weave. That “clean boundary” avoided proving
cleanup under abort. Real programs need to **handle** errors while holding an
arena or buffer (e.g. parse input after allocating working storage).

**M16** is the first controlled relaxation: **terminal `handle call` may appear
in a total weave that also owns arenas/buffers/tables**, because handle
**discharges** `Error[Whole]` without unwinding past those owners.

Non-goals for M16: abortive `raise`/`forward` while resources are live;
resource-carrying error payloads; `raises Whole` weaves that allocate; nursery +
resource composition; automatic `drop`/defer; resumption/effects beyond M4.

---

## 2. Core claim

> A total weave may own M2/M6 resource state and use terminal `handle call` to
> discharge a resource-free, copy-only `Error[Whole]` callee. Live Arena,
> Buffer, table, Text, Bytes, and record owners may remain live across the
> handle. Exclusive `access` loans must not span a handle. Abortive effect
> control (`raises` / `raise` / `forward`) and nurseries remain incompatible
> with resource ownership in the same weave.

---

## 3. Design decisions

### D1 — Split effect sites by abortivity

| Site | Resource-owning weave | Live owners at site |
| --- | --- | --- |
| `handle call` (total caller) | **Allowed** | Arena/Buffer/table/Text/Bytes/record **allowed**; `access` loan **forbidden** |
| `raise` / `forward call` | **Forbidden** (weave-level) | Full clean boundary retained |
| `raises Whole` signature | **Forbidden** with any resource form in body/params beyond existing copy-only rules | — |
| `together` nursery | **Forbidden** with resources (unchanged M7) | Full clean boundary |

### D2 — Why handle + live resources is sound

1. Handle is **total**: both branches rejoin the same weave; no guest stack
   unwind past the arena.  
2. Logical destruction of arena/buffer/table still occurs at **lexical owner
   end** (M1/M2), after both branches.  
3. The callee remains **copy-only** and **resource-free** (M4); it cannot leave
   orphan guest resources.  
4. Exclusive `access` is operation-scoped and must not outlive a single
   resource op (existing M2); spanning a handle would violate that.

### D3 — Weave-level ban rewrite

**Before (M4):** any resource form + any of raise/forward/handle → reject.

**After (M16):**

```text
if resource_owning_weave && (raise | forward | raises-body-allocation) → AE-EFFECT-003
if resource_owning_weave && nursery → AE-TASK-003   # unchanged
# handle alone does not trigger the mix ban
```

Concrete checks:

1. `weave_uses_resource(w) && weave_uses_abortive_effect(w)` → `AE-EFFECT-003`  
   where abortive = `raise` or `forward` in body.  
2. `w.effect == ErrorWhole && weave_uses_resource(w)` → `AE-EFFECT-003`  
3. Site rule for `handle`: `validate_handle_boundary` rejects only live
   `AccessArena` (and any future exclusive loan kinds).  
4. Site rule for `raise`/`forward`/`together`: keep full
   `validate_effect_boundary` (no live unique owner, Arena, Buffer, table).

### D4 — Verifier

Split verifier helper:

- **Abortive boundary** (RAISE, FORWARD_CALL, NURSERY_*): unchanged — no live
  unique/Arena/Buffer.  
- **Handle boundary** (HANDLE_CALL): empty stack after args; reject live
  `AccessArena` locals only; allow Arena/Buffer/table/Text/Bytes/record.

### D5 — AETH / seed

- **No new opcodes.**  
- Seed emits the same HANDLE_CALL sequences; dual-compare a positive mix
  example.  
- Bootstrap remains diagnostic authority for invalid mixes.

### D6 — Diagnostics

| Code | M16 meaning |
| --- | --- |
| `AE-EFFECT-003` | Abortive effect or erroring weave mixed with resources; or handle spans `access` loan |
| `AE-TASK-003` | Nursery still incompatible with resources |
| `AE-EFFECT-001/002/004` | Unchanged M4 route/type rules |

### D7 — Package pin

Suggested **0.21.0** at implementation ship. Document in `AETHER_0.21.md`.

### D8 — Stop conditions

- Need for `defer`/finalizers to meet the slice  
- Error payloads that own resources  
- Resource ops inside erroring callees  
- Nursery cancel while holding buffers  
- Access loan surviving handle  

---

## 4. Examples

### Positive

```aether
world resource_handle

weave ok [] -> Whole raises Whole:
  yield 7

weave main [] -> Whole:
  bind memory <- arena 64
  bind mutable success <- 0
  bind mutable code <- 0
  handle call ok into success otherwise error into code
```

Expected exit: `7` (terminal handle selects the success destination). Arena
remains live across handle; destroyed at end of `main`.

### Negative

```aether
# abortive raise with live Text owner — still rejected
weave bad [] -> Whole raises Whole:
  bind label <- "x"
  raise 1

# resource + raise in same weave — rejected
weave bad [] -> Whole raises Whole:
  bind memory <- arena 8
  raise 1

# resource + together — still AE-TASK-003
```

---

## 5. Invariants

| ID | Invariant |
| --- | --- |
| M16-INV-001 | Total + `handle` + live Arena/Buffer/table is legal when callee is copy-only Error[Whole] |
| M16-INV-002 | Erroring weaves remain resource-free |
| M16-INV-003 | `raise`/`forward` still require full clean boundary and no resource mix |
| M16-INV-004 | Live `access` cannot span `handle` |
| M16-INV-005 | Nursery + resource ban unchanged |
| M16-INV-006 | M4 pure effect examples and M2 pure resource examples still dual-compare |
| M16-INV-007 | Seed≡bootstrap on M16 mix corpus |
| M16-INV-008 | No resource-carrying error payload |

---

## 6. Implementation plan

1. Split weave-level mix ban (abortive vs handle).  
2. `validate_handle_boundary` vs full `validate_effect_boundary`.  
3. Verifier handle path relaxes Arena/Buffer/unique.  
4. Positive example + negatives; update M4 test expectations for resource+handle.  
5. Seed dual-compare.  
6. DOC-SYNC 0.21 + delivery report.

---

## 7. Deferred (future T-RX slices)

| Slice | Why deferred |
| --- | --- |
| `raise` after all resources ended | Needs precise liveness / end-of-life proof |
| Resource ops in may-error weaves | Cleanup on abort |
| Nursery + arena | Cancel vs partial buffer fills |
| Owned error payloads | Type + host ABI redesign |

---

## 8. Alternatives considered

| Option | Outcome |
| --- | --- |
| Keep absolute ban forever | Rejected — blocks real total programs |
| Allow raise with live arena + implicit free | Rejected — no destruction model |
| Result values instead of handle | Deferred — larger language change |
| Only allow handle after move of all resources | Rejected as M16 — too weak; handle-without-unwind is enough |

---

*End of DESIGN-M16-RESOURCE-EFFECT.md*
