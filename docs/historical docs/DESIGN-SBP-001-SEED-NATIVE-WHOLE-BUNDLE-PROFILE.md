# SBP-001: Seed-native Whole-library bundle profile

**Status:** Implemented and release verified under ADR-128<br>
**Date:** 2026-08-21<br>
**Authority:** [ADR-128](ADR-128-barp-seed-native-whole-library-bundle-profile.md)<br>
**Rule IDs:** `RND-INVAR-001`, `SEC-INPUT-001`, `ENG-REPRO-001`, `TEST-BEHAVIOR-001`

## Core claim

> A verified Aether seed compiler can receive one bounded Text bundle, elaborate
> one canonical library-to-entry import edge inside Aether code, and emit the
> same verified AETH artifact as the bootstrap-reference M11 elaboration — with
> no host source parsing or import resolution on the production route.

The seed remains capability-closed. The host's only bundle responsibility is
transport: it reads a caller-selected local file or caller-supplied Text and
invokes the verified compiler artifact.

## Wire format

All headers are ASCII and LF-delimited. `length` is the number of Unicode scalar
values in the immediately following raw source payload. A single LF separator
follows every payload, including the final one.

```text
aether.seed-bundle/v1
entry src/main.ae
unit lib/math.ae 72
world math
export weave double [n: Whole] -> Whole:
  yield product n 2

unit src/main.ae 134
world app
import unit "lib/math.ae" as math
weave main [] -> Whole:
  bind twice <- call math.double 21
  yield call math.double twice

```

The counts are exact for the checked-in
[`seed-bundle-whole.aeb`](../../examples/seed-bundle-whole.aeb) fixture; the
canonical encoder calculates scalar counts rather than trusting a byte length.
The frame never uses sentinel source delimiters, so valid source cannot escape a
unit by containing a marker-like line.

## Bounded grammar

```text
bundle      = magic LF entry unit library-source LF unit entry-source LF
magic       = "aether.seed-bundle/v1"
entry       = "entry " safe-path
unit        = "unit " safe-path " " scalar-count
safe-path   = lower-or-digit-or-underscore-or-slash* ".ae"
library     = world LF exported-whole-weave
entry-unit  = world LF import LF main-whole-weave
import      = "import unit \"" library-path "\" as " lowercase-identifier
```

The seed validates the precise profile rather than trusting the host encoder.
`safe-path` prohibits `..`, empty segments, backslash, colon, hyphen, uppercase,
and any dot other than the terminal `.ae`; this is what makes seed-side mangle
output exactly match the established M11 lowercase mangler.

## Seed elaboration algorithm

1. Reject a non-ASCII/LF frame, over-limit bundle, malformed headers, invalid
   count, missing separator, duplicate path, wrong entry path/order, or any
   source payload beyond its scalar cap.
2. Extract and compare the two distinct `world` names.
3. Validate the library header, extract the one exported helper name, and derive
   its canonical M11 mangled name from the library path.
4. Validate the entry's one import exactly matches the library path and extract
   its alias.
5. Strip `world`, import, and `export` syntax. Rewrite each real `call` token in
   the entry from `alias.helper` to the derived mangled name. Reject every other
   call target and an entry with no helper call; Text literals and effect/resource
   forms are excluded so the rewrite has no quote-state ambiguity.
6. Assemble `world <entry-world>` + mangled library weave + rewritten entry
   weave, call the existing seed `compile`, and return its Bytes.

Any profile failure returns a sentinel to `compile_bundle`, which SPEAKs one
structured `AE-SEED-016` packet and yields empty Bytes. The forge host then
independently rejects the non-artifact Bytes and retains the seed packet in the
product diagnostic.

## Authority and security boundary

`compile_bundle` has the same primitive-only host invocation boundary as
`compile`: one borrowed Text argument, no grants, no callback, and verified
Bytes before write. The seed does not open a path named in the bundle. The path
is source identity only; it cannot escape the caller-selected file because no
guest file operation exists.

Host-side encode/decode helpers are framing utilities and test or authoring
conveniences. `compile_product_seed_bundle` never calls them or the host M11
elaborator. Their behavior is intentionally not evidence of production compiler
authority.

## Resource and performance budget

| Limit | Value | Reason |
| --- | ---: | --- |
| Units | exactly 2 | Demonstrates a real import edge while keeping seed state bounded. |
| Per-unit source | 16,384 scalars | Fits a practical small helper and bounded text copying. |
| Bundle source total | 32,768 scalars | Caps worst-case work and intermediate Text pressure. |
| Bundle wire total | 33,280 scalars | Caps header plus payload transport before seed frame extraction. |
| Character set | ASCII/LF | Makes framing, header recognition, and mangle behavior deterministic. |
| Import edges | exactly 1 | Avoids unproven graph/topological-sort authority. |

The seed uses linear Text scans and bounded concatenation. This is not a claim
of optimal linker throughput; the profile's bounded input makes worst-case
allocation and scan cost reviewable on ordinary local hardware.

## Explicit non-goals

- General M11 import DAGs, cycles, or arbitrary unit counts.
- M22 cross-package imports or registry resolution.
- Text/Bytes helpers, records, shapes, host/foreign/task declarations,
  resource/effect/nursery forms, helper-to-helper calls, or non-Whole exported
  results in the library. The seed explicitly rejects the profile's resource,
  effect, and nursery keyword set before source assembly.
- Source formatting or bootstrap diagnostic parity for rejected input.
- Any filesystem, shell, network, model, native, or registry guest authority.

## Test strategy

The validation matrix requires framing/property negatives, ordinary-source and
v1-envelope route regression, named forge ABI enforcement, seed-to-bootstrap
artifact identity, verified VM execution, seed self-rebuild identity, and the
full release gate. The executable source of truth is
[SBP-VALIDATION-MATRIX.md](../Current%20state/SBP-VALIDATION-MATRIX.md).

---

*End of SBP-001.*
