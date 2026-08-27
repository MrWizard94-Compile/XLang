# Aether current state

This collection is the living documentation surface for Aether. It describes
the current supported product and active, explicitly scoped engineering
direction; it does not replace the dated evidence in
[Historical docs](../historical%20docs/README.md).

## Start here

| Need | Current document |
|---|---|
| Product and CLI contract | [Aether 0.37](AETHER_0.37.md), [Architecture](ARCHITECTURE.md), [Forge contract](FORGE_CONTRACT.md) |
| Language and authoring rules | [Aether 0.11 surface](AETHER_0.11.md), [Authoring protocol v8](AETHER_AUTHORING_PROTOCOL_v8.md), [Seed profile](SEED_PROFILE.md) |
| Current status and order of work | [Progress report](PROGRESS_REPORT-FULL-PROJECT.md), [Roadmap](ROADMAP.md), [Mainstream program](MAINSTREAM-PROGRAM.md), [Core claims](CORE_CLAIMS.md), [North star](NORTH_STAR.md) |
| BARP / seed authority reduction | [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md), [BARP validation matrix](BARP-VALIDATION-MATRIX.md), [SBP bundle matrix](SBP-VALIDATION-MATRIX.md), [SBP-001 v1 threat model](THREAT_MODEL-SBP-001-SEED-BUNDLE.md), [SBP-002 v2 threat model](THREAT_MODEL-SBP-002-SEED-CHAIN.md), [SBP-003 v3 threat model](THREAT_MODEL-SBP-003-SEED-FANIN.md) |
| Local package lifecycle | [M25 matrix](M25-VALIDATION-MATRIX.md), [0.37 release notes](RELEASE_NOTES-0.37-LOCAL-PACKAGES.md), [package threat model](THREAT_MODEL-0.37-LOCAL-PACKAGES.md) |
| Native and registry law forks | [Native authorization](HUMAN-AUTHORIZE-NATIVE.md), [native design](DESIGN-LAW-FORK-F-NATIVE.md), [M35a matrix](M35A-VALIDATION-MATRIX.md); [registry authorization](HUMAN-AUTHORIZE-REGISTRY.md), [registry design](DESIGN-LAW-FORK-F-REGISTRY.md), [M24a matrix](M24A-VALIDATION-MATRIX.md) |
| Capability and backend risks | [Capable-host](THREAT_MODEL-v2-CAPABLE-HOST.md), [foreign ABI](THREAT_MODEL-v3-FOREIGN-ABI.md), [native](THREAT_MODEL-v4-NATIVE-BACKEND.md), and [registry](THREAT_MODEL-v5-PACKAGE-REGISTRY.md) threat models |

## Reading rules

- The repository root [MANIFEST](../../MANIFEST.md) and [AGENTS entry](../../AGENTS.md) remain binding entry documents.
- A current document must link to historical evidence when a claim depends on a
  dated decision, delivery report, benchmark, or validation record.
- The previous mainstream roadmap is historical context, not an active
  implementation claim: [2026-08-05 baseline](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md).
- The documentation topology and link-integrity contract are recorded in
  [Documentation architecture](DOCUMENTATION-ARCHITECTURE.md).
