# Aether mainstream maturity program

**Status:** Active, dependency-ordered engineering program; not a release claim,
calendar promise, or peer-superiority claim.

**Current product baseline:** Aether 0.37.0; AETH v11 by default and v12 for
the bounded M19e task-frame surface. The executable contract is the repository
[MANIFEST](../../MANIFEST.md). The earlier 0.32 planning baseline is retained as
[historical context](../historical%20docs/ROADMAP-MAINSTREAM-MATURITY.md), not
as a current scope or status source.

## Target meaning

“Mainstream level” means that a small team can choose Aether for bounded,
local-first systems and AI-authoring work without treating it as a toy. It does
not mean syntactic parity with Rust, C++, Go, Java, Python, or TypeScript; it
does not imply benchmark superiority; and it never permits weakening Aether’s
verifier-first or local-first guarantees.

The target is reached only when the following pillars have evidence at the
same time:

| Pillar | Current verified baseline | Maturity gap to close |
|---|---|---|
| Product authority | Seed-hosted default compile/check/format/structure; bootstrap recovery/oracle; bounded product LSP surface. | Continue BARP without claiming seed diagnostic or multi-file parity before proof. |
| Language usability | Typed Whole/Truth/Text/Bytes, records, modules, effects/resources, bounded tasks, comptime, and structural authoring. | Controlled generic/data modeling, richer library ergonomics, and explicit evolution policy. |
| Tooling and DX | Offline projects/workspaces, formatting, tests/reports, LSP, package verification, and a release gate. | Daily-project workflows, compatibility fixtures, tutorials, and stable extension/version policy. |
| Packages and reuse | Locked local packages, cross-package imports, cache verification, and F-REGISTRY through M24i. | Resolver/publisher-policy work only through an ADR-backed next registry vertical. |
| Systems and deployment | Grant-backed host I/O, narrow foreign ABI, verified AETH→native pilots through M35j, VM reference path. | Reproducible native toolchain/sysroot work only through an ADR-backed next native vertical. |
| Trust and evidence | AETH verification, explicit grants, deterministic artifacts, dual-compare corpus, threat models, and Constitution gates. | Expand hostile corpora, compatibility evidence, and real workload evidence as each new surface lands. |

## Governing constraints

1. Aether source remains Aether-only: the product compiler emits verified AETH;
   optional native work starts from verified AETH and never becomes source
   transpilation.
2. Verify-before-run/write, explicit host grants, offline default operation,
   deterministic artifacts, and dual-compare proof are non-negotiable.
3. Every material increment has its own design/ADR, validation matrix, tests,
   documentation update, and Constitution gate. A green happy path is not a
   completion criterion.
4. Human authorization of F-NATIVE and F-REGISTRY permits selecting their next
   verticals; it does not waive the per-vertical ADR, threat-model, matrix, or
   release evidence requirement.

## Dependency order

| Order | Program | Immediate bounded outcome | Admission gate |
|---:|---|---|---|
| 1 | BARP product independence | Broaden exact seed-SPEAK witnesses and separately design seed-native multi-file elaboration. | Seed rebuild, direct forge packet/origin proof, valid-source identity, priority negatives, full dual-compare. |
| 2 | Language and library depth | Specify only one controlled data/generic/effect ergonomics slice at a time. | Grammar, seed, verifier, VM, authoring, corpus, compatibility, and performance/limit evidence. |
| 3 | Daily-project experience | Improve documentation, project/workspace/package ergonomics, LSP behavior, test reporting, and reproducible examples. | Offline-first end-to-end project proof plus negative path/jail and no-silent-write tests. |
| 4 | F-REGISTRY next vertical | Select a resolver, trust, or standards-compliance gap only after a scoped ADR and updated registry threat analysis. | Explicit operator action, no compile-time auto-fetch, reproducible cache verification, hostile trust/input corpus. |
| 5 | F-NATIVE next vertical | Select reproducible toolchain/sysroot or target support only after a scoped ADR and updated native threat analysis. | Verified-AETH-only entry, closed target/toolchain policy, VM/native differential evidence, fail-closed tool absence. |
| 6 | Readiness proof | Demonstrate maintained multi-package applications, upgrade/compatibility policy, operational documentation, and repeatable local releases. | Full release gate, package consumer verification, security review, and an evidence-backed claim update. |

## Active BARP sequence

BARP is the first active program because it reduces residual bootstrap authority
without changing guest language or host capability. The current direct seed
SPEAK pilot is deliberately bounded; it has no full diagnostic parity claim.

1. **Completed (ADR-116):** the `AE-SEED-011` zero-argument direct-call witness
   covers `bind … <- call name` and root `yield call name` only when `name` is
   the exact end-of-line target in a canonical ordinary `Whole` weave; the full
   release gate, four-way seed identity, and packaged consumer verification pass.
