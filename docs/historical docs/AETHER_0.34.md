# Aether 0.34 Toolchain Contract (RTP-001 runtime Text fast path)

**Status:** Historical package contract — language **0.11** source forms and
AETH **v11** unchanged; current package contract is [AETHER_0.35.md](AETHER_0.35.md)
**Classification:** Runtime-performance increment, not a language-surface milestone
**Decision:** [ADR-040](ADR-040-runtime-text-ascii-fast-path.md)
**Validation:** [RTP-001 validation matrix](RTP-001-VALIDATION-MATRIX.md)

## What changed

Package 0.34 replaces the VM's internal `String` text payload with a private
`RuntimeText` value that stores the string and one cached fact: whether its
contents are ASCII. This is an implementation detail; it is neither syntax nor
a serialized AETH field.

For ASCII text, UTF-8 byte positions are exactly Unicode scalar positions. The
VM therefore performs `measure`, `glyph`, `cut`, and `seek` using direct byte
position logic rather than repeatedly traversing scalar iterators. The checked-in
seed compiler source is ASCII and heavily uses those primitives while compiling,
which makes the improvement material to self-hosting.

For non-ASCII text, the VM retains scalar-aware traversal. `measure`, `glyph`,
`cut`, and `seek` still use Unicode scalar positions; byte-oriented operations
(`extent`, `octet`, `slice`, `unpack*`, and `poke*`) retain their existing byte
contracts.

## Semantics and invariants

1. A text value is created with its ASCII fact at every VM boundary: artifact
   text constants, invocation values, host text reads/environment values,
   `decode`, and non-text `render` results.
2. `join` marks a result ASCII only when both operands are ASCII.
3. An ASCII `cut` result stays ASCII. A slice of non-ASCII text may conservatively
   retain the non-ASCII fallback even when that individual slice happens to be
   ASCII; this costs no semantic precision.
4. Runtime equality, host conversion, output, size accounting, and text limits
   continue to use the underlying UTF-8 string value.
5. AETH v4–v11 acceptance, new-emission v11, verifier behavior, the forge ABI,
   seed artifact bytes, source grammar, authoring protocol versions, and guest
   capabilities are unchanged.

The cache is sound because all Aether runtime text is valid Rust `String` UTF-8.
For ASCII UTF-8, every byte is one Unicode scalar, so byte indexing preserves
the language's scalar-index contract exactly.

## Evidence and measured scope

The reproducible local harness is:

```powershell
pwsh -NoProfile -File .\tools\measure-seed-self-host.ps1 `
  -Output target\seed-self-host-performance.json
```

It builds the release integration-test binary once, excludes that build time,
and directly runs only
`seed_profile_compiler_rebuilds_itself_and_a_distinct_valid_variant` three
times. On the recorded Windows host (Rust 1.96.0, 8 logical processors), the
package-0.34 samples were **10,469 ms**, **10,751 ms**, and **10,442 ms**
(median **10,469 ms**). The immediately preceding same-host, same-test
package-0.33 baseline samples were 129,533 ms, 128,680 ms, and 129,198 ms
(median 129,198 ms).

This is local engineering evidence for this exact self-host workload. It is not
a cross-machine, cross-language, general-VM, Unicode-text, or public competitive
performance claim. Build time is intentionally excluded, and the raw report is
written under ignored `target/` rather than treated as a source artifact.

## Validation

- `runtime_text_preserves_ascii_fast_and_unicode_scalar_offsets` exercises both
  the direct ASCII path and Unicode scalar fallback at the internal boundary.
- `runs_unicode_text_primitives_on_scalar_boundaries` preserves observable
  `cut`, `measure`, `glyph`, and `seek` behavior for `Aé🙂Z` and invalid UTF-8
  artifact rejection.
- Full debug and release workspace suites preserve verifier, seed-host,
  bootstrap≡seed, host, authoring, project, workspace, and CLI behavior.

## Non-goals

RTP-001 does not add a JIT, native backend, compiler cache, runtime ABI,
artifact-format revision, source feature, Unicode indexing shortcut, or broad
throughput guarantee. Unicode text still uses scalar traversal, and any wider
runtime optimization requires its own measured, behavior-preserving increment.

`RTP-001` is intentionally not named “M24”: the historical mainstream roadmap
already reserves that non-authoritative label for an optional registry track.

*End of AETHER_0.34.md*
