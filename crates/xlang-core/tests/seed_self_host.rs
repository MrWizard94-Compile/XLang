use aether_core::{
    apply_edit_cli_trusts_product_accept, bootstrap_is_recovery_oracle_only,
    check_product_base_gate, compile_product_bytecode, compile_to_bytecode, compile_with_seed,
    compile_with_seed_invokes_bootstrap, compile_with_seed_product_authoritative, forge_bytecode,
    host_elaborates_modules_seed_emits, lib_module_validates_via_product_seed,
    lsp_product_diagnostics_primary, lsp_product_surface_hover_definition,
    product_cli_check_without_bootstrap, product_default_cli_toolchain, product_diagnostic_abi,
    product_diagnostics, product_format_without_bootstrap, product_multi_module_invokes_bootstrap,
    product_path_forges_before_bootstrap_validate, product_path_requires_bootstrap_dual_compare,
    product_project_format_without_bootstrap, product_seed_rebuild_without_bootstrap,
    product_structure_without_bootstrap, product_surface_symbols,
    product_surface_symbols_without_bootstrap, run_bytecode,
    seed_interprets_m23_comptime_calls_natively, seed_native_multi_module_elaboration,
    seed_product_diagnostics_phase3c, seed_product_diagnostics_subset,
    seed_product_preflight_phase3b, structural_edit_accepts_via_product_seed,
    structural_edit_product_base_gate, structural_edit_product_weave_replace, verify_bytecode,
    InvocationOutput, InvocationValue, SEED_COMPILER_ARTIFACT,
};

const SEED_SOURCE: &str = include_str!("../../../seed/aether_seed.ae");
const CHECKED_IN_SEED_ARTIFACT: &[u8] = include_bytes!("../../../seed/aether_seed.aeth");

const MULTI_WEAVE_SOURCE: &str = concat!(
    "world demo\n",
    "weave double [v0: Whole] -> Whole:\n",
    "  yield product v0 2\n",
    "weave main [] -> Whole:\n",
    "  bind mutable v0 <- call double 21\n",
    "  speak render v0\n",
    "  yield v0\n",
);

/// Callee declared after the call site (pass-1 name table).
const FORWARD_CALL_SOURCE: &str = concat!(
    "world demo\n",
    "weave main [] -> Whole:\n",
    "  bind mutable v0 <- call double 21\n",
    "  speak render v0\n",
    "  yield v0\n",
    "weave double [v0: Whole] -> Whole:\n",
    "  yield product v0 2\n",
);

/// A task may be declared after the nursery that spawns it; both compiler
/// passes must preserve the declaration's task-frame descriptor and capacity.
const M19E_FORWARD_TASK_SOURCE: &str = concat!(
    "world task_forward\n",
    "weave main [] -> Whole:\n",
    "  bind mutable result <- 0\n",
    "  together:\n",
    "    spawn call later into result\n",
    "  yield result\n",
    "\n",
    "task weave later [] -> Whole:\n",
    "  bind memory <- arena 32\n",
    "  checkpoint\n",
    "  release memory\n",
    "  yield 4\n",
);

const M4_ERROR_EFFECT_SOURCE: &str = include_str!("../../../examples/error-effect.ae");
const M5_COMPTIME_SOURCE: &str = include_str!("../../../examples/comptime.ae");
const M15_COMPTIME_CHAIN_SOURCE: &str = include_str!("../../../examples/comptime-chain.ae");
const M23_COMPTIME_CALL_SOURCE: &str = include_str!("../../../examples/comptime-calls.ae");
const M16_RESOURCE_HANDLE_SOURCE: &str = include_str!("../../../examples/resource-handle.ae");
const M19A_RELEASE_RAISE_SOURCE: &str = include_str!("../../../examples/release-raise.ae");
const M19B_NURSERY_RESOURCE_SOURCE: &str = include_str!("../../../examples/nursery-resource.ae");
const M19D_SPAWN_ARENA_SOURCE: &str = include_str!("../../../examples/spawn-arena.ae");
const M19E_ACTIVE_CANCEL_SOURCE: &str = include_str!("../../../examples/active-cancel.ae");
const M19E_TASK_FRAME_CAPACITY_SOURCE: &str =
    include_str!("../../../examples/task-frame-capacity.ae");
const M19E_TASK_LOOP_SOURCE: &str = include_str!("../../../examples/task-loop.ae");
const M6_LAYOUT_SOURCE: &str = include_str!("../../../examples/layout-table.ae");
const M7_NURSERY_TOTAL_SOURCE: &str = include_str!("../../../examples/nursery-total.ae");
const M7_NURSERY_CANCEL_SOURCE: &str = include_str!("../../../examples/nursery-cancel.ae");
const M8_HOST_PILOT_SOURCE: &str = include_str!("../../../examples/host-pilot.ae");
const M14_HOST_IO_READ_SOURCE: &str = include_str!("../../../examples/host-io-read.ae");
const M14_HOST_IO_WRITE_SOURCE: &str = include_str!("../../../examples/host-io-write.ae");
const M21_FOREIGN_PILOT_SOURCE: &str = include_str!("../../../examples/foreign-pilot.ae");
const M21_FOREIGN_SUM_SOURCE: &str = include_str!("../../../examples/foreign-sum.ae");

const M4_NORMAL_EFFECT_SOURCE: &str = concat!(
    "world normal_effect\n",
    "\n",
    "weave may_succeed [] -> Whole raises Whole:\n",
    "  yield 23\n",
    "\n",
    "weave main [] -> Whole:\n",
    "  bind mutable value <- 0\n",
    "  bind mutable code <- 0\n",
    "  handle call may_succeed into value otherwise error into code\n",
);

