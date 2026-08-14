use aether_core::{
    apply_edit_cli_trusts_product_accept, bootstrap_is_recovery_oracle_only,
    check_product_base_gate, compile_product_bytecode, compile_product_multi_source_envelope,
    compile_to_bytecode, compile_with_seed, compile_with_seed_invokes_bootstrap,
    compile_with_seed_product_authoritative, encode_multi_source_envelope, forge_bytecode,
    host_elaborates_modules_seed_emits, lib_module_validates_via_product_seed,
    lsp_product_diagnostics_primary, lsp_product_surface_hover_definition,
    multi_module_product_choose_revise_supported, product_cli_check_without_bootstrap,
    product_default_cli_toolchain, product_diagnostic_abi, product_diagnostics,
    product_error_packets, product_format_without_bootstrap,
    product_multi_module_invokes_bootstrap, product_multi_source_forge_envelope,
    product_path_forges_before_bootstrap_validate, product_path_requires_bootstrap_dual_compare,
    product_project_format_without_bootstrap, product_rejects_yield_in_truth_choose,
    product_seed_error_packet_abi, product_seed_error_speak_format,
    product_seed_rebuild_without_bootstrap, product_structure_without_bootstrap,
    product_surface_symbols, product_surface_symbols_without_bootstrap, run_bytecode,
    seed_internal_error_packets, seed_interprets_m23_comptime_calls_natively,
    seed_native_multi_module_elaboration, seed_product_diagnostics_phase3c,
    seed_product_diagnostics_subset, seed_product_preflight_phase3b,
    structural_edit_accepts_via_product_seed, structural_edit_product_base_gate,
    structural_edit_product_statement_and_record_ops, structural_edit_product_top_level_weave_ops,
    structural_edit_product_weave_replace, verify_bytecode, InvocationOutput, InvocationValue,
    SEED_COMPILER_ARTIFACT, SEED_ERROR_PACKET_SCHEMA,
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
    // the seed's existing high-numbered slots (v103 through v131: M23 temps +
    // ADR-094 empty SPEAK flag).
    let variant = if SEED_SOURCE.contains("  bind mutable v65 <- 0\r\n") {
        SEED_SOURCE.replacen(
            "  bind mutable v65 <- 0\r\n",
            "  bind mutable v65 <- 0\r\n  bind mutable v132 <- 0\r\n",
            1,
        )
    } else {
        SEED_SOURCE.replacen(
            "  bind mutable v65 <- 0\n",
            "  bind mutable v65 <- 0\n  bind mutable v132 <- 0\n",
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
    assert!(
        structural_edit_product_top_level_weave_ops(),
        "ADR-068: product top-level weave replace/insertAfter/delete"
    );
    assert!(
        structural_edit_product_statement_and_record_ops(),
        "ADR-069: product weave-body statement and primitive record ops"
    );
    assert!(
        product_rejects_yield_in_truth_choose(),
        "ADR-070: product rejects yield in truth-choose"
    );
    assert!(
        multi_module_product_choose_revise_supported(),
        "ADR-070: multi-module product choose+revise supported"
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
fn barp_adr070_product_rejects_yield_in_truth_choose() {
    let source = "world w\n\nweave main [] -> Whole:\n  bind x <- 7\n  choose same x 7:\n    yield 42\n  otherwise:\n    yield -1\n";
    let error =
        compile_product_bytecode(source).expect_err("yield in truth-choose must fail closed");
    let message = error.to_string();
    assert!(
        message.contains("AE-SEED-013"),
        "expected AE-SEED-013, got {message}"
    );
    assert!(
        message.contains("truth-condition choose"),
        "expected truth-choose hint, got {message}"
    );
    let packets = product_error_packets(source);
    assert_eq!(packets[0].code, "AE-SEED-013");
    assert_eq!(packets[0].origin, "host-preflight");
    // Resource choose may still yield (M2) — product must accept.
    let resource = "world w\n\nweave main [] -> Whole:\n  bind memory <- arena 32\n  bind mutable values <- buffer Whole\n  bind mutable observed <- 0\n  choose allocate access memory move values 1 into values:\n    choose append move values 7 into values:\n      choose at borrow values 0 into observed:\n        yield observed\n      otherwise:\n        yield -3\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
    let ok =
        compile_product_bytecode(resource).expect("resource choose yield must remain product-ok");
    verify_bytecode(&ok).expect("resource choose product artifact must verify");
}

#[test]
fn barp_adr112_seed_speaks_canonical_less_choose_yield() {
    let invalid = "world w\n\nweave main [] -> Whole:\n  bind value <- 7\n  choose less value 8:\n    yield 42\n  otherwise:\n    yield -1\n";

    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, invalid)
        .expect("canonical less-choose source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-013\""),
        "expected a seed-native AE-SEED-013 packet, got: {}",
        direct.stdout
    );
    assert!(
        direct.stdout.contains("\"origin\":\"seed-speak\""),
        "expected seed-speak origin, got: {}",
        direct.stdout
    );
    assert!(
        direct.stdout.contains(
            "\"message\":\"yield is not allowed inside a canonical truth choose branch\""
        ),
        "expected the truth-choose diagnostic message, got: {}",
        direct.stdout
    );
    assert_eq!(
        direct.stdout.matches("AETHER_SEED_ERROR:").count(),
        1,
        "canonical less-choose source must emit exactly one packet: {}",
        direct.stdout
    );
    match direct.value {
        InvocationValue::Bytes(bytes) => assert!(
            bytes.is_empty(),
            "canonical less-choose SPEAK must return blank Bytes, got {} bytes",
            bytes.len()
        ),
        other => panic!("canonical less-choose SPEAK must return Bytes, got {other:?}"),
    }

    let product = compile_product_bytecode(invalid)
        .expect_err("canonical less-choose nested yield must fail product compilation");
    assert!(
        product.to_string().contains("AE-SEED-013"),
        "expected AE-SEED-013, got {product}"
    );
    let packets = product_error_packets(invalid);
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].code, "AE-SEED-013");
    assert_eq!(packets[0].origin, "host-preflight");

    let valid = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind value <- 7\n  choose less value 8:\n    revise result <- 42\n  otherwise:\n    revise result <- -1\n  yield result\n";
    let bootstrap = compile_to_bytecode(valid)
        .expect("root-yield less-choose source must bootstrap")
        .bytecode;
    let seeded = bytes(
        forge_bytecode(SEED_COMPILER_ARTIFACT, valid)
            .expect("root-yield less-choose source must forge through the seed"),
    );
    verify_bytecode(&seeded).expect("root-yield less-choose seed artifact must verify");
    assert_eq!(
        seeded, bootstrap,
        "root-yield less-choose source must retain seed/bootstrap identity"
    );
}

