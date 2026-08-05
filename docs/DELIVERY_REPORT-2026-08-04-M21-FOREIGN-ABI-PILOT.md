# Delivery Report — M21 foreign ABI pilot

**Title:** Aether 0.31.0 foreign weave pilot (human-authorized) + seed dual-compare  
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
- **Seed parse+emit of foreign weaves** with encoded `\x1eF\x1e…` host names  
- **Seed≡bootstrap** dual-compare for foreign-pilot (product default compile)  
- Package **0.31.0**  

## Not shipped

- Text/Bytes C pointers  
- C headers / bindgen  
- Memory-safety claims for foreign code  
- Expanded arity/signatures beyond Whole pilot  

## Verify

```powershell
cargo test -p aether-core --lib m21_
cargo test -p aether-core --test seed_self_host seed_profile_compiler_forges_m21_foreign_pilot
cargo build -p aether-ffi-pilot --release
cargo run -p aether-cli --release -- compile examples/foreign-pilot.ae --output target/fp.aeth
# Windows:
cargo run -p aether-cli --release -- run target/fp.aeth --grant-lib pilot=target/release/aether_ffi_pilot.dll
cargo clippy -p aether-core -p aether-cli -p aether-ffi-pilot -- -D warnings
```

## Constitution

- Residual risk accepted by human  
- Explicit library path grant only  
- No “safe FFI” marketing  
- Honest seed dual-compare claimed only for the documented foreign-pilot corpus  

*End of delivery report.*