#[test]
fn seed_profile_compiler_rebuilds_itself_and_a_distinct_valid_variant() {
    let bootstrap = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    assert_eq!(
        bootstrap, CHECKED_IN_SEED_ARTIFACT,
        "the checked-in seed artifact must match the deterministic bootstrap output"
    );

    let forged = bytes(
        forge_bytecode(&bootstrap, SEED_SOURCE)
            .expect("the bootstrap artifact must forge the seed source"),
    );
    verify_bytecode(&forged).expect("the Aether-produced seed artifact must verify");
    assert_eq!(forged, bootstrap, "seed rebuild must be byte-reproducible");

    let rebuilt = bytes(
        forge_bytecode(&forged, SEED_SOURCE)
            .expect("the Aether-produced artifact must forge its own source"),
    );
    assert_eq!(
        rebuilt, bootstrap,
        "the second self-hosting generation must match"
    );

    // Insert a fresh unused local so the variant differs without colliding with
    // the seed's existing high-numbered slots (v103 through v130 are bound for
    // M23 body-interpreter temps / BARP Phase 1).
    let variant = if SEED_SOURCE.contains("  bind mutable v65 <- 0\r\n") {
        SEED_SOURCE.replacen(
            "  bind mutable v65 <- 0\r\n",
            "  bind mutable v65 <- 0\r\n  bind mutable v131 <- 0\r\n",
            1,
        )
    } else {
        SEED_SOURCE.replacen(
            "  bind mutable v65 <- 0\n",
            "  bind mutable v65 <- 0\n  bind mutable v131 <- 0\n",
            1,
        )
    };
    assert_ne!(
        variant, SEED_SOURCE,
        "the variant fixture must differ from the seed"
    );
    let variant_bootstrap = compile_to_bytecode(&variant)
        .expect("the distinct Seed Profile variant must bootstrap")
        .bytecode;
    let variant_forged = bytes(
        forge_bytecode(&forged, &variant)
            .expect("the Aether compiler must forge a distinct Seed Profile program"),
    );
    verify_bytecode(&variant_forged).expect("the distinct Aether-produced artifact must verify");
    assert_eq!(variant_forged, variant_bootstrap);
    assert_ne!(
        variant_forged, bootstrap,
        "the compiler must not return a fixed artifact"
    );
}

#[test]
fn seed_profile_compiler_forges_multi_weave_calls_byte_identically() {
    let bootstrap = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    let multi_bootstrap = compile_to_bytecode(MULTI_WEAVE_SOURCE)
        .expect("the multi-weave Seed Profile fixture must bootstrap")
        .bytecode;
    let multi_forged = bytes(
        forge_bytecode(&bootstrap, MULTI_WEAVE_SOURCE)
            .expect("the seed compiler must forge multi-weave Seed Profile programs"),
    );
    verify_bytecode(&multi_forged).expect("the multi-weave Aether-produced artifact must verify");
    assert_eq!(
        multi_forged, multi_bootstrap,
        "seed multi-weave forge must match bootstrap byte-for-byte"
    );
    assert_ne!(
        multi_forged, bootstrap,
        "the multi-weave fixture must not collapse to the seed artifact"
    );

    let run = run_bytecode(&multi_forged).expect("the multi-weave artifact must run");
    assert_eq!(run.exit_code, 42);
    assert_eq!(run.stdout, "42");
}

#[test]
fn seed_profile_compiler_forges_forward_calls_and_crlf_sources() {
    let bootstrap = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;

    let forward_bootstrap = compile_to_bytecode(FORWARD_CALL_SOURCE)
        .expect("the forward-call Seed Profile fixture must bootstrap")
        .bytecode;
    let forward_forged = bytes(
        forge_bytecode(&bootstrap, FORWARD_CALL_SOURCE)
            .expect("the seed compiler must forge forward call programs"),
    );
    verify_bytecode(&forward_forged).expect("the forward-call artifact must verify");
    assert_eq!(
        forward_forged, forward_bootstrap,
        "seed forward-call forge must match bootstrap byte-for-byte"
    );
    let run = run_bytecode(&forward_forged).expect("the forward-call artifact must run");
    assert_eq!(run.exit_code, 42);
    assert_eq!(run.stdout, "42");

    let crlf = MULTI_WEAVE_SOURCE.replace('\n', "\r\n");
    assert!(crlf.contains("\r\n"), "fixture must exercise CRLF newlines");
    let lf_bootstrap = compile_to_bytecode(MULTI_WEAVE_SOURCE)
        .expect("the LF multi-weave fixture must bootstrap")
        .bytecode;
    let crlf_forged = bytes(
        forge_bytecode(&bootstrap, &crlf)
            .expect("the seed compiler must accept CRLF Seed Profile sources"),
    );
    verify_bytecode(&crlf_forged).expect("the CRLF-forged artifact must verify");
    assert_eq!(
        crlf_forged, lf_bootstrap,
        "CRLF Seed Profile forge must match the LF bootstrap artifact"
    );
    let crlf_run = run_bytecode(&crlf_forged).expect("the CRLF-forged artifact must run");
    assert_eq!(crlf_run.exit_code, 42);
    assert_eq!(crlf_run.stdout, "42");
}

