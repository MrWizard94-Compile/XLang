# ADR-128: BARP — seed-native Whole-library bundle profile

**Status:** Accepted — implemented and release verified (2026-08-21)<br>
**Date:** 2026-08-21<br>
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`<br>
**Program:** [BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)<br>
**Depends on:** ADR-056, ADR-061, ADR-075, ADR-127<br>
**Design:** [SBP-001](DESIGN-SBP-001-SEED-NATIVE-WHOLE-BUNDLE-PROFILE.md)

## Context

The standard M11/M22 product path remains host graph elaboration followed by
seed emission. It is useful and proven, but its import resolution, export
visibility, namespace mangling, and source rewriting are host authority. The
single-text `compile [borrow source: Text] -> Bytes` forge ABI deliberately
does not give a guest compiler filesystem, callback, shell, network, or package
resolver authority.

ADR-056 therefore correctly records that *general* seed-native multi-module
elaboration is unavailable. That truth remains in force. A complete general
resolver is not an acceptable shortcut: it would enlarge the trusted seed and
its input surface without a bounded, executable proof.

## Decision

Add a second, explicit forge entry point to the checked-in seed compiler:

```aether
weave compile_bundle [borrow bundle: Text] -> Bytes:
```

The host transports exactly one bounded `aether.seed-bundle/v1` Text argument.
It does not parse, topologically sort, resolve imports, collect exports, mangle
names, or rewrite source on the production bundle path. The seed parses and
elaborates the profile, then invokes its existing `compile` weave on the
single-world generated Aether source.

`compile_bundle` is intentionally **not** a broadened `compile` contract. Raw
`import unit` in ordinary one-file source remains fail-closed with
`AE-SEED-012`; v1 JSON `aether.multi-source/v1` remains the host-elaborated
product route. This is a distinct capability-closed forge ABI and source
protocol.

## Seed Bundle Profile v1 scope

The profile is deliberately small but end-to-end useful:

1. Exactly two ASCII/LF units: one library followed by one entry unit.
2. The library path and entry path are safe lowercase ASCII `.ae` paths; the
   entry path is explicitly named and must be the second unit.
3. The library declares exactly one `export weave` with a total `Whole` result,
   no `main`, imports, calls, host/foreign/task/record/shape declarations,
   Text literals, or resource/effect/nursery forms (`arena`, `buffer`,
   `allocate`, `release`, `access`, `handle`, `raise`, `forward`, `spawn`,
   `together`).
4. The entry declares exactly one canonical import of that library, exactly one
   `weave main [] -> Whole:`, no additional weave declaration, and no Text
   literals, resource/effect/nursery forms, or task declaration after the
   import line.
5. Every executable call in the entry must target the imported exported weave.
   The seed rewrites it to the established M11 path-mangled name
   `m_<path-with-slashes-replaced>_<weave>`.
6. The bundle has scalar-count framing, a two-unit/32,768-scalar aggregate cap,
   a 16,384-scalar cap per unit, and a 33,280-scalar wire cap. Malformed frames
   and profile violations fail closed through one seed-SPEAK `AE-SEED-016`
   packet.

The profile accepts one or more ordinary calls to the imported helper and the
bounded pure-Whole subset of the existing Seed Profile inside the helper body.
It is sufficient to compile a real two-unit arithmetic library plus entry
without host linguistic elaboration.

## Invariants

| ID | Invariant |
| --- | --- |
| SBP-INV-001 | The production bundle route sends one bounded Text directly to the verified seed `compile_bundle` weave; no host module elaborator is called. |
| SBP-INV-002 | The seed owns frame parsing, world uniqueness, canonical import matching, export visibility, path mangling, call rewriting, and generated-source assembly. |
| SBP-INV-003 | General M11/M22 is unchanged: `seed_native_multi_module_elaboration() == false`; host elaboration remains its product route. |
| SBP-INV-004 | Ordinary `compile` retains its exact one-source semantics and raw-import rejection. |
| SBP-INV-005 | Malformed/untrusted bundle data is bounded, ASCII/LF constrained, length-framed, path-jail compatible, and fails before source assembly. |
| SBP-INV-006 | Seed output must verify before it is returned or written; seed and bootstrap-reference artifacts are byte-identical for the documented corpus. |
| SBP-INV-007 | The profile adds no guest filesystem, network, shell, package cache, host callback, or ambient authority. |

## Consequences

This establishes a genuine seed-native multi-unit **profile**, not a claim that
the whole existing M11/M22 module surface is seed-native. It creates an
evolutionary seam: later profiles may enlarge unit count, source surface, or
graph shape only after a separate ADR, threat-model review, validation matrix,
and byte-identity proof.

The trade-off is explicit authoring rigidity. Users who need arbitrary module
graphs, Text helpers, package imports, records, shapes, host declarations, or
general source layout continue to use the proven host-elaborated project path.

## Proof obligations

1. Verify the compiler artifact and enforce the named `compile_bundle` ABI.
2. Show a two-unit bundle seed-compiles, verifies, runs, and exactly equals the
   Rust bootstrap compilation of the established host elaboration reference.
3. Prove repeated call rewriting, malformed scalar framing, wrong entry/order,
   unsafe path, wrong import, private/unknown call, and prohibited source forms
   fail closed.
4. Prove normal source and `aether.multi-source/v1` retain their existing
   routes and behavior.
5. Rebuild and forge the seed to byte identity, run the release gate, and
   update current/historical claims in the same delivery.

---

*End of ADR-128.*
