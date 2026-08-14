# Threat Model v3 — foreign / C ABI pilot (draft for T-FFI)

**Status:** Accepted for pilot residual risk (human authorize 2026-08-04); product pilot 0.31  
**Date:** 2026-08-04  
**Supersedes for FFI only:** does not replace TP pure freeze or host I/O v2  

## Assets

- Guest AETH integrity  
- Host process memory safety  
- Operator intent (who loads which library)

## Threats

| ID | Threat | Severity |
| --- | --- | --- |
| T1 | Arbitrary code via libloading | Critical |
| T2 | Ownership mismatch (double-free / UAF) | Critical |
| T3 | Header soup / silent ABI drift | High |
| T4 | Ambient library path search | High |
| T5 | Error/panic crossing into VM | High |

## Controls (required for any pilot)

1. **Explicit** library path operator grant (no PATH search).  
2. **Pinned** symbol signatures in Aether source (no C header parser v1).  
3. Ownership mapping: only copy primitives or explicitly transferred owned
   buffers with documented rules.  
4. Fail closed on signature mismatch.  
5. No FFI from seed compile path; host-only load after verify.  
6. Residual risk table accepted by human.

## Residual risk

Hostile `.so`/`.dll` still runs if operator grants path — same class as
running untrusted native code. Document; do not claim sandbox.

---

*End of THREAT_MODEL-v3-FOREIGN-ABI.md*