#[test]
fn seed_profile_compiler_forges_bounded_m4_error_effects_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    for (name, source, expected_exit) in [
        ("M4 error exit", M4_ERROR_EFFECT_SOURCE, 17),
        ("M4 normal exit", M4_NORMAL_EFFECT_SOURCE, 23),
    ] {
        let bootstrap = compile_to_bytecode(source)
            .unwrap_or_else(|error| panic!("{name} fixture must bootstrap: {error}"))
            .bytecode;
        let forged = bytes(
            forge_bytecode(&bootstrap_seed, source).unwrap_or_else(|error| {
                panic!("{name} fixture must forge through the seed: {error}")
            }),
        );
        verify_bytecode(&forged)
            .unwrap_or_else(|error| panic!("{name} forged artifact must verify: {error}"));
        assert_eq!(
            forged, bootstrap,
            "{name} must match bootstrap byte-for-byte"
        );
        assert_eq!(
            run_bytecode(&forged)
                .unwrap_or_else(|error| panic!("{name} forged artifact must run: {error}"))
                .exit_code,
            expected_exit,
            "{name} should preserve its documented outcome"
        );
    }
}

#[test]
fn seed_profile_compiler_forges_bounded_m5_comptime_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    let bootstrap = compile_to_bytecode(M5_COMPTIME_SOURCE)
        .expect("the M5 comptime fixture must bootstrap")
        .bytecode;
    let forged = bytes(
        forge_bytecode(&bootstrap_seed, M5_COMPTIME_SOURCE)
            .expect("the M5 comptime fixture must forge through the seed"),
    );
    verify_bytecode(&forged).expect("the M5 seed-produced artifact must verify");
    assert_eq!(
        forged, bootstrap,
        "the M5 comptime fixture must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&forged)
            .expect("the M5 seed-produced artifact must run")
            .exit_code,
        150,
        "the M5 comptime fixture should preserve its defined outcome"
    );
}

#[test]
fn seed_profile_compiler_forges_m16_resource_handle_byte_identically() {
    let bootstrap = compile_to_bytecode(M16_RESOURCE_HANDLE_SOURCE)
        .expect("M16 resource-handle must bootstrap")
        .bytecode;
    let seeded = compile_with_seed(M16_RESOURCE_HANDLE_SOURCE)
        .expect("M16 resource-handle must seed-compile")
        .bytecode;
    verify_bytecode(&seeded).expect("M16 seed artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "M16 resource-handle must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&seeded)
            .expect("M16 seed artifact must run")
            .exit_code,
        7
    );
}

#[test]
fn seed_profile_compiler_forges_m15_comptime_chain_byte_identically() {
    let bootstrap = compile_to_bytecode(M15_COMPTIME_CHAIN_SOURCE)
        .expect("the M15 chain fixture must bootstrap")
        .bytecode;
    let seeded = compile_with_seed(M15_COMPTIME_CHAIN_SOURCE)
        .expect("the M15 chain fixture must seed-compile")
        .bytecode;
    verify_bytecode(&seeded).expect("M15 seed-produced artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "M15 comptime chain must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&seeded)
            .expect("M15 seed-produced chain must run")
            .exit_code,
        288,
        "cell=64 row=256 header=32 total=288"
    );
}

#[test]
fn seed_hosted_product_path_forges_m23_comptime_calls_byte_identically() {
    let bootstrap = compile_to_bytecode(M23_COMPTIME_CALL_SOURCE)
        .expect("the M23 comptime-call fixture must bootstrap")
        .bytecode;
    let seeded = compile_with_seed(M23_COMPTIME_CALL_SOURCE)
        .expect("the M23 comptime-call fixture must compile through the seed product path")
        .bytecode;
    verify_bytecode(&seeded).expect("M23 seed-produced artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "M23 seed-native evaluation and bootstrap emission must match byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&seeded)
            .expect("M23 seed-produced artifact must run")
            .exit_code,
        512,
        "M23 comptime helper chain should exit 512"
    );
}

