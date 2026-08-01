# M1 Validation Matrix: Value and Resource Semantics

**Status:** Active validation matrix — Aether 0.6 delivers the bounded M2
subset; broader proposed rows remain future obligations
**Date:** 2026-07-28; implementation record updated 2026-08-01
**Scope:** Intended-behavior and hostile-artifact obligations for resource
work. The exact implemented subset is [AETHER_0.6.md](AETHER_0.6.md) and
[ADR-004](ADR-004-aeth-v6-bounded-resources.md).

## Purpose

M1 is complete only if its rules have counterexamples, source-negative cases,
verifier consequences, seed feasibility, and an implementation-proof plan. This
matrix turns the proposed model in
[DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md)
into testable contracts before any parser/VM change is authorized.

The tests below must be written against these intended semantics. A failing test
is evidence to correct the implementation or return to human design review; it
is not permission to weaken the test into current behavior.

## Aether 0.6 implementation record

The following rows are delivered by the Aether 0.6 bounded M2 surface and are
covered in the core behavior/hostile-artifact suite plus the seed corpus:

| Matrix IDs | Evidence now in the product |
| --- | --- |
| M1-OWN-001, M1-OWN-002 | Existing explicit owner move/borrow validation remains active; buffers participate as owners. |
| M1-LOAN-001, M1-ACCESS-001 | `borrow` and `access` are operation-scoped in the accepted M2 grammar; the verifier refuses persistent access representation and distinguishes transient buffer borrows. |
| M1-REGION-001, M1-REGION-002 | One named main arena plus access-bound helper relation is source-checked; buffers cannot be returned from weaves or cross the host ABI. |
| M1-ALLOC-001 through M1-ALLOC-003 | Named access, explicit bright/dim handling, bounded checked accounting, and owner/capacity preservation on dim paths. |
| M1-BUF-001, M1-BUF-002 | Whole/Truth-only element tags, metadata-plus-payload accounting, exhaustion/full behavior, and checked arithmetic. |
| M1-CFG-001 | Resource outcomes are terminal, avoiding unproven owner joins; verifier state merges include stack provenance. |
| M1-COMPAT-001, M1-SEED-001, M1-BOUND-001 | v4/v5 compatibility, v6 verifier gate, six-example seed/bootstrap byte corpus, and primitive-only host/forge ABI. |

The following parts of the original broader plan are deliberately **not**
claimed complete in 0.6: first-class/propagated outcomes, Buffer weave results,
general resource `revise`, property-generated sequence suites, generic effects,
and any resource extension beyond one arena and fixed-copy-element buffers.
They remain a future design-and-proof obligation rather than a silent gap.

## Historical current-boundary audit

| Current fact | Evidence location | M2 implication |
| --- | --- | --- |
| Source validation tracks `BindingState { value_type, mutable, moved }`. | `crates/xlang-core/src/lib.rs` | New forms need typed owner/loan/region state; a Boolean alone cannot prove loans or provenance. |
| Artifact verification tracks initialized/moved locals and requires identical incoming control-flow state. | `crates/xlang-core/src/lib.rs` | New verifier states must encode resource-plan, region, loan, and outcome facts. |
| `LOAD` and source `borrow` preserve existing 0.5 observable behavior; VM values currently clone on load. | `crates/xlang-core/src/lib.rs`, [AETHER_0.5.md](AETHER_0.5.md) | Do not advertise existing borrow as a general zero-copy reference or reinterpret v4/v5. |
| `REVISE` requires a readable mutable local in the verifier. | `crates/xlang-core/src/lib.rs` | Source checker must also reject a self-move RHS before code emission. |
| Current seed proof covers documented canonical 0.5 source only. | [SEED_PROFILE.md](SEED_PROFILE.md) | Every accepted M2 source form needs its own bootstrap/seed byte-identity corpus. |
| VM/forge accept verified AETH only and expose no host capability to guest code. | [ARCHITECTURE.md](ARCHITECTURE.md), [MANIFEST.md](../MANIFEST.md) | Arena instructions must remain VM-private and be verifier-gated before any run/write. |

## Invariant-to-layer matrix

