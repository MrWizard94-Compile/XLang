//! Offline Aether project documents (`aether.project/v1`).
//!
//! Projects list local units and optional SHA-256 locks. Verification is fully
//! local: schema, path confinement, lock digests, and seed compilation.
//!
//! M10: nested relative paths, multi-unit integrity, independent per-unit
//! seed compile. Units are not language modules (no cross-file linking).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::modules::{source_requires_project_modules, validate_lib_module_source};
use crate::{
    compile_product_bytecode, verify_bytecode, CompilerError, LANGUAGE_NAME, LANGUAGE_VERSION,
};

pub const PROJECT_SCHEMA_VERSION: &str = "aether.project/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectError {
    pub code: &'static str,
    pub message: String,
}

impl ProjectError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ProjectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} project error [{}]: {}",
            self.code, self.message
        )
    }
}

impl std::error::Error for ProjectError {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocument {
    pub schema: String,
    pub name: String,
    pub version: String,
    pub units: Vec<ProjectUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<ProjectLock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectUnit {
    pub path: String,
    pub role: ProjectUnitRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectUnitRole {
    Main,
    Lib,
    /// M17b: total program with main; may import project lib units; not product entry.
    Test,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLock {
    pub units: Vec<ProjectLockUnit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLockUnit {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectUnitReport {
    pub path: String,
    pub role: ProjectUnitRole,
    pub sha256: String,
    pub artifact_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectVerifyReport {
    pub name: String,
    pub version: String,
    pub units: Vec<ProjectUnitReport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFormatUnit {
    pub path: String,
    pub formatted: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFormatReport {
    pub name: String,
    pub version: String,
    pub units: Vec<ProjectFormatUnit>,
}

/// Parse and structurally validate an `aether.project/v1` JSON document.
pub fn parse_project_document(json: &str) -> Result<ProjectDocument, ProjectError> {
    if json.len() > 1_000_000 {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project document exceeds the 1_000_000-byte safety limit",
        ));
    }
    let document: ProjectDocument = serde_json::from_str(json).map_err(|error| {
        ProjectError::new(
            "AE-PROJECT-001",
            format!("invalid aether.project/v1 JSON: {error}"),
        )
    })?;
    validate_project_document(&document)?;
    Ok(document)
}

fn validate_project_document(document: &ProjectDocument) -> Result<(), ProjectError> {
    if document.schema != PROJECT_SCHEMA_VERSION {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            format!(
                "unsupported project schema {:?}; expected {PROJECT_SCHEMA_VERSION}",
                document.schema
            ),
        ));
    }
    if document.name.is_empty()
        || !document
            .name
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project name must be a non-empty lowercase ASCII identifier",
        ));
    }
    if document.version.is_empty() {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project version must be non-empty",
        ));
    }
    if document.units.is_empty() {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project units array must contain at least one unit",
        ));
    }
    if document.units.len() > 256 {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project units array exceeds the 256-unit safety limit",
        ));
    }
    let mut mains = 0_usize;
    let mut seen = BTreeSet::new();
    for unit in &document.units {
        if !seen.insert(unit.path.clone()) {
            return Err(ProjectError::new(
                "AE-PROJECT-001",
                format!("duplicate project unit path {}", unit.path),
            ));
        }
        validate_unit_path(&unit.path)?;
        if unit.role == ProjectUnitRole::Main {
            mains += 1;
        }
    }
    if mains != 1 {
        return Err(ProjectError::new(
            "AE-PROJECT-001",
            "project requires exactly one unit with role main",
        ));
    }
    if let Some(lock) = &document.lock {
        if lock.units.len() != document.units.len() {
            return Err(ProjectError::new(
                "AE-PROJECT-003",
                "project lock must list every unit path exactly once",
            ));
        }
        let mut lock_paths = BTreeSet::new();
        for entry in &lock.units {
            if !lock_paths.insert(entry.path.clone()) {
                return Err(ProjectError::new(
                    "AE-PROJECT-003",
                    format!("duplicate lock path {}", entry.path),
                ));
            }
            if !seen.contains(&entry.path) {
                return Err(ProjectError::new(
                    "AE-PROJECT-003",
                    format!("lock path {} is not a project unit", entry.path),
                ));
            }
            if !is_sha256_hex(&entry.sha256) {
                return Err(ProjectError::new(
                    "AE-PROJECT-003",
                    format!(
                        "lock digest for {} must be 64 lowercase hex characters",
                        entry.path
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Validate a project-document unit path (forward-slash grammar only).
///
/// Nested paths use `/` only. Backslash, `..`, `.`, absolute, and drive paths
/// fail closed (`AE-PROJECT-002`).
pub fn validate_unit_path(path: &str) -> Result<(), ProjectError> {
    if path.is_empty() {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            "unit path must be non-empty",
        ));
    }
    if path.contains('\\') {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must use forward slashes only"),
        ));
    }
    if path.starts_with('/') {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must be relative"),
        ));
    }
    if path.contains(':') {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must not contain a drive or URL prefix"),
        ));
    }
    if path.contains("//") {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must not contain empty segments"),
        ));
    }
    if !path.ends_with(".ae") {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must end with .ae"),
        ));
    }
    let segments: Vec<&str> = path.split('/').collect();
    if segments.is_empty() {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            "unit path must be non-empty",
        ));
    }
    for segment in &segments {
        if *segment == "." || *segment == ".." {
            return Err(ProjectError::new(
                "AE-PROJECT-002",
                format!("unit path {path} escapes the project root"),
            ));
        }
        if segment.is_empty() {
            return Err(ProjectError::new(
                "AE-PROJECT-002",
                format!("unit path {path} must not contain empty segments"),
            ));
        }
        if !is_safe_path_segment(segment) {
            return Err(ProjectError::new(
                "AE-PROJECT-002",
                format!("unit path {path} contains an illegal segment"),
            ));
        }
    }
    Ok(())
}