#[test]
fn barp_phase2_product_bytecode_forges_without_bootstrap_prevalidate() {
    assert!(
        seed_interprets_m23_comptime_calls_natively(),
        "Phase 1 tracker must remain true"
    );
    assert!(
        product_path_forges_before_bootstrap_validate(),
        "Phase 2 tracker must be true"
    );
    assert!(
        !product_path_requires_bootstrap_dual_compare(),
        "ADR-045: product path must not require dual-compare gates"
    );
    assert!(
        seed_product_diagnostics_subset(),
        "Phase 3a diagnostics subset tracker must be true"
    );
    assert!(
        !product_multi_module_invokes_bootstrap(),
        "ADR-047: multi-module product path must not invoke bootstrap"
    );
    assert!(
        structural_edit_accepts_via_product_seed(),
        "ADR-048: structural edit accept via product seed"
    );
    assert!(
        compile_with_seed_product_authoritative(),
        "ADR-049: compile_with_seed product-authoritative"
    );
    assert!(
        seed_product_preflight_phase3b(),
        "ADR-050: Phase 3b product preflight expansion"
    );
    assert!(
        !compile_with_seed_invokes_bootstrap(),
        "ADR-051: compile_with_seed must not invoke bootstrap"
    );
    assert!(
        product_cli_check_without_bootstrap(),
        "ADR-051: product CLI check without bootstrap"
    );
    assert!(
        lib_module_validates_via_product_seed(),
        "ADR-052: lib modules validate via product seed"
    );
    assert!(
        seed_product_diagnostics_phase3c(),
        "ADR-052: Phase 3c product diagnostic classification"
    );
    assert!(
        product_format_without_bootstrap(),
        "ADR-053: product format without bootstrap"
    );
    assert!(
        apply_edit_cli_trusts_product_accept(),
        "ADR-053: apply-edit CLI trusts product accept"
    );
    assert!(
        product_project_format_without_bootstrap(),
        "ADR-054: product project format without bootstrap"
    );
    assert!(
        product_structure_without_bootstrap(),
        "ADR-054: product structure without bootstrap"
    );
    assert!(product_diagnostic_abi(), "ADR-055: product diagnostic ABI");
    assert!(
        host_elaborates_modules_seed_emits(),
        "ADR-056: host elaborates modules; seed emits"
    );
    assert!(
        !seed_native_multi_module_elaboration(),
        "ADR-056 honesty: no seed-native multi-module elaboration"
    );
    assert!(
        structural_edit_product_base_gate(),
        "ADR-057: structural edit product base gate"
    );
    assert!(
        lsp_product_diagnostics_primary(),
        "ADR-058: LSP product diagnostics primary"
    );
    assert!(
        check_product_base_gate(),
        "ADR-063: check product base gate"
    );
    assert!(
        product_surface_symbols_without_bootstrap(),
        "ADR-063: product surface symbols without bootstrap"
    );
    assert!(
        product_default_cli_toolchain(),
        "ADR-064: product is default CLI toolchain"
    );
    assert!(
        bootstrap_is_recovery_oracle_only(),
        "ADR-064: bootstrap is recovery/oracle only"
    );
    assert!(
        structural_edit_product_weave_replace(),
        "ADR-065: product weave replace without bootstrap base AST"
    );
    assert!(
        lsp_product_surface_hover_definition(),
        "ADR-066: LSP product-surface hover/definition"
    );
    assert!(
        product_seed_rebuild_without_bootstrap(),
        "ADR-067: product seed rebuild without --bootstrap"
    );
    let bootstrap = compile_to_bytecode(M23_COMPTIME_CALL_SOURCE)
        .expect("M23 fixture must bootstrap")
        .bytecode;
    let product = compile_product_bytecode(M23_COMPTIME_CALL_SOURCE)
        .expect("product bytecode path must forge M23 without bootstrap pre-validate");
    verify_bytecode(&product).expect("product bytecode must verify");
    assert_eq!(
        product, bootstrap,
        "forge-first product path must dual-compare with bootstrap"
    );
    assert_eq!(
        run_bytecode(&product)
            .expect("product bytecode must run")
            .exit_code,
        512
    );
}

#[test]
fn barp_phase3a_product_path_rejects_odd_indent_with_ae_seed_code() {
    let source = "world w\n\nweave main [] -> Whole:\n yield 1\n";
    let error = compile_product_bytecode(source).expect_err("odd indent must fail closed");
    let message = error.to_string();
    assert!(
        message.contains("AE-SEED-003"),
        "expected AE-SEED-003, got {message}"
    );
    assert!(
        message.contains("aether check"),
        "expected check hint, got {message}"
    );
}

#[test]
fn barp_phase3a_product_path_maps_seed_forge_failure_to_ae_seed_code() {
    let source = "world w\n\nweave main [] -> Whole:\n  yield missing_name\n";
    let error = compile_product_bytecode(source).expect_err("unbound name must fail product path");
    let message = error.to_string();
    assert!(
        message.contains("AE-SEED-008")
            || message.contains("AE-SEED-001")
            || message.contains("AE-SEED-002"),
        "expected AE-SEED-008/001/002 for unbound, got {message}"
    );
    assert!(
        message.contains("aether check"),
        "expected check hint, got {message}"
    );
}

#[test]
fn barp_phase4_product_diagnostic_abi_and_import_unit() {
    assert!(product_diagnostic_abi());
    let good = product_diagnostics("world w\n\nweave main [] -> Whole:\n  yield 1\n");
    assert!(good.is_empty(), "valid product source has no diagnostics");
    let type_diags = product_diagnostics("world w\n\nweave main [] -> Whole:\n  yield \"x\"\n");
    assert_eq!(type_diags.len(), 1);
    assert_eq!(type_diags[0].code, "AE-SEED-010");
    let import_diags = product_diagnostics(
        "world w\n\nimport unit \"lib.ae\" as lib\n\nweave main [] -> Whole:\n  yield 1\n",
    );
    assert_eq!(import_diags.len(), 1);
    assert_eq!(import_diags[0].code, "AE-SEED-012");
}

#[test]
fn barp_phase5_product_surface_symbols_without_bootstrap() {
    assert!(product_surface_symbols_without_bootstrap());
    let symbols = product_surface_symbols(
        "world demo\n\nweave helper [] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  yield 2\n",
    )
    .expect("product symbols");
    let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"demo"));
    assert!(names.contains(&"helper"));
    assert!(names.contains(&"main"));
    assert!(symbols
        .iter()
        .any(|s| s.kind == "weave" && s.name == "main"));
}

#[test]
fn barp_phase3c_classifies_type_and_unknown_weave_product_failures() {
    assert!(
        seed_product_diagnostics_phase3c(),
        "Phase 3c tracker must be true"
    );
    let type_err =
        compile_product_bytecode("world w\n\nweave main [] -> Whole:\n  yield \"text\"\n")
            .expect_err("type mismatch must fail product path");
    assert!(
        type_err.to_string().contains("AE-SEED-010"),
        "expected AE-SEED-010, got {type_err}"
    );

    let unknown = compile_product_bytecode(
        "world w\n\nweave main [] -> Whole:\n  bind x <- call nope 1\n  yield x\n",
    )
    .expect_err("unknown weave must fail product path");
    assert!(
        unknown.to_string().contains("AE-SEED-011"),
        "expected AE-SEED-011, got {unknown}"
    );

    let unbound =
        compile_product_bytecode("world w\n\nweave main [] -> Whole:\n  yield missing_name\n")
            .expect_err("unbound must fail");
    assert!(
        unbound.to_string().contains("AE-SEED-008"),
        "expected AE-SEED-008 for opaque seed bind failure, got {unbound}"
    );
}

