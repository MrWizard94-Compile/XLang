//! Offline `aether test` discovery and execution (M17 / ADR-021).

use std::fs;
use std::path::{Path, PathBuf};

use aether_core::{
    compile_with_seed, run_bytecode, verify_bytecode, LANGUAGE_NAME, LANGUAGE_VERSION,
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

/// Seed-compile, verify, and pure-run one test source.
pub fn run_one_test(source_path: &Path) -> TestResult {
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
    let compiled = match compile_with_seed(&source) {
        Ok(output) => output,
        Err(error) => {
            return TestResult {
                path,
                ok: false,
                detail: format!("compile failed: {error}"),
            };
        }
    };
    if let Err(error) = verify_bytecode(&compiled.bytecode) {
        return TestResult {
            path,
            ok: false,
            detail: format!("verify failed: {error}"),
        };
    }
    match run_bytecode(&compiled.bytecode) {
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

pub fn run_tests(paths: &[PathBuf]) -> Result<TestReport, String> {
    let sources = collect_test_sources(paths)?;
    let results = sources.iter().map(|path| run_one_test(path)).collect();
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