#[test]
fn barp_adr113_seed_speaks_canonical_literal_truth_choose_yield() {
    for condition in ["bright", "dim"] {
        let invalid = format!(
            "world w\n\nweave main [] -> Whole:\n  choose {condition}:\n    yield 42\n  otherwise:\n    yield -1\n"
        );

        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &invalid)
            .expect("canonical literal-truth source must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-013\""),
            "expected a seed-native AE-SEED-013 packet for {condition}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {condition}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"yield is not allowed inside a canonical truth choose branch\""
            ),
            "expected the truth-choose diagnostic message for {condition}, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {condition} source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {condition} SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {condition} SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(&invalid)
            .expect_err("canonical literal-truth nested yield must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-013"),
            "expected AE-SEED-013 for {condition}, got {product}"
        );
        let packets = product_error_packets(&invalid);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-013");
        assert_eq!(packets[0].origin, "host-preflight");
    }

    for (condition, branch_value, otherwise_value, expected_exit) in
        [("bright", 42, -1, 42), ("dim", -1, 42, 42)]
    {
        let valid = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  choose {condition}:\n    revise result <- {branch_value}\n  otherwise:\n    revise result <- {otherwise_value}\n  yield result\n"
        );
        let bootstrap = compile_to_bytecode(&valid)
            .expect("root-yield literal-truth source must bootstrap")
            .bytecode;
        let seeded = bytes(
            forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
                .expect("root-yield literal-truth source must forge through the seed"),
        );
        verify_bytecode(&seeded).expect("root-yield literal-truth seed artifact must verify");
        assert_eq!(
            seeded, bootstrap,
            "root-yield {condition} source must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("root-yield literal-truth seed artifact must run")
                .exit_code,
            expected_exit
        );
    }

    for (name, source) in [
        (
            "bare Truth variable",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind flag <- bright\n  choose flag:\n    revise result <- 42\n  otherwise:\n    revise result <- -1\n  yield result\n",
        ),
        (
            "not Truth expression",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  choose not dim:\n    revise result <- 42\n  otherwise:\n    revise result <- -1\n  yield result\n",
        ),
    ] {
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .expect("nonliteral Truth source must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "{name} must remain outside the literal pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("nonliteral Truth source must forge verified AETH");
        let bootstrap = compile_to_bytecode(source)
            .expect("nonliteral Truth source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "{name} must retain seed/bootstrap identity"
        );
    }
}

