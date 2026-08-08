# Aether 0.9 Language Contract

**Status:** Historical Aether 0.9 specification. Current product contract is
[AETHER_0.35.md](AETHER_0.35.md).

**Artifact output:** AETH v9

**Compatibility input:** verified AETH v4, v5, v6, v7, v8, and v9 artifacts

## Purpose

Aether 0.9 preserves Aether 0.8's scalar, byte, record, bounded-resource, error
effect, and literal `comptime bind` contracts, then adds author-visible data
layout for one table collection form. It is not a generic type system, an
automatic AoS→SoA optimizer, or a foreign ABI.

## New M6 surface

### Shape declarations

After `world` and before weaves (alongside records):

```text
shape <name>:
  <field> Whole
```

Rules: 1–8 fields, only `Whole`, non-recursive, unique field names, at most 64
shapes per program. Field order is layout order.

### Dual-layout tables

```text
table <shape> layout rows|columns
```

Tables are M2-class arena resource owners:

```aether
bind mutable parts <- table particle layout columns
choose allocate access memory move parts 2 into parts:
  choose store move parts 0 mass 10 into parts:
    choose load borrow parts 0 mass into sample:
      yield sample
    otherwise:
      yield -4
  otherwise:
    yield -3
otherwise:
  yield -1
```

| Rule | Bound |
| --- | --- |
| Capacity | 1 through 1,024 elements |
| Physical cells | `N * F` signed 64-bit words |
| `rows` address | `i * F + f` |
| `columns` address | `f * N + i` |
| Escape | No weave results, host ABI, or forge values |

Diagnostics: `AE-LAYOUT-001` (illegal form), `AE-LAYOUT-002` (shape/field
mismatch), `AE-LAYOUT-003` (capacity/arena bound).

## AETH v9

```text
AETH | version=9:u8 | arena_capacity:u32-le |
record_count:u16-le | record table |
shape_count:u16-le | shape table |
function table
```

| Opcode | Role |
| --- | --- |
| `TABLE` (57) | Unallocated table placeholder |
| `TABLE_ALLOCATE` (58) | Closed arena allocate |
| `TABLE_STORE` (59) | Closed cell store |
| `TABLE_LOAD` (60) | Closed cell load |
| `TABLE_COUNT` (61) | Allocated capacity |

## Authoring v4

`aether.ast/v4`, `aether.edit/v4`, and `aether.diagnostic/v4` expose `Shape`
nodes and table layout. v3 remains historical and is rejected rather than
silently upgraded.

## Explicit non-goals

Generic shape parameters, non-Whole fields, nested shapes, automatic layout
conversion, whole-program specialization, table weave results, C packing ABI,
broader comptime, concurrency, and native backends remain out of scope.

See [DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md)
and [ADR-009](ADR-009-m6-explicit-layout-shapes.md).
