# SBP validation matrix

**Status:** ADR-128 and ADR-129 release verified<br>
**Date:** 2026-08-21<br>
**Designs:** [SBP-001](../historical%20docs/DESIGN-SBP-001-SEED-NATIVE-WHOLE-BUNDLE-PROFILE.md), [SBP-002](../historical%20docs/DESIGN-SBP-002-SEED-NATIVE-TRANSITIVE-CHAIN-PROFILE.md)

| ID | Intended behavior / invariant | Evidence |
| --- | --- | --- |
| SBP-001 | The checked-in seed exposes `compile_bundle [borrow bundle: Text] -> Bytes`. | Named forge ABI test accepts the seed and rejects missing/wrong signatures. |
| SBP-002 | Product bundle compilation invokes the verified seed entry directly, not a host parser/elaborator. | Product-route tracker plus direct product test; implementation inspection shows no decode/elaborate call. |
| SBP-003 | A canonical library + entry bundle emits byte-identical AETH to bootstrap compilation of the established M11 host elaboration reference. | Seed self-host integration test compares bytes and verifier accepts both. |
| SBP-004 | Repeated canonical qualified calls are all rewritten and the artifact executes the intended Whole result. | Seed bundle test runs the verified artifact and asserts exit code. |
| SBP-005 | Ordinary raw `import unit` source remains `AE-SEED-012`; v1 JSON envelope remains host-elaborated. | Existing product diagnostics and v1 envelope regressions remain green. |
| SBP-006 | Scalar framing, separator, unit count/order, duplicate/unsafe path, wrong import, duplicate world, forbidden Text/resource/effect/nursery forms, and non-export calls fail closed. | Hostile bundle tests observe one `AE-SEED-016` / `seed-speak` packet; framing helpers reject hostile authoring input without source parsing. |
| SBP-007 | Encoder/decoder frame Unicode scalars deterministically and do not perform linguistic elaboration. | Framing unit tests plus code-level API boundary test. |
| SBP-008 | No bundle result is written or run before normal AETH verifier acceptance. | Forge and CLI artifact-write tests; verifier regression suite. |
| SBP-009 | Seed source, bootstrap rebuild, seed forge, and checked-in artifact remain byte-identical. | Seed self-host suite and release gate four-way identity. |
| SBP-010 | No new host capability is available to guest seed code. | Forge ABI inspection, grant-free invocation tests, and threat-model/documentation review. |
| SBP-011 | The existing named `compile_bundle [borrow bundle: Text] -> Bytes` ABI accepts the separately versioned v2 protocol without weakening v1. | v1 regression corpus, v2 direct product/named-forge test, and ABI tests. |
| SBP-012 | v2 is exactly three scalar-framed units with safe unique paths, a final named entry, ASCII/LF payloads, and 16,384/49,152/49,920 scalar caps. | Source-opaque framing unit test and hostile seed frame cases. |
| SBP-013 | A canonical foundation -> bridge -> entry bundle seed-elaborates both import edges, verifies, exits 84, and exactly matches independent M11/bootstrap output. | ADR-129 seed self-host integration test plus canonical fixture. |
| SBP-014 | Wrong dependency order/import, duplicate world, private/unknown bridge call, overdeclared frame, and forbidden resource source all fail closed with one `AE-SEED-016` seed-SPEAK packet. | ADR-129 hostile-profile corpus. |
| SBP-015 | v2 does not promote general seed-native modules and is exercised by product, named-forge, release, and isolated consumer paths. | General false tracker, route tracker, `aether-gate`, and preview-verifier coverage. |

## Release gate

`pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release` must pass after
the implementation, seed regeneration, documentation synchronization, and
pack verification.

**ADR-128 evidence (2026-08-21):** PASS — the release gate completed pack
integrity, current documentation links, formatter, warning-denied Clippy, full
workspace and seed-self-host suites, 32-example seed/bootstrap identity,
four-way seed identity, release packaging, consumer verification, and the
unlisted-file tamper rejection probe.

**ADR-129 evidence (2026-08-21):** PASS — the release gate completed pack
integrity, current documentation links, formatter, warning-denied Clippy, the
full workspace and 48-test seed-self-host suites, 32-example seed/bootstrap
identity, four-way seed identity, v1/v2 product-to-named-forge proof, release
packaging, independent consumer verification, and the unlisted-file tamper
rejection probe.

---

*End of SBP validation matrix.*
