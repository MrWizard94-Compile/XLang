# Delivery Report — M17b project role:test

**Title:** Aether 0.28.0 `aether project test`  
**Date:** 2026-08-04  

## Shipped

- DESIGN-M17B + ADR-030 + matrix + AETHER_0.28  
- `role: test` in `aether.project/v1`  
- Selectable entry elaboration (`compile_project_entry`)  
- CLI `aether project test <project-file>`  
- stdlib `import_whole_test.ae` imports `whole.ae`  

## Not shipped

- Grants-in-tests  
- JUnit/XML  
- Parallel runners  

## Verify

```powershell
cargo test -p aether-core --lib modules::
cargo test -p aether-cli shipped_stdlib
cargo run -p aether-cli -- project test stdlib/aether.project.json
cargo clippy -p aether-core -p aether-cli -- -D warnings
```

## Constitution

- Design before schema expansion  
- Offline pure run only  
- Fail closed on empty test list  

*End of delivery report.*
