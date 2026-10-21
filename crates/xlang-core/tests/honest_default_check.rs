//! ADR-133: default single-file check/compile must reject every program the
//! bootstrap validator rejects, and the reported text must name a source line
//! without leaking VM unpack failures.

use aether_core::{
    compile_product_bytecode, compile_product_checked, compile_source,
    product_single_file_check_fail_closed, CompilerError,
};
use std::fs;
use std::path::Path;

const CORPUS: &[(&str, &str, &str)] = &[
    (
        "junk after yield",
        "world w\n\nweave main [] -> Whole:\n  yield 1\n  bad thing\n",
        "unknown Aether statement",
    ),
    (
        "junk before yield",
        "world w\n\nweave main [] -> Whole:\n  bad thing\n  yield 1\n",
        "unknown Aether statement",
    ),
    (
        "undefined yield",
        "world w\n\nweave main [] -> Whole:\n  yield missing_name\n",
        "has not been bound",
    ),
    (
        "undefined bind",
        "world w\n\nweave main [] -> Whole:\n  bind value <- missing_name\n  yield value\n",
        "has not been bound",
    ),
    (
        "undefined revise",
        "world w\n\nweave main [] -> Whole:\n  bind value <- 1\n  revise missing <- 2\n  yield value\n",
        "has not been introduced",
    ),
    (
        "undefined speak",
        "world w\n\nweave main [] -> Whole:\n  speak missing_name\n  yield 0\n",
        "has not been bound",
    ),
    (
        "type text yield",
        "world w\n\nweave main [] -> Whole:\n  yield \"text\"\n",
        "requires Whole",
    ),
    (
        "type truth result",
        "world w\n\nweave flag [] -> Truth:\n  yield 1\n\nweave main [] -> Whole:\n  yield 0\n",
        "requires Truth",
    ),
    (
        "type sum",
        "world w\n\nweave main [] -> Whole:\n  yield sum \"a\" 1\n",
        "requires Whole",
    ),
    (
        "missing world",
        "weave main [] -> Whole:\n  yield 1\n",
        "top-level world",
    ),
    (
        "missing main",
        "world w\n\nweave helper [] -> Whole:\n  yield 1\n",
        "weave main",
    ),
    (
        "arity too few",
        "world w\n\nweave add [left: Whole, right: Whole] -> Whole:\n  yield sum left right\n\nweave main [] -> Whole:\n  yield call add 1\n",
        "requires 2 argument",
    ),
    (
        "arity too many",
        "world w\n\nweave add [left: Whole, right: Whole] -> Whole:\n  yield sum left right\n\nweave main [] -> Whole:\n  yield call add 1 2 3\n",
        "requires 2 argument",
    ),
    (
        "unknown weave",
        "world w\n\nweave main [] -> Whole:\n  bind value <- call nope 1\n  yield value\n",
        "has not been declared",
    ),
    (
        "bad record field",
        "world w\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card 7\n  bind score <- field borrow value missing\n  yield score\n",
        "has no field named",
    ),
    (
        "record field type",
        "world w\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card \"nope\"\n  yield 0\n",
        "requires Whole",
    ),
    (
        "unknown record",
        "world w\n\nweave main [] -> Whole:\n  bind value <- make nope 1\n  yield 0\n",
        "has not been declared",
    ),
    (
        "duplicate record field",
        "world w\n\nrecord card [score: Whole, score: Whole]\n\nweave main [] -> Whole:\n  yield 1\n",
        "declared more than once",
    ),
    (
        "duplicate shape field",
        "world w\n\nshape point:\n  x Whole\n  x Whole\n\nweave main [] -> Whole:\n  yield 1\n",
        "declared more than once",
    ),
    (
        "shape field type",
        "world w\n\nshape point:\n  x Text\n\nweave main [] -> Whole:\n  yield 1\n",
        "may use only Whole",
    ),
    (
        "raise without effect",
        "world w\n\nweave main [] -> Whole:\n  raise 1\n",
        "AE-EFFECT-001",
    ),
    (
        "unsupported effect phrase",
        "world w\n\nweave main [] -> Whole raises Text:\n  yield 1\n",
        "raises Whole",
    ),
    (
        "forward without effect",
        "world w\n\nweave boom [value: Whole] -> Whole:\n  yield value\n\nweave main [] -> Whole:\n  forward call boom 1\n",
        "raises Whole",
    ),
    (
        "ordinary call of raising weave",
        "world w\n\nweave boom [value: Whole] -> Whole raises Whole:\n  raise value\n\nweave main [] -> Whole:\n  bind value <- call boom 1\n  yield value\n",
        "AE-EFFECT-001",
    ),
    (
        "handle of total weave",
        "world w\n\nweave boom [value: Whole] -> Whole:\n  yield value\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call boom 1 into success otherwise error into code\n",
        "AE-EFFECT-002",
    ),
    (
        "odd indent",
        "world w\n\nweave main [] -> Whole:\n yield 1\n",
        "indentation",
    ),
    (
        "legacy fn",
        "world w\n\nfn main() -> Int { return 0; }\n",
        "legacy",
    ),
    (
        "empty weave body",
        "world w\n\nweave main [] -> Whole:\n",
        "at least one body statement",
    ),
    (
        "duplicate weave",
        "world w\n\nweave main [] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  yield 2\n",
        "declared more than once",
    ),
    (
        "yield then speak",
        "world w\n\nweave main [] -> Whole:\n  yield 1\n  speak \"later\"\n",
        "yield is allowed only",
    ),
    (
        "checkpoint outside task",
        "world w\n\nweave main [] -> Whole:\n  checkpoint\n  yield 1\n",
        "AE-TASK-004",
    ),
    (
        "together without spawn",
        "world w\n\nweave main [] -> Whole:\n  together:\n    yield 1\n  yield 0\n",
        "AE-TASK-001",
    ),
    (
        "raw import",
        "world w\n\nimport unit \"lib.ae\" as lib\n\nweave main [] -> Whole:\n  yield 1\n",
        "import unit",
    ),
    (
        "yield in truth choose",
        "world w\n\nweave main [] -> Whole:\n  bind x <- 7\n  choose same x 7:\n    yield 42\n  otherwise:\n    yield -1\n",
        "truth-condition choose",
    ),
    (
        "task without checkpoint",
        "world w\n\ntask weave worker [] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  yield 0\n",
        "checkpoint",
    ),
    (
        "reserved timeout",
        "world w\n\nweave main [] -> Whole:\n  timeout 1\n  yield 1\n",
        "reserved future task surface",
    ),
    (
        "empty source",
        "   \n\n",
        "empty",
    ),
];

