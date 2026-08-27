# Delivery report — GSM-001 general seed module catalog

**Date:** 2026-08-27<br>
**Status:** Full and release gates verified<br>
**Scope:** ADR-131 / GSM-001 bounded general seed-native M11/M22 module graphs<br>
**Rule IDs:** `CONST-DEP-001`, `COST-CACHE-001`, `DOC-ADR-001`,
`ENG-WARN-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

## Delivered behavior

GSM-001 replaces the former host-source-graph elaboration on normal bounded
project, workspace, multi-source, and explicit-catalog product routes. The host
performs only caller-selected manifest, lock, path-confinement, UTF-8, and
direct-package-authority framing. It supplies the resulting closed
`aether.seed-modules/v1` catalog as opaque Text to the checked-in Aether-written
seed ABI:

```aether
weave compile_modules [borrow catalog: Text] -> Bytes:
```

The verified seed validates scalar framing, parses M11/M22 imports, resolves
the reachable acyclic graph, enforces local/direct-package authority and
library roles, validates exports and aliases, detects cycles and duplicate
worlds, deterministically rewrites qualified calls, and emits AETH through its
existing `compile` weave. Returned Bytes remain independently verified before
any CLI write or VM run.

This is a general bounded graph protocol, not a general resolver. It does not
let the seed discover files, open a filesystem path, resolve a registry or
network dependency, infer transitive package authority, add guest capability,
or claim full bootstrap diagnostic parity. Raw unframed single-file
`import unit` source still fails closed; callers use a project, workspace,
multi-source envelope, or explicit GSM catalog to opt into seed elaboration.

## Delivered files

| Area | Delivery |
| --- | --- |
| Seed | `modules_*` catalog framing, parser, graph, validator, deterministic rewrite/elaboration helpers, and `compile_modules` in `seed/aether_seed.ae`; rebuilt checked-in `seed/aether_seed.aeth`. |
| Core | `SeedModuleCatalog` / `SeedModuleUnit`, deterministic host framing, safe project/workspace source selection, seed product route, named forge ABI, explicit route trackers, and behavior tests. |
| CLI | `aether forge-modules <compiler-artifact> <catalog-file> --output <artifact-file>` with verified-input/verified-output enforcement. |
| Canonical corpus | `examples/seed-modules-general.aem`: six candidate units, local fan-in, direct M22 `math` library, and unreachable library; it exits 85. |
| Quality tooling | GSM product-versus-named-forge hash proof in `tools/aether-gate.ps1` and isolated-preview proof in `tools/verify-preview.ps1`. |
| Documentation | ADR-131, design, validation matrix, threat model, contracts, architecture, claims, roadmap, progress report, package readme, and this report. |

## Bounds and authority controls

| Control | Enforced limit or rule |
| --- | --- |
| Candidate graph | 1–256 explicitly carried source units; arbitrary acyclic topology within the catalog |
| Package authority | 0–64 manifest-authorized direct packages; foreign catalog units must be `lib` |
| Per-unit source | 1–16,384 Unicode scalars |
| Aggregate source | At most 196,608 Unicode scalars |
| Complete wire catalog | At most 250,000 Unicode scalars |
| Runtime transport safety | The wire cap is no greater than one quarter of Aether's 1,000,000-byte Text invocation limit, so worst-case four-byte Unicode remains invocable. |
| Identity / traversal | Confined local or `package::confined` `.ae` identity up to 256 scalars; deterministic depth-first traversal with a 66,000-step guard |

The boundary test deliberately uses maximum four-byte Unicode scalars. It proves
that the Rust framing and Aether seed both accept the documented inclusive
per-unit/aggregate boundary, reject one scalar beyond the per-unit limit, and
never create a catalog Text value larger than the existing runtime invocation
limit. The runtime guard was preserved; GSM capacity was reduced to fit it.

## Focused verification evidence

| Check | Result |
| --- | --- |
| Inclusive Unicode-boundary proof | PASS — `seed_catalog_validator_accepts_inclusive_unit_and_aggregate_scalar_limits` validates the exact maximum and one-scalar rejection through the checked-in seed source. |
| Module suite | PASS — `cargo test -p aether-core modules::tests --lib`: 16 tests, including opaque framing, local irregular graph, M22 direct dependency, cycle/role negatives, product path, and byte identity. |
| Core named ABI | PASS — malformed/missing `compile_modules` signature is rejected before invocation. |
| CLI named forge | PASS — `forge-modules` writes only independently verified returned AETH. |
| Seed self-host suite | PASS — 49 tests, including whole-language corpus, product/forge identity, and historical fixed-profile regressions. |
| Canonical GSM product route | PASS — product compile and VM run of `examples/seed-modules-general.aem` exit **85**. |
| Canonical named forge | PASS — `aether forge-modules seed/aether_seed.aeth examples/seed-modules-general.aem` produces the same verified artifact. |
| Product/forge identity | PASS — SHA-256 `63E89C0AAB275FCE6591C68B08B320D2EDEDD4B2FAF0E44B27CB2F3F10B179DC`. |
| Aether Atlas | PASS — deterministic replay SHA-256 `02b482d20056abb4986f541eff047ab7df0c5743a0af2e0b1025c086454dc645`; policy audit exit 0; replay exit 4,720,242; accepted and rejected receipts, v12 fault exit 91, tamper rejection, and M25 bundle/cache/install proof all succeeded. |

## Full and release verification

The complete gate was rerun after the implementation and documentation changes.

| Gate | Result |
| --- | --- |
| Constitution integrity | PASS — AGENTS Constitution 5.1.0, `GOV-INT-001`. |
| Documentation integrity | PASS — documentation verifier fixtures and the repository-local link graph. |
| Formatting and static analysis | PASS — `cargo fmt --check`; workspace Clippy with `-D warnings`. |
| Behavior | PASS — `cargo test --workspace`, including 42 CLI tests, 158 core tests, and 49 seed self-host proofs. |
| Seed identity | PASS — full gate bootstrap/product/independent-forge/checked-in seed identity. |
| Public GSM transport | PASS — product and external named forge hash-match the six-unit local-plus-M22 fixture and run it to exit 85. |
| Release packaging | PASS — release CLI build, checksummed local technical-preview package, independent consumer verification, and unlisted-file tamper rejection. |

## Section 0 self-audit

| # | Check | Evidence |
| --- | --- | --- |
| 1 | Completeness | Seed compiler, artifact, host framing, project/workspace route, named forge, fixture, tests, gates, ADR, design, matrix, threat model, contracts, and dated record are delivered together. |
| 2 | Dependency-first | Closed catalog framing and input bounds precede source selection; seed graph implementation precedes artifact rebuild; tests and gate coverage precede release claims. |
| 3 | Zero warnings/errors | The full and release gates passed formatter and warning-denied workspace Clippy. |
| 4 | Intended-behavior tests | Tests exercise valid/invalid framing, Unicode bounds, local/M22 resolution, graph order, cycles, roles, aliases, exports, byte identity, verification, and CLI output safety. |
| 5 | Documentation synchronized | All current and historical documents named above state the same bounded host-frame/seed-elaborate authority split. |
| 6 | Security and input validation | Caller Text is size-, scalar-, identity-, role-, and package-bounded; the seed gains no filesystem, resolver, network, shell, callback, or guest capability. |
| 7 | Performance reasoning | Unit, aggregate, wire, identity, and traversal limits bound catalog scans and temporary Text work; the Unicode-safe wire cap preserves the pre-existing runtime memory guard. |
| 8 | Version and stack fidelity | Rust 1.88, Aether 0.37, AETH v11/v12, and the bootstrap recovery/oracle role remain unchanged. |
| 9 | Package ready | A release package and independent consumer verification passed; an unlisted-file tamper probe was rejected. |
| 10 | Resource and constraint check | No new runtime resource class, effect, service, dependency, or host authority was added. A non-sensitive task-local cache was used only for revalidated repository facts and verification planning. |
| 11 | Reproducibility and determinism | Seed identity and the canonical product/named-forge artifact hash both passed; Atlas replay matched deterministically. |
| 12 | IP and invention hygiene | This is original repository engineering; no external source code, protected asset, new dependency, or licensing claim was introduced. |
| 13 | Multi-agent coordination | N/A — one implementation agent performed this delivery. |
| 14 | Review and packaging | Product authority, input bounds, seed source, artifact rebuild, documentation, package membership, consumer proof, and tamper defense were reviewed. |
| 15 | Self-audit log | This table records the completed Section 0 review and traceable evidence. |

## Repeat verification

```powershell
cargo test -p aether-core modules::tests --lib
cargo test -p aether-core --test seed_self_host
cargo run -p aether-cli -- compile .\examples\seed-modules-general.aem --output .\target\seed-modules-general.aeth
cargo run -p aether-cli -- forge-modules .\seed\aether_seed.aeth .\examples\seed-modules-general.aem --output .\target\seed-modules-general-forged.aeth
pwsh -NoProfile -File .\showcases\aether-atlas\tools\verify.ps1
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of GSM-001 delivery report.*