#[test]
fn barp_phase3b_product_path_rejects_missing_main_with_ae_seed_004() {
    let source = "world w\n\nweave helper [] -> Whole:\n  yield 1\n";
    let error = compile_product_bytecode(source).expect_err("missing main must fail closed");
    let message = error.to_string();
    assert!(
        message.contains("AE-SEED-004"),
        "expected AE-SEED-004, got {message}"
    );
}

#[test]
fn barp_phase3b_product_path_rejects_empty_missing_world_and_legacy() {
    let empty = compile_product_bytecode("   \n\t\n").expect_err("empty must fail");
    assert!(empty.to_string().contains("AE-SEED-005"), "got {}", empty);

    let no_world = compile_product_bytecode("weave main [] -> Whole:\n  yield 1\n")
        .expect_err("missing world must fail");
    assert!(
        no_world.to_string().contains("AE-SEED-006"),
        "got {}",
        no_world
    );

    let legacy = compile_product_bytecode("world w\n\nfn main() -> Int { return 0; }\n")
        .expect_err("legacy syntax must fail");
    assert!(legacy.to_string().contains("AE-SEED-007"), "got {}", legacy);

    // ADR-055/056: raw import unit fails closed with AE-SEED-012 (not AE-SEED-007).
    let import_unit = concat!(
        "world demo\n\n",
        "import unit \"lib.ae\" as lib\n\n",
        "weave main [] -> Whole:\n  yield 1\n"
    );
    let import_err =
        compile_product_bytecode(import_unit).expect_err("raw import unit must fail product");
    assert!(
        import_err.to_string().contains("AE-SEED-012"),
        "expected AE-SEED-012, got {import_err}"
    );
    assert!(
        !import_err.to_string().contains("AE-SEED-007"),
        "import unit must not trip legacy preflight: {import_err}"
    );
}

#[test]
fn product_path_rebuilds_seed_compiler_byte_identically() {
    // Aether independence: the embedded seed forges its own source to the
    // checked-in artifact without bootstrap emit (oracle still dual-compared).
    let product =
        compile_product_bytecode(SEED_SOURCE).expect("product path must forge the seed source");
    assert_eq!(
        product.as_slice(),
        CHECKED_IN_SEED_ARTIFACT,
        "product seed rebuild must match the checked-in seed artifact"
    );
}

#[test]
fn seed_profile_compiler_forges_m19a_release_raise_byte_identically() {
    let bootstrap = compile_to_bytecode(M19A_RELEASE_RAISE_SOURCE)
        .expect("M19a release-raise must bootstrap")
        .bytecode;
    let seeded = compile_with_seed(M19A_RELEASE_RAISE_SOURCE)
        .expect("M19a release-raise must seed-compile")
        .bytecode;
    verify_bytecode(&seeded).expect("M19a seed artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "M19a release-raise must match bootstrap byte-for-byte"
    );
    assert!(seeded.contains(&66), "seed path must emit OP_RELEASE (66)");
    assert_eq!(
        run_bytecode(&seeded)
            .expect("M19a seed artifact must run")
            .exit_code,
        9,
        "handled raise after release should exit 9"
    );
}

#[test]
fn seed_profile_compiler_forges_m19b_nursery_resource_byte_identically() {
    let bootstrap = compile_to_bytecode(M19B_NURSERY_RESOURCE_SOURCE)
        .expect("M19b nursery-resource must bootstrap")
        .bytecode;
    let seeded = compile_with_seed(M19B_NURSERY_RESOURCE_SOURCE)
        .expect("M19b nursery-resource must seed-compile")
        .bytecode;
    verify_bytecode(&seeded).expect("M19b seed artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "M19b nursery-resource must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&seeded)
            .expect("M19b seed artifact must run")
            .exit_code,
        7
    );
}

#[test]
fn seed_profile_compiler_forges_m19e_active_task_frames_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    for (name, source, expected_exit, expected_capacity) in [
        ("active cancellation", M19E_ACTIVE_CANCEL_SOURCE, 9, 64_u32),
        (
            "multi-task frame capacity",
            M19E_TASK_FRAME_CAPACITY_SOURCE,
            3,
            96_u32,
        ),
        ("checkpointed task loop", M19E_TASK_LOOP_SOURCE, 3, 0_u32),
        (
            "forward-declared task frame",
            M19E_FORWARD_TASK_SOURCE,
            4,
            32_u32,
        ),
    ] {
        let bootstrap = compile_to_bytecode(source)
            .unwrap_or_else(|error| panic!("M19e {name} must bootstrap: {error}"))
            .bytecode;
        let forged = bytes(
            forge_bytecode(&bootstrap_seed, source)
                .unwrap_or_else(|error| panic!("M19e {name} must forge through the seed: {error}")),
        );
        verify_bytecode(&forged)
            .unwrap_or_else(|error| panic!("M19e {name} seed artifact must verify: {error}"));
        assert_eq!(
            forged, bootstrap,
            "M19e {name} must match bootstrap byte-for-byte"
        );
        assert_eq!(forged[4], 12, "M19e {name} must emit AETH v12");
        assert_eq!(
            u32::from_le_bytes(forged[5..9].try_into().expect("AETH v12 capacity header")),
            expected_capacity,
            "M19e {name} must encode its exact concurrent frame plan"
        );
        assert_eq!(
            run_bytecode(&forged)
                .unwrap_or_else(|error| panic!("M19e {name} seed artifact must run: {error}"))
                .exit_code,
            expected_exit,
            "M19e {name} must preserve its documented runtime outcome"
        );
    }
}

