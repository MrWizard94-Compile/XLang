# Design brief: post-M10 track portfolio

**Status:** Companion to [ADR-014](ADR-014-post-m10-track-portfolio.md)  
**Date:** 2026-08-04  
**Authority:** Portfolio only — not an implementable feature design  

## Purpose

Give engineers and agents a single place to understand **what may come after
M10**, in what **default order**, and what **stop conditions** apply. Detailed
design for each track still requires its own document.

## Current product pin

| Item | Value |
| --- | --- |
| Language surface | 0.11 / AETH v11 |
| Toolchain package | 0.13.0 |
| Projects | Multi-unit offline integrity; **independent** compile units |
| Host ABI | Pure fixtures only (`whole_inc`, `text_extent`) |
| Authoring | Top-level structural edit v6 |

## Default queue (ADR-014)

```text
T-MOD  language modules          ← next design (default)
T-EDIT fine-grained edits
T-LSP  bounded LSP
T-CT   comptime expansion
T-RX   resource ↔ effect
T-HOST host I/O                  ← threat model rewrite first
T-FFI  C / foreign ABI
T-PKG  multi-package offline
T-NATIVE blocked by law
```

## Comparison matrix

| Track | Seed impact | Threat delta | AI value | Systems value | Ready to code? |
| --- | --- | --- | --- | --- | --- |
| T-MOD | High | Low–med | High | High | **No** — need DESIGN+ADR+matrix |
| T-EDIT | None/low | Low | **Highest** | Low | No |
| T-LSP | None | Med | High | Med | No |
| T-CT | High | Med | Med | Med | No |
| T-RX | High | Med | Med | High | No |
| T-HOST | Med | **High** | Med | **Highest** | No |
| T-FFI | High | **Highest** | Low | High | No |
| T-PKG | Low | Med | Med | Med | After T-MOD |

## T-MOD sketch (for upcoming design — non-normative)

Open questions for the future modules design (not decided here):

1. Syntax: `import` / `use` / weave-path / project-declared graph?  
2. Visibility: world-private vs export list?  
3. Cycles: forbid vs stratified?  
4. Link step: host-only link of many AETH vs single multi-unit artifact?  
5. Seed: full dual-compare required before product path?  
6. Interaction with `lib` role and projects without `main` in non-entry units?

Stop conditions: ambient globals, path escape via import, seed unprovable.

## T-HOST sketch (threat-first)

Host I/O must **not** start from happy-path `read_file` examples. Required first:

1. Revised threat model (who grants which path).  
2. Capability tokens or explicit host weave catalog with deny-by-default.  
3. Negative corpus: missing grant, path escape, symlink, oversized read.  
4. Interaction with forge ABI and pure fixtures.  

## T-LSP sketch (authority-first)

1. LSP is a **client of** bootstrap diagnostics / structure — not a second seed.  
2. Writes only through existing explicit-path CLI/edit contracts.  
3. Offline only for technical preview lineage.  

## What agents must not do

- Implement any track under this brief alone.  
- Bundle modules + host I/O + LSP in one PR.  
- Mark any track **Proven now** without matrix + delivery report.  
- Weaken project invariants for convenience.

## Next concrete docs (default)

1. ~~M11 modules~~ **Done (0.15)**  
2. ~~M12 fine-grained edits~~ **Done (0.16)**  
3. ~~M13 bounded LSP design~~ **Done (ADR-017)**  
4. **Next:** implement M13a `aether lsp` per ADR-017  

---

*End of portfolio design brief.*
