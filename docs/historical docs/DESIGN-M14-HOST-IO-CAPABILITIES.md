# M14 Design: capability-mediated host I/O

**Status:** Accepted design for ADR-018 — **implemented in package 0.19.0**
**Date:** 2026-08-04
**Decision record:** [ADR-018](ADR-018-m14-host-io-capabilities.md)
**Threat model:** [THREAT_MODEL-v2-CAPABLE-HOST.md](../Current%20state/THREAT_MODEL-v2-CAPABLE-HOST.md)
**Validation:** [M14 validation matrix](M14-VALIDATION-MATRIX.md)
**Depends on:** M8 pure host ABI (`host weave` / `HOST_CALL`), threat model v2
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) **T-HOST**
**Rule IDs:** `SEC-INPUT-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`

---

## 1. Purpose and boundary

M8 proved pure host fixtures. Real CLI programs need **read/write/env** without
ambient OS authority.

**M14** adds a **small, grant-backed host service catalog** installed only when
the operator passes grant flags to `aether run` (or test host sessions).

Non-goals: network, shell, FFI, directory listing, ambient cwd grants.

---

## 2. Core claim

> With explicit operator grants, guest programs may call installed I/O host
> weaves using only relative paths under grant roots. Without grants or with
> path escape, calls fail closed. Pure fixtures remain available without grants.

---

## 3. Design decisions

### D1 — Source surface

Reuse M8 `host weave` declarations. Programs **declare** the services they need:

```aether
world io_demo

host weave read_text [borrow path: Text] -> Text
host weave write_text [borrow path: Text, borrow body: Text] -> Whole
host weave env_get [borrow name: Text] -> Text

weave main [] -> Whole:
  bind cfg <- call read_text borrow "config.txt"
  bind n <- call text_extent borrow cfg
  yield n
```

`whole_inc` / `text_extent` remain pure fixtures (no grant).

### D2 — Product host catalog (M14)

| Name | Signature | Grant required | Behavior |
| --- | --- | --- | --- |
| `whole_inc` | `[Whole] -> Whole` | none | pure (M8) |
| `text_extent` | `[borrow Text] -> Whole` | none | pure (M8) |
| `read_text` | `[borrow path: Text] -> Text` | `--grant-read <root>` | UTF-8 file under root; max size |
| `read_bytes` | `[borrow path: Text] -> Bytes` | `--grant-read <root>` | raw bytes under root; max size |
| `write_text` | `[borrow path: Text, borrow body: Text] -> Whole` | `--grant-write <root>` | write UTF-8; return byte length |
| `write_bytes` | `[borrow path: Text, borrow body: Bytes] -> Whole` | `--grant-write <root>` | write bytes; return length |
| `env_get` | `[borrow name: Text] -> Text` | `--grant-env <NAME>` per name | return value or fail closed if missing grant/var |

All I/O host weaves are **total** at the language level; failure is runtime host
fail-closed (same family as `AE-HOST-003`), not `raises Whole` in M14.

### D3 — Guest path grammar

Same spirit as project units:

- Non-empty relative
- `/` separators only
- No `..`, no absolute, no drive/URL
- Segments safe charset
- After join+canonicalize under root, path must stay under root

### D4 — CLI

```text
aether run <artifact.aeth> \
  [--grant-read <dir>]... \
  [--grant-write <dir>]... \
  [--grant-env <NAME>]...
```

Multiple roots allowed (union). Empty grants → only pure fixtures installed
(backward compatible with M8).

### D5 — Limits

| Limit | Value (pilot) |
| --- | --- |
| Max read/write bytes | 1_000_000 |
| Max path length | 4096 |
| Max env name length | 256 |

### D6 — AETH / seed

- No new opcodes if existing `HOST_CALL` + host catalog suffice.
- Seed must parse programs that **declare** new host weave names (already name-agnostic).
- Dual-compare: programs using only pure fixtures unchanged; I/O programs need host session in run tests (not seed emit difference).

### D7 — Failure codes

| Code | Meaning |
| --- | --- |
| `AE-HOST-003` | Missing service or grant / I/O failure fail-closed |
| `AE-HOST-004` | Guest path illegal or escapes grant root (new) |
| `AE-HOST-005` | Size limit exceeded (new) |

### D8 — Package pin

Suggested **0.19.0** at implementation ship.

### D9 — Stop conditions

- Ambient cwd without grant root
- Shell/network
- Guest absolute paths accepted
- Symlink escape
- Silent best-effort partial writes

---

## 4. Invariants

| ID | Invariant |
| --- | --- |
| M14-INV-001 | I/O host weaves require matching grants |
| M14-INV-002 | Guest paths relative + under root after canonicalize |
| M14-INV-003 | Pure fixtures work without grants |
| M14-INV-004 | Missing service/grant fails closed |
| M14-INV-005 | No network/shell weaves |
| M14-INV-006 | Size caps enforced |
| M14-INV-007 | Threat model v2 cited in MANIFEST when shipped |

---

## 5. Example corpus (implementation)

- `examples/host-io-read.ae` + fixture file under temp grant root
- Negative: `../escape`, absolute path, missing grant

---

## 6. Implementation plan

1. Host session grant table API in core
2. Implement catalog services
3. Wire CLI `run` flags
4. Tests matrix
5. DOC-SYNC + seed dual-compare for pure programs regression
6. Delivery report

---

## 7. Non-goals

Network, process, FFI, directory list, capability values as first-class Aether
types, multi-tenant hosts.

---

*End of DESIGN-M14-HOST-IO-CAPABILITIES.md*
