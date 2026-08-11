//! M32a bounded local measurements of Aether's verified AETH execution path.
//!
//! This module deliberately embeds a small, reviewed corpus rather than
//! accepting caller-supplied source or artifacts. The VM has no general CLI
//! fuel limit, so that closed input boundary keeps a timing request bounded and
//! capability-free (ADR-104).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use aether_core::{
    compile_product_bytecode, run_bytecode, sha256_hex, verify_bytecode, LANGUAGE_NAME,
    LANGUAGE_VERSION,
};
use serde::Serialize;

const DEFAULT_WARMUP_ITERATIONS: u16 = 3;
const DEFAULT_MEASURED_ITERATIONS: u16 = 11;
const MAX_WARMUP_ITERATIONS: u16 = 100;
const MAX_MEASURED_ITERATIONS: u16 = 1_000;
const BENCHMARK_REPORT_SCHEMA: &str = "aether.benchmark-report/v1";
const MEASUREMENT_SCOPE: &str =
    "verified-aeth-run/v1: verify + decode + execute; seed compilation excluded";

const WELCOME_SOURCE: &str = include_str!("../../../examples/welcome.ae");
const ARENA_BUFFER_SOURCE: &str = include_str!("../../../examples/arena-buffer.ae");
const TASK_LOOP_SOURCE: &str = include_str!("../../../examples/task-loop.ae");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Workload {
    Welcome,
    ArenaBuffer,
    TaskLoop,
}

