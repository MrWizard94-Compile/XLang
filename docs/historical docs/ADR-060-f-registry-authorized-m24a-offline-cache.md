# ADR-060: F-REGISTRY authorized — M24a offline registry cache verify

**Status:** Accepted — implemented (bounded pilot)
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Law fork:** [ADR-038](ADR-038-f-registry-law-fork.md), [HUMAN-AUTHORIZE-REGISTRY.md](../Current%20state/HUMAN-AUTHORIZE-REGISTRY.md)
**Threat:** [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](../Current%20state/THREAT_MODEL-v5-PACKAGE-REGISTRY.md) (binding for this path)

## Context

Human authorized F-REGISTRY (2026-08-10). First vertical slice: **offline** cache
integrity — no network client yet.

## Decision

1. **Fork open:** signed offline-first registry direction; guest AETH remains
   network-free; compile never requires network when cache is complete.
2. **M24a pilot:** local cache layout `aether.registry-cache/v1` under a
   path-jailed root; each package pin binds name, version, artifact path, SHA-256.
3. **CLI (no sockets):**
   - `aether registry pin-local <cache-root> --name N --version V --artifact PATH`
   - `aether registry verify-cache <cache-root>`
4. Fail closed on digest mismatch, path escape, missing artifact, bad schema.
5. **No network** in this pilot. Fetch/publish is a later ADR.
6. Trackers: `f_registry_authorized() == true`,
   `registry_offline_cache_verify() == true`.

## Honesty

- Not crates.io; not signed keys yet — digest-bound pins only (v1).
- Signature keys / fetch land in a later M24b slice.

## Links

- DESIGN-LAW-FORK-F-REGISTRY §6, ADR-038

---

*End of ADR-060.*
