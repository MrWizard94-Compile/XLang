# Delivery Report — M19a explicit release

**Title:** Aether 0.25.0 `release` (seed product path)  
**Date:** 2026-08-04  

## Shipped

- Statement `release name`, OP_RELEASE (66), verifier, VM  
- Raise-after-release proof (`examples/release-raise.ae`)  
- Seed emission of RELEASE (inline emit + local slot lookup)  
- Seed≡bootstrap dual-compare for release corpus; self-host forge identity  

## Not shipped

- Nursery×resource  
- FFI (still blocked)  

## Verify

```powershell
cargo test -p aether-core --lib m19a_
cargo test -p aether-core --test seed_self_host m19a_release
cargo run -p aether-cli -- compile examples/release-raise.ae --output target/rr.aeth
cargo run -p aether-cli -- run target/rr.aeth
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- No FFI without human authorize (ADR-025)  
- Seed parity claimed only after dual-compare green  
- Design → ADR-027 → matrix → bootstrap + seed  

*End of delivery report.*
