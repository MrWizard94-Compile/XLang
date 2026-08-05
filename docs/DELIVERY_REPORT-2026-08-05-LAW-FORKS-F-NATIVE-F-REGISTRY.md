# Delivery Report — Law forks F-NATIVE / F-REGISTRY (decision packages only)

**Title:** Human decision packages for optional native backend and package registry  
**Date:** 2026-08-05  
**Package bump:** **None** (no product code)

## Scope (design only)

| Fork | Artifacts |
| --- | --- |
| **F-NATIVE** | DESIGN-LAW-FORK-F-NATIVE, THREAT v4 draft, HUMAN-AUTHORIZE-NATIVE, ADR-037 |
| **F-REGISTRY** | DESIGN-LAW-FORK-F-REGISTRY, THREAT v5 draft, HUMAN-AUTHORIZE-REGISTRY, ADR-038 |

## Explicit non-delivery

- No LLVM/cranelift product path  
- No registry network client  
- No CLM-010 / AGENTS invariant rewrite until human §3 phrases  

## How the human opens a fork

**Native:**

> I authorize Aether law fork F-NATIVE under ADR-037 / threat model v4 residual risk acceptance: optional native lower of **verified AETH only**, VM remains default and reference, no Aether-source transpile to other languages.

**Registry:**

> I authorize Aether law fork F-REGISTRY under ADR-038 / threat model v5 residual risk acceptance: signed offline-first registry with optional explicit fetch, guest AETH remains network-free, compile never requires network when cache is complete.

Either, both, or neither may be authorized. Authorization still requires a
follow-on vertical ADR before implementation.

## Constitution

- `CONST-DEP-001` — design before code  
- `SEC-INPUT-001` — residual risk explicit  
- `CLM-011` — no superiority claims  
- Mirrors M21 HUMAN-AUTHORIZE-FFI pattern  

*End of delivery report.*
