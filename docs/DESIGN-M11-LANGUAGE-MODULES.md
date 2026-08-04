# M11 Design: language modules (import / export)

**Status:** M11a implemented (bootstrap project build); M11b seed pending  
**Date:** 2026-08-04  
**Decision record:** [ADR-015](ADR-015-m11-language-modules.md)  
**Validation record:** [M11 validation matrix](M11-VALIDATION-MATRIX.md)  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) track **T-MOD**  
**Depends on:** M10 multi-unit projects, Aether 0.11 surface, seed dual-compare discipline  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`, `DOC-SYNC-001`

---

## 1. Purpose and boundary

M10 proved **offline multi-unit project integrity** with **independent** seed
compile of each unit. That is intentionally not multi-file *programming*: a
`lib` unit cannot supply weaves to `main`.

**M11** introduces a **bounded language module system**:

- Explicit **exports** of weaves from library units  
- Explicit **imports** by project-relative path + alias  
- **Qualified calls** `call alias.weave …`  
- **Library units without `main`**  
- **Linked compile** of a closed module graph into **one** verified AETH artifact  

M11 does **not**:

- Add a network registry or URL imports  
- Add dynamic loading / plugins  
- Add package version ranges  
- Allow ambient filesystem imports outside the project unit set  
- Allow silent source concatenation marketed as modules  
- Expand host I/O, FFI, generics, or nested modules packages  

---

## 2. Core claim

> A closed set of project units can form a DAG of path imports. The compiler
> resolves exported weaves through aliases, typechecks the graph under ordinary
> Aether ownership rules, and emits one AETH program whose entry is the unique
> `main`. Library units need not define `main`.

**Falsifiers:**

- Importing a non-exported weave succeeds  
- Import path escapes project root or names a file not in the project document  
- Cycles are accepted  
- Product docs claim seed multi-module identity without proof  
- Implementation uses opaque text paste without a specified elaboration model  

---

## 3. Design decisions (normative for ADR-015)

### D1 — Import form

Root-only, after `world`, before records/shapes/weaves/host weaves:

```text
import unit "<unit-path>" as <alias>
```

| Piece | Rule |
| --- | --- |
| `unit-path` | Same grammar as M10 project unit paths (`/` only, `.ae`, no `..`) |
| `alias` | Lowercase ASCII identifier (`[a-z][a-z0-9_]*`) |
| Uniqueness | Alias unique in the importing unit; unit-path not imported twice under two aliases in one unit |

Example:

```aether
world app

import unit "lib/math.ae" as math

weave main [] -> Whole:
  yield call math.double 21
```

### D2 — Export form

Weaves that may be imported must be declared with `export`:

```aether
world math

export weave double [n: Whole] -> Whole:
  yield product n 2
