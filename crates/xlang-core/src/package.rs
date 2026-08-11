//! Offline local package publication (`aether.package/v1`) — M25.
//!
//! A package is a transparent directory bundle containing exactly a locked
//! `aether.project/v1` manifest and its declared Aether source units. The
//! module deliberately does not resolve versions, fetch networks, admit
//! arbitrary payloads, or grant any guest authority.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::project::{
    parse_project_document, resolve_unit_path, sha256_hex, verify_project, ProjectDocument,
    ProjectError,
};
use crate::{LANGUAGE_NAME, LANGUAGE_VERSION};

/// Strict metadata schema for a transparent M25 directory bundle.
pub const PACKAGE_SCHEMA_VERSION: &str = "aether.package/v1";
/// Metadata file at the root of every package bundle.
pub const PACKAGE_MANIFEST_FILE: &str = "aether.package.json";
/// Fixed directory carrying a materialized project inside a bundle.
pub const PACKAGE_PROJECT_DIRECTORY: &str = "project";
/// Fixed project metadata path inside a bundle.
pub const PACKAGE_PROJECT_PATH: &str = "project/aether.project.json";
/// Deterministic child directory used by a dedicated M25 local cache.
pub const PACKAGE_CACHE_DIRECTORY: &str = "packages";

const PROJECT_MANIFEST_FILE: &str = "aether.project.json";
const MAX_PACKAGE_METADATA_BYTES: usize = 1_000_000;
const MAX_PACKAGE_FILES: usize = 257;
const MAX_PACKAGE_FILE_BYTES: usize = 1_000_000;
const MAX_PACKAGE_TOTAL_BYTES: usize = 16 * 1024 * 1024;
const MAX_CACHE_PACKAGES: usize = 256;
const MAX_PACKAGE_VERSION_BYTES: usize = 64;
const CONTENT_DIGEST_DOMAIN: &[u8] = b"aether.package/v1\0";

/// Closed package-tooling diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageError {
    /// Stable package diagnostic code.
    pub code: &'static str,
    /// Human-readable contextual message.
    pub message: String,
}

impl PackageError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} package error [{}]: {}",
            self.code, self.message
        )
    }
}

impl std::error::Error for PackageError {}

/// One file whose raw bytes are bound by an [`PackageDocument`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageFile {
    /// Forward-slash path rooted at the bundle directory.
    pub path: String,
    /// SHA-256 of the raw file bytes.
    pub sha256: String,
}

/// Canonical metadata for one transparent directory bundle.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageDocument {
    /// Always [`PACKAGE_SCHEMA_VERSION`].
    pub schema: String,
    /// Package/project identity.
    pub name: String,
    /// Exact local package version, not a range.
    pub version: String,
    /// Fixed path to the bundled project manifest.
    pub project: String,
    /// SHA-256 of the raw bundled project manifest.
    pub project_sha256: String,
    /// Domain-separated digest over every bound path and raw file byte sequence.
    pub content_sha256: String,
    /// Sorted, complete list of the project manifest and declared unit files.
    pub files: Vec<PackageFile>,
}

/// Successful package verification evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageVerifyReport {
    /// Verified package identity.
    pub name: String,
    /// Verified exact package version.
    pub version: String,
    /// Domain-separated package content identity.
    pub content_sha256: String,
    /// Number of raw project metadata/source files in the bundle.
    pub file_count: usize,
    /// Sum of all bound raw file bytes.
    pub total_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PackageSourceFile {
    path: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectSnapshot {
    root: PathBuf,
    document: ProjectDocument,
    project_bytes: Vec<u8>,
    files: Vec<PackageSourceFile>,
}

#[derive(Debug, Clone)]
struct VerifiedPackage {
    document: PackageDocument,
    manifest_bytes: Vec<u8>,
    project: ProjectDocument,
    project_bytes: Vec<u8>,
    files: Vec<PackageSourceFile>,
}

/// Parse and structurally validate an `aether.package/v1` document.
pub fn parse_package_document(json: &str) -> Result<PackageDocument, PackageError> {
    if json.len() > MAX_PACKAGE_METADATA_BYTES {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!("package metadata exceeds the {MAX_PACKAGE_METADATA_BYTES}-byte safety limit"),
        ));
    }
    let document: PackageDocument = serde_json::from_str(json).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-001",
            format!("invalid aether.package/v1 JSON: {error}"),
        )
    })?;
    validate_package_document(&document)?;
    Ok(document)
}

/// Render one validated package document as deterministic local JSON.
pub fn serialize_package_document(document: &PackageDocument) -> Result<String, PackageError> {
    validate_package_document(document)?;
    serde_json::to_string_pretty(document)
        .map(|json| format!("{json}\n"))
        .map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-001",
                format!("could not serialize aether.package/v1 document: {error}"),
            )
        })
}

fn validate_package_document(document: &PackageDocument) -> Result<(), PackageError> {
    if document.schema != PACKAGE_SCHEMA_VERSION {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!(
                "unsupported package schema {:?}; expected {PACKAGE_SCHEMA_VERSION}",
                document.schema
            ),
        ));
    }
    if !is_safe_package_name(&document.name) {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            "package name must be a non-empty lowercase ASCII identifier",
        ));
    }
    if !is_safe_package_version(&document.version) {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!(
                "package version must contain 1..={MAX_PACKAGE_VERSION_BYTES} ASCII characters from [A-Za-z0-9._+-] and begin with an alphanumeric character"
            ),
        ));
    }
    if document.project != PACKAGE_PROJECT_PATH {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!(
                "package project path must be exactly {PACKAGE_PROJECT_PATH:?}, got {:?}",
                document.project
            ),
        ));
    }
    if !is_sha256_hex(&document.project_sha256) || !is_sha256_hex(&document.content_sha256) {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            "package project_sha256 and content_sha256 must be 64 lowercase hexadecimal characters",
        ));
    }
    if document.files.len() < 2 || document.files.len() > MAX_PACKAGE_FILES {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!(
                "package files must contain the project manifest plus 1..={} unit file(s)",
                MAX_PACKAGE_FILES - 1
            ),
        ));
    }
    let mut previous = None::<&str>;
    let mut seen = BTreeSet::new();
    for file in &document.files {
        validate_bundle_file_path(&file.path)?;
        if !is_sha256_hex(&file.sha256) {
            return Err(PackageError::new(
                "AE-PACKAGE-001",
                format!(
                    "package file digest for {} must be 64 lowercase hexadecimal characters",
                    file.path
                ),
            ));
        }
        if !seen.insert(file.path.as_str()) {
            return Err(PackageError::new(
                "AE-PACKAGE-001",
                format!("package files lists {} more than once", file.path),
            ));
        }
        if let Some(prior) = previous {
            if prior >= file.path.as_str() {
                return Err(PackageError::new(
                    "AE-PACKAGE-001",
                    "package files must be strictly sorted by forward-slash path",
                ));
            }
        }
        previous = Some(file.path.as_str());
    }
    if document
        .files
        .first()
        .is_none_or(|file| file.path != PACKAGE_PROJECT_PATH)
    {
        return Err(PackageError::new(
            "AE-PACKAGE-001",
            format!("package files must begin with {PACKAGE_PROJECT_PATH}"),
        ));
    }
    Ok(())
}

