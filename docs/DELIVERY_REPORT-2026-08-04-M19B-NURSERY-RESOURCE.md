# Delivery Report — M19b nursery × resource Policy A

**Title:** Aether 0.26.0 nursery with parent resources  
**Date:** 2026-08-04  

## Shipped

- Design DESIGN-M19B + ADR-028 + matrix + AETHER_0.26  
- Bootstrap: remove weave-level resource∧nursery ban  
- Nursery site boundary: live owners OK; access loans rejected  
- Spawn callee resource-free check (`AE-TASK-003`)  
- Example `examples/nursery-resource.ae` (exit 7)  
- Dual-compare seed≡bootstrap for mix corpus  
- Package version **0.26.0**  

## Not shipped

- Policy B cancel-destroy of spawn-local owners  
- Resourceful spawn callees  
- Free-on-raise  
- FFI (still blocked)  

## Verify

```powershell
cargo test -p aether-core --lib m19b_
cargo test -p aether-core --test seed_self_host m19b_
cargo run -p aether-cli -- compile examples/nursery-resource.ae --output target/nr.aeth
cargo run -p aether-cli -- run target/nr.aeth
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- Design → ADR → matrix → impl → dual-compare → DOC-SYNC  
- No FFI without human authorize (ADR-025)  
- No fake Policy B claim  

*End of delivery report.*
