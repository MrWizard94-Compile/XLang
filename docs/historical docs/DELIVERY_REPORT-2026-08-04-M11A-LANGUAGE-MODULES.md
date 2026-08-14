# Delivery Report — M11a language modules (bootstrap multi-module)

**Title:** Aether 0.14.0 / language modules M11a  
**Date:** 2026-08-04  
**One-sentence summary:** `import unit` / `export weave` / qualified calls with bootstrap `project build` and honest non-seed multi-module authority until M11b.

## Scope

- Parser/hooks for import reject on single-file, export weave, qualified call reject on single-file  
- `modules` elaboration + `compile_project_modules`  
- CLI `project build`  
- Lib without main validation  
- Example `examples/project-modules` (exit 42)  
- Package **0.14.0**  

## Rule ID self-audit

| Rule ID | Status | Notes |
| --- | --- | --- |
| CONST-GATE-001 | Pass | module/project tests + CLI |
| CONST-DONE-001 | Pass | M11a vertical slice |
| CONST-COMPLETE-001 | Pass | No fake seed multi-module claim |
| CONST-DEP-001 | Pass | ADR-015 design first |
| ENG-WARN-001 | Pass | clippy `-D warnings` |
| TEST-BEHAVIOR-001 | Pass | exit 42, cycle, private weave, AE-MOD-007 |
| DOC-SYNC-001 | Pass | AETHER modules note, claims, roadmap |
| SEC-INPUT-001 | Pass | path jail via project units |
| DOC-ADR-001 | Pass | ADR-015 |

## How to verify

```powershell
cargo test -p aether-core --lib modules::
cargo test -p aether-core --lib project::
cargo clippy -p aether-core -p aether-cli -- -D warnings
cargo run -p aether-cli -- project build examples/project-modules/aether.project.json --output target/modules.aeth
cargo run -p aether-cli -- run target/modules.aeth
# exited with 42
```

## Risks

- Elaboration is text-based rewrite (specified, deterministic); M11b may replace with deeper seed IR.  
- Lib units restricted to weaves-only in M11a.

## Next

M11b seed multi-module dual-compare before switching product authority.
