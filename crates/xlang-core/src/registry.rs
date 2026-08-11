//! M24a offline registry cache (F-REGISTRY pilot, ADR-060).
//!
//! No network. Digest-bound package pins under a path-jailed cache root.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::sha256_hex;

pub const REGISTRY_CACHE_SCHEMA: &str = "aether.registry-cache/v1";
pub const REGISTRY_INDEX_FILE: &str = "aether.registry-cache.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryError {
    pub code: &'static str,
    pub message: String,
}

impl RegistryError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryCacheDocument {
    pub schema: String,
    pub packages: Vec<RegistryPackagePin>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryPackagePin {
    pub name: String,
    pub version: String,
    /// Relative path under cache root (POSIX-style, no `..`).
    pub artifact: String,
    pub sha256: String,
}

#[must_use]
pub const fn f_registry_authorized() -> bool {
    true
}

#[must_use]
pub const fn registry_offline_cache_verify() -> bool {
    true
}

pub fn empty_registry_cache() -> RegistryCacheDocument {
    RegistryCacheDocument {
        schema: REGISTRY_CACHE_SCHEMA.to_owned(),
        packages: Vec::new(),
    }
}

pub fn parse_registry_cache(json: &str) -> Result<RegistryCacheDocument, RegistryError> {
    let document: RegistryCacheDocument = serde_json::from_str(json).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("invalid registry cache JSON: {error}"),
        )
    })?;
    if document.schema != REGISTRY_CACHE_SCHEMA {
        return Err(RegistryError::new(
            "AE-REG-001",
            format!(
                "unsupported registry cache schema {} (want {REGISTRY_CACHE_SCHEMA})",
                document.schema
            ),
        ));
    }
    for package in &document.packages {
        validate_pin_fields(package)?;
    }
    Ok(document)
}

pub fn serialize_registry_cache(document: &RegistryCacheDocument) -> Result<String, RegistryError> {
    serde_json::to_string_pretty(document).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("could not serialize registry cache: {error}"),
        )
    })
}

fn validate_pin_fields(package: &RegistryPackagePin) -> Result<(), RegistryError> {
    if package.name.is_empty() || package.version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    if package.sha256.len() != 64 || !package.sha256.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RegistryError::new(
            "AE-REG-003",
            format!(
                "package {}@{} sha256 must be 64 hex characters",
                package.name, package.version
            ),
        ));
    }
    validate_relative_artifact_path(&package.artifact)?;
    Ok(())
}

fn validate_relative_artifact_path(path: &str) -> Result<(), RegistryError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.starts_with('\\')
        || path.contains("..")
        || path.contains('\\')
        || Path::new(path).is_absolute()
    {
        return Err(RegistryError::new(
            "AE-REG-004",
            format!("artifact path must be relative and path-jailed: {path}"),
        ));
    }
    Ok(())
}

/// Pin a local artifact into the cache (offline). Copies bytes under
/// `packages/<name>/<version>/` and records SHA-256.
pub fn pin_local_package(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact_path: &Path,
) -> Result<RegistryPackagePin, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_offline_cache_verify(),
        "ADR-060: registry offline pilot"
    );
    if name.is_empty() || version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    let bytes = fs::read(artifact_path).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!(
                "could not read artifact {}: {error}",
                artifact_path.display()
            ),
        )
    })?;
    let digest = sha256_hex(&bytes);
    let relative = format!("packages/{name}/{version}/artifact.bin");
    validate_relative_artifact_path(&relative)?;
    let destination = resolve_cache_path(cache_root, &relative)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            RegistryError::new(
                "AE-REG-005",
                format!("could not create package directory: {error}"),
            )
        })?;
    }
    fs::write(&destination, &bytes).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write package artifact: {error}"),
        )
    })?;

    let pin = RegistryPackagePin {
        name: name.to_owned(),
        version: version.to_owned(),
        artifact: relative,
        sha256: digest,
    };
    let mut document = load_or_empty_cache(cache_root)?;
    document
        .packages
        .retain(|existing| !(existing.name == pin.name && existing.version == pin.version));
    document.packages.push(pin.clone());
    document
        .packages
        .sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    write_cache_index(cache_root, &document)?;
    Ok(pin)
}

