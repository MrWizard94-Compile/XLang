# ADR-100: M24i — X.509-lite CA store + chain verify

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-11)  
**Depends on:** ADR-096  

## Context

M24h issues and verifies single X.509-lite certificates. Operators need a
cache-local store and multi-hop chain verification without full RFC 5280 DER.

## Decision

1. **Store schema** `aether.registry-x509-lite-store/v1` in
   `aether.registry-x509-lite-store.json`.  
2. **`store_x509_lite_certificate`** issues, signature-verifies, and persists.  
3. **`verify_x509_lite_store_chain`** walks issuer→subject links (depth ≤ 8)
   until a trust root key or self-signed root cert.  
4. Fail closed `AE-REG-013` on store/chain errors.  
5. Tracker: `registry_x509_lite_ca_store() == true`.  

## Honesty

- Still not X.509 DER / RFC 5280.  
- Complements M24g key certification; does not replace pin signatures.  

## Links

- ADR-096, ADR-092, M24A-VALIDATION-MATRIX  

---

*End of ADR-100.*
