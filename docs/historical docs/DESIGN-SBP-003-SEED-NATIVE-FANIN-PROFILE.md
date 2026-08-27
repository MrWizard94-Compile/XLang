# SBP-003: Seed-native bounded fan-in profile

**Status:** Implemented — release verified<br>
**Date:** 2026-08-26<br>
**Authority:** [ADR-130](ADR-130-barp-seed-native-fanin-bundle-profile.md)<br>
**Rule IDs:** `RND-INVAR-001`, `SEC-INPUT-001`, `ENG-REPRO-001`, `TEST-BEHAVIOR-001`

## Core claim

> A verified Aether seed compiler can accept one bounded Text bundle containing
> two independent pure `Whole` leaf libraries, one merge library with exactly
> two imports, and one main entry; it can elaborate those three fixed import
> edges inside Aether source and emit the same verified AETH artifact as the
> established Rust M11/bootstrap reference—without host source parsing or
> import resolution on the production route.

SBP-003 is the first seed-native profile with a real, bounded two-import merge.
It proves one fan-in shape, not a general DAG, package resolver, or module
system. The product host still transports the caller-selected bundle as opaque
Text and gives the seed no filesystem or other host capability.

## Wire format

Every header is ASCII and LF-delimited. `scalar-count` is the number of Unicode
scalar values in the raw payload immediately following the header. Every
payload, including the final entry, is followed by exactly one LF frame
separator.

```text
aether.seed-bundle/v3
entry src/main.ae
unit lib/base.ae 70
world base
export weave increment [n: Whole] -> Whole:
  yield sum n 1
unit lib/scale.ae 72
world scale
export weave double [n: Whole] -> Whole:
  yield product n 2
unit lib/combine.ae 212
world combine
import unit "lib/base.ae" as base
import unit "lib/scale.ae" as scale
export weave double_after_increment [n: Whole] -> Whole:
  bind raised <- call base.increment n
  yield call scale.double raised
unit src/main.ae 144
world app
import unit "lib/combine.ae" as combine
weave main [] -> Whole:
  bind result <- call combine.double_after_increment 41
  yield result
```

The checked-in [`seed-bundle-fanin.aeb`](../../examples/seed-bundle-fanin.aeb)
fixture has exactly those scalar counts. The authoring encoder counts Unicode
scalars; it does not parse or elaborate source. The production path does not
invoke that encoder or its decoder.

## Closed grammar

```text
bundle       = magic LF entry framed-unit framed-unit framed-unit framed-unit
magic        = "aether.seed-bundle/v3"
entry        = "entry " safe-path LF
framed-unit  = "unit " safe-path " " scalar-count LF source LF
safe-path    = lowercase-ascii-path ".ae"
left-leaf    = world LF exported-whole-weave
right-leaf   = world LF exported-whole-weave
merge        = world LF exact-left-import LF exact-right-import LF exported-whole-weave
entry-unit   = world LF exact-merge-import LF main-whole-weave
```

The fixed wire order is `left leaf -> right leaf -> merge -> entry`. The merge
must import the two preceding paths in that same order with two distinct safe
aliases. It exports one total `Whole` helper and may call only those two imported
helpers. The entry imports only the merge path and may call only the merge
helper. All paths and `world` identities are pairwise distinct.

## Seed elaboration algorithm

1. Reject malformed/non-ASCII framing, zero or oversized payloads, unsafe or
   duplicate paths, wrong entry/order, trailing data, and cap violations before
   source assembly.
2. Extract four source payloads; validate ASCII/LF-only text and four distinct
   `world` declarations.
3. Validate both leaves as one exported pure `Whole` helper with no import or
   call. Derive their established M11-compatible mangled names.
4. Validate the merge's two exact imports, distinct aliases, one exported pure
   `Whole` helper, and an exact two-target call boundary. Rewrite each leaf call
   to its own mangled name.
5. Validate the entry's one exact merge import and canonical `main`, then
   rewrite its calls to the mangled merge helper.
6. Assemble a single-world program in dependency order and invoke the existing
   seed `compile` weave. Any profile failure returns the sentinel consumed by
   `compile_bundle`, which emits one `AE-SEED-016` seed-SPEAK packet and empty
   Bytes; the forge host independently refuses non-AETH output.

`bundle_rewrite_two_calls` is deliberately exact: every merge `call` must name
one of the two imported helpers, and both helpers must be called at least once.
This makes lexical rewriting reviewable without claiming a general name
resolver. The established `bundle_pure_whole_source` guard retains the bounded
resource/effect/nursery exclusion boundary.

## Limits and performance reasoning

| Limit | Value | Reason |
| --- | ---: | --- |
| Units | exactly 4 | Proves one two-import merge without an unbounded list or graph algorithm. |
| Import edges | exactly 3 | Two fixed leaf-to-merge edges and one merge-to-entry edge. |
| Per-unit source | 16,384 scalars | Bounds each scan and temporary Text allocation. |
| Aggregate source | 65,536 scalars | Bounds all source extraction and generated-source work. |
| Complete wire input | 66,560 scalars | Bounds headers and opaque transport before seed extraction. |
| Character set | ASCII plus LF only | Makes framing, headers, and call-rewrite boundaries deterministic. |

The implementation uses bounded linear scans and bounded Text assembly. These
are resource and predictability limits, not a claim of linker throughput or a
replacement for the normal M11/M22 product path.

## Explicit non-goals

- General M11/M22 graphs, arbitrary unit counts, cycles, alternate edge order,
  package imports, registry resolution, or a topological-sort algorithm.
- More than one helper/export per library, more than two merge imports,
  arbitrary merge-local helpers, leaf calls, or a different fan-in shape.
- `Text`/`Bytes`, records, shapes, host/foreign/task declarations, effects,
  resources, nurseries, source formatting, or bootstrap-quality diagnostics for
  rejected bundles.
- Filesystem, process, shell, network, model, native, cache, registry, grant,
  callback, or ambient guest authority.

## Test strategy

The executable acceptance matrix is
[SBP-VALIDATION-MATRIX.md](../Current%20state/SBP-VALIDATION-MATRIX.md). It
requires source-opaque scalar framing tests; default-product and named-forge
execution; M11/bootstrap byte identity; hostile frame, order, world, import,
alias, call, and resource cases; v1/v2/general-M11/M22 regression coverage;
four-way seed identity; and isolated package-consumer verification. The exact
input/authority boundary is recorded in
[THREAT_MODEL-SBP-003-SEED-FANIN.md](../Current%20state/THREAT_MODEL-SBP-003-SEED-FANIN.md).

---

*End of SBP-003.*