#[test]
fn seed_profile_compiler_forges_bounded_m6_layout_tables_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    let bootstrap = compile_to_bytecode(M6_LAYOUT_SOURCE)
        .expect("the M6 layout fixture must bootstrap")
        .bytecode;
    let forged = bytes(
        forge_bytecode(&bootstrap_seed, M6_LAYOUT_SOURCE)
            .expect("the M6 layout fixture must forge through the seed"),
    );
    verify_bytecode(&forged).expect("the M6 seed-produced artifact must verify");
    assert_eq!(
        forged, bootstrap,
        "the M6 layout fixture must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&forged)
            .expect("the M6 seed-produced artifact must run")
            .exit_code,
        10,
        "the M6 layout fixture should preserve its defined outcome"
    );
}

#[test]
fn seed_profile_compiler_forges_bounded_m7_nurseries_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;

    let total_bootstrap = compile_to_bytecode(M7_NURSERY_TOTAL_SOURCE)
        .expect("the M7 total nursery fixture must bootstrap")
        .bytecode;
    let total_forged = bytes(
        forge_bytecode(&bootstrap_seed, M7_NURSERY_TOTAL_SOURCE)
            .expect("the M7 total nursery fixture must forge through the seed"),
    );
    verify_bytecode(&total_forged).expect("the M7 total seed-produced artifact must verify");
    assert_eq!(
        total_forged, total_bootstrap,
        "the M7 total nursery fixture must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&total_forged)
            .expect("the M7 total seed-produced artifact must run")
            .exit_code,
        7,
        "the M7 total nursery fixture should preserve its defined outcome"
    );

    let cancel_bootstrap = compile_to_bytecode(M7_NURSERY_CANCEL_SOURCE)
        .expect("the M7 cancel nursery fixture must bootstrap")
        .bytecode;
    let cancel_forged = bytes(
        forge_bytecode(&bootstrap_seed, M7_NURSERY_CANCEL_SOURCE)
            .expect("the M7 cancel nursery fixture must forge through the seed"),
    );
    verify_bytecode(&cancel_forged).expect("the M7 cancel seed-produced artifact must verify");
    assert_eq!(
        cancel_forged, cancel_bootstrap,
        "the M7 cancel nursery fixture must match bootstrap byte-for-byte"
    );
    assert_eq!(
        run_bytecode(&cancel_forged)
            .expect("the M7 cancel seed-produced artifact must run")
            .exit_code,
        9,
        "the M7 cancel nursery fixture should preserve its defined outcome"
    );
}

#[test]
fn seed_profile_compiler_forges_bounded_m8_host_pilot_byte_identically() {
    let bootstrap_seed = compile_to_bytecode(SEED_SOURCE)
        .expect("the checked-in Aether seed source must bootstrap")
        .bytecode;
    let bootstrap = compile_to_bytecode(M8_HOST_PILOT_SOURCE)
        .expect("the M8 host-pilot fixture must bootstrap")
        .bytecode;
    let forged = bytes(
        forge_bytecode(&bootstrap_seed, M8_HOST_PILOT_SOURCE)
            .expect("the M8 host-pilot fixture must forge through the seed"),
    );
    verify_bytecode(&forged).expect("the M8 seed-produced artifact must verify");
    assert_eq!(
        forged, bootstrap,
        "the M8 host-pilot fixture must match bootstrap byte-for-byte"
    );
    assert!(
        forged.contains(&65),
        "seed host-pilot forge must emit HOST_CALL opcode 65"
    );
    assert_eq!(
        run_bytecode(&forged)
            .expect("the M8 seed-produced artifact must run")
            .exit_code,
        48,
        "the M8 host-pilot fixture should preserve its defined outcome"
    );
}

#[test]
fn seed_profile_compiler_forges_m14_host_io_declarations_byte_identically() {
    // Compile/dual-compare only: I/O host weaves require grants at run time.
    for (name, source) in [
        ("host-io-read", M14_HOST_IO_READ_SOURCE),
        ("host-io-write", M14_HOST_IO_WRITE_SOURCE),
    ] {
        let bootstrap = compile_to_bytecode(source)
            .unwrap_or_else(|error| panic!("M14 {name} must bootstrap: {error}"))
            .bytecode;
        let seeded = compile_with_seed(source)
            .unwrap_or_else(|error| panic!("M14 {name} must seed-compile: {error}"))
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "M14 {name} seed-hosted compile must match bootstrap byte-for-byte"
        );
        verify_bytecode(&seeded).expect("M14 seed-produced artifact must verify");
        assert!(seeded.contains(&65), "M14 {name} must emit HOST_CALL");
        let denied = run_bytecode(&seeded).expect_err("M14 I/O without grants fails closed");
        assert!(
            denied.message.contains("AE-HOST-003"),
            "M14 {name} deny without grant: {}",
            denied.message
        );
    }
}

#[test]
fn seed_profile_compiler_forges_m21_foreign_pilot_byte_identically() {
    // Compile/dual-compare + deny without library grant. Granted run needs the
    // pilot cdylib path (covered by CLI/core M21 integration tests).
    for (name, source) in [
        ("foreign-pilot", M21_FOREIGN_PILOT_SOURCE),
        ("foreign-sum", M21_FOREIGN_SUM_SOURCE),
    ] {
        let bootstrap = compile_to_bytecode(source)
            .unwrap_or_else(|error| panic!("M21 {name} must bootstrap: {error}"))
            .bytecode;
        let seeded = compile_with_seed(source)
            .unwrap_or_else(|error| panic!("M21 {name} must seed-compile: {error}"))
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "M21 {name} seed-hosted compile must match bootstrap byte-for-byte"
        );
        verify_bytecode(&seeded).expect("M21 seed-produced artifact must verify");
        assert!(
            seeded.contains(&65),
            "M21 {name} must emit HOST_CALL opcode 65"
        );
        let denied = run_bytecode(&seeded).expect_err("M21 foreign without grant fails closed");
        assert!(
            denied.message.contains("AE-FFI-003"),
            "M21 {name} deny without grant: {}",
            denied.message
        );
    }
}

