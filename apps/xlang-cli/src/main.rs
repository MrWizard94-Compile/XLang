use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod lsp;
mod test_runner;

use aether_core::{
    apply_edit_cli_trusts_product_accept, apply_structural_edit, canonical_ast,
    check_product_base_gate, compile_product_bytecode, compile_project_modules, compile_source,
    compile_to_bytecode, compile_workspace_package, forge_bytecode, format_project, format_source,
    format_source_product, lower_verified_aeth_to_c, multi_module_authority_note,
    parse_project_document, parse_workspace_document, pin_local_package,
    product_cli_check_without_bootstrap, product_format_without_bootstrap,
    product_project_format_without_bootstrap, product_structure_json,
    product_structure_without_bootstrap, refresh_project_lock, refresh_workspace_lock,
    run_bytecode, run_bytecode_with_grants, run_project_tests_with_grants,
    serialize_project_document, serialize_workspace_document, structural_document_json,
    unit_artifact_file_name, verify_bytecode, verify_project, verify_registry_cache,
    verify_workspace, HostGrantConfig, InvocationValue, LANGUAGE_NAME, LANGUAGE_VERSION,
};

fn usage() {
    eprintln!(
        "Usage:\n  aether check <source-file>\n  aether check --product <source-file>\n  aether structure <source-file>\n  aether structure --product <source-file>\n  aether apply-edit <source-file> <edit-file> --output <source-file>\n  aether format <source-file> [--output <source-file>]\n  aether format --product <source-file> [--output <source-file>]\n  aether project verify <project-file> [--output-dir <dir>]\n  aether project format <project-file> [--write] [--product]\n  aether project lock <project-file> [--write]\n  aether project build <project-file> --output <artifact-file>\n  aether project test <project-file>\n  aether workspace verify <workspace-file>\n  aether workspace lock <workspace-file> [--write]\n  aether workspace build <workspace-file> --package <name> --output <artifact-file>\n  aether compile <source-file> --output <artifact-file> [--bootstrap|--native-c]\n  aether registry verify-cache <cache-root>\n  aether registry pin-local <cache-root> --name <n> --version <v> --artifact <path>\n  aether forge <compiler-artifact> <source-file> --output <artifact-file>\n  aether run <artifact-file> [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]... [--grant-lib KEY=PATH]...\n  aether test [path...] [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]... [--grant-lib KEY=PATH]... [--report <file.json>] [--report-junit <file.xml>]\n  aether lsp\n  aether version\n\ncheck uses bootstrap full diagnostics + canonical AST by default.\ncheck --product validates via the seed product path only (forge + verify + AE-SEED preflights; no bootstrap AST).\nformat uses bootstrap AST-canonical rewrite by default.\nformat --product LF-normalizes and product-accepts only (no bootstrap AST rewrite).\ncompile uses the Aether-written seed compiler by default for single-file sources (including M21 foreign weave pilot; seed≡bootstrap proven for examples/foreign-pilot.ae).\nM19e task source emits AETH v12; source without task frames retains AETH v11.\nstructure emits aether.ast/v8 JSON (bootstrap). structure --product emits aether.product-structure/v1 (seed accept + LF source; no AST).\napply-edit accepts aether.edit/v8 (including statement-level ops), bootstrap-canonical base parse, product seed accept in core before write (CLI does not re-forge).\nproject verify is offline: schema, nested path confinement, optional SHA-256 lock; module units validated for M11.\nproject lock derives a complete local unit lock after verification; --write is required to replace the project manifest.\nproject build elaborates import unit / export weave graphs then seed-compiles (M11b; dual-compare is test/oracle only).\nproject test elaborates each role:test unit as entry (M11b dual-compare), pure-runs; pass requires exit 0 (M17b); optional --grant-* (M17c); optional --report / --report-junit (M17d).\nproject format prints canonical source per unit; --write overwrites listed unit paths only; --product uses seed product format per unit (no bootstrap AST rewrite).\nworkspace verify is offline multi-package integrity (aether.workspace/v1): path-jail package roots, acyclic depends_on, nested project verify (M18).\nworkspace lock pins every package's project identity and requires nested project locks; --write is required to replace the workspace manifest.\nworkspace build elaborates one package main cone with M22 import unit from package (depends_on only), seed dual-compare; locked workspaces verify before artifact output.\naether test discovers *_test.ae under directories (or runs explicit .ae files), seed-compiles, pure-runs; pass requires exit 0 (M17); optional --grant-* (M17c); optional --report / --report-junit (M17d).\naether lsp [--project <aether.project.json>] is an offline stdio Language Server (product-primary diagnostics ADR-058; bootstrap AST for symbols/format/hover; project-aware import definition/hover; no product AETH emit; no silent disk writes).\naether run grants: M14 I/O roots/names and M21 --grant-lib KEY=PATH (explicit library file; no PATH search). Empty grants keep pure fixtures only.\nPass --bootstrap to emit with the Rust bootstrap (seed rebuild / diagnostics / dual-compare proofs).\nPass --native-c to lower verified AETH to ISO C (F-NATIVE M35a pure Whole pilot; not default).\nregistry pin-local/verify-cache are offline-only F-REGISTRY M24a (no network)."
    );
}

fn read_source(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn read_artifact(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn check(source_path: &Path, product: bool) -> Result<(), String> {
    let source = read_source(source_path)?;
    if product {
        debug_assert!(
            product_cli_check_without_bootstrap(),
            "ADR-051: product check must not require bootstrap"
        );
        let bytecode = compile_product_bytecode(&source).map_err(|error| error.to_string())?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} product check passed: {} byte(s) via seed path in {}",
            bytecode.len(),
            source_path.display()
        );
        return Ok(());
    }
    // ADR-063: when both product and bootstrap reject, prefer product AE-SEED codes.
    debug_assert!(
        check_product_base_gate(),
        "ADR-063: check product base gate"
    );
    let product_result = compile_product_bytecode(&source);
    let program = match compile_source(&source) {
        Ok(program) => program,
        Err(bootstrap_error) => {
            if let Err(product_error) = product_result {
                return Err(product_error.to_string());
            }
            return Err(bootstrap_error.to_string());
        }
    };
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} check passed: {} significant token(s) in {} (bootstrap diagnostics)",
        program.significant_token_count(),
        source_path.display()
    );
    println!("{}", canonical_ast(&program));
    Ok(())
}

