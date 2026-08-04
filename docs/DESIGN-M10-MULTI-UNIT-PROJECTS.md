# M10 Design: multi-unit offline projects

**Status:** Accepted design (implementation pending gate)  
**Date:** 2026-08-04  
**Decision record:** [ADR-013](ADR-013-m10-multi-unit-projects.md)  
**Validation record:** [M10 validation matrix](M10-VALIDATION-MATRIX.md)  
**Depends on:** [DESIGN-M9-PROJECT-TOOLING.md](DESIGN-M9-PROJECT-TOOLING.md), [ADR-012](ADR-012-m9-project-tooling.md)  
**Threat model:** [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md) (no authority expansion)  
**Related Rule IDs:** `DOC-ADR-001`, `DOC-SYNC-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`, `CONST-COMPLETE-001`

## 1. Purpose and boundary

M9 proved **offline project identity** for a single-unit fixture. The project
schema already admits up to 256 units and a `lib` role, but:

1. The shipped example is single-unit only.  
2. Path grammar effectively forbids nested paths (JSON schema rejects `/`).  
3. There is no first-class `project format` / multi-unit compile entry.  
4. Semantics of multi-unit **linking** are undefined because Aether has no
   source-level import/module system.

**M10** makes multi-unit offline projects **real, tested, and honest**:

- Multiple local units under one project root  
- Nested relative paths with strict confinement  
- Locks spanning every unit  
- Verify + format + optional per-unit artifact write  
- **Independent compilation units** (no cross-file name resolution)

M10 does **not** introduce language modules, import syntax, weave visibility
across files, package registries, version ranges, or LSP.

## 2. Core claim

> An Aether project may list many local `.ae` units. The host verifies schema,
> path bounds, and optional SHA-256 locks, then seed-compiles **each unit as its
> own complete program**. Units do not form a linked multi-file language graph
> until a future language-module ADR.

This claim is falsifiable: if a test expects `lib/util.ae` symbols to resolve
inside `src/main.ae` without a language import feature, the design is violated.

## 3. Threat model (delta over M9 / TP)

| Concern | M10 rule |
| --- | --- |
| Path escape | Same as M9: no absolute paths, no drive prefixes, no `..` components. Nested paths use **forward slash only** in the document. |
| Symlink escape | Resolve with canonicalize; resolved path must stay under project root. |
| Graph explosion | Max **256** units (existing). No remote edges. |
| Authority | Host-only I/O; guest AETH unchanged. |
| Seed free host | Multi-unit must not require seed to parse project JSON or link units. |
| Registry creep | Forbidden. No URLs, no package names outside local path units. |

**Stop conditions (abort design / implementation):**

1. Any requirement that the seed compiler read project metadata.  
2. Ambient network or registry resolution.  
3. Cross-unit linking without a language-module ADR and seed dual-compare plan.  
4. Absolute or host-global unit paths.  
5. Turning the project document into a second unsafe language (scripting, macros).

## 4. Invariants

| ID | Invariant |
| --- | --- |
| M10-INV-001 | Schema remains `aether.project/v1` with an **expanded path grammar** (nested relative `.ae` paths). Unknown fields still rejected. |
| M10-INV-002 | Exactly one unit with `role: "main"`; zero or more `role: "lib"`. |
| M10-INV-003 | Unit paths are non-empty, relative, use `/` separators only in the document, end with `.ae`, contain only safe path segments (`[A-Za-z0-9._-]+`), and never include `.` or `..` as a segment. |
| M10-INV-004 | Resolved filesystem paths stay under the project root after canonicalize. |
| M10-INV-005 | When `lock` is present, every unit path appears exactly once with a matching SHA-256 of exact file bytes. |
| M10-INV-006 | `project verify` seed-compiles **each** unit independently and verifies each AETH artifact. Failure on any unit fails closed. |
| M10-INV-007 | Units are **independent programs**. A `lib` unit is a complete Aether source that seed-compiles on its own (including its own `world` and typically its own `main` for the pilot, **or** a documented lib-only subset if later approved — default pilot requires each unit to be a full valid program). |
| M10-INV-008 | `project format` formats every unit to formatter-canonical text; writes only when an explicit output mode is requested. |
| M10-INV-009 | Optional `project verify --output-dir` writes one artifact per unit with a deterministic name derived from the unit path (see §7). |
| M10-INV-010 | No network, registry, URL dependency, model call, or language import syntax. |
| M10-INV-011 | Diagnostic codes stay in the `AE-PROJECT-*` family; new codes only if needed and documented. |

### 4.1 Honest `lib` role (pilot)

Until language modules exist, **`lib` is a documentation / tooling role only**:

- Verify still seed-compiles the file as a full program.  
- For the M10 pilot corpus, every unit (including `lib`) must be **seed-valid complete source** (has `world` and a total `main` if that is required by the language for standalone compile).  
- The role is retained so AI/human tools can label intent and so a future module ADR can attach meaning without renumbering roles.

If a future ADR allows library units without `main`, that is a **language** change and out of M10 scope.

## 5. Path grammar (document form)

Canonical document path:

```text
segment     ::= [A-Za-z0-9._-]+   except "." and ".."
rel-path    ::= segment ("/" segment)* ".ae"   ; actually ends with .ae on last segment
unit-path   ::= segment ("/" segment)*          ; last segment must end with .ae
```