| ID | Invariant / intended behavior | Source checker | Semantic IR | AETH verifier | VM | Seed / corpus |
| --- | --- | --- | --- | --- | --- | --- |
| M1-OWN-001 | Copy values read without move state; owners do not. | Classify categories and reject ordinary owner use. | `copy` versus owner place operation. | Type-check compatible load/copy form. | Preserve value semantics. | Scalar and owner contrast cases. |
| M1-OWN-002 | Owner transfer is whole-value and explicit. | Require `move` where ownership is consumed. | `move place` transitions only at commit. | Reject use-after-move and duplicate move. | Remove source owner only on successful transfer. | Move/result/call corpus. |
| M1-LOAN-001 | `borrow` is read-only and operation-scoped. | Reject borrow escape/storage/yield/jump crossing. | Create/end `ReadLoan` inside one operation. | Reject persistent loan stack/local state. | No user-visible reference object. | Borrow call/projection corpus. |
| M1-ACCESS-001 | `access` is exclusive and arena-only. | Accept only live Arena; reject overlap/escape. | Create/end `AccessLoan(Arena(ρ))`. | Check operand and resource relation. | Mutate only VM-private arena accounting. | Allocation helper corpus. |
| M1-REGION-001 | Every dynamic value has a visible arena identity. | Validate matching signature relation. | Carry `ρ` in Buffer type. | Validate region metadata and operand types. | Associate storage with declared arena. | Cross-weave return corpus. |
| M1-REGION-002 | A local arena-derived value cannot escape. | Reject return/yield/store that outlives local arena. | Reject invalid region escape. | Reject artifact path with invalid region relation. | Defense-in-depth type/provenance check. | Local-escape negative case. |
| M1-ALLOC-001 | Dynamic allocation has no ambient allocator. | Require `access arena` at every allocation. | Emit `alloc ρ` fact. | Require declared arena reference. | No fallback allocation route. | Missing-arena negative case. |
| M1-ALLOC-002 | Allocation result is visible and total. | Require match/declared propagation. | `Allocation[T]` branch required. | Validate outcome stack/tag transitions. | Return `allocated` or `exhausted`, never panic. | Success/exhausted corpus. |
| M1-ALLOC-003 | Allocation failure is atomic. | Preserve owner states on failure path. | Commit only after reserve succeeds. | Validate both successor states. | Preserve owner and capacity on failure. | Forced-exhaustion fixture. |
| M1-REV-001 | `revise` evaluates RHS before destroying old value. | Reserve target; permit only its read loan. | `ReservedForRevise` then commit. | Reject revise of a moved/loaned target. | Retain old value on failure; replace on success. | Borrow-target revise corpus. |
| M1-REV-002 | `revise x = move x` is invalid. | Reject before emission. | No legal transition. | Reject crafted move/revise sequence. | Must not execute it. | Source and hostile-artifact negatives. |
| M1-DROP-001 | Destruction has no user code or host effects. | No destructor syntax/escape route. | Compiler-only `destroy`. | No arbitrary callback opcode. | Logical cleanup only. | Scope-end corpus. |
| M1-DROP-002 | M2 uses arena-end reclamation, not individual free/reset. | Reject/reset omission by grammar. | No individual-free IR node. | Reject invalid free/reset opcodes. | Reclaim only at arena end. | Retention-bound fixture. |
| M1-BUF-001 | M2 Buffer elements are Copy-only. | Accept only Whole/Truth. | Restrict `κ`. | Validate element tag. | Type-check insertion/read. | Text/record element negatives. |
| M1-BUF-002 | Fixed capacity accounts safely. | Validate non-negative, bounded capacity expression. | Encode checked byte request. | Reject overflow/out-of-plan capacity. | Checked arithmetic and deterministic outcome. | Max, overflow, exhausted cases. |
| M1-CFG-001 | Control-flow joins cannot hide ownership/resource disagreement. | Join only compatible states. | Explicit merged owner/region state. | Reject divergent incoming state. | Not relied upon for safety. | Branch/loop move and outcome cases. |
| M1-COMPAT-001 | AETH v4/v5 keep existing byte meaning. | Do not emit new forms in old versions. | Version gate. | Reject new resource opcodes in v4/v5. | Decode only supported version. | Existing 0.5 regression corpus unchanged. |
| M1-SEED-001 | Seed and bootstrap emit canonical M2 identically. | Same accepted canonical corpus. | Equivalent lowering contract. | Verify each emitted artifact. | Run selected artifacts. | Byte-for-byte tri-compare. |
| M1-BOUND-001 | Guest resource values never cross host ABI. | No source bridge form. | No host-lowering route. | Artifact ABI signature excludes these types. | Invoke/forge rejects them. | Host-boundary negative tests. |

