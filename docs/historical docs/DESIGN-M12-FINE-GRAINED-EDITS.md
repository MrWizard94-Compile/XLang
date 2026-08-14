# M12 Design: fine-grained structural edits (T-EDIT)

**Status:** Implemented (package 0.16.0)  
**Date:** 2026-08-04  
**Decision record:** [ADR-016](ADR-016-m12-fine-grained-edits.md)  
**Validation record:** [M12 validation matrix](M12-VALIDATION-MATRIX.md)  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) track **T-EDIT**  
**Depends on:** Authoring v6, M11 modules (import/export in AST), seed compile-before-write  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`

---

## 1. Purpose and boundary

Today `aether.edit/v6` only supports **top-level** declaration ops:

- `replace` / `insertAfter` / `delete` on `world` | `record:name` | `shape:name` | `weave:name`

AI agents that need to change a single `bind` or `yield` inside `main` must
replace the entire weave. That is fragile and over-broad.

**M12** adds **statement-level** structural edits inside weave bodies, still:

- Exact `baseSource` stale guard  
- Typed JSON payloads (not free-form JSON-Path scripts)  
- Formatter reparse + **seed compile before write**  
- Bounded ops/nodes  
- No model/network authority  

M12 does **not**:

- Allow arbitrary expression-subtree surgery (e.g. rewrite one atom deep in `sum`)  
- Invent a general query language over the AST  
- Grant LSP authority (T-LSP is separate)  
- Bypass validation or seed compile  

---

## 2. Core claim

> An agent may replace, insert, or delete **whole statements** at typed paths
> under a weave body. The resulting program is reparsed to canonical form and
> seed-compiled before any source write. Invalid paths, kind mismatches, and
> stale bases fail closed.

**Falsifiers:** partial expression edits accepted; write without seed compile;
path escape into non-statement nodes; unbounded operation count.

---

## 3. Protocol versions

| Artifact | Version |
| --- | --- |
| Edit protocol | **`aether.edit/v7`** |
| AST schema | **`aether.ast/v7`** |
| Diagnostics | **`aether.diagnostic/v7`** (new codes only; prior codes preserved) |

v6 remains accepted for top-level-only clients **or** is rejected with clear
upgrade error — **ADR-016 chooses: reject v6 on apply-edit for product 0.16+
with message to use v7** (simpler dual support optional later).

Recommended package pin at ship: **0.16.0**.

---

## 4. Design decisions

### D1 — Statement paths (typed, not JSONPath)

Path grammar (string targets):

```text
path := "weave:" name "/body/" index
      | "weave:" name "/body/" index "/whenBright/" index
      | "weave:" name "/body/" index "/whenDim/" index
      | "weave:" name "/body/" index "/body/" index          ; while body
