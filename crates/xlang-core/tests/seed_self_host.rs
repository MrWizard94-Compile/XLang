use aether_core::{
    compile_to_bytecode, forge_bytecode, verify_bytecode, InvocationOutput, InvocationValue,
};

const SEED_SOURCE: &str = include_str!("../../../seed/aether_seed.ae");
const CHECKED_IN_SEED_ARTIFACT: &[u8] = include_bytes!("../../../seed/aether_seed.aeth");

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
        "  bind mutable v50 <- 0\n",
        "  bind mutable v50 <- 0\n  bind mutable v51 <- 0\n",
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

fn bytes(output: InvocationOutput) -> Vec<u8> {
    let InvocationValue::Bytes(value) = output.value else {
        panic!("Seed Profile compiler ABI requires Bytes output");
    };
    value
}
