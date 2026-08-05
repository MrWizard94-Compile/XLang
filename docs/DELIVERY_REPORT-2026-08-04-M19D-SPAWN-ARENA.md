# Delivery Report — M19d multi-weave arenas + Policy B cooperative

**Title:** Aether 0.32.0 spawn-scoped / multi-weave arenas (T-RX)  
**Date:** 2026-08-04  

## Scope

- ADR-035 / DESIGN-M19D-SPAWN-SCOPED-ARENA  
- ADR-036 cooperative Policy B claim (no mid-frame cancel)  
- Package **0.32.0**  

## Shipped

- Total non-main arenas; header capacity = sum  
- Policy A+ resourceful total spawn callees  
- Seed capacity sum; dual-compare `examples/spawn-arena.ae`  
- Verifier multi-`OP_ARENA`  

## Not shipped

- Mid-frame cancel destroy engine  
- Free-on-raise  
- Capacity reclaim  

## Verify

```powershell
cargo test -p aether-core --lib m19 -- --nocapture
cargo test -p aether-core --test seed_self_host seed_hosted -- --nocapture
cargo run -p aether-cli --release -- compile examples/spawn-arena.ae --output target/sa.aeth
cargo run -p aether-cli --release -- run target/sa.aeth
pwsh -File .\tools\aether-gate.ps1 -Mode quick
```

## Constitution

- Design → ADR → matrix → impl → dual-compare → DOC-SYNC  
- Honest Policy B bounds (cooperative only)  

*End of delivery report.*
