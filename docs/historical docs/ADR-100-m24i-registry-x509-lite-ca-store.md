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
2. **`store_x509_lite_certificate`** issues, signature-verifies, validates the
   real Gregorian certificate window, and persists only a chain that is valid
   on the operator's UTC date. A store holds at most 256 certificates and has
   one certificate per subject.
3. **`verify_x509_lite_store_chain`** walks issuer→subject links (depth ≤ 8),
   detects cycles, and accepts a self-signed anchor only when its active trust
   key is explicitly marked `is_root`.
4. **`verify_x509_lite_store`** checks every persisted certificate. The normal
   `registry verify-cache` path invokes it before package-cache verification,
   so a malformed, expired, or unanchored CA store fails the cache gate.
5. The CLI exposes `registry trust-root` and
   `registry certify-ed25519-key` for the explicitly supplied 32-byte local
   seed files that prepare the M24f/g trust chain, plus
   `registry store-x509-lite` and `registry verify-x509-lite-store`.
   Certificate output is written only to an explicit `--output` path,
   otherwise emitted as PEM on standard output.
6. Certificate-content failures use `AE-REG-012`; store/chain failures use
   `AE-REG-013`.
7. Tracker: `registry_x509_lite_ca_store() == true`.

## Honesty

- Still not X.509 DER / RFC 5280.  
- Complements M24g key certification; does not replace pin signatures.  
- The cache-local store is a bounded local integrity adjunct, not a general
  purpose public-key infrastructure or an authority to fetch packages.

## Links

- ADR-096, ADR-092, M24A-VALIDATION-MATRIX  

---

*End of ADR-100.*
