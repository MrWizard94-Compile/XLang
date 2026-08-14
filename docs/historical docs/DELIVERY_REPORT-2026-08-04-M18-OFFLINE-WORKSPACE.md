# Delivery Report — M18 offline workspace

**Title:** Aether 0.23.0 multi-package workspace verify  
**Date:** 2026-08-04  
**Summary:** `aether.workspace/v1` + `aether workspace verify` with path jail, acyclic `depends_on`, nested project verify. No language cross-package linking.

## Verify

```powershell
cargo test -p aether-core workspace::
cargo test -p aether-cli shipped_workspace
cargo run -p aether-cli -- workspace verify examples/workspace/aether.workspace.json
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M18) |
| TEST-BEHAVIOR-001 | Pass |
| SEC-INPUT-001 | Pass (path jail) |
| DOC-SYNC-001 | Pass (0.23) |

*End of delivery report.*