fn is_safe_package_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_safe_package_version(version: &str) -> bool {
    let bytes = version.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= MAX_PACKAGE_VERSION_BYTES
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn validate_bundle_file_path(path: &str) -> Result<(), PackageError> {
    if path.is_empty()
        || path.contains('\\')
        || path.starts_with('/')
        || path.contains(':')
        || path.contains("//")
        || !path.starts_with("project/")
    {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("package file path {path:?} must be a safe project/ relative path"),
        ));
    }
    for segment in path.split('/') {
        if segment.is_empty()
            || matches!(segment, "." | "..")
            || !segment.bytes().all(
                |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'),
            )
        {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!("package file path {path:?} contains an unsafe segment"),
            ));
        }
    }
    Ok(())
}

fn join_forward_path(root: &Path, path: &str) -> PathBuf {
    let mut joined = root.to_path_buf();
    for segment in path.split('/') {
        joined.push(segment);
    }
    joined
}

fn package_error_from_project(context: &str, error: ProjectError) -> PackageError {
    PackageError::new(
        "AE-PACKAGE-003",
        format!(
            "{context}: nested project verification failed [{}]: {}",
            error.code, error.message
        ),
    )
}

fn canonical_existing_directory(path: &Path, label: &str) -> Result<PathBuf, PackageError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!("could not inspect {label} {}: {error}", path.display()),
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("{label} {} must not be a symlink", path.display()),
        ));
    }
    if !metadata.is_dir() {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("{label} {} must be a directory", path.display()),
        ));
    }
    path.canonicalize().map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!("could not canonicalize {label} {}: {error}", path.display()),
        )
    })
}

fn require_regular_file(path: &Path, label: &str) -> Result<(), PackageError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!("could not inspect {label} {}: {error}", path.display()),
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("{label} {} must not be a symlink", path.display()),
        ));
    }
    if !metadata.is_file() {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("{label} {} must be a regular file", path.display()),
        ));
    }
    Ok(())
}

fn read_bounded_regular_file(
    path: &Path,
    label: &str,
    byte_limit: usize,
) -> Result<Vec<u8>, PackageError> {
    require_regular_file(path, label)?;
    let bytes = fs::read(path).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!("could not read {label} {}: {error}", path.display()),
        )
    })?;
    if bytes.len() > byte_limit {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!(
                "{label} {} exceeds the {byte_limit}-byte safety limit",
                path.display()
            ),
        ));
    }
    Ok(bytes)
}

fn package_content_sha256(files: &[PackageSourceFile]) -> String {
    let mut digest = Sha256::new();
    digest.update(CONTENT_DIGEST_DOMAIN);
    for file in files {
        let path_bytes = file.path.as_bytes();
        let path_length = u32::try_from(path_bytes.len())
            .expect("bounded package path length must fit in u32")
            .to_le_bytes();
        let file_length = u64::try_from(file.bytes.len())
            .expect("bounded package file length must fit in u64")
            .to_le_bytes();
        digest.update(path_length);
        digest.update(path_bytes);
        digest.update(file_length);
        digest.update(&file.bytes);
    }
    hex_digest(&digest.finalize())
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn snapshot_locked_project(project_path: &Path) -> Result<ProjectSnapshot, PackageError> {
    if project_path.file_name().and_then(|name| name.to_str()) != Some(PROJECT_MANIFEST_FILE) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!(
                "package source manifest must be named {PROJECT_MANIFEST_FILE}, got {}",
                project_path.display()
            ),
        ));
    }
    require_regular_file(project_path, "project manifest")?;
    let parent = project_path.parent().ok_or_else(|| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!(
                "project manifest {} has no parent directory",
                project_path.display()
            ),
        )
    })?;
    let root = canonical_existing_directory(parent, "project root")?;
    let resolved_manifest = project_path.canonicalize().map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!(
                "could not canonicalize project manifest {}: {error}",
                project_path.display()
            ),
        )
    })?;
    if !resolved_manifest.starts_with(&root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "project manifest escapes its project root",
        ));
    }
    let project_bytes = read_bounded_regular_file(
        &resolved_manifest,
        "project manifest",
        MAX_PACKAGE_FILE_BYTES,
    )?;
    let project_text = String::from_utf8(project_bytes.clone())
        .map_err(|_| PackageError::new("AE-PACKAGE-001", "project manifest must be valid UTF-8"))?;
    let document = parse_project_document(&project_text)
        .map_err(|error| package_error_from_project("could not parse project manifest", error))?;
    if document.lock.is_none() {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "package source project must carry a complete project lock",
        ));
    }

    let mut files = vec![PackageSourceFile {
        path: PACKAGE_PROJECT_PATH.to_owned(),
        bytes: project_bytes.clone(),
    }];
    let mut units = document.units.clone();
    units.sort_by(|left, right| left.path.cmp(&right.path));
    let mut total_bytes = project_bytes.len();
    for unit in units {
        let source_path = join_forward_path(&root, &unit.path);
        require_regular_file(&source_path, &format!("project unit {}", unit.path))?;
        let resolved = resolve_unit_path(&root, &unit.path).map_err(|error| {
            package_error_from_project(
                &format!("could not resolve project unit {}", unit.path),
                error,
            )
        })?;
        if !resolved.starts_with(&root) {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!("project unit {} escapes the project root", unit.path),
            ));
        }
        let bytes = read_bounded_regular_file(
            &source_path,
            &format!("project unit {}", unit.path),
            MAX_PACKAGE_FILE_BYTES,
        )?;
        total_bytes = total_bytes.checked_add(bytes.len()).ok_or_else(|| {
            PackageError::new("AE-PACKAGE-002", "package source byte count overflowed")
        })?;
        if total_bytes > MAX_PACKAGE_TOTAL_BYTES {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "package source exceeds the {MAX_PACKAGE_TOTAL_BYTES}-byte total safety limit"
                ),
            ));
        }
        files.push(PackageSourceFile {
            path: format!("{PACKAGE_PROJECT_DIRECTORY}/{}", unit.path),
            bytes,
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(ProjectSnapshot {
        root,
        document,
        project_bytes,
        files,
    })
}

fn stable_verified_project_snapshot(project_path: &Path) -> Result<ProjectSnapshot, PackageError> {
    let before = snapshot_locked_project(project_path)?;
    let _ = verify_project(&before.root, &before.document)
        .map_err(|error| package_error_from_project("source project verification", error))?;
    let after = snapshot_locked_project(project_path)?;
    if before != after {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "project manifest or source units changed while package verification was in progress",
        ));
    }
    Ok(before)
}

fn document_from_snapshot(snapshot: &ProjectSnapshot) -> Result<PackageDocument, PackageError> {
    let document = PackageDocument {
        schema: PACKAGE_SCHEMA_VERSION.to_owned(),
        name: snapshot.document.name.clone(),
        version: snapshot.document.version.clone(),
        project: PACKAGE_PROJECT_PATH.to_owned(),
        project_sha256: sha256_hex(&snapshot.project_bytes),
        content_sha256: package_content_sha256(&snapshot.files),
        files: snapshot
            .files
            .iter()
            .map(|file| PackageFile {
                path: file.path.clone(),
                sha256: sha256_hex(&file.bytes),
            })
            .collect(),
    };
    validate_package_document(&document)?;
    Ok(document)
}