#[test]
fn seed_hosted_compile_matches_bootstrap_for_shipped_examples() {
    assert_eq!(
        SEED_COMPILER_ARTIFACT,
        compile_to_bytecode(SEED_SOURCE)
            .expect("seed source must bootstrap")
            .bytecode
            .as_slice(),
        "embedded seed artifact must match bootstrap of seed source"
    );

    let examples = [
        (
            "welcome",
            include_str!("../../../examples/welcome.ae"),
            None,
        ),
        (
            "control-flow",
            include_str!("../../../examples/control-flow.ae"),
            None,
        ),
        ("weaves", include_str!("../../../examples/weaves.ae"), None),
        (
            "unicode",
            include_str!("../../../examples/unicode.ae"),
            None,
        ),
        (
            "seed-multi-weave",
            include_str!("../../../examples/seed-multi-weave.ae"),
            None,
        ),
        (
            "seed-forward-call",
            include_str!("../../../examples/seed-forward-call.ae"),
            None,
        ),
        (
            "records",
            include_str!("../../../examples/records.ae"),
            None,
        ),
        (
            "arena-buffer",
            include_str!("../../../examples/arena-buffer.ae"),
            Some(7),
        ),
        (
            "arena-exhausted",
            include_str!("../../../examples/arena-exhausted.ae"),
            Some(-1),
        ),
        (
            "arena-full",
            include_str!("../../../examples/arena-full.ae"),
            Some(-2),
        ),
        (
            "arena-lookup-fallback",
            include_str!("../../../examples/arena-lookup-fallback.ae"),
            Some(99),
        ),
        (
            "arena-truth-buffer",
            include_str!("../../../examples/arena-truth-buffer.ae"),
            Some(1),
        ),
        (
            "arena-access-weave",
            include_str!("../../../examples/arena-access-weave.ae"),
            Some(1),
        ),
        (
            "error-effect",
            include_str!("../../../examples/error-effect.ae"),
            Some(17),
        ),
        ("comptime", M5_COMPTIME_SOURCE, Some(150)),
        ("comptime-chain", M15_COMPTIME_CHAIN_SOURCE, Some(288)),
        ("comptime-calls", M23_COMPTIME_CALL_SOURCE, Some(512)),
        ("resource-handle", M16_RESOURCE_HANDLE_SOURCE, Some(7)),
        ("layout-table", M6_LAYOUT_SOURCE, Some(10)),
        ("nursery-total", M7_NURSERY_TOTAL_SOURCE, Some(7)),
        ("nursery-cancel", M7_NURSERY_CANCEL_SOURCE, Some(9)),
        ("nursery-resource", M19B_NURSERY_RESOURCE_SOURCE, Some(7)),
        ("spawn-arena", M19D_SPAWN_ARENA_SOURCE, Some(7)),
        ("active-cancel", M19E_ACTIVE_CANCEL_SOURCE, Some(9)),
        (
            "task-frame-capacity",
            M19E_TASK_FRAME_CAPACITY_SOURCE,
            Some(3),
        ),
        ("task-loop", M19E_TASK_LOOP_SOURCE, Some(3)),
        ("release-raise", M19A_RELEASE_RAISE_SOURCE, Some(9)),
        ("host-pilot", M8_HOST_PILOT_SOURCE, Some(48)),
    ];
    for (name, source, expected_exit_code) in examples {
        let bootstrap = compile_to_bytecode(source)
            .expect("example must bootstrap")
            .bytecode;
        let seeded = compile_with_seed(source)
            .expect("example must compile through the seed path")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "seed-hosted compile must match bootstrap for shipped example {name}"
        );
        let run = run_bytecode(&seeded).expect("seed-hosted artifact must run");
        if let Some(expected_exit_code) = expected_exit_code {
            assert_eq!(
                run.exit_code, expected_exit_code,
                "seed-hosted M2 example {name} should preserve its defined outcome"
            );
        } else {
            assert!(
                run.exit_code >= 0,
                "seed-hosted example {name} should produce a nonnegative Whole exit"
            );
        }
    }
}

