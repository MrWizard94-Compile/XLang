# Aether 0.29 Toolchain Contract (M17c grants-in-tests)

**Status:** Current package — language **0.11** / AETH **v11** + optional test grants  
**Depends on:** ADR-031, M17, M14  

## Surface

```text
aether test [path...] [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]...
aether project test <project-file> [--grant-read <dir>]... ...
```

Default: empty grants (pure). Explicit grants match `aether run` path jail.

## Design-only deferred (not product)

| Track | ADR | Note |
| --- | --- | --- |
| M19c Policy B | ADR-032 | Blocked pending resourceful spawn preconditions |
| M21 FFI | ADR-025 | Blocked pending human implement authorization |

*End of AETHER_0.29.md*