fn expected_bundle_paths(project: &ProjectDocument) -> Vec<String> {
    let mut paths = Vec::with_capacity(project.units.len() + 1);
    paths.push(PACKAGE_PROJECT_PATH.to_owned());
    paths.extend(
        project
            .units
            .iter()
            .map(|unit| format!("{PACKAGE_PROJECT_DIRECTORY}/{}", unit.path)),
    );
    paths.sort();
    paths
}

fn resolve_bundle_file(root: &Path, path: &str, label: &str) -> Result<PathBuf, PackageError> {
    validate_bundle_file_path(path)?;
    let candidate = join_forward_path(root, path);
    require_regular_file(&candidate, label)?;
    let resolved = candidate.canonicalize().map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-002",
            format!(
                "could not canonicalize {label} {}: {error}",
                candidate.display()
            ),
        )
    })?;
    if !resolved.starts_with(root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            format!("{label} {path} escapes the package root"),
        ));
    }
    Ok(resolved)
}

fn audit_bundle_tree(root: &Path, expected_files: &BTreeSet<String>) -> Result<(), PackageError> {
    let mut seen = BTreeSet::new();
    audit_bundle_directory(root, root, "", expected_files, &mut seen)?;
    if seen != *expected_files {
        let missing = expected_files
            .difference(&seen)
            .next()
            .expect("sets differ")
            .to_owned();
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            format!("package bundle is missing expected file {missing}"),
        ));
    }
    Ok(())
}

fn audit_bundle_directory(
    root: &Path,
    directory: &Path,
    relative: &str,
    expected_files: &BTreeSet<String>,
    seen: &mut BTreeSet<String>,
) -> Result<(), PackageError> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "could not enumerate package directory {}: {error}",
                    directory.display()
                ),
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "could not enumerate package directory {}: {error}",
                    directory.display()
                ),
            )
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry
            .file_name()
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| {
                PackageError::new(
                    "AE-PACKAGE-002",
                    format!("package path under {} is not valid UTF-8", root.display()),
                )
            })?;
        if !name.bytes().all(
            |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'),
        ) {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!("package path component {name:?} is unsafe"),
            ));
        }
        let child_relative = if relative.is_empty() {
            name
        } else {
            format!("{relative}/{name}")
        };
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-002",
                format!("could not inspect package path {}: {error}", path.display()),
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!("package path {} must not be a symlink", child_relative),
            ));
        }
        if metadata.is_dir() {
            let allowed_directory = child_relative == PACKAGE_PROJECT_DIRECTORY
                || expected_files
                    .iter()
                    .any(|file| file.starts_with(&format!("{child_relative}/")));
            if !allowed_directory {
                return Err(PackageError::new(
                    "AE-PACKAGE-003",
                    format!("package bundle contains unlisted directory {child_relative}"),
                ));
            }
            audit_bundle_directory(root, &path, &child_relative, expected_files, seen)?;
        } else if metadata.is_file() {
            let allowed_manifest = child_relative == PACKAGE_MANIFEST_FILE;
            if !allowed_manifest && !expected_files.contains(&child_relative) {
                return Err(PackageError::new(
                    "AE-PACKAGE-003",
                    format!("package bundle contains unlisted file {child_relative}"),
                ));
            }
            if expected_files.contains(&child_relative) {
                seen.insert(child_relative);
            }
        } else {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "package path {} is not a regular file or directory",
                    path.display()
                ),
            ));
        }
    }
    Ok(())
}

fn verify_package_bundle_inner(bundle_root: &Path) -> Result<VerifiedPackage, PackageError> {
    let root = canonical_existing_directory(bundle_root, "package bundle root")?;
    let manifest_path = root.join(PACKAGE_MANIFEST_FILE);
    let manifest_bytes = read_bounded_regular_file(
        &manifest_path,
        "package metadata",
        MAX_PACKAGE_METADATA_BYTES,
    )?;
    let manifest_text = String::from_utf8(manifest_bytes.clone())
        .map_err(|_| PackageError::new("AE-PACKAGE-001", "package metadata must be valid UTF-8"))?;
    let document = parse_package_document(&manifest_text)?;

    let project_directory = root.join(PACKAGE_PROJECT_DIRECTORY);
    let project_root = canonical_existing_directory(&project_directory, "bundle project root")?;
    if !project_root.starts_with(&root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "bundle project root escapes the package root",
        ));
    }
    let project_path = resolve_bundle_file(&root, &document.project, "bundle project manifest")?;
    let project_bytes = read_bounded_regular_file(
        &project_path,
        "bundle project manifest",
        MAX_PACKAGE_FILE_BYTES,
    )?;
    if sha256_hex(&project_bytes) != document.project_sha256 {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "package project manifest digest does not match project_sha256",
        ));
    }
    let project_text = String::from_utf8(project_bytes.clone()).map_err(|_| {
        PackageError::new(
            "AE-PACKAGE-001",
            "bundle project manifest must be valid UTF-8",
        )
    })?;
    let project = parse_project_document(&project_text).map_err(|error| {
        package_error_from_project("could not parse bundle project manifest", error)
    })?;
    if project.name != document.name || project.version != document.version {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            format!(
                "package metadata identity {}@{} does not match bundled project {}@{}",
                document.name, document.version, project.name, project.version
            ),
        ));
    }
    if project.lock.is_none() {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "bundled project must carry a complete project lock",
        ));
    }

    let expected_paths = expected_bundle_paths(&project);
    let mut files = Vec::with_capacity(expected_paths.len());
    let mut total_bytes = 0usize;
    for path in &expected_paths {
        let source_path = resolve_bundle_file(&root, path, &format!("bundle file {path}"))?;
        let bytes = read_bounded_regular_file(
            &source_path,
            &format!("bundle file {path}"),
            MAX_PACKAGE_FILE_BYTES,
        )?;
        total_bytes = total_bytes.checked_add(bytes.len()).ok_or_else(|| {
            PackageError::new("AE-PACKAGE-002", "package bundle byte count overflowed")
        })?;
        if total_bytes > MAX_PACKAGE_TOTAL_BYTES {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "package bundle exceeds the {MAX_PACKAGE_TOTAL_BYTES}-byte total safety limit"
                ),
            ));
        }
        files.push(PackageSourceFile {
            path: path.clone(),
            bytes,
        });
    }
    let observed_files: Vec<PackageFile> = files
        .iter()
        .map(|file| PackageFile {
            path: file.path.clone(),
            sha256: sha256_hex(&file.bytes),
        })
        .collect();
    if document.files != observed_files {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "package files list does not exactly match the bundled project manifest and unit bytes",
        ));
    }
    if package_content_sha256(&files) != document.content_sha256 {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "package content digest does not match the bound raw file sequence",
        ));
    }
    let expected_set = expected_paths.into_iter().collect();
    audit_bundle_tree(&root, &expected_set)?;
    let _ = verify_project(&project_root, &project)
        .map_err(|error| package_error_from_project("bundled project verification", error))?;

    Ok(VerifiedPackage {
        document,
        manifest_bytes,
        project,
        project_bytes,
        files,
    })
}

