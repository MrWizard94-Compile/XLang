# Aether 0.30 Toolchain Contract (M17d structured test reports)

**Status:** Current package — language **0.11** / AETH **v11** + optional test reports  
**Depends on:** ADR-033, M17  

## Surface

```text
aether test … [--report out.json] [--report-junit out.xml]
aether project test … [--report out.json] [--report-junit out.xml]
```

- JSON schema: `aether.test-report/v1`  
- JUnit: single offline testsuite  
- Default: stdout only  

## Still deferred (not product)

| Track | Status |
| --- | --- |
| M19c Policy B | Design-only (ADR-032); needs resourceful-spawn precondition |
| M19d precondition | Design-only (ADR-034) |
| M21 FFI | Design-only; **human implement authorize** required (ADR-025) |
| Native/registry | Law forks |

*End of AETHER_0.30.md*
