use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod bench_runner;
mod lsp;
mod test_runner;

use aether_core::{
    apply_edit_cli_trusts_product_accept, apply_structural_edit, canonical_ast,
    compile_product_bytecode, compile_project_modules, compile_source, compile_to_bytecode,
    compile_workspace_package, encode_x509_lite_pem, fetch_signed_package, forge_bundle_bytecode,
    forge_bytecode, forge_modules_bytecode, format_project, format_source, format_source_product,
    install_certified_signing_key, install_package_bundle, install_package_from_cache,
    install_trust_key, install_trust_root, issue_x509_lite_certificate, lower_verified_aeth_to_c,
    lower_verified_aeth_to_llvm_ir, lower_verified_aeth_to_llvm_object,
    lower_verified_aeth_to_native_exe, lower_verified_aeth_to_native_exe_for_target,
    lower_verified_aeth_to_native_object, multi_module_authority_note, native_target_is_host,
    pack_project, parse_project_document, parse_workspace_document, pin_local_package,
    pin_local_package_signed, probe_native_toolchain, product_cli_check_without_bootstrap,
    product_default_cli_toolchain, product_format_without_bootstrap,
    product_project_format_without_bootstrap, product_seed_rebuild_without_bootstrap,
    product_structure_json, product_structure_without_bootstrap, publish_package_bundle,
    refresh_project_lock, refresh_workspace_lock, require_hermetic_native_toolchain,
    revoke_trust_key, rotate_trust_key, run_bytecode, run_bytecode_with_grants,
    run_project_tests_with_grants, serialize_project_document, serialize_workspace_document,
    set_trust_key_validity, store_x509_lite_certificate, structural_document_json,
    unit_artifact_file_name, verify_bytecode, verify_package_bundle, verify_package_cache,
    verify_project, verify_registry_cache, verify_workspace, verify_x509_lite_store,
    HostGrantConfig, InvocationValue, LANGUAGE_NAME, LANGUAGE_VERSION,
};

fn usage() {
    eprintln!(
        "M32a/M32b verified-execution benchmarks:\n  aether bench <all|welcome|arena-buffer|task-loop> [--warmup <0..=100>] [--iterations <1..=1000>] [--profile <lowercase-ascii-id>] [--report <file.json>]\n  aether bench compare <baseline-report.json> <candidate-report.json> [--report <file.json>]\n  aether bench --list\nBenchmark workloads are embedded, pure, and bounded; no caller source/artifact input or grants are accepted. --profile emits a comparison-ready v2 report; use a non-secret label describing the pinned local context. compare reads bounded v2 data only and never executes it."
    );
    eprintln!(
        "M24f/g key setup:\n  aether registry trust-root <cache-root> --key-id <id> --seed-file <32-byte-path>\n  aether registry certify-ed25519-key <cache-root> --key-id <id> --seed-file <32-byte-path> --parent-key-id <id>\nThe supplied Ed25519 seed files remain local and must be exactly 32 bytes."
    );
    eprintln!(
        "M25 offline local packages:\n  aether pkg pack <aether.project.json> --output <bundle-dir>\n  aether pkg verify <bundle-dir>\n  aether pkg publish <bundle-dir> --cache <cache-dir>\n  aether pkg install <bundle-dir> --output <package-dir>\n  aether pkg install --cache <cache-dir> --name <name> --version <version> --output <package-dir>\n  aether pkg verify-cache <cache-dir>\nBundles contain verified Aether source only; cache and install paths stay local and are never network-resolved."
    );
    eprintln!(
        "Usage:\n  aether check <source-file>\n  aether check --bootstrap <source-file>\n  aether structure <source-file>\n  aether structure --bootstrap <source-file>\n  aether apply-edit <source-file> <edit-file> --output <source-file>\n  aether format <source-file> [--output <source-file>]\n  aether format --bootstrap <source-file> [--output <source-file>]\n  aether project verify <project-file> [--output-dir <dir>]\n  aether project format <project-file> [--write] [--bootstrap]\n  aether project lock <project-file> [--write]\n  aether project build <project-file> --output <artifact-file>\n  aether project test <project-file>\n  aether workspace verify <workspace-file>\n  aether workspace lock <workspace-file> [--write]\n  aether workspace build <workspace-file> --package <name> --output <artifact-file>\n  aether compile <source-file> --output <artifact-file> [--bootstrap|--native-c|--native-exe [--target <triple>]]\n  aether native probe\n  aether registry verify-cache <cache-root>\n  aether registry pin-local <cache-root> --name <n> --version <v> --artifact <path>\n  aether registry trust-key <cache-root> --key-id <id> --key-file <path>\n  aether registry pin-local-signed <cache-root> --name <n> --version <v> --artifact <path> --key-id <id>\n  aether registry fetch-signed <cache-root> --name <n> --version <v> --url <url> --signature <hex> --key-id <id>\n  aether registry issue-x509-lite <cache-root> --issuer <id> --subject <id> --serial <s> --not-before <YYYY-MM-DD> --not-after <YYYY-MM-DD> [--output <pem>]\n  aether registry store-x509-lite <cache-root> --issuer <id> --subject <id> --serial <s> --not-before <YYYY-MM-DD> --not-after <YYYY-MM-DD> [--output <pem>]\n  aether registry verify-x509-lite-store <cache-root>\n  aether forge <compiler-artifact> <source-file> --output <artifact-file>\n  aether forge-bundle <compiler-artifact> <bundle-file> --output <artifact-file>\n  aether forge-modules <compiler-artifact> <catalog-file> --output <artifact-file>\n  aether run <artifact-file> [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]... [--grant-lib KEY=PATH]...\n  aether test [path...] [--grant-read <dir>]... [--grant-write <dir>]... [--grant-env <NAME>]... [--grant-lib KEY=PATH]... [--report <file.json>] [--report-junit <file.xml>]\n  aether lsp\n  aether version\n\nADR-064: product seed path is default for check/format/structure/project format.\ncheck --bootstrap: full bootstrap AST diagnostics (recovery).\nformat --bootstrap: AST-canonical rewrite (recovery).\nstructure --bootstrap: aether.ast/v8 (recovery).\nDefault check/format/structure use seed product path only.\ncompile uses the Aether-written seed compiler by default for single-file sources and recognizes the bounded ADR-128/129/130 seed-bundle profiles plus GSM-001 aether.seed-modules/v1 catalogs.\nforge-bundle invokes only compile_bundle [borrow bundle: Text] -> Bytes; forge-modules invokes only compile_modules [borrow catalog: Text] -> Bytes, each on a caller-selected verified compiler artifact.\nM19e task source emits AETH v12; source without task frames retains AETH v11.\nDefault structure emits aether.product-structure/v1; --bootstrap emits aether.ast/v8.\napply-edit accepts aether.edit/v8 (including statement-level ops), bootstrap-canonical base parse, product seed accept in core before write (CLI does not re-forge).\nproject verify is offline: schema, nested path confinement, optional SHA-256 lock; module units validated for M11.\nproject lock derives a complete local unit lock after verification; --write is required to replace the project manifest.\nproject build frames its manifest-selected source catalog; the seed resolves import unit / export weave graphs and emits AETH (GSM-001; dual-compare is test/oracle only).\nproject test frames each role:test entry catalog, pure-runs; pass requires exit 0 (M17b); optional --grant-* (M17c); optional --report / --report-junit (M17d).\nproject format defaults to product unit format; --bootstrap uses AST-canonical format; --write overwrites unit paths.\nworkspace verify is offline multi-package integrity (aether.workspace/v1): path-jail package roots, acyclic depends_on, nested project verify (M18).\nworkspace lock pins every package's project identity and requires nested project locks; --write is required to replace the workspace manifest.\nworkspace build frames one package main catalog with M22 import unit from package (depends_on only); the seed resolves the graph and locked workspaces verify before artifact output.\naether test discovers *_test.ae under directories (or runs explicit .ae files), seed-compiles, pure-runs; pass requires exit 0 (M17); optional --grant-* (M17c); optional --report / --report-junit (M17d).\naether lsp [--project <aether.project.json>] is an offline stdio Language Server (product-primary diagnostics ADR-058; product-surface symbols/hover/definition ADR-063/066; product format ADR-064; project-aware import definition/hover; no product AETH emit; no silent disk writes).\naether run grants: M14 I/O roots/names and M21 --grant-lib KEY=PATH (explicit library file; no PATH search). Empty grants keep pure fixtures only.\nPass --bootstrap for recovery AST diagnostics / dual-compare oracle emit (product seed rebuild needs no --bootstrap; ADR-067).\nPass --native-exe --target <triple> for the closed F-NATIVE M35j matrix; host targets retain dual-run and cross targets are link-only.\nregistry pin-local/verify-cache are offline F-REGISTRY M24a; trust-key/pin-local-signed/fetch-signed are M24b+; issue-x509-lite is M24h; store-x509-lite / verify-x509-lite-store are M24i (not full RFC 5280)."
    );
    eprintln!(
        "Bounded seed-bundle profiles: ADR-128 v1 accepts one pure Whole library -> entry; ADR-129 v2 accepts foundation -> bridge -> entry; ADR-130 v3 accepts left leaf + right leaf -> two-import merge -> entry. GSM-001 is the separate general bounded seed-native M11/M22 catalog path."
    );
}

