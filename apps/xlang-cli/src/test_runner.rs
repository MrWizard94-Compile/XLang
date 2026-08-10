//! Offline `aether test` discovery and execution (M17 / ADR-021).

use std::fs;
use std::path::{Path, PathBuf};

use aether_core::{
    compile_product_bytecode, run_bytecode, run_bytecode_with_grants, verify_bytecode,
    HostGrantConfig, LANGUAGE_NAME, LANGUAGE_VERSION,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestResult {
    pub path: PathBuf,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestReport {
    pub results: Vec<TestResult>,
}

impl TestReport {
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|result| result.ok).count()
    }

    pub fn failed(&self) -> usize {
        self.results.iter().filter(|result| !result.ok).count()
    }

    pub fn all_passed(&self) -> bool {
        !self.results.is_empty() && self.failed() == 0
    }
}

/// Collect test sources from caller-selected paths (M17 discovery).
pub fn collect_test_sources(paths: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let mut collected = Vec::new();
    if paths.is_empty() {
        collect_from_path(Path::new("."), &mut collected)?;
    } else {
        for path in paths {
            collect_from_path(path, &mut collected)?;
        }
    }
    collected.sort();
    collected.dedup();
    if collected.is_empty() {
        return Err(
            "no test sources found (expected *_test.ae under directories, or explicit .ae files)"
                .to_owned(),
        );
    }
    Ok(collected)
}

fn collect_from_path(path: &Path, collected: &mut Vec<PathBuf>) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("test path not found: {}", path.display()));
    }
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) != Some("ae") {
            return Err(format!(
                "test file must be an Aether source (.ae): {}",
                path.display()
            ));
        }
        collected.push(path.to_path_buf());
        return Ok(());
    }
    if path.is_dir() {
        let root = path.canonicalize().map_err(|error| {
            format!(
                "could not resolve test directory {}: {error}",
                path.display()
            )
        })?;
        // Walk the caller-selected path so reported paths stay relative when possible.
        walk_test_dir(&root, path, collected)?;
        return Ok(());
    }
    Err(format!(
        "test path is neither a file nor a directory: {}",
        path.display()
    ))
}

fn walk_test_dir(root: &Path, dir: &Path, collected: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|error| format!("could not read directory {}: {error}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("could not read directory entry: {error}"))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|error| format!("could not stat {}: {error}", path.display()))?;
        // Skip symlinks to reduce path-escape risk (M17-INV-002).
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            let Ok(canon) = path.canonicalize() else {
                continue;
            };
            if !canon.starts_with(root) {
                continue;
            }
            walk_test_dir(root, &canon, collected)?;
            continue;
        }
        if metadata.is_file() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if name.ends_with("_test.ae") {
                if let Ok(canon) = path.canonicalize() {
                    if canon.starts_with(root) {
                        collected.push(path);
                    }
                } else {
                    collected.push(path);
                }
            }
        }
    }
    Ok(())
}

/// Seed-compile, verify, and run one test with optional M14 grants (M17c).
/// Empty grants install pure fixtures only.
pub fn run_one_test_with_grants(source_path: &Path, grants: HostGrantConfig) -> TestResult {
    let path = source_path.to_path_buf();
    let source = match fs::read_to_string(source_path) {
        Ok(source) => source,
        Err(error) => {
            return TestResult {
                path,
                ok: false,
                detail: format!("could not read source: {error}"),
            };
        }
    };
    let bytecode = match compile_product_bytecode(&source) {
        Ok(bytes) => bytes,
        Err(error) => {
            return TestResult {
                path,
                ok: false,
                detail: format!("compile failed: {error}"),
            };
        }
    };
    if let Err(error) = verify_bytecode(&bytecode) {
        return TestResult {
            path,
            ok: false,
            detail: format!("verify failed: {error}"),
        };
    }
    let run_result = if grants.read_roots.is_empty()
        && grants.write_roots.is_empty()
        && grants.env_names.is_empty()
    {
        run_bytecode(&bytecode)
    } else {
        run_bytecode_with_grants(&bytecode, grants)
    };
    match run_result {
        Ok(output) if output.exit_code == 0 => TestResult {
            path,
            ok: true,
            detail: "exit 0".to_owned(),
        },
        Ok(output) => TestResult {
            path,
            ok: false,
            detail: format!("exit {}", output.exit_code),
        },
        Err(error) => TestResult {
            path,
            ok: false,
            detail: format!("run failed: {error}"),
        },
    }
}

