# Threat model — SBP-002 seed-native transitive chain

**Status:** ADR-129 implementation and release verification complete<br>
**Date:** 2026-08-21<br>
**Decision:** [ADR-129](../historical%20docs/ADR-129-barp-seed-native-transitive-library-chain-profile.md)<br>
**Design:** [SBP-002](../historical%20docs/DESIGN-SBP-002-SEED-NATIVE-TRANSITIVE-CHAIN-PROFILE.md)<br>
**Rule IDs:** `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-DEP-001`

## Scope

SBP-002 adds only `aether.seed-bundle/v2`: one scalar-framed foundation
library, bridge library, and main entry in exact dependency order. The existing
verified seed `compile_bundle [borrow bundle: Text] -> Bytes` parses that
specific shape and compiles a generated one-world program. This document does
not expand or replace SBP-001's v1 threat model. It does not claim general
seed-native M11/M22: the public general tracker remains false and ordinary
projects, workspaces, raw imports, and `aether.multi-source/v1` remain on their
established host-elaborated route.

The product compile path and `forge-bundle` read one explicit local file, pass
opaque Text to the verified seed, and verify returned AETH before write or run.
No path within the bundle is opened by guest code.

## Assets and trust boundaries

| Asset / boundary | Required security property |
| --- | --- |
| Caller-selected v2 bundle Text | Untrusted; exact scalar framing, bounded source/wire size, no implicit discovery. |
| Foundation, bridge, entry identities | Syntax-only safe paths and pairwise distinct worlds; never file grants. |
| `compile_bundle` artifact | Verified AETH and exact borrowed-Text/Bytes ABI before invocation. |
| Generated single-world source | Created only after all fixed-profile checks and call rewrites pass. |
| Returned artifact Bytes | Independently AETH-verified before product return, CLI write, or VM execution. |
| General M11/M22 route | Deliberately separate, preserving its existing host-elaboration boundary. |

## Threats and controls

| Threat | Control | Evidence |
| --- | --- | --- |
| A hostile count or payload forces unbounded slicing/allocation | Seed rejects a wire input over 49,920 scalars, each payload over 16,384, and all three over 49,152 before generated-source assembly. | SBP-012, hostile frame tests. |
| Framing ambiguity or Unicode length mismatch moves a unit boundary | ASCII/LF source only, Unicode-scalar counts, exact post-payload LF, and exactly three frames. | SBP-012, scalar round-trip unit test. |
| A path causes traversal or guest file access | Lowercase `.ae` identity grammar rejects unsafe forms; the seed does not open paths or install a filesystem capability. | SBP-012, forge boundary. |
| A host silently resolves or rewrites the chain | Product dispatch sends original Text only to `compile_bundle`; no decoder or M11 elaborator is called on this route. | SBP-002, route tracker and integration proof. |
| A bridge targets a private, wrong, or transposed dependency | Exact wire order, exact import path/alias parsing, one exported helper per library, and qualified-call rewriting reject any other target. | SBP-013, SBP-014. |
| Text, resource, effect, or nursery syntax confuses lexical rewriting or imports authority | The narrow source profile rejects its unsafe lexical forms before assembly; unsupported input fails instead of receiving generalized parser treatment. | SBP-014, source-profile negatives. |
| A malicious compiler output bypasses verification or abuses a mismatched ABI | Forge verifies compiler ABI before invocation and verifies returned AETH before exposing Bytes to the CLI. | SBP-001, SBP-008, named-forge tests. |
| v2 is represented as general seed-native modules | Current contracts retain the false general tracker and name exactly one three-unit chain profile. | SBP-015, ADR-129, current contracts. |
| Seed code gains ambient authority | `compile_bundle` receives only borrowed Text; no grant, callback, filesystem, shell, network, model, cache, registry, or native capability is installed. | SBP-010, Forge Contract. |

## Residual risk and non-goals

- `AE-SEED-016` is intentionally coarse and does not provide full parser-grade
  locations or multi-file semantic diagnostics.
- The source-profile checker is a closed safety boundary for this one chain; it
  does not establish full lexical/parser equivalence outside that shape.
- The caller chooses a local bundle file. SBP-002 does not add source signing,
  provenance, package identity, or remote acquisition.
- General graph resolution, packages, M22 imports, assets, and capability
  grants remain outside this profile and need separate authorization and proof.

## Reverification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr129_seed_native_three_unit_chain_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_chain_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-chain.aeb --output .\target\seed-bundle-chain.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-chain.aeb --output .\target\seed-bundle-chain-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of SBP-002 threat model.*
