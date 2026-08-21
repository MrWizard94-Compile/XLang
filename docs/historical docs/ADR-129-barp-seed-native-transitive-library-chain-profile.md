# ADR-129: BARP — seed-native transitive library-chain profile

**Status:** Accepted — implemented and release verified (2026-08-21)<br>
**Date:** 2026-08-21<br>
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`<br>
**Program:** [BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)<br>
**Depends on:** ADR-056, ADR-075, ADR-127, ADR-128<br>
**Design:** [SBP-002](DESIGN-SBP-002-SEED-NATIVE-TRANSITIVE-CHAIN-PROFILE.md)

## Context

ADR-128 proved one useful seed-owned import edge: an exact pure `Whole`
library followed by one entry unit. It deliberately did not establish any
transitive dependency behavior. The normal M11/M22 path still lets the host
parse, resolve, sort, inspect exports, mangle names, and rewrite a general
module graph before the seed emits bytecode.

The next authority-reduction step must be materially more capable while still
being reviewable as a closed input protocol. Generalizing the v1 bundle into an
arbitrary module list, resolver, package loader, or graph algorithm would make
an unproven trust-boundary jump. It would also be incompatible with the
existing honest tracker:

```text
seed_native_multi_module_elaboration() == false
```

## Decision

Retain the existing, capability-closed forge entry:

```aether
weave compile_bundle [borrow bundle: Text] -> Bytes:
```

Add the distinct `aether.seed-bundle/v2` protocol. It accepts exactly three
ASCII/LF, scalar-framed source units in this wire order:

```text
foundation library -> bridge library -> main entry
```

The foundation has one exported pure `Whole` helper and no imports or calls.
The bridge has one exact import of the foundation, one exported pure `Whole`
helper, and calls only that imported foundation helper. The entry has one exact
import of the bridge, one `weave main [] -> Whole:`, and calls only that
imported bridge helper. All paths and worlds are distinct; the entry path names
the final unit. The seed validates those conditions, derives established
M11-compatible mangled helper names, rewrites both qualified call boundaries,
then compiles its one generated source with its existing `compile` weave.

The product and `forge-bundle` routes transport the original caller-selected
Text to the verified seed. Neither route decodes source, invokes the M11
elaborator, resolves an import, or reads a path embedded in the bundle. The
Rust framing helpers are authoring and independent-test utilities only.

## Bounded profile

| Dimension | Bound |
| --- | ---: |
| Units | exactly 3 |
| Dependency edges | exactly 2, foundation -> bridge -> entry |
| Per-unit source | 16,384 Unicode scalars |
| Aggregate source | 49,152 Unicode scalars |
| Complete wire input | 49,920 Unicode scalars |
| Source character set | ASCII plus LF only |
| Guest authority | none beyond borrowed `Text` and returned verified `Bytes` |

Every profile failure yields the existing coarse, structured
`AE-SEED-016` seed-SPEAK packet and empty Bytes. The forge host independently
verifies returned AETH before product return, CLI write, or VM execution.

## Invariants

| ID | Invariant |
| --- | --- |
| SBP2-INV-001 | v2 uses the existing exact `compile_bundle` ABI; it neither relaxes ordinary `compile` nor adds a host callback or guest capability. |
| SBP2-INV-002 | The production route transports opaque bundle Text directly to the verified seed and never calls a host decoder or M11/M22 elaborator. |
| SBP2-INV-003 | Seed validation owns frame parsing, path identity, dependency order, worlds, imports, export visibility, mangling, call rewriting, and generated-source assembly. |
| SBP2-INV-004 | The profile is exactly foundation -> bridge -> entry. Arbitrary unit counts, graph shapes, cycles, package imports, and resolver behavior remain outside it. |
| SBP2-INV-005 | The canonical v2 fixture must be byte-identical to independent Rust M11 elaboration plus bootstrap emission, verify, and run with its documented result. |
| SBP2-INV-006 | General M11/M22 remains host elaborate + seed emit; `seed_native_multi_module_elaboration() == false` remains the public truth. |
| SBP2-INV-007 | No filesystem, process, shell, network, registry, native, model, cache, grant, or callback authority is added to seed code. |

## Consequences

This is a real transitive source-library profile: one library helper may be
called through another library helper before entry execution. It therefore
removes a specific additional host elaboration step from the documented corpus.
It remains intentionally rigid. Users needing more helpers, a fan-out/fan-in
graph, records, Text/Bytes, effects, tasks, resources, packages, source
formatting, or detailed multi-file diagnostics must use the established
host-elaborated product path.

Versioned v1 remains supported and unchanged. v2 does not widen v1's accepted
surface, alter Aether language syntax, change AETH encoding/versioning, alter
VM behavior, modify project/workspace schemas, or broaden host grants.

## Proof obligations

1. Rebuild the checked-in Aether seed from source and retain seed bootstrap,
   product, forge, and checked-in artifact identity.
2. Prove the canonical three-unit fixture compiles through direct product and
   external named-forge routes, verifies, executes with exit 84, and is
   byte-identical to independent M11/bootstrap output.
3. Prove fixed count/order, scalar framing, safe paths, source size, distinct
   worlds, exact imports, private/unknown calls, and forbidden resource forms
   fail closed with `AE-SEED-016`.
4. Keep ADR-128 v1 and raw/import-envelope/general M11/M22 route regressions
   green, including the false general seed-native tracker.
5. Extend the release gate and isolated preview consumer proof so the shipped
   v2 fixture exercises product compilation and external `forge-bundle`.
6. Complete Constitution Section 0 review, documentation synchronization, pack
   verification, warning-denied analysis, full tests, and release packaging.

---

*End of ADR-129.*