pub fn verify_registry_cache(cache_root: &Path) -> Result<RegistryCacheDocument, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_offline_cache_verify(),
        "ADR-060: registry offline pilot"
    );
    let document = load_or_empty_cache(cache_root)?;
    for package in &document.packages {
        validate_pin_fields(package)?;
        let path = resolve_cache_path(cache_root, &package.artifact)?;
        let bytes = fs::read(&path).map_err(|error| {
            RegistryError::new(
                "AE-REG-005",
                format!(
                    "package {}@{} missing artifact {}: {error}",
                    package.name, package.version, package.artifact
                ),
            )
        })?;
        let actual = sha256_hex(&bytes);
        if actual != package.sha256 {
            return Err(RegistryError::new(
                "AE-REG-003",
                format!(
                    "package {}@{} digest mismatch (expected {}, got {actual})",
                    package.name, package.version, package.sha256
                ),
            ));
        }
    }
    Ok(document)
}

fn load_or_empty_cache(cache_root: &Path) -> Result<RegistryCacheDocument, RegistryError> {
    let index = cache_root.join(REGISTRY_INDEX_FILE);
    if !index.is_file() {
        return Ok(empty_registry_cache());
    }
    let json = fs::read_to_string(&index).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not read registry index: {error}"),
        )
    })?;
    parse_registry_cache(&json)
}

fn write_cache_index(
    cache_root: &Path,
    document: &RegistryCacheDocument,
) -> Result<(), RegistryError> {
    fs::create_dir_all(cache_root).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not create cache root: {error}"),
        )
    })?;
    let json = serialize_registry_cache(document)?;
    fs::write(cache_root.join(REGISTRY_INDEX_FILE), json).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write registry index: {error}"),
        )
    })
}

fn resolve_cache_path(cache_root: &Path, relative: &str) -> Result<PathBuf, RegistryError> {
    validate_relative_artifact_path(relative)?;
    let root = cache_root
        .canonicalize()
        .unwrap_or_else(|_| cache_root.to_path_buf());
    let joined = root.join(relative);
    if let Ok(canonical) = joined.canonicalize() {
        if !canonical.starts_with(&root) {
            return Err(RegistryError::new(
                "AE-REG-004",
                format!("artifact path escapes cache root: {relative}"),
            ));
        }
        return Ok(canonical);
    }
    // File may not exist yet (pin write path): jail parent.
    if let Some(parent) = joined.parent() {
        if parent.exists() {
            let parent_canon = parent.canonicalize().map_err(|error| {
                RegistryError::new("AE-REG-004", format!("path jail failed: {error}"))
            })?;
            if !parent_canon.starts_with(&root) {
                return Err(RegistryError::new(
                    "AE-REG-004",
                    format!("artifact path escapes cache root: {relative}"),
                ));
            }
        }
    }
    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> PathBuf {
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("aether-registry-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("temp");
        path
    }

    #[test]
    fn pin_and_verify_cache_round_trip() {
        assert!(f_registry_authorized());
        assert!(registry_offline_cache_verify());
        let root = temp_dir();
        let artifact = root.join("input.aeth");
        fs::write(&artifact, b"AETH\x0bpure-fixture").expect("write");
        let pin = pin_local_package(&root, "demo", "1.0.0", &artifact).expect("pin");
        assert_eq!(pin.name, "demo");
        assert_eq!(pin.sha256.len(), 64);
        let verified = verify_registry_cache(&root).expect("verify");
        assert_eq!(verified.packages.len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn verify_rejects_tampered_artifact() {
        let root = temp_dir();
        let artifact = root.join("input.aeth");
        fs::write(&artifact, b"AETH\x0bgood").expect("write");
        let pin = pin_local_package(&root, "demo", "1.0.0", &artifact).expect("pin");
        let dest = root.join(&pin.artifact);
        fs::write(&dest, b"AETH\x0btampered").expect("tamper");
        let error = verify_registry_cache(&root).expect_err("tamper must fail");
        assert_eq!(error.code, "AE-REG-003");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_path_escape_in_index() {
        let json = r#"{
  "schema": "aether.registry-cache/v1",
  "packages": [{
    "name": "x",
    "version": "1",
    "artifact": "../escape.bin",
    "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  }]
}"#;
        let error = parse_registry_cache(json).expect_err("escape");
        assert_eq!(error.code, "AE-REG-004");
    }
}