impl Workload {
    const ALL: [Self; 3] = [Self::Welcome, Self::ArenaBuffer, Self::TaskLoop];

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "welcome" => Ok(Self::Welcome),
            "arena-buffer" => Ok(Self::ArenaBuffer),
            "task-loop" => Ok(Self::TaskLoop),
            _ => Err(format!(
                "bench workload must be all, welcome, arena-buffer, or task-loop; received {value:?}"
            )),
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Welcome => "welcome",
            Self::ArenaBuffer => "arena-buffer",
            Self::TaskLoop => "task-loop",
        }
    }

    const fn source(self) -> &'static str {
        match self {
            Self::Welcome => WELCOME_SOURCE,
            Self::ArenaBuffer => ARENA_BUFFER_SOURCE,
            Self::TaskLoop => TASK_LOOP_SOURCE,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkloadSelection {
    All,
    One(Workload),
}

impl WorkloadSelection {
    fn parse(value: &str) -> Result<Self, String> {
        if value == "all" {
            Ok(Self::All)
        } else {
            Workload::parse(value).map(Self::One)
        }
    }

    fn workloads(self) -> Vec<Workload> {
        match self {
            Self::All => Workload::ALL.to_vec(),
            Self::One(workload) => vec![workload],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BenchOptions {
    selection: WorkloadSelection,
    warmup_iterations: u16,
    measured_iterations: u16,
    report_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BenchCommand {
    List,
    Run(BenchOptions),
}

#[derive(Debug, Serialize)]
struct BenchmarkReport {
    schema: &'static str,
    language: &'static str,
    version: &'static str,
    measurement_scope: &'static str,
    warmup_iterations: u16,
    iterations: u16,
    workloads: Vec<WorkloadReport>,
}

#[derive(Debug, Serialize)]
struct WorkloadReport {
    name: &'static str,
    source_sha256: String,
    artifact_sha256: String,
    artifact_bytes: usize,
    exit_code: i64,
    samples_ns: Vec<u64>,
    min_ns: u64,
    median_ns: u64,
    max_ns: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObservableResult {
    stdout: String,
    exit_code: i64,
}

pub(super) fn execute(arguments: &mut impl Iterator<Item = OsString>) -> Result<(), String> {
    match parse_arguments(arguments)? {
        BenchCommand::List => {
            for workload in Workload::ALL {
                println!("{}", workload.name());
            }
            Ok(())
        }
        BenchCommand::Run(options) => {
            let report = run_benchmarks(&options)?;
            print_summary(&report);
            if let Some(report_path) = options.report_path.as_deref() {
                write_report(&report, report_path)?;
                println!("report={}", report_path.display());
            }
            Ok(())
        }
    }
}

fn parse_arguments(arguments: &mut impl Iterator<Item = OsString>) -> Result<BenchCommand, String> {
    let selection = next_utf8_argument(arguments, "bench selection")?;
    if selection == "--list" {
        if arguments.next().is_some() {
            return Err("bench --list accepts no further arguments".to_owned());
        }
        return Ok(BenchCommand::List);
    }

    let selection = WorkloadSelection::parse(&selection)?;
    let mut warmup_iterations = None;
    let mut measured_iterations = None;
    let mut report_path = None;

    while let Some(flag) = arguments.next() {
        let flag = flag
            .into_string()
            .map_err(|_| "bench option must be valid UTF-8".to_owned())?;
        match flag.as_str() {
            "--warmup" => {
                if warmup_iterations.is_some() {
                    return Err("bench accepts --warmup at most once".to_owned());
                }
                let value = next_utf8_argument(arguments, "--warmup value")?;
                warmup_iterations =
                    Some(parse_count(&value, "--warmup", 0, MAX_WARMUP_ITERATIONS)?);
            }
            "--iterations" => {
                if measured_iterations.is_some() {
                    return Err("bench accepts --iterations at most once".to_owned());
                }
                let value = next_utf8_argument(arguments, "--iterations value")?;
                measured_iterations = Some(parse_count(
                    &value,
                    "--iterations",
                    1,
                    MAX_MEASURED_ITERATIONS,
                )?);
            }
            "--report" => {
                if report_path.is_some() {
                    return Err("bench accepts --report at most once".to_owned());
                }
                let path = arguments
                    .next()
                    .ok_or_else(|| "bench requires a file path after --report".to_owned())?;
                if path.to_str().is_none() {
                    return Err("bench report path must be valid UTF-8".to_owned());
                }
                let path = PathBuf::from(path);
                if path.as_os_str().is_empty() {
                    return Err("bench report path must not be empty".to_owned());
                }
                report_path = Some(path);
            }
            _ => {
                return Err(format!(
                    "bench accepts only --warmup <0..={MAX_WARMUP_ITERATIONS}>, --iterations <1..={MAX_MEASURED_ITERATIONS}>, and --report <file.json>"
                ));
            }
        }
    }

    Ok(BenchCommand::Run(BenchOptions {
        selection,
        warmup_iterations: warmup_iterations.unwrap_or(DEFAULT_WARMUP_ITERATIONS),
        measured_iterations: measured_iterations.unwrap_or(DEFAULT_MEASURED_ITERATIONS),
        report_path,
    }))
}

fn run_benchmarks(options: &BenchOptions) -> Result<BenchmarkReport, String> {
    let workloads = options
        .selection
        .workloads()
        .into_iter()
        .map(|workload| {
            run_workload(
                workload,
                options.warmup_iterations,
                options.measured_iterations,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BenchmarkReport {
        schema: BENCHMARK_REPORT_SCHEMA,
        language: LANGUAGE_NAME,
        version: LANGUAGE_VERSION,
        measurement_scope: MEASUREMENT_SCOPE,
        warmup_iterations: options.warmup_iterations,
        iterations: options.measured_iterations,
        workloads,
    })
}

fn write_report(report: &BenchmarkReport, path: &Path) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "benchmark report parent directory does not exist: {}",
            parent.display()
        ));
    }
    if path.is_dir() {
        return Err(format!(
            "benchmark report path is a directory: {}",
            path.display()
        ));
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| format!("could not serialize benchmark report: {error}"))?;
    fs::write(path, format!("{json}\n")).map_err(|error| {
        format!(
            "could not write benchmark report {}: {error}",
            path.display()
        )
    })
}

fn next_utf8_argument(
    arguments: &mut impl Iterator<Item = OsString>,
    description: &str,
) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("bench requires {description}"))?
        .into_string()
        .map_err(|_| format!("bench {description} must be valid UTF-8"))
}

fn parse_count(value: &str, flag: &str, min: u16, max: u16) -> Result<u16, String> {
    let count = value.parse::<u16>().map_err(|_| {
        format!("bench {flag} must be an integer in {min}..={max}; received {value:?}")
    })?;
    if !(min..=max).contains(&count) {
        return Err(format!(
            "bench {flag} must be in {min}..={max}; received {count}"
        ));
    }
    Ok(count)
}

fn run_workload(
    workload: Workload,
    warmup_iterations: u16,
    measured_iterations: u16,
) -> Result<WorkloadReport, String> {
    let source = workload.source();
    let bytecode = compile_product_bytecode(source).map_err(|error| {
        format!(
            "bench workload {} failed product seed compilation: {error}",
            workload.name()
        )
    })?;
    verify_bytecode(&bytecode).map_err(|error| {
        format!(
            "bench workload {} produced unverifiable AETH: {error}",
            workload.name()
        )
    })?;

    let mut expected = None;
    for _ in 0..warmup_iterations {
        observe_workload(workload, &bytecode, &mut expected)?;
    }

    let mut samples_ns = Vec::with_capacity(usize::from(measured_iterations));
    for _ in 0..measured_iterations {
        let started = Instant::now();
        let observed = run_bytecode(&bytecode).map_err(|error| {
            format!(
                "bench workload {} failed verified execution: {error}",
                workload.name()
            )
        })?;
        let elapsed_ns = u64::try_from(started.elapsed().as_nanos()).map_err(|_| {
            format!(
                "bench workload {} elapsed duration exceeded u64 nanoseconds",
                workload.name()
            )
        })?;
        match_observable_result(workload, &mut expected, observed.stdout, observed.exit_code)?;
        samples_ns.push(elapsed_ns);
    }

    let expected = expected.ok_or_else(|| {
        format!(
            "bench workload {} produced no observable execution result",
            workload.name()
        )
    })?;
    let (min_ns, median_ns, max_ns) = summarize_samples(&samples_ns)?;
    Ok(WorkloadReport {
        name: workload.name(),
        source_sha256: sha256_hex(source.as_bytes()),
        artifact_sha256: sha256_hex(&bytecode),
        artifact_bytes: bytecode.len(),
        exit_code: expected.exit_code,
        samples_ns,
        min_ns,
        median_ns,
        max_ns,
    })
}

fn observe_workload(
    workload: Workload,
    bytecode: &[u8],
    expected: &mut Option<ObservableResult>,
) -> Result<(), String> {
    let observed = run_bytecode(bytecode).map_err(|error| {
        format!(
            "bench workload {} failed verified execution: {error}",
            workload.name()
        )
    })?;
    match_observable_result(workload, expected, observed.stdout, observed.exit_code)
}

fn match_observable_result(
    workload: Workload,
    expected: &mut Option<ObservableResult>,
    stdout: String,
    exit_code: i64,
) -> Result<(), String> {
    let observed = ObservableResult { stdout, exit_code };
    if let Some(expected) = expected {
        if expected != &observed {
            return Err(format!(
                "bench workload {} changed observable stdout or exit code between runs",
                workload.name()
            ));
        }
    } else {
        *expected = Some(observed);
    }
    Ok(())
}

fn summarize_samples(samples_ns: &[u64]) -> Result<(u64, u64, u64), String> {
    let min_ns = *samples_ns
        .iter()
        .min()
        .ok_or_else(|| "bench requires at least one measured sample".to_owned())?;
    let max_ns = *samples_ns
        .iter()
        .max()
        .ok_or_else(|| "bench requires at least one measured sample".to_owned())?;
    let mut sorted = samples_ns.to_vec();
    sorted.sort_unstable();
    let median_ns = sorted[(sorted.len() - 1) / 2];
    Ok((min_ns, median_ns, max_ns))
}

fn print_summary(report: &BenchmarkReport) {
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} benchmark: verified AETH run; seed compilation excluded"
    );
    for workload in &report.workloads {
        println!(
            "workload={} iterations={} min_ns={} median_ns={} max_ns={}",
            workload.name, report.iterations, workload.min_ns, workload.median_ns, workload.max_ns
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TemporaryDirectory {
        path: PathBuf,
    }

    impl TemporaryDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "aether-cli-bench-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temporary benchmark directory should be created");
            Self { path }
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn parse(values: &[&str]) -> Result<BenchCommand, String> {
        let mut arguments = values
            .iter()
            .map(OsString::from)
            .collect::<Vec<_>>()
            .into_iter();
        parse_arguments(&mut arguments)
    }

    #[test]
    fn parser_accepts_closed_selection_and_options() {
        let command = parse(&[
            "all",
            "--warmup",
            "0",
            "--iterations",
            "1",
            "--report",
            "result.json",
        ])
        .expect("closed benchmark command should parse");
        let BenchCommand::Run(options) = command else {
            panic!("expected a benchmark run request");
        };
        assert_eq!(options.selection, WorkloadSelection::All);
        assert_eq!(options.warmup_iterations, 0);
        assert_eq!(options.measured_iterations, 1);
        assert_eq!(options.report_path, Some(PathBuf::from("result.json")));

        assert!(matches!(parse(&["--list"]), Ok(BenchCommand::List)));

        let BenchCommand::Run(defaults) = parse(&["welcome"]).expect("defaults should parse")
        else {
            panic!("expected default benchmark run request");
        };
        assert_eq!(defaults.warmup_iterations, DEFAULT_WARMUP_ITERATIONS);
        assert_eq!(defaults.measured_iterations, DEFAULT_MEASURED_ITERATIONS);
    }

    #[test]
    fn parser_rejects_open_or_malformed_requests() {
        for values in [
            vec![],
            vec!["unknown"],
            vec!["welcome", "--warmup"],
            vec!["welcome", "--warmup", "1", "--warmup", "2"],
            vec!["welcome", "--warmup", "101"],
            vec!["welcome", "--iterations", "0"],
            vec!["welcome", "--iterations", "1001"],
            vec!["welcome", "--report"],
            vec![
                "welcome",
                "--report",
                "first.json",
                "--report",
                "second.json",
            ],
            vec!["--list", "welcome"],
            vec!["welcome", "--source", "outside.ae"],
        ] {
            assert!(
                parse(&values).is_err(),
                "request should fail closed: {values:?}"
            );
        }
    }

    #[test]
    fn one_workload_has_complete_one_sample_evidence() {
        let BenchCommand::Run(options) = parse(&["welcome", "--warmup", "0", "--iterations", "1"])
            .expect("single workload should parse")
        else {
            panic!("expected run request");
        };
        let report = run_benchmarks(&options).expect("embedded welcome workload should run");
        assert_eq!(report.schema, BENCHMARK_REPORT_SCHEMA);
        assert_eq!(report.measurement_scope, MEASUREMENT_SCOPE);
        assert_eq!(report.workloads.len(), 1);
        let workload = &report.workloads[0];
        assert_eq!(workload.name, "welcome");
        assert_eq!(workload.source_sha256.len(), 64);
        assert_eq!(workload.artifact_sha256.len(), 64);
        assert!(workload.artifact_bytes > 0);
        assert_eq!(workload.samples_ns.len(), 1);
        assert_eq!(workload.min_ns, workload.samples_ns[0]);
        assert_eq!(workload.median_ns, workload.samples_ns[0]);
        assert_eq!(workload.max_ns, workload.samples_ns[0]);
    }

    #[test]
    fn aggregate_selection_has_declared_order() {
        let BenchCommand::Run(options) = parse(&["all", "--warmup", "0", "--iterations", "1"])
            .expect("all workloads should parse")
        else {
            panic!("expected run request");
        };
        let report = run_benchmarks(&options).expect("embedded workloads should run");
        let names = report
            .workloads
            .iter()
            .map(|workload| workload.name)
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["welcome", "arena-buffer", "task-loop"]);
    }

    #[test]
    fn report_writer_requires_existing_parent_and_emits_json() {
        let BenchCommand::Run(options) = parse(&["welcome", "--warmup", "0", "--iterations", "1"])
            .expect("workload should parse")
        else {
            panic!("expected run request");
        };
        let report = run_benchmarks(&options).expect("workload should run");
        let temporary = TemporaryDirectory::create();
        let report_path = temporary.path.join("bench.json");
        write_report(&report, &report_path).expect("explicit report path should write");
        let document = fs::read_to_string(&report_path).expect("report should be readable");
        let value: serde_json::Value = serde_json::from_str(&document).expect("valid JSON report");
        assert_eq!(value["schema"], BENCHMARK_REPORT_SCHEMA);
        assert_eq!(value["workloads"][0]["name"], "welcome");

        let missing_parent = temporary.path.join("absent").join("bench.json");
        let error = write_report(&report, &missing_parent).expect_err("missing parent must fail");
        assert!(error.contains("parent directory does not exist"), "{error}");
        assert!(!missing_parent.exists());
    }

    #[test]
    fn median_uses_the_lower_central_sample_for_even_counts() {
        assert_eq!(
            summarize_samples(&[8, 2, 6, 4]).expect("samples"),
            (2, 4, 8)
        );
        assert_eq!(summarize_samples(&[9, 3, 6]).expect("samples"), (3, 6, 9));
        assert!(summarize_samples(&[]).is_err());
    }
}
