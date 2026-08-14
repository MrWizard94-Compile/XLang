# Threat Model v2 — Capable Host (I/O grants)

**Status:** Accepted freeze for **design of M14 host I/O** (supersedes TP freeze *only for tracks that cite this document*)
**Date:** 2026-08-04
**Product pin at authoring:** package **0.18.0** / language **0.11+M11–M13** / AETH **v11**
**Supersedes for I/O tracks:** [THREAT_MODEL-TECHNICAL-PREVIEW.md](../historical%20docs/THREAT_MODEL-TECHNICAL-PREVIEW.md) §6 item 1–2 *when* M14 is implemented under ADR-018
**Does not supersede:** offline-first CLI, no model integration, no ambient guest shell/network unless a later ADR
**Rule IDs:** `SEC-INPUT-001`, `REL-HARDEN-001`, `CONST-GATE-001`, `DOC-ADR-001`

---

## 1. Purpose

Enable **capability-mediated guest→host I/O** so Aether can grow toward CLI/systems
usefulness without granting ambient OS authority to untrusted AETH.

This model is **required reading before any I/O-bearing host weave is coded**.

---

## 2. Assets (expanded)

| Asset | Why it matters |
| --- | --- |
| Operator filesystem | Confidentiality/integrity of files outside grant roots |
| Grant tokens / capability table | Controls what guest may touch |
| Verified AETH | Still untrusted for ambient authority |
| Seed artifact | Product compile integrity |
| Process environment | Secrets in env vars |
| Project locks / sources | Integrity of local programs |

---

## 3. Actors and trust

| Actor | Trust | Authority |
| --- | --- | --- |
| **Human operator** | Sovereign | Launches CLI; chooses grant roots and outputs |
| **Host CLI / library** | Trusted *as process owner* | Installs only operator-selected host services; enforces grants |
| **Guest AETH** | **Untrusted** | May call only **declared + installed** host weaves; no ambient FS/net/shell |
| **Editor / LSP** | Trusted like CLI | Still no product AETH authority; no ambient grants to guest |
| **Network** | Untrusted / out of M14 | No network host weaves in M14 |
| **AI / model** | Out of toolchain | Never grant installer |

---

## 4. Trust boundary (v2)

```text
┌────────────────────────────────────────────────────────────────┐
│ Operator process                                                │
│  ┌─────────────────────┐     grant table      ┌──────────────┐ │
│  │ Host (CLI/run)      │◄────────────────────►│ FS / env     │ │
│  │ - verifies AETH     │   only via grants    │ (selected)   │ │
│  │ - installs services │                      └──────────────┘ │
│  └──────────┬──────────┘                                       │
│             │ HOST_CALL (typed)                                │
│  ┌──────────▼──────────┐                                       │
│  │ Guest VM            │  still: no open(), no shell, no net   │
│  │ (untrusted AETH)    │                                       │
│  └─────────────────────┘                                       │
└────────────────────────────────────────────────────────────────┘
```

**Invariant:** Guest never receives a raw OS handle. Host weaves accept/return
only Aether primitive ABI types (Whole/Truth/Text/Bytes) as in M8, unless a
later ADR introduces opaque capability values (out of M14 pilot).

---

## 5. Grant model (normative for M14)

### 5.1 Deny by default

- Missing host service → **fail closed** (`AE-HOST-003`).
- Host service present but grant missing/expired → **fail closed**.
- Grant for path A never implies path B (no parent-walk).

### 5.2 Grant kinds (M14 pilot catalog)

| Grant | Meaning | Bound |
| --- | --- | --- |
| `read_file` | Read whole file into `Bytes` or `Text` (UTF-8 check) | Relative path under a **grant root**; max size cap |
| `write_file` | Write `Bytes`/`Text` to file (create/truncate) | Relative path under grant root; max size |
| `env_get` | Read one env var name → `Text` or missing fail | Allow-list of names **or** explicit name grant |
| Pure fixtures | `whole_inc`, `text_extent` | Unchanged; no grant needed |

### 5.3 How grants are installed (host-side)

M14 pilot (design):

```text
aether run artifact.aeth --grant-read <root> --grant-write <root> [--grant-env NAME]
```

- Roots must be absolute filesystem paths chosen by the operator.
- Guest paths are **relative** strings (same jail spirit as projects: no `..`,
  no absolute, `/`-only in guest path grammar).
- Host joins `root + guest_relative` and **canonicalizes**; result must stay
  under root.

Library/API form (for tests): install grant table on host session before
`run_bytecode_with_host`.

### 5.4 Non-grants in M14

| Not granted | Why |
| --- | --- |
| Shell / process spawn | Command injection; ambient |
| Network sockets | Separate threat epoch |
| Directory list / recursive walk | Easy data exfil; later ADR |
| Arbitrary absolute guest paths | Jail break |
| Symlink following outside root | Policy: **reject** if canonicalize escapes (or reject symlinks entirely in pilot) |

---

## 6. Entry points (delta)

| Entry | Change under v2 |
| --- | --- |
| `run` | May install grant-backed host weaves when flags present |
| `compile` / forge / project | Unchanged (no guest I/O) |
| `lsp` | Still no guest I/O; no grants |

---

## 7. Abuse cases and controls

| Abuse | Control |
| --- | --- |
| Guest path `../secret` | Reject at path validate / canonicalize-under-root |
| Symlink to `/etc/passwd` | Canonicalize must remain under root or fail |
| Huge file read DoS | Max read/write byte cap (e.g. 1_000_000 aligned with source limits) |
| Env secret harvest | Only named grants; no `env_list` in M14 |
| Missing service | Fail closed |
| Confused deputy (host writes outside root) | Host only joins under grant roots |
| Grant confusion across runs | Grants are per-invocation; not persisted in AETH |

---

## 8. Residual risks (human-accepted for M14 pilot)

1. Operator who grants a root containing secrets is trusting the guest program.
2. Bootstrap vs seed diagnostic differences remain for authoring.
3. No formal multi-tenant isolation (single operator process).
4. Symlink/TOCTOU races on concurrent host FS mutation — accept for pilot; document.

---

## 9. Relationship to technical preview model

| Topic | TP model | Capable host v2 |
| --- | --- | --- |
| Pure host fixtures | Only I/O-free | Still available |
| Guest file I/O | Forbidden | **Grant-mediated only** |
| Network | Out of scope | Still out of M14 |
| Offline-first CLI | Yes | Yes |
| Ambient guest OS | Forbidden | Still forbidden |

When M14 ships, product docs must **dual-cite**: TP model for non-I/O surfaces;
this document for grant-backed `run`.

---

## 10. Change control

Invalidates this freeze (new ADR required):

- Network host weaves
- Process/shell spawn
- Opaque file descriptors as guest values
- Ambient “current directory” without grant root
- Multi-tenant shared host with untrusted operators

---

*End of threat model v2.*
