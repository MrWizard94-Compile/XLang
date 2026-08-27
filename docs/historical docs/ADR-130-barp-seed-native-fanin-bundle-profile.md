# ADR-130: BARP — seed-native bounded fan-in bundle profile

**Status:** Accepted — implemented; release verified<br>
**Date:** 2026-08-26<br>
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`<br>
**Program:** [BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)<br>
**Depends on:** ADR-056, ADR-075, ADR-127, ADR-128, ADR-129<br>
**Design:** [SBP-003](DESIGN-SBP-003-SEED-NATIVE-FANIN-PROFILE.md)

## Context

ADR-128 proved one seed-owned import edge. ADR-129 proved one fixed transitive
chain. Neither profile can represent a merge that consumes two independently
exported library helpers, so the production host remains authoritative for that
useful graph shape through normal M11 elaboration.

An arbitrary module list or graph resolver would be an unjustified authority
jump. It would force the seed to own discovery, sorting, cycles, export tables,
and a substantially broader diagnostic surface while invalidating the explicit
truth that general seed-native M11/M22 elaboration remains unproven.

## Decision

Retain the existing capability-closed forge entry:

```aether
weave compile_bundle [borrow bundle: Text] -> Bytes:
```

Add the distinct `aether.seed-bundle/v3` protocol. It accepts exactly four
ASCII/LF scalar-framed source units in this wire order:

```text
left pure Whole leaf -> right pure Whole leaf -> two-import merge -> main entry
```

Each leaf has one exported pure `Whole` helper and no imports or calls. The
merge imports the exact left path and right path under distinct safe aliases,
exports one pure `Whole` helper, and calls exactly those two imported helpers.
The entry imports only the merge, has canonical `main`, and calls only the
merge helper. The seed validates paths, worlds, import order, aliases, exports,
and calls; rewrites each qualified boundary to established M11-compatible
mangled names; assembles a single source; and invokes its existing `compile`
weave.

Product compilation and `forge-bundle` transport the original bundle Text
directly to the verified seed. The host does not decode sources, invoke the
M11 elaborator, resolve imports, open paths embedded in the bundle, or rewrite
qualified calls on this route. Rust framing helpers are authoring and
independent-test utilities only.

## Bounded profile

| Dimension | Bound |
| --- | ---: |
| Units | exactly 4 |
| Dependency edges | exactly 3 |
| Per-unit source | 16,384 Unicode scalars |
| Aggregate source | 65,536 Unicode scalars |
| Complete wire input | 66,560 Unicode scalars |
| Source character set | ASCII plus LF only |
| Guest authority | none beyond borrowed `Text` and returned verified `Bytes` |

Every profile failure yields the existing coarse structured `AE-SEED-016`
seed-SPEAK packet and empty Bytes. The forge host independently verifies AETH
before product return, CLI write, or VM execution.

## Invariants

| ID | Invariant |
| --- | --- |
| SBP3-INV-001 | v3 uses the established `compile_bundle` ABI; ordinary `compile` and all guest capability contracts are unchanged. |
| SBP3-INV-002 | The product route transports opaque caller-selected Text directly to the verified seed and never invokes a host decoder or M11/M22 elaborator. |
| SBP3-INV-003 | Seed validation owns framing, path/world uniqueness, fixed edge order, import/export visibility, alias checks, mangling, rewrites, and generated-source assembly. |
| SBP3-INV-004 | The profile is exactly left leaf -> right leaf -> merge -> entry; alternate graph shapes, unit counts, and resolver behavior remain outside it. |
| SBP3-INV-005 | The canonical v3 fixture must be byte-identical to independent Rust M11 elaboration plus bootstrap emission, verify, and run with exit 84. |
| SBP3-INV-006 | General M11/M22 remains host elaborate + seed emit; `seed_native_multi_module_elaboration() == false` remains the public truth. |
| SBP3-INV-007 | No filesystem, process, shell, network, registry, native, model, cache, grant, or callback authority is added to seed code. |

## Consequences

This is the first real seed-native merge profile: one pure helper consumes two
independent imported helper results before the entry executes. It removes three
specific host elaboration actions from the documented corpus. It remains rigid
by design. Users who need a different import shape, multiple exports, packages,
records, Text/Bytes, effects, tasks, resources, source formatting, or detailed
multi-file diagnostics continue to use the established host-elaborated path.

Versions v1 and v2 remain supported unchanged. v3 does not change Aether
syntax, AETH encoding/versioning, VM semantics, project/workspace schemas,
M25 packages, native/registry scope, or host grants.

## Proof obligations

1. Rebuild the checked-in seed from source and retain bootstrap, product, forge,
   and checked-in artifact identity.
2. Prove the canonical four-unit fixture compiles through direct product and
   external named-forge routes, verifies, executes with exit 84, and is
   byte-identical to independent M11/bootstrap output.
3. Prove fixed count/order, scalar framing, safe paths, source size, distinct
   worlds, exact two imports, alias collision, private/unknown calls, and
   forbidden resource forms fail closed with `AE-SEED-016`.
4. Keep ADR-128 v1, ADR-129 v2, raw/import-envelope/general M11/M22, and false
   general-seed-native tracker regressions green.
5. Extend the release gate and isolated preview consumer proof so the shipped
   v3 fixture exercises product compilation and external `forge-bundle`.
6. Complete Constitution Section 0 review, documentation synchronization, pack
   verification, warning-denied analysis, full tests, and release packaging.

---

*End of ADR-130.*
