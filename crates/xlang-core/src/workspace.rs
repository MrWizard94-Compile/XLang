//! Offline multi-package workspaces (`aether.workspace/v1`) — M18 / M22.
//!
//! A workspace lists local package directories, each containing
//! `aether.project.json`. Optional `depends_on` edges form an acyclic graph for
//! verification order and authorize M22 `import unit … from package`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::modules::compile_project_modules_with_packages;
use crate::project::{
    parse_project_document, sha256_hex, verify_project, ProjectDocument, ProjectError,
    ProjectVerifyReport,
};
use crate::{LANGUAGE_NAME, LANGUAGE_VERSION};

pub const WORKSPACE_SCHEMA_VERSION: &str = "aether.workspace/v1";
pub const WORKSPACE_PROJECT_FILE: &str = "aether.project.json";
const MAX_WORKSPACE_PACKAGES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceError {
    pub code: &'static str,
    pub message: String,
}

impl WorkspaceError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{LANGUAGE_NAME} {LANGUAGE_VERSION} workspace error [{}]: {}",
            self.code, self.message
        )
    }
}

impl std::error::Error for WorkspaceError {}

impl From<ProjectError> for WorkspaceError {
    fn from(error: ProjectError) -> Self {
        Self::new(
            "AE-WORKSPACE-004",
            format!(
                "nested project verify failed [{}]: {}",
                error.code, error.message
            ),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceDocument {
    pub schema: String,
    pub name: String,
    pub version: String,
    pub packages: Vec<WorkspacePackage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<WorkspaceLock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

/// Complete local package identity pins for one `aether.workspace/v1` document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceLock {
    pub packages: Vec<WorkspaceLockPackage>,
}

/// One package's project-manifest identity inside a [`WorkspaceLock`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceLockPackage {
    pub name: String,
    pub path: String,
    pub project_name: String,
    pub project_version: String,
    pub project_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePackageReport {
    pub name: String,
    pub path: String,
    pub project: ProjectVerifyReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceVerifyReport {
    pub name: String,
    pub version: String,
    pub packages: Vec<WorkspacePackageReport>,
}

/// Parse and structurally validate an `aether.workspace/v1` JSON document.
pub fn parse_workspace_document(json: &str) -> Result<WorkspaceDocument, WorkspaceError> {
    if json.len() > 1_000_000 {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            "workspace document exceeds the 1_000_000-byte safety limit",
        ));
    }
    let document: WorkspaceDocument = serde_json::from_str(json).map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-001",
            format!("invalid aether.workspace/v1 JSON: {error}"),
        )
    })?;
    validate_workspace_document(&document)?;
    Ok(document)
}

fn validate_workspace_document(document: &WorkspaceDocument) -> Result<(), WorkspaceError> {
    if document.schema != WORKSPACE_SCHEMA_VERSION {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            format!(
                "unsupported workspace schema {:?}; expected {WORKSPACE_SCHEMA_VERSION}",
                document.schema
            ),
        ));
    }
    if document.name.is_empty() || document.version.is_empty() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            "workspace name and version must be non-empty",
        ));
    }
    if !is_safe_workspace_ident(&document.name) {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            "workspace name must match [A-Za-z][A-Za-z0-9_]*",
        ));
    }
    if document.packages.is_empty() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            "workspace packages array must contain at least one package",
        ));
    }
    if document.packages.len() > MAX_WORKSPACE_PACKAGES {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-001",
            format!("workspace packages array exceeds the {MAX_WORKSPACE_PACKAGES}-package safety limit"),
        ));
    }

    let mut names = BTreeSet::new();
    for package in &document.packages {
        if !is_safe_workspace_ident(&package.name) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-002",
                format!(
                    "package name {:?} must match [A-Za-z][A-Za-z0-9_]*",
                    package.name
                ),
            ));
        }
        if !names.insert(package.name.clone()) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-003",
                format!("duplicate package name {}", package.name),
            ));
        }
        validate_package_dir_path(&package.path)?;
        let mut dep_seen = BTreeSet::new();
        for dep in &package.depends_on {
            if dep == &package.name {
                return Err(WorkspaceError::new(
                    "AE-WORKSPACE-003",
                    format!("package {} cannot depend on itself", package.name),
                ));
            }
            if !dep_seen.insert(dep.clone()) {
                return Err(WorkspaceError::new(
                    "AE-WORKSPACE-003",
                    format!(
                        "package {} lists dependency {dep} more than once",
                        package.name
                    ),
                ));
            }
        }
    }
    for package in &document.packages {
        for dep in &package.depends_on {
            if !names.contains(dep) {
                return Err(WorkspaceError::new(
                    "AE-WORKSPACE-003",
                    format!("package {} depends_on unknown package {dep}", package.name),
                ));
            }
        }
    }
    let _ = topological_package_order(document)?;
    if let Some(lock) = &document.lock {
        validate_workspace_lock(document, lock)?;
    }
    Ok(())
}

