# Delivery Report: M19e T-RX Active-Frame Cancellation Design

**Status:** Historical design milestone. At the time of this report, **no
compiler, VM, seed, source syntax, artifact, package, commit, or publish action
had been performed**. It is superseded for current implementation status by the
[M19e implementation delivery report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md).
**Date:** 2026-08-08
**Executable contract at design delivery:** [AETHER_0.35.md](AETHER_0.35.md) / AETH v11
**Current executable contract:** [AETHER_0.36.md](AETHER_0.36.md) / AETH v11 or v12
**Design:** [M19e active-frame cancellation and destruction](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md)
**Decision:** [ADR-042](ADR-042-m19e-active-frame-cancel.md)
**Implementation gate:** [M19e validation matrix](../Current%20state/M19E-VALIDATION-MATRIX.md)

---

## 1. Delivered decision boundary

This milestone completes the required **design → ADR → validation-matrix** gate
for the next T-RX work item: real ownership-aware cancellation of a started
task frame. It does not relabel the existing M19c behavior.

The accepted future design selects:

1. explicit v12-only `task weave` and `checkpoint` forms;
2. deterministic source-order round-robin scheduling with no OS threads or
   wall-clock behavior;
3. cancellation only for a started task parked at a verifier-approved,
   empty-stack checkpoint;
4. reverse-slot logical destruction, private task arena lanes, slab-level
   zeroization/release, and no user cleanup code;
5. a closed task/companion subset that excludes resource crossings, ordinary
   task calls, host/foreign/stdout work, nested nurseries, task handles, and
   free-on-raise; and
6. AETH v12 metadata/verifier, seed parity, authoring v8, compatibility, and
   hostile-artifact proof as one later implementation delivery.

The proof fixture in the design is intentionally stronger than M19c: a task
allocates a local Buffer, parks at `checkpoint`, a later sibling raises, and the
task is destroyed before its next statement or `yield`. Its parent destination
remains unchanged and the sibling's `Error[Whole]` propagates only after join.

## 2. What remains unimplemented

Package 0.35/AETH v11 is still the only executable product contract. In
particular, the current compiler does **not** accept `task weave` or
`checkpoint`; the seed does not emit v12; the v11 VM remains synchronous for a
started nursery child; and CLM-030 remains the only Proven-now Policy B claim.

No source example, generated artifact, benchmark, or performance/safety claim
was added for M19e. The matrix deliberately leaves implementation rows unchecked
until intended-behavior, negative-source, and hostile-artifact evidence exists.

## 3. Research and implementation evidence

The design was reconciled against the implemented M1/M7/M16/M19a–d contracts,
the current compiler/verifier/VM structure, the project research synthesis, and
the historical language intent. The current runtime confirms that `NURSERY_SPAWN`
recursively executes a child to completion; a new task-frame model is therefore
necessary for a truthful active-frame claim.

The design also records current primary-source comparison inputs: Rust
destruction semantics, Swift structured-concurrency cancellation, and Tokio's
task/cancellation-safety documentation. They inform the safety boundary only;
Aether copies no external API or scheduler semantics.

## 4. Documentation synchronization

The following current-facing documents now distinguish the accepted M19e design
from the unchanged 0.35/v11 product:

- [ROADMAP.md](../Current%20state/ROADMAP.md) and [BACKLOG-HUMAN-2026-08-05.md](BACKLOG-HUMAN-2026-08-05.md)
  make M19e the next complete implementation gate.
- [CORE_CLAIMS.md](../Current%20state/CORE_CLAIMS.md) adds CLM-039 as **Accepted direction**, while
  retaining CLM-030's bounded current claim.
- [MANIFEST.md](../../MANIFEST.md), [AETHER_0.35.md](AETHER_0.35.md),
  [ARCHITECTURE.md](../Current%20state/ARCHITECTURE.md), [SEED_PROFILE.md](../Current%20state/SEED_PROFILE.md),
  [NORTH_STAR.md](../Current%20state/NORTH_STAR.md), and
  [PROGRESS_REPORT-FULL-PROJECT.md](../Current%20state/PROGRESS_REPORT-FULL-PROJECT.md) state that
  no v12/product behavior has shipped.

## 5. Verification performed

All commands ran from `C:\WPAI\Software\XLang` on 2026-08-08:

| Command | Result |
| --- | --- |
| Changed/current-document Markdown link check | Pass — all local links resolved. |
| `git diff --check` | Pass. Git emitted workspace CRLF-normalization notices; no whitespace error was reported. |
| `cargo fmt --all -- --check` | Pass. |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full -SkipPack` | Pass — format, Clippy with warnings denied, core/CLI tests, documented seed≡bootstrap corpus, host-pilot, project flows, and seed forge hash identity all completed. |

The full gate used `-SkipPack` because the checkout lacks `tools/verify-pack.ps1`.
No package-preview verification is claimed, and this design-only milestone
creates no package artifact to verify.

## 6. Constitution self-audit

| Rule intent | Result |
| --- | --- |
| Dependency-first / complete boundary | Pass for design scope: source, artifact, verifier, VM, seed, authoring, capacity, teardown, and test dependencies are specified together. |
| Honest claims | Pass: current v11 remains unstarted-only cancellation; M19e is Accepted direction, not Proven now. |
| Intended-behavior testing | Pass for the design gate: the matrix fixes P/N/H expected behavior before implementation; no existing test was weakened. |
| Security / least authority | Pass: no host effects, task handles, callbacks, resource escape, ambient allocator, or external cancellation authority is introduced. |
| Documentation synchronization | Pass for M19e design scope: contract, claims, roadmap, seed boundary, architecture, north star, backlog, and progress report agree. |
| Zero-warning gate | Pass for the existing executable baseline; no M19e code exists yet. |

## 7. Next lawful action at design delivery

Implement M19e only as the full v12 vertical slice described in the design and
matrix. A parser-only `checkpoint`, a scheduler-only cancellation flag, or a
logical-drop-only patch would fail this ADR and must not be delivered as
active-frame cancellation.

---

*End of M19e design delivery report.*
