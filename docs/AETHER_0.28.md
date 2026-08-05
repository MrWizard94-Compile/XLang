# Aether 0.28 Toolchain Contract (M17b project test)

**Status:** Current package — language **0.11** / AETH **v11** + project `role: test`  
**Depends on:** ADR-030, M17, M11  

## Surface

```json
{ "path": "import_whole_test.ae", "role": "test" }
```

```text
aether project test <project-file>
```

Each test unit is a total program with `main`, may import project **lib** units,
is elaborated as the entry cone (seed≡bootstrap), pure-run, pass on exit **0**.

Standalone `aether test` discovery is unchanged.

## Non-goals

Grants-in-tests, JUnit, parallel runners.

*End of AETHER_0.28.md*