fn compile(
    source_path: &Path,
    output_path: &Path,
    use_bootstrap: bool,
    native_c: bool,
) -> Result<(), String> {
    let source = read_source(source_path)?;
    // BARP Phase 2 (ADR-044): default product compile forges seed bytecode without
    // a bootstrap validate precondition. --bootstrap remains rebuild/oracle path.
    // ADR-059: --native-c lowers verified AETH to ISO C (F-NATIVE pure pilot).
    if native_c && use_bootstrap {
        return Err("compile accepts either --bootstrap or --native-c, not both".to_owned());
    }
    let bytecode = if use_bootstrap {
        compile_to_bytecode(&source)
            .map_err(|error| error.to_string())?
            .bytecode
    } else {
        compile_product_bytecode(&source).map_err(|error| error.to_string())?
    };
    if native_c {
        let c_source = lower_verified_aeth_to_c(&bytecode).map_err(|error| error.to_string())?;
        write_source(output_path, &c_source)?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} lowered verified AETH to C {} (F-NATIVE M35a)",
            output_path.display()
        );
        return Ok(());
    }
    write_artifact(output_path, bytecode)?;
    let engine = if use_bootstrap { "bootstrap" } else { "seed" };
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} compiled {} to {} ({engine})",
        source_path.display(),
        output_path.display()
    );
    Ok(())
}

fn registry_verify_cache(cache_root: &Path) -> Result<(), String> {
    let document = verify_registry_cache(cache_root).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry cache verified {} package pin(s) at {}",
        document.packages.len(),
        cache_root.display()
    );
    for package in &document.packages {
        println!(
            "  {}@{} sha256={} artifact={}",
            package.name, package.version, package.sha256, package.artifact
        );
    }
    Ok(())
}

fn registry_pin_local(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact: &Path,
) -> Result<(), String> {
    let pin = pin_local_package(cache_root, name, version, artifact)
        .map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry pinned {}@{} -> {} (sha256={})",
        pin.name, pin.version, pin.artifact, pin.sha256
    );
    Ok(())
}

fn structure(source_path: &Path, product: bool) -> Result<(), String> {
    let source = read_source(source_path)?;
    let document = if product {
        debug_assert!(
            product_structure_without_bootstrap(),
            "ADR-054: product structure must not require bootstrap"
        );
        product_structure_json(&source).map_err(|error| error.to_string())?
    } else {
        structural_document_json(&source).map_err(|error| error.to_string())?
    };
    println!("{document}");
    Ok(())
}

fn apply_edit(source_path: &Path, edit_path: &Path, output_path: &Path) -> Result<(), String> {
    let source = read_source(source_path)?;
    let edit = read_source(edit_path)?;
    // ADR-048/053: core apply_structural_edit product-seed accepts; CLI does not re-forge.
    debug_assert!(
        apply_edit_cli_trusts_product_accept(),
        "ADR-053: apply-edit CLI must trust core product accept"
    );
    let result = apply_structural_edit(&source, &edit).map_err(|error| error.to_string())?;
    write_source(output_path, &result.source)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} applied {} validated structural edit(s) from {} to {}",
        result.operation_count,
        edit_path.display(),
        output_path.display()
    );
    Ok(())
}

fn write_artifact(output_path: &Path, artifact: Vec<u8>) -> Result<(), String> {
    verify_bytecode(&artifact).map_err(|error| {
        format!(
            "refusing to write an invalid Aether artifact to {}: {error}",
            output_path.display()
        )
    })?;
    let parent = output_path
        .parent()
        .filter(|candidate| !candidate.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "output directory {} does not exist",
            parent.display()
        ));
    }
    fs::write(output_path, artifact)
        .map_err(|error| format!("could not write {}: {error}", output_path.display()))?;
    Ok(())
}

fn write_source(output_path: &Path, source: &str) -> Result<(), String> {
    let parent = output_path
        .parent()
        .filter(|candidate| !candidate.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "output directory {} does not exist",
            parent.display()
        ));
    }
    fs::write(output_path, source)
        .map_err(|error| format!("could not write {}: {error}", output_path.display()))?;
    Ok(())
}

fn forge(compiler_path: &Path, source_path: &Path, output_path: &Path) -> Result<(), String> {
    let compiler = read_artifact(compiler_path)?;
    let source = read_source(source_path)?;
    let output = forge_bytecode(&compiler, &source).map_err(|error| error.to_string())?;
    if !output.stdout.is_empty() {
        eprint!("{}", output.stdout);
    }
    let InvocationValue::Bytes(artifact) = output.value else {
        return Err("the compiler weave must yield Bytes".to_owned());
    };
    write_artifact(output_path, artifact)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} forged {} with {} to {}",
        source_path.display(),
        compiler_path.display(),
        output_path.display()
    );
    Ok(())
}

fn execute_artifact(artifact_path: &Path, grants: HostGrantConfig) -> Result<(), String> {
    let artifact = read_artifact(artifact_path)?;
    let pure = grants.read_roots.is_empty()
        && grants.write_roots.is_empty()
        && grants.env_names.is_empty()
        && grants.library_grants.is_empty();
    let output = if pure {
        run_bytecode(&artifact).map_err(|error| error.to_string())?
    } else {
        run_bytecode_with_grants(&artifact, grants).map_err(|error| error.to_string())?
    };
    print!("{}", output.stdout);
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} exited with {}",
        output.exit_code
    );
    Ok(())
}

fn require_existing_grant_root(path: &Path, kind: &str) -> Result<PathBuf, String> {
    if !path.is_dir() {
        return Err(format!(
            "{kind} grant root must be an existing directory: {}",
            path.display()
        ));
    }
    path.canonicalize().map_err(|error| {
        format!(
            "could not resolve {kind} grant root {}: {error}",
            path.display()
        )
    })
}

