# Delivery Report — BARP seed-native Whole-library bundle profile

**Date:** 2026-08-21<br>
**Status:** Release verified<br>
**Scope:** ADR-128 / SBP-001 bounded `aether.seed-bundle/v1` seed-native source profile<br>
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

## Delivered behavior

The checked-in Aether seed now exposes a second closed forge weave:

```aether
weave compile_bundle [borrow bundle: Text] -> Bytes:
```

It accepts exactly one scalar-framed `aether.seed-bundle/v1` Text input with
one pure Whole library followed by one main entry. The seed—not host code—checks
the two-unit frame, safe source identities, distinct worlds, one export, one
exact import, qualified calls, resource/effect/nursery exclusions, M11-compatible
name mangling, and generated-source assembly. It then invokes its established
`compile` weave and returns only verified AETH to the product path.

The normal product `compile` detects this explicit envelope before raw-import
preflight and routes it directly to seed `compile_bundle`; the external
`aether forge-bundle` command performs the same named ABI invocation on one
caller-selected compiler artifact and bundle file. Neither route calls a host
bundle decoder, source parser, module elaborator, resolver, or rewriting step.

This is deliberately **not** a general seed-native M11/M22 claim. General
projects, workspaces, and `aether.multi-source/v1` remain host-elaborated then
seed-emitted; `seed_native_multi_module_elaboration() == false` remains true.

## Delivered artifacts

| Artifact | Delivered content |
| --- | --- |
| `seed/aether_seed.ae` | Seed-side bounded frame parser, profile validator, M11-compatible mangler/call rewriter, and `compile_bundle` entry. |
| `seed/aether_seed.aeth` | Rebuilt checked-in seed artifact, identical to bootstrap/product/forge rebuilds. |
| `crates/xlang-core/src/modules.rs` | Source-opaque framing utilities, safe authoring/decode boundaries, and scalar/wire caps. |
| `crates/xlang-core/src/lib.rs` | Closed product-route dispatch, explicit ABI forge helper, and honesty trackers. |
| `apps/xlang-cli/src/main.rs` | `aether forge-bundle <compiler> <bundle> --output <artifact>` command and verified-write test. |
| `examples/seed-bundle-whole.aeb` | Canonical two-unit arithmetic bundle fixture. |
| `crates/xlang-core/tests/seed_self_host.rs` | Seed/product/bootstrap identity, VM execution, hostile frame/profile, and authority-boundary regressions. |
| ADR-128 / SBP-001 / current contracts | Bounded scope, validation matrix, threat model, roadmap, claim register, architecture, and forge/seed contracts. |

## Focused verification evidence

| Check | Result |
| --- | --- |
| Seed bootstrap rebuild | PASS — `seed/aether_seed.ae` compiled cleanly to the checked-in artifact. |
| Seed product / bootstrap / independent forge identity | PASS — SHA-256 `0E328A6BAA0A838562649AC2119477D98CD4BFEE2E3B5B619D79E419147CB26C`. |
| Canonical bundle product route | PASS — verifies and exits **84**. |
| Canonical external `forge-bundle` route | PASS — emits the same verified 102-byte artifact, SHA-256 `E9B1F1D773D488F967E92909697EDA61BAE78A69A0E76FC37DE67D995475C6E0`. |
| Bootstrap M11 elaboration comparison | PASS — canonical seed-bundle artifact is byte-identical to the independent bootstrap reference. |
| Product-route authority | PASS — direct bundle route tracker confirms no host module elaborator. |
| Framing boundary | PASS — Unicode scalar round trip stays source-opaque; malformed count, order, unsafe path, and wire-over-limit cases fail closed. |
| Source-profile boundary | PASS — malformed, overdeclared, wrong import, duplicate world, unknown/private call, Text, and resource forms emit `AE-SEED-016` with `origin: seed-speak`. |
| Named ABI boundary | PASS — owned/missing/wrong `compile_bundle` signatures are rejected before invocation. |
| CLI write discipline | PASS — `forge-bundle` writes only its verifier-accepted Bytes result. |

