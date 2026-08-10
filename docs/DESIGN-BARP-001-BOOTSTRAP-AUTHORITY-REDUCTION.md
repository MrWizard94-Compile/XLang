# BARP-001: Bootstrap Authority Reduction Program

**Status:** Active program — Phase 0 inventory + Phase 1 design  
**Date:** 2026-08-08  
**Decision:** [ADR-043](ADR-043-bootstrap-authority-reduction.md)  
**Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`

---

## 1. Goal

Reduce **Rust bootstrap authority** over the **product** compile path without
weakening:

- verify-before-run/write  
- seed-hosted product emission  
- dual-compare proofs for documented corpora  
- honest Seed Profile claims  

Bootstrap remains **required** for:

| Role | Why kept |
| --- | --- |
| Seed rebuild (`compile --bootstrap`) | Only way to emit new `seed/*.aeth` |
| `check` / structure / diagnostics | Full diagnostic authority (not claimed for seed) |
| Dual-compare tests | Proof oracle |
| Authoring AST (`aether.ast/v8`) | Tooling structure |

---

## 2. Current product-path bootstrap surface (inventory)

| Site | Authority today |
| --- | --- |
| `compile_with_seed` | Bootstrap **parse + validate** entire program before forge |
| `lower_m23_comptime_calls_for_seed` | Bootstrap **folds** M23 `call` → M5 literal form for seed input |
| Module elaborate | Host-side graph; then seed emit + dual-compare |
| `apply-edit` | Bootstrap structure + seed-compile before write |
| CLI `check` | Bootstrap only |

**Primary product emission** is already seed forge. The remaining **product-critical**
bootstrap authority is: **validation + M23 materialization**.

---

## 3. Phases

### Phase 0 — Inventory & gate (this delivery)

- Live `aether-gate -Mode release` green  
- Document inventory (this file)  
- ADR-043 accepted as program law  

### Phase 1 — Seed-native M23 (next implementable)

**Done when:**

1. Seed evaluates M23 pure comptime calls under the same D2a rules as ADR-039  
2. `compile_with_seed` forges **original** source (no `lower_m23_*` rewrite)  
3. `examples/comptime-calls.ae` seed≡bootstrap without materialization  
4. Seed Profile claim 17 updated to **seed interprets M23**  
5. Dual-compare + release gate green  

**Algorithm (seed):**

1. When `comptime` directive (`v89 = 56`) and RHS starts with `call`:  
2. Resolve args (literals + prior comptime env `v103`)  
3. Resolve callee among **prior** total Whole guest weaves  
4. Interpret callee body: Whole `bind` / `revise` / terminal `yield` only  
5. Emit `COMPTIME_WHOLE` (56) with folded value (same as arithmetic path)  
6. Fail closed (`invalid` / no silent CALL opcode under comptime)  

### Phase 2 — Validate-light product path (later ADR)

Optional: product compile may forge seed first and use bootstrap only as
dual-compare oracle in tests (not required for every CLI compile). Requires
seed fail-closed parity for all product surface errors of interest.

### Phase 3 — Seed diagnostics subset (later ADR)

Bounded seed error messages for common invalid inputs; bootstrap remains full
diagnostic authority.

### Out of scope / never BARP

- Removing dual-compare tests  
- Removing verify-before-run  
- Source transpile / native without F-NATIVE authorize  
- Network registry without F-REGISTRY authorize  

---

## 4. Success metrics

| Metric | Target |
| --- | --- |
| Product emit engine | Seed forge |
| M23 materialization bridge | **Removed** after Phase 1 |
| Bootstrap roles | Rebuild seed, check/AST, dual-compare oracle |
| Gate | `aether-gate -Mode release` PASS |

---

*End of DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md*