fn validate_workspace_lock(
    document: &WorkspaceDocument,
    lock: &WorkspaceLock,
) -> Result<(), WorkspaceError> {
    if lock.packages.len() != document.packages.len() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-005",
            "workspace lock must list every workspace package exactly once",
        ));
    }
    let declared: BTreeMap<&str, &WorkspacePackage> = document
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let mut seen = BTreeSet::new();
    for entry in &lock.packages {
        if !seen.insert(entry.name.as_str()) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!("workspace lock lists package {} more than once", entry.name),
            ));
        }
        let package = declared.get(entry.name.as_str()).ok_or_else(|| {
            WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!("workspace lock names unknown package {}", entry.name),
            )
        })?;
        if entry.path != package.path {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!(
                    "workspace lock path {} for package {} does not match declared path {}",
                    entry.path, entry.name, package.path
                ),
            ));
        }
        if !is_safe_project_ident(&entry.project_name) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!(
                    "workspace lock project_name for package {} must be a lowercase ASCII identifier",
                    entry.name
                ),
            ));
        }
        if entry.project_version.is_empty() {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!(
                    "workspace lock project_version for package {} must be non-empty",
                    entry.name
                ),
            ));
        }
        if !is_sha256_hex(&entry.project_sha256) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!(
                    "workspace lock project_sha256 for package {} must be 64 lowercase hex characters",
                    entry.name
                ),
            ));
        }
    }
    for package in &document.packages {
        if !seen.contains(package.name.as_str()) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!("workspace lock is missing package {}", package.name),
            ));
        }
    }
    Ok(())
}

fn is_safe_workspace_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => {
            chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        }
        _ => false,
    }
}

fn is_safe_project_ident(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn validate_package_dir_path(path: &str) -> Result<(), WorkspaceError> {
    if path.is_empty() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-002",
            "package path must be non-empty",
        ));
    }
    if path.contains('\\') || path.starts_with('/') || path.contains(':') || path.contains("//") {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-002",
            format!("package path {path} must be relative with '/' separators only"),
        ));
    }
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-002",
                format!("package path {path} escapes the workspace root or has empty segments"),
            ));
        }
        if !segment.bytes().all(
            |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'),
        ) {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-002",
                format!("package path {path} contains an illegal segment"),
            ));
        }
    }
    Ok(())
}

fn join_rel(root: &Path, rel: &str) -> PathBuf {
    let mut joined = root.to_path_buf();
    for segment in rel.split('/') {
        joined.push(segment);
    }
    joined
}

/// Resolve a package directory under the workspace root.
pub fn resolve_package_path(
    workspace_root: &Path,
    package_path: &str,
) -> Result<PathBuf, WorkspaceError> {
    validate_package_dir_path(package_path)?;
    let root = workspace_root.canonicalize().map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-002",
            format!(
                "could not resolve workspace root {}: {error}",
                workspace_root.display()
            ),
        )
    })?;
    let joined = join_rel(&root, package_path);
    let resolved = joined.canonicalize().map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-002",
            format!("could not resolve package path {package_path}: {error}"),
        )
    })?;
    if !resolved.starts_with(&root) {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-002",
            format!("package path {package_path} escapes the workspace root"),
        ));
    }
    if !resolved.is_dir() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-002",
            format!("package path {package_path} is not a directory"),
        ));
    }
    Ok(resolved)
}

