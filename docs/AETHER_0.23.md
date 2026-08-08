# Aether 0.23 Toolchain Contract (M18 offline workspace)

**Status:** Historical package contract — language surface remains **0.11** / AETH **v11**; package **0.23** adds multi-package workspaces; current package contract is [AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** M9–M11 project tooling, [ADR-022](ADR-022-m18-offline-workspace.md)

## Purpose

Offline **workspace** documents list multiple local packages (each an
`aether.project/v1` root) with optional path dependency edges for verify order.

## Document

```json
{
  "schema": "aether.workspace/v1",
  "name": "workspace_demo",
  "version": "0.1.0",
  "packages": [
    { "name": "util", "path": "util" },
    { "name": "app", "path": "app", "depends_on": ["util"] }
  ]
}
```

## CLI

```text
aether workspace verify <workspace-file>
```

Path-jails package directories under the workspace root, rejects cycles, and
runs project verify per package in topological order.

## Non-goals

Network registry, cross-package language `import`, version ranges.

## Evidence

- Design: [DESIGN-M18-OFFLINE-WORKSPACE.md](DESIGN-M18-OFFLINE-WORKSPACE.md)
- Example: `examples/workspace/`

*End of AETHER_0.23.md*
