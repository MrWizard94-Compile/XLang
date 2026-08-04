# M15 Design: bounded comptime expansion (T-CT)

**Status:** Accepted design for implementable ADR-019 (**code not started**)  
**Date:** 2026-08-04  
**Decision record:** [ADR-019](ADR-019-m15-comptime-expansion.md)  
**Validation:** [M15 validation matrix](M15-VALIDATION-MATRIX.md)  
**Depends on:** M5 ([DESIGN-M5-DETERMINISTIC-COMPTIME.md](DESIGN-M5-DETERMINISTIC-COMPTIME.md), [ADR-008](ADR-008-m5-deterministic-comptime.md))  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) **T-CT**  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`

---

## 1. Purpose and boundary

M5 proved a pure, fuel-bounded compile-time kernel: one literal `Whole`
arithmetic operation per `comptime bind`, fixed 1,024-directive budget, AETH
`COMPTIME_WHOLE` provenance, seed dual-compare.

**M15** is the first **controlled expansion** of that kernel. It admits **named
dependencies among prior root-level comptime Whole bindings** so authors can
chain sizes and constants without runtime arithmetic noise—and without opening
calls, control flow, host observation, macros, or type-level metaprogramming.

Non-goals for M15: weave calls at comptime, `if`/`while` at comptime, text/bytes
evaluation, Truth results, type computation, source generation, host I/O or
env observation (including M14 grants), user-adjustable fuel, nested multi-op
expression trees beyond one binary op per directive.

---

## 2. Core claim

> A `comptime bind` may evaluate one checked Whole arithmetic operation whose
> operands are either Whole literals or **earlier** immutable root-level
> `comptime bind` names in the same weave. Evaluation remains pure, ordered,
> and budgeted (≤1,024 directives). Results still lower to `COMPTIME_WHOLE`.
> Missing names, forward references, runtime names, and host-touching forms fail
> closed at compile time.

---

## 3. Design decisions

### D1 — Operand grammar (expansion of M5)

```text
comptime-bind     ::= "comptime bind" name "<-" whole-arithmetic
whole-arithmetic  ::= arithmetic-op whole-operand whole-operand
arithmetic-op     ::= "sum" | "difference" | "product" | "quotient" | "remainder"
whole-operand     ::= whole-literal | comptime-name
comptime-name     ::= name of a prior root-level immutable comptime bind
                      in the same weave (source order)
```

M5 remains a subset: both operands may still be literals.

### D2 — Name resolution rules

| Rule | Behavior |
| --- | --- |
| Scope | Same weave root only; not nested blocks |
| Order | Name must refer to a **prior** `comptime bind` (no forward refs) |
| Stage | Name must be a comptime binding; runtime `bind` is rejected |
| Mutability | Only immutable comptime binds (M5 already forbids mutable comptime) |
| Shadowing | Root slots already unique per weave; no rebind of same name |

### D3 — Fuel model unchanged

- Each `comptime bind` costs **exactly one** directive unit.  
- Program-wide cap remains **1,024**.  
- Name resolution does not add extra fuel; evaluation is still O(directives).  
- No user-adjustable quota.

### D4 — Checked arithmetic unchanged

Same signed-64-bit checked semantics as VM / M5 (`AE-COMPTIME-002` on overflow
or divide-by-zero).

### D5 — Purity / authority (harden vs M14)

Comptime evaluation **must not**:

- read files, env, process, network, shell  
- observe M14 host grants or guest paths  
- allocate arenas/buffers  
- call weaves (host or guest)  
- emit or mutate source

Compile-time purity is independent of run-time grants. M14 does not expand
comptime authority.

### D6 — AETH

- **No new opcode** if every accepted directive still folds to one `Whole`
  immediate via existing `COMPTIME_WHOLE` (56).  
- Artifact version remains **v11** product output (provenance instruction
  already valid in v8+).  
- Verifier/VM contracts unchanged for the opcode.

### D7 — Seed plan

- Seed must evaluate the same name-chaining subset for dual-compare on valid
  corpus.  
- Bootstrap remains diagnostic authority for invalid forms.  
- Honest claim: no full invalid-source diagnostic parity for seed.

### D8 — Authoring

- Structural AST already carries `stage: "comptime"` and expression trees.  
- Expression operands may be named atoms that resolve as comptime Whole.  
- Prefer **no protocol version bump** if v7 can represent the form; if a new
  constraint field is required, document it in the implement delivery—not as a
  silent schema change.  
- Hostile edits that introduce forward refs or runtime names must fail on
  reparse/validate before write.

### D9 — Diagnostics

| Code | Meaning (M15 additions) |
| --- | --- |
| `AE-COMPTIME-001` | Illegal form: forward ref, runtime name, non-Whole operand, non-root, mutable, wrong arity/op |
| `AE-COMPTIME-002` | Checked arithmetic failure |
| `AE-COMPTIME-003` | Budget exceeded (unchanged) |

Messages should distinguish “unknown or not yet defined comptime name” from
“runtime binding used in comptime operand” when practical.

### D10 — Package pin

Suggested **0.20.0** at implementation ship. Language surface remains **0.11**
base with M15 comptime expansion documented in `AETHER_0.20.md` (or toolchain
contract) at implement time.

### D11 — Stop conditions (abort M15 if hit)

- Need for recursive comptime functions to meet the slice  
- Any host/env observation required for demos  
- Fuel model becomes user-tunable from source  
- Seed cannot dual-compare a fixed chain corpus  
- Pressure to add textual macros or source generation  

---

## 4. Source examples

### Positive (chain)

```aether
world comptime_chain

