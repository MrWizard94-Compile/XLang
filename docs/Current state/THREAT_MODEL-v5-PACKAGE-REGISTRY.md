# Threat Model v5 — optional package registry (F-REGISTRY)

**Status:** **Binding** for F-REGISTRY product slices (authorized 2026-08-10)  
**Date:** 2026-08-05 (binding 2026-08-10)  
**Depends on:** [DESIGN-LAW-FORK-F-REGISTRY.md](DESIGN-LAW-FORK-F-REGISTRY.md), [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md)  
**Related Rule IDs:** `SEC-INPUT-001`, `CONST-DEP-001`

## 1. Purpose

Define trust boundaries if the **host CLI** may fetch/publish signed packages.
Guest AETH remains without network.

## 2. Assets (added)

| Asset | Why |
| --- | --- |
| Offline package cache | Integrity of dependency graph |
| Signing keys / trust roots | Package authenticity |
| Registry transport | MITM / availability |

## 3. Actors

| Actor | Trust |
| --- | --- |
| Operator | Enables fetch/publish; holds trust roots |
| Host registry client | Trusted only as far as operator + signatures allow |
| Remote registry | **Untrusted** until signature + digest verify |
| Guest AETH | **No network** (unchanged) |

## 4. Controls (required if implemented)

1. Network only on explicit registry subcommands.  
2. Signature verify + content digest fail closed.  
3. `compile` / `run` / `project build` never require network if cache complete.  
4. Path-jail for cache and extracted trees.  
5. No ambient guest sockets.  

## 5. Explicit non-goals

crates.io-scale ecosystem claims, unsigned defaults, auto-update on compile.

---

*End of THREAT_MODEL-v5-PACKAGE-REGISTRY.md*