## Source-negative test catalog

The implementation must add focused tests with stable diagnostic codes and spans
once an M2 grammar is accepted. Test names may follow project convention, but
the behavior and rejection reason below are mandatory.

| Test ID | Invalid shape | Required result |
| --- | --- | --- |
| M1-SRC-001 | Ordinary use of a `Buffer` owner. | Reject: explicit `borrow` or `move` required. |
| M1-SRC-002 | Second `move` of an owner. | Reject: already moved. |
| M1-SRC-003 | `borrow` of a moved owner. | Reject: moved value cannot be borrowed. |
| M1-SRC-004 | Return, yield, bind, record-store, or buffer-store of a read loan. | Reject: non-escaping loan. |
| M1-SRC-005 | Return/yield/store of an `access` arena loan. | Reject: non-escaping exclusive capability. |
| M1-SRC-006 | Allocation form without named `access` arena. | Reject: no ambient allocator. |
| M1-SRC-007 | Allocation from a capability other than an Arena. | Reject: capability type mismatch. |
| M1-SRC-008 | Buffer result tied to a locally created arena. | Reject: region escape. |
| M1-SRC-009 | Weave signature mentions Buffer region without matching arena relation. | Reject: unbound region identity. |
| M1-SRC-010 | `revise x = move x`. | Reject before bytecode emission. |
| M1-SRC-011 | `revise` target revised twice or accessed exclusively during its RHS. | Reject: replacement reservation conflict. |
| M1-SRC-012 | Ignored allocation outcome. | Reject: outcome must be handled/propagated. |
| M1-SRC-013 | Buffer of Text, Bytes, record, Buffer, Arena, or outcome. | Reject: M2 element must be Whole or Truth. |
| M1-SRC-014 | Capacity below zero, above policy, or arithmetic-overflowing. | Reject: bounded arena request invalid. |
| M1-SRC-015 | Manual free/reset/destructor syntax. | Reject: M2 does not expose individual reclamation/user destruction. |
| M1-SRC-016 | Arena/Buffer/outcome used in forge or primitive invoke boundary. | Reject: host ABI excludes resource values. |
| M1-SRC-017 | Read/access loan reaches a branch merge, loop back-edge, or yield. | Reject: loan scope crosses control-flow boundary. |
| M1-SRC-018 | Fallible call appears to consume an owner before `exhausted` branch. | Reject: failure-atomicity violation. |

Each case must assert diagnostic identity and a precise source span where the
parser/checker has enough information. The seed compiler's canonical valid
emission parity does not need full invalid-source diagnostic parity; the Rust
bootstrap remains the diagnostic authority as documented for Aether 0.5.

## Hostile-artifact verifier catalog

Tests must construct or mutate AETH bytes directly so the verifier is tested as
the security boundary rather than merely mirroring the parser.

| Test ID | Artifact condition | Required verifier result |
| --- | --- | --- |
| M1-ART-001 | New resource opcode in v4 or v5 artifact. | Reject before execution: version/opcode mismatch. |
| M1-ART-002 | Unknown future resource artifact version. | Reject before execution. |
| M1-ART-003 | Missing or duplicate arena resource-plan record. | Reject malformed metadata. |
| M1-ART-004 | Arena capacity exceeds M2 policy or overflows encoded arithmetic. | Reject resource plan. |
| M1-ART-005 | Allocation references undeclared/out-of-scope arena. | Reject invalid region reference. |
| M1-ART-006 | Buffer type/arena identity disagrees with allocation operand. | Reject type/provenance mismatch. |
| M1-ART-007 | Crafted `MOVE; REVISE` of same local. | Reject moved-local revision. |
| M1-ART-008 | Loan/access value stored in a local, record, result, or persistent stack state. | Reject non-escaping loan representation. |
| M1-ART-009 | Join inputs disagree about owner/loan/region/outcome state. | Reject non-convergent resource control flow. |
| M1-ART-010 | Allocation outcome consumed as a buffer or discarded before yield. | Reject outcome stack/type misuse. |
| M1-ART-011 | Individual free/reset or destructor callback opcode. | Reject unknown/forbidden resource operation. |
| M1-ART-012 | Arena/Buffer/outcome appears in primitive invoke/forge compile signature. | Reject ABI signature. |
| M1-ART-013 | Resource instruction has invalid immediate length, truncated bytes, or trailing bytes. | Reject malformed encoding. |
| M1-ART-014 | Capacity request succeeds after arena is exhausted. | Reject dynamic invariant or VM fail-closed. |
| M1-ART-015 | Guest path reaches VM without verified resource metadata. | Impossible by API; assertion test confirms verify-before-run/write. |