fn report_from_verified(verified: &VerifiedPackage) -> PackageVerifyReport {
    PackageVerifyReport {
        name: verified.document.name.clone(),
        version: verified.document.version.clone(),
        content_sha256: verified.document.content_sha256.clone(),
        file_count: verified.files.len(),
        total_bytes: verified.files.iter().map(|file| file.bytes.len()).sum(),
    }
}

/// Verify a package directory before use, publication, or installation.
pub fn verify_package_bundle(bundle_root: &Path) -> Result<PackageVerifyReport, PackageError> {
    let verified = verify_package_bundle_inner(bundle_root)?;
    Ok(report_from_verified(&verified))
}

fn output_parent_and_target(
    output: &Path,
    label: &str,
) -> Result<(PathBuf, PathBuf), PackageError> {
    match fs::symlink_metadata(output) {
        Ok(_) => {
            return Err(PackageError::new(
                "AE-PACKAGE-004",
                format!("{label} output {} already exists", output.display()),
            ));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(PackageError::new(
                "AE-PACKAGE-004",
                format!(
                    "could not inspect {label} output {}: {error}",
                    output.display()
                ),
            ));
        }
    }
    let parent = output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let canonical_parent = canonical_existing_directory(parent, &format!("{label} output parent"))?;
    let name = output.file_name().ok_or_else(|| {
        PackageError::new(
            "AE-PACKAGE-004",
            format!(
                "{label} output {} must name a new directory",
                output.display()
            ),
        )
    })?;
    if name == "." || name == ".." {
        return Err(PackageError::new(
            "AE-PACKAGE-004",
            format!(
                "{label} output {} must name a new directory",
                output.display()
            ),
        ));
    }
    Ok((canonical_parent.clone(), canonical_parent.join(name)))
}

fn create_staging_directory(parent: &Path, purpose: &str) -> Result<PathBuf, PackageError> {
    for attempt in 0..128_u16 {
        let staging = parent.join(format!(
            ".aether-{purpose}-staging-{}-{attempt}",
            std::process::id()
        ));
        match fs::create_dir(&staging) {
            Ok(()) => return Ok(staging),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not create {purpose} staging directory {}: {error}",
                        staging.display()
                    ),
                ));
            }
        }
    }
    Err(PackageError::new(
        "AE-PACKAGE-004",
        format!("could not allocate a unique {purpose} staging directory"),
    ))
}

fn remove_staging_quietly(staging: &Path) {
    let _ = fs::remove_dir_all(staging);
}

fn write_staging_file(root: &Path, relative: &str, bytes: &[u8]) -> Result<(), PackageError> {
    if relative != PACKAGE_MANIFEST_FILE {
        validate_bundle_file_path(relative)?;
    }
    let path = if relative == PACKAGE_MANIFEST_FILE {
        root.join(relative)
    } else {
        join_forward_path(root, relative)
    };
    let parent = path.parent().ok_or_else(|| {
        PackageError::new(
            "AE-PACKAGE-004",
            format!("staging path {} has no parent", path.display()),
        )
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-004",
            format!(
                "could not create staging directory {}: {error}",
                parent.display()
            ),
        )
    })?;
    fs::write(&path, bytes).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-004",
            format!("could not write staging file {}: {error}", path.display()),
        )
    })
}

fn copy_verified_bundle_to_staging(
    verified: &VerifiedPackage,
    staging: &Path,
) -> Result<(), PackageError> {
    write_staging_file(staging, PACKAGE_MANIFEST_FILE, &verified.manifest_bytes)?;
    for file in &verified.files {
        write_staging_file(staging, &file.path, &file.bytes)?;
    }
    Ok(())
}

fn finalize_staging(staging: &Path, output: &Path, label: &str) -> Result<(), PackageError> {
    fs::rename(staging, output).map_err(|error| {
        PackageError::new(
            "AE-PACKAGE-004",
            format!(
                "could not finalize {label} output {}: {error}",
                output.display()
            ),
        )
    })
}

/// Create a new verified directory bundle from one locked project manifest.
pub fn pack_project(
    project_manifest: &Path,
    output_bundle: &Path,
) -> Result<PackageVerifyReport, PackageError> {
    let snapshot = stable_verified_project_snapshot(project_manifest)?;
    let document = document_from_snapshot(&snapshot)?;
    let manifest_bytes = serialize_package_document(&document)?.into_bytes();
    let (parent, output) = output_parent_and_target(output_bundle, "package pack")?;
    let staging = create_staging_directory(&parent, "package-pack")?;
    let result = (|| {
        write_staging_file(&staging, PACKAGE_MANIFEST_FILE, &manifest_bytes)?;
        for file in &snapshot.files {
            write_staging_file(&staging, &file.path, &file.bytes)?;
        }
        let staged = verify_package_bundle_inner(&staging)?;
        if staged.document != document {
            return Err(PackageError::new(
                "AE-PACKAGE-003",
                "staged package metadata differs from the source snapshot",
            ));
        }
        finalize_staging(&staging, &output, "package pack")?;
        verify_package_bundle(&output)
    })();
    if result.is_err() && staging.exists() {
        remove_staging_quietly(&staging);
    }
    result
}

fn ensure_cache_directory(cache_root: &Path) -> Result<PathBuf, PackageError> {
    match fs::symlink_metadata(cache_root) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    format!(
                        "package cache root {} must be a non-symlink directory",
                        cache_root.display()
                    ),
                ));
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(cache_root).map_err(|create_error| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not create package cache root {}: {create_error}",
                        cache_root.display()
                    ),
                )
            })?;
        }
        Err(error) => {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "could not inspect package cache root {}: {error}",
                    cache_root.display()
                ),
            ));
        }
    }
    let root = canonical_existing_directory(cache_root, "package cache root")?;
    let packages = root.join(PACKAGE_CACHE_DIRECTORY);
    match fs::symlink_metadata(&packages) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    format!(
                        "package cache directory {} must be a non-symlink directory",
                        packages.display()
                    ),
                ));
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(&packages).map_err(|create_error| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not create package cache directory {}: {create_error}",
                        packages.display()
                    ),
                )
            })?;
        }
        Err(error) => {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "could not inspect package cache directory {}: {error}",
                    packages.display()
                ),
            ));
        }
    }
    canonical_existing_directory(&packages, "package cache directory")
}

fn cache_entry_path(
    cache_packages: &Path,
    name: &str,
    version: &str,
) -> Result<PathBuf, PackageError> {
    if !is_safe_package_name(name) || !is_safe_package_version(version) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "cache package name/version must use the package metadata path grammar",
        ));
    }
    Ok(cache_packages.join(name).join(version))
}

fn ensure_cache_package_parent(cache_packages: &Path, name: &str) -> Result<PathBuf, PackageError> {
    let parent = cache_packages.join(name);
    match fs::symlink_metadata(&parent) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    format!(
                        "package cache identity directory {} must be a non-symlink directory",
                        parent.display()
                    ),
                ));
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(&parent).map_err(|create_error| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not create package cache identity directory {}: {create_error}",
                        parent.display()
                    ),
                )
            })?;
        }
        Err(error) => {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!(
                    "could not inspect package cache identity directory {}: {error}",
                    parent.display()
                ),
            ));
        }
    }
    canonical_existing_directory(&parent, "package cache identity directory")
}

