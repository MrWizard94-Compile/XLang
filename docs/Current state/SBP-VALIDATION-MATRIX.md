# SBP-001 validation matrix

**Status:** Release verified — ADR-128 acceptance matrix<br>
**Date:** 2026-08-21<br>
**Design:** [SBP-001](../historical%20docs/DESIGN-SBP-001-SEED-NATIVE-WHOLE-BUNDLE-PROFILE.md)

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

## Release gate

`pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release` must pass after
the implementation, seed regeneration, documentation synchronization, and
pack verification.

**2026-08-21 evidence:** PASS — the release gate completed pack integrity,
current documentation links, formatter, warning-denied Clippy, full workspace
and seed-self-host suites, 32-example seed/bootstrap identity, four-way seed
identity, release packaging, consumer verification, and the unlisted-file
tamper rejection probe.

---

*End of SBP-001 validation matrix.*
