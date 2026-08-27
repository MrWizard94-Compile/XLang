# Threat model — SBP-001 seed-native source bundle

**Status:** ADR-128 implementation and release verification complete<br>
**Date:** 2026-08-21<br>
**Decision:** [ADR-128](../historical%20docs/ADR-128-barp-seed-native-whole-library-bundle-profile.md)<br>
**Design:** [SBP-001](../historical%20docs/DESIGN-SBP-001-SEED-NATIVE-WHOLE-BUNDLE-PROFILE.md)<br>
**Rule IDs:** `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-DEP-001`

## Scope

SBP-001 adds the bounded `aether.seed-bundle/v1` source protocol and the named
seed ABI `compile_bundle [borrow bundle: Text] -> Bytes`. It carries exactly
one library and one entry unit in caller-selected Text. The verified seed
parses the frame, checks the narrow source profile, mangle-rewrites the one
import edge, and invokes its pre-existing `compile` weave. This remains a fixed
profile rather than the general graph protocol. GSM-001 now separately owns
bounded catalog-framed M11/M22 resolution for ordinary projects, workspaces,
and `aether.multi-source/v1`; see
[the GSM threat model](THREAT_MODEL-GSM-001-SEED-MODULE-CATALOG.md).

The host reads only an explicit local bundle path. On the production bundle
route it transports opaque Text to the verified seed and verifies returned
AETH before write or run. It neither decodes the bundle nor invokes the host
module elaborator.

## Assets and trust boundaries

| Asset / boundary | Security property |
| --- | --- |
| Caller-selected bundle Text | Untrusted, bounded, exact scalar framing; no implicit file discovery. |
| Seed `compile_bundle` weave | Verified AETH and exact borrowed-Text/Bytes ABI before invocation. |
| Source identity paths | Syntax-only names; they never authorize guest file access. |
| Generated single-world source | Constructed only after full profile validation and emitted by the established seed compiler. |
| Returned AETH Bytes | Independently verifier-checked before CLI write or VM execution. |
| General M11/M22 path | Kept separate so the profile cannot silently inherit host resolver authority. |

## Threats and controls

| Threat | Control | Evidence |
| --- | --- | --- |
| Oversized transport or hostile declared scalar count causes unbounded seed slicing | Host framing helpers cap authoring input; the seed rejects wire input over 33,280 scalars and each declared/source unit over 16,384 before payload extraction. | `SBP-006`, seed self-host hostile-frame tests. |
| Header/payload delimiter confusion or Unicode length mismatch changes unit boundaries | ASCII/LF-only source payloads, scalar-count framing, required LF separators, and exact two-unit/order checks. | `SBP-006`, framing round-trip test. |
| Path traversal or source identity spoofing reaches the filesystem | Strict lowercase `.ae` path grammar rejects traversal, separators outside `/`, and unsafe segments; the seed never opens a bundle path. | `SBP-006`, `bundle_safe_path`, forge ABI boundary. |
| Host silently resolves imports or rewrites source | `compile_product_seed_bundle` passes opaque Text directly to seed `compile_bundle`; no decode or M11 elaborator call is reachable on that route. | `SBP-002`, route trackers and integration test. |
| A malformed / private / unknown import call binds a different weave | One exported library helper, one exact import, safe alias, exact qualified helper target, and deterministic M11-compatible mangling. | `SBP-003`, `SBP-004`, `SBP-006`. |
| Text or effect/resource/nursery syntax confuses lexical call rewriting or imports authority | The profile rejects Text literals after the import and rejects the bounded resource/effect/nursery keyword set before assembly. | `SBP-006` negative corpus. |
| A malicious compiler artifact returns arbitrary Bytes or exploits an ABI mismatch | Forge verifies the compiler first, requires one borrowed Text parameter and Bytes result, then verifies output AETH before write. | `SBP-001`, `SBP-008`, forge ABI tests. |
| Profile success is overstated as GSM-001 general modules | Public contract documents SBP-001 as an exact two-unit profile and names GSM-001 as the separate bounded general catalog protocol. | `SBP-002`, ADR-128, current architecture/Seed Profile. |
| Guest compiler acquires filesystem, shell, network, cache, callback, or grant authority | `compile_bundle` receives only borrowed Text; all I/O remains forge-owned after verification and no grants are installed for the compiler weave. | `SBP-010`, Forge Contract. |

## Residual risk and non-goals

- The profile is deliberately not a general import graph, package resolver, or
  multi-file diagnostic engine.
- `AE-SEED-016` is a coarse, stable profile-rejection packet; it does not claim
  bootstrap-quality location or semantic diagnostics for every rejected input.
- The exact lexical exclusions prevent rewrite ambiguity in this profile; they
  are not a replacement for the complete Aether parser outside it.
- Input remains an explicit local file selected by the operator. This model does
  not authenticate that file or provide signing/provenance for source bundles.
- No guest filesystem, process, shell, network, model, native, registry, or
  callback capability is added.

## Reverification

```powershell
cargo test -p aether-core --test seed_self_host barp_adr128_seed_native_whole_library_bundle_matches_bootstrap_and_rejects_out_of_profile -- --exact
cargo test -p aether-core modules::tests::seed_bundle_framing_is_scalar_exact_bounded_and_source_opaque -- --exact
cargo run -p aether-cli -- compile .\examples\seed-bundle-whole.aeb --output .\target\seed-bundle-whole.aeth
cargo run -p aether-cli -- forge-bundle .\seed\aether_seed.aeth .\examples\seed-bundle-whole.aeb --output .\target\seed-bundle-forged.aeth
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

---

*End of SBP-001 threat model.*