fn native_probe() -> Result<(), String> {
    let probe = require_hermetic_native_toolchain().map_err(|error| error.to_string())?;
    let _ = probe_native_toolchain();
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} native probe: os={} arch={} target={} hermetic={} cc={} clang={} llc={}",
        probe.host_os,
        probe.host_arch,
        probe.target_triple.as_deref().unwrap_or("-"),
        probe.hermetic,
        probe.cc.as_deref().unwrap_or("-"),
        probe.clang.as_deref().unwrap_or("-"),
        probe.llc.as_deref().unwrap_or("-"),
    );
    Ok(())
}

fn read_source(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn read_artifact(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

/// ADR-064: default is product seed path. Pass `bootstrap = true` for recovery
/// AST diagnostics (`aether check --bootstrap`).
fn check(source_path: &Path, bootstrap: bool) -> Result<(), String> {
    let source = read_source(source_path)?;
    if !bootstrap {
        debug_assert!(
            product_cli_check_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: default check is product seed path"
        );
        let bytecode = compile_product_bytecode(&source).map_err(|error| error.to_string())?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} check passed: {} byte(s) via seed path in {}",
            bytecode.len(),
            source_path.display()
        );
        return Ok(());
    }
    // Recovery: full bootstrap diagnostics + canonical AST dump.
    let program = compile_source(&source).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} check passed: {} significant token(s) in {} (bootstrap recovery diagnostics)",
        program.significant_token_count(),
        source_path.display()
    );
    println!("{}", canonical_ast(&program));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn compile(
    source_path: &Path,
    output_path: &Path,
    use_bootstrap: bool,
    native_c: bool,
    native_object: bool,
    native_llvm_ir: bool,
    native_llvm_object: bool,
    native_exe: bool,
    native_target: Option<&str>,
) -> Result<(), String> {
    let source = read_source(source_path)?;
    // BARP Phase 2 (ADR-044): default product compile forges seed bytecode without
    // a bootstrap validate precondition. --bootstrap remains rebuild/oracle path.
    // ADR-059/079/083/087/091: native lower flags for verified AETH.
    let native_modes = [
        native_c,
        native_object,
        native_llvm_ir,
        native_llvm_object,
        native_exe,
    ]
    .into_iter()
    .filter(|v| *v)
    .count();
    if native_modes > 0 && use_bootstrap {
        return Err(
            "compile accepts either --bootstrap or a native lower flag, not both".to_owned(),
        );
    }
    if native_modes > 1 {
        return Err(
            "compile accepts only one native lower flag (--native-c, --native-object, --native-llvm-ir, --native-llvm-object, --native-exe)"
                .to_owned(),
        );
    }
    if native_target.is_some() && !native_exe {
        return Err("compile --target requires --native-exe".to_owned());
    }
    // ADR-067: product path (no --bootstrap) is seed rebuild + product compile.
    // --bootstrap remains dual-compare / recovery oracle emit only.
    // ADR-078: multi-source envelopes are accepted on the product path.
    let bytecode = if use_bootstrap {
        compile_to_bytecode(&source)
            .map_err(|error| error.to_string())?
            .bytecode
    } else {
        debug_assert!(
            product_seed_rebuild_without_bootstrap(),
            "ADR-067: product seed rebuild without --bootstrap"
        );
        compile_product_bytecode(&source).map_err(|error| error.to_string())?
    };
    if native_c {
        let c_source = lower_verified_aeth_to_c(&bytecode).map_err(|error| error.to_string())?;
        write_source(output_path, &c_source)?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} lowered verified AETH to C {} (F-NATIVE M35c)",
            output_path.display()
        );
        return Ok(());
    }
    if native_object {
        let cc = lower_verified_aeth_to_native_object(&bytecode, output_path)
            .map_err(|error| error.to_string())?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} lowered verified AETH to native object {} via {cc} (F-NATIVE M35e)",
            output_path.display()
        );
        return Ok(());
    }
    if native_llvm_ir {
        let ir = lower_verified_aeth_to_llvm_ir(&bytecode).map_err(|error| error.to_string())?;
        write_source(output_path, &ir)?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} lowered verified AETH to LLVM IR {} (F-NATIVE M35f)",
            output_path.display()
        );
        return Ok(());
    }
    if native_llvm_object {
        let tool = lower_verified_aeth_to_llvm_object(&bytecode, output_path)
            .map_err(|error| error.to_string())?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} lowered verified AETH to LLVM object {} via {tool} (F-NATIVE M35g)",
            output_path.display()
        );
        return Ok(());
    }
    if native_exe {
        if let Some(target) = native_target {
            let report = lower_verified_aeth_to_native_exe_for_target(
                &bytecode,
                output_path,
                target,
                native_target_is_host(target),
            )
            .map_err(|error| error.to_string())?;
            let mode = if native_target_is_host(target) {
                "host dual-run"
            } else {
                "cross link-only"
            };
            println!(
                "{LANGUAGE_NAME} {LANGUAGE_VERSION} linked verified AETH to native exe {} for {target} via {} ({mode}; F-NATIVE M35j)",
                output_path.display(),
                report.cc_command
            );
        } else {
            let report = lower_verified_aeth_to_native_exe(&bytecode, output_path, true)
                .map_err(|error| error.to_string())?;
            println!(
                "{LANGUAGE_NAME} {LANGUAGE_VERSION} linked verified AETH to native exe {} via {} (F-NATIVE M35h)",
                output_path.display(),
                report.cc_command
            );
        }
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

fn registry_trust_key(cache_root: &Path, key_id: &str, key_file: &Path) -> Result<(), String> {
    let key_bytes = fs::read(key_file)
        .map_err(|error| format!("could not read trust key {}: {error}", key_file.display()))?;
    let key =
        install_trust_key(cache_root, key_id, &key_bytes).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry trust key {} installed at {}",
        key.key_id, key.key_path
    );
    Ok(())
}

