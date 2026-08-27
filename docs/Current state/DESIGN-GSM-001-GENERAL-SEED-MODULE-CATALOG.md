# GSM-001 — General Seed Module Catalog

**Status:** Implemented; full gate passed 2026-08-27  
**Date:** 2026-08-27  
**Decision:** [ADR-131](../historical%20docs/ADR-131-gsm-general-seed-module-catalog.md)  
**Validation:** [GSM-001 matrix](GSM-VALIDATION-MATRIX.md)  
**Security:** [GSM-001 threat model](THREAT_MODEL-GSM-001-SEED-MODULE-CATALOG.md)  
**Scope:** Default product M11/M22 graph elaboration; no language, AETH, VM, or guest-capability expansion

## 1. Outcome

GSM-001 replaces host source-graph elaboration on the default project and
workspace product path with a bounded seed-native route. The host frames an
explicit, deterministic catalog of manifest-authorized source bytes. The
Aether-written seed validates and resolves the M11/M22 graph, rewrites the
necessary module boundaries, and emits verified AETH.

The outcome is deliberately not a filesystem resolver inside the seed. Source
discovery stays at the host boundary under existing caller-selected manifest,
lock, path-jail, and direct-dependency rules. Once the catalog exists, source
content is opaque to Rust until the verified seed has finished the graph work.

## 2. Public protocol

Schema: "aether.seed-modules/v1"

~~~text
aether.seed-modules/v1
entry <local-main-or-test-key>
packages <N>
package <safe-package-name>
...
units <N>
unit <identity> <main|lib|test> <source-scalar-count>
...
source
<source payloads concatenated in unit-header order>
~~~

A local identity is a project-relative confined ".ae" path. A foreign identity
is "package::project-relative-path"; the package name is safe ASCII and the
path is independently confined. The entry must be local. A foreign identity is
always role "lib".

The source payload is not escaped, line-normalized, decoded, or transformed by
the protocol framing layer. Unicode scalar counts delimit each source exactly.
That makes LF, quotes, non-ASCII source text, and arbitrary valid Aether body
contents unambiguous on the wire.

## 3. Authority split

| Operation | Host product route | Seed "compile_modules" |
| --- | --- | --- |
| Read caller-selected manifest/lock | Yes | No |
| Resolve local/project package roots | Yes, existing path jail | No |
| Read manifest-selected UTF-8 source files | Yes | No |
| Enforce direct workspace dependency authority | Yes, then represent it in catalog | Recheck catalog package authority for every foreign edge |
| Decode Aether source imports | No | Yes |
| Discover graph edges / detect cycles | No | Yes |
| Inspect roles, exports, aliases, and qualified calls | No | Yes |
| Mangle names / rewrite qualified calls / assemble source | No | Yes |
| Parse/compile assembled Aether source | No | Yes |
| Verify returned AETH before return/write/run | Yes | Seed produces Bytes only |

The retained Rust M11/M22 elaborator is an independent bootstrap/reference
oracle. Tests may compare its result with the product route. It is not invoked
by "compile_project_modules", project test execution, workspace builds, the
multi-source envelope product route, or "forge-modules".

## 4. Host catalog framing

"encode_project_seed_module_catalog" is intentionally limited to:

1. validate a selected entry is a local "main" or "test" manifest unit;
2. read all manifest-declared local units through the established path jail;
3. for each explicitly allowed package, read its manifest and include its
   declared "lib" units under "package::path";
4. reject unavailable roots, unsafe names/paths, invalid UTF-8, bad roles,
   duplicate keys, and published scalar bounds; and
5. sort identities and encode the exact deterministic wire record.

The function never calls the Rust Aether parser, "load_graph", the old
elaborator, mangle helpers, or source rewriting. In-memory local-unit APIs frame
only local keys and reject cross-package imports because they have no explicit
package authority.

Workspace framing is direct only: a consumer may name its own package and the
packages declared in that consumer's "depends_on". It does not obtain
transitive-package access merely because a dependency has one.

## 5. Seed graph algorithm

The seed:

1. validates scalar framing, metadata, source limits, identity uniqueness,
   package list uniqueness, and entry role;
