# ADR-074: M24b — signed registry pins + explicit fetch-signed

**Status:** Accepted — implemented (HMAC pilot)  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-REGISTRY ([HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md); reaffirmed 2026-08-10)  
**Threat:** [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Depends on:** ADR-060  

## Context

M24a provided offline digest-bound cache pin/verify with **no** network and no
signatures. Roadmap M24b required signed packages and explicit fetch under
operator control.

## Decision

1. **Trust roots (offline):** `aether.registry-trust/v1` under the cache root;
   HMAC key material at path-jailed `trust/keys/<key_id>.hmac`.  
   CLI: `aether registry trust-key <cache> --key-id ID --key-file PATH`.  
2. **Signed pins:** pin binding `HMAC-SHA256(key, name || "\\n" || version || "\\n" || sha256)`.  
   CLI: `aether registry pin-local-signed … --key-id ID`.  
   `verify-cache` verifies digest **and** signature when present.  
3. **Explicit fetch-signed:**  
   `aether registry fetch-signed <cache> --name --version --url --signature --key-id`  
   - URL schemes: `file://`, bare local path, `http://` (minimal HTTP/1.0 GET)  
   - **No TLS** in this pilot (`https://` fails closed with guidance)  
   - Signature verified **before** cache install  
4. **Never** invoked by `compile` / `run` / `project build` / workspace build.  
5. Guest AETH remains network-free.  
6. Tracker: `registry_signed_fetch_pilot() == true`.  

## Honesty

- HMAC shared-secret pilot, not Ed25519/public-key PKI.  
- No redirect following; no HTTPS.  
- Not crates.io; not auto-fetch on compile.  

## Links

- ADR-038, ADR-060, DESIGN-LAW-FORK-F-REGISTRY, M24A-VALIDATION-MATRIX  

---

*End of ADR-074.*
