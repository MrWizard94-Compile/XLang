# Changelog — Aether 0.12+ technical preview lineage

Honest user-facing deltas from the bounded resource core through the technical
preview package. This is **not** a claim of 1.0 completeness.

## 0.18.0 — project-aware LSP imports (M13b)

- `aether lsp --project <aether.project.json>` (or `initializationOptions.projectFile`).
- Definition/hover for `call alias.weave` resolve **exported** weaves under project path jail.
- Private weaves do not resolve; module files get informational AE-MOD-007 in the editor.

## 0.17.0 — bounded offline LSP (M13a)

- `aether lsp` stdio JSON-RPC Language Server.
- Bootstrap diagnostics, document symbols, formatting, hover, definition.
- **Not** a product AETH compiler; **no** silent disk writes; offline only.
- Editor diagnostics ≠ seed product compile (documented in diagnostic messages).

## 0.16.0 — fine-grained structural edits (M12)

- `aether.edit/v7` / `aether.ast/v7` / `aether.diagnostic/v7`.
- Statement ops: `replaceStatement`, `insertStatementAt`, `insertStatementAfter`, `deleteStatement`.
- Paths like `weave:main/body/1`; Choose/While nested lists supported.
- Product apply-edit accepts **v7 only** (v6 rejected).
- Seed compile before write unchanged.

## 0.15.0 — language modules M11b (seed product path)

- Package **0.15.0**: multi-module build elaborates then **seed-compiles**.
- Bootstrap≡seed dual-compare on every `project build`.
- M11 language surface complete under ADR-015 (still host elaboration, not seed multi-file parse).

## 0.14.0 — language modules M11a (bootstrap multi-module)

- Package **0.14.0**: `import unit`, `export weave`, `call alias.weave`.
- `aether project build` elaborates the import DAG and bootstrap-compiles one AETH.
- Lib units may omit `main`; single-file seed compile rejects imports.
- Example: `examples/project-modules` (exit 42).

## 0.13.0 — multi-unit offline projects (M10)

- Package version **0.13.0**; language surface remains **0.11** / AETH **v11**.
- Nested relative unit paths (`src/main.ae`) with forward-slash-only grammar.
- Multi-unit locks, independent per-unit seed compile, `project format [--write]`.
- `project verify --output-dir` flat mapping (`src/main.ae` → `src__main.aeth`).
- Example: `examples/project-multi/`.
- **Not included:** language modules/imports, registry, lib-without-main.

## 0.12.0 — offline project tooling (M9)

- Package version **0.12.0**; language surface remains **0.11** / AETH **v11**.
- `aether.project/v1` documents with relative units and optional SHA-256 locks.
- CLI: `aether project verify` (schema, path confinement, locks, seed-compile).
- CLI: `aether format` for canonical source (explicit `--output` or stdout).
- **Not included:** package registry, network fetch, multi-unit dependency graph, LSP.

## 0.11.0 — pure host ABI pilot (M8)

- `host weave` declarations and `HOST_CALL` (AETH v11).
- Product pure fixtures only: `whole_inc`, `text_extent`; missing services fail closed.
- **Not included:** C/FFI, libloading, ambient guest file/network/shell I/O.

## 0.10.0 — structured nurseries (M7)

- Lexical `together` / `spawn` nurseries; cooperative source-order execution.
- First child `Error[Whole]` cancels remaining unstarted spawns.
- **Not included:** OS threads, task handles, nested nursery product surface claims beyond corpus.

## 0.9.0 — dual-layout tables (M6)

- `shape` + `table … layout rows|columns` with closed allocate/store/load.
- Semantic equivalence between layouts; no automatic layout rewrite.

## 0.8.0 — deterministic comptime (M5)

- Root-only literal `comptime bind` for one arithmetic op per directive.
- Fixed 1,024-directive budget; `COMPTIME_WHOLE` provenance.
- **Not included:** names, calls, macros, host I/O at compile time.

## 0.7.0 — typed error effect (M4)

- Bounded abortive `Error[Whole]` with `raises`, `raise`, `forward call`, `handle call`.
- No hidden exception path; clean boundary vs live resource owners.

## 0.6.0 — bounded resources (M2)

- One positive `arena` in `main`; Whole/Truth buffers; closed allocate outcomes.
- No ambient allocator; resources do not cross host/forge ABI.

## Authoring / product shape (cross-cutting)

- Seed-hosted product compile (default); Rust bootstrap for rebuild/diagnostics.
- Structural authoring contracts through **v6** (top-level structural edits only).
- Aether Studio retired; CLI is the product interface (ADR-006).
- Verify-before-run/write; AETH v4–v11 compatibility inputs as documented.

## Known non-claims

- Not full diagnostic parity for the seed compiler.
- Not a general systems language 1.0.
- Not multi-package offline projects (queued as P4.1 after preview).
- Not “better than all languages.”
