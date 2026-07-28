# Delivery Report

**Title:** Aether M1 value and resource semantic decision package
**Date:** 2026-07-28
**One-sentence summary:** Delivered an evidence-backed, verifier-oriented
proposal for owned values, non-escaping loans, fixed-capacity arenas, explicit
allocation outcomes, and logical destruction; the model is intentionally
awaiting human approval before any M2 implementation.

## Scope

This is a documentation, research, and design-decision delivery. It does not
change Aether 0.5 source syntax, the AETH v4/v5 format, bootstrap compiler,
seed artifact/source, VM, forge ABI, Studio behavior, package dependencies, or
host authority.

The package resolves the required M1 design work into one proposed model:

- Copy values are `Whole` and `Truth`; `Text`, `Bytes`, records, and future
  buffers remain explicit owners.
- `borrow` is a read-only, non-escaping operation-scoped loan.
- `access` is a proposed exclusive, non-escaping arena-capability loan.
- New dynamic storage must use a named fixed-capacity arena, initially proposed
  at one 1,000,000-logical-byte arena per invocation.
- Construction reports `Allocation[T] = allocated(T) | exhausted`;
  fixed-capacity append reports `Append[T] = appended(T) | full(T)`; read-only
  indexed observation reports `Lookup[T] = found(T) | absent`.
- Failure is atomic: an expected allocation failure changes neither input-owner
  state nor arena capacity.
- Logical destruction executes no user code; M2 reclaims arena backing storage
  only at arena end.
- M2 begins with one Copy-element `Buffer`, not recursive aggregates,
  general references, manual free, user destructors, or host-ABI resource
  values.

The package is **not** an accepted semantic change. M1's roadmap status is
“decision package delivered — awaiting human approval”; ADR-003 remains
**Proposed**.

## Rule ID self-audit

| Rule ID | Status | Notes |
| --- | --- | --- |
| CONST-GATE-001 | Pass | Scope, applicability, research, design, risk, tests, documentation, and review packaging were checked before commit. |
| CONST-DONE-001 | Pass for this decision-package delivery | Complete design, evidence, counterexamples, layer impacts, test plan, ADR, doc sync, and validation evidence are present; M1 milestone approval remains a human action. |
| CONST-COMPLETE-001 | Pass | No partial compiler skeleton was introduced; the M1 deliverable includes the dependencies needed to make an informed semantic decision. |
| CONST-DEP-001 | Pass | M2 stays blocked behind human approval and requires source/IR/verifier/VM/seed proof as one complete increment. |
| ENG-WARN-001 | Pass | Rust formatting, Clippy with `-D warnings`, Studio ESLint with `--max-warnings=0`, tests, and production build passed. |
| TEST-BEHAVIOR-001 | Pass | Existing intended-behavior test suites passed unchanged; the M1 matrix defines future intended behavior rather than rewriting tests to current limitations. |
| DOC-SYNC-001 | Pass | Entry points, manifest, architecture boundary, claim register, roadmap, research, design, ADR, and validation matrix agree that the model is proposed, not 0.5 behavior. |
| DOC-ADR-001 | Pass | ADR-003 records the material decision, alternatives, compatibility/security implications, and explicit approval condition. |
| SEC-INPUT-001 | Pass | No parser/VM behavior changed; proposal preserves verifier-before-run, bounded resource admission, no ambient allocator, and no host-capability leak. |
| RND-INVAR-001 / RND-CORE-001 / RND-DOC-001 | Pass | Design has explicit invariants, finite proof plan, primary sources, counterexamples, and a bounded first implementation target. |
| IP-INVENTION-001 | Pass | The research records prior mechanisms and Aether-specific differentiation without making an unsupported novelty or superiority claim. |
| SOP-PHASE-001 | Pass through M1 design/review preparation | Research, decomposition, design, test plan, documentation, and gate evidence are complete; implementation awaits human decision. |
| REV-PACK-001 | Pass | This report contains scope, manifest, verification, known risks, rule traceability, review path, commit message, and next action. |
| VCS-ATOMIC-001 / VCS-CLEAN-001 | Pending commit at report creation | One documentation-only commit will contain the complete package; pre-existing unrelated untracked constitution-copy directories remain untouched. |

## Modules loaded

- Project `AGENTS.md`; constitution pack `AGENTS.md`, `SOP.md`, and Definition
  of Done.
- Engineering, testing, documentation, security, performance, constrained
  hardware, novel-R&D, IP/invention, institutional-memory, delivery, review
  packaging, and version-control modules as applicable.

## Manifest