weave main [] -> Whole:
  comptime bind cell <- product 8 8
  comptime bind row <- product cell 4
  comptime bind header <- sum 16 16
  comptime bind total <- sum row header
  yield total
```

Expected: `cell=64`, `row=256`, `header=32`, `total=288`.

### Negative

```aether
# forward reference
comptime bind a <- sum b 1
comptime bind b <- sum 1 1

# runtime name in comptime
bind r <- 3
comptime bind a <- sum r 1

# host/env (must never be admitted)
comptime bind a <- sum 1 1   # ok alone
# any form that calls env_get / read_text at comptime — reject always
```

---

## 5. Invariants

| ID | Invariant |
| --- | --- |
| M15-INV-001 | M5 literal-only programs remain valid and byte-stable under dual-compare |
| M15-INV-002 | Comptime operands are only literals or prior same-weave comptime Whole names |
| M15-INV-003 | Forward refs and runtime names fail closed (`AE-COMPTIME-001`) |
| M15-INV-004 | Fuel remains ≤1,024 directives; no user quota |
| M15-INV-005 | Evaluation is pure; no host I/O or M14 grant observation |
| M15-INV-006 | Results emit `COMPTIME_WHOLE`; no new ambient runtime authority |
| M15-INV-007 | Seed≡bootstrap on documented M15 chain corpus before product claim |
| M15-INV-008 | No calls, control flow, text, types, or code gen in this slice |

---

## 6. Implementation plan (after ADR Accepted)

1. Bootstrap: resolve comptime env map during root validation; evaluate chains.  
2. Negatives: forward ref, runtime name, unknown name, overflow through chain.  
3. Seed: same evaluation order; dual-compare example + corpus.  
4. Authoring round-trip for chain example.  
5. DOC-SYNC 0.20 + delivery report.  
6. Gates: fmt, clippy `-D warnings`, core/CLI/seed tests.

---

## 7. Deferred (future T-CT slices, new ADRs)

| Slice | Why deferred |
| --- | --- |
| Multi-op expression trees | Fuel accounting per op; parse complexity |
| Pure total weave calls at comptime | Effect/termination/stack model |
| Comptime control flow | Branch explosion / non-termination |
| Truth / comparisons | Type surface growth |
| Type-level / shape computation | Generics interaction |
| Comptime text/bytes | Memory bounds + different fuel model |

---

## 8. Alternatives considered

| Option | Outcome |
| --- | --- |
| Full Zig-style `comptime` blocks now | Rejected — unbounded surface |
| Only multi-op literals without names | Rejected — less author value than chaining |
| Automatic constant folding of runtime binds | Rejected — erases stage intent (M5 law) |
| Raise 1,024 budget | Rejected for M15 — separate evidence required |
| Observe env at comptime for “config” | Rejected — purity + reproducibility |

---

*End of DESIGN-M15-COMPTIME-EXPANSION.md*