#[test]
fn honest_default_check_corpus_rejects_with_located_text() {
    assert!(product_single_file_check_fail_closed());
    assert!(
        CORPUS.len() >= 20,
        "the invalid-program corpus must contain at least 20 cases, got {}",
        CORPUS.len()
    );
    for (name, source, needle) in CORPUS {
        assert_honest(name, source, needle);
    }
}

#[test]
fn honest_default_check_keeps_valid_seed_bytes() {
    let source = "world w\n\nweave main [] -> Whole:\n  yield 1\n";
    let checked = compile_product_checked(source).expect("valid source");
    let forged = compile_product_bytecode(source).expect("valid forge");
    assert_eq!(checked, forged);
    compile_source(source).expect("bootstrap validator accepts the valid source");

    let resource = "world w\n\nweave main [] -> Whole:\n  bind memory <- arena 32\n  bind mutable values <- buffer Whole\n  bind mutable observed <- 0\n  choose allocate access memory move values 1 into values:\n    choose append move values 7 into values:\n      choose at borrow values 0 into observed:\n        yield observed\n      otherwise:\n        yield -3\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
    let checked = compile_product_checked(resource).expect("resource choose yield");
    assert_eq!(
        checked,
        compile_product_bytecode(resource).expect("resource forge")
    );
}