Examples **accepted**:

- `main.ae`  
- `src/main.ae`  
- `lib/math_util.ae`  
- `vendor/local_fixture.ae`

Examples **rejected**:

- `../escape.ae`  
- `/abs/main.ae`  
- `C:\x\main.ae`  
- `src\\main.ae` (backslash)  
- `src//main.ae`  
- `src/./main.ae`  
- empty, non-`.ae`, URL-like `file:x.ae`

**Normalization:** The document path is the identity key for locks and reports.
The host joins project root + path with OS semantics after splitting on `/`.
No silent rewrite of `\` to `/` in the document (reject instead).

## 6. Schema delta (`aether.project/v1`)

Keep schema id **`aether.project/v1`** (expand allowed path pattern; do not bump to
v2 solely for nested paths). Document the expansion in schema `description`.

Path property pattern (conceptual):

```text
^(?:[A-Za-z0-9_-]+(?:\.[A-Za-z0-9_-]+)*)(?:/(?:[A-Za-z0-9_-]+(?:\.[A-Za-z0-9_-]+)*))*\.ae$
```

(Equivalent structural checks in Rust must match the schema.)

No new required fields. Optional fields that would enable linking (`depends_on`,
`exports`, `package`) are **not** added in M10 — `deny_unknown_fields` continues
to reject them so premature clients fail closed.

## 7. CLI surface

### 7.1 Existing (clarified)

```text
aether project verify <project-file> [--output-dir <dir>]
```

- Validates multi-unit + nested paths.  
- Seed-compiles each unit independently.  
- With `--output-dir`, writes artifacts:

| Unit path | Artifact file name |
| --- | --- |
| `main.ae` | `main.aeth` |
| `src/main.ae` | `src__main.aeth` (path separators → `__`) |
| `lib/util.ae` | `lib__util.aeth` |

Deterministic, flat output directory (no nested write tree in pilot) to avoid
accidental directory creation complexity. Alternative nested mirror may be a
later option; pilot chooses **flat mapped names**.

### 7.2 New: project format

```text
aether project format <project-file> [--write]
```

| Mode | Behavior |
| --- | --- |
| default | Print a multi-section report or emit formatted text to stdout per unit with clear headers (implementation pick one; tests pin it). **Recommended:** write nothing; print `=== path ===` + canonical source for each unit in declaration order. |
| `--write` | Overwrite each unit file in place with canonical format after validation that the unit still seed-compiles (or format-only via bootstrap formatter then re-verify — prefer **format via bootstrap formatter then optional verify**). |

Authority: `--write` only touches unit paths already listed in the project
document (still under root).

### 7.3 Not in M10

- `project add` / `project lock` generators (nice-to-have; may land if vertical
  slice is cheap, not required for Done)  
- `project run` (ambiguous which unit)  
- Dependency installers  

## 8. Shipped corpus

| Path | Content |
| --- | --- |
| `examples/project-multi/aether.project.json` | main + one lib, nested paths, full lock |
| `examples/project-multi/src/main.ae` | standalone program (role main) |
| `examples/project-multi/lib/helper.ae` | standalone program (role lib) |
| Keep `examples/project/` | single-unit regression |

Both units must dual-compare seed≡bootstrap as ordinary examples **or** only
via project verify (prefer also listing as ordinary compile if paths are under
`examples/` — nested examples may be project-only).

## 9. Implementation plan (after this design is accepted)

Vertical slice order (`CONST-DEP-001`):

1. Path grammar + schema update + unit tests (escape, nested accept).  
2. Multi-unit lock verify tests.  
3. Artifact naming helper + `--output-dir` multi-unit write tests.  
4. `project format` CLI + tests.  
5. Ship `examples/project-multi`.  
6. DOC-SYNC: MANIFEST, AETHER_0.12 or 0.13 note, ROADMAP, CORE_CLAIMS, SEED_PROFILE if needed.  
7. Gate: `aether-gate -Mode quick` + project-multi verify.  
8. Delivery report + Rule ID self-audit.

**Package versioning recommendation:** implement as **0.13.0 tooling** while
language surface stays **0.11** / AETH **v11**, unless human prefers staying on
0.12 with a patch. Design does not force a language AETH bump.

## 10. Seed impact

**None required** for M10 if units remain independent complete programs.

Seed dual-compare applies per unit source only. Do not claim multi-file seed
programs.

## 11. Non-goals (explicit)

| Non-goal | Deferred to |
| --- | --- |
| `import` / modules / cross-file weaves | Language ADR (post-M10) |
| Multi-package graphs / path deps across project roots | Later tooling ADR |
| Network registry | Blocked by law without threat model rewrite |
| Full LSP | Separate track |
| Lib units without `main` | Language + seed decision |

## 12. Acceptance for this design package (SOP 4–6)

This design package is **Done for planning** when:

1. ADR-013 accepted (this delivery proposes **Accepted** under P0 P4.1 queue).  
2. M10 validation matrix lists positive/negative cases.  
3. ROADMAP marks M10 as designed, implementation next.  
4. No code lands until an implement turn runs the vertical slice against the matrix.

---

*End of DESIGN-M10-MULTI-UNIT-PROJECTS.md*
