# Threat Model — Aether Technical Preview (0.12)

> **Historical 0.12 threat-model freeze.** The current 0.37 local-package
> boundary is [THREAT_MODEL-0.37-LOCAL-PACKAGES.md](../Current%20state/THREAT_MODEL-0.37-LOCAL-PACKAGES.md).
> This document remains the dated pure-surface baseline and must not be read as
> the current authority for grant I/O, foreign pilot, workspace locks, local
> package publication, or v12 task frames.

**Status:** Frozen for technical preview (P2 / TP-2)
**Date:** 2026-08-04
**Product pin:** package 0.12.0 / language surface 0.11 / AETH v11
**Related:** [ADR-011](ADR-011-m8-host-abi-pilot.md), [ADR-012](ADR-012-m9-project-tooling.md), [MANIFEST.md](../../MANIFEST.md), [AUDIT_REPORT-2026-08-04-COMPLETION-READINESS.md](AUDIT_REPORT-2026-08-04-COMPLETION-READINESS.md)
**Rule IDs:** `SEC-INPUT-001`, `REL-HARDEN-001`, `CONST-GATE-001`

## 1. Purpose

Define trust boundaries and non-goals for the **local technical preview** so
operators and agents do not expand host authority by accident. This document
does not claim a formal certification or 1.0 security audit.

## 2. Assets

| Asset | Why it matters |
| --- | --- |
| Host filesystem (caller-selected paths) | Only I/O surface the CLI is allowed to touch |
| Seed compiler artifact (`seed/aether_seed.aeth`) | Product compile authority; integrity of default compile |
| Verified AETH bytecode | Execution input; must verify before run/write |
| Project documents + locks | Offline integrity of multi-file source trees |
| Operator trust in claims | Overstated claims are a product integrity risk |

## 3. Actors and trust

| Actor | Trust | Authority |
| --- | --- | --- |
| **Human operator** | Sovereign | Chooses which local files to compile, verify, run, or write |
| **Host CLI / library** (`aether`) | Trusted by the operator who launched it | Read caller-selected inputs; write only explicit outputs; install pure host fixtures for `run` |
| **Guest AETH artifact** | **Untrusted** | No ambient file, process, network, shell, or model authority |
| **Project document** | Untrusted input | Must fail closed on path escape, bad schema, lock mismatch |
| **Structural edit JSON** | Untrusted input | Exact-base stale guard; seed-compile before write |
| **Network / package registry** | **Out of scope** | Not present in product |
| **AI / model service** | **Out of scope** | Not integrated into the toolchain |

## 4. Trust boundaries

```text
┌─────────────────────────────────────────────────────────────┐
│ Operator machine (offline-first)                            │
│  ┌──────────────────┐     explicit paths      ┌──────────┐ │
│  │ aether CLI host  │◄───────────────────────►│ FS files │ │
│  │ (trusted)        │                         └──────────┘ │
│  │  - parse/format  │                                      │
│  │  - seed forge    │     verified AETH only               │
│  │  - project verify│───────────────────────►┌──────────┐ │
│  │  - pure host fix.│                        │ VM guest │ │
│  └──────────────────┘                        │(untrusted│ │
│                                              │  guest)  │ │
│                                              └──────────┘ │
│  No network, no registry, no model API in this product    │
└─────────────────────────────────────────────────────────────┘
```

## 5. Entry points and controls

| Entry | Untrusted input | Controls |
| --- | --- | --- |
| `compile` / `check` / `structure` | Source text | Parser/validator; seed or bootstrap path; no network |
| `run` | AETH bytes | `verify_bytecode` before execute; pure host fixtures only |
| `forge` | Compiler AETH + source | Verify compiler artifact; write only explicit output |
| `apply-edit` | Source + edit JSON | Stale-base reject; reparse; seed-compile before write |
| `format` | Source text | Formatter only; explicit output or stdout |
| `project verify` | Project JSON + unit files | Schema, relative path confinement (no `..` escape), optional SHA-256 locks, seed-compile each unit |
| `version` | None | Print version string |

## 6. Deny-by-default product rules

1. Guest AETH cannot open files, spawn processes, open sockets, or invoke a shell.
2. Product pure host fixtures are only `whole_inc` and `text_extent` (deterministic, I/O-free). Missing host services **fail closed**.
3. CLI writes only to paths the operator named (`--output`, `--output-dir`).
4. Project units must be relative `.ae` paths under the project root; absolute paths, drive prefixes, and `..` escape fail closed.
5. Lock mismatch fails closed (no “best effort” compile of tampered units).
6. Unknown / unsupported AETH versions are rejected by the verifier.
7. `unsafe_code = "forbid"`; Clippy `all = "deny"` on the product crates.

## 7. Explicit non-goals (technical preview)

| Non-goal | Why |
| --- | --- |
| C / FFI / libloading | Ownership and hostile-input surface not designed for preview |
| Ambient guest I/O | Conflicts with capability confinement |
| Network package registry | Offline-first law; no threat model for remote trust |
| Full LSP / multi-tenant IDE host | Would create a second compiler authority risk |
| Multi-user shared host with untrusted operators | CLI assumes the process operator is trusted |
| “1.0 security certification” | Preview only; residual risk accepted by human |
| Native / LLVM backend | Project law forbids without explicit law/ADR change |

## 8. Dependency surface (runtime)

| Crate | Pin | Role | Network at runtime? |
| --- | --- | --- | --- |
| `serde` / `serde_json` | pinned exact in core | Project + authoring JSON | No |
| `sha2` | pinned exact in core | Project lock digests | No |
| `aether-core` | path dep | Language stack | No |

No TLS, HTTP, or OS package client is linked into the product CLI.

## 9. Operational assumptions

1. The operator does not run untrusted shell wrappers that rewrite CLI arguments into unexpected write paths.
2. The seed artifact and release binary are obtained through a channel the operator trusts (local build or checksummed `dist/`).
3. Technical preview is **not** multi-tenant SaaS.
4. Full seed self-host rebuild (`aether-gate -Mode full`) is release-blocking for packaging, not for every edit.

## 10. Residual risks (accepted for preview)

| Risk | Mitigation / accept |
| --- | --- |
| Operator can overwrite any path they pass to `--output` | Documented; intentional host authority |
| Resource exhaustion via huge source / deep nests | Bounded language limits; not a full DoS product claim |
| Seed forge is expensive | Gate `--quick` vs `--full`; full before TP package |
| Claim drift after features | DOC-SYNC + gate; TP-1 audit filed |
| Future host I/O expansion | **Requires new ADR + this threat model rewrite** |

## 11. Preview package expectations

Local folder delivery (P0 freeze):

- `aether.exe` built `--release`
- SHA-256SUMS over package contents
- Seed artifact + schemas + examples needed offline
- Consumer `verify-preview.ps1` green on the operator machine

No GitHub Release or public tag is required for this preview channel.

## 12. Change control

Any of the following **invalidate** this freeze and require a new ADR (and SOP design loop):

- I/O-bearing host weaves
- C headers / dynamic libraries
- Network dependency resolution
- Ambient guest capabilities
- Shipping as “1.0” without a new audit scorecard

---

*End of technical preview threat model.*
