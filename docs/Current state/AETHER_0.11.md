# Aether 0.11 Language Contract

**Status:** Historical executable product contract — M8 host ABI pilot; current
package contract is [AETHER_0.35.md](../historical%20docs/AETHER_0.35.md)
(capability-closed)

**Artifact output:** AETH v11

**Compatibility input:** verified AETH v4 through v11 artifacts

## Purpose

Aether 0.11 preserves Aether 0.10 and adds a **narrow, typed, ownership-aware
host boundary** for pure host services. Guests gain no file, process, network,
shell, or model authority. There is no C-header parser, libloading, or ambient
I/O host catalog.

## New M8 surface

```aether
host weave whole_inc [value: Whole] -> Whole
host weave text_extent [borrow message: Text] -> Whole

weave main [] -> Whole:
  bind after <- call whole_inc 41
  bind message <- "Aether"
  bind len <- call text_extent borrow message
  yield sum after len
```

Rules:

- `host weave` declares a total external signature with **no body** and no
  `raises`
- Host parameters: owned/copy `Whole`/`Truth`, or `borrow Text`/`borrow Bytes`
- Host results: `Whole` | `Truth` | `Text` | `Bytes` only
- Ordinary `call` of a host weave emits `HOST_CALL` (opcode 65), not `CALL`
- Host services succeed only when the host session installed a matching pure
  service; otherwise fail closed (`AE-HOST-003`)
- Product pure fixture installs only `whole_inc` and `text_extent`

Diagnostics: `AE-HOST-001` (form), `AE-HOST-002` (signature/call),
`AE-HOST-003` (missing/denied service).

## AETH v11

Same header layout as v10 for arena, records, and shapes. Function table entries
include a kind byte after the effect byte:

| Kind | Meaning |
| --- | --- |
| `0` guest | Ordinary weave with guest bytecode |
| `1` host | External signature; `code_len` must be 0 |

| Opcode | Role |
| --- | --- |
| `HOST_CALL` (65) | Invoke host weave (`function:u16`, `argc:u8`) |

`HOST_CALL` is valid only in v11. v4–v10 remain immutable.

## Authoring v6

`aether.ast/v6`, `aether.edit/v6`, `aether.diagnostic/v6` expose `HostWeave`
nodes and `hostWeaves` on the program document. v5 remains historical.

## Explicit non-goals (M8 document)

C ABI, dynamic libraries, header ingestion, async host, ambient guest
file/network I/O, mutable host memory views, and erroring host weaves. Package
**0.19 / M14** later adds **grant-backed** read/write/env host weaves under
threat model v2 — see [AETHER_0.19.md](../historical%20docs/AETHER_0.19.md). Shell, network, FFI, and
ambient FS remain non-goals without a further ADR.
See [DESIGN-M8-HOST-ABI-PILOT.md](../historical%20docs/DESIGN-M8-HOST-ABI-PILOT.md).
