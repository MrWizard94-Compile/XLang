# Aether 0.24 Toolchain Contract (M20 stdlib layer 0 + M22 cross-package import)

**Status:** Historical package contract — language **0.11** / AETH **v11**; package **0.24**; current package contract is [AETHER_0.35.md](AETHER_0.35.md)
**Also documented:** M19/M21 design-only (deeper T-RX; FFI blocked)

## M20 — Stdlib layer 0

- Path: `stdlib/` multi-unit project (`whole.ae` exports `double`, `inc`, `is_zero`)  
- Demo entry: `stdlib/main.ae` imports whole, exit 42 via `double 21`  
- Pure modules only; no host I/O  

## M22 — Cross-package import

```aether
import unit "whole.ae" from package util as wh
```

- Authorized only when the consumer package `depends_on` that package (workspace).  
- CLI: `aether workspace build <ws.json> --package app --output out.aeth`  

## Design-only (not product code)

| Track | ADR | Note |
| --- | --- | --- |
| M19 deeper T-RX | ADR-023 | Needs destruction model first |
| M21 foreign ABI | ADR-025 + threat v3 | **Blocked** until human implement authorization |

## Evidence

- `stdlib/`, `examples/workspace/`, CLI workspace build test (exit 42)  

*End of AETHER_0.24.md*
