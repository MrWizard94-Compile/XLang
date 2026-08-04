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

const M4_ERROR_EFFECT_SOURCE: &str = include_str!("../../../examples/error-effect.ae");
const M5_COMPTIME_SOURCE: &str = include_str!("../../../examples/comptime.ae");
const M6_LAYOUT_SOURCE: &str = include_str!("../../../examples/layout-table.ae");
const M7_NURSERY_TOTAL_SOURCE: &str = include_str!("../../../examples/nursery-total.ae");
const M7_NURSERY_CANCEL_SOURCE: &str = include_str!("../../../examples/nursery-cancel.ae");

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

    let variant = SEED_SOURCE.replacen(
        "  bind mutable v65 <- 0\n",
        "  bind mutable v65 <- 0\n  bind mutable v103 <- 0\n",
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
        ("layout-table", M6_LAYOUT_SOURCE, Some(10)),
        ("nursery-total", M7_NURSERY_TOTAL_SOURCE, Some(7)),
        ("nursery-cancel", M7_NURSERY_CANCEL_SOURCE, Some(9)),
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
