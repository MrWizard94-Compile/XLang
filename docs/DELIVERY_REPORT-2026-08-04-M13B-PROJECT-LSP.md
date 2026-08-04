# Delivery Report — M13b project-aware LSP

**Title:** Aether 0.18.0 project-aware import navigation  
**Date:** 2026-08-04  
**Summary:** `aether lsp --project` / `initializationOptions.projectFile` enables cross-file definition and hover for exported import weaves; path jail via project units; private weaves do not resolve.

## Verify

```powershell
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M13b) |
| TEST-BEHAVIOR-001 | Pass |
| SEC-INPUT-001 | Pass (project unit jail) |
| DOC-SYNC-001 | Pass |
