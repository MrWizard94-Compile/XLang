# Delivery Report — M10 multi-unit offline projects

**Title:** Aether 0.13 multi-unit project tooling  
**Date:** 2026-08-04  
**One-sentence summary:** Nested multi-unit `aether.project/v1` integrity with independent per-unit seed compile, `project format`, and matrix-backed negatives — no language modules.

## Scope

- Path grammar (forward slash nested paths; reject `\`, `..`, absolute, unknown fields)  
- Multi-unit verify + lock + bad-unit fail-closed  
- Artifact naming `src/main.ae` → `src__main.aeth`  
- `project format` / `--write`  
- `examples/project-multi`  
- Package version **0.13.0** (language surface remains 0.11 / AETH v11)  

## Rule ID self-audit

| Rule ID | Status | Notes |
| --- | --- | --- |
| CONST-GATE-001 | Pass | Project unit tests + CLI exercise |
| CONST-DONE-001 | Pass | Vertical slice complete for M10 design |
| CONST-COMPLETE-001 | Pass | No partial multi-unit claim |
| CONST-DEP-001 | Pass | Design/ADR/matrix before code |
| ENG-WARN-001 | Pass | clippy `-D warnings` |
| TEST-BEHAVIOR-001 | Pass | Matrix cases in `project::tests` |
| DOC-SYNC-001 | Pass | MANIFEST, ROADMAP, claims, AETHER_0.13 |
| SEC-INPUT-001 | Pass | Path escape / unknown field / lock fail closed |
| REV-PACK-001 | Pass | This delivery report |
| DOC-ADR-001 | Pass | ADR-013 |

## How to verify

```powershell
cargo test -p aether-core --lib project::
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings
cargo run -p aether-cli -- project verify examples/project-multi/aether.project.json
cargo run -p aether-cli -- project format examples/project-multi/aether.project.json
cargo run -p aether-cli -- project verify examples/project/aether.project.json
```

## Suggested commit message

```
feat(tooling): implement Aether 0.13 multi-unit offline projects (M10)
```

## Risks / trade-offs

- `lib` remains a tooling role; each unit must be a complete program.  
- No cross-unit linking — intentional honesty until a modules ADR.

## Next actions

1. Optional: re-stage technical preview package at 0.13.  
2. Language modules remain a separate SOP loop if desired.
