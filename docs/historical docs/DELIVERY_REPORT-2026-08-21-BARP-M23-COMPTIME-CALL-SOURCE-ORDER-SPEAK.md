# Delivery Report — BARP M23 comptime-call source-order integrity

**Date:** 2026-08-21
**Status:** Release verified
**Scope:** ADR-127 seed-native product-path M23 source-order correction
**Rule IDs:** CONST-DEP-001, DOC-ADR-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Delivered behavior

The checked-in Aether seed now searches only the source prefix preceding a
canonical root M23 comptime call when resolving an ordinary helper header. A
forward target is no longer accepted by product compilation. Direct seed forge
produces one `aether.seed-error/v1` `AE-SEED-011` packet with
`origin: "seed-speak"`; product packet merge rejects the compile. Bootstrap
continues to reject the same input with its established `AE-COMPTIME-001`.

The existing direct unknown-target witness remains authoritative for a target
with no header anywhere. The resolver checks the existing error flag first, so
that source produces one established packet rather than a duplicate
source-order packet.

## Delivered artifacts

| Artifact | Delivered content |
| --- | --- |
| `seed/aether_seed.ae` | Prefix-bounded ordinary-header lookup plus deduplicated root/ordinary/total/non-erroring source-order guard |
| `seed/aether_seed.aeth` | Rebuilt compiler artifact matching bootstrap, product, and independent forge output |
| `crates/xlang-core/tests/seed_self_host.rs` | Direct, product, bootstrap, valid-source, identity, and boundary regression coverage |
| ADR-127 and current contracts | Exact authority scope, diagnostic distinction, and residual limits |

## Focused verification evidence

| Check | Result |
| --- | --- |
| Seed source bootstrap compile | PASS |
| Bootstrap/product/forge artifact identity | PASS — SHA-256 `F064D66DD9B8C53BC9D1757D0DE5D1217B1F0906F389A2D9CB497807C553DFDD`, 52,089 bytes |
| Promoted checked-in artifact identity | PASS — matches the same SHA-256 |
| ADR-127 focused self-host test | PASS — 1 passed, 0 failed |
| Unknown-target packet priority | PASS — one established `AE-SEED-011`, no duplicate source-order packet |
| Forward-target product integrity | PASS — direct/product `AE-SEED-011`; bootstrap `AE-COMPTIME-001` |
| Valid prior M23 helpers | PASS — one-argument and zero-argument cases verify, match bootstrap, and exit 42 |
| Scope boundaries | PASS — non-call, missing-target, nested, Text-literal, task, erroring-parent, and missing-world inputs stay outside the new packet branch |

## Security and authority boundary

The correction operates only on already-loaded source Text in the seed compiler.
It makes no host call and adds no filesystem, process, shell, network, model,
guest capability, package, registry, native, AETH, verifier, or VM authority.

It is deliberately not an M23 parser or type checker. Return type, parameter
mode, purity, D2a body, argument count, nested-call, control-flow, and
multi-file rules remain outside this source-order witness.

## Release verification

The complete release gate passed on 2026-08-21:

| Release check | Result |
|---|---|
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1 |
| Documentation links | PASS — 361 Markdown documents and 1,461 links |
| Formatting and static analysis | PASS — cargo fmt and strict workspace Clippy with warnings denied |
| Behavioral suite | PASS — complete workspace tests, including the 46-test seed self-host suite |
| Product/bootstrap corpus | PASS — all 32 top-level examples match byte-for-byte |
| Seed compiler identity | PASS — bootstrap, product, independent forge, and checked-in artifact match SHA-256 F064D66DD9B8C53BC9D1757D0DE5D1217B1F0906F389A2D9CB497807C553DFDD |
| Package and consumer | PASS — 446 package files plus SHA-256SUMS; independent consumer verified every listed file and rejected an unlisted tamper probe |

The repeat command below is the reproducible release check for this exact,
synchronized documentation state.

## Section 0 self-audit

| # | Check | Evidence |
|---|---|---|
| 1 | Completeness | Source, checked-in seed artifact, regression coverage, ADR, matrices, contract, and delivery record are included; no deferred implementation remains. |
| 2 | Dependency-first | The product seed, bootstrap oracle, forge artifact, verifier, and package paths were rebuilt and exercised together. |
| 3 | Zero warnings/errors | Release gate passed cargo fmt and strict workspace Clippy with warnings denied. |
| 4 | Intended-behavior tests | Focused forward/unknown/prior-helper/boundary coverage passed; the full workspace and seed self-host suites passed. |
| 5 | Documentation synchronized | Current contracts, BARP and M23 matrices, roadmap, progress report, ADR, MANIFEST, and this report describe the same bounded correction. |
| 6 | Security and validation | The correction reads only already-loaded source Text and adds no host, filesystem, process, shell, network, model, guest-capability, package, registry, native, AETH, verifier, or VM authority. |
| 7 | Performance reasoning | The lookup is bounded to the current source prefix and executes only for the already-recognized canonical M23 root shape. |
| 8 | Version and stack fidelity | The Rust 1.88 / Aether 0.37 seed-hosted product contract and bootstrap-oracle boundary remain unchanged. |
| 9 | Package ready | The release package contains 446 listed files plus SHA-256SUMS, and independent consumer verification passed. |
| 10 | Resource and constraint check | No dependency, service, host capability, or runtime resource budget changed; the gate remains fully offline. |
| 11 | Reproducibility and determinism | Four-way seed identity and all 32 product/bootstrap examples matched byte-for-byte. |
| 12 | IP and invention hygiene | N/A — no external IP, licensed asset, or new invention claim was introduced. |
| 13 | Multi-agent coordination | N/A — this delivery was completed by one implementation agent. |
| 14 | Review packaging | MANIFEST, ADR, validation matrices, current-state records, repeat verification command, and bounded residuals are synchronized. |
| 15 | Self-audit log | This table records the completed Section 0 review and its evidence. |

## Repeat verification

~~~powershell
cargo test -p aether-core --test seed_self_host barp_adr127_seed_speaks_canonical_root_comptime_bind_unknown_calls -- --exact
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
~~~

---

*End of ADR-127 delivery report.*
