# Aether 0.12 Toolchain Contract

**Status:** Historical product toolchain contract — M9 offline project tooling
pilot; current package contract is [AETHER_0.35.md](AETHER_0.35.md)

**Language surface:** Aether 0.11 (AETH v11) unchanged

**Package version:** 0.12.0

## Purpose

Aether 0.12 adds offline project identity and first-class formatting without a
package registry, network dependency fetch, or LSP server.

## Project documents

`aether.project/v1` JSON lists local `.ae` units and optional SHA-256 locks.
See [DESIGN-M9-PROJECT-TOOLING.md](DESIGN-M9-PROJECT-TOOLING.md) and
[schemas/aether-project-v1.schema.json](../schemas/aether-project-v1.schema.json).

## CLI additions

```text
aether format <source-file> [--output <source-file>]
aether project verify <project-file> [--output-dir <dir>]
```

`project verify` is fully offline: schema, path confinement, lock digests, and
seed-compile of every unit.

## Explicit non-goals

Full LSP, multi-package graphs, remote registries, and signed public releases
are out of scope for M9.