```

| Rule | Detail |
| --- | --- |
| Default | Non-`export` weaves are **module-private** |
| `main` | Never exportable; only the graph entry may define `main` |
| `host weave` | **Not exportable** in M11 (host catalog stays entry-unit only) |
| Records / shapes | **Not importable** in M11 (local to defining unit only) |
| Effects | Exported weaves may `raises Whole` under ordinary M4 rules |

### D3 — Qualified call

```text
call <alias>.<weave-name> <args…>
```

Same argument/ownership rules as unqualified `call`. Forward-call and handle
forms: M11 pilot allows qualified names wherever an ordinary weave name is
today for **total** calls; effectful qualified calls follow M4 once typechecked.

Unqualified `call name` resolves only in the **current unit**.

### D4 — Library units without `main`

| Role | `main` required? |
| --- | --- |
| Project `main` unit (exactly one) | **Yes** — graph entry |
| Project `lib` unit | **No** — must not define `main` in M11 |
| Standalone single-file compile (no project) | **Yes** — unchanged 0.11 rule |

A `lib` unit that defines `main` is a compile error under M11 project build.

### D5 — Closed project graph

Imports are legal only when:

1. Path validates under M10 path grammar  
2. Path appears as a `units[].path` in the same `aether.project/v1` document  
3. Target unit’s role is `lib` (importing `main` unit from a lib is **error**)  
4. Resolving imports transitively stays inside the unit set  

**Orphan units** (listed but unreachable from main) may still be
`project verify`’d as libraries, but **`project build`** compiles the
**closed import cone** of the main unit only (plus main).

### D6 — Cycles

Import graph must be a **DAG**. Any cycle fails closed with a stable diagnostic.

### D7 — World names

Each unit has one `world <name>`.

| Rule | Detail |
| --- | --- |
| Uniqueness | `world` names unique across the compile graph |
| Alias | Import alias need not equal world name (alias is the qualifier) |
| Identity | Module identity for resolution is **unit path**, not world name |

### D8 — Compilation model (elaboration, not concatenation theater)

**Normative model:** multi-source **semantic compilation**.

1. Load all sources in the closed cone (UTF-8).  
2. Parse each unit to an AST with a module identity = unit path.  
3. Resolve imports/exports; build qualified weave table.  
4. Typecheck/ownership-check as one program with module barriers.  
5. Emit **one** AETH artifact (version **v12** if metadata requires; see D10).  

**Rejected:**

- Pasting file texts with banner comments and calling it modules  
- Runtime dynamic load of extra `.aeth` without verify-before-run story  

**Internal name mangling** (e.g. function symbols) is an implementation detail
if and only if observability of `run` matches source-level semantics.

### D9 — Product compile path and seed honesty (phased)

Forge ABI remains `compile [borrow source: Text] -> Bytes` (single `Text`).

Therefore M11 **does not pretend** the seed already multi-file compiles:

| Phase | Product behavior | Proof obligation |
| --- | --- | --- |
| **M11a** (first ship) | `project build` uses **bootstrap multi-source** compile API | Tests for graph semantics; single-file `aether compile` still **seed** |
| **M11b** (same program, may be later PR) | Seed accepts a **documented multi-module IR or multi-file host protocol** and dual-compares | Seed ≡ bootstrap on module corpus before multi-module becomes default product authority |

**Claim rule:** Until M11b green, MANIFEST must say multi-module project build is
**bootstrap-hosted**. Single-file seed path unchanged.

Optional M11a bridge (not required): host may feed seed a **canonical single-file
elaboration** that is *itself* valid Aether 0.11+ with mangled weave names —
only if the elaboration is **fully specified**, deterministic, and tests prove
source programs and elaborated forms correspond. Prefer honest bootstrap
multi-source over clever paste.

### D10 — AETH versioning

| Case | Action |
| --- | --- |
| Single AETH, no new opcodes, only more functions | May remain v11 if verifier needs no module table |
| Need module debug metadata / symbol table | Emit **AETH v12** with optional module string table; v11 remains readable |

ADR-015 default: **prefer v11 if possible**; bump to v12 only if metadata is
required for verify/run. Document choice in implementation delivery.

### D11 — CLI

```text
aether project build <project-file> --output <artifact.aeth>
aether project verify <project-file>   # existing; add module-aware checks when M11 language on
```

| Command | M11 behavior |
| --- | --- |
| `project verify` | Path/lock as M10; for each unit, parse OK; optional “lib has no main”; does **not** require full graph link until build |
| `project build` | Resolve cone from main; multi-source compile; write one AETH; verify before write |
| `compile` single file | Unchanged seed path; `import unit` **rejected** in single-file mode (must use project build) |

### D12 — Diagnostics

New codes (stable strings):

| Code | Meaning |
| --- | --- |
| `AE-MOD-001` | Illegal import/export syntax or placement |
| `AE-MOD-002` | Import path not in project / bad path grammar |
| `AE-MOD-003` | Unknown export / private weave |
| `AE-MOD-004` | Import cycle |
| `AE-MOD-005` | Duplicate alias, world, or export name conflict |
| `AE-MOD-006` | Lib unit defines `main` or main missing on entry |
| `AE-MOD-007` | Single-file compile saw `import unit` |

### D13 — Authoring protocol

M11 requires authoring schema bump (likely **v7**) when implemented:

- Nodes: `ImportUnit`, `ExportWeave` (or weave flag `exported`)  
- Edits: insert/delete import lines; toggle export on weaves  

May ship in the same delivery as language support or immediately after; matrix
must include structure round-trip for module examples.

### D14 — Capability and security

| Concern | Rule |
| --- | --- |
| Path jail | Import paths ⊆ project units; same resolve as M10 |
| No network | No URL imports |
| Guest | Still no ambient I/O; host weaves entry-only |
| Artifact | Verify before run/write preserved |

---

## 4. Invariants

| ID | Invariant |
| --- | --- |
| M11-INV-001 | `import unit` / `export weave` / `call alias.name` match §3 grammar |
| M11-INV-002 | Only exported weaves are import-visible |
| M11-INV-003 | Import graph is a DAG over project unit paths |
| M11-INV-004 | Exactly one `main` in the build cone (entry unit) |
| M11-INV-005 | Lib units must not define `main` |
| M11-INV-006 | Import targets must be listed project units under root |
| M11-INV-007 | Single-file seed compile rejects `import unit` |
| M11-INV-008 | `project build` emits one verified AETH for the cone |
| M11-INV-009 | Until M11b, multi-module build authority is bootstrap (documented) |
| M11-INV-010 | Host weaves not exportable; records/shapes not importable in M11 |
| M11-INV-011 | World names unique in the cone |

---

## 5. Example corpus (normative sketches)

### 5.1 `examples/project-modules/` (to ship with implementation)

`lib/math.ae`:

```aether
world math

