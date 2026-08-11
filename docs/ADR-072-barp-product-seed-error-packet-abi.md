# ADR-072: BARP — product seed-error packet ABI (ADR-061 first vertical)

**Status:** Accepted — implemented (host packet ABI)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [ADR-061](ADR-061-barp-seed-error-packets-and-multifile-forge-direction.md)  

## Context

ADR-061 directed seed-internal structured error packets. Product diagnostics
already surface stable `AE-SEED-*` codes (ADR-055), but tooling lacked a
versioned **packet** envelope for machine consumers and a decode path for
future seed-emitted packets.

## Decision

1. **Schema** `aether.seed-error/v1` with fields: `schema`, `code`, `message`,
   `line`, `column`, `origin`.  
2. **Host ABI:** `product_error_packets(source)` / `product_error_packets_json`
   normalize product diagnostics into packets.  
3. **Origin tags:**  
   - `host-preflight` — AE-SEED preflights (003–007, 012, 013)  
   - `host-classify` — forge/verify classification  
   - `seed-speak` — decoded when message contains `AETHER_SEED_ERROR:{json}`  
4. **Honesty trackers:**  
   - `product_seed_error_packet_abi() == true`  
   - `seed_internal_error_packets() == false` (seed binary still does not emit)  
5. Multi-file forge remains direction-only (`seed_native_multi_module_elaboration() == false`).  

## Consequences

### Positive

- Stable machine-readable product error packets without bootstrap AST  
- Forward-compatible decode for seed-speak packets  

### Residual

- Seed does not yet SPEAK packets (requires seed rebuild vertical)  
- Multi-file forge ABI still open  

## Links

- ADR-055, ADR-061, DESIGN-BARP-001  

---

*End of ADR-072.*
