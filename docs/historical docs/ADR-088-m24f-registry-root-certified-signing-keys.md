# ADR-088: M24f — root-certified signing keys (lightweight trust chain)

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-10)  
**Depends on:** ADR-077, ADR-080, ADR-084  

## Context

M24e multi-root policy is not a CA. Operators need a **certify signing key by
root** step without full X.509.

## Decision

1. Trust keys may be **`is_root`** (Ed25519).  
2. **`install_trust_root`** installs root keys.  
3. **`install_certified_signing_key`** installs a leaf Ed25519 key with
   `certified_by` + `certification` (root signature over  
   `certify\\n{key_id}\\ned25519\\n{sha256(key_material)}`).  
4. Sign/verify paths validate certification chains (`AE-REG-011`).  
5. Tracker: `registry_root_certified_signing_keys() == true`.  

## Honesty

- Not X.509 / RFC 5280 CA hierarchy.  
- Uncertified keys remain allowed unless policy later requires roots-only.  

## Links

- ADR-038, ADR-084, M24A-VALIDATION-MATRIX  

---

*End of ADR-088.*
