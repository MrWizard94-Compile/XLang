# Delivery Report — M17d structured reports + deferred design package

**Title:** Aether 0.30.0 test reports; M19d/M21 deferred design hygiene  
**Date:** 2026-08-04  

## Shipped (product)

- Optional `--report` JSON (`aether.test-report/v1`)  
- Optional `--report-junit` offline JUnit-compatible XML  
- Available on `aether test` and `aether project test`  
- Package **0.30.0**  

## Shipped (design / process only)

- M19d resourceful-spawn precondition design (ADR-034) — **no code**  
- `HUMAN-AUTHORIZE-FFI.md` checklist — **no FFI code**  

## Still blocked

| Item | Blocker |
| --- | --- |
| M21 FFI product | Human §3 authorize phrase |
| M19c Policy B product | M19d precondition product ADR |
| Native/registry | Law fork |

## Verify

```powershell
cargo test -p aether-cli report
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- No FFI without human authorize  
- No Policy B theater  
- Explicit report paths only  

*End of delivery report.*
