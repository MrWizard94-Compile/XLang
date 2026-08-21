# Delivery report — ADR-129 seed-native transitive library chain

**Date:** 2026-08-21<br>
**Status:** Release verified<br>
**Scope:** ADR-129 / SBP-002 bounded `aether.seed-bundle/v2` transitive profile<br>
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

## Delivered behavior

ADR-129 extends the existing capability-closed `compile_bundle [borrow bundle:
Text] -> Bytes` ABI with a separately versioned `aether.seed-bundle/v2`
envelope. It accepts exactly three scalar-framed, ASCII/LF source payloads in
fixed dependency order:

```text
foundation library -> bridge library -> main entry
```

The verified seed, not the product host, validates the frame and narrow source
profile. It validates safe paths, distinct worlds, one foundation export, one
bridge import/export, one entry import/main, and the two exact qualified call
boundaries. It applies the established M11-compatible path mangle/rewrite
inside Aether source, then invokes the existing seed `compile` weave.

The product route and explicit `aether forge-bundle` command send opaque Text
directly to the seed. They do not call a host bundle decoder, parser, resolver,
or M11 elaborator. The profile carries no guest capability and does not alter
v1, ordinary raw-source import rejection, general M11/M22, source syntax,
AETH, VM, project schema, package behavior, or grants.

## Delivered files

| Area | Delivery |
| --- | --- |
| Seed | `bundle_chain_elaborate` and v2 dispatch in `seed/aether_seed.ae`, followed by a rebuilt checked-in seed artifact. |
| Core route | v2 schema, fixed frame bounds, source-opaque authoring/test framing utilities, product dispatch, and an explicit truth tracker. |
| Canonical corpus | `examples/seed-bundle-chain.aeb`, a three-unit foundation -> bridge -> entry fixture that exits 84. |
| Behavioral proof | Seed/product/bootstrap identity, verifier/VM run, hostile frame/source-profile cases, and no-host-elaborator evidence in `seed_self_host`. |
| Release tooling | `aether-gate` product/named-forge checks for both v1 and v2; isolated preview consumer checks for the shipped v2 fixture. |
| Documentation | ADR-129, SBP-002, threat model, validation matrix, current product contracts, roadmap, claim register, and this delivery report. |

## Focused verification evidence

| Check | Result |
| --- | --- |
| Bootstrap seed rebuild | PASS — the changed `seed/aether_seed.ae` compiles cleanly through `--bootstrap`. |
| Canonical v2 product route | PASS — `examples/seed-bundle-chain.aeb` verifies and exits **84**. |
| Canonical external named forge | PASS — emits the same 175-byte verified AETH artifact as product compilation. |
| Product/forge artifact identity | PASS — SHA-256 `73DEE92072402E98F5C1662F8B88BBFAABC89A58C6678FCF41A9CFA4AAD4550F`. |
| Independent M11/bootstrap reference | PASS — focused seed self-host test compares exact artifact bytes. |
| Host authority boundary | PASS — the general multi-module tracker remains false and the product bundle route tracker reports no host elaborator. |
| Source/profile boundary | PASS — malformed frame, overdeclared and oversized wire size, trailing payload, wrong import/order, duplicate world, private call, and resource form fail through `AE-SEED-016` / `seed-speak`. |

## Security and authority boundary

The v2 bundle is untrusted caller-selected local Text. Its fixed bounds are
three units, 16,384 scalars per source, 49,152 source scalars total, and 49,920
wire scalars. Its paths are source identity only; the seed never opens them.
`compile_bundle` receives one borrowed Text argument, no grant, callback,
filesystem, process, shell, network, model, native, cache, package, or registry
authority.

Returned Bytes remain subject to the normal verifier before product return, CLI
write, or VM execution. The fixed profile's lexical exclusions make two call
rewrites reviewable; they are not a substitute for the established full compiler
outside SBP-002. General modules stay on the host-elaborate + seed-emit route.

## Release verification

The 2026-08-21 release gate passed:

| Release check | Result |
| --- | --- |
| Constitution pack integrity | PASS — AGENTS Constitution 5.0.1 (`GOV-INT-001`). |
| Documentation links | PASS — fixtures plus 370 Markdown files and 1,531 local links. |
| Formatting and static analysis | PASS — `cargo fmt --check` and warning-denied workspace Clippy. |
| Behavioral suite | PASS — full workspace suite, including 48 seed self-host tests. |
| Product/bootstrap corpus | PASS — all 32 top-level examples match byte-for-byte. |
| Seed compiler identity | PASS — bootstrap, product, independent forge, and checked-in seed share SHA-256 `552128E5F8AFB4F1584B4BBC5A83D5AEDBA241AE4CCEB814B3C48CD0F419C2B0`. |
| Public v1/v2 bundle transport | PASS — release gate compiles/runs both profiles and compares product output with the named `forge-bundle` output. |
| Package and consumer | PASS — 457-file staged local technical-preview package passed independent consumer verification, including the v2 chain. |
| Package tamper probe | PASS — the release gate rejected an added unlisted package file. |

## Section 0 self-audit

| # | Check | Evidence |
| --- | --- | --- |
| 1 | Completeness | The seed profile, checked-in artifact, core route, fixture, hostile cases, release/consumer gates, ADR, design, matrix, threat model, contracts, and delivery report are one closed delivery. |
| 2 | Dependency-first | Frame bounds and path rules precede product dispatch; seed implementation precedes artifact rebuild; fixtures/tests/gates precede published claims. |
| 3 | Zero warnings/errors | Final release gate passed formatter and warning-denied workspace Clippy. |
| 4 | Intended-behavior tests | Product/forge execution, M11/bootstrap identity, frame limits, trailing bytes, order, imports, worlds, calls, resources, v1 regressions, and CLI transport passed. |
| 5 | Documentation synchronized | README, MANIFEST, Aether 0.37, Seed Profile, Forge Contract, architecture, BARP, roadmap, claims, progress, ADR, design, matrix, threat model, package readme, and this report state the same bounded claim. |
| 6 | Security and input validation | Untrusted Text is fixed-count/scalar-bounded and seed-validated; no host elaboration, guest file open, or capability is exposed. |
| 7 | Performance reasoning | Three-unit, per-unit, aggregate, and wire caps bound scans and temporary Text work; no throughput claim is made. |
| 8 | Version and stack fidelity | Rust 1.88, Aether 0.37, existing AETH v11/v12, and bootstrap-oracle roles remain unchanged. |
| 9 | Package ready | Release package SHA-256SUMS and independent consumer verification passed, including v2 and tamper rejection. |
| 10 | Resource and constraint check | The profile rejects resource/effect/nursery forms and adds no dependency, service, runtime resource class, or host authority. |
| 11 | Reproducibility and determinism | Four-way seed identity and canonical v2 product/forge/bootstrap identity passed byte-for-byte. |
| 12 | IP and invention hygiene | The bounded chain profile is original internal engineering work; no external code, protected asset, third-party license, or legal ownership claim was introduced. |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this delivery. |
| 14 | Review and packaging | Source, docs, package membership, consumer, and tamper evidence were reviewed against the explicit authority boundary. |
| 15 | Self-audit log | This table records the completed Section 0 review and traceable evidence. |

## Repeat verification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr129_seed_native_three_unit_chain_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_chain_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-chain.aeb --output .\target\seed-bundle-chain.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-chain.aeb --output .\target\seed-bundle-chain-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of ADR-129 delivery report.*
