# ADR-084: M24e — multi-root registry trust policy

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-10)  
**Depends on:** ADR-060, ADR-074, ADR-077, ADR-080  

## Context

M24d added per-key revoke/rotate/validity. Operators need a **cache-level policy**
for multi-root stores (require signatures, algorithm allowlists, preferred roots)
without a full X.509 CA.

## Decision

1. Policy file `aether.registry-trust-policy/v1` under cache root.  
2. Fields: `require_signature`, `allow_hmac`, `allow_ed25519`, `max_active_keys`,
   `preferred_roots`.  
3. `verify_registry_cache` enforces policy (`AE-REG-010` on violation).  
4. Tracker: `registry_multi_root_trust_policy() == true`.  

## Honesty

- Not a certificate authority / chain-of-trust PKI.  
- Preferred roots must be installed; missing preferred root fails closed.  

## Links

- ADR-038, ADR-080, M24A-VALIDATION-MATRIX  

---

*End of ADR-084.*
