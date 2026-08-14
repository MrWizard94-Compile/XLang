# Delivery Report — deferred tracks: M17c grants-in-tests + M19c Policy B design

**Title:** Aether 0.29.0 test grants; Policy B design-only  
**Date:** 2026-08-04  

## Shipped (product)

- M17c: optional `--grant-*` on `aether test` and `aether project test`  
- Default pure behavior unchanged  
- Design ADR-031 + matrix + AETHER_0.29  

## Shipped (design only)

- M19c Policy B cancel-destroy design + ADR-032  
- Honest preconditions; **no Policy B code**  

## Not shipped (still blocked)

- **FFI / M21** — ADR-025 requires human implement authorization after threat residual acceptance  
- Policy B product implementation  
- Native/registry law forks  
- JUnit/XML test reports  

## Verify

```powershell
cargo test -p aether-cli test_runner
cargo test -p aether-cli grants
cargo run -p aether-cli -- test examples/tests
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- FFI not implemented without human authorize  
- No fake Policy B Done claim  
- Grants default-deny  

*End of delivery report.*