#[test]
fn barp_adr114_seed_speaks_canonical_unary_literal_truth_choose_yield() {
    for condition in ["not bright", "not dim"] {
        let invalid = format!(
            "world w\n\nweave main [] -> Whole:\n  choose {condition}:\n    yield 42\n  otherwise:\n    yield -1\n"
        );

        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &invalid)
            .expect("canonical unary-literal source must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-013\""),
            "expected a seed-native AE-SEED-013 packet for {condition}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {condition}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"yield is not allowed inside a canonical truth choose branch\""
            ),
            "expected the truth-choose diagnostic message for {condition}, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {condition} source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {condition} SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {condition} SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(&invalid)
            .expect_err("canonical unary literal nested yield must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-013"),
            "expected AE-SEED-013 for {condition}, got {product}"
        );
        let packets = product_error_packets(&invalid);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-013");
        assert_eq!(packets[0].origin, "host-preflight");
    }

    for (condition, branch_value, otherwise_value, expected_exit) in
        [("not bright", -1, 42, 42), ("not dim", 42, -1, 42)]
    {
        let valid = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  choose {condition}:\n    revise result <- {branch_value}\n  otherwise:\n    revise result <- {otherwise_value}\n  yield result\n"
        );
        let bootstrap = compile_to_bytecode(&valid)
            .expect("root-yield unary-literal source must bootstrap")
            .bytecode;
        let seeded = bytes(
            forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
                .expect("root-yield unary-literal source must forge through the seed"),
        );
        verify_bytecode(&seeded).expect("root-yield unary-literal seed artifact must verify");
        assert_eq!(
            seeded, bootstrap,
            "root-yield {condition} source must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("root-yield unary-literal seed artifact must run")
                .exit_code,
            expected_exit
        );
    }

    let nonliteral = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind flag <- dim\n  choose not flag:\n    revise result <- 42\n  otherwise:\n    revise result <- -1\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, nonliteral)
        .expect("unary Truth-variable source must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "unary Truth-variable source must remain outside the literal pilot: {}",
        direct.stdout
    );
    let seeded = bytes(direct);
    verify_bytecode(&seeded).expect("unary Truth-variable source must forge verified AETH");
    let bootstrap = compile_to_bytecode(nonliteral)
        .expect("unary Truth-variable source must bootstrap")
        .bytecode;
    assert_eq!(
        seeded, bootstrap,
        "unary Truth-variable source must retain seed/bootstrap identity"
    );
}

#[test]
fn barp_adr115_seed_speaks_canonical_whole_truth_literal_yield() {
    for literal in ["bright", "dim"] {
        let invalid = format!("world w\n\nweave main [] -> Whole:\n  yield {literal}\n");

        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &invalid)
            .expect("canonical Whole Truth-literal source must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-010\""),
            "expected a seed-native AE-SEED-010 packet for {literal}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {literal}, got: {}",
            direct.stdout
        );
        assert!(
            direct
                .stdout
                .contains("\"message\":\"Whole weave cannot yield a Truth literal\""),
            "expected the Truth-literal diagnostic message for {literal}, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {literal} source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {literal} SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {literal} SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(&invalid)
            .expect_err("canonical Whole Truth literal must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-010"),
            "expected AE-SEED-010 for {literal}, got {product}"
        );
        let packets = product_error_packets(&invalid);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-010");
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for literal in ["bright", "dim"] {
        let valid = format!(
            "world w\n\nweave truth_value [] -> Truth:\n  yield {literal}\n\nweave main [] -> Whole:\n  yield 42\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("Truth-return literal source must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "Truth-return {literal} source must remain outside the Whole pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("Truth-return literal seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("Truth-return literal source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "Truth-return {literal} source must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("Truth-return literal seed artifact must run")
                .exit_code,
            42
        );
    }
}

#[test]
fn barp_adr116_seed_speaks_canonical_zero_argument_unknown_calls() {
    for (form, source) in [
        (
            "bind",
            "world w\n\nweave main [] -> Whole:\n  bind result <- call nope\n  yield result\n",
        ),
        (
            "root yield",
            "world w\n\nweave main [] -> Whole:\n  yield call nope\n",
        ),
    ] {
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .expect("canonical zero-argument unknown call must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "expected a seed-native AE-SEED-011 packet for {form}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {form}, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"canonical direct call target does not name a top-level declared weave\""
            ),
            "expected the unknown-call message for {form}, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical zero-argument {form} source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical zero-argument {form} SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => {
                panic!("canonical zero-argument {form} SPEAK must return Bytes, got {other:?}")
            }
        }

        let product = compile_product_bytecode(source)
            .expect_err("canonical zero-argument unknown call must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-011"),
            "expected AE-SEED-011 for {form}, got {product}"
        );
        let packets = product_error_packets(source);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-011");
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for (form, body) in [
        ("bind", "bind result <- call helper\n  yield result"),
        ("root yield", "yield call helper"),
    ] {
        let valid = format!(
            "world w\n\nweave main [] -> Whole:\n  {body}\n\nweave helper [] -> Whole:\n  yield 42\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("declared zero-argument call must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "declared zero-argument {form} call must remain outside the pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("declared zero-argument call seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("declared zero-argument call source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "declared zero-argument {form} call must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("declared zero-argument call seed artifact must run")
                .exit_code,
            42
        );
    }

    let literal =
        "world w\n\nweave main [] -> Whole:\n  speak \"bind result <- call nope\"\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("call-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "call-shaped Text literal must remain outside the zero-argument pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct)).expect("call-shaped Text literal artifact must verify");

    let missing_world = "weave main [] -> Whole:\n  bind result <- call nope\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world zero-argument source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority zero-argument pilot: {}",
        direct.stdout
    );
}