2. parses the documented M11/M22 normal-form module headers and import lines;
3. rejects unknown, unavailable, unauthorized, non-lib, malformed, or cyclic
   import targets;
4. walks the entry's reachable DAG in deterministic dependency-first order,
   subject to the traversal guard;
5. checks worlds, aliases, declarations, visibility, and qualified calls;
6. assigns deterministic module-qualified weave names, rewrites calls, strips
   import/export transport syntax, and emits one single-world source; then
7. invokes the existing seed "compile" weave.

An unreachable valid candidate unit is allowed in the catalog. It supplies no
extra import target to the entry cone unless an actual reachable source names
it. This allows closed manifest framing without making topology fixed.

Failures return empty Bytes and a coarse structured "AE-SEED-017" seed-SPEAK
packet. The product host preserves that packet through its normal forge-error
channel. GSM-001 does not claim bootstrap-equivalent diagnostic detail.

## 6. Resource bounds

| Resource | Bound |
| --- | ---: |
| Candidate units | 256 |
| Explicit package authorities | 64 |
| Unit source payload | 16,384 Unicode scalars |
| Aggregate source payload | 196,608 Unicode scalars |
| Full catalog wire text | 250,000 Unicode scalars |
| Identity length | 256 Unicode scalars |
| Seed graph steps | 66,000 |

These are hard fail-closed protocol limits, not advisory operating targets.
The 250,000-scalar wire cap is Unicode-safe: at most four UTF-8 bytes per
scalar means every catalog remains within Aether's existing 1,000,000-byte
invocation-Text safety limit. They make a broad graph shape possible without
permitting unbounded parse, memory, or traversal work.

## 7. CLI and API surface

- "compile_product_seed_modules(catalog)" invokes the checked-in seed's exact
  "compile_modules [borrow catalog: Text] -> Bytes" weave.
- "aether compile" recognizes a GSM-001 catalog before ordinary raw-source
  preflights, because a valid catalog intentionally contains "import unit"
  source lines.
- "aether project build", project test, and "aether workspace build" use host
  framing plus that product seed route.
- "aether forge-modules <compiler.aeth> <catalog.aem> --output <artifact.aeth>"
  exposes the same named ABI for an operator-selected catalog file.
- The core and CLI require a verified compiler artifact, the exact one-borrowed
  Text/Bytes signature, and an independently verified returned artifact.

The canonical "examples/seed-modules-general.aem" fixture contains six candidate
units: a local two-leaf fan-in, one direct M22 "math" dependency, and one
unreachable library. It exits 85 and proves ordinary product compilation equals
the external named-forge result.

## 8. Compatibility and non-goals

Unchanged:

- Aether language syntax and language version;
- supported AETH input/output versions and deterministic artifact rules;
- VM execution and verifier rules;
- grants, host ABI, foreign ABI, package cache, registry, and native authority;
- legacy "aether.seed-bundle/v1", "/v2", and "/v3" APIs.

Not claimed:

- filesystem discovery by seed code;
- registry/network package resolution;
- ambient or transitive package authority;
- dynamic imports, package version solving, or arbitrary graph size;
- full bootstrap diagnostic parity;
- seed-native handling of raw single-file "import unit" input outside a framed
  catalog;
- a guarantee that any syntactically arbitrary source body is accepted beyond
  the documented seed compiler and M11/M22 normal-form surface.

## 9. Verification contract

The delivery must retain all of these evidence classes:

1. unit tests for catalog framing and limits;
2. a test proving host framing accepts opaque non-Aether source rather than
   parsing it;
3. an irregular six-unit local graph whose product artifact equals the
   Rust-elaborated/bootstrap oracle;
4. a direct M22 graph whose product artifact equals the oracle and whose
   undeclared package edge fails closed;
5. negative core and CLI tests for the exact "forge-modules" ABI;
6. gate execution of the checked-in six-unit catalog through product compile,
   VM run, and external named forge with byte identity; and
7. the Aether Atlas multi-package showcase verification.

The full repository gate, seed identity proof, documentation link check, pack
integrity check, and Constitution Section 0 review remain release requirements.

---

*End of DESIGN-GSM-001.*
