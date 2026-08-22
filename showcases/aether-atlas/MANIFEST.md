# MANIFEST — Aether Atlas Systems Showcase

**Delivery title:** Aether Atlas bounded journal-recovery systems showcase
**Date:** 2026-08-22
**Author:** Codex with human-directed scope
**Related Rule IDs:** CONST-GATE-001, CONST-DONE-001, CONST-COMPLETE-001,
CONST-DEP-001, ENG-WARN-001, TEST-BEHAVIOR-001, DOC-SYNC-001,
SEC-INPUT-001, REV-PACK-001

## Summary

A complete, reproducible Aether-first recovery system that demonstrates the
current language/toolchain surface through a real bounded journal protocol,
resource projection, task failure behavior, explicit grants, and offline
package evidence.

## Files

| Path | Purpose | New / Modified |
| --- | --- | --- |
| ../../README.md | Links the repository landing page to the executable systems showcase. | Modified |
| aether.workspace.json | Locked six-package Atlas workspace graph. | New |
| README.md | System contract, feature matrix, expected outcomes, and reproduction entry point. | New |
| THREAT_MODEL.md | Boundary, input validation, residual-risk, and operational guidance. | New |
| audit/aether.project.json | Locked cross-package policy audit project. | New |
| audit/main.ae | Runtime assertion of cross-package policy acceptance and rejection. | New |
| protocol/aether.project.json | Locked protocol project and its parser tests. | New |
| protocol/main.ae | Minimal package entry required by the project contract. | New |
| protocol/fingerprint.ae | Bounded Bytes fingerprint and Whole mixing helpers. | New |
| protocol/journal.ae | Strict fixed-frame Text parser, decimal folding, and deterministic recovery score. | New |
| protocol/fingerprint_test.ae | Behavioral test for byte fingerprint semantics. | New |
| protocol/journal_test.ae | Acceptance and malformed-frame rejection behavioral test. | New |
| policy/aether.project.json | Locked policy project. | New |
| policy/main.ae | Minimal package entry required by the project contract. | New |
| policy/policy.ae | Cross-package recovery-policy transformation and binary-frame classifier. | New |
| replay/aether.project.json | Locked replay project. | New |
| replay/main.ae | Pure recovery projection using records, M2, M6, M7, M5/M23, and M19a. | New |
| replay/resource_test.ae | Resource/table/nursery behavior test entry. | New |
| operator/aether.project.json | Locked explicit-grant operator project. | New |
| operator/main.ae | Capability-bounded journal reader and receipt writer. | New |
| fault/aether.project.json | Locked M4/M19e fault persona project. | New |
| fault/main.ae | Checkpointed task, active cancellation, and terminal Error[Whole] boundary. | New |
| fault/task_success_test.ae | Cooperative checkpoint task success-path behavior test. | New |
| fixtures/atlas.journal | Canonical valid ATLAS/1 input fixture. | New |
| fixtures/corrupt/atlas.journal | Canonically shaped but malformed input fixture for rejection evidence. | New |
| tools/verify.ps1 | Fresh-output end-to-end evidence runner. | New |
| MANIFEST.md | Complete inventory for this delivery. | New |
| DELIVERY_REPORT.md | Rule-ID audit, verification record, risks, and review path. | New |

## Verification

    pwsh -NoProfile -File .\showcases\aether-atlas\tools\verify.ps1
    pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
    C:\Users\Bulkl\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe C:\Users\Bulkl\.codex\skills\agents-constitution\scripts\gate_check.py C:\WPAI\Software\XLang --na 13

Observed Atlas evidence:

- replay builds byte-identically with SHA-256
  02b482d20056abb4986f541eff047ab7df0c5743a0af2e0b1025c086454dc645;
- cross-package policy audit exits 0 after asserting canonical acceptance and
  malformed-frame rejection;
- pure replay exits 4720242 after SPEAKing atlas-replay;
- valid and corrupt grant-backed operator runs write the documented receipts;
- no-grant operator execution fails with AE-HOST-003;
- the fault persona is AETH v12 and exits 91;
- product structural representations cover both the operator and v12 fault
  persona;
- a one-byte-tampered replay artifact is refused with an AETH artifact error;
- protocol local package pack, verify, publish, cache verify, install, and
  installed-project verify pass; and
- full Aether gate passes: Constitution integrity, docs links, cargo format,
  clippy with warnings denied, and all workspace tests.

## Risks / Trade-offs

- ATLAS/1 is intentionally a fixed three-event, 62-scalar bounded grammar.
- Fingerprints are deterministic identifiers, not cryptographic integrity or
  authentication.
- There is no ambient I/O, network, database, fsync, or crash-recovery claim.
- Current product compilation keeps host-ABI code separate from cross-package
  imports; the operator is intentionally an isolated adapter.

## Next actions for human

1. Run the one-command evidence script and inspect its generated SUMMARY.md.
2. Review protocol/journal.ae, replay/main.ae, and fault/main.ae in that order.
3. Use this as the evidence baseline for any decision to expand Aether’s
   general multi-module host-ABI composition.
