//! Offline Aether project documents (`aether.project/v1`).
//!
//! Projects list local units and optional SHA-256 locks. Verification is fully
//! local: schema, path confinement, lock digests, and seed compilation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{compile_with_seed, verify_bytecode, CompilerError, LANGUAGE_NAME, LANGUAGE_VERSION};

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocument {
    pub schema: String,
    pub name: String,
    pub version: String,
    pub units: Vec<ProjectUnit>,
    #[serde(default)]
    pub lock: Option<ProjectLock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectUnit {
    pub path: String,
    pub role: ProjectUnitRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectUnitRole {
    Main,
    Lib,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLock {
    pub units: Vec<ProjectLockUnit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

fn validate_unit_path(path: &str) -> Result<(), ProjectError> {
    if path.is_empty() {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            "unit path must be non-empty",
        ));
    }
    if path.starts_with('/') || path.starts_with('\\') {
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
    if !path.ends_with(".ae") {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must end with .ae"),
        ));
    }
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        return Err(ProjectError::new(
            "AE-PROJECT-002",
            format!("unit path {path} must be relative"),
        ));
    }
    for component in candidate.components() {
        match component {
            Component::Normal(part) => {
                let text = part.to_string_lossy();
                if text.is_empty() || text == "." {
                    return Err(ProjectError::new(
                        "AE-PROJECT-002",
                        format!("unit path {path} is illegal"),
                    ));
                }
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ProjectError::new(
                    "AE-PROJECT-002",
                    format!("unit path {path} escapes the project root"),
                ));
            }
        }
    }
    Ok(())
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
    let joined = root.join(unit_path);
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
/// and the resulting artifact is verified.
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
        let compiled = compile_with_seed(&source).map_err(|error: CompilerError| {
            ProjectError::new(
                "AE-PROJECT-004",
                format!("unit {} failed seed compile: {error}", unit.path),
            )
        })?;
        verify_bytecode(&compiled.bytecode).map_err(|error| {
            ProjectError::new(
                "AE-PROJECT-004",
                format!("unit {} produced an invalid artifact: {error}", unit.path),
            )
        })?;
        reports.push(ProjectUnitReport {
            path: unit.path.clone(),
            role: unit.role,
            sha256: digest,
            artifact_bytes: compiled.bytecode.len(),
        });
    }
    Ok(ProjectVerifyReport {
        name: document.name.clone(),
        version: document.version.clone(),
        units: reports,
    })
}

/// Canonical-format Aether source using the bootstrap formatter.
pub fn format_source(source: &str) -> Result<String, CompilerError> {
    let program = crate::compile_source(source)?;
    Ok(crate::format_program(&program))
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
}