struct LoadedWorkspaceProject {
    package_root: PathBuf,
    project_bytes: Vec<u8>,
    project: ProjectDocument,
}

fn resolve_workspace_project_file(
    package_root: &Path,
    package: &WorkspacePackage,
) -> Result<PathBuf, WorkspaceError> {
    let candidate = package_root.join(WORKSPACE_PROJECT_FILE);
    let resolved = candidate.canonicalize().map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "package {} path {} is missing {WORKSPACE_PROJECT_FILE}: {error}",
                package.name, package.path
            ),
        )
    })?;
    require_project_manifest_within_package_root(package_root, &resolved, package)?;
    if !resolved.is_file() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "package {} path {} is missing {WORKSPACE_PROJECT_FILE}",
                package.name, package.path
            ),
        ));
    }
    Ok(resolved)
}

/// Enforce post-canonicalization containment so a manifest symlink cannot make
/// a declared package read an `aether.project.json` outside that package.
fn require_project_manifest_within_package_root(
    package_root: &Path,
    resolved: &Path,
    package: &WorkspacePackage,
) -> Result<(), WorkspaceError> {
    if resolved.starts_with(package_root) {
        return Ok(());
    }
    Err(WorkspaceError::new(
        "AE-WORKSPACE-002",
        format!(
            "package {} project manifest escapes package root",
            package.name
        ),
    ))
}

fn load_workspace_project(
    workspace_root: &Path,
    package: &WorkspacePackage,
) -> Result<LoadedWorkspaceProject, WorkspaceError> {
    let package_root = resolve_package_path(workspace_root, &package.path)?;
    let project_file = resolve_workspace_project_file(&package_root, package)?;
    let project_bytes = fs::read(&project_file).map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "could not read {} for package {}: {error}",
                project_file.display(),
                package.name
            ),
        )
    })?;
    let project_text = String::from_utf8(project_bytes.clone()).map_err(|_| {
        WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "could not read {} for package {}: project manifest is not valid UTF-8",
                project_file.display(),
                package.name
            ),
        )
    })?;
    let project = parse_project_document(&project_text)?;
    Ok(LoadedWorkspaceProject {
        package_root,
        project_bytes,
        project,
    })
}

fn verify_workspace_lock_entry(
    package: &WorkspacePackage,
    entry: &WorkspaceLockPackage,
    loaded: &LoadedWorkspaceProject,
) -> Result<(), WorkspaceError> {
    let observed_digest = sha256_hex(&loaded.project_bytes);
    if entry.project_sha256 != observed_digest {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-005",
            format!(
                "workspace lock project manifest digest mismatch for package {}: expected {}, observed {}",
                package.name,
                entry.project_sha256,
                observed_digest
            ),
        ));
    }
    if entry.project_name != loaded.project.name {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-005",
            format!(
                "workspace lock project name mismatch for package {}: expected {}, observed {}",
                package.name, entry.project_name, loaded.project.name
            ),
        ));
    }
    if entry.project_version != loaded.project.version {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-005",
            format!(
                "workspace lock project version mismatch for package {}: expected {}, observed {}",
                package.name, entry.project_version, loaded.project.version
            ),
        ));
    }
    if loaded.project.lock.is_none() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-005",
            format!(
                "workspace lock requires package {} project manifest to carry a complete project lock",
                package.name
            ),
        ));
    }
    Ok(())
}

