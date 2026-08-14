# Threat Model — Aether 0.36.0 Local Technical Preview

**Status:** Current local-preview threat model

**Date:** 2026-08-08

**Product pin:** Aether package 0.36.0; language surface 0.11 plus bounded
M19e task frames; AETH v11 without task frames and v12 with task frames

**Related:** [MANIFEST.md](../../MANIFEST.md), [AETHER_0.36.md](AETHER_0.36.md),
[M14 capable-host model](../Current%20state/THREAT_MODEL-v2-CAPABLE-HOST.md),
[M21 foreign-ABI model](../Current%20state/THREAT_MODEL-v3-FOREIGN-ABI.md), and
[ADR-042](ADR-042-m19e-active-frame-cancel.md)

**Rule IDs:** `SEC-INPUT-001`, `SEC-PRIV-001`, `SEC-SECRET-001`,
`REL-HARDEN-001`, `REL-DETERM-001`, `CONST-GATE-001`

## 1. Purpose and scope

This model defines the trust boundary for the local 0.36 technical-preview
package. It combines the prior pure-preview, capable-host, and foreign-pilot
models only for their already-implemented bounded surfaces. It does not claim
formal certification, multi-tenant isolation, a public-release security review,
or safety for operator-granted native libraries.

## 2. Assets

| Asset | Why it matters |
| --- | --- |
| Operator filesystem and environment | May contain valuable data; guest access must remain grant-bounded. |
| Release binary, seed, metadata, and checksums | Establish the local product/compile authority. |
| Source, project/workspace JSON, edit JSON, and AETH | All are untrusted inputs that must fail closed. |
| Grant roots, environment names, and library paths | Define the only host authorities that a guest can receive. |
| Task-frame lanes and arena slabs | Must never leak into another task or survive cancellation. |
| Operator intent and licensing boundary | Local package is `UNLICENSED`; overbroad distribution is not authorized. |

## 3. Actors and authority

| Actor | Trust | Authority |
| --- | --- | --- |
| Human operator | Sovereign | Chooses files, explicit output paths, host grants, and whether to run native code. |
| Aether CLI / bootstrap / verifier | Trusted by the operator | Parses, validates, seed-compiles, verifies, and installs only explicit capabilities. |
| Seed compiler artifact | Trusted only after current package/forge evidence | Default compile authority embedded in the CLI. |
| Guest AETH | Untrusted | Executes only verifier-approved bytecode; has no ambient OS authority. |
| Project/workspace/edit documents | Untrusted | Must satisfy schemas, canonicalization, confinement, and lock checks. |
| Granted foreign library | Operator-approved but not sandboxed | Native code executes in the host process only after explicit path grant. |
| Network/model service/registry | Out of scope | No product authority or runtime dependency. |

## 4. Trust boundary

```text
┌────────────────────────────────────────────────────────────────┐
│ Operator-controlled local process                               │
│  source / JSON / AETH ──► parser + verifier ──► VM guest       │
│                               │                    (untrusted) │
│                               │ explicit grants only            │
│                               ▼                                 │
│                        selected FS/env/native library           │
│  no ambient guest process, shell, socket, registry, or model API│
└────────────────────────────────────────────────────────────────┘
```

Task frames remain inside the verified VM. They use pre-admitted private lanes,
park only at verified checkpoints, and are destroyed without guest callbacks on
an eligible sibling failure. They do not create a new host authority.

## 5. Entry points and controls

| Entry point | Untrusted input | Required control |
| --- | --- | --- |
| `check`, `compile`, `format`, `structure` | Aether source | Parser/semantic validation; seed or bootstrap boundary; explicit output only. |
| `apply-edit` | Source plus edit JSON | Exact-base stale guard, typed schema, canonical reparse, seed compile before write. |
| `run` | AETH bytes | Version-specific verifier before VM execution; default pure host catalog. |
| `forge` | Compiler AETH plus source | Verify compiler and returned artifact before explicit output write. |
| `project` / `workspace` | Local JSON, unit files, locks | Relative-path confinement, schema validation, complete lock checks where present. |
| `--grant-read` / `--grant-write` / `--grant-env` | Operator capability selection | Per-invocation allow-list, path canonicalization under roots, no ambient grant. |
| `--grant-lib KEY=PATH` | Operator-selected library | Explicit path and signature boundary; native code risk remains with operator. |
| preview verifier | Package files and checksums | Normalized confined paths, exact file membership, SHA-256, behavior probes. |

## 6. Deny-by-default invariants

1. Unknown/invalid AETH and malformed source/JSON are rejected before use.
2. Guest code cannot open arbitrary paths, spawn a process, run a shell, list a
   directory, open a socket, or call a model service.
3. Missing host services and missing grants fail closed.
4. Guest I/O paths are relative, canonicalized beneath an operator-selected
   root, and rejected if they escape it.
5. Environment access is named-grant only; there is no environment enumeration.
6. Foreign code requires an exact operator grant; no PATH search or header parser
   is used by the pilot.
7. Task code cannot carry host/foreign effects, task handles, timeouts, or
   arbitrary preemption into the v12 frame model.
8. Package verification rejects checksum path traversal, duplicate entries,
   mismatched hashes, omitted files, and unlisted files.

## 7. Residual risks and explicit limits

| Risk | Treatment |
| --- | --- |
| An operator grants a root containing sensitive files | Intentional operator authority; do not run an untrusted guest with such a grant. |
| An operator grants a hostile DLL | Not sandboxed; equivalent class of risk to native code execution. |
| Symlink/TOCTOU changes under an I/O root | Bounded pilot residual; canonicalization rejects escapes but is not multi-tenant isolation. |
| Resource exhaustion by large/deep inputs | Language and verifier bounds reduce exposure; no full DoS-resistance claim. |
| Invalid-source diagnostic differences | Bootstrap remains authority; seed diagnostic parity is not claimed. |
| Local package copied after verification | Re-run `verify-preview.ps1`; checksums cover the staged snapshot only. |
| Public distribution/licensing | Blocked: package is `UNLICENSED` and local-only until human direction. |

## 8. Release controls

A local technical-preview package is valid only after:

1. `aether-gate.ps1 -Mode release` passes, including pack integrity, source
   tests, seed identity, release build, consumer verification, and an unlisted
   package-file rejection probe.
2. The package is delivered with `SHA-256SUMS`, `RELEASE-METADATA.json`, current
   contract/release notes/threat model, seed, schemas, examples, and stdlib.
3. The package is kept in the local channel unless a human separately directs
   licensing, public distribution, tagging, and publication.

## 9. Change control

A new ADR and threat-model review are required before adding a network registry,
process/shell/network host capability, generalized foreign ABI, raw OS handles,
task handles/timeouts/parallelism, a native backend, multi-tenant hosting, or a
public-release/license claim.

*End of Aether 0.36 local-preview threat model.*
