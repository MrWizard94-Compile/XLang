# Delivery report — ADR-130 seed-native bounded fan-in profile

**Date:** 2026-08-26<br>
**Status:** Release verified<br>
**Scope:** ADR-130 / SBP-003 bounded \`aether.seed-bundle/v3\` profile<br>
**Rule IDs:** \`CONST-DEP-001\`, \`COST-CACHE-001\`, \`DOC-ADR-001\`,
\`ENG-WARN-001\`, \`RND-INVAR-001\`, \`SEC-INPUT-001\`, \`TEST-BEHAVIOR-001\`

## Delivered behavior

ADR-130 extends the existing capability-closed \`compile_bundle [borrow bundle:
Text] -> Bytes\` ABI with a separately versioned \`aether.seed-bundle/v3\`
envelope. It accepts exactly four scalar-framed, ASCII/LF source payloads in
fixed dependency order:

\`\`\`text
left pure Whole leaf + right pure Whole leaf -> merge -> entry
\`\`\`

The verified seed, not the product host, validates the frame and the narrow
source profile. It validates safe unique paths, four distinct worlds, each leaf
export, the merge's two imports/export, the entry import/main, and all three
fixed import edges. It rewrites both qualified merge calls through the
established M11-compatible path mangling, then invokes the existing seed
\`compile\` weave.

The product route and explicit \`aether forge-bundle\` command send opaque Text
directly to the seed. They do not call a host bundle decoder, parser, resolver,
or M11 elaborator. The profile carries no guest capability and does not alter
v1, v2, ordinary raw-source import rejection, general M11/M22, source syntax,
AETH, VM, project schema, package behavior, or grants.

## Delivered files

| Area | Delivery |
| --- | --- |
| Seed | \`bundle_rewrite_two_calls\`, \`bundle_fanin_elaborate\`, and v3 dispatch in \`seed/aether_seed.ae\`, followed by a rebuilt checked-in seed artifact. |
| Core route | v3 schema, fixed frame bounds, source-opaque authoring/test framing utilities, product dispatch, and an explicit truth tracker. |
| Canonical corpus | \`examples/seed-bundle-fanin.aeb\`, a four-unit left leaf + right leaf -> merge -> entry fixture that exits 84. |
| Behavioral proof | Seed/product/bootstrap identity, verifier/VM run, hostile frame/source-profile cases, and no-host-elaborator evidence in \`seed_self_host\`. |
| Release tooling | \`aether-gate\` product/named-forge checks for v1, v2, and v3; isolated preview consumer checks for the shipped v3 fixture. |
| Documentation | ADR-130, SBP-003, threat model, validation matrix, current product contracts, roadmap, claim register, progress report, and this delivery report. |

## Focused verification evidence

| Check | Result |
| --- | --- |
| Bootstrap seed rebuild | PASS — the changed \`seed/aether_seed.ae\` compiles cleanly through \`--bootstrap\`. |
| Canonical v3 product route | PASS — \`examples/seed-bundle-fanin.aeb\` verifies and exits **84**. |
| Canonical external named forge | PASS — emits the same verified AETH artifact as product compilation. |
| Product/forge artifact identity | PASS — SHA-256 \`31CBB227BA7733E99DFFE0A3D9B6B8B523B0972E8A31F14A74F50DA0D47C0756\`. |
| Independent M11/bootstrap reference | PASS — the focused seed self-host test compares exact artifact bytes. |
| Host authority boundary | PASS — the general multi-module tracker remains false and the product bundle route tracker reports no host elaborator. |
| Source/profile boundary | PASS — malformed/overdeclared/trailing frames, wrong order/import/alias, duplicate world, private call, and resource forms fail through \`AE-SEED-016\` / \`seed-speak\`. |
| Formatting | PASS — \`cargo fmt --all -- --check\` exits successfully. |

## Security and authority boundary

The v3 bundle is untrusted caller-selected local Text. Its fixed bounds are
four units, 16,384 scalars per source, 65,536 source scalars total, and 66,560
wire scalars. Its paths are source identity only; the seed never opens them.
\`compile_bundle\` receives one borrowed Text argument, no grant, callback,
filesystem, process, shell, network, model, native, cache, package, or registry
authority.

Returned Bytes remain subject to the normal verifier before product return, CLI
write, or VM execution. The fixed profile's lexical exclusions make the two
call rewrites reviewable; they are not a substitute for the established full
compiler outside SBP-003. General modules stay on the host-elaborate + seed-emit
route.

## Release verification

The 2026-08-26 release gate passed after documentation integrity was restored.

| Release check | Result |
| --- | --- |
| Constitution pack integrity | PASS — AGENTS Constitution 5.1.0 (GOV-INT-001). |
| Documentation links | PASS — fixtures plus 374 Markdown files and 1,559 local links. |
| Formatting and static analysis | PASS — cargo fmt --check and warning-denied workspace Clippy. |
| Behavioral suite | PASS — full workspace suite, including 49 seed self-host tests. |
| Product/bootstrap corpus | PASS — all 32 top-level examples match byte-for-byte. |
| Seed compiler identity | PASS — bootstrap, product, independent forge, and checked-in seed share SHA-256 B1E32615B60B9593FF92D10779B3B427F77522961E1BB476C4DEF0BB3D0AEAAE. |
| Public v1/v2/v3 bundle transport | PASS — release gate compiles/runs each profile and compares its product output with named forge-bundle output. |
| Package and consumer | PASS — 462-file staged local technical-preview package passed independent consumer verification, including the v3 fan-in fixture. |
| Package tamper probe | PASS — the release gate rejected an added unlisted package file. |

## Section 0 self-audit

| # | Check | Evidence |
| --- | --- | --- |
| 1 | Completeness | The seed profile, checked-in artifact, core route, fixture, hostile corpus, release/consumer gates, ADR, design, matrix, threat model, contracts, and delivery report form one closed delivery. |
| 2 | Dependency-first | Frame bounds and path rules precede product dispatch; seed implementation precedes artifact rebuild; fixtures/tests/gates precede release claims. |
| 3 | Zero warnings/errors | The final release gate passed the formatter and warning-denied workspace Clippy. |
| 4 | Intended-behavior tests | Product/forge execution, M11/bootstrap identity, frame limits, trailing bytes, graph order, imports, aliases, worlds, calls, resources, v1/v2 regressions, and CLI transport passed. |
| 5 | Documentation synchronized | README, MANIFEST, Aether 0.37, Seed Profile, Forge Contract, architecture, BARP, roadmap, claims, progress, ADR, design, matrix, threat model, package readme, and this report state the same bounded claim. |
| 6 | Security and input validation | Untrusted Text is fixed-count/scalar-bounded and seed-validated; no host elaboration, guest file open, or capability is exposed. No secret is added. |
| 7 | Performance reasoning | Four-unit, per-unit, aggregate, and wire caps bound scans and temporary Text work; no throughput claim is made. |
| 8 | Version and stack fidelity | Rust 1.88, Aether 0.37, existing AETH v11/v12, and bootstrap-oracle roles remain unchanged. |
| 9 | Package ready | A checksummed 462-file preview package and independent consumer verification passed; the unlisted-file probe was rejected. |
| 10 | Resource and constraint check | The profile rejects resource/effect/nursery forms and adds no dependency, service, runtime resource class, or host authority. A task-local, non-sensitive cache held only profile limits, hashes, and verification plan and was revalidated before the release gate. |
| 11 | Reproducibility and determinism | Four-way seed identity and canonical v3 product/forge/bootstrap identity passed byte-for-byte. |
| 12 | IP and invention hygiene | The bounded fan-in profile is original internal engineering work; no external code, protected asset, third-party license, or legal ownership claim was introduced. |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this delivery. |
| 14 | Review and packaging | Source, docs, package membership, consumer, tamper evidence, and explicit authority boundary were reviewed; MANIFEST and delivery evidence are linked. |
| 15 | Self-audit log | This table records the completed Section 0 review and traceable evidence. |

## Repeat verification

\`\`\`powershell
cargo test -p aether-core --test seed_self_host barp_adr130_seed_native_four_unit_fanin_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_fanin_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-fanin.aeb --output .\target\seed-bundle-fanin.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-fanin.aeb --output .\target\seed-bundle-fanin-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
\`\`\`

---

*End of ADR-130 delivery report.*
