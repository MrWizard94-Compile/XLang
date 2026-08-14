# ADR-096: M24h — X.509-lite registry certificates

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-11)  
**Depends on:** ADR-088, ADR-092  

## Context

M24f/g provide lightweight root/intermediate/leaf key certification without
structured certificate documents. Operators need an exportable certificate
envelope closer to X.509 practice without claiming full RFC 5280.

## Decision

1. **Schema `aether.registry-x509-lite/v1`:** TBS fields (version, serial, issuer,
   subject, validity window, subject key id, public key algorithm + sha256).  
2. **Sign:** Ed25519 over canonical text TBS (`tbs_signing_message`).  
3. **API:** `issue_x509_lite_certificate`, `verify_x509_lite_certificate`,
   PEM-like `encode_x509_lite_pem` / `decode_x509_lite_pem` (`-----BEGIN AETHER CERT-----`,
   hex JSON body).  
4. **CLI:** `aether registry issue-x509-lite …`.  
5. **Code:** `AE-REG-012` on certificate failures.  
6. Tracker: `registry_x509_lite_certificates() == true`.  

## Honesty

- **Not** X.509 DER / RFC 5280 / ASN.1.  
- Ed25519 only; no OCSP/CRL.  
- Complements (does not replace) M24g multi-level key certification chains.  

## Links

- ADR-038, ADR-092, M24A-VALIDATION-MATRIX  

---

*End of ADR-096.*
