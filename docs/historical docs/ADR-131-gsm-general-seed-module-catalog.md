# ADR-131: GSM-001 — general bounded seed-native module catalog

**Status:** Accepted — implemented; full gate passed 2026-08-27<br>
**Date:** 2026-08-27<br>
**Related Rule IDs:** CONST-DEP-001, DOC-ADR-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001<br>
**Program:** [BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)<br>
**Supersedes for the default product route:** ADR-056 host graph elaboration<br>
**Depends on:** ADR-015, ADR-045, ADR-047, ADR-049, ADR-056, ADR-075, ADR-128, ADR-129, ADR-130<br>
**Design:** [GSM-001](../Current%20state/DESIGN-GSM-001-GENERAL-SEED-MODULE-CATALOG.md)<br>
**Security review:** [GSM-001 threat model](../Current%20state/THREAT_MODEL-GSM-001-SEED-MODULE-CATALOG.md)

## Context

M11/M22 source graphs had been parsed, resolved, validated, mangled, and
assembled by Rust before the seed emitted the resulting single-file program.
ADR-128 through ADR-130 reduced that authority for three deliberately fixed
bundle shapes, but they did not make ordinary project or workspace module
builds seed-native. The visible product claim therefore remained “host
elaborate + seed emit.”

That split was the largest remaining bootstrap-authority boundary in the
default compiler path. It also prevented a normal multi-package Aether program
from serving as direct self-host evidence: the Aether-written seed compiled the
assembled result but did not own the import graph that produced it.

The replacement must not turn the host into an ambient resolver, let the seed
open paths, or silently broaden guest capabilities. The boundary must remain
local, explicit, bounded, deterministic, and independently testable.

## Decision

Add the closed "aether.seed-modules/v1" catalog protocol and the named seed
forge entry:

~~~aether
weave compile_modules [borrow catalog: Text] -> Bytes:
~~~

The catalog is scalar-indexed, deterministic Text. Its metadata is LF-only
ASCII, and its source payload is concatenated raw Unicode Text whose boundaries
are defined by declared scalar counts:

~~~text
aether.seed-modules/v1
entry src/main.ae
packages <count>
package <directly-authorized-package>
units <count>
unit <identity> <main|lib|test> <scalar-count>
source
<concatenated source units in header order>
~~~

Local identities are confined project-relative ".ae" paths. A foreign M22
identity is "package::project-relative-path"; it must be a "lib" unit and its
package must appear in the explicit allowed-package list. The entry is a local
"main" or "test" identity. Source strings are opaque to the Rust framing layer.

On "aether project build", project test, and "aether workspace build", the host
does only the following before "compile_modules":

1. parse the caller-selected manifest/lock and enforce existing nested-path and
   UTF-8 rules;
2. select local units and direct "depends_on" package "lib" units from those
   explicit roots;
3. assign manifest-derived identities and roles, sort them deterministically,
   and frame the bounded catalog; and
4. invoke the verified seed entry and independently verify returned AETH before
   writing or running it.

The seed owns catalog validation, M11/M22 import parsing, direct-package
authorization checks, graph reachability and cycle rejection, role and export
checks, world uniqueness, deterministic mangling, qualified-call rewrite,
assembly, and compilation. Rust keeps its prior elaborator only as a
bootstrap/reference oracle for tests and recovery analysis; it is not used by
the default product project/workspace route.

"aether forge-modules <compiler.aeth> <catalog.aem> --output <artifact.aeth>"
is the public transport for one caller-selected catalog. It verifies the input
compiler and returned artifact, requires the exact borrowed-Text/Bytes ABI, and
forwards catalog Text without decoding source or elaborating a graph.

## Bounds and deterministic rules

| Dimension | Bound / rule |
| --- | --- |
| Candidate units | 1–256 source units; graph topology is not fixed |
| Direct package authorities | 0–64 explicitly named packages |
| Per-unit payload | 1–16,384 Unicode scalars |
| Aggregate source payload | At most 196,608 Unicode scalars |
| Whole wire catalog | At most 250,000 Unicode scalars (at most 1,000,000 UTF-8 bytes) |
| Identity length | At most 256 Unicode scalars |
| Traversal | Bounded, deterministic depth-first graph walk; 66,000-step guard |
| Identity grammar | Local confined ".ae", or one safe "package::confined .ae" pair |
| Entry | Exactly one declared local "main" or "test" unit |
| Foreign units | "lib" only and only from the explicit package list |
| Ordering | Host sorts catalog identities; seed graph order is deterministic post-order |
| Artifact acceptance | Forge independently verifies returned supported AETH before return/write/run |

