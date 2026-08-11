# ADR-043: Bootstrap Authority Reduction Program (BARP)

**Status:** Accepted (program) — through ADR-064  
**Date:** 2026-08-08 (Phase 1–2 / ADR-044–064 2026-08-10)  
**Related Rule IDs:** `CONST-DEP-001`, `RND-INVAR-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`  
**Design:** [DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)

## Context

Aether product compile already emits via the seed forge ABI. Before Phase 1, Rust
bootstrap still owned parse/validate on the product path and **M23
materialization** (`lower_m23_comptime_calls_for_seed`). Human direction
(2026-08-08): after a live release gate stamp, begin reducing bootstrap
authority under Constitution.

## Decision

1. Adopt DESIGN-BARP-001 as the multi-phase program.  
2. **Do not** weaken dual-compare, verify-before-run, or seed-hosted emission.  
3. **Phase 1 (done):** seed-native M23 pure comptime calls; product path forges
   original source; materialization bridge removed;
   `seed_interprets_m23_comptime_calls_natively() == true`.  
4. **Phase 2 (done; ADR-044):** validate-light product path — forge-first
   product bytecode without bootstrap validate pre-gate;
   `product_path_forges_before_bootstrap_validate() == true`.  
5. **ADR-045 (done):** product multi-module emit without dual-compare product
   gate; `product_path_requires_bootstrap_dual_compare() == false`.  
6. **Phase 3a (done; ADR-046):** bounded product `AE-SEED-*` diagnostics + odd
   indent fail-closed; `seed_product_diagnostics_subset() == true`.  
7. **ADR-047 (done):** multi-module product emit without bootstrap AST parse;
   `product_multi_module_invokes_bootstrap() == false`.  
8. **ADR-048 (done):** structural-edit accept via product seed;
   `structural_edit_accepts_via_product_seed() == true`.  
9. **ADR-049 (done):** `compile_with_seed` product-authoritative;
   gate seed identity includes product rebuild;
   `compile_with_seed_product_authoritative() == true`.  
10. **ADR-050 (done):** Phase 3b product preflight `AE-SEED-005`–`007` + roadmap DOC-SYNC;
    `seed_product_preflight_phase3b() == true`.  
11. **ADR-051 (done):** `compile_with_seed` never invokes bootstrap; CLI
    `check --product` is seed-only; `compile_with_seed_invokes_bootstrap() == false`,
    `product_cli_check_without_bootstrap() == true`.  
12. **ADR-052 (done):** lib module project-verify via product seed;
    Phase 3c `AE-SEED-008`/`010`/`011`; `lib_module_validates_via_product_seed()`,
    `seed_product_diagnostics_phase3c()`.  
13. **ADR-053 (done):** `format --product` seed-only LF+accept; apply-edit CLI
    trusts core product accept; `product_format_without_bootstrap()`,
    `apply_edit_cli_trusts_product_accept()`.  
14. **ADR-054 (done):** `project format --product` + `structure --product`
    (`aether.product-structure/v1`); `product_project_format_without_bootstrap()`,
    `product_structure_without_bootstrap()`.  
15. **ADR-055 (done):** `product_diagnostics` host-facing product diagnostic ABI;
    `AE-SEED-012` raw import unit; `product_diagnostic_abi()`.  
16. **ADR-056 (done):** host elaborate + seed emit multi-module contract;
    `host_elaborates_modules_seed_emits()`; `seed_native_multi_module_elaboration() == false`.  
17. **ADR-057 (done):** structural-edit product base gate when both reject;
    `structural_edit_product_base_gate()`.  
18. **ADR-058 (done):** LSP product diagnostics primary;
    `lsp_product_diagnostics_primary()`.  
19. **ADR-062 (done):** format product base gate; M35b AETH→C Whole locals;
    `format_source_product_base_gate()`, `native_aeth_to_c_locals_pilot()`.  
20. **ADR-063 (done):** default check product base gate; product-surface symbols
    for LSP without bootstrap AST; `check_product_base_gate()`,
    `product_surface_symbols_without_bootstrap()`.  
21. **ADR-064 (done):** product is **default** CLI check/format/structure/project
    format and LSP format; bootstrap is recovery/oracle only
    (`product_default_cli_toolchain()`, `bootstrap_is_recovery_oracle_only()`).  
22. **ADR-065 (done):** product structural weave-replace without bootstrap base
    AST; `structural_edit_product_weave_replace()`.  
23. **ADR-066 (done):** LSP product-surface hover/definition;
    `lsp_product_surface_hover_definition()`.  
24. **ADR-067 (done):** product seed rebuild without `--bootstrap`;
    `product_seed_rebuild_without_bootstrap()`.  
25. **ADR-068 (done):** product top-level weave insertAfter/delete (with replace);
    `structural_edit_product_top_level_weave_ops()`.  
26. **ADR-069 (done):** product weave-body statement ops + primitive record ops;
    `structural_edit_product_statement_and_record_ops()`.  
27. **ADR-070 (done):** product fail-closed yield-in-truth-choose (`AE-SEED-013`);
    multi-module choose+revise supported (`product_rejects_yield_in_truth_choose`,
    `multi_module_product_choose_revise_supported`). Showcase-driven.  
28. **ADR-071 (done):** product nested choose/while body-list statement ops;
    `structural_edit_product_nested_body_list_ops()`.  
29. **ADR-072 (done):** product seed-error packet ABI (`product_error_packets`);
    seed-binary emit still false.  
30. **ADR-075 (done):** SPEAK-format product errors; multi-source envelope +
    in-memory multi-unit product forge (host elaborate + seed emit).  
31. **ADR-078 (done):** product compile auto-accepts multi-source envelopes;
    seed SPEAK diagnostic merge.  
32. **ADR-082 (done):** stable SPEAK packet protocol; product multi-file host path.  
33. Residual bootstrap: dual-compare oracle, recovery flags, full `aether.ast/v8`.  
34. Each phase ships with matrix green, DOC-SYNC, and honest Seed Profile claims.

## Consequences

### Positive

- Clear ladder toward seed-owned product validation/emission  
- Honest accounting of remaining Rust authority  

### Costs

- Seed complexity for M23 interpreter  
- Careful dual-compare during transition  

### Risks

| Risk | Mitigation |
| --- | --- |
| Silent loss of dual-compare | Forbidden by this ADR |
| Seed accepts invalid M23 | Fail closed + bootstrap oracle tests |
| Scope creep to full seed diagnostics | Phase 3 separate ADR |

## Links

- DESIGN-BARP-001, ADR-039, ADR-044–064, SEED_PROFILE  

---

*End of ADR-043.*
