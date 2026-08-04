# Delivery Report — M17 offline test runner

**Title:** Aether 0.22.0 `aether test`  
**Date:** 2026-08-04  
**Summary:** Offline discovery of `*_test.ae` (and explicit `.ae` files), seed-compile, pure run; pass requires exit 0. Symlinks skipped; empty discovery fails closed.

## Verify

```powershell
cargo test -p aether-cli
cargo run -p aether-cli -- test examples/tests
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M17) |
| TEST-BEHAVIOR-001 | Pass |
| SEC-INPUT-001 | Pass (path confinement / symlink skip) |
| DOC-SYNC-001 | Pass (0.22) |

*End of delivery report.*
