# BARP-001: Bootstrap Authority Reduction Program

**Status:** Active program — Phase 0–2 complete; Phase 3 (seed diagnostics) later  
**Date:** 2026-08-08 (Phase 1 2026-08-10; Phase 2 2026-08-10)  
**Decision:** [ADR-043](ADR-043-bootstrap-authority-reduction.md), [ADR-044](ADR-044-barp-phase2-validate-light-product-path.md)  
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
| `compile_product_bytecode` | **Phase 2:** seed forge + verify only (no bootstrap pre-validate) |
| `compile_with_seed` | Forge-first product bytes; bootstrap parse only for returned `Program` AST |
| ~~`lower_m23_comptime_calls_for_seed`~~ | **Removed in Phase 1** — seed interprets raw M23 `call` |
| Module elaborate | Host-side graph; then seed emit + dual-compare |
| `apply-edit` | Bootstrap structure + **product** seed compile before write |
| CLI `check` | Bootstrap only |

**Primary product emission** is seed forge without bootstrap as a pre-gate
(Phase 2). Bootstrap remains rebuild, `check`/AST diagnostics, and dual-compare
oracle.

---

## 3. Phases

### Phase 0 — Inventory & gate (this delivery)

- Live `aether-gate -Mode release` green  
- Document inventory (this file)  
- ADR-043 accepted as program law  

### Phase 1 — Seed-native M23 (**complete** 2026-08-10)

**Done when:**

1. Seed evaluates M23 pure comptime calls under the same D2a rules as ADR-039 — **done**  
2. `compile_with_seed` forges **original** source (no `lower_m23_*` rewrite) — **done**  
3. `examples/comptime-calls.ae` seed≡bootstrap without materialization — **done**  
4. Seed Profile claim 17 updated to **seed interprets M23** — **done**  
5. Dual-compare + release gate green — verify at ship  

**Algorithm (seed) — implemented:**

1. When `comptime` directive (`v89 = 56`) and RHS starts with `call`:  
2. Resolve args (literals + prior comptime env `v103`)  
3. Resolve callee by scanning prior `weave name [` definitions in source  
4. Interpret callee body: Whole `bind` / `revise` / terminal `yield` only  
5. Emit `COMPTIME_WHOLE` (56) with folded value (same as arithmetic path)  
6. Bootstrap still validates product path; dual-compare is the correctness gate  

### Phase 2 — Validate-light product path (**complete** 2026-08-10; ADR-044)

**Done when:**

1. Product bytecode path forges seed first without bootstrap validate pre-gate — **done**
   (`compile_product_bytecode`; CLI default `compile` uses it).  
2. Dual-compare tests remain the oracle — **unchanged**.  
3. CLI `check` / LSP remain bootstrap diagnostics — **unchanged**.  
4. Honest claims: no seed diagnostic parity — **documented**.  
5. Tracker `product_path_forges_before_bootstrap_validate() == true` — **done**.

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
| Product emit engine | Seed forge (forge-first; Phase 2) |
| M23 materialization bridge | **Removed** (Phase 1) |
| Bootstrap product pre-validate | **Not required** (Phase 2) |
| Bootstrap roles | Rebuild seed, check/AST, dual-compare oracle |
| Gate | `aether-gate -Mode release` PASS |

---

*End of DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md*
