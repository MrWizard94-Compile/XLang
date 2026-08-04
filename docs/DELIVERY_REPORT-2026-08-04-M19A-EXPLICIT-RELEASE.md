# Delivery Report — M19a explicit release

**Title:** Aether 0.25.0 `release` (bootstrap)  
**Date:** 2026-08-04  

## Shipped

- Statement `release name`, OP_RELEASE (66), verifier, VM  
- Raise-after-release proof (`examples/release-raise.ae`)  
- Honest seed limitation documented  

## Not shipped

- Seed emission / dual-compare for RELEASE  
- Nursery×resource  
- FFI (still blocked)  

## Verify

```powershell
cargo test -p aether-core --lib m19a_
cargo run -p aether-cli -- compile examples/release-raise.ae --output target/rr.aeth --bootstrap
cargo run -p aether-cli -- run target/rr.aeth
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- No FFI without human authorize (ADR-025)  
- No fake seed parity claim for release  
- Design → ADR-027 → matrix → bootstrap impl  

*End of delivery report.*
