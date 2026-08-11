# ADR-077: M24c — Ed25519 trust keys + HTTPS fetch-signed

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY (reaffirmed 2026-08-10)  
**Threat:** [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Depends on:** ADR-060, ADR-074  

## Context

M24b used HMAC shared secrets and `http://`/`file://` only. Roadmap M24c asked
for PKI-style signatures and HTTPS under explicit operator fetch.

## Decision

1. **Algorithms:** trust keys carry `algorithm`: `hmac-sha256` (default) or
   `ed25519`.  
2. **Ed25519:** `generate_ed25519_trust_key` / `install_trust_key_with_algorithm`;
   pin binding signed with Ed25519 over `name\\nversion\\nsha256`.  
3. **HTTPS:** `fetch_signed_package` accepts `https://` via `ureq` TLS; signature
   verified before cache install.  
4. **Never** invoked by compile/run/project build. Guest AETH remains network-free.  
5. Tracker: `registry_ed25519_https_pilot() == true`.  

## Honesty

- Operator-held Ed25519 seeds stored under path-jailed cache trust roots (not a
  full CA/PKI hierarchy).  
- Dependencies: `ed25519-dalek`, `ureq` (TLS).  

## Links

- ADR-038, ADR-074, M24A-VALIDATION-MATRIX  

---

*End of ADR-077.*
