# M19a Design: explicit `release` (resource end-of-life foundation)

**Status:** Accepted design for ADR-027 — bootstrap implemented 0.25.0; seed dual-compare pending  
**Date:** 2026-08-04  
**Depends on:** ADR-023 (deeper T-RX needs destruction model), M2 resources, M4 effects  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`

---

## 1. Purpose

ADR-023 defers abortive×resource and nursery×resource until an explicit
end-of-life form exists. **M19a** adds that form:

```text
release <name>
```

Logical destruction of a live owner binding so later effect sites can observe a
**clean** boundary without ambient free-on-raise.

## 2. Core claim

> A root-level `release name` logically destroys a live unique or resource
> owner binding and marks it moved. After all such owners are released (or
> never live), abortive `raise`/`forward` sites may pass the existing clean
> boundary even if the weave earlier bound resources. Destruction is pure
> (no host I/O, no user destructor).

## 3. Design decisions

### D1 — Syntax

```aether
release label
release values
release memory
```

- Root-level only (same family as bind/revise).  
- Single name operand; no expression.  
- Not terminal (statements may follow, including raise).

### D2 — What may be released

| Type | Allowed | Notes |
| --- | --- | --- |
| Text, Bytes, Record | yes | unique values |
| BufferWhole, BufferTruth, Table | yes | resource owners |
| Arena | yes only if no live Buffer/Table/AccessArena remains |
| Whole, Truth | **no** | copy types; not owners |
| AccessArena | **no** | loan ends with operation, not release |

### D3 — Semantics

1. Name must be bound, not yet moved.  
2. Apply D2 type rules.  
3. Arena special case: reject if any other live resource owner/loan exists.  
4. Mark binding **moved** (unusable).  
5. Emit `RELEASE` (opcode 66, slot:u16) — VM drops/clears the local.  
6. No result value; pure total statement.

### D4 — Effect interaction (M19a deliverable)

- Keep site-level `validate_effect_boundary` for raise/forward.  
- **Relax** weave-level ban `resource ∧ abortive` so a weave may bind resources,
  `release` them, then `raise`/`forward` when the site is clean.  
- Keep `Error[Whole] ∧ weave_uses_resource` **only if** the erroring weave still
  contains unreleased resource **forms** that remain live at some abortive site
  — actually simpler: drop weave-level abortive×resource ban; rely on site
  boundary. Keep ban on erroring weaves that still *use* resource ops without
  release? Site boundary is enough if every raise is checked.

- Erroring weaves may bind Text and release before raise (primary demo).  
- Main remains total (no raise).  

### D5 — AETH

| Opcode | Encoding | Meaning |
| --- | --- | --- |
| `RELEASE` (66) | `slot:u16-le` | Destroy local at slot; mark moved |

Valid in **v11** (same as HOST_CALL family). Verifier: slot initialized, not
already moved, type is releasable; after, moved.

### D6 — Seed

Seed must parse `release <name>`, resolve slot, emit 66+u16 for dual-compare.

### D7 — Non-goals

- User destructors / RAII hooks  
- Auto-release on raise  
- Nursery cancel cleanup (later M19b)  
- release of access loans  

### D8 — Package

**0.25.0**

---

## 4. Examples

### Positive — raise after release Text

```aether
weave boom [] -> Whole raises Whole:
  bind label <- "secret"
  release label
  raise 9

weave main [] -> Whole:
  bind mutable success <- 0
  bind mutable code <- 0
  handle call boom into success otherwise error into code
```

Exit: 9.

### Positive — release buffer then continue

```aether
# after buffer finished, release before pure handle
```

### Negative

- `release` copy Whole  
- `release` already moved  
- `release` arena while buffer live  
- `raise` while Text still live (unchanged)  

---

## 5. Invariants

| ID | Invariant |
| --- | --- |
| M19A-INV-001 | release only on live releasable owners |
| M19A-INV-002 | after release, name is moved |
| M19A-INV-003 | raise/forward still fail if live owner remains |
| M19A-INV-004 | release after release of all owners allows raise |
| M19A-INV-005 | no host I/O during release |
| M19A-INV-006 | seed≡bootstrap on release corpus |

---

*End of DESIGN-M19A-EXPLICIT-RELEASE.md*