## Runtime behavior and property tests

| Test ID | Fixture | Required observation |
| --- | --- | --- |
| M1-VM-001 | One admitted arena with one successful buffer construction. | Remaining capacity decreases by exactly checked payload-plus-metadata bytes. |
| M1-VM-002 | Request beyond remaining capacity. | `exhausted` result; owner inputs and remaining capacity are byte-for-byte/state-for-state unchanged. |
| M1-VM-003 | Successful fallible replacement. | Old owner is destroyed only after replacement is ready; result is live. |
| M1-VM-004 | Failed fallible replacement. | Original mutable binding remains live and unchanged. |
| M1-VM-005 | Scope exit with live buffer. | Logical destruction occurs; no callback/I/O; arena reclamation occurs only at arena end. |
| M1-VM-006 | Repeated append until capacity full. | Defined visible full/outcome behavior; no fallback allocation or host failure. |
| M1-VM-007 | Integer boundary capacities and element counts. | No wraparound, negative conversion, or unchecked multiplication. |
| M1-VM-008 | Valid M2 artifact under a no-network/no-file/no-process test harness. | Same guest capability set as current AETH VM: none. |
| M1-PROP-001 | Generated valid owner-operation sequences. | Exactly one logical owner at every program point; no use after move. |
| M1-PROP-002 | Generated capacity requests within/outside remaining budget. | Success iff checked request fits; failure preserves state. |
| M1-PROP-003 | Generated branch/loop resource states. | Only compatible joins verify; verifier never accepts an ambiguous owner state. |
| M1-PROP-004 | Byte mutations around resource metadata/opcodes. | Decoder/verifier rejects malformed artifacts without panic or execution. |

Property tests must be deterministic: pin the generator seed in failures, bound
artifact/input size, preserve the minimized reproducer, and keep VM execution
inside the declared arena budget.

## Seed feasibility and reproducibility plan

1. Extend the bootstrap parser and typed semantic IR first; no seed source form
   is admitted until its typed IR and verifier rules are specified.
2. Add a small canonical M2 corpus covering arena declaration, successful
   allocation, exhausted outcome, `access` helper call, buffer movement,
   borrowed observation, successful/failed `revise`, and allowed region return.
3. Implement the same canonical source lowering in `seed/aether_seed.ae`.
4. Rebuild the checked-in seed artifact using the Rust bootstrap.
5. Compile each canonical M2 corpus source through both bootstrap and default
   seed product paths; compare bytes, verify both artifacts, and execute the
   behavior fixtures.
6. Forge the seed source and compare the forged artifact with the rebuilt seed
   artifact.
7. Keep invalid-source diagnostics as bootstrap tests unless and until a later
   approved seed-diagnostic parity project explicitly changes that boundary.

The initial corpus must include both AETH compatibility checks for unchanged
v4/v5 inputs and the new resource-version artifact samples. No new source
feature becomes default product compile merely because the bootstrap compiles it.

## Required implementation gate

After ADR-003 approval, M2 cannot be considered complete unless all relevant
items below pass in the same delivery:

```powershell
Set-Location C:\WPAI\Software\XLang
pwsh -File "..\..\AGENTS Constitution\tools\verify-pack.ps1"
cargo fmt --all -- --check
cargo test -p aether-core
cargo test -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings

cargo run -p aether-cli -- check <canonical-m2-source>
cargo run -p aether-cli -- compile --bootstrap <canonical-m2-source> --output <bootstrap-artifact>
cargo run -p aether-cli -- compile <canonical-m2-source> --output <seed-artifact>
cargo run -p aether-cli -- run <seed-artifact>
cargo run -p aether-cli -- forge <rebuilt-seed> .\seed\aether_seed.ae --output <forged-seed>

```

The actual delivery report must record exact test counts, failed-test absence,
hashes for each applicable artifact comparison, capacity-policy measurement, and
any intentionally deferred behavior. It must not report M1/M2 as complete based
only on source parsing or happy-path VM execution.

## M1 review conclusion

This matrix provides the required negative compile cases, verifier
consequences, seed plan, hostile-input requirements, deterministic test
strategy, and success gate for the proposed model. It does not substitute for
human approval of the model or for the M2 implementation/proof package.