export weave double [n: Whole] -> Whole:
  yield product n 2

weave secret [n: Whole] -> Whole:
  yield sum n 1
```

`src/main.ae`:

```aether
world app

import unit "lib/math.ae" as math

weave main [] -> Whole:
  yield call math.double 21
```

`aether.project.json`: units `src/main.ae` main + `lib/math.ae` lib, locks.

**Expect:** build runs / yields program exit **42**.  
**Expect:** `call math.secret` fails `AE-MOD-003`.

### 5.2 Independence preserved for M10 example

`examples/project-multi` remains valid as **non-module** independent units until
migrated; M10 verify behavior preserved.

---

## 6. Implementation plan (after ADR-015 accept)

### Phase M11a — bootstrap multi-module product path

1. Parser: `import unit`, `export`, qualified `call`  
2. Project: lib without main; graph resolve; cycle check  
3. `compile_modules(sources) -> AETH` in core (bootstrap)  
4. CLI `project build`  
5. Tests per matrix (no seed multi-file claim)  
6. DOC-SYNC: language version bump recommendation **0.12 surface** / package **0.14**  
7. Authoring v7 if in-slice  

### Phase M11b — seed multi-module proof

1. Seed strategy design amendment if needed (multi-source host API vs bundle)  
2. Dual-compare module corpus  
3. Switch default multi-module authority only when green  

### Suggested package pins at M11a ship

| Pin | Value |
| --- | --- |
| Language surface | **0.12** (modules) |
| Package | **0.14.0** |
| AETH | v11 or v12 per D10 |

---

## 7. Stop conditions

Abort or redesign if:

1. Seed pressure forces silent concatenation marketed as modules  
2. Import can read files outside project unit list  
3. Private weaves leak across modules  
4. Cycles or diamond ambiguity without rules (diamonds OK if DAG + unique exports)  
5. Multi-module claimed seed-hosted without dual-compare  
6. Host/FFI smuggled through “module” side channels  

---

## 8. Non-goals (explicit)

| Non-goal | Later track |
| --- | --- |
| `import` from absolute/URL paths | Never under offline law |
| Re-exports / `export import` | Post-M11 |
| Import records/shapes/types | Post-M11 |
| Package versions / registry | T-PKG / F-REGISTRY |
| Nested packages as first-class | T-PKG |
| LSP module navigation | T-LSP |
| Generics across modules | T-GEN |

---

## 9. Acceptance of this design package

Design package Done when:

- [x] This design committed  
- [x] ADR-015 accepted (process) for implementation readiness  
- [x] M11 validation matrix listed  
- [ ] Implementation M11a  
- [ ] Implementation M11b (seed)  

---

*End of DESIGN-M11-LANGUAGE-MODULES.md*