#[test]
fn barp_adr117_seed_speaks_canonical_revise_unknown_calls() {
    for (form, source) in [
        (
            "argument-bearing",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  revise result <- call nope 41\n  yield result\n",
        ),
        (
            "zero-argument",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  revise result <- call nope\n  yield result\n",
        ),
    ] {
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .expect("canonical revise unknown call must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "expected a seed-native AE-SEED-011 packet for {form} revise, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {form} revise, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"canonical direct call target does not name a top-level declared weave\""
            ),
            "expected the unknown-call message for {form} revise, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {form} revise source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {form} revise SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {form} revise SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(source)
            .expect_err("canonical revise unknown call must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-011"),
            "expected AE-SEED-011 for {form} revise, got {product}"
        );
        let packets = product_error_packets(source);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-011");
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for (form, main_call, helper) in [
        (
            "argument-bearing",
            "revise result <- call helper 41",
            "weave helper [value: Whole] -> Whole:\n  yield sum value 1",
        ),
        (
            "zero-argument",
            "revise result <- call helper",
            "weave helper [] -> Whole:\n  yield 42",
        ),
    ] {
        let valid = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  {main_call}\n  yield result\n\n{helper}\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("declared revise call must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "declared {form} revise call must remain outside the pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("declared revise-call seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("declared revise-call source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "declared {form} revise call must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("declared revise-call seed artifact must run")
                .exit_code,
            42
        );
    }

    let literal =
        "world w\n\nweave main [] -> Whole:\n  speak \"revise result <- call nope\"\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("revise-call-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "revise-call-shaped Text literal must remain outside the pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct)).expect("revise-call-shaped Text literal artifact must verify");

    let missing_world =
        "weave main [] -> Whole:\n  bind mutable result <- 0\n  revise result <- call nope\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world revise-call source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority revise-call pilot: {}",
        direct.stdout
    );
}

#[test]
fn barp_adr118_seed_speaks_canonical_root_speak_unknown_calls() {
    for (form, source) in [
        (
            "argument-bearing",
            "world w\n\nweave main [] -> Whole:\n  speak call nope 41\n  yield 0\n",
        ),
        (
            "zero-argument",
            "world w\n\nweave main [] -> Whole:\n  speak call nope\n  yield 0\n",
        ),
    ] {
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .expect("canonical root speak unknown call must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "expected a seed-native AE-SEED-011 packet for {form} root speak, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {form} root speak, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"canonical direct call target does not name a top-level declared weave\""
            ),
            "expected the unknown-call message for {form} root speak, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {form} root speak source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {form} root speak SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {form} root speak SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(source)
            .expect_err("canonical root speak unknown call must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-011"),
            "expected AE-SEED-011 for {form} root speak, got {product}"
        );
        let packets = product_error_packets(source);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].code, "AE-SEED-011");
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for (form, main_call, helper) in [
        (
            "argument-bearing",
            "speak call render_value 41",
            "weave render_value [value: Whole] -> Text:\n  yield \"ready\"",
        ),
        (
            "zero-argument",
            "speak call message",
            "weave message [] -> Text:\n  yield \"ready\"",
        ),
    ] {
        let valid =
            format!("world w\n\nweave main [] -> Whole:\n  {main_call}\n  yield 42\n\n{helper}\n");
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("declared root speak call must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "declared {form} root speak call must remain outside the pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("declared root speak-call seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("declared root speak-call source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "declared {form} root speak call must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("declared root speak-call seed artifact must run")
                .exit_code,
            42
        );
    }

    let literal = "world w\n\nweave main [] -> Whole:\n  speak \"speak call nope\"\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("root speak call-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "root speak call-shaped Text literal must remain outside the pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct))
        .expect("root speak call-shaped Text literal artifact must verify");

    let missing_world = "weave main [] -> Whole:\n  speak call nope\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world root speak-call source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority root speak-call pilot: {}",
        direct.stdout
    );
}

