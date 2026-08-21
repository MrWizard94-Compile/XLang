# SBP-002: Seed-native transitive library-chain profile

**Status:** Implemented and release verified under ADR-129<br>
**Date:** 2026-08-21<br>
**Authority:** [ADR-129](ADR-129-barp-seed-native-transitive-library-chain-profile.md)<br>
**Rule IDs:** `RND-INVAR-001`, `SEC-INPUT-001`, `ENG-REPRO-001`, `TEST-BEHAVIOR-001`

## Core claim

> A verified Aether seed compiler can accept one bounded Text bundle containing
> a foundation library, a bridge library, and a main entry; elaborate the two
> exact import edges inside Aether source; and emit the same verified AETH as
> the established Rust M11/bootstrap reference, without host source parsing or
> import resolution on the production route.

This extends SBP-001's one edge without promoting a general graph resolver.
The host reads a caller-selected local bundle and transports it as opaque Text.
It neither opens a path named within the bundle nor gives the seed any host
authority.

## Wire format

Every header is ASCII and LF-delimited. `scalar-count` is the number of Unicode
scalar values in the immediately following raw source payload. A single LF
separator follows each payload, including the final entry payload.

```text
aether.seed-bundle/v2
entry src/main.ae
unit lib/base.ae 70
world base
export weave increment [n: Whole] -> Whole:
  yield sum n 1
unit lib/math.ae 165
world math
import unit "lib/base.ae" as base
export weave double_after_increment [n: Whole] -> Whole:
  bind raised <- call base.increment n
  yield product raised 2
unit src/main.ae 135
world app
import unit "lib/math.ae" as math
weave main [] -> Whole:
  bind result <- call math.double_after_increment 41
  yield result
```

The payloads in the checked-in
[`seed-bundle-chain.aeb`](../../examples/seed-bundle-chain.aeb) fixture do not
include terminal LFs; the three required separators are frame delimiters. The
canonical encoder counts Unicode scalars, not UTF-8 bytes. It does not inspect
or elaborate source.

## Closed grammar

```text
bundle       = magic LF entry framed-unit framed-unit framed-unit
magic        = "aether.seed-bundle/v2"
entry        = "entry " safe-path LF
framed-unit  = "unit " safe-path " " scalar-count LF source LF
safe-path    = lowercase-ascii-path ".ae"
foundation   = world LF exported-whole-weave
bridge       = world LF exact-foundation-import LF exported-whole-weave
entry-unit   = world LF exact-bridge-import LF main-whole-weave
```

The protocol fixes wire order rather than accepting arbitrary paths followed by
a topological sort. The path in `entry` must be the third frame. Source worlds
must be pairwise distinct. Path identity uses lowercase ASCII letters, digits,
underscores, forward slashes, and a terminal `.ae`; it is never a filesystem
capability.

## Seed elaboration algorithm

1. Reject non-ASCII/LF input, a malformed magic/header/count/separator, zero or
   oversized payload, repeated path, wrong unit count/order, or a wire/source
   cap breach before source assembly.
2. Extract the three unit payloads and validate safe identities plus three
   distinct `world` declarations.
3. Validate that the foundation has exactly one exported pure `Whole` helper,
   no imports, and no calls. Derive its M11-compatible mangled name.
4. Validate that the bridge imports exactly the foundation path under a safe
   alias, exports one pure `Whole` helper, and calls only the imported
   foundation helper. Derive its mangled name and rewrite those calls.
5. Validate that the entry imports exactly the bridge path under a safe alias,
   declares one canonical `main`, and calls only the exported bridge helper.
   Rewrite those calls.
6. Assemble `world <entry-world>`, the mangled foundation weave, the mangled
   rewritten bridge weave, and the rewritten main weave. Invoke the existing
   seed `compile` weave.

`bundle_pure_whole_source` supplies the narrow lexical resource/effect/nursery
exclusion boundary used by both profiles. It is intentionally a profile guard,
not a substitute for the full Aether parser outside this exact source shape.
The profile rejects a malformed or unsupported form instead of guessing its
meaning.

## Limits and performance reasoning

| Limit | Value | Reason |
| --- | ---: | --- |
| Units | exactly 3 | Proves two transitive edges without an unbounded collection or graph algorithm. |
| Per-unit source | 16,384 scalars | Keeps each source scan and temporary copy bounded. |
| Aggregate source | 49,152 scalars | Bounds total profile parsing and generated-source work. |
| Wire input | 49,920 scalars | Bounds headers and raw transport before seed extraction. |
| Character set | ASCII/LF | Keeps header scans, scalar framing, and source rewrite boundaries deterministic. |
| Calls | foundation: none; bridge/entry: imported helper only | Makes deterministic rewriting reviewable without generalized name resolution. |

The seed uses bounded linear scans and bounded Text assembly. This is a safety
and predictability budget, not a throughput benchmark or a claim that the
profile replaces a general linker.

## Explicit non-goals

- General M11/M22 import graphs, unit counts, cycles, package imports, or
  registry/resolver behavior.
- More than one helper/export/import per unit, local library calls, or a
  fan-out/fan-in dependency shape.
- Text/Bytes helpers, records, shapes, host/foreign/task declarations,
  effects, resources, nursery forms, source formatting, or bootstrap-quality
  diagnostics for rejected bundles.
- Any guest filesystem, process, shell, network, model, native, cache,
  registry, grant, or callback authority.

## Test strategy

The executable acceptance matrix is
[SBP-VALIDATION-MATRIX.md](../Current%20state/SBP-VALIDATION-MATRIX.md). It
requires independent scalar-frame tests, direct product and external named-forge
execution, M11/bootstrap byte identity, hostile frame/source-profile cases,
v1 and general M11/M22 route regressions, seed rebuild identity, and a packaged
consumer proof. The exact threat boundary is recorded in
[THREAT_MODEL-SBP-002-SEED-CHAIN.md](../Current%20state/THREAT_MODEL-SBP-002-SEED-CHAIN.md).

---

*End of SBP-002.*
