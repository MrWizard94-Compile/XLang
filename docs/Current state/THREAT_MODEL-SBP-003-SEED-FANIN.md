# Threat model — SBP-003 seed-native bounded fan-in

**Status:** Implemented — release verified<br>
**Date:** 2026-08-26<br>
**Decision:** [ADR-130](../historical%20docs/ADR-130-barp-seed-native-fanin-bundle-profile.md)<br>
**Design:** [SBP-003](../historical%20docs/DESIGN-SBP-003-SEED-NATIVE-FANIN-PROFILE.md)<br>
**Rule IDs:** `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-DEP-001`

## Scope

SBP-003 adds only `aether.seed-bundle/v3`: two independent pure `Whole` leaf
libraries, one exact two-import merge library, and one main entry in fixed wire
order. The existing verified seed `compile_bundle` parses that closed shape and
compiles one generated single-world program. This does not expand or replace
SBP-001/SBP-002. It remains a fixed fan-in profile; GSM-001 separately provides
bounded catalog-framed general M11/M22 graph elaboration for ordinary projects,
workspaces, packages, and `aether.multi-source/v1` without widening this
profile's capability boundary.

The product compile path and `forge-bundle` read one explicit local file, pass
opaque Text to the verified seed, and verify returned AETH before write or run.
No path contained in the bundle is opened by guest code.

## Assets and trust boundaries

| Asset / boundary | Required security property |
| --- | --- |
| Caller-selected v3 bundle Text | Untrusted; exact scalar framing, fixed count/order, and bounded source/wire size. |
| Leaf, merge, and entry identities | Syntax-only safe paths and pairwise distinct worlds; never file grants. |
| `compile_bundle` artifact | Verified AETH and exact borrowed-Text/Bytes ABI before invocation. |
| Generated single-world source | Created only after all profile checks and three qualified-call rewrites pass. |
| Returned artifact Bytes | Independently AETH-verified before product return, CLI write, or VM execution. |
| General M11/M22 route | Deliberately separate GSM-001 catalog protocol with its own bounded seed-native authority and threat model. |

## Threats and controls

| Threat | Control | Evidence |
| --- | --- | --- |
| Hostile count/payload causes unbounded slicing or allocation | Seed rejects wire input over 66,560 scalars, each payload over 16,384, and all four over 65,536 before generated-source assembly. | SBP matrix framing and hostile-size tests. |
| Framing ambiguity or Unicode length mismatch shifts a unit boundary | ASCII/LF-only source, Unicode-scalar counts, one mandatory post-payload LF, and exactly four frames. | Scalar round-trip and malformed-frame tests. |
| Path traversal or bundle identity opens an unintended file | Lowercase `.ae` grammar rejects unsafe paths; seed treats paths as syntax-only identity and has no filesystem capability. | Framing tests, forge boundary, source inspection. |
| Host silently resolves, reorders, or rewrites the graph | Product dispatch sends the original Text only to `compile_bundle`; no decoder, M11 elaborator, or embedded-path file read is reachable. | Route tracker and ADR-130 integration proof. |
| Merge binds wrong/private helper or hides an alias collision | Fixed leaf order, exact two imports, distinct safe aliases, one export each, and exact two-target call rewriting reject any other target. | Wrong import/order, alias collision, and private-call negative corpus. |
| Text, resource, effect, or nursery source bypasses lexical rewriting | ASCII/LF constraint plus closed pure-Whole guard rejects Text quotes, resources, effects, nursery/task, record, shape, host, and foreign forms. | Hostile source-profile tests. |
| Malicious compiler output bypasses ABI/artifact verification | Forge verifies the compiler ABI before invocation and AETH before exposing output to the CLI or VM. | Forge ABI and verifier regression suites. |
| v3 is overstated as the GSM-001 general protocol | Current contracts name one exact four-unit profile and distinguish GSM-001's separate bounded catalog authority. | ADR-130, validation matrix, core tracker test. |
| Seed code gains ambient authority | `compile_bundle` receives only borrowed Text; no grant, callback, filesystem, shell, network, model, cache, registry, or native capability is installed. | Forge Contract and grant-free invocation tests. |

## Residual risk and non-goals

- `AE-SEED-016` is intentionally coarse and does not provide full parser-grade
  locations or general multi-file semantic diagnostics.
- The profile's lexical checks are a narrow safety boundary for this exact
  fan-in shape, not a substitute for the full parser outside it.
- The caller chooses a local bundle file. SBP-003 does not add source signing,
  provenance, package identity, remote acquisition, or automatic dependency
  discovery.
- Arbitrary graphs, packages, assets, capability grants, and guest I/O remain
  outside this profile and require their own authorization and proof.

## Reverification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr130_seed_native_four_unit_fanin_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_fanin_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-fanin.aeb --output .\target\seed-bundle-fanin.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-fanin.aeb --output .\target\seed-bundle-fanin-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of SBP-003 threat model.*