2. **Completed (ADR-117):** the `AE-SEED-011` witness covers canonical root
   `revise name <- call target` forms with the existing opaque target scan and
   exact next-space/end-of-line tail boundary; full release, seed identity, and
   packaged consumer verification pass.
3. **Completed (ADR-118):** the `AE-SEED-011` witness covers
   canonical root `speak call target` forms with the same opaque target scan and
   exact next-space/end-of-line tail boundary; it still leaves Text-result and
   call legality to the full compiler; full release, seed identity, and packaged
   consumer verification pass.
4. **Completed (ADR-119):** the `AE-SEED-011` witness covers canonical root
   `handle call target ... into success otherwise error into code` forms only
   when both fixed delimiters leave a destination suffix. It leaves M4 effects,
   result, arguments, destinations, terminality, and general call legality to
   the full compiler; full release, seed identity, and packaged consumer
   verification pass.
5. **Completed (ADR-120):** the
   `AE-SEED-011` witness covers canonical root `forward call target` only under
   a literal `-> Whole raises Whole:` caller header marker. It leaves full
   header parsing, terminality, target effects/results, arguments, resources,
   and general M4 call legality to the full compiler; full release, seed
   identity, and packaged consumer verification pass.
6. **Completed (ADR-121):** the
   `AE-SEED-011` witness covers only an ordinary total-Whole root `together:`
   with an immediate four-space zero-argument
   `spawn call target into destination` child. It leaves M7 nesting, task
   identity, arguments, destination, effects/results, resource policy, and
   scheduling to the full compiler; full release, seed identity, and packaged
   consumer verification pass.
7. **Completed (ADR-122):** the `AE-SEED-011` witness adds only the
   immediate four-space `spawn call target digit into destination` child when
   `digit` is exactly one ASCII decimal Whole literal. The separately bounded
   positive-two-digit form is now ADR-123; signed, general, and multi-argument
   forms plus all M7 semantic legality otherwise remain with the full compiler;
   full release, seed identity, and
   packaged consumer verification pass.
8. **Completed (ADR-123):** the
   `AE-SEED-011` witness adds only the immediate four-space
   `spawn call target digits into destination` child when `digits` is exactly
   two ASCII decimal characters with a nonzero first digit (`10`–`99`). Signed,
   leading-zero, three-or-more-digit, general, and multi-argument forms plus
   all M7 semantic legality remain with the full compiler; full release, seed
   identity, and packaged consumer verification pass.
9. Select one broader `AE-SEED-011` or `AE-SEED-013` shape only after
   documenting its false-positive boundary and valid-source corpus.
10. Design seed-native multi-file elaboration as a separate forge ABI decision;
   host elaboration plus seed emission remains the product contract until that
   design is proven.
11. Never remove the dual-compare oracle or convert a seed line-state witness
   into an unbounded parser/type checker by accretion.

## Fork boundaries

| Authorized fork | Proven boundary now | Next work remains gated by |
|---|---|---|
| F-NATIVE | M35a–M35j: verified AETH to C/object/LLVM IR/LLVM object/native executable, closed target matrix, VM reference behavior. | A new ADR, native threat-model update, toolchain/supply-chain policy, and differential execution matrix before M35k+. |
| F-REGISTRY | M24a–M24i: offline cache/pins, explicit signed fetch, trust policy, and X.509-lite CA store. | A new ADR, registry threat-model update, explicit fetch-only policy, and hostile trust corpus before a resolver or standards-expansion vertical. |

## Readiness gate

Aether may be described as mainstream-ready for its stated domain only after
all of these are evidenced together: maintained multi-package reference
applications; stable project/package upgrade rules; practical editor/test/debug
loops; reproducible offline and native-optional releases; threat-model-reviewed
I/O/registry/native boundaries; compatibility and hostile artifact corpora; and
an independently repeatable full release gate. Until then, the accurate claim
is a verified, local-first technical preview with advancing maturity.

## Current references

- [Product status](PROGRESS_REPORT-FULL-PROJECT.md)
- [Near-term milestone ledger](ROADMAP.md)
- [BARP design](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [BARP validation matrix](BARP-VALIDATION-MATRIX.md)
- [Native authorization](HUMAN-AUTHORIZE-NATIVE.md) and [native design](DESIGN-LAW-FORK-F-NATIVE.md)
- [Registry authorization](HUMAN-AUTHORIZE-REGISTRY.md) and [registry design](DESIGN-LAW-FORK-F-REGISTRY.md)
- [Claims register](CORE_CLAIMS.md)