```

| Rule | Detail |
| --- | --- |
| `name` | Existing weave name, lowercase identifier |
| `index` | Decimal non-negative integer, no leading zeros except `0` |
| Depth | At most **2** nested body levels in M12 (choose/while children only) |
| Together | **Out of scope** for M12 (spawn lines stay whole-weave replace) |

Invalid path → `AE-EDIT-010`.

### D2 — New operations (in addition to v6 top-level)

| `op` | Fields | Meaning |
| --- | --- | --- |
| `replaceStatement` | `path`, `statement` | Replace statement at path |
| `insertStatementAfter` | `path`, `statement` | Insert after path (path may be `weave:name/body/-1` meaning before first? **No** — use `insertStatementAt`) |
| `insertStatementAt` | `pathPrefix`, `index`, `statement` | Insert at index in that body list (`index == len` appends) |
| `deleteStatement` | `path` | Delete statement at path |

Where `pathPrefix` is `weave:name/body` or `weave:name/body/N/whenBright` etc.

**Simpler unified form (normative for ADR-016):**

| `op` | Fields |
| --- | --- |
| `replaceStatement` | `path` + `statement` |
| `insertStatementAfter` | `path` + `statement` — if path ends with `/body` only (no index), insert at start? **Reject.** Path must include index; insert **after** that index. For insert at start, use `insertStatementAt` with index 0. |
| `insertStatementAt` | `list` + `index` + `statement` where `list` is `weave:name/body` or nested list path without trailing index |
| `deleteStatement` | `path` |

### D3 — Statement payload

`statement` is a **typed JSON object** matching `aether.ast/v7` statement defs
already produced by `structure` (kind Bind, Revise, Speak, Yield, Raise,
Forward, Handle, Choose, While).  

M12 **accepts** all statement kinds that structure emits, including Choose/While
(whole statement replace). Nested partial edit uses nested paths.

`Together` statements: **replace whole Together only** via path to that
statement index; no per-spawn edit in M12.

### D4 — Preserved top-level ops

v7 still supports:

- `replace` / `insertAfter` / `delete` for `world` | `record:x` | `shape:x` | `weave:x` | **`hostWeave:x`** | **`import:alias`** (new)

### D5 — Module-aware targets (v7)

| Target | Ops |
| --- | --- |
| `import:alias` | replace / delete / insertAfter world |
| Weave with `exported: true` | top-level replace includes export flag |
| `import unit` declarations | ordered after world in AST |

### D6 — Safety limits

| Limit | Value |
| --- | --- |
| Max operations | 32 (unchanged) |
| Max structural nodes touched | existing `MAX_STRUCTURAL_EDIT_NODES` or raise to 512 if needed |
| Max path depth | 6 path segments |
| Max body length after edit | same as source byte cap |

### D7 — Application pipeline (unchanged order)

1. Parse edit JSON (strict)  
2. Exact `baseSource` match against caller source  
3. Parse base to program  
4. Apply ops in order  
5. Format canonical  
6. Reparse validate  
7. **Seed compile**  
8. Return source (+ optional diagnostics)  

### D8 — Stop conditions

- Expression-level atom rewrite  
- Regex/text patch ops  
- Paths into parameter lists or type positions  
- Silent success when seed compile fails  

---

## 5. Invariants

| ID | Invariant |
| --- | --- |
| M12-INV-001 | Only v7 protocol accepted on product apply-edit after ship |
| M12-INV-002 | Statement paths address statements only |
| M12-INV-003 | Stale baseSource rejected |
| M12-INV-004 | Seed compile required before write |
| M12-INV-005 | Kind mismatch (e.g. Record payload on statement path) rejected |
| M12-INV-006 | Index out of range rejected |
| M12-INV-007 | Nested path only into Choose/While as specified |
| M12-INV-008 | No network/model authority |

---

## 6. Diagnostics

| Code | Meaning |
| --- | --- |
| `AE-EDIT-010` | Invalid or unsupported path |
| `AE-EDIT-011` | Statement index out of range |
| `AE-EDIT-012` | Statement kind/payload mismatch |
| `AE-EDIT-013` | Nested edit not allowed at this node (e.g. Together spawn) |

Prior `AE-EDIT-001..006` retained.

---

## 7. Example

Base:

```aether
world app

weave main [] -> Whole:
  bind n <- 20
  yield n
```

Edit: replace statement `weave:main/body/1` with Yield of `sum n 1` → program exit 21 after apply.

---

## 8. Implementation plan

1. Schemas `aether-ast-v7`, `aether-edit-v7`, `aether-diagnostic-v7`  
2. Structure emission includes stable statement indices in ids if not already  
3. Authoring apply path resolution + statement decode from JSON  
4. Module import nodes in AST if missing  
5. Core/CLI tests per matrix  
6. Protocol doc `AETHER_AUTHORING_PROTOCOL_v7.md`  
7. Package 0.16.0  

---

## 9. Non-goals

| Non-goal | Track |
| --- | --- |
| LSP | T-LSP |
| Full expression zipper edits | Later |
| Multi-file project edit in one request | Later (per-file apply-edit remains) |

---

*End of DESIGN-M12-FINE-GRAINED-EDITS.md*
