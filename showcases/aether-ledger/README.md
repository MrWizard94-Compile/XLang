# Aether Integrity Ledger (showcase)

**Status:** Aether-only multi-package + demo showcase (package 0.36 surface)  
**Language:** 100% Aether source under this directory (no Rust product code)  
**Authority:** local-first, verify-before-run, pure default path (no grants)

## What it is

An **offline integrity toolkit for Whole-valued event streams**: polynomial
mixing/digest libraries, multi-package module graph, cooperative nursery
segment hashing, arena buffer packing, and M23 comptime pure calls — composed
into a workspace app that yields a single integrity score.

Companion **single-file demos** add M19e task-frame cancellation, dual-layout
tables, and pure host + `release` — still 100% Aether.

## Layout

```text
showcases/aether-ledger/
  aether.workspace.json
  core/                     digest + policy library package
  engine/                   pipeline + concurrent nursery
  app/                      workspace main entry
  demos/
    effects_and_tasks.ae    M19e task cancel + Error[Whole]
    layout_records_host.ae  M6 dual-layout table
    host_and_release.ae     M8 host + M19a release
  README.md
```

## Features exercised

| Surface | Where |
| --- | --- |
| Multi-package workspace (M18/M22) | workspace + cross-package imports |
| `export weave` libraries | `core/digest.ae`, `engine/*` |
| Polynomial Whole integrity mix | `core/digest.ae` |
| Cooperative nursery (M7) | `engine/concurrent.ae` |
| Arena + Whole buffer packing (M2) | `app/main.ae` |
| M15/M23 comptime pure calls | `app/main.ae` |
| M19e task + checkpoint cancel | `demos/effects_and_tasks.ae` |
| M4 `Error[Whole]` | `demos/effects_and_tasks.ae` |
| M6 dual-layout tables | `demos/layout_records_host.ae` |
| M8 host + M19a release | `demos/host_and_release.ae` |

### Control-flow rule (ADR-070 / showcase lesson)

Truth-condition `choose` **must not** `yield` in a branch (bootstrap and product
both reject; product uses `AE-SEED-013`). Legal multi-module product pattern:

```aether
bind mutable ok <- 0
choose same sealed preview:
  revise ok <- 1
yield ok
```

Resource `choose allocate|append|at|…` may still `yield` in branches. Host/table/
record combos remain denser as single-file demos.

## Build and run

From the repository root:

```powershell
cargo run -q -p aether-cli -- project lock .\showcases\aether-ledger\core\aether.project.json --write
cargo run -q -p aether-cli -- project lock .\showcases\aether-ledger\engine\aether.project.json --write
cargo run -q -p aether-cli -- project lock .\showcases\aether-ledger\app\aether.project.json --write
cargo run -q -p aether-cli -- workspace lock .\showcases\aether-ledger\aether.workspace.json --write

cargo run -q -p aether-cli -- workspace verify .\showcases\aether-ledger\aether.workspace.json
cargo run -q -p aether-cli -- workspace build .\showcases\aether-ledger\aether.workspace.json --package app --output .\target\aether-ledger.aeth
cargo run -q -p aether-cli -- run .\target\aether-ledger.aeth
# Expected exit: 257789099815072090
# (mix includes sealed/conc/head ok flags via ADR-070 choose+revise)
```

### Companion demos

```powershell
cargo run -q -p aether-cli -- compile .\showcases\aether-ledger\demos\effects_and_tasks.ae --output .\target\ledger-effects.aeth
cargo run -q -p aether-cli -- run .\target\ledger-effects.aeth
# Expected exit: 9

cargo run -q -p aether-cli -- compile .\showcases\aether-ledger\demos\layout_records_host.ae --output .\target\ledger-layout.aeth
cargo run -q -p aether-cli -- run .\target\ledger-layout.aeth
# Expected exit: 10

cargo run -q -p aether-cli -- compile .\showcases\aether-ledger\demos\host_and_release.ae --output .\target\ledger-host.aeth
cargo run -q -p aether-cli -- run .\target\ledger-host.aeth
# Expected exit: 48
```

## Design notes

- **Useful:** offline integrity mixing for bounded Whole streams — local lock
  pins, event logs, agent-authored manifests without network or ambient FS.
- **Advanced:** densest multi-package composition that product seed elaborates
  today, plus demos for cancel/layout/host.
- **Not claimed:** cryptographic hash; multi-module seed still residual for some
  control/host forms (documented above).

---

*Aether-only showcase — product path default (BARP).*