/// Refresh all package identity entries for a workspace without writing files.
///
/// Package projects must already carry complete project unit locks. This keeps
/// workspace refresh a one-document operation: callers explicitly refresh
/// project documents first, then write the returned workspace document only if
/// they requested that mutation.
pub fn refresh_workspace_lock(
    workspace_root: &Path,
    document: &WorkspaceDocument,
) -> Result<WorkspaceDocument, WorkspaceError> {
    validate_workspace_document(document)?;
    let order = topological_package_order(document)?;
    let by_name: BTreeMap<&str, &WorkspacePackage> = document
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let mut packages = Vec::with_capacity(order.len());
    for name in order {
        let package = by_name.get(name.as_str()).expect("package in order");
        let loaded = load_workspace_project(workspace_root, package)?;
        if loaded.project.lock.is_none() {
            return Err(WorkspaceError::new(
                "AE-WORKSPACE-005",
                format!(
                    "workspace lock requires package {} project manifest to carry a complete project lock",
                    package.name
                ),
            ));
        }
        let _ = verify_project(&loaded.package_root, &loaded.project)?;
        packages.push(WorkspaceLockPackage {
            name: package.name.clone(),
            path: package.path.clone(),
            project_name: loaded.project.name,
            project_version: loaded.project.version,
            project_sha256: sha256_hex(&loaded.project_bytes),
        });
    }
    let mut refreshed = document.clone();
    refreshed.lock = Some(WorkspaceLock { packages });
    validate_workspace_document(&refreshed)?;
    Ok(refreshed)
}

/// Render a validated workspace document as canonical local JSON.
pub fn serialize_workspace_document(
    document: &WorkspaceDocument,
) -> Result<String, WorkspaceError> {
    validate_workspace_document(document)?;
    serde_json::to_string_pretty(document)
        .map(|json| format!("{json}\n"))
        .map_err(|error| {
            WorkspaceError::new(
                "AE-WORKSPACE-001",
                format!("could not serialize aether.workspace/v1 document: {error}"),
            )
        })
}

/// Topological order: dependencies before dependents.
pub fn topological_package_order(
    document: &WorkspaceDocument,
) -> Result<Vec<String>, WorkspaceError> {
    let index: BTreeMap<&str, &WorkspacePackage> = document
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let mut indegree: BTreeMap<&str, usize> = index.keys().map(|name| (*name, 0usize)).collect();
    let mut edges: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for package in &document.packages {
        for dep in &package.depends_on {
            // edge dep -> package (dep must come first)
            edges
                .entry(dep.as_str())
                .or_default()
                .push(package.name.as_str());
            *indegree.get_mut(package.name.as_str()).expect("indegree") += 1;
        }
    }
    let mut queue: VecDeque<&str> = indegree
        .iter()
        .filter_map(|(name, degree)| (*degree == 0).then_some(*name))
        .collect();
    // Stable order among ready nodes.
    let mut ready: Vec<&str> = queue.drain(..).collect();
    ready.sort_unstable();
    queue.extend(ready);

    let mut order = Vec::with_capacity(document.packages.len());
    while let Some(name) = queue.pop_front() {
        order.push(name.to_owned());
        if let Some(children) = edges.get(name) {
            let mut next_ready = Vec::new();
            for child in children {
                let degree = indegree.get_mut(child).expect("indegree child");
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    next_ready.push(*child);
                }
            }
            next_ready.sort_unstable();
            queue.extend(next_ready);
        }
    }
    if order.len() != document.packages.len() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-003",
            "workspace package depends_on graph contains a cycle",
        ));
    }
    Ok(order)
}

