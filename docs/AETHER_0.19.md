# Aether 0.19 Toolchain Contract (M14 host I/O)

**Status:** Current package contract — language surface remains **0.11** / AETH **v11**; package **0.19** adds capability-mediated host I/O  
**Depends on:** [AETHER_0.11.md](AETHER_0.11.md) (M8 pure host), [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md), [ADR-018](ADR-018-m14-host-io-capabilities.md)

## Purpose

Package 0.19 extends the M8 product host catalog with **grant-backed**
read/write/env services. Default `aether run` without grants remains pure-fixture
only (`whole_inc`, `text_extent`). Operator-selected grants install I/O services
for that run session only.

## Product host catalog (M14)

| Name | Signature | Grant | Behavior |
| --- | --- | --- | --- |
| `whole_inc` | `[Whole] -> Whole` | none | pure (M8) |
| `text_extent` | `[borrow Text] -> Whole` | none | pure (M8) |
| `read_text` | `[borrow path: Text] -> Text` | `--grant-read <dir>` | UTF-8 under root; max 1_000_000 bytes |
| `read_bytes` | `[borrow path: Text] -> Bytes` | `--grant-read <dir>` | raw bytes under root; max 1_000_000 bytes |
| `write_text` | `[borrow path: Text, borrow body: Text] -> Whole` | `--grant-write <dir>` | write UTF-8; return length |
| `write_bytes` | `[borrow path: Text, borrow body: Bytes] -> Whole` | `--grant-write <dir>` | write bytes; return length |
| `env_get` | `[borrow name: Text] -> Text` | `--grant-env <NAME>` | value or fail closed |

## CLI

```text
aether run <artifact.aeth> \
  [--grant-read <dir>]... \
  [--grant-write <dir>]... \
  [--grant-env <NAME>]...
```

Grant roots must exist. Guest paths are relative, `/`-separated, no `..`, no
absolute/drive forms; after join+canonicalize they must stay under a grant root.

## Diagnostics

| Code | Meaning |
| --- | --- |
| `AE-HOST-003` | Missing service/grant or I/O failure (fail closed) |
| `AE-HOST-004` | Illegal guest path or path jail escape |
| `AE-HOST-005` | Size / name length safety limit exceeded |

## Core API

- `HostGrantConfig { read_roots, write_roots, env_names }`
- `run_bytecode_with_grants` / `invoke_bytecode_with_grants`

## Explicit non-goals

Network, shell, process spawn, directory listing, ambient cwd grants, FFI,
capability values as language types.

## Evidence

- Design: [DESIGN-M14-HOST-IO-CAPABILITIES.md](DESIGN-M14-HOST-IO-CAPABILITIES.md)
- Matrix: [M14-VALIDATION-MATRIX.md](M14-VALIDATION-MATRIX.md)
- Examples: `examples/host-io-read.ae`, `examples/host-io-write.ae`
- Threat: [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md)

*End of AETHER_0.19.md*
