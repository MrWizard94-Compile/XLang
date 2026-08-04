# Delivery Report — M16 resource ↔ handled effect

**Title:** Aether 0.21.0 terminal handle over live resources  
**Date:** 2026-08-04  
**Summary:** Total weaves may own arenas/buffers/tables and terminal-handle pure Error[Whole] callees. Abortive raise/forward and nurseries remain resource-free. Seed dual-compare for `resource-handle` green.

## Verify

```powershell
cargo test -p aether-core --lib m16_
cargo test -p aether-core --test seed_self_host seed_profile_compiler_forges_m16
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M16) |
| TEST-BEHAVIOR-001 | Pass |
| DOC-SYNC-001 | Pass (0.21) |
| DOC-ADR-001 | Pass (ADR-020) |

*End of delivery report.*