fn registry_trust_root(cache_root: &Path, key_id: &str, seed_file: &Path) -> Result<(), String> {
    let seed = fs::read(seed_file).map_err(|error| {
        format!(
            "could not read Ed25519 seed {}: {error}",
            seed_file.display()
        )
    })?;
    let key = install_trust_root(cache_root, key_id, &seed).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry Ed25519 trust root {} installed at {}",
        key.key_id, key.key_path
    );
    Ok(())
}

fn registry_certify_ed25519_key(
    cache_root: &Path,
    key_id: &str,
    seed_file: &Path,
    parent_key_id: &str,
) -> Result<(), String> {
    let seed = fs::read(seed_file).map_err(|error| {
        format!(
            "could not read Ed25519 seed {}: {error}",
            seed_file.display()
        )
    })?;
    let key = install_certified_signing_key(cache_root, key_id, &seed, parent_key_id)
        .map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry certified Ed25519 key {} under {} at {}",
        key.key_id, parent_key_id, key.key_path
    );
    Ok(())
}

fn registry_pin_local_signed(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact: &Path,
    key_id: &str,
) -> Result<(), String> {
    let pin = pin_local_package_signed(cache_root, name, version, artifact, key_id)
        .map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry signed-pin {}@{} -> {} (sha256={} sig={} key={})",
        pin.name,
        pin.version,
        pin.artifact,
        pin.sha256,
        pin.signature.as_deref().unwrap_or(""),
        pin.key_id.as_deref().unwrap_or("")
    );
    Ok(())
}

fn registry_fetch_signed(
    cache_root: &Path,
    name: &str,
    version: &str,
    source_url: &str,
    signature: &str,
    key_id: &str,
) -> Result<(), String> {
    let pin = fetch_signed_package(cache_root, name, version, source_url, signature, key_id)
        .map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry fetch-signed {}@{} -> {} (sha256={} key={})",
        pin.name,
        pin.version,
        pin.artifact,
        pin.sha256,
        pin.key_id.as_deref().unwrap_or("")
    );
    Ok(())
}

fn package_pack(project_manifest: &Path, output_bundle: &Path) -> Result<(), String> {
    let report =
        pack_project(project_manifest, output_bundle).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package packed {}@{} -> {} (files={} bytes={} sha256={})",
        report.name,
        report.version,
        output_bundle.display(),
        report.file_count,
        report.total_bytes,
        report.content_sha256
    );
    Ok(())
}

fn package_verify(bundle_root: &Path) -> Result<(), String> {
    let report = verify_package_bundle(bundle_root).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package verified {}@{} at {} (files={} bytes={} sha256={})",
        report.name,
        report.version,
        bundle_root.display(),
        report.file_count,
        report.total_bytes,
        report.content_sha256
    );
    Ok(())
}

fn package_publish(bundle_root: &Path, cache_root: &Path) -> Result<(), String> {
    let report =
        publish_package_bundle(bundle_root, cache_root).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package published {}@{} from {} into {} (files={} bytes={} sha256={})",
        report.name,
        report.version,
        bundle_root.display(),
        cache_root.display(),
        report.file_count,
        report.total_bytes,
        report.content_sha256
    );
    Ok(())
}

fn package_install_bundle(bundle_root: &Path, output_directory: &Path) -> Result<(), String> {
    let report =
        install_package_bundle(bundle_root, output_directory).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package installed {}@{} from {} to {} (files={} bytes={} sha256={})",
        report.name,
        report.version,
        bundle_root.display(),
        output_directory.display(),
        report.file_count,
        report.total_bytes,
        report.content_sha256
    );
    Ok(())
}

fn package_install_cache(
    cache_root: &Path,
    name: &str,
    version: &str,
    output_directory: &Path,
) -> Result<(), String> {
    let report = install_package_from_cache(cache_root, name, version, output_directory)
        .map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package installed {}@{} from cache {} to {} (files={} bytes={} sha256={})",
        report.name,
        report.version,
        cache_root.display(),
        output_directory.display(),
        report.file_count,
        report.total_bytes,
        report.content_sha256
    );
    Ok(())
}

fn package_verify_cache(cache_root: &Path) -> Result<(), String> {
    let reports = verify_package_cache(cache_root).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} package cache verified {} package(s) at {}",
        reports.len(),
        cache_root.display()
    );
    for report in reports {
        println!(
            "  {}@{} files={} bytes={} sha256={}",
            report.name,
            report.version,
            report.file_count,
            report.total_bytes,
            report.content_sha256
        );
    }
    Ok(())
}

/// ADR-064: default product envelope; `bootstrap = true` for aether.ast/v8 recovery.
fn structure(source_path: &Path, bootstrap: bool) -> Result<(), String> {
    let source = read_source(source_path)?;
    let document = if !bootstrap {
        debug_assert!(
            product_structure_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: default structure is product envelope"
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

/// Invoke the closed ADR-128/129/130 bundle forge ABI on one caller-selected bundle file.
///
/// The core verifies both the supplied compiler and returned artifact. This CLI
/// layer reads only the explicit paths and never decodes, elaborates, or rewrites
/// the bundle's Aether source.
fn forge_bundle(
    compiler_path: &Path,
    bundle_path: &Path,
    output_path: &Path,
) -> Result<(), String> {
    let compiler = read_artifact(compiler_path)?;
    let bundle = read_source(bundle_path)?;
    let output = forge_bundle_bytecode(&compiler, &bundle).map_err(|error| error.to_string())?;
    if !output.stdout.is_empty() {
        eprint!("{}", output.stdout);
    }
    let InvocationValue::Bytes(artifact) = output.value else {
        return Err("the compile_bundle weave must yield Bytes".to_owned());
    };
    write_artifact(output_path, artifact)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} forged bounded bundle {} with {} to {}",
        bundle_path.display(),
        compiler_path.display(),
        output_path.display()
    );
    Ok(())
}

/// Invoke the GSM-001 general seed-module forge ABI on one caller-selected catalog file.
///
/// The core verifies both the supplied compiler and returned artifact. This CLI
/// layer reads only explicit paths and forwards the catalog as opaque Text; it
/// does not parse Aether source, resolve imports, or rewrite unit contents.
fn forge_modules(
    compiler_path: &Path,
    catalog_path: &Path,
    output_path: &Path,
) -> Result<(), String> {
    let compiler = read_artifact(compiler_path)?;
    let catalog = read_source(catalog_path)?;
    let output = forge_modules_bytecode(&compiler, &catalog).map_err(|error| error.to_string())?;
    if !output.stdout.is_empty() {
        eprint!("{}", output.stdout);
    }
    let InvocationValue::Bytes(artifact) = output.value else {
        return Err("the compile_modules weave must yield Bytes".to_owned());
    };
    write_artifact(output_path, artifact)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} forged seed module catalog {} with {} to {}",
        catalog_path.display(),
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

/// ADR-064: default product LF+accept; `bootstrap = true` for AST-canonical recovery format.
fn format_file(
    source_path: &Path,
    output_path: Option<&Path>,
    bootstrap: bool,
) -> Result<(), String> {
    let source = read_source(source_path)?;
    let formatted = if !bootstrap {
        debug_assert!(
            product_format_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: default format is product seed path"
        );
        format_source_product(&source).map_err(|error| error.to_string())?
    } else {
        format_source(&source).map_err(|error| error.to_string())?
    };
    if let Some(output) = output_path {
        write_source(output, &formatted)?;
        let mode = if bootstrap { "bootstrap " } else { "" };
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
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} project {}@{} built {} (GSM-001 seed-native multi-module)",
        document.name,
        document.version,
        output_path.display()
    );
    Ok(())
}

