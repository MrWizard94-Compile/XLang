# Aether Atlas

**Aether Atlas is a bounded, deterministic, offline journal-recovery system
written primarily in Aether.** It is a systems showcase, not a toy language
feature sample: it validates a compact journal frame, derives a recovery
decision, projects event state into bounded resource storage, runs concurrent
segments, exercises a failure/cancellation path, and exposes a narrowly granted
operator adapter.

The application logic under this directory is Aether source. Rust remains the
toolchain bootstrap/verifier/VM implementation; it is not a source
transpilation target. Default compilation uses the Aether-written seed compiler
through the forge ABI.

## System contract

Atlas accepts exactly one bounded ATLAS/1 recovery journal:

    ATLAS/1
    PUT 0041 0007
    PUT 0043 0011
    PUT 0047 0013
    COMMIT 0003

The journal is exactly 62 ASCII scalar values:

- one ATLAS/1 header;
- three fixed-width PUT key amount records, each with four decimal digits per
  value;
- one COMMIT 0003 record; and
- required line endings with no trailing data.

protocol/journal.ae never calls a fallible numeric conversion on input. It
checks width and ASCII decimal glyph ranges, folds the values explicitly, and
returns the deterministic rejection sentinel -100 for any malformed frame.
That makes the accepted grammar and failure behavior auditable in source.

## Package graph

    protocol -> policy -> replay
                    |
                    +-> audit

    operator     fault

| Package | Responsibility |
| --- | --- |
| protocol | Journal framing, decimal decoding, Text/Bytes fingerprints, and parser behavioral tests. |
| policy | Converts a valid protocol score into the recovery-policy score. |
| replay | Projects decoded events into a bounded arena, buffer, and both table layouts; runs two nursery segments; emits the final pure recovery score. |
| audit | Cross-package runtime assertion that policy accepts the canonical journal and rejects the corrupt journal. |
| operator | Explicit-grant read/write adapter for the named journal and receipt paths. |
| fault | Independent M4 + M19e fault-injection persona that proves active-frame cancellation on a verified v12 artifact. |

The pure multi-package recovery graph is deliberately separate from the host
adapter. Current product compilation rejects a host-ABI package combined with
cross-package imports; Atlas treats that as a real, verified toolchain boundary
instead of routing around it with unverified host logic. The operator therefore
has no imports and contains only capability-adapter logic.

The audit package imports policy and explicitly declares both policy and
protocol in its workspace dependency closure. That declaration is required by
the current M22 elaborator when policy itself imports protocol; it is a
toolchain constraint, not an ambient dependency.

## Aether surfaces exercised

| Surface | Atlas evidence |
| --- | --- |
| Seed product compile / deterministic AETH | workspace build runs twice on replay and compares SHA-256 artifacts. |
| M11/M18/M22 modules, projects, workspaces | Six locked packages and cross-package imports on the pure recovery graph. |
| Text parsing and bounded loops | protocol/journal.ae uses measure, glyph, cut, and explicit decimal folding. |
| M5/M23 comptime | replay/main.ae fixes table widths and page layout at compile time. |
| Records, Text, Bytes, M19a release | replay_receipt carries the label/frame/page projection; the label is explicitly released. |
| M2 arenas and Copy-element buffers | Three decoded event encodings are admitted only through a 512-byte arena budget. |
| M6 rows and columns | The same three events are stored and read through both physical table layouts. |
| M7 nursery | The replay runs two deterministic segment computations together. |
| M4 Error[Whole] + M19e task cancellation | fault/main.ae combines a checkpointed task with a sibling corruption effect and emits AETH v12. |
| M14 grant-backed host I/O | operator/main.ae reads and writes only after explicit grant-read / grant-write. |
| M17 tests | Parser acceptance/rejection, resource/layout behavior, and a checkpointed task success path. |
| M25 local packages | The verifier script packs, verifies, publishes, cache-verifies, installs, and re-verifies atlas_protocol. |

## Reproduce the evidence

From the repository root:

    pwsh -NoProfile -File .\showcases\aether-atlas\tools\verify.ps1

The script creates a fresh target/aether-atlas-evidence-GUID directory and
writes:

- two byte-identical replay AETH artifacts plus their SHA-256 values;
- product structural representations for the v12 fault persona and
  grant-backed operator;
- valid and corrupt operator receipts;
- a deliberately tampered artifact that the AETH loader/verifier rejects;
- an M25 protocol bundle, local cache, and installed project; and
- SUMMARY.md with the observed outcomes.

Expected observable results are:

| Run | Expected result |
| --- | --- |
| Policy audit | exits 0 after proving both cross-package acceptance and rejection. |
| Pure replay | SPEAKs atlas-replay; exits 4720242. |
| Valid operator | exits 20; writes ATLAS-ACCEPTED 10650. |
| Corrupt operator | exits 14; writes ATLAS-REJECTED. |
| Ungranted operator | fails closed with AE-HOST-003. |
| Fault persona | exits 91; artifact begins AETH 12. |
| Tampered replay artifact | is rejected before execution with an AETH artifact error. |

## Security and scope

Read [THREAT_MODEL.md](THREAT_MODEL.md) before treating Atlas as a deployment
template. It has no network, shell, database, cryptographic, authentication, or
durability claim. Its strength is a narrow, reproducible demonstration of
verified execution and explicit authority on the Aether surface that exists
today.
