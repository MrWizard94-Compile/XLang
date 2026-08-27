# Threat Model — GSM-001 General Seed Module Catalog

**Status:** Implemented; full gate passed 2026-08-27  
**Date:** 2026-08-27  
**Decision:** [ADR-131](../historical%20docs/ADR-131-gsm-general-seed-module-catalog.md)  
**Design:** [GSM-001 design](DESIGN-GSM-001-GENERAL-SEED-MODULE-CATALOG.md)  
**Validation:** [GSM-001 matrix](GSM-VALIDATION-MATRIX.md)

## Assets and trust boundaries

| Asset / boundary | Required protection |
| --- | --- |
| Caller-selected project/workspace manifest and lock | Existing schema, lock, nested-path, and direct-dependency validation before source framing |
| Local source files and package roots | Explicit root selection, path confinement, UTF-8 decoding, no guest-controlled path opens |
| Catalog metadata | Safe identities, scalar-count framing, uniqueness, fixed resource bounds, deterministic order |
| Catalog source payload | Treated as untrusted Aether source; never parsed by the host product framer |
| Seed compiler artifact | Verified supported AETH before invocation; source/artifact identity proven by the full gate |
| Returned AETH artifact | Independently verified before product return, CLI write, or VM run |
| Guest seed execution | Pure fixture-only host services; no filesystem, shell, process, network, registry, cache, grant, foreign-library, or model authority |
| Workspace package authority | Only the consumer package's direct "depends_on" plus explicit self package can be represented in a project/workspace catalog |

## Threats and controls

| Threat | Control | Residual |
| --- | --- | --- |
| A source import attempts path traversal or opens a host-selected file. | The seed receives an already closed catalog and has no filesystem service. Catalog identities and project roots are path-jail validated before framing. | A trusted operator can deliberately place any selected local source in a standalone catalog; that is an explicit caller choice, not source-driven discovery. |
| A catalog claims a foreign package not granted by the workspace. | Product framing derives foreign units from direct allowed roots only. Rust and seed both validate package identities; seed rejects an edge not in the explicit list. | The standalone "forge-modules" command accepts an operator-supplied authority list; it is a local explicit input, not a workspace policy bypass. |
| A package uses an unapproved transitive dependency. | Catalog framing includes only consumer direct dependencies and self. Seed requires every foreign target to name one catalog authority. | A direct dependency can expose its own library code as included source, but it cannot cause a new package root to be read. |
| Malformed metadata desynchronizes source boundaries. | Scalar counts, exact end-of-payload checks, fixed schema order, duplicate checks, and total/wire bounds are validated independently in Rust framing and seed code. | Diagnostics are intentionally coarse. |
| Oversized catalog consumes disproportionate time or memory. | Hard limits: 256 units, 64 packages, 16,384 scalars per unit, 196,608 aggregate source scalars, 250,000 wire scalars, 256-scalar identities, and a 66,000-step seed traversal guard. The wire cap is at most one quarter of the 1,000,000-byte Aether Text invocation limit, including worst-case Unicode. | Bounded inputs can still consume their declared budget; capacity planning remains an operator concern. |
| Cyclic or highly connected graph causes nontermination. | Seed keeps deterministic visiting/visited/processed sets and rejects cycles; graph work has an explicit step guard. | The valid maximum graph remains nontrivial by design. |
| Private or missing weave is invoked across a module boundary. | Seed parses module declarations, validates "export" visibility, rewrites only declared export calls, and rejects unresolved qualified aliases. | The implementation follows the documented normal-form M11/M22 surface; it does not promise full bootstrap diagnostic detail. |
| Alias collision, duplicate unit key, duplicate package, or duplicate world changes binding. | Catalog and seed each reject duplicate identities/packages; seed rejects duplicate worlds and aliases. Deterministic ordering avoids ambient map iteration behavior. | Name mangling is a defined representation contract and must remain regression-tested. |
| Host quietly resumes Rust graph elaboration. | Product routes call only catalog framing plus "compile_product_seed_modules"; explicit tracker and source-level opaque-framing tests protect the boundary. Rust elaboration is retained only for test/recovery oracle use. | A future route must receive a new ADR and validation evidence before it may invoke the elaborator. |
| A forged compiler returns malformed or hostile bytes. | Both "forge_modules_bytecode" and the CLI verify the compiler first and verify returned AETH before accepting it. ABI mismatch/missing entry fails closed. | Verified AETH still has the existing Aether VM/resource semantics; GSM changes none of them. |
| Seed code obtains an ambient capability while processing source. | The named forge invocation uses pure fixture host services and accepts borrowed Text only. No capability-bearing parameters or grants are installed. | Existing pure fixture behavior remains in scope of its own threat models. |
| An attacker relies on full error detail to probe source/catalog content. | GSM emits one coarse AE-SEED-017 class for catalog/graph rejection. Host maps it through existing product error handling. | Coarse messages aid safety but do not provide a full diagnostic-parity promise. |
| Nondeterministic framing or graph traversal produces different artifacts. | Host identity sort, scalar framing, deterministic seed post-order, stable mangling, byte identity tests, and product-vs-named-forge gate checks. | Determinism remains contingent on the existing deterministic compiler/VM contract. |

## Explicit non-authority

GSM-001 does not authorize:

- seed filesystem access or package path discovery;
- network resolution, registry fetch, package range solving, or cache mutation;
- shell/process execution, native compilation, foreign library loading, or models;
- grant propagation into the seed compiler;
- dynamic imports or graph expansion beyond the closed catalog;
- unbounded source, graph, package, or traversal resource consumption; or
- accepting a returned artifact without independent verifier approval.

## Operational guidance

Use project/workspace commands for normal builds so that existing manifests,
locks, root confinement, and direct dependency rules create the catalog. Use
"forge-modules" only when the operator intentionally supplies one local catalog
file and wants to exercise the exact named compiler ABI. Preserve the catalog
alongside a reproducibility record when it is used as release evidence.

Any request to add dynamic discovery, additional package authorities, a registry,
network transport, a new guest capability, or a larger resource envelope must
receive a separate decision, hostile-input test plan, and updated threat model.

---

*End of THREAT_MODEL-GSM-001-SEED-MODULE-CATALOG.*