#[test]
fn seed_profile_matches_bootstrap_across_the_complete_language_surface() {
    let cases = [
        (
            "escaped UTF-8 text",
            r#"world escaped

weave main [] -> Whole:
  bind source <- "line\nquote: \" slash: \\ tab:\t return:\r é🙂"
  bind size <- measure borrow source
  bind glyph_value <- glyph borrow source 1
  bind section <- cut borrow source 0 4
  bind found <- seek borrow source "quote" 0
  bind score <- sum size glyph_value
  speak borrow section
  yield sum score found
"#,
        ),
        (
            "whole and Truth operations",
            r#"world arithmetic

weave main [] -> Whole:
  bind parsed <- number "42"
  bind added <- sum parsed 8
  bind subtracted <- difference added 10
  bind multiplied <- product subtracted 2
  bind divided <- quotient multiplied 4
  bind remainder_value <- remainder multiplied 7
  bind ordering <- less divided multiplied
  bind equality <- same remainder_value 1
  bind inverted <- not dim
  bind rendered <- render multiplied
  bind mutable result <- 0
  choose ordering:
    speak borrow rendered
  otherwise:
    speak "unexpected"
  choose equality:
    revise result <- sum divided 1
  otherwise:
    choose inverted:
      revise result <- remainder_value
    otherwise:
      revise result <- 0
  yield result
"#,
        ),
        (
            "Bytes and fixed-width primitives",
            r#"world binary

weave package [borrow input: Text] -> Bytes:
  bind encoded <- encode borrow input
  bind suffix <- bytes "AaFf"
  bind fused <- fuse move encoded move suffix
  bind appended <- append move fused 1
  bind packed16 <- pack16 4660
  bind packed32 <- pack32 16909060
  bind packed64 <- pack64 -1
  bind unpacked16 <- unpack16 borrow packed16 0
  bind unpacked32 <- unpack32 borrow packed32 0
  bind patched <- poke borrow packed32 1 255
  bind restored <- poke32 borrow patched 0 16909060
  bind equality <- same borrow packed16 borrow packed16
  bind first <- octet borrow appended 0
  bind preview <- slice borrow appended 0 2
  bind decoded <- decode move preview
  bind mutable output <- bytes ""
  speak move decoded
  choose equality:
    revise output <- borrow appended
  otherwise:
    revise output <- bytes ""
  yield move output

weave main [] -> Whole:
  bind packet <- call package "é"
  bind size <- extent borrow packet
  yield size
"#,
        ),
        (
            "named parameters, owned values, and forward calls",
            r#"world calls

weave main [] -> Whole:
  bind payload <- bytes "00ff"
  bind label <- "Aether"
  bind active <- call is_small 7
  bind result <- call decorate move label active borrow payload 7
  bind consumed <- call consume move payload
  speak move result
  yield consumed

weave is_small [value: Whole] -> Truth:
  yield less value 8

weave decorate [label: Text, enabled: Truth, borrow payload: Bytes, count: Whole] -> Text:
  bind size <- extent borrow payload
  bind count_text <- render count
  bind size_text <- render size
  bind mutable output <- ""
  choose enabled:
    revise output <- join borrow label borrow count_text
  otherwise:
    revise output <- join borrow label borrow size_text
  yield move output

weave consume [payload: Bytes] -> Whole:
  yield extent borrow payload
"#,
        ),
        (
            "nested control flow",
            r#"world nested

weave main [] -> Whole:
  bind mutable outer <- 0
  bind mutable inner <- 0
  bind mutable total <- 0
  while less outer 3:
    revise inner <- 0
    while less inner 2:
      choose less inner 1:
        revise total <- sum total outer
      otherwise:
        revise total <- sum total inner
      revise inner <- sum inner 1
    revise outer <- sum outer 1
  yield total
"#,
        ),
        (
            "identifier forms and optional otherwise",
            r#"world names_7

weave main [] -> Whole:
  bind source_value <- "Aether"
  bind bytes_value <- bytes "00ff"
  yield call helper_2 borrow source_value 7 bright borrow bytes_value

weave helper_2 [borrow source_value: Text, whole_7: Whole, flag_2: Truth, borrow bytes_1: Bytes] -> Whole:
  bind text_size <- measure borrow source_value
  bind byte_size <- extent borrow bytes_1
  bind mutable result_9 <- whole_7
  choose flag_2:
    revise result_9 <- sum text_size byte_size
  choose not dim:
    speak "optional otherwise"
  yield result_9
"#,
        ),
        (
            "immutable nominal records",
            r#"world records

record card [label: Text, score: Whole, payload: Bytes, active: Truth]

weave copy [borrow value: card] -> card:
  bind label <- field borrow value label
  bind score <- field borrow value score
  bind payload <- field borrow value payload
  bind active <- field borrow value active
  yield make card borrow label score borrow payload active

weave main [] -> Whole:
  bind original <- make card "Aether" 7 bytes "0102" bright
  bind copied <- call copy borrow original
  bind equal <- same borrow original borrow copied
  bind label <- field borrow copied label
  bind score <- field borrow copied score
  choose equal:
    speak borrow label
  otherwise:
    speak "mismatch"
  yield score
"#,
        ),
        (
            "last statement without a trailing line feed",
            "world final_line\n\nweave main [] -> Whole:\n  bind message <- \"final\"\n  speak borrow message\n  yield 5",
        ),
    ];

    for (name, source) in cases {
        assert_seed_matches_bootstrap(name, source);
    }
}

fn assert_seed_matches_bootstrap(name: &str, source: &str) {
    let bootstrap = compile_to_bytecode(source)
        .unwrap_or_else(|error| panic!("{name} fixture must bootstrap: {error}"))
        .bytecode;
    let seeded = bytes(
        forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .unwrap_or_else(|error| panic!("{name} fixture must forge through the seed: {error}")),
    );

    verify_bytecode(&seeded)
        .unwrap_or_else(|error| panic!("{name} seed artifact must verify: {error}"));
    assert_eq!(
        seeded, bootstrap,
        "{name} seed artifact must match bootstrap byte-for-byte"
    );
    let seeded_run = run_bytecode(&seeded)
        .unwrap_or_else(|error| panic!("{name} seed artifact must run: {error}"));
    let bootstrap_run = run_bytecode(&bootstrap)
        .unwrap_or_else(|error| panic!("{name} bootstrap artifact must run: {error}"));
    assert_eq!(
        seeded_run, bootstrap_run,
        "{name} seed artifact must preserve bootstrap runtime behavior"
    );
}

fn bytes(output: InvocationOutput) -> Vec<u8> {
    let InvocationValue::Bytes(value) = output.value else {
        panic!("Seed Profile compiler ABI requires Bytes output");
    };
    value
}