fn parse_one_grant_flag(
    flag: &str,
    arguments: &mut impl Iterator<Item = OsString>,
    grants: &mut HostGrantConfig,
) -> Result<bool, String> {
    match flag {
        "--grant-read" => {
            let root = next_argument(arguments, "--grant-read directory")?;
            grants
                .read_roots
                .push(require_existing_grant_root(Path::new(&root), "read")?);
            Ok(true)
        }
        "--grant-write" => {
            let root = next_argument(arguments, "--grant-write directory")?;
            grants
                .write_roots
                .push(require_existing_grant_root(Path::new(&root), "write")?);
            Ok(true)
        }
        "--grant-env" => {
            let name = next_argument(arguments, "--grant-env NAME")?;
            let name = name
                .into_string()
                .map_err(|_| "grant-env NAME must be valid UTF-8".to_owned())?;
            if name.is_empty() || name.len() > 256 {
                return Err("grant-env NAME must be non-empty and at most 256 bytes".to_owned());
            }
            if !name
                .bytes()
                .all(|byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_'))
            {
                return Err(
                    "grant-env NAME may contain only ASCII letters, digits, and underscore"
                        .to_owned(),
                );
            }
            grants.env_names.push(name);
            Ok(true)
        }
        "--grant-lib" => {
            let binding = next_argument(arguments, "--grant-lib KEY=PATH")?;
            let binding = binding
                .into_string()
                .map_err(|_| "grant-lib KEY=PATH must be valid UTF-8".to_owned())?;
            let Some((key, path)) = binding.split_once('=') else {
                return Err(
                    "grant-lib requires KEY=PATH where KEY matches foreign from \"KEY\"".to_owned(),
                );
            };
            if key.is_empty() || key.len() > 128 {
                return Err("grant-lib KEY must be 1..=128 characters".to_owned());
            }
            if !key.bytes().all(
                |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-'),
            ) {
                return Err(
                    "grant-lib KEY may contain only letters, digits, underscore, dot, and hyphen"
                        .to_owned(),
                );
            }
            let path = PathBuf::from(path);
            if !path.is_file() {
                return Err(format!(
                    "grant-lib PATH must be an existing library file: {}",
                    path.display()
                ));
            }
            let absolute = path.canonicalize().map_err(|error| {
                format!(
                    "could not resolve grant-lib path {}: {error}",
                    path.display()
                )
            })?;
            grants.library_grants.insert(key.to_owned(), absolute);
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn parse_run_grants(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<HostGrantConfig, String> {
    let mut grants = HostGrantConfig::default();
    while let Some(flag) = arguments.next() {
        let flag = flag.to_string_lossy().into_owned();
        if parse_one_grant_flag(&flag, arguments, &mut grants)? {
            continue;
        }
        return Err(format!(
            "run accepts optional --grant-read/--grant-write/--grant-env after the artifact; unknown flag {flag}"
        ));
    }
    Ok(grants)
}

#[derive(Default)]
struct TestCliOptions {
    paths: Vec<PathBuf>,
    grants: HostGrantConfig,
    report_json: Option<PathBuf>,
    report_junit: Option<PathBuf>,
}

/// Parse remaining CLI args as test paths, optional M17c grants, and M17d reports.
fn parse_test_args(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<TestCliOptions, String> {
    let mut options = TestCliOptions::default();
    while let Some(argument) = arguments.next() {
        let text = argument.to_string_lossy().into_owned();
        if parse_one_grant_flag(&text, arguments, &mut options.grants)? {
            continue;
        }
        match text.as_str() {
            "--report" => {
                let path = next_argument(arguments, "--report file")?;
                options.report_json = Some(PathBuf::from(path));
            }
            "--report-junit" => {
                let path = next_argument(arguments, "--report-junit file")?;
                options.report_junit = Some(PathBuf::from(path));
            }
            other if other.starts_with("--") => {
                return Err(format!(
                    "test accepts paths, optional --grant-*, --report, and --report-junit; unknown flag {other}"
                ));
            }
            _ => options.paths.push(PathBuf::from(argument)),
        }
    }
    Ok(options)
}

fn parse_project_test_tail(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<(HostGrantConfig, Option<PathBuf>, Option<PathBuf>), String> {
    let mut grants = HostGrantConfig::default();
    let mut report_json = None;
    let mut report_junit = None;
    while let Some(argument) = arguments.next() {
        let text = argument.to_string_lossy().into_owned();
        if parse_one_grant_flag(&text, arguments, &mut grants)? {
            continue;
        }
        match text.as_str() {
            "--report" => {
                let path = next_argument(arguments, "--report file")?;
                report_json = Some(PathBuf::from(path));
            }
            "--report-junit" => {
                let path = next_argument(arguments, "--report-junit file")?;
                report_junit = Some(PathBuf::from(path));
            }
            other => {
                return Err(format!(
                    "project test accepts optional --grant-*, --report, and --report-junit; unknown flag {other}"
                ));
            }
        }
    }
    Ok((grants, report_json, report_junit))
}

fn format_file(
    source_path: &Path,
    output_path: Option<&Path>,
    product: bool,
) -> Result<(), String> {
    let source = read_source(source_path)?;
    let formatted = if product {
        debug_assert!(
            product_format_without_bootstrap(),
            "ADR-053: product format must not require bootstrap"
        );
        format_source_product(&source).map_err(|error| error.to_string())?
    } else {
        format_source(&source).map_err(|error| error.to_string())?
    };
    if let Some(output) = output_path {
        write_source(output, &formatted)?;
        let mode = if product { "product " } else { "" };
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} {mode}formatted {} to {}",
            source_path.display(),
            output.display()
        );
    } else {
        print!("{formatted}");
    }
    Ok(())
}

fn project_root_for(project_path: &Path) -> &Path {
    project_path
        .parent()
        .filter(|candidate| !candidate.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn workspace_verify(workspace_path: &Path) -> Result<(), String> {
    let json = read_source(workspace_path)?;
    let document = parse_workspace_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(workspace_path);
    let report = verify_workspace(root, &document).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} workspace {}@{} verified {} package(s)",
        report.name,
        report.version,
        report.packages.len()
    );
    for package in &report.packages {
        println!(
            "  package {} path {} project {}@{} units {}",
            package.name,
            package.path,
            package.project.name,
            package.project.version,
            package.project.units.len()
        );
    }
    Ok(())
}

fn workspace_build(
    workspace_path: &Path,
    package_name: &str,
    output_path: &Path,
) -> Result<(), String> {
    let json = read_source(workspace_path)?;
    let document = parse_workspace_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(workspace_path);
    let bytecode = compile_workspace_package(root, &document, package_name)
        .map_err(|error| error.to_string())?;
    write_artifact(output_path, bytecode)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} workspace {} package {} built {} (seed multi-module via elaboration; M22)",
        document.name,
        package_name,
        output_path.display()
    );
    Ok(())
}

fn workspace_lock(workspace_path: &Path, write: bool) -> Result<(), String> {
    let json = read_source(workspace_path)?;
    let document = parse_workspace_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(workspace_path);
    let refreshed = refresh_workspace_lock(root, &document).map_err(|error| error.to_string())?;
    let rendered = serialize_workspace_document(&refreshed).map_err(|error| error.to_string())?;
    if write {
        write_source(workspace_path, &rendered)?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} refreshed workspace lock in {}",
            workspace_path.display()
        );
    } else {
        print!("{rendered}");
    }
    Ok(())
}

fn project_verify(project_path: &Path, output_dir: Option<&Path>) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    let report = verify_project(root, &document).map_err(|error| error.to_string())?;
    if let Some(dir) = output_dir {
        if !dir.is_dir() {
            return Err(format!("output directory {} does not exist", dir.display()));
        }
        for unit in &document.units {
            let source_path = aether_core::resolve_unit_path(root, &unit.path)
                .map_err(|error| error.to_string())?;
            let source = read_source(&source_path)?;
            let bytecode = compile_product_bytecode(&source).map_err(|error| error.to_string())?;
            let artifact_path = dir.join(unit_artifact_file_name(&unit.path));
            write_artifact(&artifact_path, bytecode)?;
        }
    }
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} project {}@{} verified {} unit(s)",
        report.name,
        report.version,
        report.units.len()
    );
    for unit in &report.units {
        let role = match unit.role {
            aether_core::ProjectUnitRole::Main => "main",
            aether_core::ProjectUnitRole::Lib => "lib",
            aether_core::ProjectUnitRole::Test => "test",
        };
        println!(
            "  [{role}] {} sha256={} artifact_bytes={}",
            unit.path, unit.sha256, unit.artifact_bytes
        );
    }
    Ok(())
}

