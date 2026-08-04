# Aether 0.13 — Multi-unit offline project tooling

**Status:** Implemented (toolchain package **0.13.0**)  
**Language surface:** unchanged **0.11** / AETH **v11**  
**Decision:** [ADR-013](ADR-013-m10-multi-unit-projects.md)  
**Design:** [DESIGN-M10-MULTI-UNIT-PROJECTS.md](DESIGN-M10-MULTI-UNIT-PROJECTS.md)  
**Matrix:** [M10-VALIDATION-MATRIX.md](M10-VALIDATION-MATRIX.md)

## Summary

Package 0.13 extends M9 offline projects:

1. Nested relative unit paths (`src/main.ae`) with forward-slash grammar.  
2. Real multi-unit examples with full SHA-256 locks.  
3. Independent seed-compile of each unit (no language modules).  
4. `aether project format` (stdout sections or `--write`).  
5. `project verify --output-dir` uses flat mapped names (`src/main.ae` → `src__main.aeth`).

## Non-claims

- No `import` / cross-file weave resolution.  
- No package registry or multi-root package graphs.  
- No lib unit without a complete standalone program.  
- No AETH version bump (still v11 default emit).

## CLI

```text
aether project verify <project-file> [--output-dir <dir>]
aether project format <project-file> [--write]
```

## Example

`examples/project-multi/` — main + lib nested units with lock.
