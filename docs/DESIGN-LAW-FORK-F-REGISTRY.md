# Law fork F-REGISTRY — optional offline-first package registry

**Status:** Design decision package — **authorized** 2026-08-10; product pilot ADR-060  
**Date:** 2026-08-05  
**Decision records:** [ADR-038](ADR-038-f-registry-law-fork.md), [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md)  
**Threat draft:** [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-CONTRACT-001`

---

## 1. Purpose

Decide whether Aether’s package story may grow beyond **path-local offline
workspaces** (M18/M22) to a **signed, offline-first registry** with optional
network fetch under operator control.

This document is **not** an implement go. Product code requires:

1. Human residual-risk acceptance via [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md)  
2. ADR-038 status flip to implementable  
3. Vertical-slice design (M24-class) with matrix and fail-closed tests  

## 2. Default law today (if fork is rejected)

| Rule | Default |
| --- | --- |
| Packages | Local directories + `aether.workspace/v1` + `depends_on` |
| Imports | Path-jail + depends_on authorization (M22) |
| Network | **No** product network client in CLI |
| Distribution | Human copies trees / git; local stdlib only |

Rejecting F-REGISTRY preserves strongest local-first / no-cloud-authority claims.

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

## 6. First implementable slice (only after authorize)

**Not in this package.** Candidate later vertical (M24-class):

1. Local cache layout + `aether registry verify-cache` (no network)  
2. Optional `aether registry fetch` with pinned URL + signature verify  
3. Workspace resolve from cache roots only  
4. Negatives: bad signature, digest mismatch, path escape, fetch disabled  

## 7. Stop conditions

- Implementing registry network without HUMAN-AUTHORIZE-REGISTRY phrase  
- Guest AETH network  
- Silent auto-update of packages during `compile`  

---

*End of DESIGN-LAW-FORK-F-REGISTRY.md*