#[test]
fn barp_adr119_seed_speaks_canonical_root_handle_unknown_calls() {
    for (form, source) in [
        (
            "argument-bearing",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call nope 41 into result otherwise error into code\n",
        ),
        (
            "zero-argument",
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call nope into result otherwise error into code\n",
        ),
    ] {
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
            .expect("canonical root handle unknown call must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "expected a seed-native AE-SEED-011 packet for {form} root handle, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {form} root handle, got: {}",
            direct.stdout
        );
        assert!(
            direct
                .stdout
                .contains("\"schema\":\"aether.seed-error/v1\""),
            "expected seed error schema for {form} root handle, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"line\":1,\"column\":1"),
            "expected 1:1 seed-SPEAK position for {form} root handle, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"canonical direct call target does not name a top-level declared weave\""
            ),
            "expected the unknown-call message for {form} root handle, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {form} root handle source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {form} root handle SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {form} root handle SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(source)
            .expect_err("canonical root handle unknown call must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-011"),
            "expected AE-SEED-011 for {form} root handle, got {product}"
        );
        let packets = product_error_packets(source);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].schema, SEED_ERROR_PACKET_SCHEMA);
        assert_eq!(packets[0].code, "AE-SEED-011");
        assert_eq!(packets[0].line, 1);
        assert_eq!(packets[0].column, 1);
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for (form, main_call, helper) in [
        (
            "argument-bearing",
            "handle call helper 41 into result otherwise error into code",
            "weave helper [value: Whole] -> Whole raises Whole:\n  yield sum value 1",
        ),
        (
            "zero-argument",
            "handle call helper into result otherwise error into code",
            "weave helper [] -> Whole raises Whole:\n  yield 42",
        ),
    ] {
        let valid = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  {main_call}\n\n{helper}\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("declared root handle call must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "declared {form} root handle call must remain outside the pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("declared root handle-call seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("declared root handle-call source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "declared {form} root handle call must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("declared root handle-call seed artifact must run")
                .exit_code,
            42
        );
    }

    let literal = "world w\n\nweave main [] -> Whole:\n  speak \"handle call nope into result otherwise error into code\"\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("root handle call-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "root handle call-shaped Text literal must remain outside the pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct))
        .expect("root handle call-shaped Text literal artifact must verify");

    let incomplete = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call nope\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, incomplete)
        .expect("incomplete root handle source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "incomplete root handle tail must remain outside the pilot: {}",
        direct.stdout
    );

    let missing_world = "weave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call nope into result otherwise error into code\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world root handle-call source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority root handle-call pilot: {}",
        direct.stdout
    );
}

