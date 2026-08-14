# Aether modules surface — M11 complete (M11a + M11b)

**Status:** Implemented  
**Toolchain package:** **0.15.0**  
**AETH:** **v11** (single linked program after host elaboration)  
**Product path:** host elaborates import DAG → **seed-compiles** elaboration  
**Proof:** seed ≡ bootstrap on elaborated source (M11b dual-compare)

## Forms

```text
import unit "<project-relative.ae>" as <alias>
export weave <name> [params] -> Type:
  ...
call <alias>.<name> <args>
```

## Rules (summary)

- Imports only immediately after `world`.  
- Import paths must be project units; graph is a DAG.  
- Only `export weave` is visible across units.  
- Lib units: no `main`; weaves only (no records/shapes/host in M11).  
- Entry unit: exactly one `main`.  
- Single-file `aether compile` rejects `import unit` and qualified calls.  

## CLI

```text
aether project build <project.json> --output out.aeth
```

Elaborates the graph, dual-compares bootstrap vs seed on the elaboration, writes
the **seed** AETH (ADR-015 M11b).

See [DESIGN-M11-LANGUAGE-MODULES.md](DESIGN-M11-LANGUAGE-MODULES.md).
