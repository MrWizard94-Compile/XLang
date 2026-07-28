use aether_core::{
    compile_to_bytecode, compile_with_seed, forge_bytecode, run_bytecode, verify_bytecode,
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

    let variant = SEED_SOURCE.replacen(
        "  bind mutable v65 <- 0\n",
        "  bind mutable v65 <- 0\n  bind mutable v66 <- 0\n",
        1,
    );
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
        include_str!("../../../examples/welcome.ae"),
        include_str!("../../../examples/control-flow.ae"),
        include_str!("../../../examples/weaves.ae"),
        include_str!("../../../examples/unicode.ae"),
        include_str!("../../../examples/seed-multi-weave.ae"),
        include_str!("../../../examples/seed-forward-call.ae"),
    ];
    for source in examples {
        let bootstrap = compile_to_bytecode(source)
            .expect("example must bootstrap")
            .bytecode;
        let seeded = compile_with_seed(source)
            .expect("example must compile through the seed path")
            .bytecode;
        assert_eq!(
            seeded, bootstrap,
            "seed-hosted compile must match bootstrap for shipped examples"
        );
        let run = run_bytecode(&seeded).expect("seed-hosted artifact must run");
        assert!(
            run.exit_code >= 0,
            "seed-hosted example should produce a Whole exit"
        );
    }
}

fn bytes(output: InvocationOutput) -> Vec<u8> {
    let InvocationValue::Bytes(value) = output.value else {
        panic!("Seed Profile compiler ABI requires Bytes output");
    };
    value
}