/// Copy one verified local bundle into its deterministic cache identity.
///
/// An equal existing package is an idempotent no-op. An equal identity with
/// different content is never overwritten.
pub fn publish_package_bundle(
    bundle_root: &Path,
    cache_root: &Path,
) -> Result<PackageVerifyReport, PackageError> {
    let source = verify_package_bundle_inner(bundle_root)?;
    let source_report = report_from_verified(&source);
    let cache_packages = ensure_cache_directory(cache_root)?;
    let output = cache_entry_path(
        &cache_packages,
        &source.document.name,
        &source.document.version,
    )?;
    let parent = ensure_cache_package_parent(&cache_packages, &source.document.name)?;
    match fs::symlink_metadata(&output) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(PackageError::new(
                    "AE-PACKAGE-005",
                    format!(
                        "package cache entry {} is not a non-symlink directory",
                        output.display()
                    ),
                ));
            }
            let existing = verify_package_bundle_inner(&output)?;
            if existing.document.name == source.document.name
                && existing.document.version == source.document.version
                && existing.document.content_sha256 == source.document.content_sha256
            {
                return Ok(report_from_verified(&existing));
            }
            return Err(PackageError::new(
                "AE-PACKAGE-005",
                format!(
                    "package cache already contains conflicting {}@{} at {}",
                    source.document.name,
                    source.document.version,
                    output.display()
                ),
            ));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(PackageError::new(
                "AE-PACKAGE-005",
                format!(
                    "could not inspect package cache entry {}: {error}",
                    output.display()
                ),
            ));
        }
    }
    let staging = create_staging_directory(&parent, "package-publish")?;
    let result = (|| {
        copy_verified_bundle_to_staging(&source, &staging)?;
        let staged = verify_package_bundle_inner(&staging)?;
        if staged.document.content_sha256 != source.document.content_sha256 {
            return Err(PackageError::new(
                "AE-PACKAGE-003",
                "published staging package content does not match the verified source bundle",
            ));
        }
        finalize_staging(&staging, &output, "package publish")?;
        let published = verify_package_bundle_inner(&output)?;
        if published.document.content_sha256 != source_report.content_sha256 {
            return Err(PackageError::new(
                "AE-PACKAGE-003",
                "published package content does not match the verified source bundle",
            ));
        }
        Ok(report_from_verified(&published))
    })();
    if result.is_err() && staging.exists() {
        remove_staging_quietly(&staging);
    }
    result
}

fn verify_installed_project(staging: &Path, source: &VerifiedPackage) -> Result<(), PackageError> {
    let manifest = staging.join(PROJECT_MANIFEST_FILE);
    let installed_project_bytes = read_bounded_regular_file(
        &manifest,
        "installed project manifest",
        MAX_PACKAGE_FILE_BYTES,
    )?;
    if installed_project_bytes != source.project_bytes {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "installed project manifest bytes differ from the verified package bundle",
        ));
    }
    let installed_text = String::from_utf8(installed_project_bytes).map_err(|_| {
        PackageError::new(
            "AE-PACKAGE-003",
            "installed project manifest is not valid UTF-8",
        )
    })?;
    let installed_project = parse_project_document(&installed_text).map_err(|error| {
        package_error_from_project("could not parse installed project manifest", error)
    })?;
    if installed_project != source.project {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            "installed project manifest does not match the verified package project",
        ));
    }
    for file in &source.files {
        let relative = file.path.strip_prefix("project/").ok_or_else(|| {
            PackageError::new(
                "AE-PACKAGE-003",
                format!("verified package file {} lacks project/ prefix", file.path),
            )
        })?;
        let installed = join_forward_path(staging, relative);
        let bytes = read_bounded_regular_file(
            &installed,
            &format!("installed project file {relative}"),
            MAX_PACKAGE_FILE_BYTES,
        )?;
        if bytes != file.bytes {
            return Err(PackageError::new(
                "AE-PACKAGE-003",
                format!("installed project file {relative} differs from the verified package"),
            ));
        }
    }
    let _ = verify_project(staging, &installed_project)
        .map_err(|error| package_error_from_project("installed project verification", error))?;
    Ok(())
}

fn install_verified_package(
    source: &VerifiedPackage,
    output_directory: &Path,
) -> Result<PackageVerifyReport, PackageError> {
    let (parent, output) = output_parent_and_target(output_directory, "package install")?;
    let staging = create_staging_directory(&parent, "package-install")?;
    let result = (|| {
        for file in &source.files {
            let relative = file.path.strip_prefix("project/").ok_or_else(|| {
                PackageError::new(
                    "AE-PACKAGE-003",
                    format!("verified package file {} lacks project/ prefix", file.path),
                )
            })?;
            let path = join_forward_path(&staging, relative);
            let parent = path.parent().ok_or_else(|| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!("installed staging file {} has no parent", path.display()),
                )
            })?;
            fs::create_dir_all(parent).map_err(|error| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not create installed staging directory {}: {error}",
                        parent.display()
                    ),
                )
            })?;
            fs::write(&path, &file.bytes).map_err(|error| {
                PackageError::new(
                    "AE-PACKAGE-004",
                    format!(
                        "could not write installed staging file {}: {error}",
                        path.display()
                    ),
                )
            })?;
        }
        verify_installed_project(&staging, source)?;
        finalize_staging(&staging, &output, "package install")?;
        verify_installed_project(&output, source)?;
        Ok(report_from_verified(source))
    })();
    if result.is_err() && staging.exists() {
        remove_staging_quietly(&staging);
    }
    result
}

/// Materialize a verified package bundle as a workspace-ready project directory.
pub fn install_package_bundle(
    bundle_root: &Path,
    output_directory: &Path,
) -> Result<PackageVerifyReport, PackageError> {
    let source = verify_package_bundle_inner(bundle_root)?;
    install_verified_package(&source, output_directory)
}

fn resolve_cached_bundle(
    cache_root: &Path,
    name: &str,
    version: &str,
) -> Result<PathBuf, PackageError> {
    if !is_safe_package_name(name) || !is_safe_package_version(version) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "cache package name/version must use the package metadata path grammar",
        ));
    }
    let root = canonical_existing_directory(cache_root, "package cache root")?;
    let packages = root.join(PACKAGE_CACHE_DIRECTORY);
    let package_root = canonical_existing_directory(&packages, "package cache directory")?;
    if !package_root.starts_with(&root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "package cache directory escapes its cache root",
        ));
    }
    let name_root = package_root.join(name);
    let name_root = canonical_existing_directory(&name_root, "package cache identity directory")?;
    if !name_root.starts_with(&package_root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "package cache identity directory escapes the cache directory",
        ));
    }
    let entry = name_root.join(version);
    let entry = canonical_existing_directory(&entry, "package cache entry")?;
    if !entry.starts_with(&name_root) {
        return Err(PackageError::new(
            "AE-PACKAGE-002",
            "package cache entry escapes its identity directory",
        ));
    }
    Ok(entry)
}

/// Materialize one verified package from an exact local cache identity.
pub fn install_package_from_cache(
    cache_root: &Path,
    name: &str,
    version: &str,
    output_directory: &Path,
) -> Result<PackageVerifyReport, PackageError> {
    let bundle = resolve_cached_bundle(cache_root, name, version)?;
    let source = verify_package_bundle_inner(&bundle)?;
    if source.document.name != name || source.document.version != version {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            format!(
                "cache entry {} does not contain requested package {name}@{version}",
                bundle.display()
            ),
        ));
    }
    install_verified_package(&source, output_directory)
}