fn project_lock(project_path: &Path, write: bool) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    let refreshed = refresh_project_lock(root, &document).map_err(|error| error.to_string())?;
    let rendered = serialize_project_document(&refreshed).map_err(|error| error.to_string())?;
    if write {
        write_source(project_path, &rendered)?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} refreshed project lock in {}",
            project_path.display()
        );
    } else {
        print!("{rendered}");
    }
    Ok(())
}

fn project_test(
    project_path: &Path,
    grants: HostGrantConfig,
    report_json: Option<&Path>,
    report_junit: Option<&Path>,
) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    let report = run_project_tests_with_grants(root, &document, grants)
        .map_err(|error| error.to_string())?;
    for result in &report.results {
        if result.ok {
            println!("ok   {}", result.path);
        } else {
            println!("FAIL {}: {}", result.path, result.detail);
        }
    }
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} project {}@{} test: {} passed; {} failed",
        document.name,
        document.version,
        report.passed(),
        report.failed()
    );
    // Reuse test_runner serialization for a uniform on-disk schema.
    let adapted = test_runner::TestReport {
        results: report
            .results
            .iter()
            .map(|result| test_runner::TestResult {
                path: PathBuf::from(&result.path),
                ok: result.ok,
                detail: result.detail.clone(),
            })
            .collect(),
    };
    test_runner::write_structured_reports(&adapted, report_json, report_junit)?;
    if report.all_passed() {
        Ok(())
    } else {
        Err("project test suite failed".to_owned())
    }
}

fn project_build(project_path: &Path, output_path: &Path) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    let bytecode = compile_project_modules(root, &document).map_err(|error| error.to_string())?;
    write_artifact(output_path, bytecode)?;
    println!("{}", multi_module_authority_note());
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} project {}@{} built {} (seed multi-module via elaboration)",
        document.name,
        document.version,
        output_path.display()
    );
    Ok(())
}

fn project_format(project_path: &Path, write: bool, product: bool) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    if product {
        debug_assert!(
            product_project_format_without_bootstrap(),
            "ADR-054: product project format must not require bootstrap"
        );
    }
    let report = format_project(root, &document, product).map_err(|error| error.to_string())?;
    let mode = if product { "product " } else { "" };
    if write {
        for unit in &report.units {
            let path = aether_core::resolve_unit_path(root, &unit.path)
                .map_err(|error| error.to_string())?;
            write_source(&path, &unit.formatted)?;
            println!(
                "{LANGUAGE_NAME} {LANGUAGE_VERSION} {mode}formatted {}",
                unit.path
            );
        }
    } else {
        for unit in &report.units {
            println!("=== {} ===", unit.path);
            print!("{}", unit.formatted);
            if !unit.formatted.ends_with('\n') {
                println!();
            }
        }
    }
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} project {}@{} {mode}formatted {} unit(s)",
        report.name,
        report.version,
        report.units.len()
    );
    Ok(())
}

