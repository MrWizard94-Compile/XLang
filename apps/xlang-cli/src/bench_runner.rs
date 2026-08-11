//! Bounded local measurements and profile-bound comparisons of Aether's verified
//! AETH execution path.
//!
//! The runner deliberately embeds a small reviewed corpus rather than accepting
//! caller-supplied source or artifacts. The VM has no general CLI fuel limit, so
//! that closed input boundary keeps a timing request bounded and capability-free
//! (ADR-104). M32b comparison consumes only bounded report data; it never
//! executes a report, source, or artifact (ADR-105).

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

use aether_core::{
    compile_product_bytecode, run_bytecode, sha256_hex, verify_bytecode, LANGUAGE_NAME,
    LANGUAGE_VERSION,
};
use serde::{Deserialize, Serialize};

const DEFAULT_WARMUP_ITERATIONS: u16 = 3;
const DEFAULT_MEASURED_ITERATIONS: u16 = 11;
const MAX_WARMUP_ITERATIONS: u16 = 100;
const MAX_MEASURED_ITERATIONS: u16 = 1_000;
const MAX_COMPARISON_INPUT_BYTES: usize = 262_144;
const MAX_PROFILE_ID_BYTES: usize = 64;
const MAX_VERSION_BYTES: usize = 64;
const MAX_ENVIRONMENT_TOKEN_BYTES: usize = 32;
const MAX_BENCHMARK_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;
const MAX_COMPARABLE_SAMPLE_NS: u64 = i64::MAX as u64;
const BENCHMARK_REPORT_SCHEMA_V1: &str = "aether.benchmark-report/v1";
const BENCHMARK_REPORT_SCHEMA_V2: &str = "aether.benchmark-report/v2";
const BENCHMARK_COMPARISON_SCHEMA_V1: &str = "aether.benchmark-comparison/v1";
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
        Self::from_name(value).ok_or_else(|| {
            format!(
                "bench workload must be all, welcome, arena-buffer, or task-loop; received {value:?}"
            )
        })
    }

    fn from_name(value: &str) -> Option<Self> {
        match value {
            "welcome" => Some(Self::Welcome),
            "arena-buffer" => Some(Self::ArenaBuffer),
            "task-loop" => Some(Self::TaskLoop),
            _ => None,
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
    profile: Option<String>,
    report_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CompareOptions {
    baseline_report_path: PathBuf,
    candidate_report_path: PathBuf,
    report_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BenchCommand {
    List,
    Run(BenchOptions),
    Compare(CompareOptions),
}

#[derive(Debug, Serialize)]
struct BenchmarkReport {
    schema: &'static str,
    language: &'static str,
    version: &'static str,
    measurement_scope: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<BenchmarkEnvironment>,
    warmup_iterations: u16,
    iterations: u16,
    workloads: Vec<WorkloadReport>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkEnvironment {
    operating_system: String,
    architecture: String,
    pointer_width: u32,
    build_profile: String,
}

impl BenchmarkEnvironment {
    fn current() -> Self {
        Self {
            operating_system: std::env::consts::OS.to_owned(),
            architecture: std::env::consts::ARCH.to_owned(),
            pointer_width: usize::BITS,
            build_profile: if cfg!(debug_assertions) {
                "debug".to_owned()
            } else {
                "release".to_owned()
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct WorkloadReport {
    name: &'static str,
    source_sha256: String,
    artifact_sha256: String,
    artifact_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout_sha256: Option<String>,
    exit_code: i64,
    samples_ns: Vec<u64>,
    min_ns: u64,
    median_ns: u64,
    max_ns: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkReportV2 {
    schema: String,
    language: String,
    version: String,
    measurement_scope: String,
    profile: String,
    environment: BenchmarkEnvironment,
    warmup_iterations: u16,
    iterations: u16,
    workloads: Vec<BenchmarkWorkloadV2>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkWorkloadV2 {
    name: String,
    source_sha256: String,
    artifact_sha256: String,
    artifact_bytes: u64,
    stdout_sha256: String,
    exit_code: i64,
    samples_ns: Vec<u64>,
    min_ns: u64,
    median_ns: u64,
    max_ns: u64,
}

#[derive(Debug)]
struct LoadedBenchmarkReport {
    sha256: String,
    report: BenchmarkReportV2,
}

#[derive(Debug, Serialize)]
struct BenchmarkComparisonReport {
    schema: &'static str,
    measurement_scope: &'static str,
    profile: String,
    environment: BenchmarkEnvironment,
    baseline_report_sha256: String,
    candidate_report_sha256: String,
    baseline_version: String,
    candidate_version: String,
    warmup_iterations: u16,
    iterations: u16,
    workloads: Vec<WorkloadComparison>,
}

#[derive(Debug, Serialize)]
struct WorkloadComparison {
    name: String,
    source_sha256: String,
    stdout_sha256: String,
    exit_code: i64,
    baseline_artifact_sha256: String,
    candidate_artifact_sha256: String,
    artifact_identity_changed: bool,
    baseline_median_ns: u64,
    candidate_median_ns: u64,
    median_delta_ns: i64,
    median_delta_ppm: Option<String>,
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
        BenchCommand::Compare(options) => {
            ensure_comparison_output_is_distinct(&options)?;
            let report = compare_benchmarks(&options)?;
            print_comparison_summary(&report);
            if let Some(report_path) = options.report_path.as_deref() {
                write_comparison_report(&report, report_path)?;
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
    if selection == "compare" {
        return parse_compare_arguments(arguments).map(BenchCommand::Compare);
    }

    parse_run_arguments(&selection, arguments).map(BenchCommand::Run)
}

fn parse_run_arguments(
    selection: &str,
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<BenchOptions, String> {
    let selection = WorkloadSelection::parse(selection)?;
    let mut warmup_iterations = None;
    let mut measured_iterations = None;
    let mut profile = None;
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
            "--profile" => {
                if profile.is_some() {
                    return Err("bench accepts --profile at most once".to_owned());
                }
                let value = next_utf8_argument(arguments, "--profile value")?;
                validate_profile_id(&value)?;
                profile = Some(value);
            }
            "--report" => {
                if report_path.is_some() {
                    return Err("bench accepts --report at most once".to_owned());
                }
                report_path = Some(next_path_argument(arguments, "report path")?);
            }
            _ => {
                return Err(format!(
                    "bench accepts only --warmup <0..={MAX_WARMUP_ITERATIONS}>, --iterations <1..={MAX_MEASURED_ITERATIONS}>, --profile <lowercase-ascii-id>, and --report <file.json>"
                ));
            }
        }
    }

    Ok(BenchOptions {
        selection,
        warmup_iterations: warmup_iterations.unwrap_or(DEFAULT_WARMUP_ITERATIONS),
        measured_iterations: measured_iterations.unwrap_or(DEFAULT_MEASURED_ITERATIONS),
        profile,
        report_path,
    })
}

fn parse_compare_arguments(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<CompareOptions, String> {
    let baseline_report_path = next_path_argument(arguments, "baseline report path")?;
    let candidate_report_path = next_path_argument(arguments, "candidate report path")?;
    let mut report_path = None;

    while let Some(flag) = arguments.next() {
        let flag = flag
            .into_string()
            .map_err(|_| "bench comparison option must be valid UTF-8".to_owned())?;
        if flag != "--report" {
            return Err(
                "bench compare accepts only two report paths and optional --report <file.json>"
                    .to_owned(),
            );
        }
        if report_path.is_some() {
            return Err("bench compare accepts --report at most once".to_owned());
        }
        report_path = Some(next_path_argument(arguments, "comparison report path")?);
    }

    Ok(CompareOptions {
        baseline_report_path,
        candidate_report_path,
        report_path,
    })
}

fn run_benchmarks(options: &BenchOptions) -> Result<BenchmarkReport, String> {
    let include_comparison_identity = options.profile.is_some();
    let workloads = options
        .selection
        .workloads()
        .into_iter()
        .map(|workload| {
            run_workload(
                workload,
                options.warmup_iterations,
                options.measured_iterations,
                include_comparison_identity,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BenchmarkReport {
        schema: if include_comparison_identity {
            BENCHMARK_REPORT_SCHEMA_V2
        } else {
            BENCHMARK_REPORT_SCHEMA_V1
        },
        language: LANGUAGE_NAME,
        version: LANGUAGE_VERSION,
        measurement_scope: MEASUREMENT_SCOPE,
        profile: options.profile.clone(),
        environment: include_comparison_identity.then(BenchmarkEnvironment::current),
        warmup_iterations: options.warmup_iterations,
        iterations: options.measured_iterations,
        workloads,
    })
}

fn compare_benchmarks(options: &CompareOptions) -> Result<BenchmarkComparisonReport, String> {
    let baseline = read_benchmark_report_v2(&options.baseline_report_path)?;
    let candidate = read_benchmark_report_v2(&options.candidate_report_path)?;
    build_comparison_report(baseline, candidate)
}

fn build_comparison_report(
    baseline: LoadedBenchmarkReport,
    candidate: LoadedBenchmarkReport,
) -> Result<BenchmarkComparisonReport, String> {
    ensure_reports_are_comparable(&baseline.report, &candidate.report)?;

    let workloads = baseline
        .report
        .workloads
        .iter()
        .zip(&candidate.report.workloads)
        .map(|(baseline_workload, candidate_workload)| {
            let median_delta_ns =
                signed_median_delta_ns(baseline_workload.median_ns, candidate_workload.median_ns)?;
            Ok(WorkloadComparison {
                name: baseline_workload.name.clone(),
                source_sha256: baseline_workload.source_sha256.clone(),
                stdout_sha256: baseline_workload.stdout_sha256.clone(),
                exit_code: baseline_workload.exit_code,
                baseline_artifact_sha256: baseline_workload.artifact_sha256.clone(),
                candidate_artifact_sha256: candidate_workload.artifact_sha256.clone(),
                artifact_identity_changed: baseline_workload.artifact_sha256
                    != candidate_workload.artifact_sha256,
                baseline_median_ns: baseline_workload.median_ns,
                candidate_median_ns: candidate_workload.median_ns,
                median_delta_ns,
                median_delta_ppm: median_delta_ppm(
                    baseline_workload.median_ns,
                    candidate_workload.median_ns,
                ),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(BenchmarkComparisonReport {
        schema: BENCHMARK_COMPARISON_SCHEMA_V1,
        measurement_scope: MEASUREMENT_SCOPE,
        profile: baseline.report.profile,
        environment: baseline.report.environment,
        baseline_report_sha256: baseline.sha256,
        candidate_report_sha256: candidate.sha256,
        baseline_version: baseline.report.version,
        candidate_version: candidate.report.version,
        warmup_iterations: baseline.report.warmup_iterations,
        iterations: baseline.report.iterations,
        workloads,
    })
}

fn read_benchmark_report_v2(path: &Path) -> Result<LoadedBenchmarkReport, String> {
    let bytes = read_bounded_regular_file(path)?;
    let document = std::str::from_utf8(&bytes).map_err(|error| {
        format!(
            "benchmark comparison input {} is not valid UTF-8: {error}",
            path.display()
        )
    })?;
    let report: BenchmarkReportV2 = serde_json::from_str(document).map_err(|error| {
        format!(
            "benchmark comparison input {} is not a strict v2 report: {error}",
            path.display()
        )
    })?;
    validate_benchmark_report_v2(&report).map_err(|error| {
        format!(
            "benchmark comparison input {} is invalid: {error}",
            path.display()
        )
    })?;
    Ok(LoadedBenchmarkReport {
        sha256: sha256_hex(&bytes),
        report,
    })
}

fn read_bounded_regular_file(path: &Path) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|error| {
        format!(
            "could not open benchmark comparison input {}: {error}",
            path.display()
        )
    })?;
    let metadata = file.metadata().map_err(|error| {
        format!(
            "could not inspect benchmark comparison input {}: {error}",
            path.display()
        )
    })?;
    if !metadata.is_file() {
        return Err(format!(
            "benchmark comparison input must be a regular file: {}",
            path.display()
        ));
    }

    let maximum_read = u64::try_from(MAX_COMPARISON_INPUT_BYTES + 1)
        .expect("the fixed comparison input ceiling fits in u64");
    if metadata.len() > maximum_read - 1 {
        return Err(format!(
            "benchmark comparison input exceeds the {MAX_COMPARISON_INPUT_BYTES}-byte limit: {}",
            path.display()
        ));
    }
    let mut bytes = Vec::new();
    file.take(maximum_read)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            format!(
                "could not read benchmark comparison input {}: {error}",
                path.display()
            )
        })?;
    if bytes.len() > MAX_COMPARISON_INPUT_BYTES {
        return Err(format!(
            "benchmark comparison input exceeds the {MAX_COMPARISON_INPUT_BYTES}-byte limit: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn validate_benchmark_report_v2(report: &BenchmarkReportV2) -> Result<(), String> {
    if report.schema != BENCHMARK_REPORT_SCHEMA_V2 {
        return Err(format!(
            "schema must be {BENCHMARK_REPORT_SCHEMA_V2:?}; received {:?}",
            report.schema
        ));
    }
    if report.language != LANGUAGE_NAME {
        return Err(format!(
            "language must be {LANGUAGE_NAME:?}; received {:?}",
            report.language
        ));
    }
    if report.measurement_scope != MEASUREMENT_SCOPE {
        return Err("measurement_scope does not match verified AETH execution".to_owned());
    }
    validate_version(&report.version)?;
    validate_profile_id(&report.profile)?;
    validate_environment(&report.environment)?;
    if report.warmup_iterations > MAX_WARMUP_ITERATIONS {
        return Err(format!(
            "warmup_iterations must be in 0..={MAX_WARMUP_ITERATIONS}"
        ));
    }
    if !(1..=MAX_MEASURED_ITERATIONS).contains(&report.iterations) {
        return Err(format!(
            "iterations must be in 1..={MAX_MEASURED_ITERATIONS}"
        ));
    }
    validate_report_workload_selection(&report.workloads)?;
    for workload in &report.workloads {
        validate_report_workload(workload, report.iterations)?;
    }
    Ok(())
}

fn validate_report_workload_selection(workloads: &[BenchmarkWorkloadV2]) -> Result<(), String> {
    match workloads {
        [workload] => {
            if Workload::from_name(&workload.name).is_none() {
                return Err(format!("unknown workload name {:?}", workload.name));
            }
        }
        [first, second, third]
            if first.name == Workload::Welcome.name()
                && second.name == Workload::ArenaBuffer.name()
                && third.name == Workload::TaskLoop.name() => {}
        [] => {
            return Err(
                "workloads must contain one named workload or all three workloads".to_owned(),
            )
        }
        _ => {
            return Err(
                "workloads must contain one named workload or all three workloads in declared order"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn validate_report_workload(workload: &BenchmarkWorkloadV2, iterations: u16) -> Result<(), String> {
    let workload_kind = Workload::from_name(&workload.name)
        .ok_or_else(|| format!("unknown workload name {:?}", workload.name))?;
    validate_sha256(&workload.source_sha256, "source_sha256")?;
    let expected_source_sha256 = sha256_hex(workload_kind.source().as_bytes());
    if workload.source_sha256 != expected_source_sha256 {
        return Err(format!(
            "workload {:?} source_sha256 does not match the embedded source",
            workload.name
        ));
    }
    validate_sha256(&workload.artifact_sha256, "artifact_sha256")?;
    validate_sha256(&workload.stdout_sha256, "stdout_sha256")?;
    if workload.artifact_bytes == 0 || workload.artifact_bytes > MAX_BENCHMARK_ARTIFACT_BYTES {
        return Err(format!(
            "workload {:?} artifact_bytes must be in 1..={MAX_BENCHMARK_ARTIFACT_BYTES}",
            workload.name
        ));
    }
    if workload.samples_ns.len() != usize::from(iterations) {
        return Err(format!(
            "workload {:?} sample count must equal iterations",
            workload.name
        ));
    }
    if workload
        .samples_ns
        .iter()
        .any(|sample| *sample > MAX_COMPARABLE_SAMPLE_NS)
    {
        return Err(format!(
            "workload {:?} has a sample outside the signed comparison range",
            workload.name
        ));
    }
    let (min_ns, median_ns, max_ns) = summarize_samples(&workload.samples_ns)?;
    if (workload.min_ns, workload.median_ns, workload.max_ns) != (min_ns, median_ns, max_ns) {
        return Err(format!(
            "workload {:?} summary statistics do not match exact samples",
            workload.name
        ));
    }
    Ok(())
}

fn ensure_reports_are_comparable(
    baseline: &BenchmarkReportV2,
    candidate: &BenchmarkReportV2,
) -> Result<(), String> {
    if baseline.profile != candidate.profile {
        return Err("benchmark reports use different profiles".to_owned());
    }
    if baseline.environment != candidate.environment {
        return Err("benchmark reports use different environment fingerprints".to_owned());
    }
    if baseline.warmup_iterations != candidate.warmup_iterations {
        return Err("benchmark reports use different warmup iteration counts".to_owned());
    }
    if baseline.iterations != candidate.iterations {
        return Err("benchmark reports use different measured iteration counts".to_owned());
    }
    if baseline.workloads.len() != candidate.workloads.len() {
        return Err("benchmark reports select different workload sets".to_owned());
    }
    for (baseline_workload, candidate_workload) in
        baseline.workloads.iter().zip(&candidate.workloads)
    {
        if baseline_workload.name != candidate_workload.name {
            return Err("benchmark reports use different workload order".to_owned());
        }
        if baseline_workload.source_sha256 != candidate_workload.source_sha256 {
            return Err(format!(
                "benchmark reports use different source identities for workload {:?}",
                baseline_workload.name
            ));
        }
        if baseline_workload.stdout_sha256 != candidate_workload.stdout_sha256 {
            return Err(format!(
                "benchmark reports use different stdout identities for workload {:?}",
                baseline_workload.name
            ));
        }
        if baseline_workload.exit_code != candidate_workload.exit_code {
            return Err(format!(
                "benchmark reports use different exit codes for workload {:?}",
                baseline_workload.name
            ));
        }
    }
    Ok(())
}

fn ensure_comparison_output_is_distinct(options: &CompareOptions) -> Result<(), String> {
    let Some(report_path) = options.report_path.as_deref() else {
        return Ok(());
    };
    for (role, input_path) in [
        ("baseline", options.baseline_report_path.as_path()),
        ("candidate", options.candidate_report_path.as_path()),
    ] {
        if paths_identify_same_file(report_path, input_path) {
            return Err(format!(
                "bench comparison output path must not overwrite the {role} input report"
            ));
        }
    }
    Ok(())
}

fn paths_identify_same_file(first: &Path, second: &Path) -> bool {
    if first == second {
        return true;
    }
    match (fs::canonicalize(first), fs::canonicalize(second)) {
        (Ok(first), Ok(second)) => first == second,
        _ => false,
    }
}

fn write_report(report: &BenchmarkReport, path: &Path) -> Result<(), String> {
    write_json_report(report, path, "benchmark report")
}

fn write_comparison_report(report: &BenchmarkComparisonReport, path: &Path) -> Result<(), String> {
    write_json_report(report, path, "benchmark comparison report")
}

fn write_json_report<T: Serialize>(report: &T, path: &Path, label: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "{label} parent directory does not exist: {}",
            parent.display()
        ));
    }
    if path.is_dir() {
        return Err(format!("{label} path is a directory: {}", path.display()));
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| format!("could not serialize {label}: {error}"))?;
    fs::write(path, format!("{json}\n"))
        .map_err(|error| format!("could not write {label} {}: {error}", path.display()))
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

fn next_path_argument(
    arguments: &mut impl Iterator<Item = OsString>,
    description: &str,
) -> Result<PathBuf, String> {
    let path = arguments
        .next()
        .ok_or_else(|| format!("bench requires {description}"))?;
    if path.to_str().is_none() {
        return Err(format!("bench {description} must be valid UTF-8"));
    }
    let path = PathBuf::from(path);
    if path.as_os_str().is_empty() {
        return Err(format!("bench {description} must not be empty"));
    }
    Ok(path)
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

fn validate_profile_id(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > MAX_PROFILE_ID_BYTES {
        return Err(format!(
            "bench --profile must contain 1..={MAX_PROFILE_ID_BYTES} lowercase ASCII characters"
        ));
    }
    if !value.bytes().all(is_profile_byte) {
        return Err(
            "bench --profile must use only lowercase ASCII letters, digits, '.', '_', or '-'"
                .to_owned(),
        );
    }
    let bytes = value.as_bytes();
    if !is_profile_edge_byte(bytes[0]) || !is_profile_edge_byte(bytes[bytes.len() - 1]) {
        return Err(
            "bench --profile must begin and end with a lowercase ASCII letter or digit".to_owned(),
        );
    }
    Ok(())
}

const fn is_profile_byte(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
}

const fn is_profile_edge_byte(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit()
}

fn validate_version(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > MAX_VERSION_BYTES {
        return Err(format!(
            "version must contain 1..={MAX_VERSION_BYTES} ASCII SemVer characters"
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+'))
    {
        return Err("version must use only ASCII letters, digits, '.', '-', or '+'".to_owned());
    }
    Ok(())
}

fn validate_environment(environment: &BenchmarkEnvironment) -> Result<(), String> {
    validate_environment_token(
        &environment.operating_system,
        "environment.operating_system",
    )?;
    validate_environment_token(&environment.architecture, "environment.architecture")?;
    if !matches!(environment.pointer_width, 32 | 64) {
        return Err("environment.pointer_width must be 32 or 64".to_owned());
    }
    if !matches!(environment.build_profile.as_str(), "debug" | "release") {
        return Err("environment.build_profile must be debug or release".to_owned());
    }
    Ok(())
}

fn validate_environment_token(value: &str, field: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > MAX_ENVIRONMENT_TOKEN_BYTES {
        return Err(format!(
            "{field} must contain 1..={MAX_ENVIRONMENT_TOKEN_BYTES} lowercase ASCII characters"
        ));
    }
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')
    }) {
        return Err(format!(
            "{field} must use only lowercase ASCII letters, digits, '_', or '-'"
        ));
    }
    Ok(())
}

fn validate_sha256(value: &str, field: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(format!(
            "{field} must be lowercase 64-character SHA-256 hex"
        ));
    }
    Ok(())
}

fn run_workload(
    workload: Workload,
    warmup_iterations: u16,
    measured_iterations: u16,
    include_comparison_identity: bool,
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
    let artifact_bytes = u64::try_from(bytecode.len()).map_err(|_| {
        format!(
            "bench workload {} artifact size could not fit the report format",
            workload.name()
        )
    })?;
    if artifact_bytes == 0 || artifact_bytes > MAX_BENCHMARK_ARTIFACT_BYTES {
        return Err(format!(
            "bench workload {} artifact size is outside the M32b comparison bound",
            workload.name()
        ));
    }

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
        if elapsed_ns > MAX_COMPARABLE_SAMPLE_NS {
            return Err(format!(
                "bench workload {} elapsed duration exceeded the M32b signed comparison bound",
                workload.name()
            ));
        }
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
        artifact_bytes,
        stdout_sha256: include_comparison_identity.then(|| sha256_hex(expected.stdout.as_bytes())),
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

fn signed_median_delta_ns(baseline_ns: u64, candidate_ns: u64) -> Result<i64, String> {
    let baseline_ns = i64::try_from(baseline_ns)
        .map_err(|_| "baseline median is outside the signed comparison range".to_owned())?;
    let candidate_ns = i64::try_from(candidate_ns)
        .map_err(|_| "candidate median is outside the signed comparison range".to_owned())?;
    candidate_ns
        .checked_sub(baseline_ns)
        .ok_or_else(|| "median delta is outside the signed comparison range".to_owned())
}

fn median_delta_ppm(baseline_ns: u64, candidate_ns: u64) -> Option<String> {
    if baseline_ns == 0 {
        return None;
    }
    let delta = i128::from(candidate_ns) - i128::from(baseline_ns);
    let parts_per_million = delta * 1_000_000 / i128::from(baseline_ns);
    Some(parts_per_million.to_string())
}

fn print_summary(report: &BenchmarkReport) {
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} benchmark: verified AETH run; seed compilation excluded"
    );
    if let Some(profile) = &report.profile {
        println!("profile={profile}");
    }
    for workload in &report.workloads {
        println!(
            "workload={} iterations={} min_ns={} median_ns={} max_ns={}",
            workload.name, report.iterations, workload.min_ns, workload.median_ns, workload.max_ns
        );
    }
}

fn print_comparison_summary(report: &BenchmarkComparisonReport) {
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} benchmark comparison: profile={} baseline_version={} candidate_version={}",
        report.profile, report.baseline_version, report.candidate_version
    );
    for workload in &report.workloads {
        println!(
            "workload={} baseline_median_ns={} candidate_median_ns={} median_delta_ns={} median_delta_ppm={} artifact_identity_changed={}",
            workload.name,
            workload.baseline_median_ns,
            workload.candidate_median_ns,
            workload.median_delta_ns,
            workload.median_delta_ppm.as_deref().unwrap_or("undefined"),
            workload.artifact_identity_changed,
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

    fn fixture_workload(workload: Workload, samples_ns: Vec<u64>) -> BenchmarkWorkloadV2 {
        let (min_ns, median_ns, max_ns) = summarize_samples(&samples_ns).expect("fixture samples");
        BenchmarkWorkloadV2 {
            name: workload.name().to_owned(),
            source_sha256: sha256_hex(workload.source().as_bytes()),
            artifact_sha256: "a".repeat(64),
            artifact_bytes: 128,
            stdout_sha256: "b".repeat(64),
            exit_code: 0,
            samples_ns,
            min_ns,
            median_ns,
            max_ns,
        }
    }

    fn fixture_report() -> BenchmarkReportV2 {
        BenchmarkReportV2 {
            schema: BENCHMARK_REPORT_SCHEMA_V2.to_owned(),
            language: LANGUAGE_NAME.to_owned(),
            version: LANGUAGE_VERSION.to_owned(),
            measurement_scope: MEASUREMENT_SCOPE.to_owned(),
            profile: "win11-rust-1.88-release-fixed".to_owned(),
            environment: BenchmarkEnvironment::current(),
            warmup_iterations: 3,
            iterations: 3,
            workloads: vec![fixture_workload(Workload::Welcome, vec![100, 200, 300])],
        }
    }

    fn loaded_fixture(report: BenchmarkReportV2) -> LoadedBenchmarkReport {
        validate_benchmark_report_v2(&report).expect("fixture report should be valid");
        let serialized = serde_json::to_vec(&report).expect("fixture should serialize");
        LoadedBenchmarkReport {
            sha256: sha256_hex(&serialized),
            report,
        }
    }

    fn write_fixture(path: &Path, report: &BenchmarkReportV2) {
        let serialized = serde_json::to_string(report).expect("fixture should serialize");
        fs::write(path, serialized).expect("fixture report should write");
    }

    #[test]
    fn parser_accepts_closed_selection_options_profile_and_compare() {
        let command = parse(&[
            "all",
            "--warmup",
            "0",
            "--iterations",
            "1",
            "--profile",
            "win11-rust-1.88-release-fixed",
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
        assert_eq!(
            options.profile.as_deref(),
            Some("win11-rust-1.88-release-fixed")
        );
        assert_eq!(options.report_path, Some(PathBuf::from("result.json")));

        assert!(matches!(parse(&["--list"]), Ok(BenchCommand::List)));

        let command = parse(&[
            "compare",
            "baseline.json",
            "candidate.json",
            "--report",
            "comparison.json",
        ])
        .expect("closed comparison command should parse");
        let BenchCommand::Compare(options) = command else {
            panic!("expected a benchmark comparison request");
        };
        assert_eq!(options.baseline_report_path, PathBuf::from("baseline.json"));
        assert_eq!(
            options.candidate_report_path,
            PathBuf::from("candidate.json")
        );
        assert_eq!(options.report_path, Some(PathBuf::from("comparison.json")));

        let BenchCommand::Run(defaults) = parse(&["welcome"]).expect("defaults should parse")
        else {
            panic!("expected default benchmark run request");
        };
        assert_eq!(defaults.warmup_iterations, DEFAULT_WARMUP_ITERATIONS);
        assert_eq!(defaults.measured_iterations, DEFAULT_MEASURED_ITERATIONS);
        assert!(defaults.profile.is_none());
    }

    #[test]
    fn parser_rejects_open_malformed_or_cross_mode_requests() {
        let overlong_profile = "a".repeat(MAX_PROFILE_ID_BYTES + 1);
        let cases = vec![
            vec![],
            vec!["unknown"],
            vec!["welcome", "--warmup"],
            vec!["welcome", "--warmup", "1", "--warmup", "2"],
            vec!["welcome", "--warmup", "101"],
            vec!["welcome", "--iterations", "0"],
            vec!["welcome", "--iterations", "1001"],
            vec!["welcome", "--profile"],
            vec!["welcome", "--profile", "Uppercase"],
            vec!["welcome", "--profile", "contains space"],
            vec!["welcome", "--profile", ".profile"],
            vec!["welcome", "--profile", "profile-"],
            vec!["welcome", "--profile", "one", "--profile", "two"],
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
            vec!["compare"],
            vec!["compare", "baseline.json"],
            vec![
                "compare",
                "baseline.json",
                "candidate.json",
                "--profile",
                "local",
            ],
            vec![
                "compare",
                "baseline.json",
                "candidate.json",
                "--report",
                "first.json",
                "--report",
                "second.json",
            ],
        ];
        for values in cases {
            assert!(
                parse(&values).is_err(),
                "request should fail closed: {values:?}"
            );
        }
        assert!(parse(&["welcome", "--profile", &overlong_profile]).is_err());
    }

    #[test]
    fn unprofiled_workload_has_complete_v1_one_sample_evidence() {
        let BenchCommand::Run(options) = parse(&["welcome", "--warmup", "0", "--iterations", "1"])
            .expect("single workload should parse")
        else {
            panic!("expected run request");
        };
        let report = run_benchmarks(&options).expect("embedded welcome workload should run");
        assert_eq!(report.schema, BENCHMARK_REPORT_SCHEMA_V1);
        assert_eq!(report.measurement_scope, MEASUREMENT_SCOPE);
        assert!(report.profile.is_none());
        assert!(report.environment.is_none());
        assert_eq!(report.workloads.len(), 1);
        let workload = &report.workloads[0];
        assert_eq!(workload.name, "welcome");
        assert_eq!(workload.source_sha256.len(), 64);
        assert_eq!(workload.artifact_sha256.len(), 64);
        assert!(workload.artifact_bytes > 0);
        assert!(workload.stdout_sha256.is_none());
        assert_eq!(workload.samples_ns.len(), 1);
        assert_eq!(workload.min_ns, workload.samples_ns[0]);
        assert_eq!(workload.median_ns, workload.samples_ns[0]);
        assert_eq!(workload.max_ns, workload.samples_ns[0]);
    }

    #[test]
    fn profiled_workload_has_complete_v2_comparison_identity() {
        let BenchCommand::Run(options) = parse(&[
            "welcome",
            "--warmup",
            "0",
            "--iterations",
            "1",
            "--profile",
            "win11-rust-1.88-release-fixed",
        ])
        .expect("profiled workload should parse") else {
            panic!("expected run request");
        };
        let report = run_benchmarks(&options).expect("embedded welcome workload should run");
        assert_eq!(report.schema, BENCHMARK_REPORT_SCHEMA_V2);
        assert_eq!(
            report.profile.as_deref(),
            Some("win11-rust-1.88-release-fixed")
        );
        assert_eq!(report.environment, Some(BenchmarkEnvironment::current()));
        assert_eq!(
            report.workloads[0].stdout_sha256.as_deref().map(str::len),
            Some(64)
        );

        let serialized = serde_json::to_value(&report).expect("report should serialize");
        assert!(serialized["workloads"][0].get("stdout").is_none());
        assert_eq!(
            serialized["workloads"][0]["stdout_sha256"]
                .as_str()
                .map(str::len),
            Some(64)
        );
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
    fn report_writer_requires_existing_parent_and_preserves_v1_shape() {
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
        assert_eq!(value["schema"], BENCHMARK_REPORT_SCHEMA_V1);
        assert_eq!(value["workloads"][0]["name"], "welcome");
        assert!(value.get("profile").is_none());
        assert!(value["workloads"][0].get("stdout_sha256").is_none());

        let missing_parent = temporary.path.join("absent").join("bench.json");
        let error = write_report(&report, &missing_parent).expect_err("missing parent must fail");
        assert!(error.contains("parent directory does not exist"), "{error}");
        assert!(!missing_parent.exists());
    }

    #[test]
    fn strict_v2_reader_accepts_valid_bounded_report_and_rejects_invalid_inputs() {
        let temporary = TemporaryDirectory::create();
        let valid_path = temporary.path.join("valid.json");
        let valid = fixture_report();
        write_fixture(&valid_path, &valid);
        let loaded = read_benchmark_report_v2(&valid_path).expect("v2 fixture should read");
        assert_eq!(loaded.report, valid);
        assert_eq!(loaded.sha256.len(), 64);

        let v1_path = temporary.path.join("v1.json");
        let mut v1 = fixture_report();
        v1.schema = BENCHMARK_REPORT_SCHEMA_V1.to_owned();
        write_fixture(&v1_path, &v1);
        assert!(read_benchmark_report_v2(&v1_path).is_err());

        let unknown_path = temporary.path.join("unknown.json");
        let mut unknown = serde_json::to_value(fixture_report()).expect("fixture value");
        unknown
            .as_object_mut()
            .expect("fixture object")
            .insert("unexpected".to_owned(), serde_json::Value::Bool(true));
        fs::write(
            &unknown_path,
            serde_json::to_string(&unknown).expect("unknown fixture should serialize"),
        )
        .expect("unknown fixture should write");
        assert!(read_benchmark_report_v2(&unknown_path).is_err());

        let invalid_digest_path = temporary.path.join("invalid-digest.json");
        let mut invalid_digest = fixture_report();
        invalid_digest.workloads[0].artifact_sha256 = "not-a-digest".to_owned();
        write_fixture(&invalid_digest_path, &invalid_digest);
        assert!(read_benchmark_report_v2(&invalid_digest_path).is_err());

        let wrong_source_path = temporary.path.join("wrong-source.json");
        let mut wrong_source = fixture_report();
        wrong_source.workloads[0].source_sha256 = "c".repeat(64);
        write_fixture(&wrong_source_path, &wrong_source);
        assert!(read_benchmark_report_v2(&wrong_source_path).is_err());

        let invalid_statistics_path = temporary.path.join("invalid-statistics.json");
        let mut invalid_statistics = fixture_report();
        invalid_statistics.workloads[0].median_ns = 201;
        write_fixture(&invalid_statistics_path, &invalid_statistics);
        assert!(read_benchmark_report_v2(&invalid_statistics_path).is_err());

        let invalid_samples_path = temporary.path.join("invalid-samples.json");
        let mut invalid_samples = fixture_report();
        invalid_samples.workloads[0].samples_ns.pop();
        write_fixture(&invalid_samples_path, &invalid_samples);
        assert!(read_benchmark_report_v2(&invalid_samples_path).is_err());

        let invalid_order_path = temporary.path.join("invalid-order.json");
        let mut invalid_order = fixture_report();
        invalid_order.workloads = vec![
            fixture_workload(Workload::ArenaBuffer, vec![100, 200, 300]),
            fixture_workload(Workload::Welcome, vec![100, 200, 300]),
            fixture_workload(Workload::TaskLoop, vec![100, 200, 300]),
        ];
        write_fixture(&invalid_order_path, &invalid_order);
        assert!(read_benchmark_report_v2(&invalid_order_path).is_err());

        let oversized_path = temporary.path.join("oversized.json");
        fs::write(&oversized_path, vec![b' '; MAX_COMPARISON_INPUT_BYTES + 1])
            .expect("oversized fixture should write");
        let error =
            read_benchmark_report_v2(&oversized_path).expect_err("oversized input must fail");
        assert!(error.contains("byte limit"), "{error}");

        let directory_path = temporary.path.join("report-directory");
        fs::create_dir(&directory_path).expect("directory fixture should create");
        assert!(read_benchmark_report_v2(&directory_path).is_err());
    }

    #[test]
    fn comparison_accepts_matching_reports_and_exposes_artifact_change() {
        let baseline = fixture_report();
        let mut candidate = baseline.clone();
        candidate.workloads[0].artifact_sha256 = "c".repeat(64);
        candidate.workloads[0].samples_ns = vec![110, 210, 310];
        let (min_ns, median_ns, max_ns) =
            summarize_samples(&candidate.workloads[0].samples_ns).expect("candidate samples");
        candidate.workloads[0].min_ns = min_ns;
        candidate.workloads[0].median_ns = median_ns;
        candidate.workloads[0].max_ns = max_ns;

        let comparison =
            build_comparison_report(loaded_fixture(baseline), loaded_fixture(candidate))
                .expect("matching reports should compare");
        assert_eq!(comparison.schema, BENCHMARK_COMPARISON_SCHEMA_V1);
        assert_eq!(comparison.workloads.len(), 1);
        let workload = &comparison.workloads[0];
        assert!(workload.artifact_identity_changed);
        assert_eq!(workload.median_delta_ns, 10);
        assert_eq!(workload.median_delta_ppm.as_deref(), Some("50000"));
        assert_eq!(comparison.baseline_report_sha256.len(), 64);
        assert_eq!(comparison.candidate_report_sha256.len(), 64);
    }

    #[test]
    fn comparison_rejects_context_or_behavior_mismatches() {
        let baseline = fixture_report();

        let mut wrong_profile = baseline.clone();
        wrong_profile.profile = "win11-rust-1.88-debug-fixed".to_owned();
        assert!(build_comparison_report(
            loaded_fixture(baseline.clone()),
            loaded_fixture(wrong_profile)
        )
        .is_err());

        let mut wrong_environment = baseline.clone();
        wrong_environment.environment.architecture = "arm64".to_owned();
        assert!(build_comparison_report(
            loaded_fixture(baseline.clone()),
            loaded_fixture(wrong_environment)
        )
        .is_err());

        let mut wrong_policy = baseline.clone();
        wrong_policy.iterations = 1;
        wrong_policy.workloads[0].samples_ns = vec![200];
        wrong_policy.workloads[0].min_ns = 200;
        wrong_policy.workloads[0].median_ns = 200;
        wrong_policy.workloads[0].max_ns = 200;
        assert!(build_comparison_report(
            loaded_fixture(baseline.clone()),
            loaded_fixture(wrong_policy)
        )
        .is_err());

        let mut wrong_selection = baseline.clone();
        wrong_selection.workloads =
            vec![fixture_workload(Workload::ArenaBuffer, vec![100, 200, 300])];
        assert!(build_comparison_report(
            loaded_fixture(baseline.clone()),
            loaded_fixture(wrong_selection)
        )
        .is_err());

        let mut wrong_stdout = baseline.clone();
        wrong_stdout.workloads[0].stdout_sha256 = "c".repeat(64);
        assert!(build_comparison_report(
            loaded_fixture(baseline.clone()),
            loaded_fixture(wrong_stdout)
        )
        .is_err());

        let mut wrong_exit = baseline;
        wrong_exit.workloads[0].exit_code = 1;
        assert!(build_comparison_report(
            loaded_fixture(fixture_report()),
            loaded_fixture(wrong_exit)
        )
        .is_err());
    }

    #[test]
    fn comparison_output_never_overwrites_an_input_report() {
        let temporary = TemporaryDirectory::create();
        let baseline = temporary.path.join("baseline.json");
        let candidate = temporary.path.join("candidate.json");
        write_fixture(&baseline, &fixture_report());
        write_fixture(&candidate, &fixture_report());
        let options = CompareOptions {
            baseline_report_path: baseline.clone(),
            candidate_report_path: candidate,
            report_path: Some(baseline),
        };
        let error = ensure_comparison_output_is_distinct(&options)
            .expect_err("input report must not be an output target");
        assert!(error.contains("must not overwrite"), "{error}");
    }

    #[test]
    fn comparison_writer_requires_existing_parent_and_emits_json() {
        let comparison = build_comparison_report(
            loaded_fixture(fixture_report()),
            loaded_fixture(fixture_report()),
        )
        .expect("matching reports should compare");
        let temporary = TemporaryDirectory::create();
        let report_path = temporary.path.join("comparison.json");
        write_comparison_report(&comparison, &report_path)
            .expect("explicit comparison report path should write");
        let document =
            fs::read_to_string(&report_path).expect("comparison report should be readable");
        let value: serde_json::Value = serde_json::from_str(&document).expect("valid JSON report");
        assert_eq!(value["schema"], BENCHMARK_COMPARISON_SCHEMA_V1);
        assert_eq!(value["workloads"][0]["median_delta_ns"], 0);

        let missing_parent = temporary.path.join("absent").join("comparison.json");
        let error = write_comparison_report(&comparison, &missing_parent)
            .expect_err("missing parent must fail");
        assert!(error.contains("parent directory does not exist"), "{error}");
        assert!(!missing_parent.exists());
    }

    #[test]
    fn median_uses_lower_central_sample_and_comparison_arithmetic_is_total() {
        assert_eq!(
            summarize_samples(&[8, 2, 6, 4]).expect("samples"),
            (2, 4, 8)
        );
        assert_eq!(summarize_samples(&[9, 3, 6]).expect("samples"), (3, 6, 9));
        assert!(summarize_samples(&[]).is_err());

        assert_eq!(signed_median_delta_ns(200, 210), Ok(10));
        assert_eq!(signed_median_delta_ns(210, 200), Ok(-10));
        assert_eq!(median_delta_ppm(200, 210).as_deref(), Some("50000"));
        assert_eq!(median_delta_ppm(3, 4).as_deref(), Some("333333"));
        assert_eq!(median_delta_ppm(0, 4), None);
    }
}
