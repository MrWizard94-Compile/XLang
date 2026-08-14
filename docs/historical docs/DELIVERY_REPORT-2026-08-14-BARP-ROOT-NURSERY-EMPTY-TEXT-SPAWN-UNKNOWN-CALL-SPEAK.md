# Delivery Report — BARP root nursery exact empty Text spawn unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Release verified
**Summary:** ADR-126 extends the bounded AE-SEED-011 nursery witness with one exact empty Text literal without turning the seed into an M7 parser, task checker, or scheduler.
**Scope:** ADR-126 direct-seed root-nursery exact empty Text spawn unknown-target diagnostic authority reduction
**Rule IDs:** CONST-DEP-001, DOC-ADR-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

After the zero-argument (ADR-121), one-digit Whole (ADR-122), two-digit
positive Whole (ADR-123), exact bright Truth (ADR-124), and exact dim Truth
(ADR-125) root-nursery witnesses, the checked-in seed now recognizes only:

~~~aether
spawn call target "" into destination
~~~

The parent must remain an ordinary total-Whole root weave with a literal
"together:" line and an immediate four-space child. The target and destination
must be nonempty, and the source must contain the exact literal """ into "
after the target's first delimiter. A missing top-level weave declaration emits
the established AE-SEED-011 seed-SPEAK packet. A later ordinary "weave worker
[message: Text] -> Whole" target called with "" remains valid, byte-identical
between seed and bootstrap, verified, and executable.

This is deliberately not Text task-frame support. M19e task frames remain
closed to owned Whole and Truth copy parameters.

## Delivery contents

| Artifact | Delivered content |
| --- | --- |
| "seed/aether_seed.ae" and checked-in artifact | Exact bounded line-state branch and bootstrap-rebuilt seed artifact |
| "crates/xlang-core/src/lib.rs" | Explicit ADR-126 tracker and contract boundary |
| "crates/xlang-core/tests/seed_self_host.rs" | Direct, product, valid, lexical, signature, caller-state, delimiter, and priority evidence |
| ADR-126 and BARP matrix | Decision, false-positive boundary, direct/valid/product/priority matrix entries |
| Current contract docs | Accurate exact-empty-Text scope and no task-frame expansion claim |

## Verification evidence

| Check | Status |
| --- | --- |
| Test-first regression | PASS — the new canonical source produced no direct AE-SEED-011 before the seed change |
| Targeted direct/product/valid proof | PASS — one exact packet, product merge, byte identity, verification, and exit 42 |
| False-positive boundary | PASS — nonempty Text, Bytes, name, repeated argument, missing destination, delayed/descendant child, wrong parameter, and erroring parent do not emit ADR-126 |
| Priority boundary | PASS — missing world retains AE-SEED-006 over the lower-priority witness |
| Full seed self-host corpus | PASS — 45 passed, 0 failed |
| BARP tracker | PASS — ADR-126 registers while full conformance remains false |
| Seed Profile variant boundary | PASS — no seed binding was added; the established v132 unused-local self-host variant probe remains intact |
| Release-quality gate | PASS — 2026-08-14 release gate: pack v5.0.1, 359 Markdown files / 1,447 local links, zero-warning workspace suite, four-way seed identity, 444-file package plus SHA-256SUMS, and independent consumer verification |

## Security, performance, and authority boundary

The recognition branch is a bounded source-line scan: it requires one literal
prefix, searches for the first target delimiter, and checks a literal empty
Text marker plus a nonempty suffix. It reuses the existing opaque
top-level-header scan. It adds no host filesystem, process, shell, network,
model, guest-capability, VM, AETH, parser, scheduler, resource, or task-frame
authority.

Full bootstrap authority remains responsible for nonempty or escaped Text,
Bytes, names, type/ownership, task parameters, signature, destination,
nesting, effects/results, resources, and scheduler semantics.

## Section 0 status

Implementation, rebuild, focused behavior evidence, predecessor evidence,
full self-host, documentation synchronization, static tracker evidence, and
the final release gate are complete. The gate passed with pack v5.0.1, 359
Markdown files / 1,447 local links, seed SHA-256
"D0D17756F587709BC85E323E0545281E304362288B687BAA0D48D8C334E18AFB", a
444-file technical-preview package plus SHA-256SUMS, and independent consumer
verification.

## Final Section 0 self-audit

1. **Completeness:** the seed matcher, rebuilt artifact, tracker, intended-behavior
   coverage, ADR, matrix, current contracts, and historical cross-references ship
   together; no deferred-work marker or stub was introduced.
2. **Dependency-first:** the seed source and checked-in seed artifact change as
   one verified dependency set, with the Rust tracker and tests wired to it.
3. **Zero warnings/errors:** the final release gate passes formatter, strict
   workspace Clippy, build, and test checks.
4. **Tests:** direct packet, product merge, valid ordinary-Text source,
   lexical/delimiter/caller/type/priority boundaries, full self-host, and
   workspace tests pass.
5. **Documentation:** the release gate verifies 359 Markdown files and 1,447
   local links; scope wording distinguishes exact empty Text from general Text.
6. **Security:** the line-state matcher adds no host, guest, filesystem, process,
   shell, network, model, or task-frame authority.
7. **Performance:** one literal bounded scan reuses existing line state and
   header scan; no unbounded parser, allocator, or runtime hot path is added.
8. **Stack fidelity:** Rust 2021 / MSRV and Aether 0.37 contracts remain
   unchanged; no dependency, AETH, VM, or capability change occurs.
9. **Package readiness:** a 444-file SHA-256SUMS package and independent
   consumer both pass.
10. **Constraints:** verifier-first, seed-default, bootstrap-oracle, no-ambient-
    authority, task-frame Whole/Truth closure, and honest-parity boundaries hold.
11. **Reproducibility:** bootstrap, product, forge, and checked-in seed artifacts
    share SHA-256 D0D17756F587709BC85E323E0545281E304362288B687BAA0D48D8C334E18AFB.
12. **IP hygiene:** no external code, asset, dependency, or license material is
    introduced.
13. **Coordination:** no subagent or parallel implementation work was used.
14. **Review packaging:** ADR, delivery report, validation matrix, product
    contracts, and progress record are synchronized.
15. **Self-audit:** mechanical constitution check reports 0 FAIL, 0 WARN,
    11 MANUAL reviewed above, 3 PASS, and 1 N/A (multi-agent coordination).

## Repeat verification

~~~powershell
cargo test -p aether-core --test seed_self_host
cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
~~~

## Residuals

ADR-121 through ADR-125 remain discrete predecessor witnesses. General Text
including nonempty or escaped literals, Bytes, names, multiple arguments, Text
task-frame parameters, general parsing, semantic validation, full diagnostic
parity, and seed-native multi-file elaboration remain outside this ADR.

---

*End of ADR-126 delivery report.*
