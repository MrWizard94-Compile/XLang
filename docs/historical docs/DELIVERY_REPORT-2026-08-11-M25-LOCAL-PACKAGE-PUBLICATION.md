# Delivery Report — M25 Local Package Publication (Aether 0.37.0)

**Date:** 2026-08-11
**Package:** `aether-core` / `aether-cli` 0.37.0
**Scope:** M25 transparent local source-package publication
**Decision:** [ADR-107](ADR-107-m25-local-package-publication.md)
**Contract:** [AETHER_0.37.md](../Current%20state/AETHER_0.37.md)
**Security review:** [THREAT_MODEL-0.37-LOCAL-PACKAGES.md](../Current%20state/THREAT_MODEL-0.37-LOCAL-PACKAGES.md)
**Rules:** `CONST-GATE-001`, `CONST-COMPLETE-001`, `CONST-DEP-001`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`, `SEC-INPUT-001`, `REL-PACKAGE-001`, `REL-DETERM-001`, `OPS-DEL-001`

## Outcome

M25 is complete as a local-only package lifecycle. A complete locked Aether
project can be packed into a strict transparent `aether.package/v1` directory,
verified from raw bytes, published to an explicit collision-safe local cache,
and installed as a normal locked workspace project. The core evidence proves
two independent M22 consumers build and run against one installed package.

This is not a remote registry, package resolver, signature/provenance system,
archive format, automatic workspace editor, package-script runner, asset
channel, or guest capability. Those would each require a distinct authorized
design and threat model.

## Delivered surface

| Area | Delivered behavior |
| --- | --- |
| Core API | `pack_project`, bundle/cache verification, local publish, direct/cache install, strict `PackageDocument`, stable `AE-PACKAGE-001` through `005` diagnostics. |
| CLI | `aether pkg pack|verify|publish|install|verify-cache` with closed arguments and explicit destinations. |
| Integrity | Complete PKG-001 project lock, raw manifest/unit SHA-256 values, domain-separated content SHA-256, exact tree audit, nested project verification. |
| Filesystem safety | Path confinement, no symlinks/nonregular inputs, byte/file/cache bounds, staging plus post-copy verification, no overwrite of pack/install outputs, cache collision fail-closed. |
| Reuse | Direct/cache install produces ordinary locked project roots; two independent M22 consumers prove explicit reuse. |
| Distribution | Version-derived 0.37 local technical preview and standalone consumer verifier exercise the M25 lifecycle. |

## Evidence

| Verification | Result |
| --- | --- |
| `cargo test -p aether-core package::tests --lib` | PASS — 6 M25 tests: deterministic/reuse, locks, hostile metadata/input, symlink, cache collision, output preservation, and bounds. |
| `cargo test -p aether-cli package_install_arguments_are_closed_and_complete --bin aether` | PASS — direct/cache install parser closure. |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode quick` | PASS — M25 real CLI lifecycle plus standard quick gate. |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` | PASS — pack integrity, `fmt --check`, Clippy `-D warnings`, workspace suite, 32 seed/bootstrap examples, M25 lifecycle, and seed proof. |
| Seed identity | `86391C07D31069526D5FC33ADDC287FF1F79D4C35C663D388E3B6E2108F5ADCB`; bootstrap = product = forge = checked-in seed. |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release` | PASS — release build, staged 0.37 preview, exact checksum verifier, M25 standalone consumer path, and unlisted-file rejection. |

The release verifier runs the staged `aether.exe`, verifies `SHA-256SUMS` with
no unlisted files, checks v11/v12 behavior, authoring v8, project/workspace
integrity, and performs pack → verify → idempotent publish → cache verify →
direct/cache install → installed-project verification in a temporary directory.

## Security review summary

Untrusted local metadata, bundle trees, cache entries, and output requests are
validated before acceptance. Traversal, separator tricks, malformed JSON,
digest mismatch, stale/missing/incomplete lock, unlisted bundle file, symlink,
nonregular source, oversize file, malformed cache entry, and cache identity
collision are intended rejection paths with tests. The remaining OS-level
pathname race and local-author trust limits are explicitly documented in the
threat model; cryptographic publisher identity is not claimed.

## Section 0 self-audit (15 points)

1. **Completeness:** Core API, CLI, gate, preview verifier, fixture, docs, and
   release artifacts are wired; no stubs or TODOs remain.
2. **Dependency-first:** M25 uses complete PKG-001 locks and M18/M22 workspace
   boundaries; no new resolver is implied.
3. **Zero warnings/errors:** Full and release gates pass `cargo fmt --check` and
   Clippy workspace/all-target `-D warnings`.
4. **Tests:** Positive lifecycle, two-consumer reuse, parser closure, and hostile
   filesystem/metadata/cache boundaries pass.
5. **Docs:** Contract, design, ADR, matrix, threat model, README, manifest,
   architecture, roadmap, progress, claims, preview docs, and release notes agree.
6. **Security:** Explicit hostile-path/filesystem review and negative tests are
   included; no authority expansion is hidden.
7. **Performance:** Bounded file/count/cache limits; no hot-path or performance
   claim introduced.
8. **Stack fidelity:** Rust 2021 / 1.88 workspace conventions and existing CLI
   error/validation idioms are retained.
9. **Package readiness:** Preview stages deterministic source/docs/binary files,
   SHA-256SUMS, and a standalone consumer verifier.
10. **Constraints:** Local-only implementation, bounded input memory, no cloud or
    network need, and explicit operator paths preserve the product model.
11. **Determinism:** Repeated pack identity, idempotent equal publication, seed
    identity, and release checksum verification pass.
12. **IP/invention hygiene:** M25 invariants, boundaries, ADR, and claim register
    are explicit; no novelty or superiority claim is made.
13. **Multi-agent coordination:** N/A — this delivery used no subagents.
14. **Review packaging:** Manifest, contract, release notes, threat model,
    validation matrix, command evidence, residual risks, and next boundaries are present.
15. **Self-audit:** All applicable checks above passed; no unresolved delivery
    blocker remains.

## Follow-on boundary

The next mainstream increment should be separately scoped. Plausible package
follow-ons—publisher authentication, provenance, resolver/version ranges,
assets, remote distribution, or multi-writer cache transactions—are not implied
by this delivery. BARP seed diagnostics, seed-native multi-file elaboration,
F-NATIVE hermetic tooling, F-REGISTRY full DER, and task-runtime depth remain
independent roadmap work.

*End of DELIVERY_REPORT-2026-08-11-M25-LOCAL-PACKAGE-PUBLICATION.md*
