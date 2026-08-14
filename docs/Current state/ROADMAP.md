# Aether engineering roadmap

**Status:** Living near-term ledger; each future increment still requires a
scoped design, ADR, validation matrix, behavior tests, documentation sync, and
a Constitution gate.

**Current contract:** Aether 0.37.0, language surface 0.11, AETH v11 by default
and v12 for the bounded M19e task-frame surface. The executable contract is the
repository [MANIFEST](../../MANIFEST.md).

**Strategic program:** [Mainstream maturity program](MAINSTREAM-PROGRAM.md).
The 0.32-era multi-epoch plan remains [historical context](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md);
it is not a current status source or authorization.

## Current product baseline

| Area | Proven product boundary |
|---|---|
| Compiler authority | Seed-hosted default compile/check/format/structure; Rust bootstrap is recovery, full AST, and dual-compare oracle. |
| Language/runtime | Typed Whole/Truth/Text/Bytes, records, bounded resources/effects/layout/nurseries/tasks, and pure bounded comptime. |
| Projects and tooling | Offline projects/workspaces/modules, structural authoring v8, LSP, test reports, formatting, and local release/package verification. |
| Packages | Locked local source packages and cross-package imports; no implicit resolver, network fetch, or guest authority. |
| Host/system boundary | Explicit grant-backed I/O, Whole-only foreign pilot, verifier-first VM, and optional verified-AETH native pilots. |
| Evidence | Deterministic artifacts, verifier/hostile-artifact tests, seed/bootstrap corpus identity, and release consumer verification. |

## Immediate priority sequence

1. **BARP ADR-116 — release verified.** The bounded AE-SEED-011 seed-SPEAK
   witness covers canonical zero-argument direct bind and root-yield calls and
   has passed the full release gate. See [ADR-116](../historical%20docs/ADR-116-barp-seed-speak-zero-argument-unknown-call-pilot.md).
2. **BARP ADR-117 — release verified.** The bounded AE-SEED-011 seed-SPEAK
   witness covers canonical root `revise name <- call target` forms with the
   established next-space or exact end-of-line target boundary. See [ADR-117](../historical%20docs/ADR-117-barp-seed-speak-revise-unknown-call-pilot.md).
3. **BARP next design selection (current).** Choose exactly one broader AE-SEED-011 or
   AE-SEED-013 shape with an explicit false-positive boundary, or separately
   design seed-native multi-file elaboration. Do not infer parser/type-checker
   authority from the existing line-state witnesses.
4. **Mainstream language/library vertical.** Select one controlled generic,
   data-model, effect, or standard-library ergonomics increment only after its
   grammar, seed, verifier, VM, authoring, compatibility, and limit design are
   written and accepted.
5. **Daily-project maturity.** Add repeatable examples, compatibility/upgrade
   fixtures, and offline project workflows before expanding surface area merely
   for feature count.
6. **Authorized law-fork verticals.** F-NATIVE M35k+ and F-REGISTRY beyond
   M24i are available for a next scoped ADR, threat-model update, validation
   matrix, and release proof. Neither is an automatic expansion of current
   authority.

## BARP boundary

The active program is [BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md).
Current direct seed-SPEAK packets cover a deliberately bounded subset of
AE-SEED-003/004/005/006/007/010/011/012/013/014/015. The residual is not a
missing feature claim: full seed diagnostic conformance and seed-native
multi-file elaboration remain unproven. The product multi-file contract remains
host elaboration plus seed emission until a separate forge ABI design proves
otherwise.

## Law-fork boundary

| Fork | Current proven vertical | New work requires |
|---|---|---|
| F-NATIVE | M35a–M35j verified AETH-to-native pilots and closed target matrix | Scoped ADR, native threat-model update, toolchain/supply-chain policy, differential execution proof, and release gate |
| F-REGISTRY | M24a–M24i offline cache/trust/fetch/X.509-lite CA store | Scoped ADR, registry threat-model update, explicit fetch-only policy, hostile trust/input corpus, and release gate |

Human authorization allows those verticals to be selected; it does not weaken
verify-before-run/write, source-to-AETH product compilation, no ambient guest
authority, or the Constitution process.

## Milestone acceptance

Every increment must supply all applicable evidence:

1. Design and ADR for the material decision.
2. Intended-behavior tests, malformed/hostile inputs, priority/compatibility
   cases, and exact boundaries.
3. Seed-hosted product artifact proof and dual comparison wherever claimed.
4. Verifier/VM/forge changes that preserve capability closure.
5. Formatting, deny-warning static analysis, complete tests, link integrity,
   Constitution pack integrity, and release package/consumer verification.
6. Updated claim register, current documentation, dated delivery record, and
   atomic commit.

## Historical evidence

Past language milestones, decisions, audits, and delivery reports live in
[Historical docs](../historical%20docs/README.md). They explain provenance but
do not override the current contract, this ledger, or the active mainstream
program.
