# Law fork F-REGISTRY — optional offline-first package registry

**Status:** Design decision package — **authorized** 2026-08-10; bounded F-REGISTRY implemented through M24i / ADR-060–100
**Date:** 2026-08-11
**Decision records:** [ADR-038](../historical%20docs/ADR-038-f-registry-law-fork.md), [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md), [ADR-060](../historical%20docs/ADR-060-f-registry-authorized-m24a-offline-cache.md)–[ADR-100](../historical%20docs/ADR-100-m24i-registry-x509-lite-ca-store.md)
**Threat draft:** [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-CONTRACT-001`

---

## 1. Purpose

Decide whether Aether’s package story may grow beyond **path-local offline
workspaces** (M18/M22) to a **signed, offline-first registry** with optional
network fetch under operator control.

This document authorized the bounded M24a–i product work. Any **new** registry
scope beyond that authorized boundary still requires:

1. Human residual-risk acceptance via [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md)  
2. ADR-038 status flip to implementable  
3. Vertical-slice design (M24-class) with matrix and fail-closed tests  

## 2. Invariants retained by the accepted fork

| Rule | Default |
| --- | --- |
| Packages | Local directories + `aether.workspace/v1` + `depends_on`, with explicit local registry-cache operations |
| Imports | Path-jail + depends_on authorization (M22) |
| Network | No ambient guest network; host fetch is only an explicit signed-registry operation |
| Distribution | Offline cache pin/verify and signed operator-controlled fetch; no automatic resolver |

The accepted fork retains local-first authority: compilation and cache
verification remain offline when inputs are already present, and guest Aether
never receives network capability.

## 3. If fork is accepted — allowed product shape

### 3.1 Preferred shape (recommended)

```text
Operator ──► aether registry fetch/publish (explicit)
                │
                ▼
         local offline cache (signed artifacts)
                │
                ▼
         workspace resolve (no ambient guest network)
```

| Property | Requirement |
| --- | --- |
| Guest AETH | **Still no network** — registry is host CLI only |
| Trust | Signed packages (keys offline-verifiable); fail closed on bad sig |
| Default compile | Works **fully offline** from cache + path deps |
| Network | Only on explicit `registry` subcommands; never on bare `compile`/`run` |
| Lockfiles | Content digests (SHA-256) for reproducibility |
| Offline-first | Fetch is optional; verify/build never require network if cache complete |

### 3.2 Rejected shapes (even if fork is accepted)

- Ambient guest sockets from verified AETH  
- Auto-fetch on every `compile` without operator config  
- Unsigned “best effort” packages as product default  
- Model/cloud as package authority  
- Replacing path workspaces (M18) — registry **extends**, does not delete offline graphs |

## 4. Law rewrites required on accept

| Artifact | Change |
| --- | --- |
| `AGENTS.md` | CLI may use network **only** for explicit registry ops |
| Threat model | Adopt threat v5; TP model no longer forbids all product network |
| `CORE_CLAIMS` CLM-003/004 | Refine “no network” → “no ambient guest network; optional host registry” |
| `MANIFEST.md` | Document registry commands + offline cache |
| Portfolio | Unblock M24-class registry track |

## 5. Residual risks (human must accept)

| ID | Risk |
| --- | --- |
| G1 | Host CLI gains a network client (supply-chain / MITM if sigs fail open) |
| G2 | Compromised signing keys ship malicious Aether packages |
| G3 | Cache poisoning if digests not bound |
| G4 | Operator confusion: “offline-first” still true only if fetch is optional |
| G5 | Ecosystem pressure to weaken path-jail / depends_on |

## 6. First implementable slice (implemented historical boundary)

The original M24-class vertical is implemented and expanded through M24i:

1. Local cache pin/verify under `aether.registry-cache/v1` with digest-bound
   local artifacts.
2. Explicit signed fetch via `aether registry fetch-signed`; no bare compile or
   run command fetches packages.
3. Offline signing-key trust, key rotation/revocation, multi-root trust policy,
   and bounded X.509-lite CA-store verification.
4. Fail-closed negatives cover malformed cache data, signature/digest mismatch,
   path escape, invalid trust chains, and invalid validity windows.
5. Resolver/range selection, automatic workspace mutation, package assets, and
   general RFC 5280 support remain outside the implemented boundary.

## 7. Stop conditions

- Implementing registry scope beyond authorized M24a–i without a renewed named human authorization and scoped ADR
- Guest AETH network  
- Silent auto-update of packages during `compile`  

---

*End of DESIGN-LAW-FORK-F-REGISTRY.md*
