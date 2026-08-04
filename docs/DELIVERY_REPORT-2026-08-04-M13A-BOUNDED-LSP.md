# Delivery Report — M13a bounded offline LSP

**Title:** Aether 0.17.0 `aether lsp`  
**Date:** 2026-08-04  
**Summary:** Stdio JSON-RPC language server with bootstrap diagnostics, document symbols, formatting, hover, and definition. No product AETH emission; no server disk writes.

## Scope

- CLI: `aether lsp`  
- initialize capabilities  
- didOpen/didChange/didClose → publishDiagnostics  
- documentSymbol, formatting, hover, definition (single-file)  
- Unit tests for handlers  

## Rule ID self-audit

| Rule ID | Status |
| --- | --- |
| CONST-DONE-001 | Pass (M13a) |
| TEST-BEHAVIOR-001 | Pass |
| DOC-SYNC-001 | Pass |
| SEC-INPUT-001 | Pass (stdio only; no silent writes) |
| ENG-WARN-001 | Pass |

## Verify

```powershell
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings
# Manual: configure editor to run `aether lsp` over stdio
```

## Honesty

Editor diagnostics are **bootstrap**. Product compile and apply-edit remain **seed-hosted**.

## Next

M13b project-aware imports; or host I/O threat model track.