/// Discover and run tests with optional grants (M17c). Empty grants = pure.
pub fn run_tests_with_grants(
    paths: &[PathBuf],
    grants: HostGrantConfig,
) -> Result<TestReport, String> {
    let sources = collect_test_sources(paths)?;
    let results = sources
        .iter()
        .map(|path| run_one_test_with_grants(path, grants.clone()))
        .collect();
    Ok(TestReport { results })
}

pub fn print_report(report: &TestReport) {
    for result in &report.results {
        if result.ok {
            println!("ok   {}", result.path.display());
        } else {
            println!("FAIL {}: {}", result.path.display(), result.detail);
        }
    }
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} test: {} passed; {} failed",
        report.passed(),
        report.failed()
    );
}

/// Native structured report schema id (M17d).
pub const TEST_REPORT_SCHEMA: &str = "aether.test-report/v1";

fn json_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out
}

fn xml_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            c => out.push(c),
        }
    }
    out
}

/// Serialize the suite as `aether.test-report/v1` JSON (M17d).
#[must_use]
pub fn format_report_json(report: &TestReport) -> String {
    let mut body = String::new();
    body.push_str("{\n");
    body.push_str(&format!(
        "  \"schema\": \"{TEST_REPORT_SCHEMA}\",\n  \"language\": \"{LANGUAGE_NAME}\",\n  \"version\": \"{LANGUAGE_VERSION}\",\n"
    ));
    body.push_str(&format!(
        "  \"passed\": {},\n  \"failed\": {},\n  \"results\": [\n",
        report.passed(),
        report.failed()
    ));
    for (index, result) in report.results.iter().enumerate() {
        if index > 0 {
            body.push_str(",\n");
        }
        let path = json_escape(&result.path.display().to_string());
        let detail = json_escape(&result.detail);
        let ok = if result.ok { "true" } else { "false" };
        body.push_str(&format!(
            "    {{\"path\": \"{path}\", \"ok\": {ok}, \"detail\": \"{detail}\"}}"
        ));
    }
    body.push_str("\n  ]\n}\n");
    body
}

/// Serialize the suite as a bounded JUnit-compatible XML document (M17d).
#[must_use]
pub fn format_report_junit(report: &TestReport) -> String {
    let mut body = String::new();
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    body.push_str(&format!(
        "<testsuite name=\"aether-test\" tests=\"{}\" failures=\"{}\" errors=\"0\">\n",
        report.results.len(),
        report.failed()
    ));
    for result in &report.results {
        let name = xml_escape(&result.path.display().to_string());
        if result.ok {
            body.push_str(&format!("  <testcase name=\"{name}\"/>\n"));
        } else {
            let message = xml_escape(&result.detail);
            body.push_str(&format!(
                "  <testcase name=\"{name}\">\n    <failure message=\"{message}\"/>\n  </testcase>\n"
            ));
        }
    }
    body.push_str("</testsuite>\n");
    body
}

/// Write structured reports to explicit paths (M17d). Parent dirs must exist.
pub fn write_structured_reports(
    report: &TestReport,
    json_path: Option<&Path>,
    junit_path: Option<&Path>,
) -> Result<(), String> {
    if let Some(path) = json_path {
        fs::write(path, format_report_json(report)).map_err(|error| {
            format!(
                "could not write JSON test report {}: {error}",
                path.display()
            )
        })?;
    }
    if let Some(path) = junit_path {
        fs::write(path, format_report_junit(report)).map_err(|error| {
            format!(
                "could not write JUnit test report {}: {error}",
                path.display()
            )
        })?;
    }
    Ok(())
}
