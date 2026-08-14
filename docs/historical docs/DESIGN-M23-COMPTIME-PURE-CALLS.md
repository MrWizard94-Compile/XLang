# M23 Design: pure comptime weave calls (T-CT)

**Status:** Implemented in package **0.33.0**
**Accepted:** 2026-08-05; **implemented:** 2026-08-07
**Decision record:** [ADR-039](ADR-039-m23-comptime-pure-calls.md)
**Validation:** [M23-VALIDATION-MATRIX.md](../Current%20state/M23-VALIDATION-MATRIX.md)
**Depends on:** M5 ([ADR-008](ADR-008-m5-deterministic-comptime.md)), M15 ([ADR-019](ADR-019-m15-comptime-expansion.md))
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) **T-CT**
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `RND-INVAR-001`

---

## 1. Purpose and boundary

M5 proved pure literal Whole arithmetic at compile time. M15 proved **name
chaining** of prior comptime bindings. Authors still cannot factor shared
compile-time formulas into helper weaves without runtime `call`.

**M23** admits **one pure total guest weave call** as a `comptime bind` right-hand
side, with Whole-only arguments and result, under the same purity and fuel
discipline as M5/M15.

### Non-goals (M23)

| Out | Why |
| --- | --- |
| Host / foreign / I/O weaves | Authority leak (M14/M21) |
| `raises Whole` callees | Effect surface at compile time |
| Resource / arena / buffer / table / access in callee | Resource plan at comptime |
| Nursery / spawn in callee | Concurrency at compile time |
| Comptime control flow (`choose`/`while`) | Separate T-CT slice |
| Nested multi-op expression trees | Still one call **or** one arithmetic op per directive |
| Recursion / mutual recursion | Fuel & termination DoS |
| Forward callee (declared after call site) | Ordered evaluation like M15 names |
| Runtime `bind` names as call args | Stage confusion |
| Text/Bytes/Truth results or params | Keep Whole-only pilot |
| User-settable fuel | AI-hostile budget (M5-INV-003) |
| Source generation / macros | Authority & dual-compare |

---

## 2. Core claim

> A root-level `comptime bind name <- call W a b …` evaluates, at compile time,
> a **total guest** weave `W` that has only owned/copy `Whole` parameters and a
> `Whole` result, contains no resource/effect/nursery/host forms, and is declared
> **before** the call site in the same program. Arguments are Whole literals or
> prior comptime names. Evaluation is pure, fuel-bounded, and folds to
> `COMPTIME_WHOLE`. Illegal callees, args, recursion, and host observation fail
> closed (`AE-COMPTIME-001` / budget / arithmetic codes as applicable).

---

## 3. Design decisions

### D1 — Directive grammar (extends M15)

```text
comptime-bind     ::= "comptime bind" name "<-" comptime-rhs
comptime-rhs      ::= whole-arithmetic
                    | "call" weave-name whole-operand*
whole-arithmetic  ::= arithmetic-op whole-operand whole-operand   # unchanged M15
whole-operand     ::= whole-literal | comptime-name               # unchanged M15
```

Still **exactly one** RHS form per directive: either one binary arithmetic op
**or** one `call` (not both nested).

### D2 — Callee eligibility (`comptime-pure`)

Weave `W` is comptime-callable iff **all** hold:

1. Guest weave (not host, not foreign)
2. `effect == Total` (no `raises Whole`)
3. Result type `Whole`
4. Every parameter is owned/copy `Whole` (no `borrow`, no `access`, no Text/Bytes/…)
5. Body uses only: root binds/revises of copy Wholes, arithmetic/compare atoms
   already legal at runtime for Whole, `choose`/`while` on **runtime** control
   of Whole values **inside the callee body when evaluated at comptime** —
   **wait**

Control flow inside callee: if we allow full runtime subset of Whole programs,
evaluation is an interpreter. That's larger.

**M23 tightens callee body** for the first vertical slice:

#### D2a — Callee body subset (M23 pilot)

Callee body may contain **only**:

- root-level `bind` / `revise` of `Whole` (runtime stage inside interpreter)
- `yield` of a Whole atom (literal, local name, or one binary arithmetic op)
- **No** nested `choose`, `while`, `together`, `handle`, `raise`, `forward`,
  `release`, resources, records, text, bytes, calls (including nested calls)

That keeps the evaluator a thin stack of Whole locals + arithmetic, matching
seed feasibility.

**Future T-CT slice** may admit nested calls and pure control flow under a new
ADR.

### D3 — Call arguments

