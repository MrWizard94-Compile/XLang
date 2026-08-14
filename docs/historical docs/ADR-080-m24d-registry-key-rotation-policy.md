# ADR-080: M24d — registry trust key rotation and validity policy

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-10)  
**Depends on:** ADR-060, ADR-074, ADR-077  

## Context

M24c added Ed25519 and HTTPS fetch. Operators need fail-closed revoke/rotate and
optional validity windows without inventing a full CA hierarchy.

## Decision

1. Trust keys gain: `not_before`, `not_after` (`YYYY-MM-DD`), `revoked`,
   `rotated_from`.  
2. **APIs / CLI:**  
   - `revoke_trust_key` / `registry revoke-key`  
   - `rotate_trust_key` / `registry rotate-key` (installs new, revokes old)  
   - `set_trust_key_validity` / `registry set-key-validity`  
3. Sign and verify **fail closed** on revoked or out-of-window keys (`AE-REG-009`).  
4. Tracker: `registry_key_rotation_policy() == true`.  

## Honesty

- Not a CA / certificate chain.  
- Host clock used for validity windows.  
- Packages signed under a later-revoked key fail `verify-cache` until re-pinned.  

## Links

- ADR-038, ADR-077, M24A-VALIDATION-MATRIX  

---

*End of ADR-080.*
