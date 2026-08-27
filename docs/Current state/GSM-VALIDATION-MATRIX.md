# GSM-001 Validation Matrix — General Seed Module Catalog

**Status:** Implemented; full gate passed 2026-08-27  
**Date:** 2026-08-27  
**Decision:** [ADR-131](../historical%20docs/ADR-131-gsm-general-seed-module-catalog.md)  
**Design:** [GSM-001 design](DESIGN-GSM-001-GENERAL-SEED-MODULE-CATALOG.md)  
**Threat model:** [GSM-001 threat model](THREAT_MODEL-GSM-001-SEED-MODULE-CATALOG.md)

## Scope statement

This matrix validates the default bounded seed-native M11/M22 catalog route.
It does not replace the historical SBP-001/002/003 fixed-profile matrix or claim
full diagnostic parity. A test can use the retained Rust elaborator only as an
independent oracle; it must not make that elaborator part of the product route.

| ID | Intended behavior | Evidence / test | Expected result |
| --- | --- | --- | --- |
| GSM-VAL-001 | Catalog metadata uses deterministic scalar framing with bounded identities, roles, package list, and payload counts. | "encode_seed_module_catalog", "decode_seed_module_catalog", and validation unit tests in "modules.rs". | Valid records round-trip exactly; malformed count/schema/source-boundary records fail closed. |
| GSM-VAL-001a | Published scalar limits are inclusive and every valid Unicode catalog fits Aether's Text invocation boundary. | "seed_catalog_validator_accepts_inclusive_unit_and_aggregate_scalar_limits" uses maximum four-byte Unicode scalars. | The host and seed both accept the documented unit/aggregate maxima, reject one scalar beyond the unit maximum, and the complete catalog remains at or below 1,000,000 UTF-8 bytes. |
| GSM-VAL-002 | Host catalog framing never parses Aether payload source. | "project_catalog_framing_reads_manifest_units_without_parsing_source". | Manifest/path/UTF-8 framing accepts opaque text payloads; source syntax is deferred to seed. |
| GSM-VAL-003 | Product recognizes GSM input before raw-source import preflight. | Direct catalog compile test and "aether compile examples/seed-modules-general.aem". | Catalog reaches "compile_modules"; raw source semantics are not misclassified as an ambient import. |
| GSM-VAL-004 | The checked-in seed validates a closed general local graph. | "bootstrap_seed_module_catalog_validator_accepts_closed_local_catalog". | A valid catalog is accepted by the seed path. |
| GSM-VAL-005 | The seed owns dependency-first graph order instead of fixed wire order. | "bootstrap_seed_module_graph_orders_closed_local_catalog". | Graph order is deterministic and follows reachable dependencies. |
| GSM-VAL-006 | Import scanning is bounded and normal-form local imports are recognized. | "bootstrap_seed_module_import_counter_is_bounded_for_local_import". | Valid import counts are returned; malformed input fails without unbounded scanning. |
| GSM-VAL-007 | An irregular acyclic local graph is product-compiled by seed and matches the independent oracle. | "seed_catalog_elaborates_an_irregular_transitive_graph_byte_identically". | Non-topological six-unit input, fan-in, and unreachable unit produce verified AETH, VM exit 42, and byte identity against Rust elaboration plus bootstrap emission. |
| GSM-VAL-008 | Project framing takes the default local M11 route through opaque catalog text. | Existing "compile_project_modules" project tests plus GSM product route tests. | Host reads selected local units only; seed resolves imports and emits AETH; no bootstrap production invocation. |
| GSM-VAL-009 | Direct M22 dependencies are seed-resolved and byte-identical to the independent oracle. | "project_seed_catalog_elaborates_direct_m22_dependency_byte_identically". | Consumer plus direct "math" package exits 42 and matches Rust elaboration/bootstrap bytes. |
| GSM-VAL-010 | A foreign package absent from explicit authority is denied before execution. | "project_seed_catalog_elaborates_direct_m22_dependency_byte_identically" negative case. | Product failure has AE-PROJECT-004 wrapping the coarse AE-SEED-017 packet; no artifact runs. |
| GSM-VAL-011 | Cycles and invalid lib/main role edges fail closed through the product route. | "rejects_import_cycle_and_lib_main". | Product returns AE-PROJECT-004 with AE-SEED-017; no partial artifact. |
| GSM-VAL-012 | "forge-modules" requires exactly the named borrowed-Text/Bytes compiler ABI. | Core test "forge_modules_rejects_a_compiler_weave_without_the_required_borrowed_text_abi". | Owned Text, wrong result, and missing named weave are rejected. |
| GSM-VAL-013 | CLI transport writes only verified output for a valid compiler result. | CLI test "forge_modules_invokes_compile_modules_and_writes_only_a_verified_artifact". | Artifact bytes equal fixture result and pass verifier. |
| GSM-VAL-014 | Public product compile and external named forge agree for a non-profile catalog. | "tools/aether-gate.ps1" GSM-001 section using "examples/seed-modules-general.aem". | Product and "forge-modules" SHA-256 values match; VM exits 85. |
| GSM-VAL-015 | Product workspace graph handling survives a real six-package systems workload. | "showcases/aether-atlas/tools/verify.ps1". | Deterministic replay, package/lock evidence, policy audit, resource/task behavior, fault rejection, and M25 lifecycle all pass. |
| GSM-VAL-016 | Seed source and checked-in compiler remain reproducible. | Full gate seed rebuild/forge identity plus seed self-host suite. | Bootstrap rebuilt, product rebuilt, forged, and checked-in seed artifact are byte-identical where required. |
| GSM-VAL-017 | No quality gate regresses. | "aether-gate.ps1 -Mode full", Constitution pack verify, rustfmt, Clippy warnings denied, workspace tests, documentation links. | All commands exit 0 with no warnings/errors. |

## Negative-boundary checklist

The implementation must retain explicit tests or code-level fail-closed checks for:

- empty/unknown schema and trailing payload;
- zero, oversized, or mismatched scalar counts;
- wire text exceeding 250,000 Unicode scalars or Aether's 1,000,000-byte
  invocation-Text limit, including worst-case four-byte Unicode scalars;
- unsafe absolute, traversal, repeated-separator, and malformed package keys;
- duplicate package identities and duplicate unit keys;
- foreign entry or foreign non-lib role;
- missing entry unit and entry with "lib" role;
- local/M22 imports whose target is absent, non-lib, unapproved, cyclic, private,
  unknown, malformed, or alias-colliding;
- duplicate worlds;
- graph traversal exhaustion;
- missing "compile_modules" or an ABI other than one borrowed "catalog: Text"
  and "Bytes" result; and
- failed return-artifact verification before CLI write or VM run.

## Evidence interpretation

"General" means arbitrary acyclic graph topology within the protocol bounds and
the documented M11/M22 normal-form module surface. It does not mean unbounded
discovery, unrestricted package authorities, full bootstrap diagnostic parity,
or a new guest capability. The Rust oracle and bootstrap remain evidence tools,
not inputs to successful normal product compilation.

---

*End of GSM-VALIDATION-MATRIX.*