| Rule | Behavior |
| --- | --- |
| Count | Must match parameter count |
| Each arg | Whole literal or prior **comptime** name (same weave as the `comptime bind`) |
| Mode | Pass by value (Whole copy) |

### D4 — Declaration order

- Callee weave must appear **textually before** the calling weave in the program
  (same multi-weave order rules as ordinary programs).
- No recursive call graph: callee body contains **no** `call` (D2a).
- Cross-weave: only from a `comptime bind` in weave A to pure weave B declared
  earlier.

### D5 — Fuel model

| Event | Cost |
| --- | --- |
| Each `comptime bind` (arithmetic or call) | **1** directive unit (unchanged program-wide count) |
| Inside callee evaluation | **No extra** program-wide directive units in M23 (body is small; D2a forbids loops) |

Program-wide cap remains **1,024** `comptime bind` directives.

**Rationale:** With D2a forbidding loops and nested calls, callee evaluation is
O(body size) bounded by source size, not open-ended. A later slice that admits
`while` must charge fuel per iteration (new ADR).

### D6 — Purity / authority (harden)

Comptime evaluation **must not**:

- observe host grants, env, files, network, shell, models
- load foreign libraries
- allocate arenas/buffers/tables
- run nurseries
- raise/forward/handle

Compile-time purity is independent of run-time grants (same as M15 D5).

### D7 — AETH

- Call folds to a single Whole immediate.
- Emit existing **`COMPTIME_WHOLE` (56)** only (no new opcode).
- Artifact version remains **v11**.
- Verifier/VM unchanged for the opcode.

### D8 — Seed plan

- Product path uses bootstrap validation/evaluation, then materializes each
  accepted call into an equivalent M5 literal directive for seed emission; the
  product artifact must dual-compare to direct bootstrap output.
- Dual-compare corpus: chain-of-calls example + M5/M15 regressions.
- Honest boundary: the checked-in seed does not independently evaluate raw M23
  source. The default product path is seed-emitted **after** bootstrap
  materialization; no direct-seed-evaluator claim is made.

### D9 — Diagnostics

| Code | Use |
| --- | --- |
| `AE-COMPTIME-001` | Illegal call form, ineligible callee, bad arg stage, nested call in callee, unknown weave |
| `AE-COMPTIME-002` | Checked arithmetic failure during call body |
| `AE-COMPTIME-003` | Budget exceeded (comptime bind count) |

---

## 4. Example surface (product claim target)

```aether
world comptime_calls

weave double [v: Whole] -> Whole:
  yield product v 2

weave area [w: Whole, h: Whole] -> Whole:
  yield product w h

weave main [] -> Whole:
  comptime bind cell <- product 8 8
  comptime bind wide <- call double cell
  comptime bind total <- call area wide 4
  yield total
```

Expected: `cell=64`, `wide=128`, `total=512`, exit **512**.

### Negative examples

```aether
# host weave not callable
host weave whole_inc [value: Whole] -> Whole
comptime bind x <- call whole_inc 1

# erroring weave not callable
weave boom [v: Whole] -> Whole raises Whole:
  raise v
comptime bind x <- call boom 1

# nested call in callee (M23)
weave bad [v: Whole] -> Whole:
  yield call double v
```

---

## 5. Implementation sketch (for engineers; not authorization alone)

1. ✅ Bootstrap: classify `comptime-pure` weaves; evaluate `call` RHS with a
   Whole local frame.
2. ✅ Reject host/foreign/effect/resource/control callees.
3. ✅ Seed-emitted path: materialize eligible calls into M5 literals and
   dual-compare.
4. ✅ Positives, negatives, structural-authoring, and product-path parity.
5. ✅ Package **0.33.0** + AETHER_0.33, matrix, and delivery report.

---

## 6. Ordered backlog context (human 2026-08-05)

| Priority | Track | Status after this design |
| --- | --- | --- |
| 1 | **T-CT ADR** (this document + ADR-039) | Design complete; implement next when directed |
| 2 | Offline package polish | Waiting |
| 3 | Ownership + destroy + mid-frame cancel | Waiting (full T-RX ADR) |
| 4 | Human law forks F-NATIVE / F-REGISTRY | Packages ready; need §3 authorize phrases |

---

## 7. Stop conditions

- Implementing comptime host/foreign observation
- Claiming general metaprogramming
- Shipping without seed dual-compare
- Admitting recursive comptime calls without fuel model ADR

---

*End of DESIGN-M23-COMPTIME-PURE-CALLS.md*