fn is_safe_path_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment.bytes().all(
            |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'),
        )
}

/// Deterministic flat artifact file name for `project verify --output-dir`.
///
/// `src/main.ae` → `src__main.aeth`
#[must_use]
pub fn unit_artifact_file_name(unit_path: &str) -> String {
    let without_ext = unit_path
        .strip_suffix(".ae")
        .unwrap_or(unit_path)
        .replace('/', "__");
    format!("{without_ext}.aeth")
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// SHA-256 lowercase hex digest of raw file bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

/// Derive a complete local unit lock after verifying an unlocked project view.
///
/// An existing valid-but-stale lock is intentionally ignored while deriving a
/// replacement: refresh must repair a changed project rather than requiring
/// authors to hand-delete its old hashes first. The source is still verified by
/// the normal project pipeline before the replacement lock is returned.
pub fn refresh_project_lock(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<ProjectDocument, ProjectError> {
    let mut unlocked = document.clone();
    unlocked.lock = None;
    let before = project_lock_units(project_root, &unlocked)?;
    let _ = verify_project(project_root, &unlocked)?;
    let after = project_lock_units(project_root, &unlocked)?;
    if before != after {
        return Err(ProjectError::new(
            "AE-PROJECT-003",
            "project source changed while project lock refresh was in progress",
        ));
    }

    let mut refreshed = document.clone();
    refreshed.lock = Some(ProjectLock { units: after });
    validate_project_document(&refreshed)?;
    Ok(refreshed)
}

/// Render a validated project document as canonical local JSON.
pub fn serialize_project_document(document: &ProjectDocument) -> Result<String, ProjectError> {
    validate_project_document(document)?;
    serde_json::to_string_pretty(document)
        .map(|json| format!("{json}\n"))
        .map_err(|error| {
            ProjectError::new(
                "AE-PROJECT-001",
                format!("could not serialize aether.project/v1 document: {error}"),
            )
        })
}

fn project_lock_units(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<Vec<ProjectLockUnit>, ProjectError> {
    document
        .units
        .iter()
        .map(|unit| {
            let path = resolve_unit_path(project_root, &unit.path)?;
            let bytes = fs::read(&path).map_err(|error| {
                ProjectError::new(
                    "AE-PROJECT-002",
                    format!("could not read unit {}: {error}", unit.path),
                )
            })?;
            Ok(ProjectLockUnit {
                path: unit.path.clone(),
                sha256: sha256_hex(&bytes),
            })
        })
        .collect()
}

/// Join project-document path segments under the project root.
fn join_unit_path(project_root: &Path, unit_path: &str) -> PathBuf {
    let mut joined = project_root.to_path_buf();
    for segment in unit_path.split('/') {
        joined.push(segment);
    }
    joined
}

/// Resolve a unit path under the project root, rejecting escapes.
pub fn resolve_unit_path(project_root: &Path, unit_path: &str) -> Result<PathBuf, ProjectError> {
    validate_unit_path(unit_path)?;
    let root = project_root.canonicalize().map_err(|error| {
        ProjectError::new(
            "AE-PROJECT-002",
            format!(
                "could not resolve project root {}: {error}",
                project_root.display()
            ),
        )
    })?;
    let joined = join_unit_path(&root, unit_path);
    let resolved = joined.canonicalize().map_err(|error| {
        ProjectError::new(
            "AE-PROJECT-002",
            format!("could not resolve unit path {unit_path}: {error}"),
        )
    })?;
    if !resolved.starts_with(&root) {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {unit_path} escapes the project root"),
        ));
    }
    Ok(resolved)
}

/// Verify a project document against files under `project_root`.
///
/// When `lock` is present, file digests must match. Every unit is seed-compiled
/// independently and the resulting artifact is verified.
pub fn verify_project(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<ProjectVerifyReport, ProjectError> {
    validate_project_document(document)?;
    let lock_map = document.lock.as_ref().map(|lock| {
        lock.units
            .iter()
            .map(|unit| (unit.path.as_str(), unit.sha256.as_str()))
            .collect::<BTreeMap<_, _>>()
    });

    let mut reports = Vec::with_capacity(document.units.len());
    for unit in &document.units {
        let path = resolve_unit_path(project_root, &unit.path)?;
        let bytes = fs::read(&path).map_err(|error| {
            ProjectError::new(
                "AE-PROJECT-002",
                format!("could not read unit {}: {error}", unit.path),
            )
        })?;
        let digest = sha256_hex(&bytes);
        if let Some(map) = &lock_map {
            let expected = map.get(unit.path.as_str()).ok_or_else(|| {
                ProjectError::new(
                    "AE-PROJECT-003",
                    format!("lock is missing unit path {}", unit.path),
                )
            })?;
            if *expected != digest {
                return Err(ProjectError::new(
                    "AE-PROJECT-003",
                    format!(
                        "lock digest mismatch for {}: expected {expected}, observed {digest}",
                        unit.path
                    ),
                ));
            }
        }
        let source = String::from_utf8(bytes).map_err(|_| {
            ProjectError::new(
                "AE-PROJECT-004",
                format!("unit {} is not valid UTF-8", unit.path),
            )
        })?;
        // M11: lib units without main (or any unit using import/export) are not
        // independently seed-compiled; validate module surface instead.
        let module_surface = source_requires_project_modules(&source)
            || (unit.role == ProjectUnitRole::Lib
                && !source.lines().any(|line| {
                    let t = line.trim_start();
                    t.starts_with("weave main ") || t.starts_with("export weave main ")
                }))
            || unit.role == ProjectUnitRole::Test;
        let artifact_bytes = if module_surface {
            if unit.role == ProjectUnitRole::Lib {
                validate_lib_module_source(&unit.path, &source)?;
            } else if unit.role == ProjectUnitRole::Test || source_requires_project_modules(&source)
            {
                // Main/test with imports: full graph check deferred to project build/test.
                if !source.lines().any(|line| {
                    let t = line.trim_start();
                    t.starts_with("weave main ") || t.starts_with("export weave main ")
                }) {
                    return Err(ProjectError::new(
                        "AE-MOD-006",
                        format!("entry unit {} must declare weave main", unit.path),
                    ));
                }
            }
            0
        } else {
            let bytecode = compile_product_bytecode(&source).map_err(|error: CompilerError| {
                ProjectError::new(
                    "AE-PROJECT-004",
                    format!("unit {} failed seed compile: {error}", unit.path),
                )
            })?;
            verify_bytecode(&bytecode).map_err(|error| {
                ProjectError::new(
                    "AE-PROJECT-004",
                    format!("unit {} produced an invalid artifact: {error}", unit.path),
                )
            })?;
            bytecode.len()
        };
        reports.push(ProjectUnitReport {
            path: unit.path.clone(),
            role: unit.role,
            sha256: digest,
            artifact_bytes,
        });
    }
    Ok(ProjectVerifyReport {
        name: document.name.clone(),
        version: document.version.clone(),
        units: reports,
    })
}

/// Format every unit in declaration order.
///
/// When `product` is false (default CLI path), uses bootstrap [`format_source`].
/// When `product` is true (ADR-054), uses [`format_source_product`] only — no
/// bootstrap AST rewrite.
///
/// Does not write files. Callers may write with an explicit CLI flag.
pub fn format_project(
    project_root: &Path,
    document: &ProjectDocument,
    product: bool,
) -> Result<ProjectFormatReport, ProjectError> {
    if product {
        debug_assert!(
            crate::product_project_format_without_bootstrap(),
            "ADR-054: product project format must not require bootstrap"
        );
    }
    validate_project_document(document)?;
    let mut units = Vec::with_capacity(document.units.len());
    for unit in &document.units {
        let path = resolve_unit_path(project_root, &unit.path)?;
        let bytes = fs::read(&path).map_err(|error| {
            ProjectError::new(
                "AE-PROJECT-002",
                format!("could not read unit {}: {error}", unit.path),
            )
        })?;
        let source = String::from_utf8(bytes).map_err(|_| {
            ProjectError::new(
                "AE-PROJECT-004",
                format!("unit {} is not valid UTF-8", unit.path),
            )
        })?;
        let formatted = if product {
            format_unit_product(&unit.path, unit.role, &source)?
        } else {
            format_source(&source).map_err(|error| {
                ProjectError::new(
                    "AE-PROJECT-004",
                    format!("unit {} failed format: {error}", unit.path),
                )
            })?
        };
        units.push(ProjectFormatUnit {
            path: unit.path.clone(),
            formatted,
        });
    }
    Ok(ProjectFormatReport {
        name: document.name.clone(),
        version: document.version.clone(),
        units,
    })
}

/// Canonical-format Aether source using the bootstrap formatter.
///
/// ADR-062: when both product and bootstrap reject the source, prefer product
/// `AE-SEED-*` diagnostics (bootstrap AST authority only on success or when
/// bootstrap alone can accept — e.g. some lib surfaces).
pub fn format_source(source: &str) -> Result<String, CompilerError> {
    debug_assert!(
        crate::format_source_product_base_gate(),
        "ADR-062: format_source product base gate"
    );
    let product_result = crate::compile_product_bytecode(source);
    let program = match crate::compile_source(source) {
        Ok(program) => program,
        Err(bootstrap_error) => {
            product_result?;
            return Err(bootstrap_error);
        }
    };
    Ok(crate::format_program(&program))
}

/// Product-path format: LF normalize + seed accept (ADR-053).
///
/// Does **not** invoke the bootstrap compiler and does **not** rewrite to full
/// AST-canonical form ([`format_source`] / `format_program`). Use default
/// [`format_source`] when bootstrap-canonical rewrite is required.
pub fn format_source_product(source: &str) -> Result<String, CompilerError> {
    debug_assert!(
        crate::product_format_without_bootstrap(),
        "ADR-053: product format must not require bootstrap"
    );
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    crate::compile_product_bytecode(&normalized)?;
    Ok(normalized)
}

/// Product project-unit format (ADR-054): LF normalize + role-aware product accept.
///
/// - Standalone main/test (no module surface): [`format_source_product`]
/// - Lib units: LF + product lib probe ([`validate_lib_module_source`])
/// - Units with import/export: LF only after module surface structure checks
///   (single-file product emit cannot elaborate imports)
fn format_unit_product(
    unit_path: &str,
    role: ProjectUnitRole,
    source: &str,
) -> Result<String, ProjectError> {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    if role == ProjectUnitRole::Lib {
        validate_lib_module_source(unit_path, &normalized)?;
        return Ok(normalized);
    }
    if source_requires_project_modules(&normalized) {
        // Single-file product emit cannot elaborate imports; LF-normalize only.
        // Entry units still require weave main (aligned with verify_project).
        if (role == ProjectUnitRole::Main || role == ProjectUnitRole::Test)
            && !normalized.lines().any(|line| {
                let t = line.trim_start();
                t.starts_with("weave main ")
                    || t.starts_with("export weave main ")
                    || t.starts_with("task weave main ")
            })
        {
            return Err(ProjectError::new(
                "AE-MOD-006",
                format!("entry unit {unit_path} must declare weave main"),
            ));
        }
        return Ok(normalized);
    }
    format_source_product(source).map_err(|error| {
        ProjectError::new(
            "AE-PROJECT-004",
            format!("unit {unit_path} failed product format: {error}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> PathBuf {
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("aether-project-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    #[test]
    fn verifies_locked_project_and_rejects_escape_or_mismatch() {
        let root = temp_dir();
        let source = "world project_demo\n\nweave main [] -> Whole:\n  yield 7\n";
        fs::write(root.join("main.ae"), source).expect("write unit");
        let digest = sha256_hex(source.as_bytes());
        let json = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "project_demo",
  "version": "0.1.0",
  "units": [{{ "path": "main.ae", "role": "main" }}],
  "lock": {{ "units": [{{ "path": "main.ae", "sha256": "{digest}" }}] }}
}}"#
        );
        let document = parse_project_document(&json).expect("project parses");
        let report = verify_project(&root, &document).expect("verify");
        assert_eq!(report.units.len(), 1);
        assert_eq!(report.units[0].sha256, digest);

        let escaped = r#"{
  "schema": "aether.project/v1",
  "name": "bad",
  "version": "0.1.0",
  "units": [{ "path": "../escape.ae", "role": "main" }]
}"#;
        let error = parse_project_document(escaped).expect_err("escape rejected");
        assert_eq!(error.code, "AE-PROJECT-002");

        let mismatch = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "project_demo",
  "version": "0.1.0",
  "units": [{{ "path": "main.ae", "role": "main" }}],
  "lock": {{ "units": [{{ "path": "main.ae", "sha256": "{}" }}] }}
}}"#,
            "0".repeat(64)
        );
        let document = parse_project_document(&mismatch).expect("parse");
        let error = verify_project(&root, &document).expect_err("lock mismatch");
        assert_eq!(error.code, "AE-PROJECT-003");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn format_source_is_canonical() {
        let messy = "world fmt\n\nweave main [] -> Whole:\n  yield sum 1 2\n";
        let formatted = format_source(messy).expect("format");
        assert_eq!(
            formatted,
            "world fmt\n\nweave main [] -> Whole:\n  yield sum 1 2\n"
        );
    }

    #[test]
    fn format_source_product_normalizes_lf_and_accepts_without_bootstrap() {
        assert!(crate::product_format_without_bootstrap(), "ADR-053 tracker");
        let crlf = "world fmt\r\n\r\nweave main [] -> Whole:\r\n  yield 1\r\n";
        let formatted = format_source_product(crlf).expect("product format");
        assert_eq!(
            formatted,
            "world fmt\n\nweave main [] -> Whole:\n  yield 1\n"
        );
        let legacy = format_source_product("world w\n\nfn main() -> Int { return 0; }\n")
            .expect_err("legacy must fail product format");
        assert!(legacy.to_string().contains("AE-SEED-007"), "got {legacy}");
    }

    #[test]
    fn format_source_prefers_product_ae_seed_when_both_reject() {
        assert!(crate::format_source_product_base_gate(), "ADR-062 tracker");
        let legacy = format_source("world w\n\nfn main() -> Int { return 0; }\n")
            .expect_err("legacy must fail");
        assert!(
            legacy.to_string().contains("AE-SEED-007"),
            "expected product AE-SEED-007, got {legacy}"
        );
    }

    #[test]
    fn product_project_format_handles_lib_and_main_without_bootstrap() {
        assert!(
            crate::product_project_format_without_bootstrap(),
            "ADR-054 tracker"
        );
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).expect("lib");
        let main_src = "world multi_main\r\n\r\nweave main [] -> Whole:\r\n  yield 3\r\n";
        let lib_src = "world multi_lib\r\n\r\nexport weave helper [] -> Whole:\r\n  yield 9\r\n";
        fs::write(root.join("main.ae"), main_src).expect("main");
        fs::write(root.join("lib").join("helper.ae"), lib_src).expect("lib");
        let json = r#"{
  "schema": "aether.project/v1",
  "name": "product_fmt",
  "version": "0.1.0",
  "units": [
    { "path": "main.ae", "role": "main" },
    { "path": "lib/helper.ae", "role": "lib" }
  ]
}"#;
        let document = parse_project_document(json).expect("parse");
        let report = format_project(&root, &document, true).expect("product project format");
        assert_eq!(report.units.len(), 2);
        assert_eq!(
            report.units[0].formatted,
            "world multi_main\n\nweave main [] -> Whole:\n  yield 3\n"
        );
        assert_eq!(
            report.units[1].formatted,
            "world multi_lib\n\nexport weave helper [] -> Whole:\n  yield 9\n"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn nested_multi_unit_project_verifies_with_lock() {
        let root = temp_dir();
        fs::create_dir_all(root.join("src")).expect("src");
        fs::create_dir_all(root.join("lib")).expect("lib");
        let main_src = "world multi_main\n\nweave main [] -> Whole:\n  yield 3\n";
        let lib_src = "world multi_lib\n\nweave main [] -> Whole:\n  yield 9\n";
        fs::write(root.join("src").join("main.ae"), main_src).expect("main");
        fs::write(root.join("lib").join("helper.ae"), lib_src).expect("lib");
        let main_digest = sha256_hex(main_src.as_bytes());
        let lib_digest = sha256_hex(lib_src.as_bytes());
        let json = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "multi_demo",
  "version": "0.1.0",
  "units": [
    {{ "path": "src/main.ae", "role": "main" }},
    {{ "path": "lib/helper.ae", "role": "lib" }}
  ],
  "lock": {{
    "units": [
      {{ "path": "src/main.ae", "sha256": "{main_digest}" }},
      {{ "path": "lib/helper.ae", "sha256": "{lib_digest}" }}
    ]
  }}
}}"#
        );
        let document = parse_project_document(&json).expect("parse multi");
        let report = verify_project(&root, &document).expect("verify multi");
        assert_eq!(report.units.len(), 2);
        assert_eq!(report.units[0].path, "src/main.ae");
        assert_eq!(report.units[1].path, "lib/helper.ae");
        assert_eq!(unit_artifact_file_name("src/main.ae"), "src__main.aeth");
        assert_eq!(unit_artifact_file_name("lib/helper.ae"), "lib__helper.aeth");

        let formatted = format_project(&root, &document, false).expect("format multi");
        assert_eq!(formatted.units.len(), 2);
        assert_eq!(formatted.units[0].formatted, main_src);
        assert_eq!(formatted.units[1].formatted, lib_src);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_illegal_paths_roles_unknown_fields_and_bad_source() {
        for (json, code) in [
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"/abs/main.ae","role":"main"}]}"#,
                "AE-PROJECT-002",
            ),
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"src\\main.ae","role":"main"}]}"#,
                "AE-PROJECT-002",
            ),
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"src/./main.ae","role":"main"}]}"#,
                "AE-PROJECT-002",
            ),
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"a.ae","role":"main"},{"path":"b.ae","role":"main"}]}"#,
                "AE-PROJECT-001",
            ),
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"a.ae","role":"lib"}]}"#,
                "AE-PROJECT-001",
            ),
            (
                r#"{"schema":"aether.project/v1","name":"x","version":"1","units":[{"path":"a.ae","role":"main"}],"depends_on":[]}"#,
                "AE-PROJECT-001",
            ),
        ] {
            let error = parse_project_document(json).expect_err(json);
            assert_eq!(error.code, code, "{json}");
        }

        let root = temp_dir();
        fs::create_dir_all(root.join("src")).expect("src");
        fs::create_dir_all(root.join("lib")).expect("lib");
        let main_src = "world multi_main\n\nweave main [] -> Whole:\n  yield 1\n";
        let bad_lib = "world multi_lib\n\nthis is not valid aether\n";
        fs::write(root.join("src").join("main.ae"), main_src).expect("main");
        fs::write(root.join("lib").join("helper.ae"), bad_lib).expect("lib");
        let json = r#"{
  "schema": "aether.project/v1",
  "name": "bad_unit",
  "version": "0.1.0",
  "units": [
    { "path": "src/main.ae", "role": "main" },
    { "path": "lib/helper.ae", "role": "lib" }
  ]
}"#;
        let document = parse_project_document(json).expect("parse");
        let error = verify_project(&root, &document).expect_err("bad lib");
        assert!(
            error.code == "AE-PROJECT-004" || error.code == "AE-MOD-001",
            "unexpected {}",
            error.code
        );

        // Independence: main does not resolve weaves from another file.
        let main_only =
            "world only_main\n\nweave main [] -> Whole:\n  yield call helper_from_lib\n";
        assert!(crate::compile_source(main_only).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn multi_unit_lock_must_cover_every_unit() {
        let root = temp_dir();
        fs::create_dir_all(root.join("src")).expect("src");
        fs::create_dir_all(root.join("lib")).expect("lib");
        let main_src = "world multi_main\n\nweave main [] -> Whole:\n  yield 1\n";
        let lib_src = "world multi_lib\n\nweave main [] -> Whole:\n  yield 2\n";
        fs::write(root.join("src").join("main.ae"), main_src).expect("main");
        fs::write(root.join("lib").join("helper.ae"), lib_src).expect("lib");
        let main_digest = sha256_hex(main_src.as_bytes());
        let incomplete = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "incomplete_lock",
  "version": "0.1.0",
  "units": [
    {{ "path": "src/main.ae", "role": "main" }},
    {{ "path": "lib/helper.ae", "role": "lib" }}
  ],
  "lock": {{
    "units": [
      {{ "path": "src/main.ae", "sha256": "{main_digest}" }}
    ]
  }}
}}"#
        );
        let error = parse_project_document(&incomplete).expect_err("incomplete lock");
        assert_eq!(error.code, "AE-PROJECT-003");

        let wrong = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "wrong_lock",
  "version": "0.1.0",
  "units": [
    {{ "path": "src/main.ae", "role": "main" }},
    {{ "path": "lib/helper.ae", "role": "lib" }}
  ],
  "lock": {{
    "units": [
      {{ "path": "src/main.ae", "sha256": "{main_digest}" }},
      {{ "path": "lib/helper.ae", "sha256": "{}" }}
    ]
  }}
}}"#,
            "0".repeat(64)
        );
        let document = parse_project_document(&wrong).expect("parse");
        let error = verify_project(&root, &document).expect_err("mismatch");
        assert_eq!(error.code, "AE-PROJECT-003");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn refresh_project_lock_replaces_a_stale_lock_after_validation() {
        let root = temp_dir();
        let source = "world lock_refresh\n\nweave main [] -> Whole:\n  yield 11\n";
        fs::write(root.join("main.ae"), source).expect("write source");
        let stale = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "lock_refresh",
  "version": "0.1.0",
  "units": [{{ "path": "main.ae", "role": "main" }}],
  "lock": {{ "units": [{{ "path": "main.ae", "sha256": "{}" }}] }}
}}"#,
            "0".repeat(64)
        );
        let document = parse_project_document(&stale).expect("stale lock is structurally valid");

        let refreshed = refresh_project_lock(&root, &document).expect("refresh project lock");
        let lock = refreshed.lock.as_ref().expect("refreshed lock");
        assert_eq!(lock.units.len(), 1);
        assert_eq!(lock.units[0].path, "main.ae");
        assert_eq!(lock.units[0].sha256, sha256_hex(source.as_bytes()));
        assert_ne!(lock.units[0].sha256, "0".repeat(64));

        let rendered = serialize_project_document(&refreshed).expect("serialize refreshed lock");
        assert!(rendered.ends_with('\n'));
        let reparsed = parse_project_document(&rendered).expect("parse canonical project document");
        verify_project(&root, &reparsed).expect("refreshed project lock verifies");
        let _ = fs::remove_dir_all(&root);
    }
}
