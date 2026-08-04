# Delivery Report — M20 stdlib + M22 cross-package import (+ M19/M21 designs)

**Title:** Aether 0.24.0 stdlib layer 0 and workspace package imports  
**Date:** 2026-08-04  

## Shipped

- M20: `stdlib/` pure Whole helpers + project build  
- M22: `import unit "…" from package name as alias` + `workspace build --package`  
- M19: design-only ADR-023 (deeper T-RX deferred)  
- M21: design-only ADR-025 + threat v3 draft (FFI blocked)  

## Verify

```powershell
cargo test -p aether-core --lib
cargo test -p aether-cli
cargo run -p aether-cli -- project build stdlib/aether.project.json --output target/stdlib.aeth
cargo run -p aether-cli -- workspace build examples/workspace/aether.workspace.json --package app --output target/ws_app.aeth
cargo run -p aether-cli -- run target/ws_app.aeth
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Honesty

FFI is **not** implemented. Deeper abortive resource×effect is **not** implemented.

*End of delivery report.*
