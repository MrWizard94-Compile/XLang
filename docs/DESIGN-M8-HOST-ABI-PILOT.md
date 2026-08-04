# M8 Design: host ABI pilot (capability-closed)

**Status:** Accepted implementation design for Aether 0.11 / M8

**Date:** 2026-08-04

**Decision record:** [ADR-011](ADR-011-m8-host-abi-pilot.md)

**Validation record:** [M8 validation matrix](M8-VALIDATION-MATRIX.md)

## Purpose and boundary

M8 proves a **narrow, typed, ownership-aware host boundary** without granting
AETH guests file, process, network, shell, or model authority, and without a
C-header parser or native backend.

The existing forge ABI (`compile [borrow source: Text] -> Bytes`) remains the
compiler-host path. M8 adds an explicit **guest→host call** path for services
the host chooses to install. Services are pure fixtures in the product pilot;
I/O-bearing hosts would require a new ADR.

## Threat model (pilot)

| Actor | Trust | Authority |
| --- | --- | --- |
| Guest AETH artifact | Untrusted | None ambient; only declared host services the host installs |
| Host (CLI/library) | Trusted | May install pure host functions; owns all filesystem I/O |
| Forge | Trusted host | Unchanged; guest still has no host I/O |

**Denied in M8:** guest-initiated open/read/write of files, process spawn,
network, shell, model calls, dynamic library loading, C header ingestion,
pointer escapes into host memory, and ambient “whatever the OS allows.”

## Core claim and invariants

| ID | Invariant |
| --- | --- |
| M8-INV-001 | A `host weave` declares a total external signature with no Aether body. |
| M8-INV-002 | Host weaves accept only primitive copy `Whole`/`Truth` and `borrow Text`/`borrow Bytes` parameters; results are only `Whole`, `Truth`, `Text`, or `Bytes`. |
| M8-INV-003 | Calling a host weave emits `HOST_CALL` (AETH v11). It is not an ordinary internal call and cannot target guest weaves. |
| M8-INV-004 | A host call succeeds only if the host session installed a matching pure service; otherwise it fails closed with a deterministic error (no host panic, no capability expansion). |
| M8-INV-005 | Host arguments obey ownership: `borrow` does not transfer; host may not observe records, arenas, buffers, tables, access loans, or erroring weaves. |
| M8-INV-006 | Guest cannot return resources/records to the host invoke boundary (existing rule preserved). |
| M8-INV-007 | Product pure fixture services are deterministic and I/O-free. |
| M8-INV-008 | Seed emits host weaves/HOST_CALL byte-identically for the documented corpus. |
| M8-INV-009 | v4–v10 remain immutable; `HOST_CALL` is v11-only. |

## Source surface

```text
host-weave ::= "host weave" name "[" params "]" "->" result-type
params     ::= /* empty or comma-separated entries */
param      ::= ["borrow "] name ": " (Whole|Truth|Text|Bytes)
result     ::= Whole | Truth | Text | Bytes
```

Host weaves appear after `world` / records / shapes and before ordinary weaves.
They have **no body and no trailing colon block**.

```aether
world host_pilot

host weave whole_inc [value: Whole] -> Whole
host weave text_extent [borrow message: Text] -> Whole

weave main [] -> Whole:
  bind mutable n <- 41
  bind after <- call whole_inc n
  bind len <- call text_extent borrow "Aether"
  yield sum after len
```

Ordinary `call` of a host weave is legal and total. Host weaves cannot
`raises Whole`. `main` may call host weaves.

## Product pure fixture catalog

Installed by default for `aether run` and library invoke helpers used in tests:

| Host weave | ABI | Behavior |
| --- | --- | --- |
| `whole_inc` | `[value: Whole] -> Whole` | `value + 1` with checked overflow → closed failure |
| `text_extent` | `[borrow message: Text] -> Whole` | UTF-8 byte length of the text |

No other host names exist in the product fixture. A program that declares a
different host weave name fails closed at run/verify-against-fixture time.

## AETH v11

Header layout matches v10 (arena, records, shapes, functions). Function table
entries for host weaves use a **host effect/tag** or empty code with host flag:

- Function metadata: `kind: guest | host` (u8), host_id:u16 for host entries
- Opcode `HOST_CALL` (65): `host_id:u16`, `argc:u8` — pops args, pushes result

Verifier rules:

- `HOST_CALL` only in v11
- Target must be a host function entry
- Arg types match signature; stack discipline holds
- Host functions have zero guest bytecode length

## Failure modes

| Code | Meaning |
| --- | --- |
| `AE-HOST-001` | Illegal host weave form (body, effect, bad types, resources). |
| `AE-HOST-002` | Call/signature mismatch for a host weave. |
| `AE-HOST-003` | Host service missing or denied at runtime (fail closed). |

## Seed and authoring

- Seed parses `host weave` and emits v11 host function table + `HOST_CALL`.
- Authoring `aether.ast/v6` adds `HostWeave` nodes; edit/diagnostic v6.
- Docs: AETHER_0.11, MANIFEST, FORGE/HOST contract, CORE_CLAIMS.

## Explicit non-goals

C ABI, libloading, headers, async host, guest file I/O, mutable host memory
views, and erroring host weaves. Each needs a new threat model and ADR.

## Stop conditions

Do not ship if:

1. guest can reach file/process/network/shell without a new ADR;
2. host calls succeed without an installed service;
3. records/resources cross the host boundary;
4. seed cannot match bootstrap on the host corpus;
5. C-header or native backend work is smuggled in.