/// ADR-064: default product project format; `bootstrap = true` for AST-canonical recovery.
fn project_format(project_path: &Path, write: bool, bootstrap: bool) -> Result<(), String> {
    let json = read_source(project_path)?;
    let document = parse_project_document(&json).map_err(|error| error.to_string())?;
    let root = project_root_for(project_path);
    let product = !bootstrap;
    if product {
        debug_assert!(
            product_project_format_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: default project format is product seed path"
        );
    }
    let report = format_project(root, &document, product).map_err(|error| error.to_string())?;
    let mode = if bootstrap { "bootstrap " } else { "" };
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

fn next_string_argument(
    arguments: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<String, String> {
    Ok(next_argument(arguments, name)?
        .to_string_lossy()
        .into_owned())
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

/// ADR-064: product is default. Optional `--bootstrap` enables recovery AST path.
/// Legacy `--product` is accepted as an explicit no-op synonym for the default.
fn parse_product_default_source_flag(
    arguments: &mut impl Iterator<Item = OsString>,
    command: &str,
) -> Result<(bool, OsString), String> {
    let first = next_argument(arguments, "source file or flag")?;
    let (mut bootstrap, source) = if first == "--bootstrap" {
        let source = next_argument(arguments, "source file")?;
        (true, source)
    } else if first == "--product" {
        // Legacy flag: product is already default.
        let source = next_argument(arguments, "source file")?;
        (false, source)
    } else {
        (false, first)
    };
    for extra in arguments.by_ref() {
        if extra == "--bootstrap" {
            bootstrap = true;
        } else if extra == "--product" {
            // ignore; product default
        } else {
            return Err(format!(
                "{command} accepts <source-file> and optional --bootstrap (or legacy --product)"
            ));
        }
    }
    Ok((bootstrap, source))
}

#[derive(Debug)]
struct CompileArguments {
    source: OsString,
    output: OsString,
    use_bootstrap: bool,
    native_c: bool,
    native_object: bool,
    native_llvm_ir: bool,
    native_llvm_object: bool,
    native_exe: bool,
    native_target: Option<String>,
}

#[derive(Debug)]
struct X509LiteCertificateRequest {
    issuer: String,
    subject: String,
    serial: String,
    not_before: String,
    not_after: String,
    output: Option<PathBuf>,
}

#[derive(Debug)]
struct RegistryEd25519KeyRequest {
    key_id: String,
    seed_file: PathBuf,
    parent_key_id: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum PackageInstallSource {
    Bundle(PathBuf),
    Cache {
        cache_root: PathBuf,
        name: String,
        version: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct PackageInstallArguments {
    source: PackageInstallSource,
    output_directory: PathBuf,
}

fn parse_package_install_arguments(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<PackageInstallArguments, String> {
    let source = next_argument(arguments, "bundle directory or --cache")?;
    if source != "--cache" {
        let output_flag = next_argument(arguments, "--output flag")?;
        if output_flag != "--output" {
            return Err("pkg install <bundle-dir> requires --output <package-dir>".to_owned());
        }
        let output_directory = PathBuf::from(next_argument(arguments, "package output directory")?);
        if arguments.next().is_some() {
            return Err("pkg install <bundle-dir> accepts only --output <package-dir>".to_owned());
        }
        return Ok(PackageInstallArguments {
            source: PackageInstallSource::Bundle(PathBuf::from(source)),
            output_directory,
        });
    }

    let cache_root = PathBuf::from(next_argument(arguments, "package cache directory")?);
    let mut name = None;
    let mut version = None;
    let mut output_directory = None;
    while let Some(flag) = arguments.next() {
        if flag == "--name" {
            if name
                .replace(next_string_argument(arguments, "package name")?)
                .is_some()
            {
                return Err("pkg install --cache accepts --name once".to_owned());
            }
        } else if flag == "--version" {
            if version
                .replace(next_string_argument(arguments, "package version")?)
                .is_some()
            {
                return Err("pkg install --cache accepts --version once".to_owned());
            }
        } else if flag == "--output" {
            if output_directory
                .replace(PathBuf::from(next_argument(
                    arguments,
                    "package output directory",
                )?))
                .is_some()
            {
                return Err("pkg install --cache accepts --output once".to_owned());
            }
        } else {
            return Err(
                "pkg install --cache accepts --name <name> --version <version> --output <package-dir>"
                    .to_owned(),
            );
        }
    }
    Ok(PackageInstallArguments {
        source: PackageInstallSource::Cache {
            cache_root,
            name: name.ok_or_else(|| "pkg install --cache requires --name".to_owned())?,
            version: version.ok_or_else(|| "pkg install --cache requires --version".to_owned())?,
        },
        output_directory: output_directory
            .ok_or_else(|| "pkg install --cache requires --output".to_owned())?,
    })
}

fn parse_registry_ed25519_key_request(
    arguments: &mut impl Iterator<Item = OsString>,
    command: &str,
    requires_parent: bool,
) -> Result<RegistryEd25519KeyRequest, String> {
    let mut key_id = None;
    let mut seed_file = None;
    let mut parent_key_id = None;
    while let Some(flag) = arguments.next() {
        if flag == "--key-id" {
            if key_id
                .replace(next_string_argument(arguments, "key id")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --key-id once"));
            }
        } else if flag == "--seed-file" {
            if seed_file
                .replace(PathBuf::from(next_argument(
                    arguments,
                    "Ed25519 seed file",
                )?))
                .is_some()
            {
                return Err(format!("registry {command} accepts --seed-file once"));
            }
        } else if flag == "--parent-key-id" && requires_parent {
            if parent_key_id
                .replace(next_string_argument(arguments, "parent key id")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --parent-key-id once"));
            }
        } else if requires_parent {
            return Err(format!(
                "registry {command} accepts --key-id --seed-file --parent-key-id"
            ));
        } else {
            return Err(format!("registry {command} accepts --key-id --seed-file"));
        }
    }
    Ok(RegistryEd25519KeyRequest {
        key_id: key_id.ok_or_else(|| format!("registry {command} requires --key-id"))?,
        seed_file: seed_file.ok_or_else(|| format!("registry {command} requires --seed-file"))?,
        parent_key_id: if requires_parent {
            Some(
                parent_key_id
                    .ok_or_else(|| format!("registry {command} requires --parent-key-id"))?,
            )
        } else {
            None
        },
    })
}

fn parse_x509_lite_certificate_request(
    arguments: &mut impl Iterator<Item = OsString>,
    command: &str,
) -> Result<X509LiteCertificateRequest, String> {
    let mut issuer = None;
    let mut subject = None;
    let mut serial = None;
    let mut not_before = None;
    let mut not_after = None;
    let mut output = None;
    while let Some(flag) = arguments.next() {
        if flag == "--issuer" {
            if issuer
                .replace(next_string_argument(arguments, "issuer key id")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --issuer once"));
            }
        } else if flag == "--subject" {
            if subject
                .replace(next_string_argument(arguments, "subject key id")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --subject once"));
            }
        } else if flag == "--serial" {
            if serial
                .replace(next_string_argument(arguments, "serial")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --serial once"));
            }
        } else if flag == "--not-before" {
            if not_before
                .replace(next_string_argument(arguments, "not-before date")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --not-before once"));
            }
        } else if flag == "--not-after" {
            if not_after
                .replace(next_string_argument(arguments, "not-after date")?)
                .is_some()
            {
                return Err(format!("registry {command} accepts --not-after once"));
            }
        } else if flag == "--output" {
            if output
                .replace(PathBuf::from(next_argument(arguments, "PEM output")?))
                .is_some()
            {
                return Err(format!("registry {command} accepts --output once"));
            }
        } else {
            return Err(format!(
                "registry {command} accepts --issuer --subject --serial --not-before --not-after [--output]"
            ));
        }
    }
    Ok(X509LiteCertificateRequest {
        issuer: issuer.ok_or_else(|| format!("registry {command} requires --issuer"))?,
        subject: subject.ok_or_else(|| format!("registry {command} requires --subject"))?,
        serial: serial.ok_or_else(|| format!("registry {command} requires --serial"))?,
        not_before: not_before
            .ok_or_else(|| format!("registry {command} requires --not-before"))?,
        not_after: not_after.ok_or_else(|| format!("registry {command} requires --not-after"))?,
        output,
    })
}

fn write_or_print_x509_lite_pem(
    cert: &aether_core::RegistryX509LiteCert,
    output: Option<PathBuf>,
    action: &str,
) -> Result<(), String> {
    let pem = encode_x509_lite_pem(cert);
    if let Some(path) = output {
        fs::write(&path, &pem)
            .map_err(|error| format!("could not write {}: {error}", path.display()))?;
        println!(
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry {action} X.509-lite cert {} -> {} to {}",
            cert.tbs.issuer,
            cert.tbs.subject,
            path.display()
        );
    } else {
        print!("{pem}");
    }
    Ok(())
}

fn parse_compile_arguments(
    arguments: &mut impl Iterator<Item = OsString>,
) -> Result<CompileArguments, String> {
    let source = next_argument(arguments, "source file")?;
    let output_flag = next_argument(arguments, "--output flag")?;
    if output_flag != "--output" {
        return Err("compile requires --output <artifact-file>".to_owned());
    }
    let output = next_argument(arguments, "artifact output file")?;
    let mut use_bootstrap = false;
    let mut native_c = false;
    let mut native_object = false;
    let mut native_llvm_ir = false;
    let mut native_llvm_object = false;
    let mut native_exe = false;
    let mut native_target = None;
    while let Some(extra) = arguments.next() {
        if extra == "--bootstrap" {
            use_bootstrap = true;
        } else if extra == "--native-c" {
            native_c = true;
        } else if extra == "--native-object" {
            native_object = true;
        } else if extra == "--native-llvm-ir" {
            native_llvm_ir = true;
        } else if extra == "--native-llvm-object" {
            native_llvm_object = true;
        } else if extra == "--native-exe" {
            native_exe = true;
        } else if extra == "--target" {
            let target = next_argument(arguments, "target triple")?
                .to_string_lossy()
                .into_owned();
            if native_target.replace(target).is_some() {
                return Err("compile accepts --target at most once".to_owned());
            }
        } else {
            return Err(
                "compile accepts --output <file>, optional --bootstrap or one native lower flag, and --target only with --native-exe"
                    .to_owned(),
            );
        }
    }
    let native_modes = [
        native_c,
        native_object,
        native_llvm_ir,
        native_llvm_object,
        native_exe,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count();
    if native_modes > 0 && use_bootstrap {
        return Err(
            "compile accepts either --bootstrap or a native lower flag, not both".to_owned(),
        );
    }
    if native_modes > 1 {
        return Err(
            "compile accepts only one native lower flag (--native-c, --native-object, --native-llvm-ir, --native-llvm-object, --native-exe)"
                .to_owned(),
        );
    }
    if native_target.is_some() && !native_exe {
        return Err("compile --target requires --native-exe".to_owned());
    }
    Ok(CompileArguments {
        source,
        output,
        use_bootstrap,
        native_c,
        native_object,
        native_llvm_ir,
        native_llvm_object,
        native_exe,
        native_target,
    })
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let command = next_argument(&mut arguments, "command")?;
    match command.to_string_lossy().as_ref() {
        "check" => {
            let (bootstrap, source) = parse_product_default_source_flag(&mut arguments, "check")?;
            check(Path::new(&source), bootstrap)
        }
        "structure" => {
            let (bootstrap, source) =
                parse_product_default_source_flag(&mut arguments, "structure")?;
            structure(Path::new(&source), bootstrap)
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
            let options = parse_compile_arguments(&mut arguments)?;
            compile(
                Path::new(&options.source),
                Path::new(&options.output),
                options.use_bootstrap,
                options.native_c,
                options.native_object,
                options.native_llvm_ir,
                options.native_llvm_object,
                options.native_exe,
                options.native_target.as_deref(),
            )
        }
        "native" => {
            let subcommand = next_argument(&mut arguments, "native subcommand")?;
            match subcommand.to_string_lossy().as_ref() {
                "probe" => {
                    if arguments.next().is_some() {
                        return Err("native probe accepts no further arguments".to_owned());
                    }
                    native_probe()
                }
                other => Err(format!("unknown native subcommand {other}")),
            }
        }
        "pkg" => {
            let subcommand = next_argument(&mut arguments, "package subcommand")?;
            match subcommand.to_string_lossy().as_ref() {
                "pack" => {
                    let project_manifest = next_argument(&mut arguments, "project manifest")?;
                    let output_flag = next_argument(&mut arguments, "--output flag")?;
                    if output_flag != "--output" {
                        return Err("pkg pack requires --output <bundle-dir>".to_owned());
                    }
                    let output_bundle = next_argument(&mut arguments, "bundle output directory")?;
                    if arguments.next().is_some() {
                        return Err(
                            "pkg pack accepts one project manifest and --output <bundle-dir>"
                                .to_owned(),
                        );
                    }
                    package_pack(Path::new(&project_manifest), Path::new(&output_bundle))
                }
                "verify" => {
                    let bundle_root = next_argument(&mut arguments, "bundle directory")?;
                    if arguments.next().is_some() {
                        return Err("pkg verify accepts one bundle directory".to_owned());
                    }
                    package_verify(Path::new(&bundle_root))
                }
                "publish" => {
                    let bundle_root = next_argument(&mut arguments, "bundle directory")?;
                    let cache_flag = next_argument(&mut arguments, "--cache flag")?;
                    if cache_flag != "--cache" {
                        return Err("pkg publish requires --cache <cache-dir>".to_owned());
                    }
                    let cache_root = next_argument(&mut arguments, "package cache directory")?;
                    if arguments.next().is_some() {
                        return Err(
                            "pkg publish accepts one bundle directory and --cache <cache-dir>"
                                .to_owned(),
                        );
                    }
                    package_publish(Path::new(&bundle_root), Path::new(&cache_root))
                }
                "install" => {
                    let request = parse_package_install_arguments(&mut arguments)?;
                    match request.source {
                        PackageInstallSource::Bundle(bundle_root) => {
                            package_install_bundle(&bundle_root, &request.output_directory)
                        }
                        PackageInstallSource::Cache {
                            cache_root,
                            name,
                            version,
                        } => package_install_cache(
                            &cache_root,
                            &name,
                            &version,
                            &request.output_directory,
                        ),
                    }
                }
                "verify-cache" => {
                    let cache_root = next_argument(&mut arguments, "package cache directory")?;
                    if arguments.next().is_some() {
                        return Err("pkg verify-cache accepts one cache directory".to_owned());
                    }
                    package_verify_cache(Path::new(&cache_root))
                }
                other => Err(format!("unknown pkg subcommand {other}")),
            }
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
                "trust-key" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut key_id = None;
                    let mut key_file = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--key-id" {
                            key_id = Some(next_argument(&mut args, "key id")?);
                        } else if flag == "--key-file" {
                            key_file = Some(next_argument(&mut args, "key file")?);
                        } else {
                            return Err(
                                "registry trust-key accepts --key-id --key-file".to_owned()
                            );
                        }
                    }
                    let key_id =
                        key_id.ok_or_else(|| "registry trust-key requires --key-id".to_owned())?;
                    let key_file = key_file
                        .ok_or_else(|| "registry trust-key requires --key-file".to_owned())?;
                    registry_trust_key(
                        Path::new(&root),
                        &key_id.to_string_lossy(),
                        Path::new(&key_file),
                    )
                }
                "trust-root" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let request = parse_registry_ed25519_key_request(
                        &mut arguments,
                        "trust-root",
                        false,
                    )?;
                    registry_trust_root(
                        Path::new(&root),
                        &request.key_id,
                        &request.seed_file,
                    )
                }
                "certify-ed25519-key" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let request = parse_registry_ed25519_key_request(
                        &mut arguments,
                        "certify-ed25519-key",
                        true,
                    )?;
                    let parent_key_id = request.parent_key_id.as_deref().ok_or_else(|| {
                        "registry certify-ed25519-key requires --parent-key-id".to_owned()
                    })?;
                    registry_certify_ed25519_key(
                        Path::new(&root),
                        &request.key_id,
                        &request.seed_file,
                        parent_key_id,
                    )
                }
                "pin-local-signed" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut name = None;
                    let mut version = None;
                    let mut artifact = None;
                    let mut key_id = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--name" {
                            name = Some(next_argument(&mut args, "package name")?);
                        } else if flag == "--version" {
                            version = Some(next_argument(&mut args, "package version")?);
                        } else if flag == "--artifact" {
                            artifact = Some(next_argument(&mut args, "artifact path")?);
                        } else if flag == "--key-id" {
                            key_id = Some(next_argument(&mut args, "key id")?);
                        } else {
                            return Err(
                                "registry pin-local-signed accepts --name --version --artifact --key-id"
                                    .to_owned(),
                            );
                        }
                    }
                    let name = name
                        .ok_or_else(|| "registry pin-local-signed requires --name".to_owned())?;
                    let version = version.ok_or_else(|| {
                        "registry pin-local-signed requires --version".to_owned()
                    })?;
                    let artifact = artifact.ok_or_else(|| {
                        "registry pin-local-signed requires --artifact".to_owned()
                    })?;
                    let key_id = key_id
                        .ok_or_else(|| "registry pin-local-signed requires --key-id".to_owned())?;
                    registry_pin_local_signed(
                        Path::new(&root),
                        &name.to_string_lossy(),
                        &version.to_string_lossy(),
                        Path::new(&artifact),
                        &key_id.to_string_lossy(),
                    )
                }
                "fetch-signed" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut name = None;
                    let mut version = None;
                    let mut url = None;
                    let mut signature = None;
                    let mut key_id = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--name" {
                            name = Some(next_argument(&mut args, "package name")?);
                        } else if flag == "--version" {
                            version = Some(next_argument(&mut args, "package version")?);
                        } else if flag == "--url" {
                            url = Some(next_argument(&mut args, "source url")?);
                        } else if flag == "--signature" {
                            signature = Some(next_argument(&mut args, "signature hex")?);
                        } else if flag == "--key-id" {
                            key_id = Some(next_argument(&mut args, "key id")?);
                        } else {
                            return Err(
                                "registry fetch-signed accepts --name --version --url --signature --key-id"
                                    .to_owned(),
                            );
                        }
                    }
                    let name = name
                        .ok_or_else(|| "registry fetch-signed requires --name".to_owned())?;
                    let version = version
                        .ok_or_else(|| "registry fetch-signed requires --version".to_owned())?;
                    let url =
                        url.ok_or_else(|| "registry fetch-signed requires --url".to_owned())?;
                    let signature = signature.ok_or_else(|| {
                        "registry fetch-signed requires --signature".to_owned()
                    })?;
                    let key_id = key_id
                        .ok_or_else(|| "registry fetch-signed requires --key-id".to_owned())?;
                    registry_fetch_signed(
                        Path::new(&root),
                        &name.to_string_lossy(),
                        &version.to_string_lossy(),
                        &url.to_string_lossy(),
                        &signature.to_string_lossy(),
                        &key_id.to_string_lossy(),
                    )
                }
                "revoke-key" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut key_id = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--key-id" {
                            key_id = Some(next_argument(&mut args, "key id")?);
                        } else {
                            return Err("registry revoke-key accepts --key-id".to_owned());
                        }
                    }
                    let key_id =
                        key_id.ok_or_else(|| "registry revoke-key requires --key-id".to_owned())?;
                    let key = revoke_trust_key(Path::new(&root), &key_id.to_string_lossy())
                        .map_err(|error| error.to_string())?;
                    println!(
                        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry revoked trust key {}",
                        key.key_id
                    );
                    Ok(())
                }
                "rotate-key" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut old_id = None;
                    let mut new_id = None;
                    let mut key_file = None;
                    let mut algorithm = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--old-key-id" {
                            old_id = Some(next_argument(&mut args, "old key id")?);
                        } else if flag == "--new-key-id" {
                            new_id = Some(next_argument(&mut args, "new key id")?);
                        } else if flag == "--key-file" {
                            key_file = Some(next_argument(&mut args, "key file")?);
                        } else if flag == "--algorithm" {
                            algorithm = Some(next_argument(&mut args, "algorithm")?);
                        } else {
                            return Err(
                                "registry rotate-key accepts --old-key-id --new-key-id --key-file [--algorithm]"
                                    .to_owned(),
                            );
                        }
                    }
                    let old_id = old_id
                        .ok_or_else(|| "registry rotate-key requires --old-key-id".to_owned())?;
                    let new_id = new_id
                        .ok_or_else(|| "registry rotate-key requires --new-key-id".to_owned())?;
                    let key_file = key_file
                        .ok_or_else(|| "registry rotate-key requires --key-file".to_owned())?;
                    let algorithm = algorithm
                        .as_ref()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "hmac-sha256".to_owned());
                    let bytes = fs::read(Path::new(&key_file)).map_err(|error| {
                        format!("could not read key file {}: {error}", key_file.to_string_lossy())
                    })?;
                    let key = rotate_trust_key(
                        Path::new(&root),
                        &old_id.to_string_lossy(),
                        &new_id.to_string_lossy(),
                        &bytes,
                        &algorithm,
                    )
                    .map_err(|error| error.to_string())?;
                    println!(
                        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry rotated trust key {} -> {} ({})",
                        old_id.to_string_lossy(),
                        key.key_id,
                        key.algorithm
                    );
                    Ok(())
                }
                "set-key-validity" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let mut key_id = None;
                    let mut not_before = None;
                    let mut not_after = None;
                    let mut args = arguments;
                    while let Some(flag) = args.next() {
                        if flag == "--key-id" {
                            key_id = Some(next_argument(&mut args, "key id")?);
                        } else if flag == "--not-before" {
                            not_before = Some(next_argument(&mut args, "not-before date")?);
                        } else if flag == "--not-after" {
                            not_after = Some(next_argument(&mut args, "not-after date")?);
                        } else {
                            return Err(
                                "registry set-key-validity accepts --key-id [--not-before] [--not-after]"
                                    .to_owned(),
                            );
                        }
                    }
                    let key_id = key_id.ok_or_else(|| {
                        "registry set-key-validity requires --key-id".to_owned()
                    })?;
                    let key = set_trust_key_validity(
                        Path::new(&root),
                        &key_id.to_string_lossy(),
                        not_before
                            .as_ref()
                            .map(|s| s.to_string_lossy().into_owned())
                            .as_deref(),
                        not_after
                            .as_ref()
                            .map(|s| s.to_string_lossy().into_owned())
                            .as_deref(),
                    )
                    .map_err(|error| error.to_string())?;
                    println!(
                        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry key {} validity not_before={:?} not_after={:?}",
                        key.key_id, key.not_before, key.not_after
                    );
                    Ok(())
                }
                "issue-x509-lite" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let request =
                        parse_x509_lite_certificate_request(&mut arguments, "issue-x509-lite")?;
                    let cert = issue_x509_lite_certificate(
                        Path::new(&root),
                        &request.issuer,
                        &request.subject,
                        &request.serial,
                        &request.not_before,
                        &request.not_after,
                    )
                    .map_err(|error| error.to_string())?;
                    write_or_print_x509_lite_pem(&cert, request.output, "issued")
                }
                "store-x509-lite" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    let request =
                        parse_x509_lite_certificate_request(&mut arguments, "store-x509-lite")?;
                    let cert = store_x509_lite_certificate(
                        Path::new(&root),
                        &request.issuer,
                        &request.subject,
                        &request.serial,
                        &request.not_before,
                        &request.not_after,
                    )
                    .map_err(|error| error.to_string())?;
                    write_or_print_x509_lite_pem(&cert, request.output, "stored")
                }
                "verify-x509-lite-store" => {
                    let root = next_argument(&mut arguments, "cache root")?;
                    if arguments.next().is_some() {
                        return Err(
                            "registry verify-x509-lite-store accepts one cache root directory"
                                .to_owned(),
                        );
                    }
                    let store =
                        verify_x509_lite_store(Path::new(&root)).map_err(|error| error.to_string())?;
                    println!(
                        "{LANGUAGE_NAME} {LANGUAGE_VERSION} registry X.509-lite store verified {} certificate(s) at {}",
                        store.certificates.len(),
                        Path::new(&root).display()
                    );
                    Ok(())
                }
                other => Err(format!(
                    "unknown registry subcommand {other} (use verify-cache, pin-local, trust-key, trust-root, certify-ed25519-key, pin-local-signed, fetch-signed, revoke-key, rotate-key, set-key-validity, issue-x509-lite, store-x509-lite, or verify-x509-lite-store)"
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
        "forge-bundle" => {
            let compiler = next_argument(&mut arguments, "compiler artifact")?;
            let bundle = next_argument(&mut arguments, "bundle file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("forge-bundle requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            if arguments.next().is_some() {
                return Err(
                    "forge-bundle accepts one compiler artifact, one bundle file, and one Aether artifact output file"
                        .to_owned(),
                );
            }
            forge_bundle(Path::new(&compiler), Path::new(&bundle), Path::new(&output))
        }
        "forge-modules" => {
            let compiler = next_argument(&mut arguments, "compiler artifact")?;
            let catalog = next_argument(&mut arguments, "catalog file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("forge-modules requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            if arguments.next().is_some() {
                return Err(
                    "forge-modules accepts one compiler artifact, one catalog file, and one Aether artifact output file"
                        .to_owned(),
                );
            }
            forge_modules(
                Path::new(&compiler),
                Path::new(&catalog),
                Path::new(&output),
            )
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
            let first = next_argument(&mut arguments, "source file or flag")?;
            let mut bootstrap = false;
            let source = if first == "--bootstrap" {
                bootstrap = true;
                next_argument(&mut arguments, "source file")?
            } else if first == "--product" {
                next_argument(&mut arguments, "source file")?
            } else {
                first
            };
            let mut output = None;
            while let Some(flag) = arguments.next() {
                if flag == "--bootstrap" {
                    bootstrap = true;
                } else if flag == "--product" {
                    // legacy synonym for default product path
                } else if flag == "--output" {
                    let path = next_argument(&mut arguments, "source output file")?;
                    output = Some(path);
                } else {
                    return Err(
                        "format accepts <source-file> [--bootstrap] [--output <file>] (product is default; --product is legacy)"
                            .to_owned(),
                    );
                }
            }
            format_file(
                Path::new(&source),
                output.as_ref().map(Path::new),
                bootstrap,
            )
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
                    let mut bootstrap = false;
                    for flag in arguments.by_ref() {
                        if flag == "--write" {
                            write = true;
                        } else if flag == "--bootstrap" {
                            bootstrap = true;
                        } else if flag == "--product" {
                            // legacy synonym for default product path
                        } else {
                            return Err(
                                "project format accepts optional --write and --bootstrap (product is default)"
                                    .to_owned()
                            );
                        }
                    }
                    project_format(Path::new(&project), write, bootstrap)
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
        "bench" => bench_runner::execute(&mut arguments),
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
    fn package_install_arguments_are_closed_and_complete() {
        let mut direct = ["bundle", "--output", "installed"]
            .into_iter()
            .map(OsString::from);
        let direct = parse_package_install_arguments(&mut direct).expect("direct install parse");
        assert_eq!(
            direct,
            PackageInstallArguments {
                source: PackageInstallSource::Bundle(PathBuf::from("bundle")),
                output_directory: PathBuf::from("installed"),
            }
        );

        let mut cache = [
            "--cache",
            "cache",
            "--version",
            "1.2.3",
            "--output",
            "installed",
            "--name",
            "math",
        ]
        .into_iter()
        .map(OsString::from);
        let cache = parse_package_install_arguments(&mut cache).expect("cache install parse");
        assert_eq!(
            cache,
            PackageInstallArguments {
                source: PackageInstallSource::Cache {
                    cache_root: PathBuf::from("cache"),
                    name: "math".to_owned(),
                    version: "1.2.3".to_owned(),
                },
                output_directory: PathBuf::from("installed"),
            }
        );

        let mut duplicate = [
            "--cache",
            "cache",
            "--name",
            "math",
            "--name",
            "other",
            "--version",
            "1.2.3",
            "--output",
            "installed",
        ]
        .into_iter()
        .map(OsString::from);
        let error = parse_package_install_arguments(&mut duplicate)
            .expect_err("duplicate cache package name must fail");
        assert!(error.contains("--name once"), "unexpected error: {error}");
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
    fn forge_bundle_invokes_compile_bundle_and_writes_only_a_verified_artifact() {
        let temporary = TemporaryDirectory::create();
        let target = compile_to_bytecode(
            "world target\n\nweave main [] -> Whole:\n  speak \"built by bundle forge\"\n  yield 0\n",
        )
        .expect("target source should compile")
        .bytecode;
        let compiler_source = format!(
            "world forge\n\nweave compile_bundle [borrow bundle: Text] -> Bytes:\n  bind target <- bytes \"{}\"\n  yield move target\n\nweave main [] -> Whole:\n  yield 0\n",
            hex_encode(&target)
        );
        let compiler = compile_to_bytecode(&compiler_source)
            .expect("bundle compiler fixture should compile")
            .bytecode;
        let compiler_path = temporary.path.join("compiler.aeth");
        let bundle_path = temporary.path.join("input.aeb");
        let output_path = temporary.path.join("output.aeth");
        fs::write(&compiler_path, compiler).expect("compiler artifact should be written");
        fs::write(&bundle_path, "aether.seed-bundle/v1\n").expect("bundle input should be written");

        forge_bundle(&compiler_path, &bundle_path, &output_path)
            .expect("bundle forge should write the compiler result");

        let generated = fs::read(&output_path).expect("bundle forge artifact should be readable");
        assert_eq!(generated, target);
        verify_bytecode(&generated).expect("bundle forge output must verify");
    }

    #[test]
    fn forge_modules_invokes_compile_modules_and_writes_only_a_verified_artifact() {
        let temporary = TemporaryDirectory::create();
        let target = compile_to_bytecode(
            "world target\n\nweave main [] -> Whole:\n  speak \"built by module forge\"\n  yield 0\n",
        )
        .expect("target source should compile")
        .bytecode;
        let compiler_source = format!(
            "world forge\n\nweave compile_modules [borrow catalog: Text] -> Bytes:\n  bind target <- bytes \"{}\"\n  yield move target\n\nweave main [] -> Whole:\n  yield 0\n",
            hex_encode(&target)
        );
        let compiler = compile_to_bytecode(&compiler_source)
            .expect("module compiler fixture should compile")
            .bytecode;
        let compiler_path = temporary.path.join("compiler.aeth");
        let catalog_path = temporary.path.join("input.aem");
        let output_path = temporary.path.join("output.aeth");
        fs::write(&compiler_path, compiler).expect("compiler artifact should be written");
        fs::write(&catalog_path, "aether.seed-modules/v1\n")
            .expect("catalog input should be written");

        forge_modules(&compiler_path, &catalog_path, &output_path)
            .expect("module forge should write the compiler result");

        let generated = fs::read(&output_path).expect("module forge artifact should be readable");
        assert_eq!(generated, target);
        verify_bytecode(&generated).expect("module forge output must verify");
    }

    #[test]
    fn product_check_accepts_seed_valid_source_without_bootstrap_ast() {
        assert!(
            product_cli_check_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: product is default check path"
        );
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("ok.ae");
        fs::write(
            &source_path,
            "world cli\n\nweave main [] -> Whole:\n  yield 0\n",
        )
        .expect("source should write");
        // bootstrap=false → product default (ADR-064)
        check(&source_path, false).expect("product check must accept valid seed surface");
    }

    #[test]
    fn product_format_normalizes_crlf_without_bootstrap() {
        assert!(
            product_format_without_bootstrap() && product_default_cli_toolchain(),
            "ADR-064: product is default format path"
        );
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("crlf.ae");
        let output_path = temporary.path.join("out.ae");
        fs::write(
            &source_path,
            "world cli\r\n\r\nweave main [] -> Whole:\r\n  yield 0\r\n",
        )
        .expect("source should write");
        // bootstrap=false → product default
        format_file(&source_path, Some(&output_path), false).expect("product format");
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
        let error = check(&source_path, false).expect_err("legacy must fail product check");
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
    fn compile_target_flag_is_closed_and_native_exe_only() {
        let mut valid = [
            "input.ae",
            "--output",
            "out.exe",
            "--native-exe",
            "--target",
            "aarch64-unknown-linux-gnu",
        ]
        .map(OsString::from)
        .into_iter();
        let parsed = parse_compile_arguments(&mut valid).expect("cross target arguments");
        assert!(parsed.native_exe);
        assert_eq!(
            parsed.native_target.as_deref(),
            Some("aarch64-unknown-linux-gnu")
        );

        let mut wrong_mode = [
            "input.ae",
            "--output",
            "out.c",
            "--native-c",
            "--target",
            "aarch64-unknown-linux-gnu",
        ]
        .map(OsString::from)
        .into_iter();
        let error = parse_compile_arguments(&mut wrong_mode)
            .expect_err("target without native executable must fail");
        assert!(error.contains("requires --native-exe"), "{error}");

        let mut duplicate = [
            "input.ae",
            "--output",
            "out.exe",
            "--native-exe",
            "--target",
            "x86_64-unknown-linux-gnu",
            "--target",
            "aarch64-unknown-linux-gnu",
        ]
        .map(OsString::from)
        .into_iter();
        let error =
            parse_compile_arguments(&mut duplicate).expect_err("duplicate target must fail");
        assert!(error.contains("at most once"), "{error}");
    }

    #[test]
    fn x509_lite_cli_request_requires_one_of_each_field() {
        let mut valid = [
            "--issuer",
            "ca-root",
            "--subject",
            "leaf",
            "--serial",
            "1",
            "--not-before",
            "2000-01-01",
            "--not-after",
            "2100-01-01",
        ]
        .map(OsString::from)
        .into_iter();
        let parsed = parse_x509_lite_certificate_request(&mut valid, "store-x509-lite")
            .expect("complete certificate request");
        assert_eq!(parsed.issuer, "ca-root");
        assert_eq!(parsed.subject, "leaf");
        assert!(parsed.output.is_none());

        let mut duplicate = [
            "--issuer",
            "ca-root",
            "--issuer",
            "other-root",
            "--subject",
            "leaf",
            "--serial",
            "1",
            "--not-before",
            "2000-01-01",
            "--not-after",
            "2100-01-01",
        ]
        .map(OsString::from)
        .into_iter();
        let error = parse_x509_lite_certificate_request(&mut duplicate, "store-x509-lite")
            .expect_err("duplicate issuer must fail");
        assert!(error.contains("--issuer once"), "{error}");
    }

    #[test]
    fn ed25519_ca_operator_key_requests_are_closed_and_complete() {
        let mut root = ["--key-id", "ca-root", "--seed-file", "root.seed"]
            .map(OsString::from)
            .into_iter();
        let parsed = parse_registry_ed25519_key_request(&mut root, "trust-root", false)
            .expect("root key request");
        assert_eq!(parsed.key_id, "ca-root");
        assert_eq!(parsed.seed_file, PathBuf::from("root.seed"));
        assert!(parsed.parent_key_id.is_none());

        let mut child = [
            "--key-id",
            "ca-mid",
            "--seed-file",
            "mid.seed",
            "--parent-key-id",
            "ca-root",
        ]
        .map(OsString::from)
        .into_iter();
        let parsed = parse_registry_ed25519_key_request(&mut child, "certify-ed25519-key", true)
            .expect("certified key request");
        assert_eq!(parsed.parent_key_id.as_deref(), Some("ca-root"));

        let mut root_with_parent = [
            "--key-id",
            "ca-root",
            "--seed-file",
            "root.seed",
            "--parent-key-id",
            "other",
        ]
        .map(OsString::from)
        .into_iter();
        let error = parse_registry_ed25519_key_request(&mut root_with_parent, "trust-root", false)
            .expect_err("root command cannot accept a parent");
        assert!(error.contains("--key-id --seed-file"), "{error}");
    }

    #[test]
    fn ed25519_ca_operator_helpers_prepare_a_storable_chain() {
        let temporary = TemporaryDirectory::create();
        let root_seed_file = temporary.path.join("root.seed");
        let mid_seed_file = temporary.path.join("mid.seed");
        let leaf_seed_file = temporary.path.join("leaf.seed");
        fs::write(&root_seed_file, [51u8; 32]).expect("root seed");
        fs::write(&mid_seed_file, [52u8; 32]).expect("intermediate seed");
        fs::write(&leaf_seed_file, [53u8; 32]).expect("leaf seed");
        registry_trust_root(temporary.path.as_path(), "ca-root", &root_seed_file)
            .expect("root command helper");
        registry_certify_ed25519_key(
            temporary.path.as_path(),
            "ca-mid",
            &mid_seed_file,
            "ca-root",
        )
        .expect("intermediate command helper");
        registry_certify_ed25519_key(temporary.path.as_path(), "leaf", &leaf_seed_file, "ca-mid")
            .expect("leaf command helper");
        store_x509_lite_certificate(
            temporary.path.as_path(),
            "ca-root",
            "ca-mid",
            "1",
            "2000-01-01",
            "2100-01-01",
        )
        .expect("intermediate certificate");
        store_x509_lite_certificate(
            temporary.path.as_path(),
            "ca-mid",
            "leaf",
            "2",
            "2000-01-01",
            "2100-01-01",
        )
        .expect("leaf certificate");
        let store = verify_x509_lite_store(temporary.path.as_path()).expect("store verify");
        assert_eq!(store.certificates.len(), 2);
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