#[test]
fn barp_adr120_seed_speaks_canonical_root_forward_unknown_calls() {
    for (form, relay_forward) in [
        ("argument-bearing", "forward call nope 41"),
        ("zero-argument", "forward call nope"),
    ] {
        let source = format!(
            "world w\n\nweave relay [] -> Whole raises Whole:\n  {relay_forward}\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call relay into result otherwise error into code\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &source)
            .expect("canonical root forward unknown call must reach the seed forge");
        assert!(
            direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "expected a seed-native AE-SEED-011 packet for {form} root forward, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"origin\":\"seed-speak\""),
            "expected seed-speak origin for {form} root forward, got: {}",
            direct.stdout
        );
        assert!(
            direct
                .stdout
                .contains("\"schema\":\"aether.seed-error/v1\""),
            "expected seed error schema for {form} root forward, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains("\"line\":1,\"column\":1"),
            "expected 1:1 seed-SPEAK position for {form} root forward, got: {}",
            direct.stdout
        );
        assert!(
            direct.stdout.contains(
                "\"message\":\"canonical direct call target does not name a top-level declared weave\""
            ),
            "expected the unknown-call message for {form} root forward, got: {}",
            direct.stdout
        );
        assert_eq!(
            direct.stdout.matches("AETHER_SEED_ERROR:").count(),
            1,
            "canonical {form} root forward source must emit exactly one packet: {}",
            direct.stdout
        );
        match direct.value {
            InvocationValue::Bytes(bytes) => assert!(
                bytes.is_empty(),
                "canonical {form} root forward SPEAK must return blank Bytes, got {} bytes",
                bytes.len()
            ),
            other => panic!("canonical {form} root forward SPEAK must return Bytes, got {other:?}"),
        }

        let product = compile_product_bytecode(&source)
            .expect_err("canonical root forward unknown call must fail product compilation");
        assert!(
            product.to_string().contains("AE-SEED-011"),
            "expected AE-SEED-011 for {form} root forward, got {product}"
        );
        let packets = product_error_packets(&source);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].schema, SEED_ERROR_PACKET_SCHEMA);
        assert_eq!(packets[0].code, "AE-SEED-011");
        assert_eq!(packets[0].line, 1);
        assert_eq!(packets[0].column, 1);
        assert_eq!(packets[0].origin, "seed-speak");
    }

    for (form, relay_forward, leaf) in [
        (
            "argument-bearing",
            "forward call leaf 42",
            "weave leaf [value: Whole] -> Whole raises Whole:\n  raise value",
        ),
        (
            "zero-argument",
            "forward call leaf",
            "weave leaf [] -> Whole raises Whole:\n  raise 42",
        ),
    ] {
        let valid = format!(
            "world w\n\nweave relay [] -> Whole raises Whole:\n  {relay_forward}\n\n{leaf}\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call relay into result otherwise error into code\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &valid)
            .expect("declared root forward call must reach the seed forge");
        assert!(
            !direct.stdout.contains("AETHER_SEED_ERROR:"),
            "declared {form} root forward call must remain outside the pilot: {}",
            direct.stdout
        );
        let seeded = bytes(direct);
        verify_bytecode(&seeded).expect("declared root forward-call seed artifact must verify");
        let bootstrap = compile_to_bytecode(&valid)
            .expect("declared root forward-call source must bootstrap")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "declared {form} root forward call must retain seed/bootstrap identity"
        );
        assert_eq!(
            run_bytecode(&seeded)
                .expect("declared root forward-call seed artifact must run")
                .exit_code,
            42
        );
    }

    let literal = "world w\n\nweave relay [] -> Whole raises Whole:\n  speak \"forward call nope\"\n  raise 42\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call relay into result otherwise error into code\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("erroring-weave forward-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "forward-shaped Text literal must remain outside the pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct))
        .expect("erroring-weave forward-shaped Text literal artifact must verify");

    let total_caller = "world w\n\nweave relay [] -> Whole:\n  forward call nope\n\nweave main [] -> Whole:\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, total_caller)
        .expect("total-caller forward source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "total-caller forward must remain outside the erroring-caller pilot: {}",
        direct.stdout
    );

    let incomplete = "world w\n\nweave relay [] -> Whole raises Whole:\n  forward call\n\nweave main [] -> Whole:\n  yield 0\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, incomplete)
        .expect("incomplete root forward source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "incomplete root forward prefix must remain outside the pilot: {}",
        direct.stdout
    );

    let missing_world = "weave relay [] -> Whole raises Whole:\n  forward call nope\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  bind mutable code <- 0\n  handle call relay into result otherwise error into code\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world root forward-call source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority root forward-call pilot: {}",
        direct.stdout
    );
}

