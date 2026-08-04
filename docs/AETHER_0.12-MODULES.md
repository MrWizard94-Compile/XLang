# Aether 0.12 language surface — modules (M11a)

**Status:** Implemented for **bootstrap multi-module project build**  
**Toolchain package:** **0.14.0**  
**AETH:** still **v11** (single linked program after elaboration)  
**Seed:** multi-module **not** seed-hosted yet (**M11b**)

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
- Lib units: no `main`; weaves only (no records/shapes/host in M11a).  
- Entry unit: exactly one `main`.  
- Single-file `aether compile` rejects `import unit` and qualified calls.  

## CLI

```text
aether project build <project.json> --output out.aeth
```

Uses **bootstrap** multi-source elaboration (ADR-015 M11a).

See [DESIGN-M11-LANGUAGE-MODULES.md](DESIGN-M11-LANGUAGE-MODULES.md).