fn sorted_directory_entries(directory: &Path) -> Result<Vec<fs::DirEntry>, PackageError> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-002",
                format!("could not enumerate {}: {error}", directory.display()),
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            PackageError::new(
                "AE-PACKAGE-002",
                format!("could not enumerate {}: {error}", directory.display()),
            )
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    Ok(entries)
}

/// Verify every exact package identity in a dedicated local M25 cache.
pub fn verify_package_cache(cache_root: &Path) -> Result<Vec<PackageVerifyReport>, PackageError> {
    let root = canonical_existing_directory(cache_root, "package cache root")?;
    let root_entries = sorted_directory_entries(&root)?;
    if root_entries.len() != 1 || root_entries[0].file_name() != PACKAGE_CACHE_DIRECTORY {
        return Err(PackageError::new(
            "AE-PACKAGE-003",
            format!(
                "package cache root {} must contain only the {PACKAGE_CACHE_DIRECTORY}/ directory",
                root.display()
            ),
        ));
    }
    let cache_directory = root.join(PACKAGE_CACHE_DIRECTORY);
    let cache_directory =
        canonical_existing_directory(&cache_directory, "package cache directory")?;
    let mut reports = Vec::new();
    for name_entry in sorted_directory_entries(&cache_directory)? {
        let name = name_entry
            .file_name()
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| {
                PackageError::new(
                    "AE-PACKAGE-002",
                    "package cache identity name is not valid UTF-8",
                )
            })?;
        if !is_safe_package_name(&name) {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                format!("package cache identity name {name:?} is unsafe"),
            ));
        }
        let name_path =
            canonical_existing_directory(&name_entry.path(), "package cache identity directory")?;
        if !name_path.starts_with(&cache_directory) {
            return Err(PackageError::new(
                "AE-PACKAGE-002",
                "package cache identity directory escapes the cache root",
            ));
        }
        for version_entry in sorted_directory_entries(&name_path)? {
            if reports.len() >= MAX_CACHE_PACKAGES {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    format!("package cache exceeds the {MAX_CACHE_PACKAGES}-package safety limit"),
                ));
            }
            let version = version_entry
                .file_name()
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| {
                    PackageError::new("AE-PACKAGE-002", "package cache version is not valid UTF-8")
                })?;
            if !is_safe_package_version(&version) {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    format!("package cache version {version:?} is unsafe"),
                ));
            }
            let bundle =
                canonical_existing_directory(&version_entry.path(), "package cache entry")?;
            if !bundle.starts_with(&name_path) {
                return Err(PackageError::new(
                    "AE-PACKAGE-002",
                    "package cache entry escapes the cache identity directory",
                ));
            }
            let verified = verify_package_bundle_inner(&bundle)?;
            if verified.document.name != name || verified.document.version != version {
                return Err(PackageError::new(
                    "AE-PACKAGE-003",
                    format!(
                        "package cache path {name}/{version} does not match bundle identity {}@{}",
                        verified.document.name, verified.document.version
                    ),
                ));
            }
            reports.push(report_from_verified(&verified));
        }
    }
    Ok(reports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{
        refresh_project_lock, serialize_project_document, ProjectUnit, ProjectUnitRole,
    };
    use crate::workspace::{
        compile_workspace_package, parse_workspace_document, refresh_workspace_lock,
        verify_workspace,
    };
    use crate::{
        run_bytecode, ProjectDocument, WorkspaceDocument, WorkspacePackage,
        WORKSPACE_SCHEMA_VERSION,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn create() -> Self {
            let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("aether-package-{}-{sequence}", std::process::id()));
            fs::create_dir_all(&path).expect("temp dir");
            Self { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_locked_project(root: &Path, name: &str, version: &str, main: &str, lib: Option<&str>) {
        fs::create_dir_all(root).expect("project root");
        fs::write(root.join("main.ae"), main).expect("main source");
        let mut units = vec![ProjectUnit {
            path: "main.ae".to_owned(),
            role: ProjectUnitRole::Main,
        }];
        if let Some(source) = lib {
            fs::create_dir_all(root.join("lib")).expect("lib root");
            fs::write(root.join("lib/math.ae"), source).expect("lib source");
            units.push(ProjectUnit {
                path: "lib/math.ae".to_owned(),
                role: ProjectUnitRole::Lib,
            });
        }
        let project = ProjectDocument {
            schema: crate::PROJECT_SCHEMA_VERSION.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
            units,
            lock: None,
        };
        let locked = refresh_project_lock(root, &project).expect("refresh project lock");
        fs::write(
            root.join(PROJECT_MANIFEST_FILE),
            serialize_project_document(&locked).expect("serialize project"),
        )
        .expect("project manifest");
    }

    fn source_main(world: &str, value: i64) -> String {
        format!("world {world}\n\nweave main [] -> Whole:\n  yield {value}\n")
    }

    #[test]
    fn package_pack_verify_publish_install_is_deterministic_and_workspace_consumable() {
        let temp = TempDir::create();
        let source = temp.path.join("source");
        write_locked_project(
            &source,
            "math_pkg",
            "0.1.0",
            &source_main("math_pkg", 0),
            Some("world math\n\nexport weave double [value: Whole] -> Whole:\n  yield product value 2\n"),
        );

        let bundle_one = temp.path.join("bundle-one");
        let bundle_two = temp.path.join("bundle-two");
        let one = pack_project(&source.join(PROJECT_MANIFEST_FILE), &bundle_one).expect("pack one");
        let two = pack_project(&source.join(PROJECT_MANIFEST_FILE), &bundle_two).expect("pack two");
        assert_eq!(one, two, "repeated pack must retain one content identity");
        assert_eq!(verify_package_bundle(&bundle_one).expect("verify one"), one);

        let cache = temp.path.join("cache");
        assert_eq!(
            publish_package_bundle(&bundle_one, &cache).expect("publish"),
            one
        );
        assert_eq!(
            publish_package_bundle(&bundle_one, &cache).expect("idempotent publish"),
            one
        );
        assert_eq!(
            verify_package_cache(&cache).expect("verify cache"),
            vec![one.clone()]
        );

        let direct = temp.path.join("direct-install");
        assert_eq!(
            install_package_bundle(&bundle_one, &direct).expect("direct install"),
            one
        );
        let workspace_root = temp.path.join("workspace");
        fs::create_dir_all(&workspace_root).expect("workspace root");
        let installed_math = workspace_root.join("math");
        assert_eq!(
            install_package_from_cache(&cache, "math_pkg", "0.1.0", &installed_math)
                .expect("cache install"),
            one
        );
        let direct_project = fs::read(direct.join(PROJECT_MANIFEST_FILE)).expect("direct project");
        assert_eq!(
            direct_project,
            fs::read(source.join(PROJECT_MANIFEST_FILE)).expect("source project"),
            "install must retain the exact locked project manifest"
        );

        let consumer_one = workspace_root.join("consumer_one");
        let consumer_two = workspace_root.join("consumer_two");
        write_locked_project(
            &consumer_one,
            "consumer_one",
            "0.1.0",
            "world consumer_one\n\nimport unit \"lib/math.ae\" from package math as math\n\nweave main [] -> Whole:\n  yield call math.double 11\n",
            None,
        );
        write_locked_project(
            &consumer_two,
            "consumer_two",
            "0.1.0",
            "world consumer_two\n\nimport unit \"lib/math.ae\" from package math as math\n\nweave main [] -> Whole:\n  yield call math.double 12\n",
            None,
        );
        let workspace = WorkspaceDocument {
            schema: WORKSPACE_SCHEMA_VERSION.to_owned(),
            name: "published_reuse".to_owned(),
            version: "0.1.0".to_owned(),
            packages: vec![
                WorkspacePackage {
                    name: "math".to_owned(),
                    path: "math".to_owned(),
                    depends_on: Vec::new(),
                },
                WorkspacePackage {
                    name: "consumer_one".to_owned(),
                    path: "consumer_one".to_owned(),
                    depends_on: vec!["math".to_owned()],
                },
                WorkspacePackage {
                    name: "consumer_two".to_owned(),
                    path: "consumer_two".to_owned(),
                    depends_on: vec!["math".to_owned()],
                },
            ],
            lock: None,
        };
        let workspace =
            refresh_workspace_lock(&workspace_root, &workspace).expect("workspace lock");
        verify_workspace(&workspace_root, &workspace).expect("workspace verify");
        for (consumer, expected_exit) in [("consumer_one", 22), ("consumer_two", 24)] {
            let bytecode = compile_workspace_package(&workspace_root, &workspace, consumer)
                .expect("workspace build");
            assert_eq!(
                run_bytecode(&bytecode).expect("workspace run").exit_code,
                expected_exit
            );
        }
        let workspace_json = serde_json::to_string(&workspace).expect("workspace JSON");
        parse_workspace_document(&workspace_json).expect("workspace schema round trip");
    }

    #[test]
    fn package_rejects_stale_lock_tampering_unlisted_files_and_existing_output() {
        let temp = TempDir::create();
        let source = temp.path.join("source");
        write_locked_project(
            &source,
            "lock_pkg",
            "0.1.0",
            &source_main("lock_pkg", 1),
            None,
        );
        fs::write(source.join("main.ae"), source_main("lock_pkg", 2)).expect("stale source");
        let stale = pack_project(
            &source.join(PROJECT_MANIFEST_FILE),
            &temp.path.join("stale"),
        )
        .expect_err("stale project lock must reject pack");
        assert_eq!(stale.code, "AE-PACKAGE-003");

        write_locked_project(
            &source,
            "lock_pkg",
            "0.1.0",
            &source_main("lock_pkg", 1),
            None,
        );
        let bundle = temp.path.join("bundle");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &bundle).expect("fresh pack");
        fs::write(bundle.join("project/main.ae"), source_main("lock_pkg", 9)).expect("tamper unit");
        let tampered =
            verify_package_bundle(&bundle).expect_err("tampered unit must reject verify");
        assert_eq!(tampered.code, "AE-PACKAGE-003");

        let fresh = temp.path.join("fresh");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &fresh).expect("fresh bundle");
        fs::write(
            fresh.join("project/unlisted.ae"),
            source_main("unlisted", 0),
        )
        .expect("unlisted");
        let unlisted = verify_package_bundle(&fresh).expect_err("unlisted file must reject verify");
        assert_eq!(unlisted.code, "AE-PACKAGE-003");

        let installable = temp.path.join("installable");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &installable)
            .expect("fresh installable bundle");

        let existing = temp.path.join("existing-output");
        fs::create_dir_all(&existing).expect("existing output");
        fs::write(existing.join("keep.txt"), b"preserve").expect("sentinel");
        let output = pack_project(&source.join(PROJECT_MANIFEST_FILE), &existing)
            .expect_err("existing output must reject pack");
        assert_eq!(output.code, "AE-PACKAGE-004");
        assert_eq!(
            fs::read(existing.join("keep.txt")).expect("sentinel"),
            b"preserve"
        );

        let existing_install = temp.path.join("existing-install");
        fs::create_dir_all(&existing_install).expect("existing install output");
        fs::write(existing_install.join("keep.txt"), b"preserve").expect("install sentinel");
        let install = install_package_bundle(&installable, &existing_install)
            .expect_err("existing install output must reject");
        assert_eq!(install.code, "AE-PACKAGE-004");
        assert_eq!(
            fs::read(existing_install.join("keep.txt")).expect("install sentinel"),
            b"preserve"
        );

        let oversized = temp.path.join("oversized");
        write_locked_project(
            &oversized,
            "oversized_pkg",
            "0.1.0",
            &source_main("oversized_pkg", 0),
            None,
        );
        fs::write(
            oversized.join("main.ae"),
            vec![b'x'; MAX_PACKAGE_FILE_BYTES + 1],
        )
        .expect("oversized source");
        let oversized = pack_project(
            &oversized.join(PROJECT_MANIFEST_FILE),
            &temp.path.join("oversized-bundle"),
        )
        .expect_err("oversized source file must reject pack");
        assert_eq!(oversized.code, "AE-PACKAGE-002");
    }

    #[test]
    fn package_rejects_cache_collision_and_malformed_cache_entry() {
        let temp = TempDir::create();
        let first = temp.path.join("first");
        let second = temp.path.join("second");
        write_locked_project(
            &first,
            "collision_pkg",
            "0.1.0",
            &source_main("first", 1),
            None,
        );
        write_locked_project(
            &second,
            "collision_pkg",
            "0.1.0",
            &source_main("second", 2),
            None,
        );
        let bundle_one = temp.path.join("bundle-one");
        let bundle_two = temp.path.join("bundle-two");
        pack_project(&first.join(PROJECT_MANIFEST_FILE), &bundle_one).expect("first pack");
        pack_project(&second.join(PROJECT_MANIFEST_FILE), &bundle_two).expect("second pack");
        let cache = temp.path.join("cache");
        let first_report = publish_package_bundle(&bundle_one, &cache).expect("first publish");
        let collision = publish_package_bundle(&bundle_two, &cache)
            .expect_err("same cache identity with different content must reject");
        assert_eq!(collision.code, "AE-PACKAGE-005");
        assert_eq!(
            verify_package_cache(&cache).expect("cache remains valid"),
            vec![first_report]
        );

        fs::create_dir_all(
            cache
                .join(PACKAGE_CACHE_DIRECTORY)
                .join("bad")
                .join("0.1.0"),
        )
        .expect("malformed cache entry");
        let malformed =
            verify_package_cache(&cache).expect_err("malformed cache entry must reject");
        assert!(matches!(
            malformed.code,
            "AE-PACKAGE-001" | "AE-PACKAGE-002" | "AE-PACKAGE-003"
        ));
    }

    #[test]
    fn package_rejects_missing_and_incomplete_source_locks() {
        let temp = TempDir::create();
        let missing = temp.path.join("missing-lock");
        write_locked_project(
            &missing,
            "missing_lock_pkg",
            "0.1.0",
            &source_main("missing_lock_pkg", 0),
            None,
        );
        let manifest = missing.join(PROJECT_MANIFEST_FILE);
        let mut document = parse_project_document(
            &fs::read_to_string(&manifest).expect("locked project manifest"),
        )
        .expect("parse locked project");
        document.lock = None;
        fs::write(
            &manifest,
            serialize_project_document(&document).expect("serialize missing lock"),
        )
        .expect("write missing lock");
        let missing_lock = pack_project(&manifest, &temp.path.join("missing-lock.bundle"))
            .expect_err("source without a lock must reject package pack");
        assert_eq!(missing_lock.code, "AE-PACKAGE-003");

        let incomplete = temp.path.join("incomplete-lock");
        write_locked_project(
            &incomplete,
            "incomplete_lock_pkg",
            "0.1.0",
            &source_main("incomplete_lock_pkg", 0),
            Some("world math\n\nexport weave double [value: Whole] -> Whole:\n  yield product value 2\n"),
        );
        let manifest = incomplete.join(PROJECT_MANIFEST_FILE);
        let mut document = parse_project_document(
            &fs::read_to_string(&manifest).expect("locked project manifest"),
        )
        .expect("parse locked project");
        document.lock.as_mut().expect("lock present").units.pop();
        fs::write(
            &manifest,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&document)
                    .expect("serialize intentionally incomplete lock fixture")
            ),
        )
        .expect("write incomplete lock");
        let incomplete_lock = pack_project(&manifest, &temp.path.join("incomplete-lock.bundle"))
            .expect_err("source with an incomplete lock must reject package pack");
        assert_eq!(incomplete_lock.code, "AE-PACKAGE-003");
    }

    #[test]
    fn package_rejects_unsafe_metadata_and_nonregular_source_input() {
        let temp = TempDir::create();
        let source = temp.path.join("source");
        write_locked_project(
            &source,
            "metadata_pkg",
            "0.1.0",
            &source_main("metadata_pkg", 0),
            None,
        );

        let traversal = temp.path.join("traversal");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &traversal).expect("traversal pack");
        let manifest = traversal.join(PACKAGE_MANIFEST_FILE);
        let raw = fs::read_to_string(&manifest).expect("package metadata");
        for (label, invalid) in [
            (
                "schema",
                raw.replacen("aether.package/v1", "aether.package/v2", 1),
            ),
            (
                "name",
                raw.replacen("\"name\": \"metadata_pkg\"", "\"name\": \"Metadata\"", 1),
            ),
            (
                "version",
                raw.replacen("\"version\": \"0.1.0\"", "\"version\": \"../escape\"", 1),
            ),
            (
                "unknown field",
                raw.replacen("\n}\n", ",\n  \"unexpected\": true\n}\n", 1),
            ),
        ] {
            let error = parse_package_document(&invalid)
                .expect_err("malformed package {label} metadata must reject");
            assert_eq!(error.code, "AE-PACKAGE-001", "unexpected {label} error");
        }
        let unsafe_metadata = raw.replacen("project/main.ae", "project/../main.ae", 1);
        assert_ne!(unsafe_metadata, raw, "fixture must alter a package path");
        fs::write(&manifest, unsafe_metadata).expect("unsafe package metadata");
        let traversal_error =
            verify_package_bundle(&traversal).expect_err("traversal metadata must reject");
        assert_eq!(traversal_error.code, "AE-PACKAGE-002");

        let digest = temp.path.join("digest");
        let report =
            pack_project(&source.join(PROJECT_MANIFEST_FILE), &digest).expect("digest pack");
        let manifest = digest.join(PACKAGE_MANIFEST_FILE);
        let raw = fs::read_to_string(&manifest).expect("package metadata");
        let wrong_digest = "0".repeat(64);
        assert_ne!(
            wrong_digest, report.content_sha256,
            "fixture must change digest"
        );
        let tampered_metadata = raw.replacen(&report.content_sha256, &wrong_digest, 1);
        assert_ne!(tampered_metadata, raw, "fixture must alter content digest");
        fs::write(&manifest, tampered_metadata).expect("tampered package metadata");
        let digest_error =
            verify_package_bundle(&digest).expect_err("content digest mismatch must reject");
        assert_eq!(digest_error.code, "AE-PACKAGE-003");

        let file_digest = temp.path.join("file-digest");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &file_digest).expect("file digest pack");
        let manifest = file_digest.join(PACKAGE_MANIFEST_FILE);
        let mut document =
            parse_package_document(&fs::read_to_string(&manifest).expect("package metadata"))
                .expect("parse package metadata");
        document.files[1].sha256 = "0".repeat(64);
        fs::write(
            &manifest,
            serialize_package_document(&document).expect("serialize altered file digest"),
        )
        .expect("write altered file digest");
        let file_digest_error =
            verify_package_bundle(&file_digest).expect_err("file digest mismatch must reject");
        assert_eq!(file_digest_error.code, "AE-PACKAGE-003");

        let project_manifest = temp.path.join("project-manifest");
        pack_project(&source.join(PROJECT_MANIFEST_FILE), &project_manifest)
            .expect("project manifest pack");
        let bundled_project = project_manifest.join(PACKAGE_PROJECT_PATH);
        let project_bytes = fs::read(&bundled_project).expect("bundled project bytes");
        fs::write(&bundled_project, [project_bytes, vec![b'\n']].concat())
            .expect("alter bundled project manifest");
        let project_manifest_error = verify_package_bundle(&project_manifest)
            .expect_err("project manifest digest mismatch must reject");
        assert_eq!(project_manifest_error.code, "AE-PACKAGE-003");

        let nonregular = temp.path.join("nonregular");
        write_locked_project(
            &nonregular,
            "nonregular_pkg",
            "0.1.0",
            &source_main("nonregular_pkg", 0),
            None,
        );
        fs::remove_file(nonregular.join("main.ae")).expect("remove source file");
        fs::create_dir(nonregular.join("main.ae")).expect("replace source with directory");
        let nonregular_error = pack_project(
            &nonregular.join(PROJECT_MANIFEST_FILE),
            &temp.path.join("nonregular.bundle"),
        )
        .expect_err("nonregular source unit must reject package pack");
        assert_eq!(nonregular_error.code, "AE-PACKAGE-002");
    }

    #[cfg(unix)]
    fn create_file_symlink(target: &Path, link: &Path) -> io::Result<()> {
        std::os::unix::fs::symlink(target, link)
    }

    #[cfg(windows)]
    fn create_file_symlink(target: &Path, link: &Path) -> io::Result<()> {
        std::os::windows::fs::symlink_file(target, link)
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn package_rejects_symlinked_source_units_when_the_platform_allows_test_symlinks() {
        let temp = TempDir::create();
        let source = temp.path.join("source");
        write_locked_project(
            &source,
            "symlink_pkg",
            "0.1.0",
            &source_main("symlink_pkg", 0),
            None,
        );
        let source_file = source.join("main.ae");
        let target = source.join("main.real.ae");
        fs::rename(&source_file, &target).expect("move source target");
        match create_file_symlink(&target, &source_file) {
            Ok(()) => {
                let error = pack_project(
                    &source.join(PROJECT_MANIFEST_FILE),
                    &temp.path.join("symlink.bundle"),
                )
                .expect_err("symlinked source unit must reject package pack");
                assert_eq!(error.code, "AE-PACKAGE-002");
            }
            Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
                // Windows test environments can deny symbolic-link creation even though
                // production input checks are still enforced by require_regular_file.
            }
            Err(error) => panic!("could not create test symlink: {error}"),
        }
    }
}