| Changed file | Purpose |
| --- | --- |
| `docs/research/04-value-resource-models.md` | Official-primary-source rationale and rejected starting points for M1. |
| `docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md` | Complete proposed ownership, loan, arena, outcome, destruction, dynamic-buffer, layer, and approval specification. |
| `docs/ADR-003-value-resource-semantics.md` | Proposed material decision with consequences, alternatives, and explicit approval condition. |
| `docs/M1-VALIDATION-MATRIX.md` | Invariant/layer matrix, source/artifact negatives, runtime/property tests, seed plan, and M2 gate. |
| `docs/ROADMAP.md` | M1 status and immediate action synchronized to the delivered-but-unapproved decision package. |
| `docs/CORE_CLAIMS.md` | Links accepted explicit-allocation direction to the concrete proposed M1 model without promoting it to implementation. |
| `README.md` | Adds the M1 proposal to the design reading path with its current-status boundary. |
| `MANIFEST.md` | States that M1 is proposed and not Aether 0.5 surface area. |
| `docs/ARCHITECTURE.md` | Links the proposed model while preserving the implemented-architecture boundary. |
| `AGENTS.md` | Adds the M1 package to the project product-document map. |
| This report | Durable decision-package handoff, verification record, and review guide. |

## Review path

1. Read [ADR-003](ADR-003-value-resource-semantics.md) for the decision in
   roughly two minutes.
2. Read [DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md)
   sections 3–8 for exact rules and sections 9–12 for implementation gates.
3. Inspect [M1-VALIDATION-MATRIX.md](M1-VALIDATION-MATRIX.md) to confirm the
   proposal is falsifiable at source, verifier, VM, seed, and hostile-artifact
   boundaries.
4. Use [research/04-value-resource-models.md](research/04-value-resource-models.md)
   to audit the source basis and what was deliberately not imported.
5. Accept, reject, or request revisions to ADR-003 before authorizing M2 code.

## How to verify

```powershell
Set-Location C:\WPAI\Software\XLang
pwsh -File "..\..\AGENTS Constitution\tools\verify-pack.ps1"
cargo fmt --all -- --check
cargo test -p aether-core
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings

cargo run -p aether-cli -- check C:\WPAI\Software\XLang\examples\welcome.ae
cargo run -p aether-cli -- compile C:\WPAI\Software\XLang\examples\welcome.ae --output C:\WPAI\Software\XLang\target\welcome.aeth
cargo run -p aether-cli -- run C:\WPAI\Software\XLang\target\welcome.aeth
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash -Algorithm SHA256 .\seed\aether_seed.aeth,.\target\aether_seed.aeth,.\target\aether_seed.forged.aeth

Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
npm run lint
npm test
npm run build
```

Observed on 2026-07-28:

- Constitution pack integrity passed for pack version 5.0.1.
- `cargo fmt --all -- --check` passed.
- `cargo test -p aether-core` passed: 22 unit tests and 5 seed self-host tests;
  the full suite completed in 936.67 seconds.
- `cargo test -p aether-cli` passed: 1 test.
- `cargo clippy -p aether-core -p aether-cli -- -D warnings` passed.
- Welcome source check, seed-hosted compile, and artifact run passed; the run
  printed `Aether` and exited with 73.
- Bootstrap seed rebuild and forge passed. Checked-in, rebuilt, and forged seed
  artifacts all had SHA-256
  `7EE504CA4EB374686793FCB204226B2E5BCDD9B35A82B45C29E42303C303E62D`.
- Studio lint passed with zero warnings; 3 Studio tests passed; production build
  passed.
- Markdown relative-link validation and whitespace validation passed.

## Suggested commit message

```text
docs(language): define proposed M1 resource semantics
```

## Known risks and deliberate trade-offs

- The model is not accepted until the human decides ADR-003. It must not be
  described as implemented language behavior or used to start an unapproved
  compiler rewrite.
- The 1,000,000-logical-byte initial arena policy is a bounded proposed safety
  limit aligned to current 0.5 aggregate limits, not a measured performance
  claim or a final hardware-sizing result.
- Aether 0.5 text/byte operations remain bounded compatibility behavior with no
  source-visible allocator. M2 must add new forms rather than falsely claim
  existing operations are already explicit allocation.
- Fixed-capacity arena-end reclamation is simpler and safer for the first proof,
  but deliberately delays reuse, sharing, recursive aggregates, manual free,
  general errors/effects, and concurrency.
- Exact new source grammar and AETH byte layout are intentionally assigned to
  the M2 format/API ADR after approval; this package fixes their semantic
  contract and compatibility constraints first.

## Next action for the human

1. Review and choose the ADR-003 model: approve it, reject it, or request
   specific changes.
2. If approved, authorize M2 as one complete increment, not a spike: source
   forms, typed semantic IR, new AETH version, verifier, VM, seed proof,
   hostile-artifact tests, docs, and clean gates together.
3. Keep comparative claims scoped to [CORE_CLAIMS.md](CORE_CLAIMS.md); do not
   claim a performance/safety advantage until M2 produces the required
   measurements and counterexamples.

## Multi-agent coordination note

No subagents were used. The package was researched, authored, reviewed, and
validated as one coherent delivery.
