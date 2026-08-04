# M18 Design: offline multi-package workspace (T-PKG)

**Status:** Accepted design for ADR-022 — **implemented in package 0.23.0**  
**Date:** 2026-08-04  
**Decision record:** [ADR-022](ADR-022-m18-offline-workspace.md)  
**Validation:** [M18-VALIDATION-MATRIX.md](M18-VALIDATION-MATRIX.md)  
**Depends on:** M9–M11 project + modules tooling  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) **T-PKG**  
**Rule IDs:** `SEC-INPUT-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`

---

## 1. Purpose and boundary

M10/M11 prove **one project root** with multi-unit modules. Real monorepos need
**several packages** under one workspace, still offline, still path-confined.

**M18** adds an **`aether.workspace/v1`** document that lists local package
roots (each with its own `aether.project.json`), optional **path dependency
edges** (`depends_on`), cycle rejection, and CLI **`workspace verify`** that
runs existing project verification in topological order.

**Not in M18**

- Network registry, URLs, version ranges, semver solve  
- Cross-package language `import` (still single-project M11)  
- Publishing, binary caches  
- Shared lock file across packages (each project keeps its own lock)  

---

## 2. Core claim

> A workspace lists local package directories under the workspace root. The host
> validates path confinement, unique package names, and an acyclic `depends_on`
> graph, then verifies each package’s `aether.project/v1` document with existing
> project rules. No network. No cross-package language linking.

---

## 3. Design decisions

### D1 — Document

```json
{
  "schema": "aether.workspace/v1",
  "name": "demo_workspace",
  "version": "0.1.0",
  "packages": [
    { "name": "util", "path": "packages/util" },
    { "name": "app", "path": "packages/app", "depends_on": ["util"] }
  ]
}
```

| Field | Rule |
| --- | --- |
| `schema` | exactly `aether.workspace/v1` |
| `name` / `version` | non-empty; safe charset like projects |
| `packages` | 1..=64 entries |
| `name` | unique; `[A-Za-z][A-Za-z0-9_]*` |
| `path` | relative dir segments `/`-separated; safe segments; no `..` |
| `depends_on` | optional list of package names; must exist; no self-edge |

Each package path must contain **`aether.project.json`** after join under the
workspace root (directory containing the workspace file).

### D2 — Path jail

Same spirit as project units: resolve under workspace root after canonicalize;
reject escapes and skip trusting symlinks that leave the root.

### D3 — Graph

- `depends_on` is **tooling order + integrity only** (not language imports).  
- Cycles → fail closed (`AE-WORKSPACE-003`).  
- Verify packages in topological order (dependees before dependents).

### D4 — CLI

```text
aether workspace verify <workspace-file>
```

Reports each package name/path and project verify summary. Fail closed on first
package failure (or collect all — pilot: fail on first for simplicity, or
collect — prefer **collect all failures** for DX).

Pilot: **verify all packages**, report all, nonzero exit if any fail.

### D5 — Diagnostics

| Code | Meaning |
| --- | --- |
| `AE-WORKSPACE-001` | Schema/JSON/limits |
| `AE-WORKSPACE-002` | Path/name/jail |
| `AE-WORKSPACE-003` | Graph (cycle, missing dep, duplicate name) |
| `AE-WORKSPACE-004` | Missing project file or nested project verify failure envelope |

### D6 — Package pin

**0.23.0** at ship.

### D7 — Stop conditions

- URL/registry resolution  
- Cross-package weave import without language ADR  
- Absolute package paths  
- Workspace scripts/macros  

---

## 4. Invariants

| ID | Invariant |
| --- | --- |
| M18-INV-001 | Schema `aether.workspace/v1` only |
| M18-INV-002 | Package paths confined under workspace root |
| M18-INV-003 | Unique names; valid depends_on; acyclic |
| M18-INV-004 | Each package has `aether.project.json` and passes project verify |
| M18-INV-005 | No network |
| M18-INV-006 | No language cross-package linking claim |

---

## 5. Implementation plan

1. Core `workspace` parse/verify API  
2. CLI `workspace verify`  
3. Example `examples/workspace/`  
4. Unit tests: cycle, escape, happy path  
5. DOC-SYNC  

---

*End of DESIGN-M18-OFFLINE-WORKSPACE.md*
