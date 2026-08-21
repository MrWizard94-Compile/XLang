# BARP-001: Bootstrap Authority Reduction Program

**Status:** Active program — product-default toolchain (ADR-064–072; bounded increments through ADR-129); ADR-127–129 release verified; residual recovery bootstrap
**Date:** 2026-08-21 (Phase 1–2 / ADR-044–072; bounded seed-SPEAK pilots through ADR-127 and SBP-001/SBP-002)
**Decision:** [ADR-043](../historical%20docs/ADR-043-bootstrap-authority-reduction.md)–[ADR-129](../historical%20docs/ADR-129-barp-seed-native-transitive-library-chain-profile.md)
**Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`

---

## 1. Goal

Reduce **Rust bootstrap authority** over the **product** compile path without
weakening:

- verify-before-run/write  
- seed-hosted product emission  
- dual-compare proofs for documented corpora  
- honest Seed Profile claims  

Bootstrap remains **required** only for residual roles (ADR-065–071 reduced further):

| Role | Why kept |
| --- | --- |
| Dual-compare tests / gate oracle (`compile --bootstrap`) | Proof authority (product rebuild is default) |
| Full `aether.ast/v8` structure | Recovery / authoring tree |
| Recovery flags (`check|format|structure --bootstrap`) | Full AST diagnostics / rewrite |

---

## 2. Current product-path bootstrap surface (inventory)

| Site | Authority today |
| --- | --- |
| `compile_product_bytecode` | **Phase 2:** seed forge + verify only; **Phase 3a/3b** `AE-SEED-*` preflights |
| `compile_with_seed` | **ADR-051:** seed-only; empty placeholder `Program` (never bootstrap) |
| ~~`lower_m23_comptime_calls_for_seed`~~ | **Removed in Phase 1** — seed interprets raw M23 `call` |
| Module elaborate | Host-side graph; then **seed emit only** (ADR-045/047: no bootstrap dual-compare or AST) |
| Lib project verify | **Product seed probe** with synthetic main (ADR-052; was bootstrap) |
| `apply-edit` | Bootstrap base parse AST; **product seed accept** (ADR-048); CLI product write gate |
| CLI `check` | **Product default** (ADR-064); `--bootstrap` recovery AST diagnostics |
| CLI `format` | **Product default** LF+accept (ADR-064); `--bootstrap` AST rewrite |
| CLI `project format` | **Product default** (ADR-064); `--bootstrap` AST format |
| CLI `apply-edit` | **ADR-065/068/069/071:** product top-level weave + weave-body + nested choose/while body lists + primitive records without base AST; product accept (ADR-048/053) |
| CLI `structure` | **Product default** envelope (ADR-064); `--bootstrap` aether.ast/v8 |
| CLI `compile` (incl. seed) | **Product default** (ADR-067); `--bootstrap` dual-compare oracle |
| LSP diagnostics | **Product primary** AE-SEED (ADR-058) |
| LSP symbols | Product-surface when product accepts (ADR-063) |
| LSP format | **Product** LF+accept (ADR-064) |
| LSP hover/definition | **Product-surface** local (ADR-066); import path text scan |
| Product diagnostics API | [`product_diagnostics`] (ADR-055) |
| Multi-module | Host elaborate + seed emit; general seed-native multi-file **false** (ADR-056). ADR-128/129 are separate exact v1 two-unit and v2 transitive-three-unit profiles, not general exceptions. |

**Primary product emission** is seed forge without bootstrap pre-gate or
product dual-compare gate. Bootstrap remains dual-compare **oracle**, recovery
flags, and non–weave-replace structural AST (tests + aether-gate).

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

### Phase 3 — Seed diagnostics subset

#### Phase 3a (**complete** 2026-08-10; ADR-046)

1. Product-path stable `AE-SEED-001` / `002` / `003` codes + `aether check` hint — **done**  
2. Odd indentation fail-closed on product path — **done**  
3. Bootstrap remains full diagnostic authority — **unchanged**  

#### Phase 3b (**host preflight complete** 2026-08-10; ADR-050)

1. `AE-SEED-004` missing `weave main` — **done**  
2. `AE-SEED-005` empty source — **done**  
3. `AE-SEED-006` missing `world` — **done**  
4. `AE-SEED-007` legacy-syntax heuristics — **done**  
5. Deeper **seed-side** messages (unbound names, types as first-class seed codes)
   without claiming full bootstrap parity — **Phase 3c host classification done
   (ADR-052)**; structured seed error ABI remains later  

### ADR-048 — Structural-edit product accept (**complete** 2026-08-10)

Post-edit accept gate is product seed; base parse remains bootstrap for authoring AST.

### ADR-049 — Product-authoritative `compile_with_seed` + product seed rebuild in gate (**complete** 2026-08-10)

Product Ok no longer requires bootstrap after seed verify; gate seed identity is
bootstrap ≡ product ≡ forged ≡ checked-in.

### ADR-050 — Phase 3b preflight expansion + roadmap DOC-SYNC (**complete** 2026-08-10)

`AE-SEED-005`–`007`; authoring canonicalize skips redundant second bootstrap
compile when already canonical.

### ADR-051 — Bootstrap-free `compile_with_seed` + product CLI check (**complete** 2026-08-10)

`compile_with_seed` never calls bootstrap; `aether check --product` validates
via seed only. Default `check` remains full bootstrap diagnostics.

### ADR-052 — Product lib validate + Phase 3c diagnostics (**complete** 2026-08-10)

Project-verify lib probes use product seed; forge/verify map to
`AE-SEED-008` / `010` / `011` (host classification, not full parity).

### ADR-053 — Product format + apply-edit CLI trust (**complete** 2026-08-10)

`format --product` is LF normalize + product accept (no bootstrap AST rewrite).
CLI apply-edit trusts core product accept (no second forge).

### ADR-054 — Product project format + product structure (**complete** 2026-08-10)

`project format --product` role-aware seed format; `structure --product` emits
`aether.product-structure/v1` without bootstrap AST.

### ADR-055 — Product diagnostic ABI (**complete** 2026-08-10)

`product_diagnostics` + `AE-SEED-012` raw import unit (host-facing structured
product diagnostics; not full seed-internal error packets).

### ADR-056 — Host elaborate + seed emit multi-module (**complete** 2026-08-10)

Contract + honesty trackers; seed-native multi-file elaboration remains false.

### ADR-128 — bounded seed-native Whole-library bundle (**release verified**)

The separate `aether.seed-bundle/v1` profile carries exactly one library and one
entry source as bounded scalar-framed Text to seed
`compile_bundle [borrow bundle: Text] -> Bytes`. The seed validates framing,
paths, distinct worlds, one exact import, one exported pure Whole helper, and
qualified calls; it applies the established M11 mangle and compiles the resulting
single-world source. The product path supplies opaque Text and never invokes the
host M11 elaborator. The profile is capability-closed and retains
`seed_native_multi_module_elaboration() == false` for general projects,
workspaces, and multi-source envelopes. See [ADR-128](../historical%20docs/ADR-128-barp-seed-native-whole-library-bundle-profile.md),
[SBP-001](../historical%20docs/DESIGN-SBP-001-SEED-NATIVE-WHOLE-BUNDLE-PROFILE.md),
and [the SBP matrix](SBP-VALIDATION-MATRIX.md).

### ADR-129 — bounded seed-native transitive library chain (**release verified**)

The distinct `aether.seed-bundle/v2` profile preserves the same closed
`compile_bundle [borrow bundle: Text] -> Bytes` ABI and proves exactly two
seed-owned import edges: foundation library -> bridge library -> main entry.
The seed validates the three scalar-framed ASCII/LF units, safe paths, distinct
worlds, exact imports, one export per library, and qualified calls before it
applies M11-compatible mangling/rewrite and compiles one source. The product
route still supplies opaque Text and never invokes the host M11 elaborator.
The profile is capability-closed and keeps
`seed_native_multi_module_elaboration() == false` for ordinary projects,
workspaces, and envelopes. It is not an arbitrary three-unit graph. See
[ADR-129](../historical%20docs/ADR-129-barp-seed-native-transitive-library-chain-profile.md),
[SBP-002](../historical%20docs/DESIGN-SBP-002-SEED-NATIVE-TRANSITIVE-CHAIN-PROFILE.md),
and [the SBP-002 threat model](THREAT_MODEL-SBP-002-SEED-CHAIN.md).

### ADR-057 — Structural-edit product base gate (**complete** 2026-08-10)

When both product and bootstrap reject base source, prefer product AE-SEED.

### ADR-058 — LSP product diagnostics primary (**complete** 2026-08-10)

LSP diagnostics use product seed path; symbols/format still bootstrap AST.

### ADR-045 — Product dual-compare oracle-only (**complete** 2026-08-10)

Project/workspace product build no longer bootstrap dual-compares on every
emit; tests and gate remain the dual-compare oracle.

### ADR-047 — Multi-module product emit without bootstrap AST (**complete** 2026-08-10)

Product project/workspace compile returns seed bytes via
`compile_product_bytecode` only (`product_multi_module_invokes_bootstrap() == false`).

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
| Product dual-compare gate | **Not required** (ADR-045); oracle in tests/gate |
| Product diagnostics subset | **Phase 3a–3c** `AE-SEED-001`–`011` (ADR-046/050/052) |
| `compile_with_seed` bootstrap | **None** (ADR-051) |
| Product CLI check | **`check --product`** (ADR-051) |
| Product CLI format | **`format --product`** LF+accept (ADR-053) |
| Product project format | **`project format --product`** (ADR-054) |
| Product structure | **`structure --product`** envelope (ADR-054) |
| Product diagnostics API | **`product_diagnostics`** (ADR-055) |
| Multi-module | Host elaborate + seed emit (ADR-056); general seed-native = false. SBP-001 is a distinct exact two-unit profile and SBP-002 an exact transitive-three-unit profile (ADR-128/129). |
| LSP diagnostics / hover / def | **Product primary** (ADR-058/066) |
| Structural top-level weaves | **Product path** (ADR-065/068 replace/insert/delete) |
| Structural statements/records | **Product path** weave-body + primitive records (ADR-069) |
| Nested body lists | **Product path** choose/while `whenBright`/`whenDim`/`body` (ADR-071) |
| Seed-error packets | **Host packet ABI** `aether.seed-error/v1` (ADR-072); seed SPEAK pilot 003/004/005/006/007/010/011/012/013/014/015, with bounded lexical 003/007, line-aware canonical reserved-task 014, exact task-checkpoint 015, canonical ordinary-Whole Text-literal / exact Truth-literal yield 010, bounded total-Whole direct/nursery target existence 011, and canonical ordinary-Whole nested-yield 013 detection. ADR-127 additionally makes the seed's existing M23 evaluator reject a canonical root forward target through a source-prefix ordinary-header witness; it is not full M23 eligibility or diagnostic parity. See the validation matrix for the precise scope. |
| Yield-in-truth-choose | **Product fail-closed** `AE-SEED-013` (ADR-070); direct seed has only the ADR-109/112/113/114 canonical `choose same` / `choose less` / exact literal and unary-literal Truth witnesses; multi-module choose+revise supported |
| Seed rebuild | **Product compile** (ADR-067); `--bootstrap` oracle only |
| Lib unit project verify | **Product seed probe** (ADR-052) |
| Bootstrap residual | Dual-compare oracle, recovery flags, full ast/v8 |
| Gate | `aether-gate -Mode release` PASS |

---

*End of DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md*