The protocol intentionally carries a closed candidate set rather than allowing
source import strings to trigger filesystem discovery. An unreachable valid
unit may appear in a catalog; it is neither parsed into the assembled program
nor granted additional host authority.

## Invariants

| ID | Invariant |
| --- | --- |
| GSM-INV-001 | The host performs only manifest/lock/path/UTF-8 framing and direct package-root selection; it does not parse imports, resolve graph edges, inspect exports, mangle, rewrite, or assemble Aether source on the product route. |
| GSM-INV-002 | The seed receives only borrowed catalog Text and returns only verified Bytes. It has no filesystem, shell, process, network, registry, cache, grant, callback, or model service. |
| GSM-INV-003 | A foreign M22 edge is eligible only when its package is explicitly present in the catalog authority list; workspace framing derives that list from the consumer package’s direct "depends_on" plus explicit self import. |
| GSM-INV-004 | Catalog framing rejects duplicate identities/packages, malformed roles/counts, unsafe identities, oversized payloads, nonlocal entries, missing entries, and foreign non-lib units before the seed forge is invoked. |
| GSM-INV-005 | Seed graph processing rejects malformed module headers/imports, unavailable or unauthorized targets, non-lib imports, cycles, duplicate worlds, alias conflicts, private or unknown qualified calls, and graph/resource bound exhaustion. |
| GSM-INV-006 | The normal project/workspace product route never invokes the Rust elaborator or bootstrap compiler. Rust elaboration remains an independent oracle only. |
| GSM-INV-007 | The protocol changes no Aether syntax, AETH version/encoding, VM semantics, guest capability contract, package resolver, cache, network behavior, or native/foreign ABI authority. |
| GSM-INV-008 | Product graph failures use coarse AE-SEED-017 seed-SPEAK packets. Full multi-module diagnostic parity with bootstrap is not claimed. |

## Consequences

Ordinary bounded M11/M22 graphs now have a seed-owned product elaboration path,
including arbitrary acyclic graph shapes within the catalog limits rather than
only the three historical SBP bundle topologies. The new proof surface includes
normal local project loading, direct dependency M22 imports, an external named
forge ABI, and the six-package Aether Atlas systems showcase.

The fixed "aether.seed-bundle/v1", "/v2", and "/v3" profiles remain supported as
separate narrow APIs and retain their historical evidence. They no longer define
the maximum generality of the seed product path. Historical ADRs remain true for
the release state they recorded; this ADR supersedes their former current-route
constraint.

No claim is made that the seed is a general filesystem resolver, that an
arbitrary package graph may be discovered dynamically, that transitive package
authority is implicit, or that all bootstrap diagnostics are reproduced. The
accepted scope is a bounded, closed, manifest-authorized graph catalog and the
documented M11/M22 normal form.

## Proof obligations

1. Rebuild "seed/aether_seed.aeth" from "seed/aether_seed.ae" and preserve
   bootstrap/product/forge byte identity.
2. Prove encoder/decoder scalar framing, deterministic ordering, bounds,
   malformed metadata rejection, duplicate rejection, and path/package
   confinement.
3. Prove product catalog compilation of an irregular local graph with fan-in and
   an unreachable candidate unit is byte-identical to independent Rust
   elaboration plus bootstrap emission, verifies, and executes.
4. Prove a direct M22 dependency graph is byte-identical to the independent
   reference and that undeclared foreign packages fail closed before execution.
5. Prove the external "forge-modules" ABI rejects incorrect/missing named weave
   signatures and writes only independently verified AETH.
6. Extend the repository gate with a six-unit local-plus-M22 catalog fixture
   compiled through both normal product and external named-forge routes.
7. Run the Aether Atlas six-package verification to demonstrate the ordinary
   project/workspace route under task, resource, capability, package-lock, and
   deterministic replay pressure.
8. Complete Constitution Section 0 review, documentation synchronization,
   warning-denied analysis, full tests, pack verification, and a committed,
   pushed clean handoff before claiming delivery.

---

*End of ADR-131.*