#[test]
fn honest_default_check_leaves_framed_inputs_on_the_seed_route() {
    let catalog = "aether.seed-modules/v1\nnot-a-catalog\n";
    let direct = compile_product_bytecode(catalog)
        .expect_err("framed catalog must fail")
        .to_string();
    let checked = compile_product_checked(catalog)
        .expect_err("framed catalog must fail")
        .to_string();
    assert_eq!(checked, direct);
    let bootstrap = compile_source(catalog)
        .expect_err("bootstrap does not parse a catalog")
        .to_string();
    assert_ne!(
        checked, bootstrap,
        "framed input must not be rewritten into the bootstrap diagnostic"
    );
}

#[test]
fn honest_default_check_accepts_shipped_examples() {
    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut saw = 0usize;
    for entry in fs::read_dir(&examples).expect("examples directory") {
        let entry = entry.expect("example entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("ae") {
            continue;
        }
        let source = fs::read_to_string(&path).expect("example source");
        compile_source(&source).unwrap_or_else(|error| {
            panic!("{} failed the bootstrap validator: {error}", path.display())
        });
        let checked = compile_product_checked(&source)
            .unwrap_or_else(|error| panic!("{} failed default check: {error}", path.display()));
        let forged = compile_product_bytecode(&source)
            .unwrap_or_else(|error| panic!("{} failed the seed forge: {error}", path.display()));
        assert_eq!(checked, forged, "{}", path.display());
        saw += 1;
    }
    assert!(saw >= 20, "expected the shipped examples, saw {saw}");
}

#[test]
fn honest_default_check_accepts_the_seed_source() {
    let source = include_str!("../../../seed/aether_seed.ae");
    compile_source(source).expect("bootstrap validator accepts the seed");
    let checked = compile_product_checked(source).expect("default check accepts the seed");
    let forged = compile_product_bytecode(source).expect("seed forge");
    assert_eq!(checked, forged);
}

fn assert_honest(name: &str, source: &str, needle: &str) {
    let error = compile_product_checked(source).expect_err(&format!("{name} passed default check"));
    assert_located(name, source, &error, needle);
    let bootstrap = compile_source(source).expect_err(&format!("{name} bootstrap accepted"));
    let _ = bootstrap;
    if name.starts_with("junk") {
        compile_product_bytecode(source).unwrap_or_else(|error| {
            panic!("{name} was expected to show the seed hole, got {error}")
        });
    }
    if name.starts_with("undefined") {
        let raw = compile_product_bytecode(source)
            .expect_err("undefined names already fail the forge")
            .to_string();
        assert!(
            raw.contains("AE-SEED-008")
                || raw.contains("AE-SEED-001")
                || raw.to_ascii_lowercase().contains("unpack"),
            "{name} forge diagnostic changed before the oracle: {raw}"
        );
    }
}

fn assert_located(name: &str, source: &str, error: &CompilerError, needle: &str) {
    let text = error.to_string();
    let lower = text.to_ascii_lowercase();
    assert!(text.contains("line "), "{name}: {text}");
    assert!(text.contains("column "), "{name}: {text}");
    assert!(text.contains(needle), "{name}: {text}");
    assert!(!lower.contains("unpack16"), "{name}: {text}");
    assert!(!lower.contains("unpack32"), "{name}: {text}");
    assert!(!text.contains("index is invalid"), "{name}: {text}");
    assert!(!text.contains("AE-SEED-008"), "{name}: {text}");
    assert!(!text.contains("AE-SEED-001"), "{name}: {text}");
    assert!(!text.contains("AE-SEED-002"), "{name}: {text}");
    let lines = source.lines().count().max(1);
    assert!(
        error.span.line >= 1 && error.span.line <= lines,
        "{name}: span {} is outside {lines} lines: {text}",
        error.span
    );
    assert!(error.span.column >= 1, "{name}: {text}");
}
