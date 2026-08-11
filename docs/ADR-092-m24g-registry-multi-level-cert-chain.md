# ADR-092: M24g — multi-level registry certification chain

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-11)  
**Depends on:** ADR-088  

## Context

M24f certified leaves by a root only. Operators need root → intermediate → leaf
chains without full X.509.

## Decision

1. **`install_certified_intermediate`** installs a non-root key certified by a root.  
2. **`install_certified_signing_key`** accepts roots **or** certified intermediates
   as parents.  
3. **`verify_key_certification_chain`** walks up to 8 levels until a root.  
4. Tracker: `registry_multi_level_cert_chain() == true`.  

## Honesty

- Still not X.509 / RFC 5280.  
- Ed25519 parent keys only.  

## Links

- ADR-038, ADR-088, M24A-VALIDATION-MATRIX  

---

*End of ADR-092.*
