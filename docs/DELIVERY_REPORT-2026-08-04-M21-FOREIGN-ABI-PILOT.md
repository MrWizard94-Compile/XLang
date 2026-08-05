# Delivery Report — M21 foreign ABI pilot

**Title:** Aether 0.31.0 foreign weave pilot (human-authorized)  
**Date:** 2026-08-04  

## Authorization

Human phrase (session):  
`I authorize Aether M21 foreign ABI pilot implementation under ADR-025 / threat model v3 residual risk acceptance.`

## Shipped

- `foreign weave … from "key" symbol "name"` (Whole-only pilot)  
- Runtime libloading with `--grant-lib KEY=PATH`  
- Pilot cdylib `aether-ffi-pilot` (`aether_whole_inc`, `aether_whole_sum`)  
- Example `examples/foreign-pilot.ae`  
- Core tests P1/N1/N2/N3  
- Package **0.31.0**  

## Not shipped

- Seed dual-compare for foreign  
- Text/Bytes C pointers  
- C headers / bindgen  
- Memory-safety claims for foreign code  

## Verify

```powershell
cargo test -p aether-core --lib m21_
cargo build -p aether-ffi-pilot
cargo run -p aether-cli -- compile examples/foreign-pilot.ae --output target/fp.aeth --bootstrap
# Windows:
cargo run -p aether-cli -- run target/fp.aeth --grant-lib pilot=target/debug/deps/aether_ffi_pilot.dll
cargo clippy -p aether-core -p aether-cli -p aether-ffi-pilot -- -D warnings
```

## Constitution

- Residual risk accepted by human  
- Explicit library path grant only  
- No “safe FFI” marketing  

*End of delivery report.*
