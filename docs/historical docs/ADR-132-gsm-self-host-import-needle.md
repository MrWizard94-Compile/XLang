# ADR-132: GSM-001 self-host import needle

**Status:** Accepted — implemented<br>
**Date:** 2026-10-21<br>
**Related Rule IDs:** DOC-ADR-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, TEST-BEHAVIOR-001<br>
**Depends on:** ADR-098, ADR-131<br>
**Design:** [GSM-001](../Current%20state/DESIGN-GSM-001-GENERAL-SEED-MODULE-CATALOG.md)

## Context

Self-host of the GSM-001 seed fails in two steps. Both are inside the seed
program. The Rust bootstrap still emits a valid artifact, so the checked-in
file can match bootstrap while the product rebuild does not.

1. ADR-098 builds the `AE-SEED-012` needle at runtime and seeks the contiguous
   text `import unit ` across the whole source buffer. GSM-001 added that
   literal in `modules_import_target_line`, `modules_import_alias_line`, and
   `modules_import_count`. Forging the seed source then stops with
   `AE-SEED-012` (`raw multi-module import needs project build`). The forged
   bytes are the error packet, which is shorter than an AETH header.
2. The seed compiler's name resolver hardcoded the local name `source` to slot
   0, which is correct only when `source` is the first parameter (as in
   `compile`). GSM weaves also bind `source` at later slots
   (`modules_transform`, `modules_graph_order`, `modules_worlds_unique`).
   Those loads become slot 0, so a successful forge still disagrees with
   bootstrap. This second fault is invisible until the import needle no longer
   aborts the forge.

The host preflight is line-oriented and does not reject the literals.

## Decision

Keep the GSM-001 catalog parser, graph elaborator, and the whole-buffer
rejection of real raw `import unit` input.

Assemble the three parser prefixes the same way ADR-098 assembles the
preflight needle: join `"import "` and `"unit "` at runtime. Do not turn the
scan into a line-start check, and do not remove the parsers.

Resolve the local name `source` through the same local table as every other
name. A first-parameter `source` still resolves to slot 0. A later binding
resolves to its own slot.

Any later seed-resident copy of a fail-closed whole-buffer needle must be
assembled at runtime. Proof remains byte identity of the bootstrap rebuild,
the checked-in `seed/aether_seed.aeth`, and the product-path rebuild of
`seed/aether_seed.ae`.

## Consequences

GSM-001 behavior for framed catalogs and for genuine raw imports is unchanged.
The seed can forge its own source again. A source-text test rejects a future
contiguous `import unit ` literal in the compiler source before the long
rebuild.

## Proof obligations

1. `seed/aether_seed.ae` contains no contiguous `import unit `.
2. Bootstrap emission, the checked-in artifact, and the product-path rebuild
   are byte-identical, including a second forge generation and a distinct
   valid variant.
3. Raw single-file `import unit` input still fails `AE-SEED-012`. Catalog
   import parsing still recognizes the same prefix at runtime.
4. Locals named `source` that are not the first parameter load their real
   slot. `compile`'s borrowed `source` parameter remains slot 0.

---

*End of ADR-132.*
