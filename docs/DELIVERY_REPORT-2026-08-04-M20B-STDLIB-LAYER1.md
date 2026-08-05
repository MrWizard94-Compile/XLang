# Delivery Report — M20b stdlib layer 1

**Title:** Aether 0.27.0 pure Whole/Truth/Text stdlib expansion  
**Date:** 2026-08-04  

## Shipped

- DESIGN-M20B + ADR-029 + matrix + AETHER_0.27  
- Expanded `stdlib/whole.ae` (`dec`, `abs`, `max`, `min`, `clamp_nonneg`)  
- New pure modules `truth.ae`, `text.ae`  
- Multi-import demo (exit 42)  
- `whole_test.ae` for `aether test`  
- CLI regression for project build  

## Not shipped

- Host I/O stdlib  
- Registry packaging  
- FFI  

## Verify

```powershell
cargo test -p aether-cli shipped_stdlib
cargo run -p aether-cli -- project build stdlib/aether.project.json --output target/stdlib.aeth
cargo run -p aether-cli -- run target/stdlib.aeth
cargo run -p aether-cli -- test stdlib/whole_test.ae
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- Offline pure only (no host authority expansion)  
- Design → ADR → matrix → impl → DOC-SYNC  

*End of delivery report.*
