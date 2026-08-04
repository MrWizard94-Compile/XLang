# ADR-018: capability-mediated host I/O (M14)

**Status:** Accepted — **implemented in package 0.19.0**  
**Date:** 2026-08-04  
**Decision makers:** AGENTS Constitution; ADR-014 T-HOST after M13; threat model v2  
**Related Rule IDs:** `SEC-INPUT-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`, `CONST-COMPLETE-001`  

## Context

M8 pure host fixtures cannot read or write files. Mainstream maturity (E3)
requires I/O. Ambient guest I/O would violate project invariants. Technical
preview threat model forbade guest I/O until a capable-host model exists.

## Decision

1. **Adopt** [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md)
   as the binding threat freeze for I/O-bearing host services.  
2. **Adopt** [DESIGN-M14-HOST-IO-CAPABILITIES.md](DESIGN-M14-HOST-IO-CAPABILITIES.md).  
3. Extend the product host catalog with grant-backed `read_text` / `read_bytes` /
   `write_text` / `write_bytes` / `env_get` as specified.  
4. Install I/O services **only** when operator passes `--grant-*` on `run`
   (or test host API). Default run remains pure-fixture-only (M8 compatible).  
5. Guest paths are relative and jail under grant roots after canonicalize.  
6. **Defer** network, shell, FFI, directory listing, opaque OS handles.  

## Consequences

### Positive

- Real CLI demos without ambient authority  
- Backward compatible pure `run`  
- Testable deny-by-default  

### Costs

- Host session complexity  
- Operator must understand grants  
- Symlink/TOCTOU residual risk accepted for pilot  

### Risks

| Risk | Mitigation |
| --- | --- |
| Path escape | Grammar + canonicalize-under-root tests |
| Secret exfil via broad grant roots | Docs; human grant discipline |
| Scope creep to shell | Explicit non-goals |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Ambient guest FS | Rejected (law) |
| Wait for full WASI | Deferred; M14 is smaller pilot |
| Capability tokens as language values | Deferred (larger language change) |

## Implementation gate

1. This ADR Accepted  
2. Threat model v2 present  
3. M14 matrix present  
4. Vertical slice: grant-read + `read_text` + negative escape  

## Links

- Design: [DESIGN-M14-HOST-IO-CAPABILITIES.md](DESIGN-M14-HOST-IO-CAPABILITIES.md)  
- Threat: [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md)  
- Matrix: [M14-VALIDATION-MATRIX.md](M14-VALIDATION-MATRIX.md)  
- Prior: [ADR-011](ADR-011-m8-host-abi-pilot.md), [ADR-014](ADR-014-post-m10-track-portfolio.md)  

---

*End of ADR-018.*
