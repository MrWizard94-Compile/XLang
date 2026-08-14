# Delivery Report — M24i / M35j Operator Hardening

**Date:** 2026-08-11

**Status:** Complete local source increment; no release, commit, tag, push, or
publication performed

**One-sentence summary:** The M24i CA store now fails closed on meaningful
certificate and chain conditions with a complete local CLI key-to-store flow,
while M35j exposes a closed target flag that safely distinguishes host dual-run
from cross link-only output.

## Scope

### In scope

- Validate real Gregorian certificate dates, non-reversed certificate and trust
  key windows, certificate store size/subject uniqueness, active trust metadata,
  chain cycles, and root-only self-signed anchors.
- Make registry cache verification validate the complete persisted X.509-lite
  store before accepting package-cache contents.
- Add CLI root/certified Ed25519 setup from explicit local 32-byte seed files,
  plus X.509-lite store/verify commands with closed argument handling.
- Add `compile --native-exe --target <triple>` to the M35j closed matrix;
  host targets keep M35h dual-run and cross targets remain link-only.
- Prove a request to dual-run a cross target is rejected before output exists.
- Synchronize ADR-099/100, M24/M35 validation matrices, claims, manifest,
  README, and project-progress records.

### Out of scope

- RFC 5280/DER, public-key infrastructure, network/package auto-fetch, or a
  certificate revocation protocol.
- A bundled cross-compilation sysroot, guaranteed cross-link success, or
  execution of a cross binary on the host.
- Guest-language syntax, AETH format, seed semantics, VM behavior, a version
  bump, or a public technical-preview release.

## Delivery manifest

| Path | Delivered behavior |
| --- | --- |
| `crates/xlang-core/src/registry.rs` | Certificate/date/store/chain hardening; whole-store validation in the cache gate. |
| `crates/xlang-core/src/native.rs` | Host-target predicate and fail-before-write cross dual-run denial. |
| `apps/xlang-cli/src/main.rs` | Native `--target`, CA-store commands, and explicit Ed25519 root/certified-key setup. |
| `docs/ADR-099-m35j-native-cross-compile-target-matrix.md` | CLI and cross-execution contract. |
| `docs/ADR-100-m24i-registry-x509-lite-ca-store.md` | Store validity, operator flow, and error-boundary contract. |
| `docs/M24A-VALIDATION-MATRIX.md`, `docs/M35A-VALIDATION-MATRIX.md` | New negative and CLI coverage rows. |
| `MANIFEST.md`, `README.md`, `docs/CORE_CLAIMS.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Current bounded claims and operator semantics. |

## Verification evidence

| Command | Result |
| --- | --- |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` | Pass: Constitution pack, format, warnings-denied Clippy, 216 Rust tests, 32 seed≡bootstrap examples, project checks, and bootstrap≡product≡forged seed SHA-256 identity (`3B85696292FD5287F8778E708D095131D55094B04AB402209F8E77767B28E1A4`). |
| Focused registry/native/CLI tests | Pass: certificate windows, untrusted self-signed root rejection, cache-gate store validation, cross dual-run no-output rejection, target parsing, CA key request parsing, and root→intermediate→leaf operator helper flow. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass. |
| `cargo fmt --all -- --check` | Pass. |
| `git diff --check` | Pass after final documentation cleanup. |

## Rule ID self-audit

| Rule ID / gate | Status | Evidence |
| --- | --- | --- |
| `CONST-GATE-001` | Pass | Full gate completed after implementation and documentation synchronization. |
| `CONST-DONE-001` | Pass | Code, CLI surfaces, tests, ADRs, matrices, claims, and handoff are complete for this scoped increment. |
| `CONST-COMPLETE-001` | Pass | CA-store operations now include the prerequisite Ed25519 root/certified-key CLI path; no stub remains. |
| `CONST-DEP-001` | Pass | Validation primitives precede store persistence; CLI wraps existing verified core operations. |
| `ENG-WARN-001` | Pass | Rustfmt and warnings-denied workspace Clippy pass. |
| `TEST-BEHAVIOR-001` | Pass | Negative validity, anchor, cache, parser, and cross-output behaviors are asserted, plus the full product suite. |
| `DOC-SYNC-001` | Pass | Contract, matrices, claims, progress record, manifest, README, and this delivery report align. |
| `SEC-INPUT-001` | Pass | Closed flags, bounded store, local-only explicit seed paths, no logged key material, strict dates, root-anchor checks, and fail-before-write cross denial. |
| `REV-PACK-001` | Pass | This report states scope, files, proof, limits, and a suggested commit. |
| `OPS-DEL-001` | Pass | No dependency, generated artifact, temporary fixture, or partial implementation is part of the source delivery. |

## Modules loaded

- Constitution, SOP, Definition of Done, Engineering, Testing, Documentation,
  Security, low-level/risk review, review packaging, and Delivery Operations.

## Risks and deliberate boundaries

1. X.509-lite is a local signed JSON/PEM-shaped contract, not RFC 5280 DER;
   it has no revocation list, path constraints, public CA interoperability, or
   network trust discovery.
2. The new Ed25519 commands intentionally require caller-supplied local
   32-byte seed files. They never print seed bytes, but operators remain
   responsible for securing those files and the local cache root.
3. Cross link remains contingent on an installed host compiler and target
   sysroot. A successful link is not a remote execution claim.

## Suggested commit message

```text
feat: harden M24i CA store and wire M35j target operator flow

Validate bounded certificate chains in the cache gate, expose local Ed25519
CA setup, and make cross native output explicitly link-only.
```

## Next actions for the human

1. Review the local Ed25519 seed-file operational model before any package or
   registry distribution policy is considered.
2. Choose whether M35k needs a separately authorized reproducible bundled
   toolchain/sysroot design; do not infer it from M35j.
3. Keep RFC 5280/DER and broader CA behavior behind a new F-REGISTRY ADR.

*Delivery handoff under `CONST-GATE-001`, `CONST-DONE-001`,
`SEC-INPUT-001`, `OPS-DEL-001`, and `REV-PACK-001`.*