/// Verify every package project under the workspace root.
pub fn verify_workspace(
    workspace_root: &Path,
    document: &WorkspaceDocument,
) -> Result<WorkspaceVerifyReport, WorkspaceError> {
    validate_workspace_document(document)?;
    let order = topological_package_order(document)?;
    let by_name: BTreeMap<&str, &WorkspacePackage> = document
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let locked_by_name: Option<BTreeMap<&str, &WorkspaceLockPackage>> =
        document.lock.as_ref().map(|lock| {
            lock.packages
                .iter()
                .map(|entry| (entry.name.as_str(), entry))
                .collect()
        });

    let mut packages = Vec::with_capacity(order.len());
    let mut failures = Vec::new();
    for name in order {
        let package = by_name.get(name.as_str()).expect("package in order");
        let lock_entry = locked_by_name
            .as_ref()
            .and_then(|entries| entries.get(package.name.as_str()).copied());
        match verify_one_package(workspace_root, package, lock_entry) {
            Ok(report) => packages.push(report),
            Err(error) => failures.push((package.name.clone(), error)),
        }
    }
    if !failures.is_empty() {
        let code = if failures
            .iter()
            .any(|(_, error)| error.code == "AE-WORKSPACE-005")
        {
            "AE-WORKSPACE-005"
        } else {
            "AE-WORKSPACE-004"
        };
        return Err(WorkspaceError::new(
            code,
            format!(
                "workspace package verification failed: {}",
                failures
                    .iter()
                    .map(|(name, error)| format!("{name}: {error}"))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }
    Ok(WorkspaceVerifyReport {
        name: document.name.clone(),
        version: document.version.clone(),
        packages,
    })
}

/// Build one workspace package (main cone) with M22 cross-package imports.
pub fn compile_workspace_package(
    workspace_root: &Path,
    document: &WorkspaceDocument,
    package_name: &str,
) -> Result<Vec<u8>, WorkspaceError> {
    validate_workspace_document(document)?;
    if document.lock.is_some() {
        let _ = verify_workspace(workspace_root, document)?;
    }
    let package = document
        .packages
        .iter()
        .find(|package| package.name == package_name)
        .ok_or_else(|| {
            WorkspaceError::new(
                "AE-WORKSPACE-003",
                format!("workspace has no package named {package_name}"),
            )
        })?;
    let loaded = load_workspace_project(workspace_root, package)?;
    let package_root = loaded.package_root;
    let project = loaded.project;

    let mut package_roots = BTreeMap::new();
    let mut allowed = BTreeSet::new();
    for dep in &package.depends_on {
        allowed.insert(dep.clone());
        let dep_pkg = document
            .packages
            .iter()
            .find(|candidate| candidate.name == *dep)
            .expect("depends_on validated");
        let root = resolve_package_path(workspace_root, &dep_pkg.path)?;
        package_roots.insert(dep.clone(), root);
    }
    // Allow importing from self by package name as well.
    allowed.insert(package.name.clone());
    package_roots.insert(package.name.clone(), package_root.clone());

    compile_project_modules_with_packages(&package_root, &project, &package_roots, &allowed)
        .map_err(|error| {
            WorkspaceError::new(
                "AE-WORKSPACE-004",
                format!(
                    "workspace package {package_name} build failed [{}]: {}",
                    error.code, error.message
                ),
            )
        })
}

fn verify_one_package(
    workspace_root: &Path,
    package: &WorkspacePackage,
    lock_entry: Option<&WorkspaceLockPackage>,
) -> Result<WorkspacePackageReport, WorkspaceError> {
    let loaded = load_workspace_project(workspace_root, package)?;
    if let Some(entry) = lock_entry {
        verify_workspace_lock_entry(package, entry, &loaded)?;
    }
    let report = verify_project(&loaded.package_root, &loaded.project)?;
    Ok(WorkspacePackageReport {
        name: package.name.clone(),
        path: package.path.clone(),
        project: report,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn create() -> Self {
            let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "aether-workspace-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temp");
            Self { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_pkg(root: &Path, dir: &str, project_name: &str, exit: i64) {
        let pkg = root.join(dir);
        fs::create_dir_all(&pkg).expect("pkg dir");
        let main = format!("world {project_name}\n\nweave main [] -> Whole:\n  yield {exit}\n");
        fs::write(pkg.join("main.ae"), main).expect("main");
        let digest =
            crate::project::sha256_hex(fs::read(pkg.join("main.ae")).expect("read").as_slice());
        let project = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "{project_name}",
  "version": "0.1.0",
  "units": [{{ "path": "main.ae", "role": "main" }}],
  "lock": {{ "units": [{{ "path": "main.ae", "sha256": "{digest}" }}] }}
}}"#
        );
        fs::write(pkg.join("aether.project.json"), project).expect("project");
    }

    fn write_unlocked_pkg(root: &Path, dir: &str, project_name: &str, exit: i64) {
        let pkg = root.join(dir);
        fs::create_dir_all(&pkg).expect("pkg dir");
        let main = format!("world {project_name}\n\nweave main [] -> Whole:\n  yield {exit}\n");
        fs::write(pkg.join("main.ae"), main).expect("main");
        let project = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "{project_name}",
  "version": "0.1.0",
  "units": [{{ "path": "main.ae", "role": "main" }}]
}}"#
        );
        fs::write(pkg.join("aether.project.json"), project).expect("project");
    }

    #[test]
    fn verifies_workspace_with_depends_on_order() {
        let temp = TempDir::create();
        write_pkg(&temp.path, "util", "util_pkg", 0);
        write_pkg(&temp.path, "app", "app_pkg", 1);
        let workspace = r#"{
  "schema": "aether.workspace/v1",
  "name": "demo_ws",
  "version": "0.1.0",
  "packages": [
    { "name": "app", "path": "app", "depends_on": ["util"] },
    { "name": "util", "path": "util" }
  ]
}"#;
        let document = parse_workspace_document(workspace).expect("parse");
        let order = topological_package_order(&document).expect("order");
        assert_eq!(order, vec!["util".to_owned(), "app".to_owned()]);
        let report = verify_workspace(&temp.path, &document).expect("verify");
        assert_eq!(report.packages.len(), 2);
        assert_eq!(report.packages[0].name, "util");
        assert_eq!(report.packages[1].name, "app");
    }

    #[test]
    fn rejects_cycle_and_escape_and_missing_project() {
        let cycle = r#"{
  "schema": "aether.workspace/v1",
  "name": "cyc",
  "version": "0.1.0",
  "packages": [
    { "name": "a", "path": "a", "depends_on": ["b"] },
    { "name": "b", "path": "b", "depends_on": ["a"] }
  ]
}"#;
        let error = parse_workspace_document(cycle).expect_err("cycle");
        assert_eq!(error.code, "AE-WORKSPACE-003");

        let escape = r#"{
  "schema": "aether.workspace/v1",
  "name": "esc",
  "version": "0.1.0",
  "packages": [{ "name": "x", "path": "../x" }]
}"#;
        let error = parse_workspace_document(escape).expect_err("escape");
        assert_eq!(error.code, "AE-WORKSPACE-002");

        let temp = TempDir::create();
        fs::create_dir_all(temp.path.join("empty")).expect("empty");
        let missing = r#"{
  "schema": "aether.workspace/v1",
  "name": "miss",
  "version": "0.1.0",
  "packages": [{ "name": "empty", "path": "empty" }]
}"#;
        let document = parse_workspace_document(missing).expect("parse missing");
        let error = verify_workspace(&temp.path, &document).expect_err("missing project");
        assert_eq!(error.code, "AE-WORKSPACE-004");
    }

    #[test]
    fn project_manifest_containment_guard_rejects_an_escaped_canonical_target() {
        let temp = TempDir::create();
        let package = WorkspacePackage {
            name: "app".to_owned(),
            path: "app".to_owned(),
            depends_on: Vec::new(),
        };
        let package_root = temp.path.join("app");
        let escaped_manifest = temp.path.join("outside").join(WORKSPACE_PROJECT_FILE);

        let error = require_project_manifest_within_package_root(
            &package_root,
            &escaped_manifest,
            &package,
        )
        .expect_err("canonical manifest target outside package must fail closed");

        assert_eq!(error.code, "AE-WORKSPACE-002");
        assert!(
            error.message.contains("escapes package root"),
            "{}",
            error.message
        );
    }

    #[test]
    fn refreshes_complete_workspace_lock_and_detects_manifest_or_unit_drift() {
        let temp = TempDir::create();
        write_pkg(&temp.path, "util", "util_pkg", 0);
        write_pkg(&temp.path, "app", "app_pkg", 42);
        let workspace = r#"{
  "schema": "aether.workspace/v1",
  "name": "locked_demo",
  "version": "0.1.0",
  "packages": [
    { "name": "app", "path": "app", "depends_on": ["util"] },
    { "name": "util", "path": "util" }
  ]
}"#;
        let document = parse_workspace_document(workspace).expect("parse unlocked workspace");
        let refreshed = refresh_workspace_lock(&temp.path, &document).expect("refresh lock");
        let lock = refreshed.lock.as_ref().expect("workspace lock");
        assert_eq!(
            lock.packages
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["util", "app"]
        );
        assert_eq!(lock.packages[0].project_name, "util_pkg");
        assert_eq!(lock.packages[1].project_version, "0.1.0");
        verify_workspace(&temp.path, &refreshed).expect("fresh workspace lock verifies");

        let rendered = serialize_workspace_document(&refreshed).expect("serialize workspace");
        assert!(rendered.ends_with('\n'));
        let reparsed = parse_workspace_document(&rendered).expect("parse canonical workspace");
        verify_workspace(&temp.path, &reparsed).expect("canonical workspace verifies");

        let mut stale = refreshed.clone();
        stale.lock.as_mut().expect("stale lock").packages[0].project_sha256 = "0".repeat(64);
        let repaired = refresh_workspace_lock(&temp.path, &stale).expect("stale lock refreshes");
        assert_eq!(repaired, refreshed);

        let app_project = temp.path.join("app").join(WORKSPACE_PROJECT_FILE);
        let original_manifest = fs::read(&app_project).expect("app project manifest");
        let changed_whitespace = String::from_utf8(original_manifest.clone())
            .expect("project UTF-8")
            .replace('\n', "\r\n");
        fs::write(&app_project, changed_whitespace).expect("mutate manifest bytes");
        let error = verify_workspace(&temp.path, &refreshed).expect_err("manifest drift fails");
        assert_eq!(error.code, "AE-WORKSPACE-005");
        fs::write(&app_project, original_manifest).expect("restore manifest bytes");

        let util_source = temp.path.join("util").join("main.ae");
        fs::write(
            &util_source,
            "world util_pkg\n\nweave main [] -> Whole:\n  yield 9\n",
        )
        .expect("mutate locked source");
        let error = verify_workspace(&temp.path, &refreshed).expect_err("unit drift fails");
        assert_eq!(error.code, "AE-WORKSPACE-004");
        assert!(
            error.message.contains("AE-PROJECT-003"),
            "{}",
            error.message
        );
    }

    #[test]
    fn rejects_invalid_or_incomplete_workspace_lock_and_requires_project_locks() {
        let digest = "0".repeat(64);
        for workspace in [
            r#"{
  "schema": "aether.workspace/v1",
  "name": "bad_lock",
  "version": "0.1.0",
  "packages": [{ "name": "app", "path": "app" }],
  "lock": { "packages": [] }
}"#
            .to_owned(),
            format!(
                r#"{{
  "schema": "aether.workspace/v1",
  "name": "bad_lock",
  "version": "0.1.0",
  "packages": [{{ "name": "app", "path": "app" }}],
  "lock": {{ "packages": [{{
    "name": "app", "path": "other", "project_name": "app_pkg",
    "project_version": "0.1.0", "project_sha256": "{digest}"
  }}] }}
}}"#
            ),
        ] {
            let error = parse_workspace_document(&workspace).expect_err("invalid lock rejected");
            assert_eq!(error.code, "AE-WORKSPACE-005");
        }

        let temp = TempDir::create();
        write_unlocked_pkg(&temp.path, "app", "app_pkg", 0);
        let workspace = r#"{
  "schema": "aether.workspace/v1",
  "name": "unlocked_project",
  "version": "0.1.0",
  "packages": [{ "name": "app", "path": "app" }]
}"#;
        let document = parse_workspace_document(workspace).expect("parse workspace");
        let error =
            refresh_workspace_lock(&temp.path, &document).expect_err("project lock required");
        assert_eq!(error.code, "AE-WORKSPACE-005");
    }
}
