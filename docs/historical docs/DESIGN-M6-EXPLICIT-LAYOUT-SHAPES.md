# M6 Design: explicit layout shapes and dual-layout tables

**Status:** Accepted implementation design for Aether 0.9 / M6

**Date:** 2026-08-03

**Decision record:** [ADR-009](ADR-009-m6-explicit-layout-shapes.md)

**Validation record:** [M6 validation matrix](M6-VALIDATION-MATRIX.md)

## Purpose and boundary

M6 proves that Aether can admit **author-visible data layout** for one
collection form without automatic layout rewriting, unsound type erasure, or a
host ABI leak. It also admits a **constrained same-shape analysis** strong
enough to keep the two layouts observationally equivalent and to reject
cross-shape misuse.

Research inputs (accessed 2026-08-03):

- Odin's first-class structure-of-arrays posture
  ([overview](https://odin-lang.org/docs/overview/))
- Aether's existing closed M2 arena/Buffer model ([ADR-004](ADR-004-aeth-v6-bounded-resources.md))
- The falsifiable SoA/shape spike in
  [research/03-synthesis-and-evidence.md](research/03-synthesis-and-evidence.md)

M6 is intentionally not a generic type system, not an AoS→SoA optimizer, and
not a C-layout ABI.

## Core claim and invariants

| ID | Invariant |
| --- | --- |
| M6-INV-001 | A `shape` names a fixed, non-recursive product of only `Whole` fields. Field order in source is layout order. |
| M6-INV-002 | A `table Shape layout rows\|columns` is an explicit-layout collection owner, not an inferred rewrite of another aggregate. |
| M6-INV-003 | `rows` and `columns` store the same logical cells; only physical order differs. Put/get sequences with the same logical indices and fields yield the same `Whole` results. |
| M6-INV-004 | Table allocation is capability-oriented: one arena in `main`, closed `allocate` outcomes, no ambient heap. Capacity is a positive Whole in `1..=1024`. |
| M6-INV-005 | Shape analysis is local and total for the M6 surface: every table names a declared shape; field names must belong to that shape; two tables share a shape only when they reference the same shape declaration. |
| M6-INV-006 | Tables are M2-class resource owners: no weave results, no host/forge ABI crossing, no `revise` of the owner, no live owner across M4 effect control. |
| M6-INV-007 | AETH v9 records shape metadata and a layout tag so artifacts remain auditable; v4–v8 keep immutable meanings. |
| M6-INV-008 | Seed and bootstrap emit byte-identical artifacts for the documented M6 corpus before default product compile claims the feature. |
| M6-INV-009 | A published methodology measures a scoped workload on both layouts; claims of benefit are numbers from that harness, not optimizer marketing. |

## Source surface

```text
shape-decl     ::= "shape" name ":" newline field-line+
field-line     ::= indent name "Whole"
table-expr     ::= "table" shape-name "layout" ("rows" | "columns")
allocate-table ::= "choose" "allocate" "access" arena-name "move" table-name capacity
                   "into" table-name ":" ...
store-cell     ::= "choose" "store" "move" table-name index field-name value
                   "into" table-name ":" ...
load-cell      ::= "choose" "load" "borrow" table-name index field-name
                   "into" whole-dest ":" ...
```

Example:

```aether
world layout_demo

shape particle:
  mass Whole
  charge Whole

weave main [] -> Whole:
  bind memory <- arena 4096
  bind mutable parts <- table particle layout columns
  bind mutable sample <- 0
  choose allocate access memory move parts 2 into parts:
    choose store move parts 0 mass 10 into parts:
      choose store move parts 0 charge 3 into parts:
        choose load borrow parts 0 mass into sample:
          yield sample
        otherwise:
          yield -4
      otherwise:
        yield -3
    otherwise:
      yield -2
  otherwise:
    yield -1
```

### Limits

| Rule | Bound |
| --- | --- |
| Fields per shape | 1 through 8, all `Whole` |
| Shapes per program | at most 64 |
| Table capacity | 1 through 1,024 elements |
| Arena use | tables reserve `capacity * field_count * 8` logical bytes plus fixed metadata, inside the existing arena budget |
| Layout vocabulary | exactly `rows` (AoS: element-major) or `columns` (SoA: field-major) |

Rejected in M6: nested shapes, `Text`/`Truth`/`Bytes`/record fields, dynamic
field sets, automatic layout conversion, generic table parameters, table weave
results, host packing ABI, and user-adjustable layout policies beyond the two
keywords.

## Physical layout contract

For capacity `N` and field count `F`, storage is `N * F` signed 64-bit cells.

| Layout | Cell address of element `i`, field `f` |
| --- | --- |
| `rows` | `i * F + f` |
| `columns` | `f * N + i` |

`store` writes one cell. `load` reads one cell. Out-of-range index or full
allocation failure uses the same closed dim-branch discipline as M2 Buffer
operations: owners are preserved; no panic; no ambient recovery.

## Shape analysis prototype

M6 shape analysis is a constrained static checker, not type inference:

1. Every `table S layout _` requires a prior `shape S`.
2. Every `store`/`load` field name must be a field of that table's shape.
3. Index and stored value must be `Whole`.
4. Two tables are **same-shape** only when their shape names resolve to the
   same declaration. M6 does not admit structural shape equality across
   different names.
5. No operation silently converts `rows` to `columns` or the reverse.

Semantic equivalence is an executable property: for any valid put/get script
that does not depend on physical addresses, `layout rows` and
`layout columns` programs that differ only in the layout keyword produce the
same sequence of loaded `Whole` values and the same terminal exit code.

## AETH v9

New compilation emits AETH v9:

```text
AETH | version=9:u8 | arena_capacity:u32-le |
record_count:u16-le | record table |
shape_count:u16-le | shape table |
function table
```

Each shape table entry stores a field count and ordered field name table. Table
opcodes carry shape id and layout tag. Opcodes (v9-only):

| Opcode | Role |
| --- | --- |
| `TABLE` (57) | Push unallocated table placeholder (`shape_id`, `layout`). |
| `TABLE_ALLOCATE` (58) | Closed allocate from arena into a moved table owner. |
| `TABLE_STORE` (59) | Closed store of one cell; restores owner. |
| `TABLE_LOAD` (60) | Closed load of one cell from a borrowed table. |
| `TABLE_COUNT` (61) | Push allocated capacity from a borrowed table. |

v4–v8 remain accepted with original meanings. A v9 opcode in an older artifact
is rejected before run or forge write.

## Performance methodology (scoped benefit)

M6 does not claim universal SoA superiority. It publishes one harness:

| Item | Definition |
| --- | --- |
| Workload | Allocate capacity 256; store `mass=i`, `charge=i*2` for each index; sum every `mass` then every `charge` via `load`. |
| Variants | Identical source except `layout columns` vs `layout rows`. |
| Metrics | (1) identical exit code / semantic sum; (2) wall-clock over repeated `run_bytecode` iterations in a Rust test; (3) optional instruction-path note in the delivery report. |
| Benefit claim | Valid only if the faster layout wins by a reproducible margin on this harness **or** the report honestly states no reliable wall-clock win on the interpreter while still proving semantic equivalence and explicit layout auditability. |

Stop condition from research: if a claimed benefit requires undocumented layout
mutation or unsound erasure, the claim is rejected even if microbenchmarks look
good.

## Seed and authoring

- Seed Profile extends to parse shapes, dual-layout tables, and the closed
  allocate/store/load surface, emitting v9 byte-identically for the corpus.
- Authoring advances to `aether.ast/v4`, `aether.edit/v4`, and
  `aether.diagnostic/v4` with explicit `Shape` and table layout nodes. v3 remains
  historical and is not silently reinterpreted.
- Diagnostics: `AE-LAYOUT-001` (illegal shape/table form), `AE-LAYOUT-002`
  (shape/field mismatch), `AE-LAYOUT-003` (capacity/arena bound).

## Falsification and stop conditions

Do not promote M6 into default product compile if:

1. layout is inferred or rewritten without an author keyword;
2. rows/columns diverge on any valid semantic script in the corpus;
3. tables escape through host, forge, or weave results;
4. seed cannot match bootstrap on the documented corpus;
5. performance claims omit the harness or depend on non-deterministic noise
   presented as certainty.

Future work still requires new decisions: generic shape parameters, non-Whole
fields, multi-arena tables, automatic specialization of arbitrary weaves, and
foreign ABI layout.