fn next_argument(
    arguments: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<OsString, String> {
    arguments.next().ok_or_else(|| format!("missing {name}"))
}

fn parse_optional_write_flag(
    arguments: &mut impl Iterator<Item = OsString>,
    command: &str,
) -> Result<bool, String> {
    let Some(flag) = arguments.next() else {
        return Ok(false);
    };
    if flag != "--write" || arguments.next().is_some() {
        return Err(format!("{command} accepts an optional --write flag only"));
    }
    Ok(true)
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let command = next_argument(&mut arguments, "command")?;
    match command.to_string_lossy().as_ref() {
        "check" => {
            let first = next_argument(&mut arguments, "source file or --product")?;
            let (product, source) = if first == "--product" {
                let source = next_argument(&mut arguments, "source file")?;
                (true, source)
            } else {
                let mut product = false;
                if let Some(extra) = arguments.next() {
                    if extra == "--product" {
                        product = true;
                    } else {
                        return Err(
                            "check accepts <source-file> or --product <source-file>".to_owned()
                        );
                    }
                }
                (product, first)
            };
            if arguments.next().is_some() {
                return Err("check accepts <source-file> or --product <source-file>".to_owned());
            }
            check(Path::new(&source), product)
        }
        "structure" => {
            let first = next_argument(&mut arguments, "source file or --product")?;
            let (product, source) = if first == "--product" {
                let source = next_argument(&mut arguments, "source file")?;
                (true, source)
            } else {
                let mut product = false;
                if let Some(extra) = arguments.next() {
                    if extra == "--product" {
                        product = true;
                    } else {
                        return Err(
                            "structure accepts <source-file> or --product <source-file>".to_owned()
                        );
                    }
                }
                (product, first)
            };
            if arguments.next().is_some() {
                return Err("structure accepts <source-file> or --product <source-file>".to_owned());
            }
            structure(Path::new(&source), product)
        }
        "apply-edit" => {
            let source = next_argument(&mut arguments, "source file")?;
            let edit = next_argument(&mut arguments, "edit file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("apply-edit requires --output <source-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "source output file")?;
            if arguments.next().is_some() {
                return Err(
                    "apply-edit accepts one source file, one edit file, and --output <source-file>"
                        .to_owned(),
                );
            }
            apply_edit(Path::new(&source), Path::new(&edit), Path::new(&output))
        }
        "compile" => {
            let source = next_argument(&mut arguments, "source file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("compile requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            let mut use_bootstrap = false;
            let mut native_c = false;
            for extra in arguments.by_ref() {
                if extra == "--bootstrap" {
                    use_bootstrap = true;
                } else if extra == "--native-c" {
                    native_c = true;
                } else {
                    return Err(
                        "compile accepts --output <file> and optional --bootstrap or --native-c"
                            .to_owned(),
                    );
                }
            }
            compile(
                Path::new(&source),
                Path::new(&output),
                use_bootstrap,
                native_c,
            )
        }
        "registry" => {
            let subcommand = next_argument(&mut arguments, "registry subcommand")?;
            match subcommand.to_string_lossy().as_ref() {
                "verify-cache" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    if arguments.next().is_some() {
                        return Err(
                            "registry verify-cache accepts one cache root directory".to_owned()
                        );
                    }
                    registry_verify_cache(Path::new(&root))
                }
                "pin-local" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut name = None;
                    let mut version = None;
                    let mut artifact = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--name" {
                            name = Some(next_argument(&mut args, "package name")?);
                        } else if flag == "--version" {
                            version = Some(next_argument(&mut args, "package version")?);
                        } else if flag == "--artifact" {
                            artifact = Some(next_argument(&mut args, "artifact path")?);
                        } else {
                            return Err(
                                "registry pin-local accepts --name --version --artifact".to_owned()
                            );
                        }
                    }
                    let name =
                        name.ok_or_else(|| "registry pin-local requires --name".to_owned())?;
                    let version = version
                        .ok_or_else(|| "registry pin-local requires --version".to_owned())?;
                    let artifact = artifact
                        .ok_or_else(|| "registry pin-local requires --artifact".to_owned())?;
                    registry_pin_local(
                        Path::new(&root),
                        &name.to_string_lossy(),
                        &version.to_string_lossy(),
                        Path::new(&artifact),
                    )
                }
                other => Err(format!(
                    "unknown registry subcommand {other} (use verify-cache or pin-local)"
                )),
            }
        }
        "forge" => {
            let compiler = next_argument(&mut arguments, "compiler artifact")?;
            let source = next_argument(&mut arguments, "source file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("forge requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            if arguments.next().is_some() {
                return Err(
                    "forge accepts one compiler artifact, one source file, and one Aether artifact output file"
                        .to_owned(),
                );
            }
            forge(Path::new(&compiler), Path::new(&source), Path::new(&output))
        }
        "run" => {
            let artifact = next_argument(&mut arguments, "artifact file")?;
            let grants = parse_run_grants(&mut arguments)?;
            execute_artifact(Path::new(&artifact), grants)
        }
        "test" => {
            let options = parse_test_args(&mut arguments)?;
            let report = test_runner::run_tests_with_grants(&options.paths, options.grants)?;
            test_runner::print_report(&report);
            test_runner::write_structured_reports(
                &report,
                options.report_json.as_deref(),
                options.report_junit.as_deref(),
            )?;
            if report.all_passed() {
                Ok(())
            } else {
                Err(format!(
                    "test suite failed: {} passed; {} failed",
                    report.passed(),
                    report.failed()
                ))
            }
        }
        "format" => {
            let first = next_argument(&mut arguments, "source file or --product")?;
            let mut product = false;
            let source = if first == "--product" {
                product = true;
                next_argument(&mut arguments, "source file")?
            } else {
                first
            };
            let mut output = None;
            while let Some(flag) = arguments.next() {
                if flag == "--product" {
                    product = true;
                } else if flag == "--output" {
                    let path = next_argument(&mut arguments, "source output file")?;
                    output = Some(path);
                } else {
                    return Err(
                        "format accepts <source-file> [--product] [--output <source-file>] or --product <source-file> [--output <source-file>]"
                            .to_owned(),
                    );
                }
            }
            format_file(Path::new(&source), output.as_ref().map(Path::new), product)
        }
        "project" => {
            let subcommand = next_argument(&mut arguments, "project subcommand")?;
            match subcommand.to_string_lossy().as_ref() {
                "verify" => {
                    let project = next_argument(&mut arguments, "project file")?;
                    let mut output_dir = None;
                    if let Some(flag) = arguments.next() {
                        if flag != "--output-dir" {
                            return Err(
                                "project verify accepts optional --output-dir <dir>".to_owned()
                            );
                        }
                        let dir = next_argument(&mut arguments, "output directory")?;
                        if arguments.next().is_some() {
                            return Err(
                                "project verify accepts one project file and optional --output-dir <dir>"
                                    .to_owned(),
                            );
                        }
                        output_dir = Some(dir);
                    }
                    project_verify(Path::new(&project), output_dir.as_ref().map(Path::new))
                }
                "format" => {
                    let project = next_argument(&mut arguments, "project file")?;
                    let mut write = false;
                    let mut product = false;
                    for flag in arguments.by_ref() {
                        if flag == "--write" {
                            write = true;
                        } else if flag == "--product" {
                            product = true;
                        } else {
                            return Err(
                                "project format accepts optional --write and --product".to_owned()
                            );
                        }
                    }
                    project_format(Path::new(&project), write, product)
                }
                "lock" => {
                    let project = next_argument(&mut arguments, "project file")?;
                    let write = parse_optional_write_flag(&mut arguments, "project lock")?;
                    project_lock(Path::new(&project), write)
                }
                "build" => {
                    let project = next_argument(&mut arguments, "project file")?;
                    let output_flag = next_argument(&mut arguments, "--output flag")?;
                    if output_flag != "--output" {
                        return Err("project build requires --output <artifact-file>".to_owned());
                    }
                    let output = next_argument(&mut arguments, "artifact output file")?;
                    if arguments.next().is_some() {
                        return Err(
                            "project build accepts one project file and --output <artifact-file>"
                                .to_owned(),
                        );
                    }
                    project_build(Path::new(&project), Path::new(&output))
                }
                "test" => {
                    let project = next_argument(&mut arguments, "project file")?;
                    let (grants, report_json, report_junit) =
                        parse_project_test_tail(&mut arguments)?;
                    project_test(
                        Path::new(&project),
                        grants,
                        report_json.as_deref(),
                        report_junit.as_deref(),
                    )
                }
                _ => Err(
                    "project accepts verify, format, lock, build, or test subcommands".to_owned(),
                ),
            }
        }
        "workspace" => {
            let subcommand = next_argument(&mut arguments, "workspace subcommand")?;
            match subcommand.to_string_lossy().as_ref() {
                "verify" => {
                    let workspace = next_argument(&mut arguments, "workspace file")?;
                    if arguments.next().is_some() {
                        return Err(
                            "workspace verify accepts exactly one workspace file".to_owned()
                        );
                    }
                    workspace_verify(Path::new(&workspace))
                }
                "lock" => {
                    let workspace = next_argument(&mut arguments, "workspace file")?;
                    let write = parse_optional_write_flag(&mut arguments, "workspace lock")?;
                    workspace_lock(Path::new(&workspace), write)
                }
                "build" => {
                    let workspace = next_argument(&mut arguments, "workspace file")?;
                    let package_flag = next_argument(&mut arguments, "--package flag")?;
                    if package_flag != "--package" {
                        return Err(
                            "workspace build requires --package <name> --output <artifact-file>"
                                .to_owned(),
                        );
                    }
                    let package = next_argument(&mut arguments, "package name")?;
                    let output_flag = next_argument(&mut arguments, "--output flag")?;
                    if output_flag != "--output" {
                        return Err(
                            "workspace build requires --package <name> --output <artifact-file>"
                                .to_owned(),
                        );
                    }
                    let output = next_argument(&mut arguments, "artifact output file")?;
                    if arguments.next().is_some() {
                        return Err(
                            "workspace build accepts one workspace file, --package, and --output"
                                .to_owned(),
                        );
                    }
                    let package = package
                        .into_string()
                        .map_err(|_| "package name must be valid UTF-8".to_owned())?;
                    workspace_build(Path::new(&workspace), &package, Path::new(&output))
                }
                _ => Err("workspace accepts verify, lock, or build subcommands".to_owned()),
            }
        }
        "version" => {
            if arguments.next().is_some() {
                return Err("version does not accept arguments".to_owned());
            }
            println!("{LANGUAGE_NAME} {LANGUAGE_VERSION}");
            Ok(())
        }
        "lsp" => {
            let mut project = None;
            if let Some(flag) = arguments.next() {
                if flag != "--project" {
                    return Err("lsp accepts optional --project <aether.project.json>".to_owned());
                }
                let path = next_argument(&mut arguments, "project file")?;
                if arguments.next().is_some() {
                    return Err(
                        "lsp accepts optional --project <aether.project.json> only".to_owned()
                    );
                }
                project = Some(PathBuf::from(path));
            }
            if let Some(ref path) = project {
                if !path.is_file() {
                    return Err(format!("project file not found: {}", path.display()));
                }
            }
            lsp::run(project)
        }
        _ => Err(format!("unknown command {command:?}")),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            // Test suite failures already printed a report; avoid dumping full usage.
            if error.starts_with("test suite failed:") || error.starts_with("no test sources found")
            {
                eprintln!("{LANGUAGE_NAME} failed: {error}");
            } else {
                usage();
                eprintln!("{LANGUAGE_NAME} failed: {error}");
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_core::compile_with_seed;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TemporaryDirectory {
        path: PathBuf,
    }

    impl TemporaryDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!(
                "aether-cli-forge-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temporary forge directory should be created");
            Self { path }
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn forge_invokes_compile_and_writes_only_a_verified_artifact() {
        let temporary = TemporaryDirectory::create();
        let target = compile_to_bytecode(
            "world target\n\nweave main [] -> Whole:\n  speak \"built by forge\"\n  yield 0\n",
        )
        .expect("target source should compile")
        .bytecode;
        let compiler_source = format!(
            "world forge\n\nweave compile [borrow source: Text] -> Bytes:\n  bind target <- bytes \"{}\"\n  yield move target\n\nweave main [] -> Whole:\n  yield 0\n",
            hex_encode(&target)
        );
        let compiler = compile_to_bytecode(&compiler_source)
            .expect("compiler fixture should compile")
            .bytecode;
        let compiler_path = temporary.path.join("compiler.aeth");
        let source_path = temporary.path.join("input.ae");
        let output_path = temporary.path.join("output.aeth");
        fs::write(&compiler_path, compiler).expect("compiler artifact should be written");
        fs::write(&source_path, "world supplied\n").expect("source input should be written");

        forge(&compiler_path, &source_path, &output_path)
            .expect("forge should write the compiler result");

        let generated = fs::read(&output_path).expect("forge artifact should be readable");
        assert_eq!(generated, target);
        verify_bytecode(&generated).expect("forge output must verify");
    }

    #[test]
    fn product_check_accepts_seed_valid_source_without_bootstrap_ast() {
        assert!(
            product_cli_check_without_bootstrap(),
            "ADR-051 tracker must be true"
        );
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("ok.ae");
        fs::write(
            &source_path,
            "world cli\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("source should write");
        check(&source_path, true).expect("product check must accept valid seed surface");
    }

    #[test]
    fn product_format_normalizes_crlf_without_bootstrap() {
        assert!(
            product_format_without_bootstrap(),
            "ADR-053 tracker must be true"
        );
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("crlf.ae");
        let output_path = temporary.path.join("out.ae");
        fs::write(
            &source_path,
            "world cli\r\n\r\nweave main [] -> Whole:\r\n  yield 0\r\n",
        )
        .expect("source should write");
        format_file(&source_path, Some(&output_path), true).expect("product format");
        let written = fs::read_to_string(&output_path).expect("read");
        assert_eq!(written, "world cli\n\nweave main [] -> Whole:\n  yield 0\n");
    }

    #[test]
    fn product_structure_emits_product_schema_without_bootstrap_ast() {
        assert!(
            product_structure_without_bootstrap(),
            "ADR-054 tracker must be true"
        );
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("ok.ae");
        fs::write(
            &source_path,
            "world cli\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("source should write");
        // Capture via product_structure_json used by structure()
        let source = fs::read_to_string(&source_path).expect("read");
        let document = product_structure_json(&source).expect("product structure");
        assert!(document.contains("aether.product-structure/v1"));
        assert!(document.contains("\"productAccepted\": true"));
        assert!(!document.contains("aether.ast/v8"));
    }

    #[test]
    fn product_check_rejects_legacy_with_ae_seed_code() {
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("legacy.ae");
        fs::write(&source_path, "world w\n\nfn main() -> Int { return 0; }\n")
            .expect("legacy source should write");
        let error = check(&source_path, true).expect_err("legacy must fail product check");
        assert!(
            error.contains("AE-SEED-007"),
            "expected AE-SEED-007, got {error}"
        );
    }

    #[test]
    fn apply_edit_writes_only_canonical_seed_validated_source() {
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("input.ae");
        let edit_path = temporary.path.join("edit.json");
        let output_path = temporary.path.join("output.ae");
        let source = "world cli\n\nweave main [] -> Whole:\n  yield 0\n";
        let edit = r#"{
  "protocol": "aether.edit/v8",
  "schema": "aether.ast/v8",
  "baseSource": "world cli\n\nweave main [] -> Whole:\n  yield 0\n",
  "operations": [{
    "op": "replace",
    "target": "weave:main",
    "declaration": {
      "kind": "Weave",
      "name": "main",
      "parameters": [],
      "result": "Whole",
      "effect": "Total",
      "task": false,
      "body": [{
        "kind": "Yield",
        "value": {"kind": "Atom", "atom": {"kind": "Whole", "value": 9}}
      }]
    }
  }]
}"#;
        fs::write(&source_path, source).expect("source fixture should write");
        fs::write(&edit_path, edit).expect("edit fixture should write");

        apply_edit(&source_path, &edit_path, &output_path)
            .expect("validated structural edit should write");

        let written = fs::read_to_string(&output_path).expect("edited source should be readable");
        assert_eq!(written, "world cli\n\nweave main [] -> Whole:\n  yield 9\n");
        compile_with_seed(&written).expect("written source must remain seed compilable");
    }

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0F)]));
        }
        output
    }

    #[test]
    fn run_with_grant_read_executes_host_io_example() {
        let temporary = TemporaryDirectory::create();
        let fixture = temporary.path.join("config.txt");
        fs::write(&fixture, "Aether").expect("config fixture");
        let source = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/host-io-read.ae"),
        )
        .expect("host-io-read example");
        let artifact = compile_with_seed(&source)
            .expect("host-io-read should seed-compile")
            .bytecode;
        let artifact_path = temporary.path.join("host-io-read.aeth");
        fs::write(&artifact_path, &artifact).expect("artifact write");

        execute_artifact(
            &artifact_path,
            HostGrantConfig {
                read_roots: vec![temporary.path.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect("granted run should succeed");

        let denied = execute_artifact(&artifact_path, HostGrantConfig::default());
        assert!(denied.is_err(), "run without grant must fail closed");
    }

    #[test]
    fn test_runner_passes_zero_exit_and_fails_nonzero() {
        let temporary = TemporaryDirectory::create();
        let pass_path = temporary.path.join("ok_test.ae");
        let fail_path = temporary.path.join("bad_test.ae");
        fs::write(
            &pass_path,
            "world ok\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("pass fixture");
        fs::write(
            &fail_path,
            "world bad\n\nweave main [] -> Whole:\n  yield 1\n",
        )
        .expect("fail fixture");

        let report = test_runner::run_tests_with_grants(
            std::slice::from_ref(&temporary.path),
            HostGrantConfig::default(),
        )
        .expect("discover tests");
        assert_eq!(report.results.len(), 2);
        assert_eq!(report.passed(), 1);
        assert_eq!(report.failed(), 1);
        assert!(!report.all_passed());

        let single = test_runner::run_one_test_with_grants(&pass_path, HostGrantConfig::default());
        assert!(single.ok, "explicit pass: {}", single.detail);

        let empty = temporary.path.join("empty");
        fs::create_dir_all(&empty).expect("empty dir");
        let err = test_runner::collect_test_sources(&[empty]).expect_err("empty fails closed");
        assert!(err.contains("no test sources"), "{err}");
    }

    #[test]
    fn shipped_examples_tests_directory_passes() {
        let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/tests");
        let report = test_runner::run_tests_with_grants(&[tests_dir], HostGrantConfig::default())
            .expect("examples/tests should run");
        assert!(
            report.all_passed(),
            "shipped example tests must pass: {:?}",
            report.results
        );
        assert!(report.passed() >= 2);
    }

    #[test]
    fn shipped_workspace_example_verifies() {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/workspace/aether.workspace.json");
        workspace_verify(&workspace).expect("examples/workspace should verify");
    }

    #[test]
    fn shipped_workspace_build_app_imports_util() {
        let temporary = TemporaryDirectory::create();
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/workspace/aether.workspace.json");
        let output = temporary.path.join("app.aeth");
        workspace_build(&workspace, "app", &output).expect("workspace build app");
        let artifact = fs::read(&output).expect("artifact");
        verify_bytecode(&artifact).expect("verify");
        let run = run_bytecode(&artifact).expect("run");
        assert_eq!(run.exit_code, 42);
    }

    #[test]
    fn lock_write_flag_is_closed_and_explicit() {
        let mut absent = Vec::<OsString>::new().into_iter();
        assert!(!parse_optional_write_flag(&mut absent, "project lock").expect("no flag"));

        let mut write = vec![OsString::from("--write")].into_iter();
        assert!(parse_optional_write_flag(&mut write, "project lock").expect("write flag"));

        let mut unexpected = vec![OsString::from("--force")].into_iter();
        let error = parse_optional_write_flag(&mut unexpected, "project lock")
            .expect_err("unknown mutation flag is rejected");
        assert!(error.contains("optional --write flag only"), "{error}");

        let mut trailing = vec![OsString::from("--write"), OsString::from("extra")].into_iter();
        let error = parse_optional_write_flag(&mut trailing, "workspace lock")
            .expect_err("trailing argument is rejected");
        assert!(error.contains("optional --write flag only"), "{error}");
    }

    #[test]
    fn lock_commands_only_write_explicitly_and_locked_build_rejects_stale_manifest() {
        let temporary = TemporaryDirectory::create();
        let package_dir = temporary.path.join("app");
        fs::create_dir_all(&package_dir).expect("package dir");
        fs::write(
            package_dir.join("main.ae"),
            "world app_pkg\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("package source");
        let project_path = package_dir.join("aether.project.json");
        fs::write(
            &project_path,
            r#"{
  "schema": "aether.project/v1",
  "name": "app_pkg",
  "version": "0.1.0",
  "units": [{ "path": "main.ae", "role": "main" }]
}"#,
        )
        .expect("project manifest");
        let original_project = fs::read(&project_path).expect("read project before lock");

        project_lock(&project_path, false).expect("project lock preview");
        assert_eq!(
            fs::read(&project_path).expect("project remains untouched"),
            original_project
        );
        project_lock(&project_path, true).expect("project lock write");
        let locked_project = fs::read_to_string(&project_path).expect("locked project");
        assert!(parse_project_document(&locked_project)
            .expect("parse locked project")
            .lock
            .is_some());

        let workspace_path = temporary.path.join("aether.workspace.json");
        fs::write(
            &workspace_path,
            r#"{
  "schema": "aether.workspace/v1",
  "name": "lock_cli_demo",
  "version": "0.1.0",
  "packages": [{ "name": "app", "path": "app" }]
}"#,
        )
        .expect("workspace manifest");
        let original_workspace = fs::read(&workspace_path).expect("read workspace before lock");

        workspace_lock(&workspace_path, false).expect("workspace lock preview");
        assert_eq!(
            fs::read(&workspace_path).expect("workspace remains untouched"),
            original_workspace
        );
        workspace_lock(&workspace_path, true).expect("workspace lock write");
        workspace_verify(&workspace_path).expect("locked workspace verifies");

        let changed_manifest = locked_project.replace('\n', "\r\n");
        fs::write(&project_path, changed_manifest).expect("stale project manifest");
        let output = temporary.path.join("app.aeth");
        let error = workspace_build(&workspace_path, "app", &output)
            .expect_err("locked build rejects stale project manifest");
        assert!(error.contains("AE-WORKSPACE-005"), "{error}");
        assert!(!output.exists(), "locked build must not write an artifact");
    }

    #[test]
    fn shipped_stdlib_layer1_project_builds_and_runs() {
        let temporary = TemporaryDirectory::create();
        let project =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../stdlib/aether.project.json");
        let output = temporary.path.join("stdlib.aeth");
        project_build(&project, &output).expect("stdlib project build");
        let artifact = fs::read(&output).expect("artifact");
        verify_bytecode(&artifact).expect("verify");
        let run = run_bytecode(&artifact).expect("run");
        assert_eq!(run.exit_code, 42, "double 21 via layer1 multi-import graph");

        let test_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../stdlib/whole_test.ae");
        let report = test_runner::run_tests_with_grants(&[test_path], HostGrantConfig::default())
            .expect("stdlib whole_test");
        assert!(report.all_passed(), "{:?}", report.results);
    }

    #[test]
    fn shipped_stdlib_project_test_imports_lib() {
        let project =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../stdlib/aether.project.json");
        project_test(&project, HostGrantConfig::default(), None, None)
            .expect("stdlib project test role units");
    }

    #[test]
    fn test_runner_writes_structured_reports() {
        let temporary = TemporaryDirectory::create();
        let pass_path = temporary.path.join("ok_test.ae");
        fs::write(
            &pass_path,
            "world ok\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("pass");
        let report = test_runner::run_tests_with_grants(
            std::slice::from_ref(&pass_path),
            HostGrantConfig::default(),
        )
        .expect("run");
        let json_path = temporary.path.join("report.json");
        let junit_path = temporary.path.join("report.xml");
        test_runner::write_structured_reports(&report, Some(&json_path), Some(&junit_path))
            .expect("write reports");
        let json = fs::read_to_string(&json_path).expect("read json");
        assert!(json.contains(test_runner::TEST_REPORT_SCHEMA), "{json}");
        assert!(json.contains("\"passed\": 1"), "{json}");
        let junit = fs::read_to_string(&junit_path).expect("read junit");
        assert!(junit.contains("<testsuite"), "{junit}");
        assert!(junit.contains("<testcase"), "{junit}");
    }

    #[test]
    fn test_runner_optional_grants_for_host_io() {
        let temporary = TemporaryDirectory::create();
        let fixture = temporary.path.join("config.txt");
        fs::write(&fixture, "Aether").expect("config");
        // Exit 0 when measure of granted read equals 6 ("Aether").
        let source = r#"world host_io_grant_test

host weave read_text [borrow path: Text] -> Text

host weave text_extent [borrow message: Text] -> Whole

weave main [] -> Whole:
  bind path <- "config.txt"
  bind cfg <- call read_text borrow path
  bind n <- call text_extent borrow cfg
  bind mutable code <- 1
  choose same n 6:
    revise code <- 0
  yield code
"#;
        let test_path = temporary.path.join("host_io_grant_test.ae");
        fs::write(&test_path, source).expect("write test");

        let denied = test_runner::run_one_test_with_grants(&test_path, HostGrantConfig::default());
        assert!(
            !denied.ok,
            "empty grants must fail closed: {}",
            denied.detail
        );

        let granted = test_runner::run_one_test_with_grants(
            &test_path,
            HostGrantConfig {
                read_roots: vec![temporary.path.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        );
        assert!(
            granted.ok,
            "grant-read must allow host-io test: {}",
            granted.detail
        );
    }
}