#[test]
fn barp_adr121_seed_speaks_canonical_root_nursery_zero_argument_spawn_unknown_calls() {
    let source = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    spawn call nope into result\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, source)
        .expect("canonical root nursery spawn unknown call must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "expected a seed-native AE-SEED-011 packet for root nursery spawn, got: {}",
        direct.stdout
    );
    assert!(
        direct.stdout.contains("\"origin\":\"seed-speak\""),
        "expected seed-speak origin for root nursery spawn, got: {}",
        direct.stdout
    );
    assert!(
        direct
            .stdout
            .contains("\"schema\":\"aether.seed-error/v1\""),
        "expected seed error schema for root nursery spawn, got: {}",
        direct.stdout
    );
    assert!(
        direct.stdout.contains("\"line\":1,\"column\":1"),
        "expected 1:1 seed-SPEAK position for root nursery spawn, got: {}",
        direct.stdout
    );
    assert!(
        direct.stdout.contains(
            "\"message\":\"canonical direct call target does not name a top-level declared weave\""
        ),
        "expected the unknown-call message for root nursery spawn, got: {}",
        direct.stdout
    );
    assert_eq!(
        direct.stdout.matches("AETHER_SEED_ERROR:").count(),
        1,
        "canonical root nursery spawn source must emit exactly one packet: {}",
        direct.stdout
    );
    match direct.value {
        InvocationValue::Bytes(bytes) => assert!(
            bytes.is_empty(),
            "canonical root nursery spawn SPEAK must return blank Bytes, got {} bytes",
            bytes.len()
        ),
        other => panic!("canonical root nursery spawn SPEAK must return Bytes, got {other:?}"),
    }

    let product = compile_product_bytecode(source)
        .expect_err("canonical root nursery spawn unknown call must fail product compilation");
    assert!(
        product.to_string().contains("AE-SEED-011"),
        "expected AE-SEED-011 for root nursery spawn, got {product}"
    );
    let packets = product_error_packets(source);
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].schema, SEED_ERROR_PACKET_SCHEMA);
    assert_eq!(packets[0].code, "AE-SEED-011");
    assert_eq!(packets[0].line, 1);
    assert_eq!(packets[0].column, 1);
    assert_eq!(packets[0].origin, "seed-speak");

    let valid = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    spawn call worker into result\n  yield result\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 42\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, valid)
        .expect("declared root nursery spawn call must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "declared root nursery spawn call must remain outside the pilot: {}",
        direct.stdout
    );
    let seeded = bytes(direct);
    verify_bytecode(&seeded).expect("declared root nursery spawn seed artifact must verify");
    let bootstrap = compile_to_bytecode(valid)
        .expect("declared root nursery spawn source must bootstrap")
        .bytecode;
    assert_eq!(
        seeded, bootstrap,
        "declared root nursery spawn call must retain seed/bootstrap identity"
    );
    assert_eq!(
        run_bytecode(&seeded)
            .expect("declared root nursery spawn seed artifact must run")
            .exit_code,
        42
    );

    let literal = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  speak \"spawn call nope into result\"\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, literal)
        .expect("root nursery-spawn-shaped Text literal must reach the seed forge");
    assert!(
        !direct.stdout.contains("AETHER_SEED_ERROR:"),
        "root nursery-spawn-shaped Text literal must remain outside the pilot: {}",
        direct.stdout
    );
    verify_bytecode(&bytes(direct))
        .expect("root nursery-spawn-shaped Text literal artifact must verify");

    let erroring_caller = "world w\n\nweave relay [] -> Whole raises Whole:\n  bind mutable result <- 0\n  together:\n    spawn call nope into result\n  raise 0\n\nweave main [] -> Whole:\n  bind mutable value <- 0\n  bind mutable code <- 0\n  handle call relay into value otherwise error into code\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, erroring_caller)
        .expect("erroring root nursery spawn source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "erroring root nursery spawn must remain outside the total-caller pilot: {}",
        direct.stdout
    );

    for malformed in ["spawn call nope", "spawn call nope into"] {
        let source = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    {malformed}\n  yield result\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &source)
            .expect("malformed root nursery spawn source must reach the seed forge");
        assert!(
            !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "malformed root nursery spawn must remain outside the pilot: {}",
            direct.stdout
        );
    }

    let argument_bearing = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    spawn call nope 3 into result\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, argument_bearing)
        .expect("argument-bearing root nursery spawn source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "argument-bearing root nursery spawn must remain outside the zero-argument pilot: {}",
        direct.stdout
    );

    for (boundary, spacer) in [
        ("a blank line", "\n"),
        ("a non-spawn nursery child", "    bind mutable spare <- 0\n"),
    ] {
        let source = format!(
            "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n{spacer}    spawn call nope into result\n  yield result\n"
        );
        let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, &source)
            .expect("delayed root nursery spawn source must reach the seed forge");
        assert!(
            !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
            "root nursery spawn after {boundary} must remain outside the immediate-child pilot: {}",
            direct.stdout
        );
    }

    let descendant = "world w\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n      spawn call nope into result\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, descendant)
        .expect("six-space nursery descendant source must reach the seed forge");
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "six-space nursery descendant must remain outside the four-space child pilot: {}",
        direct.stdout
    );

    let missing_world = "weave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    spawn call nope into result\n  yield result\n";
    let direct = forge_bytecode(SEED_COMPILER_ARTIFACT, missing_world)
        .expect("missing-world root nursery spawn source must reach the seed forge");
    assert!(
        direct.stdout.contains("\"code\":\"AE-SEED-006\""),
        "missing world must retain higher-priority AE-SEED-006, got: {}",
        direct.stdout
    );
    assert!(
        !direct.stdout.contains("\"code\":\"AE-SEED-011\""),
        "missing world must suppress the lower-priority root nursery spawn pilot: {}",
        direct.stdout
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
fn barp_adr072_product_seed_error_packet_abi() {
    assert!(product_seed_error_packet_abi());
    assert!(product_seed_error_speak_format());
    assert!(
        !seed_internal_error_packets(),
        "full seed binary packet matrix remains residual (bounded ADR-094/098/102/103 pilots only)"
    );
    assert!(
        aether_core::seed_speak_emit_empty_source_pilot(),
        "ADR-094: seed SPEAK empty-source pilot"
    );
    assert!(
        aether_core::seed_speak_emit_multi_code_pilot(),
        "ADR-098: multi-code seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_lexical_edge_pilot(),
        "ADR-102: lexical seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_reserved_task_pilot(),
        "ADR-103: reserved-task seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_task_checkpoint_pilot(),
        "ADR-106: exact task-checkpoint seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_zero_arg_unknown_call_pilot(),
        "ADR-116: zero-argument unknown-call seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_revise_unknown_call_pilot(),
        "ADR-117: root revise-call unknown-call seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_root_speak_unknown_call_pilot(),
        "ADR-118: root speak-call unknown-call seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_root_handle_unknown_call_pilot(),
        "ADR-119: root handle-call unknown-call seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_root_forward_unknown_call_pilot(),
        "ADR-120: root forward-call unknown-call seed SPEAK pilot"
    );
    assert!(
        aether_core::seed_speak_emit_root_nursery_zero_argument_spawn_unknown_call_pilot(),
        "ADR-121: root nursery zero-argument spawn-call unknown-call seed SPEAK pilot"
    );
    let empty = product_error_packets("world w\n\nweave main [] -> Whole:\n  yield 1\n");
    assert!(empty.is_empty());
    let packets = product_error_packets("");
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].schema, SEED_ERROR_PACKET_SCHEMA);
    assert_eq!(packets[0].code, "AE-SEED-005");
    assert_eq!(packets[0].origin, "host-preflight");
    // ADR-075: raw product diagnostic embeds AETHER_SEED_ERROR; packet decode prefers it.
    let raw = product_diagnostics("");
    assert!(
        raw[0].message.contains("AETHER_SEED_ERROR:"),
        "ADR-075 SPEAK-compatible packet line in product diagnostic"
    );
    let typed = product_error_packets("world w\n\nweave main [] -> Whole:\n  yield \"x\"\n");
    assert_eq!(typed.len(), 1);
    assert_eq!(typed[0].code, "AE-SEED-010");
    assert_eq!(typed[0].origin, "seed-speak");

    let truth_typed = product_error_packets("world w\n\nweave main [] -> Whole:\n  yield bright\n");
    assert_eq!(truth_typed.len(), 1);
    assert_eq!(truth_typed[0].code, "AE-SEED-010");
    assert_eq!(truth_typed[0].origin, "seed-speak");
}

