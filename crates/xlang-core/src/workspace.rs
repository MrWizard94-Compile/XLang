//! Offline multi-package workspaces (`aether.workspace/v1`) — M18 / M22.
//!
//! A workspace lists local package directories, each containing
//! `aether.project.json`. Optional `depends_on` edges form an acyclic graph for
//! verification order and authorize M22 `import unit … from package`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::modules::compile_project_modules_with_packages;
use crate::project::{
    parse_project_document, verify_project, ProjectError, ProjectVerifyReport,
};
use crate::{CompileOutput, LANGUAGE_NAME, LANGUAGE_VERSION};

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
            format!("nested project verify failed [{}]: {}", error.code, error.message),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceDocument {
    pub schema: String,
    pub name: String,
    pub version: String,
    pub packages: Vec<WorkspacePackage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
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
                    format!(
                        "package {} depends_on unknown package {dep}",
                        package.name
                    ),
                ));
            }
        }
    }
    let _ = topological_package_order(document)?;
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
        if !segment.bytes().all(|byte| {
            matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-')
        }) {
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
            edges.entry(dep.as_str()).or_default().push(package.name.as_str());
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

    let mut packages = Vec::with_capacity(order.len());
    let mut failures = Vec::new();
    for name in order {
        let package = by_name.get(name.as_str()).expect("package in order");
        match verify_one_package(workspace_root, package) {
            Ok(report) => packages.push(report),
            Err(error) => failures.push(format!("{}: {error}", package.name)),
        }
    }
    if !failures.is_empty() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!("workspace package verification failed: {}", failures.join("; ")),
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
) -> Result<CompileOutput, WorkspaceError> {
    validate_workspace_document(document)?;
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
    let package_root = resolve_package_path(workspace_root, &package.path)?;
    let project_file = package_root.join(WORKSPACE_PROJECT_FILE);
    let json = fs::read_to_string(&project_file).map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "could not read {} for package {}: {error}",
                project_file.display(),
                package.name
            ),
        )
    })?;
    let project = parse_project_document(&json)?;

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

    compile_project_modules_with_packages(&package_root, &project, &package_roots, &allowed).map_err(
        |error| {
            WorkspaceError::new(
                "AE-WORKSPACE-004",
                format!(
                    "workspace package {package_name} build failed [{}]: {}",
                    error.code, error.message
                ),
            )
        },
    )
}

fn verify_one_package(
    workspace_root: &Path,
    package: &WorkspacePackage,
) -> Result<WorkspacePackageReport, WorkspaceError> {
    let package_root = resolve_package_path(workspace_root, &package.path)?;
    let project_file = package_root.join(WORKSPACE_PROJECT_FILE);
    if !project_file.is_file() {
        return Err(WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "package {} path {} is missing {WORKSPACE_PROJECT_FILE}",
                package.name, package.path
            ),
        ));
    }
    let json = fs::read_to_string(&project_file).map_err(|error| {
        WorkspaceError::new(
            "AE-WORKSPACE-004",
            format!(
                "could not read {} for package {}: {error}",
                project_file.display(),
                package.name
            ),
        )
    })?;
    let project = parse_project_document(&json)?;
    let report = verify_project(&package_root, &project)?;
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
        let digest = crate::project::sha256_hex(
            fs::read(pkg.join("main.ae")).expect("read").as_slice(),
        );
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
}