## Security and authority boundary

The bundle is untrusted caller-selected local Text. Its profile is bounded to
two units, 16,384 scalars per source, 32,768 source scalars total, and 33,280
wire scalars. The seed never opens a path named in the bundle: paths are source
identity only. It receives one borrowed Text argument and no grant, callback,
filesystem, process, shell, network, model, package, registry, or native
authority.

Returned Bytes go through the existing artifact verifier before product return,
CLI write, or VM execution. The exact lexical exclusions are a narrow
rewrite-safety boundary for this profile, not a substitute for full Aether
diagnostics outside it. The remaining general module path keeps its established
host elaboration boundary and must not be described as seed-native.

## Release verification

The complete 2026-08-21 release gate passed:

| Release check | Result |
| --- | --- |
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1. |
| Documentation links | PASS — local-link fixtures and the current documentation tree. |
| Formatting and static analysis | PASS — cargo fmt and strict workspace Clippy with warnings denied. |
| Behavioral suite | PASS — complete workspace tests, including 47 seed self-host tests. |
| Product/bootstrap corpus | PASS — all 32 top-level examples match byte-for-byte. |
| Seed compiler identity | PASS — bootstrap, product, independent forge, and checked-in artifact share SHA-256 `0E328A6BAA0A838562649AC2119477D98CD4BFEE2E3B5B619D79E419147CB26C`. |
| Package and consumer | PASS — staged technical-preview package passed independent consumer verification and rejected an unlisted tamper probe. |

## Section 0 self-audit

| # | Check | Evidence |
| --- | --- | --- |
| 1 | Completeness | The profile, named ABI, CLI route, seed artifact, fixture, tests, ADR, design, matrix, threat model, contracts, and delivery record are present; no deferred implementation remains. |
| 2 | Dependency-first | Frame limits, path validation, ABI enforcement, product dispatch, seed implementation, fixture, and behavioral proof were delivered as one closed dependency chain. |
| 3 | Zero warnings/errors | Release gate passed cargo fmt and warning-denied workspace Clippy. |
| 4 | Intended-behavior tests | Product/forge execution, byte identity, malformed frame, unsafe path/order, wrong import/world/call, forbidden source, and ABI negatives passed. |
| 5 | Documentation synchronized | MANIFEST, README, Aether 0.37, Seed Profile, Forge Contract, architecture, BARP, roadmap, claims, progress, ADR, design, matrix, threat model, and this report state the same bounded claim. |
| 6 | Security and input validation | Untrusted framed Text is bounded and validated in seed; no host elaboration, file open, or guest capability is exposed. |
| 7 | Performance reasoning | Fixed two-unit, scalar, and wire caps bound parsing/scanning/temporary Text work; no unmeasured throughput claim is made. |
| 8 | Version and stack fidelity | Rust 1.88, Aether 0.37 package contract, existing AETH v11/v12 behavior, and bootstrap-oracle roles remain unchanged. |
| 9 | Package ready | Release package and independent consumer verification passed, including tamper rejection. |
| 10 | Resource and constraint check | The profile rejects resource/effect/nursery forms and adds no dependency, service, runtime resource class, or host authority. |
| 11 | Reproducibility and determinism | Four-way seed identity and canonical bundle/bootstrap identity passed byte-for-byte. |
| 12 | IP and invention hygiene | The bounded source-bundle profile is an internal engineering artifact; no external code, protected asset, third-party license, patent assertion, or legal ownership claim was introduced. |
| 13 | Multi-agent coordination | N/A — this delivery was completed by one implementation agent. |
| 14 | Review and packaging | Source, documentation, gate, package, consumer, and tamper-probe evidence were re-reviewed against the explicit authority boundary. |
| 15 | Self-audit log | This table records the completed Section 0 review and its evidence. |

## Repeat verification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr128_seed_native_whole_library_bundle_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-whole.aeb --output .\target\seed-bundle-whole.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-whole.aeb --output .\target\seed-bundle-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of ADR-128 delivery report.*