#[test]
fn barp_adr075_multi_source_envelope_product_forge() {
    assert!(product_multi_source_forge_envelope());
    assert!(!seed_native_multi_module_elaboration());
    assert!(aether_core::product_multi_source_unit_surface_api());
    assert!(aether_core::product_multi_source_unit_digests_api());
    let lib = "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n";
    let main = "world app\n\nimport unit \"lib/math.ae\" as m\n\nweave main [] -> Whole:\n  yield call m.double 21\n";
    let envelope = encode_multi_source_envelope(&[
        ("lib/math.ae".to_owned(), lib.to_owned()),
        ("src/main.ae".to_owned(), main.to_owned()),
    ])
    .expect("encode envelope");
    assert!(envelope.contains("aether.multi-source/v1"));
    let surface = aether_core::product_multi_source_unit_surface(&envelope).expect("surface");
    assert_eq!(surface.unit_count, 2);
    assert_eq!(surface.entry_path.as_deref(), Some("src/main.ae"));
    assert!(surface.units.iter().all(|u| u.source_sha256.len() == 64));
    let bytecode = compile_product_multi_source_envelope(&envelope).expect("multi forge");
    verify_bytecode(&bytecode).expect("verify multi");
    assert_eq!(run_bytecode(&bytecode).expect("run").exit_code, 42);
    // ADR-078: product compile auto-detects multi-source envelopes.
    let via_product = compile_product_bytecode(&envelope).expect("product multi-source");
    assert_eq!(
        run_bytecode(&via_product)
            .expect("run product multi")
            .exit_code,
        42
    );
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

    let unknown_source = "world w\n\nweave main [] -> Whole:\n  bind x <- call nope 1\n  yield x\n";
    let unknown =
        compile_product_bytecode(unknown_source).expect_err("unknown weave must fail product path");
    assert!(
        unknown.to_string().contains("AE-SEED-011"),
        "expected AE-SEED-011, got {unknown}"
    );
    let unknown_packets = product_error_packets(unknown_source);
    assert_eq!(unknown_packets[0].code, "AE-SEED-011");
    assert_eq!(unknown_packets[0].origin, "seed-speak");

    let root_yield_unknown_source = "world w\n\nweave main [] -> Whole:\n  yield call nope 1\n";
    let root_yield_unknown = compile_product_bytecode(root_yield_unknown_source)
        .expect_err("unknown root-yield weave must fail product path");
    assert!(
        root_yield_unknown.to_string().contains("AE-SEED-011"),
        "expected AE-SEED-011, got {root_yield_unknown}"
    );
    let root_yield_unknown_packets = product_error_packets(root_yield_unknown_source);
    assert_eq!(root_yield_unknown_packets[0].code, "AE-SEED-011");
    assert_eq!(root_yield_unknown_packets[0].origin, "seed-speak");

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
