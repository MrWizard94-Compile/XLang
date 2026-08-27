//! M11a language modules: import unit / export weave / project build.
//!
//! General product multi-module programs use the bounded GSM-001 catalog ABI:
//! the host frames caller-selected, manifest-authorized opaque source Text and
//! the verified Aether-written seed resolves imports, elaborates the graph, and
//! emits AETH. The retained Rust elaborator is a bootstrap/reference oracle for
//! dual-compare and recovery analysis, never the default product route.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::project::{
    parse_project_document, resolve_unit_path, validate_unit_path, ProjectDocument, ProjectError,
    ProjectUnit, ProjectUnitRole, PROJECT_SCHEMA_VERSION,
};
use crate::{
    compile_product_bytecode, run_bytecode, run_bytecode_with_grants, verify_bytecode,
    HostGrantConfig, LANGUAGE_NAME, LANGUAGE_VERSION,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleImport {
    /// Unit path within the resolved package/project (project-relative grammar).
    path: String,
    /// When set, resolve under that workspace package root (M22).
    package: Option<String>,
    alias: String,
}

impl ModuleImport {
    fn graph_key(&self) -> String {
        match &self.package {
            Some(package) => format!("{package}::{}", self.path),
            None => self.path.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedModule {
    path: String,
    world: String,
    role: ProjectUnitRole,
    imports: Vec<ModuleImport>,
    /// weave name -> exported?
    weaves: BTreeMap<String, bool>,
    /// Source without import lines; `export weave` still present until strip.
    body_source: String,
    has_main: bool,
    has_records_or_shapes_or_host: bool,
}

fn module_error(code: &'static str, message: impl Into<String>) -> ProjectError {
    ProjectError {
        code,
        message: message.into(),
    }
}

/// Stable mangled weave name for a unit path + weave.
///
/// Accepts ordinary unit paths (`lib/math.ae`) and M22 package keys
/// (`util::whole.ae`). Output uses only lowercase letters, digits, and `_`.
#[must_use]
pub fn mangle_weave(unit_path: &str, weave: &str) -> String {
    let stem = unit_path
        .strip_suffix(".ae")
        .unwrap_or(unit_path)
        .replace(['/', ':', '.'], "_")
        .to_ascii_lowercase();
    format!("m_{stem}_{weave}")
}

/// Parse one module unit source (project-relative path for identity).
fn parse_module_source(
    path: &str,
    source: &str,
    role: ProjectUnitRole,
) -> Result<ParsedModule, ProjectError> {
    let lines: Vec<&str> = source
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .filter(|line| !line.is_empty())
        .collect();
    if lines.is_empty() {
        return Err(module_error(
            "AE-MOD-001",
            format!("module {path} is empty"),
        ));
    }
    let world_line = lines[0].trim_start();
    if !world_line.starts_with("world ") {
        return Err(module_error(
            "AE-MOD-001",
            format!("module {path} must begin with a world declaration"),
        ));
    }
    let world = world_line["world ".len()..].trim().to_owned();
    if world.is_empty() {
        return Err(module_error(
            "AE-MOD-001",
            format!("module {path} has an empty world name"),
        ));
    }

    let mut imports = Vec::new();
    let mut body_lines = Vec::new();
    let mut index = 1;
    while index < lines.len() {
        let content = lines[index].trim_start();
        if content.starts_with("import unit ") {
            if lines[index].starts_with(' ') || lines[index].starts_with('\t') {
                return Err(module_error(
                    "AE-MOD-001",
                    format!("import unit in {path} must be at indentation zero"),
                ));
            }
            imports.push(parse_import_line(content, path)?);
            index += 1;
            continue;
        }
        break;
    }
    while index < lines.len() {
        body_lines.push(lines[index]);
        index += 1;
    }

    let mut weaves = BTreeMap::new();
    let mut has_records_or_shapes_or_host = false;
    for line in &body_lines {
        let content = line.trim_start();
        if line.starts_with(' ') {
            continue;
        }
        if content.starts_with("record ")
            || content.starts_with("shape ")
            || content.starts_with("host ")
        {
            has_records_or_shapes_or_host = true;
        }
        if let Some(rest) = content.strip_prefix("export weave ") {
            if rest.starts_with("main ") {
                return Err(module_error(
                    "AE-MOD-006",
                    format!("module {path} cannot export main"),
                ));
            }
            let name = weave_name_from_header(rest, path)?;
            if weaves.insert(name.clone(), true).is_some() {
                return Err(module_error(
                    "AE-MOD-005",
                    format!("module {path} declares weave {name} more than once"),
                ));
            }
        } else if let Some(rest) = content.strip_prefix("weave ") {
            let name = weave_name_from_header(rest, path)?;
            if weaves.insert(name.clone(), false).is_some() {
                return Err(module_error(
                    "AE-MOD-005",
                    format!("module {path} declares weave {name} more than once"),
                ));
            }
        }
    }

    let has_main = weaves.contains_key("main");
    match role {
        ProjectUnitRole::Main | ProjectUnitRole::Test => {
            if !has_main {
                return Err(module_error(
                    "AE-MOD-006",
                    format!("entry module {path} must declare weave main"),
                ));
            }
        }
        ProjectUnitRole::Lib => {
            if has_main {
                return Err(module_error(
                    "AE-MOD-006",
                    format!("lib module {path} must not declare weave main"),
                ));
            }
            if has_records_or_shapes_or_host {
                return Err(module_error(
                    "AE-MOD-001",
                    format!(
                        "lib module {path} may only declare weaves in M11a (no records, shapes, or host weaves)"
                    ),
                ));
            }
        }
    }

    let body_source = body_lines.join("\n");
    let body_source = if body_source.is_empty() {
        body_source
    } else {
        format!("{body_source}\n")
    };

    Ok(ParsedModule {
        path: path.to_owned(),
        world,
        role,
        imports,
        weaves,
        body_source,
        has_main,
        has_records_or_shapes_or_host,
    })
}

fn parse_import_line(content: &str, module_path: &str) -> Result<ModuleImport, ProjectError> {
    // import unit "path" as alias
    // import unit "path" from package name as alias  (M22)
    let rest = content
        .strip_prefix("import unit ")
        .ok_or_else(|| module_error("AE-MOD-001", "expected import unit"))?;
    let rest = rest.trim();
    if !rest.starts_with('"') {
        return Err(module_error(
            "AE-MOD-001",
            format!("import unit in {module_path} requires a quoted path"),
        ));
    }
    let after = &rest[1..];
    let Some(end) = after.find('"') else {
        return Err(module_error(
            "AE-MOD-001",
            format!("import unit in {module_path} has an unclosed path string"),
        ));
    };
    let path = after[..end].to_owned();
    validate_unit_path(&path).map_err(|error| {
        module_error(
            "AE-MOD-002",
            format!("import path in {module_path}: {}", error.message),
        )
    })?;
    let mut tail = after[end + 1..].trim_start();
    let mut package = None;
    if let Some(pkg_rest) = tail.strip_prefix("from package ") {
        let pkg_rest = pkg_rest.trim_start();
        let Some((pkg_name, after_pkg)) = pkg_rest.split_once(" as ") else {
            return Err(module_error(
                "AE-MOD-001",
                format!("import unit in {module_path} with from package requires `as <alias>`"),
            ));
        };
        let pkg_name = pkg_name.trim();
        if pkg_name.is_empty()
            || !pkg_name
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic())
            || !pkg_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(module_error(
                "AE-MOD-001",
                format!("package name in import of {module_path} is invalid"),
            ));
        }
        package = Some(pkg_name.to_owned());
        tail = after_pkg.trim_start();
        // alias is the remainder after " as " already split
        let alias = tail.trim();
        return finish_import(path, package, alias, module_path);
    }
    let Some(alias_part) = tail.strip_prefix("as ") else {
        return Err(module_error(
            "AE-MOD-001",
            format!("import unit in {module_path} requires `as <alias>`"),
        ));
    };
    finish_import(path, package, alias_part.trim(), module_path)
}

fn finish_import(
    path: String,
    package: Option<String>,
    alias: &str,
    module_path: &str,
) -> Result<ModuleImport, ProjectError> {
    if alias.is_empty()
        || !alias.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        || !alias
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(module_error(
            "AE-MOD-001",
            format!("import alias in {module_path} must be a lowercase identifier"),
        ));
    }
    if alias == "main" || alias == "world" || alias == "call" {
        return Err(module_error(
            "AE-MOD-001",
            format!("import alias {alias} is reserved"),
        ));
    }
    Ok(ModuleImport {
        path,
        package,
        alias: alias.to_owned(),
    })
}

fn weave_name_from_header(rest: &str, module_path: &str) -> Result<String, ProjectError> {
    let Some(opening) = rest.find('[') else {
        return Err(module_error(
            "AE-MOD-001",
            format!("weave header in {module_path} is missing parameters"),
        ));
    };
    let name = rest[..opening].trim_end();
    if name.is_empty() {
        return Err(module_error(
            "AE-MOD-001",
            format!("weave in {module_path} is missing a name"),
        ));
    }
    Ok(name.to_owned())
}

/// Validate a lib unit that cannot seed-compile alone (no main).
pub fn validate_lib_module_source(path: &str, source: &str) -> Result<(), ProjectError> {
    let parsed = parse_module_source(path, source, ProjectUnitRole::Lib)?;
    if parsed.weaves.is_empty() {
        return Err(module_error(
            "AE-MOD-001",
            format!("lib module {path} must declare at least one weave"),
        ));
    }
    if !parsed.imports.is_empty() {
        // Imports need the full graph; structure-only check already passed.
        return Ok(());
    }
    // ADR-052: product-seed probe with a synthetic total main (no bootstrap).
    debug_assert!(
        crate::lib_module_validates_via_product_seed(),
        "ADR-052: lib module validation must use product seed"
    );
    let body = strip_export_keyword(&parsed.body_source);
    let probe = format!(
        "world {}\n\n{}\nweave main [] -> Whole:\n  yield 0\n",
        parsed.world,
        body.trim_end()
    );
    compile_product_bytecode(&probe).map_err(|error| {
        module_error(
            "AE-PROJECT-004",
            format!("lib unit {path} failed product validation compile: {error}"),
        )
    })?;
    Ok(())
}

/// True when source uses M11 module surface that blocks single-file seed compile.
#[must_use]
pub fn source_requires_project_modules(source: &str) -> bool {
    source.lines().any(|line| {
        let t = line.trim_start();
        t.starts_with("import unit ") || t.starts_with("export weave ")
    })
}

fn load_graph(
    project_root: &Path,
    document: &ProjectDocument,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<BTreeMap<String, ParsedModule>, ProjectError> {
    let mut by_key = BTreeMap::new();
    for unit in &document.units {
        let resolved = resolve_unit_path(project_root, &unit.path)?;
        let bytes = fs::read(&resolved).map_err(|error| {
            module_error(
                "AE-PROJECT-002",
                format!("could not read unit {}: {error}", unit.path),
            )
        })?;
        let source = String::from_utf8(bytes).map_err(|_| {
            module_error(
                "AE-PROJECT-004",
                format!("unit {} is not valid UTF-8", unit.path),
            )
        })?;
        let parsed = parse_module_source(&unit.path, &source, unit.role)?;
        by_key.insert(unit.path.clone(), parsed);
    }

    // Load foreign package units referenced by imports (M22), fixed-point.
    loop {
        let pending: Vec<ModuleImport> = by_key
            .values()
            .flat_map(|module| module.imports.clone())
            .filter(|import| import.package.is_some() && !by_key.contains_key(&import.graph_key()))
            .collect();
        if pending.is_empty() {
            break;
        }
        for import in pending {
            let package_name = import.package.as_ref().expect("foreign import");
            if !allowed_packages.contains(package_name) {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "import from package {package_name} is not allowed (missing workspace depends_on)"
                    ),
                ));
            }
            let key = import.graph_key();
            if by_key.contains_key(&key) {
                continue;
            }
            let package_root = package_roots.get(package_name).ok_or_else(|| {
                module_error(
                    "AE-MOD-002",
                    format!("workspace package {package_name} is not available"),
                )
            })?;
            let foreign = load_foreign_package_unit(package_root, package_name, &import.path)?;
            by_key.insert(key, foreign);
        }
    }
    Ok(by_key)
}

fn load_foreign_package_unit(
    package_root: &Path,
    package_name: &str,
    unit_path: &str,
) -> Result<ParsedModule, ProjectError> {
    let project_file = package_root.join("aether.project.json");
    let json = fs::read_to_string(&project_file).map_err(|error| {
        module_error(
            "AE-MOD-002",
            format!("package {package_name} missing aether.project.json: {error}"),
        )
    })?;
    let document = parse_project_document(&json)?;
    if document.schema != PROJECT_SCHEMA_VERSION {
        return Err(module_error(
            "AE-MOD-002",
            format!("package {package_name} has unsupported project schema"),
        ));
    }
    let unit = document
        .units
        .iter()
        .find(|unit| unit.path == unit_path)
        .ok_or_else(|| {
            module_error(
                "AE-MOD-002",
                format!("package {package_name} has no unit {unit_path}"),
            )
        })?;
    if unit.role != ProjectUnitRole::Lib {
        return Err(module_error(
            "AE-MOD-002",
            format!(
                "package {package_name} unit {unit_path} must be role lib for cross-package import"
            ),
        ));
    }
    let resolved = resolve_unit_path(package_root, unit_path)?;
    let bytes = fs::read(&resolved).map_err(|error| {
        module_error(
            "AE-PROJECT-002",
            format!("could not read package {package_name} unit {unit_path}: {error}"),
        )
    })?;
    let source = String::from_utf8(bytes).map_err(|_| {
        module_error(
            "AE-PROJECT-004",
            format!("package {package_name} unit {unit_path} is not valid UTF-8"),
        )
    })?;
    let key = format!("{package_name}::{unit_path}");
    let mut parsed = parse_module_source(&key, &source, ProjectUnitRole::Lib)?;
    // Graph identity uses package::path; keep path field as key for mangling.
    parsed.path = key;
    Ok(parsed)
}

fn closed_cone(
    entry: &str,
    modules: &BTreeMap<String, ParsedModule>,
) -> Result<Vec<String>, ProjectError> {
    let mut order = Vec::new();
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();

    fn dfs(
        path: &str,
        modules: &BTreeMap<String, ParsedModule>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), ProjectError> {
        if visited.contains(path) {
            return Ok(());
        }
        if !visiting.insert(path.to_owned()) {
            return Err(module_error(
                "AE-MOD-004",
                format!("import cycle involving unit {path}"),
            ));
        }
        let module = modules.get(path).ok_or_else(|| {
            module_error(
                "AE-MOD-002",
                format!("import path {path} is not a project unit"),
            )
        })?;
        for import in &module.imports {
            let target_key = import.graph_key();
            if !modules.contains_key(&target_key) {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "module {} imports {}, which is not listed in the project or workspace package graph",
                        path, target_key
                    ),
                ));
            }
            let target = &modules[&target_key];
            if target.role == ProjectUnitRole::Main || target.role == ProjectUnitRole::Test {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "module {path} cannot import the {} unit {target_key}",
                        match target.role {
                            ProjectUnitRole::Main => "main",
                            ProjectUnitRole::Test => "test",
                            ProjectUnitRole::Lib => "lib",
                        }
                    ),
                ));
            }
            dfs(&target_key, modules, visiting, visited, order)?;
        }
        visiting.remove(path);
        visited.insert(path.to_owned());
        order.push(path.to_owned());
        Ok(())
    }

    dfs(entry, modules, &mut visiting, &mut visited, &mut order)?;
    Ok(order)
}

fn rewrite_calls(
    body: &str,
    local_map: &BTreeMap<String, String>,
    import_map: &BTreeMap<String, String>,
) -> String {
    // Rewrite `call alias.name` and `call local` in expression positions.
    // Conservative: only rewrite tokens after `call `, `forward call `, `spawn call `.
    let mut output = String::with_capacity(body.len());
    for line in body.split_inclusive('\n') {
        output.push_str(&rewrite_line_calls(line, local_map, import_map));
    }
    output
}

fn rewrite_line_calls(
    line: &str,
    local_map: &BTreeMap<String, String>,
    import_map: &BTreeMap<String, String>,
) -> String {
    let mut result = line.to_owned();
    for (from, to) in import_map {
        // call alias.name
        let pattern = format!("call {from}");
        let replacement = format!("call {to}");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("forward call {from}");
        let replacement = format!("forward call {to}");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("spawn call {from}");
        let replacement = format!("spawn call {to}");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("handle call {from}");
        let replacement = format!("handle call {to}");
        result = result.replace(&pattern, &replacement);
    }
    for (from, to) in local_map {
        if from == to {
            continue;
        }
        let pattern = format!("call {from} ");
        let replacement = format!("call {to} ");
        result = result.replace(&pattern, &replacement);
        // end of line / before newline
        let pattern = format!("call {from}\n");
        let replacement = format!("call {to}\n");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("call {from}\r\n");
        let replacement = format!("call {to}\r\n");
        result = result.replace(&pattern, &replacement);
        // call name at EOL without trailing space already handled if no args - rare
        if result.trim_end() == format!("  yield call {from}")
            || result.contains(&format!("call {from}\n"))
        {
            result = result.replace(&format!("call {from}"), &format!("call {to}"));
        } else {
            // trailing call with no args before newline already
            let pattern = format!("call {from}");
            // only replace whole-token: after call space, name not followed by .
            result = replace_call_token(&result, from, to);
            let _ = pattern;
        }
        let pattern = format!("forward call {from} ");
        let replacement = format!("forward call {to} ");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("spawn call {from} ");
        let replacement = format!("spawn call {to} ");
        result = result.replace(&pattern, &replacement);
        let pattern = format!("handle call {from} ");
        let replacement = format!("handle call {to} ");
        result = result.replace(&pattern, &replacement);
    }
    result
}

fn replace_call_token(line: &str, from: &str, to: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;
    let needle = format!("call {from}");
    let n = needle.as_bytes();
    while i < bytes.len() {
        if i + n.len() <= bytes.len() && &bytes[i..i + n.len()] == n {
            let after = i + n.len();
            let boundary = after >= bytes.len()
                || matches!(bytes[after], b' ' | b'\n' | b'\r')
                || after + 9 <= bytes.len() && &bytes[after..after + 9] == b" raises ";
            // do not rewrite call math.double when replacing call math
            let not_dot = after >= bytes.len() || bytes[after] != b'.';
            if boundary && not_dot {
                out.push_str("call ");
                out.push_str(to);
                i = after;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn strip_export_keyword(body: &str) -> String {
    body.lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("export weave ") {
                format!("weave {rest}")
            } else if let Some(idx) = line.find("export weave ") {
                // indented should not happen
                let (prefix, rest) = line.split_at(idx);
                format!("{prefix}weave {}", &rest["export weave ".len()..])
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if body.ends_with('\n') { "\n" } else { "" }
}

fn rename_weaves_in_body(body: &str, renames: &BTreeMap<String, String>) -> String {
    let mut result = strip_export_keyword(body);
    // Rename declarations first (longest names first to avoid partial issues - names unique)
    let mut pairs: Vec<_> = renames.iter().collect();
    pairs.sort_by_key(|b| std::cmp::Reverse(b.0.len()));
    for (from, to) in &pairs {
        if from == to {
            continue;
        }
        result = result.replace(&format!("weave {from} ["), &format!("weave {to} ["));
    }
    result
}

/// Elaborate the main unit's import cone into one single-file Aether source.
///
/// This retained Rust elaborator is a bootstrap/reference oracle for
/// dual-compare tests and recovery analysis. Product project and workspace
/// builds use the seed-owned general module catalog instead.
pub fn elaborate_project_modules(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<String, ProjectError> {
    elaborate_project_modules_with_packages(
        project_root,
        document,
        &BTreeMap::new(),
        &BTreeSet::new(),
    )
}

/// Elaborate with optional workspace package roots for M22 cross-package imports.
pub fn elaborate_project_modules_with_packages(
    project_root: &Path,
    document: &ProjectDocument,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<String, ProjectError> {
    let entry = document
        .units
        .iter()
        .find(|unit| unit.role == ProjectUnitRole::Main)
        .ok_or_else(|| module_error("AE-MOD-006", "project has no main unit"))?;
    elaborate_project_entry_with_packages(
        project_root,
        document,
        &entry.path,
        package_roots,
        allowed_packages,
    )
}

/// Elaborate a specific entry unit's import cone (main or M17b test).
pub fn elaborate_project_entry(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
) -> Result<String, ProjectError> {
    elaborate_project_entry_with_packages(
        project_root,
        document,
        entry_path,
        &BTreeMap::new(),
        &BTreeSet::new(),
    )
}

/// Elaborate a selectable entry unit with optional workspace package roots.
pub fn elaborate_project_entry_with_packages(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<String, ProjectError> {
    let modules = load_graph(project_root, document, package_roots, allowed_packages)?;
    elaborate_modules_map(&modules, entry_path)
}

/// ADR-075: multi-source forge envelope schema (host multi-file product path).
pub const MULTI_SOURCE_ENVELOPE_SCHEMA: &str = "aether.multi-source/v1";

/// ADR-128 seed-native two-unit source-bundle schema.
///
/// Unlike [`MULTI_SOURCE_ENVELOPE_SCHEMA`], this compact, scalar-framed protocol
/// is transported unchanged to the seed's `compile_bundle` weave. The product
/// route never calls this module's M11 elaborator for a seed bundle.
pub const SEED_BUNDLE_SCHEMA: &str = "aether.seed-bundle/v1";

/// ADR-128 deliberately proves one real library-to-entry edge, not a general graph.
pub const SEED_BUNDLE_MAX_UNITS: usize = 2;

/// ADR-129 seed-native three-unit transitive source-bundle schema.
///
/// This remains a closed profile: a foundation library, a bridge library that
/// imports it, and one entry that imports the bridge. It is not a general M11
/// or M22 graph resolver.
pub const SEED_BUNDLE_CHAIN_SCHEMA: &str = "aether.seed-bundle/v2";

/// ADR-129 deliberately proves one transitive library chain, not arbitrary graphs.
pub const SEED_BUNDLE_CHAIN_MAX_UNITS: usize = 3;

/// ADR-130 seed-native four-unit fan-in source-bundle schema.
///
/// This remains a closed profile: two independent leaf libraries feed one
/// merge library, which is imported by one entry. It is not a general M11 or
/// M22 graph resolver.
pub const SEED_BUNDLE_FANIN_SCHEMA: &str = "aether.seed-bundle/v3";

/// ADR-130 deliberately proves one two-import merge, not arbitrary fan-in.
pub const SEED_BUNDLE_FANIN_MAX_UNITS: usize = 4;

/// Bounded source payload per unit, counted as Unicode scalar values.
pub const SEED_BUNDLE_MAX_UNIT_SCALARS: usize = 16_384;

/// Bounded aggregate source payload, counted as Unicode scalar values.
pub const SEED_BUNDLE_MAX_TOTAL_SCALARS: usize = 32_768;

/// Bounded aggregate source payload for the three-unit ADR-129 chain.
pub const SEED_BUNDLE_CHAIN_MAX_TOTAL_SCALARS: usize = 49_152;

/// Bounded aggregate source payload for the four-unit ADR-130 fan-in profile.
pub const SEED_BUNDLE_FANIN_MAX_TOTAL_SCALARS: usize = 65_536;

/// Bounded complete `aether.seed-bundle/v1` wire payload, including headers.
///
/// The slack covers two path-bearing headers and decimal scalar counts while
/// remaining small enough for the seed to reject oversized untrusted Text before
/// extracting either source unit.
pub const SEED_BUNDLE_MAX_WIRE_SCALARS: usize = 33_280;

/// Bounded complete aether.seed-bundle/v2 wire payload, including headers.
///
/// The additional fixed header/path allowance is intentionally small and does
/// not create an arbitrary-unit transport surface.
pub const SEED_BUNDLE_CHAIN_MAX_WIRE_SCALARS: usize = 49_920;

/// Bounded complete aether.seed-bundle/v3 wire payload, including headers.
///
/// The profile admits exactly four scalar-framed units; this cap leaves only a
/// bounded header allowance and never creates an arbitrary-unit transport.
pub const SEED_BUNDLE_FANIN_MAX_WIRE_SCALARS: usize = 66_560;

/// A structurally decoded seed bundle.
///
/// This is an authoring/test framing utility. Product bundle compilation sends
/// the original Text directly to the verified seed and does not use this
/// decoder to elaborate source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedBundle {
    pub entry_path: String,
    pub units: Vec<(String, String)>,
}

#[derive(Clone, Copy)]
struct SeedBundleFrameProfile {
    schema: &'static str,
    label: &'static str,
    required_units: &'static str,
    max_units: usize,
    max_total_scalars: usize,
    max_wire_scalars: usize,
}

const SEED_BUNDLE_PROFILE: SeedBundleFrameProfile = SeedBundleFrameProfile {
    schema: SEED_BUNDLE_SCHEMA,
    label: "seed bundle",
    required_units: "exactly one library and one entry unit",
    max_units: SEED_BUNDLE_MAX_UNITS,
    max_total_scalars: SEED_BUNDLE_MAX_TOTAL_SCALARS,
    max_wire_scalars: SEED_BUNDLE_MAX_WIRE_SCALARS,
};

const SEED_BUNDLE_CHAIN_PROFILE: SeedBundleFrameProfile = SeedBundleFrameProfile {
    schema: SEED_BUNDLE_CHAIN_SCHEMA,
    label: "seed bundle chain",
    required_units: "exactly two libraries and one entry unit",
    max_units: SEED_BUNDLE_CHAIN_MAX_UNITS,
    max_total_scalars: SEED_BUNDLE_CHAIN_MAX_TOTAL_SCALARS,
    max_wire_scalars: SEED_BUNDLE_CHAIN_MAX_WIRE_SCALARS,
};

const SEED_BUNDLE_FANIN_PROFILE: SeedBundleFrameProfile = SeedBundleFrameProfile {
    schema: SEED_BUNDLE_FANIN_SCHEMA,
    label: "seed bundle fan-in",
    required_units: "exactly three libraries and one entry unit",
    max_units: SEED_BUNDLE_FANIN_MAX_UNITS,
    max_total_scalars: SEED_BUNDLE_FANIN_MAX_TOTAL_SCALARS,
    max_wire_scalars: SEED_BUNDLE_FANIN_MAX_WIRE_SCALARS,
};

/// Encode a bounded ADR-128 seed bundle without inspecting Aether syntax.
///
/// Source text remains opaque to the host. The encoder validates only the
/// deterministic wire framing, safe path identity, and explicit two-unit order;
/// `compile_bundle` owns language parsing and elaboration.
pub fn encode_seed_bundle(
    entry_path: &str,
    units: &[(String, String)],
) -> Result<String, ProjectError> {
    encode_seed_bundle_profile(&SEED_BUNDLE_PROFILE, entry_path, units)
}

/// Decode a bounded ADR-128 seed bundle without inspecting Aether syntax.
///
/// This is intentionally not used by [`crate::compile_product_bytecode`] for
/// `aether.seed-bundle/v1`: decoding is provided for deterministic fixtures,
/// tooling, and independent test references only.
pub fn decode_seed_bundle(bundle: &str) -> Result<SeedBundle, ProjectError> {
    decode_seed_bundle_profile(&SEED_BUNDLE_PROFILE, bundle)
}

/// Encode the ADR-129 three-unit transitive seed bundle without parsing source.
///
/// The host validates only bounded framing, safe source identity paths, and the
/// fixed dependency order. The seed owns all Aether parsing and elaboration on
/// the production route.
pub fn encode_seed_bundle_chain(
    entry_path: &str,
    units: &[(String, String)],
) -> Result<String, ProjectError> {
    encode_seed_bundle_profile(&SEED_BUNDLE_CHAIN_PROFILE, entry_path, units)
}

/// Decode an ADR-129 three-unit transitive seed bundle without parsing source.
///
/// This framing utility is for deterministic authoring and independent tests;
/// the product compile path sends the original Text directly to the seed.
pub fn decode_seed_bundle_chain(bundle: &str) -> Result<SeedBundle, ProjectError> {
    decode_seed_bundle_profile(&SEED_BUNDLE_CHAIN_PROFILE, bundle)
}

/// Encode the ADR-130 four-unit fan-in seed bundle without parsing source.
///
/// The host checks only deterministic framing, safe source identity paths, and
/// fixed four-unit order. The verified seed owns the profile's source parsing,
/// import validation, mangling, and call rewriting.
pub fn encode_seed_bundle_fanin(
    entry_path: &str,
    units: &[(String, String)],
) -> Result<String, ProjectError> {
    encode_seed_bundle_profile(&SEED_BUNDLE_FANIN_PROFILE, entry_path, units)
}

/// Decode an ADR-130 four-unit fan-in seed bundle without parsing source.
///
/// This is an authoring and independent-test utility only. Product compilation
/// transports the original caller-selected Text directly to `compile_bundle`.
pub fn decode_seed_bundle_fanin(bundle: &str) -> Result<SeedBundle, ProjectError> {
    decode_seed_bundle_profile(&SEED_BUNDLE_FANIN_PROFILE, bundle)
}

fn encode_seed_bundle_profile(
    profile: &SeedBundleFrameProfile,
    entry_path: &str,
    units: &[(String, String)],
) -> Result<String, ProjectError> {
    validate_seed_bundle_profile(profile, entry_path, units)?;
    let mut bundle = format!("{}\nentry {entry_path}\n", profile.schema);
    for (path, source) in units {
        let scalar_count = source.chars().count();
        bundle.push_str(&format!("unit {path} {scalar_count}\n"));
        bundle.push_str(source);
        bundle.push('\n');
    }
    Ok(bundle)
}

fn decode_seed_bundle_profile(
    profile: &SeedBundleFrameProfile,
    bundle: &str,
) -> Result<SeedBundle, ProjectError> {
    let total_scalars = bundle.chars().count();
    if total_scalars > profile.max_wire_scalars {
        return Err(module_error(
            "AE-MOD-001",
            format!("{} exceeds its bounded wire-size limit", profile.label),
        ));
    }

    let mut cursor = 0_usize;
    let magic = take_seed_bundle_line(bundle, &mut cursor)?;
    if magic != profile.schema {
        return Err(module_error(
            "AE-MOD-001",
            format!("unsupported {} schema {magic:?}", profile.label),
        ));
    }
    let entry_line = take_seed_bundle_line(bundle, &mut cursor)?;
    let entry_path = entry_line
        .strip_prefix("entry ")
        .filter(|path| !path.is_empty())
        .ok_or_else(|| {
            module_error(
                "AE-MOD-001",
                format!("{} requires `entry <path>`", profile.label),
            )
        })?
        .to_owned();

    let mut units = Vec::new();
    while cursor < bundle.len() {
        let header = take_seed_bundle_line(bundle, &mut cursor)?;
        let (path, scalar_count) = parse_seed_bundle_unit_header(header)?;
        let source = take_seed_bundle_scalars(bundle, &mut cursor, scalar_count)?;
        units.push((path, source));
    }
    validate_seed_bundle_profile(profile, &entry_path, &units)?;
    Ok(SeedBundle { entry_path, units })
}

fn validate_seed_bundle_profile(
    profile: &SeedBundleFrameProfile,
    entry_path: &str,
    units: &[(String, String)],
) -> Result<(), ProjectError> {
    if units.len() != profile.max_units {
        return Err(module_error(
            "AE-MOD-006",
            format!(
                "{} profile requires {}",
                profile.label, profile.required_units
            ),
        ));
    }
    validate_seed_bundle_path(entry_path)?;
    let mut wire_scalars = profile
        .schema
        .chars()
        .count()
        .checked_add(1)
        .and_then(|value| value.checked_add("entry ".chars().count()))
        .and_then(|value| value.checked_add(entry_path.chars().count()))
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            module_error(
                "AE-MOD-001",
                format!("{} wire-size overflow", profile.label),
            )
        })?;
    let mut seen_paths = BTreeSet::new();
    let mut total_scalars = 0_usize;
    for (path, source) in units {
        validate_seed_bundle_path(path)?;
        if !seen_paths.insert(path) {
            return Err(module_error(
                "AE-MOD-005",
                format!("{} repeats unit path {path}", profile.label),
            ));
        }
        let scalar_count = source.chars().count();
        if scalar_count == 0 {
            return Err(module_error(
                "AE-MOD-001",
                format!("{} unit {path} is empty", profile.label),
            ));
        }
        if scalar_count > SEED_BUNDLE_MAX_UNIT_SCALARS {
            return Err(module_error(
                "AE-MOD-001",
                format!(
                    "{} unit {path} exceeds the {}-scalar limit",
                    profile.label, SEED_BUNDLE_MAX_UNIT_SCALARS
                ),
            ));
        }
        total_scalars = total_scalars.checked_add(scalar_count).ok_or_else(|| {
            module_error(
                "AE-MOD-001",
                format!("{} aggregate scalar count overflow", profile.label),
            )
        })?;
        let count_scalars = scalar_count.to_string().chars().count();
        wire_scalars = wire_scalars
            .checked_add("unit ".chars().count())
            .and_then(|value| value.checked_add(path.chars().count()))
            .and_then(|value| value.checked_add(1))
            .and_then(|value| value.checked_add(count_scalars))
            .and_then(|value| value.checked_add(1))
            .and_then(|value| value.checked_add(scalar_count))
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                module_error(
                    "AE-MOD-001",
                    format!("{} wire-size overflow", profile.label),
                )
            })?;
    }
    if total_scalars > profile.max_total_scalars {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "{} exceeds the {}-scalar aggregate limit",
                profile.label, profile.max_total_scalars
            ),
        ));
    }
    if wire_scalars > profile.max_wire_scalars {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "{} exceeds the {}-scalar wire limit",
                profile.label, profile.max_wire_scalars
            ),
        ));
    }
    if units.last().map(|(path, _)| path.as_str()) != Some(entry_path) {
        return Err(module_error(
            "AE-MOD-006",
            format!("{} entry path must name its final unit", profile.label),
        ));
    }
    Ok(())
}

fn validate_seed_bundle_path(path: &str) -> Result<(), ProjectError> {
    validate_unit_path(path).map_err(|error| {
        module_error(
            "AE-MOD-002",
            format!("seed bundle unit path {path}: {}", error.message),
        )
    })?;
    let stem = path.strip_suffix(".ae").ok_or_else(|| {
        module_error(
            "AE-MOD-002",
            format!("seed bundle unit path {path} must end with .ae"),
        )
    })?;
    if stem.is_empty()
        || !stem
            .bytes()
            .all(|byte| matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'_' | b'/'))
        || stem.starts_with('/')
        || stem.ends_with('/')
        || stem.contains("//")
    {
        return Err(module_error(
            "AE-MOD-002",
            format!(
                "seed bundle unit path {path} must use lowercase ASCII names, digits, underscores, and forward slashes"
            ),
        ));
    }
    Ok(())
}

fn take_seed_bundle_line<'a>(bundle: &'a str, cursor: &mut usize) -> Result<&'a str, ProjectError> {
    let remaining = bundle.get(*cursor..).ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed bundle cursor is not on a UTF-8 boundary",
        )
    })?;
    let line_end = remaining
        .find('\n')
        .ok_or_else(|| module_error("AE-MOD-001", "seed bundle header must end with LF"))?;
    let line = &remaining[..line_end];
    *cursor = cursor
        .checked_add(line_end + 1)
        .ok_or_else(|| module_error("AE-MOD-001", "seed bundle cursor overflow"))?;
    Ok(line)
}

fn parse_seed_bundle_unit_header(header: &str) -> Result<(String, usize), ProjectError> {
    let rest = header
        .strip_prefix("unit ")
        .ok_or_else(|| module_error("AE-MOD-001", "seed bundle expected a unit header"))?;
    let (path, count_text) = rest.split_once(' ').ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed bundle unit header requires path and scalar count",
        )
    })?;
    if path.is_empty()
        || count_text.is_empty()
        || count_text.len() > 5
        || !count_text.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(module_error(
            "AE-MOD-001",
            "seed bundle unit scalar count must be one to five ASCII digits",
        ));
    }
    let scalar_count = count_text.parse::<usize>().map_err(|_| {
        module_error(
            "AE-MOD-001",
            "seed bundle unit scalar count is not representable",
        )
    })?;
    Ok((path.to_owned(), scalar_count))
}

fn take_seed_bundle_scalars(
    bundle: &str,
    cursor: &mut usize,
    scalar_count: usize,
) -> Result<String, ProjectError> {
    let remaining = bundle.get(*cursor..).ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed bundle cursor is not on a UTF-8 boundary",
        )
    })?;
    let mut payload_end = 0_usize;
    let mut observed = 0_usize;
    for (offset, scalar) in remaining.char_indices() {
        if observed == scalar_count {
            break;
        }
        payload_end = offset + scalar.len_utf8();
        observed += 1;
    }
    if observed != scalar_count {
        return Err(module_error(
            "AE-MOD-001",
            "seed bundle unit payload is shorter than its scalar count",
        ));
    }
    let payload = remaining[..payload_end].to_owned();
    *cursor = cursor
        .checked_add(payload_end)
        .ok_or_else(|| module_error("AE-MOD-001", "seed bundle cursor overflow"))?;
    if bundle.as_bytes().get(*cursor) != Some(&b'\n') {
        return Err(module_error(
            "AE-MOD-001",
            "seed bundle unit payload must be followed by one LF separator",
        ));
    }
    *cursor = cursor
        .checked_add(1)
        .ok_or_else(|| module_error("AE-MOD-001", "seed bundle cursor overflow"))?;
    Ok(payload)
}

/// General seed-owned module-catalog schema (GSM-001).
///
/// The catalog carries a closed, explicitly selected set of source units as
/// scalar-indexed opaque Text. The host validates only framing, manifest-derived
/// identities, roles, locks, and path confinement; the seed compile_modules
/// forge entry owns all Aether import parsing, graph resolution, mangling, and
/// elaboration.
pub const SEED_MODULES_SCHEMA: &str = "aether.seed-modules/v1";

/// Maximum semantic source units admitted by one general seed module catalog.
///
/// This is a graph-size resource limit, not a fixed topology: every acyclic
/// M11/M22 graph within the catalog limits is eligible.
pub const SEED_MODULES_MAX_UNITS: usize = 256;

/// Maximum M22 package names authorized in one catalog.
pub const SEED_MODULES_MAX_PACKAGES: usize = 64;

/// Maximum scalar payload for one source unit.
pub const SEED_MODULES_MAX_UNIT_SCALARS: usize = 16_384;

/// Maximum aggregate source scalar payload for one catalog.
pub const SEED_MODULES_MAX_TOTAL_SCALARS: usize = 196_608;

/// Maximum complete scalar wire payload, including deterministic metadata.
///
/// The Aether invocation ABI accepts at most 1,000,000 UTF-8 bytes of Text.
/// A Unicode scalar occupies at most four UTF-8 bytes, so this scalar cap is
/// intentionally no larger than one quarter of that hard runtime limit.
pub const SEED_MODULES_MAX_WIRE_SCALARS: usize = 250_000;

const _: () = assert!(SEED_MODULES_MAX_WIRE_SCALARS <= crate::MAX_TEXT_BYTES / 4);

/// Maximum scalar length of a manifest-derived catalog identity.
pub const SEED_MODULES_MAX_KEY_SCALARS: usize = 256;

/// One source unit carried by a SeedModuleCatalog.
///
/// key is either a project-relative unit path or the M22 identity
/// package::project-relative-unit-path. The source remains opaque to this Rust
/// framing layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedModuleUnit {
    pub key: String,
    pub role: ProjectUnitRole,
    pub source: String,
}

/// A deterministic, scalar-indexed, closed module catalog.
///
/// This is a framing value for authoring, project loading, and independent
/// tests. Product compilation deliberately forwards its original text directly
/// to the verified seed compile_modules weave rather than decoding or
/// elaborating it in Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedModuleCatalog {
    pub entry_key: String,
    pub allowed_packages: Vec<String>,
    pub units: Vec<SeedModuleUnit>,
}

/// Encode a deterministic general seed module catalog without inspecting
/// Aether source syntax.
///
/// Metadata uses LF-only ASCII lines, followed by one concatenated source
/// payload. Scalar counts make source boundaries unambiguous even when sources
/// contain their own newlines or non-ASCII Text.
pub fn encode_seed_module_catalog(
    entry_key: &str,
    allowed_packages: &[String],
    units: &[SeedModuleUnit],
) -> Result<String, ProjectError> {
    let mut canonical_packages = allowed_packages.to_vec();
    canonical_packages.sort_unstable();
    let catalog = SeedModuleCatalog {
        entry_key: entry_key.to_owned(),
        allowed_packages: canonical_packages,
        units: units.to_vec(),
    };
    validate_seed_module_catalog(&catalog)?;

    let mut output = String::new();
    output.push_str(SEED_MODULES_SCHEMA);
    output.push('\n');
    output.push_str("entry ");
    output.push_str(&catalog.entry_key);
    output.push('\n');
    output.push_str("packages ");
    output.push_str(&catalog.allowed_packages.len().to_string());
    output.push('\n');
    for package in &catalog.allowed_packages {
        output.push_str("package ");
        output.push_str(package);
        output.push('\n');
    }
    output.push_str("units ");
    output.push_str(&catalog.units.len().to_string());
    output.push('\n');
    for unit in &catalog.units {
        output.push_str("unit ");
        output.push_str(&unit.key);
        output.push(' ');
        output.push_str(seed_module_role_name(unit.role));
        output.push(' ');
        output.push_str(&unit.source.chars().count().to_string());
        output.push('\n');
    }
    output.push_str("source\n");
    for unit in &catalog.units {
        output.push_str(&unit.source);
    }

    if output.chars().count() > SEED_MODULES_MAX_WIRE_SCALARS {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-scalar wire limit",
                SEED_MODULES_MAX_WIRE_SCALARS
            ),
        ));
    }
    Ok(output)
}

/// Decode a general seed module catalog for deterministic tooling and
/// independent reference tests.
///
/// Product compilation does not call this function: it passes original catalog
/// Text directly to the seed compile_modules entry.
pub fn decode_seed_module_catalog(bundle: &str) -> Result<SeedModuleCatalog, ProjectError> {
    if bundle.chars().count() > SEED_MODULES_MAX_WIRE_SCALARS {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-scalar wire limit",
                SEED_MODULES_MAX_WIRE_SCALARS
            ),
        ));
    }

    let mut cursor = 0_usize;
    let magic = take_seed_module_line(bundle, &mut cursor)?;
    if magic != SEED_MODULES_SCHEMA {
        return Err(module_error(
            "AE-MOD-001",
            format!("unsupported seed module schema {magic:?}"),
        ));
    }
    let entry_key = take_seed_module_line(bundle, &mut cursor)?
        .strip_prefix("entry ")
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            module_error(
                "AE-MOD-001",
                "seed module catalog requires entry plus unit identity",
            )
        })?
        .to_owned();
    let package_count = parse_seed_module_count(
        take_seed_module_line(bundle, &mut cursor)?
            .strip_prefix("packages ")
            .ok_or_else(|| {
                module_error("AE-MOD-001", "seed module catalog requires package count")
            })?,
        "package count",
    )?;
    if package_count > SEED_MODULES_MAX_PACKAGES {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-package limit",
                SEED_MODULES_MAX_PACKAGES
            ),
        ));
    }
    let mut allowed_packages = Vec::with_capacity(package_count);
    for _ in 0..package_count {
        let package = take_seed_module_line(bundle, &mut cursor)?
            .strip_prefix("package ")
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                module_error(
                    "AE-MOD-001",
                    "seed module catalog expected a package identity",
                )
            })?
            .to_owned();
        allowed_packages.push(package);
    }
    let unit_count = parse_seed_module_count(
        take_seed_module_line(bundle, &mut cursor)?
            .strip_prefix("units ")
            .ok_or_else(|| module_error("AE-MOD-001", "seed module catalog requires unit count"))?,
        "unit count",
    )?;
    if unit_count == 0 || unit_count > SEED_MODULES_MAX_UNITS {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog unit count must be in 1..={}",
                SEED_MODULES_MAX_UNITS
            ),
        ));
    }

    let mut metadata = Vec::with_capacity(unit_count);
    for _ in 0..unit_count {
        let header = take_seed_module_line(bundle, &mut cursor)?;
        metadata.push(parse_seed_module_unit_header(header)?);
    }
    if take_seed_module_line(bundle, &mut cursor)? != "source" {
        return Err(module_error(
            "AE-MOD-001",
            "seed module catalog requires one source payload marker",
        ));
    }

    let mut units = Vec::with_capacity(unit_count);
    for (key, role, scalar_count) in metadata {
        let source = take_seed_module_payload(bundle, &mut cursor, scalar_count)?;
        units.push(SeedModuleUnit { key, role, source });
    }
    if cursor != bundle.len() {
        return Err(module_error(
            "AE-MOD-001",
            "seed module catalog has trailing payload outside declared scalar counts",
        ));
    }

    let catalog = SeedModuleCatalog {
        entry_key,
        allowed_packages,
        units,
    };
    validate_seed_module_catalog(&catalog)?;
    Ok(catalog)
}

/// Validate catalog framing metadata without inspecting Aether source syntax.
pub fn validate_seed_module_catalog(catalog: &SeedModuleCatalog) -> Result<(), ProjectError> {
    validate_seed_module_key(&catalog.entry_key)?;
    if catalog.entry_key.contains("::") {
        return Err(module_error(
            "AE-MOD-006",
            "seed module catalog entry must be a local project unit",
        ));
    }
    if catalog.allowed_packages.len() > SEED_MODULES_MAX_PACKAGES {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-package limit",
                SEED_MODULES_MAX_PACKAGES
            ),
        ));
    }
    let mut package_names = BTreeSet::new();
    for package in &catalog.allowed_packages {
        if !is_seed_module_package_name(package) {
            return Err(module_error(
                "AE-MOD-001",
                format!("seed module catalog package {package:?} is invalid"),
            ));
        }
        if !package_names.insert(package.as_str()) {
            return Err(module_error(
                "AE-MOD-005",
                format!("seed module catalog repeats allowed package {package}"),
            ));
        }
    }
    if catalog.units.is_empty() || catalog.units.len() > SEED_MODULES_MAX_UNITS {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog unit count must be in 1..={}",
                SEED_MODULES_MAX_UNITS
            ),
        ));
    }

    let mut seen_keys = BTreeSet::new();
    let mut total_scalars = 0_usize;
    let mut entry_role = None;
    for unit in &catalog.units {
        validate_seed_module_key(&unit.key)?;
        if !seen_keys.insert(unit.key.as_str()) {
            return Err(module_error(
                "AE-MOD-005",
                format!("seed module catalog repeats unit identity {}", unit.key),
            ));
        }
        if let Some((package, _)) = unit.key.split_once("::") {
            if !package_names.contains(package) {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "seed module unit {} names unallowed package {package}",
                        unit.key
                    ),
                ));
            }
            if unit.role != ProjectUnitRole::Lib {
                return Err(module_error(
                    "AE-MOD-006",
                    format!("seed module foreign unit {} must have role lib", unit.key),
                ));
            }
        }
        let scalar_count = unit.source.chars().count();
        if scalar_count == 0 || scalar_count > SEED_MODULES_MAX_UNIT_SCALARS {
            return Err(module_error(
                "AE-MOD-001",
                format!(
                    "seed module unit {} must contain 1..={} source scalars",
                    unit.key, SEED_MODULES_MAX_UNIT_SCALARS
                ),
            ));
        }
        total_scalars = total_scalars.checked_add(scalar_count).ok_or_else(|| {
            module_error(
                "AE-MOD-001",
                "seed module catalog aggregate scalar count overflow",
            )
        })?;
        if unit.key == catalog.entry_key {
            entry_role = Some(unit.role);
        }
    }
    if total_scalars > SEED_MODULES_MAX_TOTAL_SCALARS {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-scalar aggregate source limit",
                SEED_MODULES_MAX_TOTAL_SCALARS
            ),
        ));
    }
    match entry_role {
        Some(ProjectUnitRole::Main | ProjectUnitRole::Test) => Ok(()),
        Some(ProjectUnitRole::Lib) => Err(module_error(
            "AE-MOD-006",
            format!(
                "seed module entry {} must have role main or test",
                catalog.entry_key
            ),
        )),
        None => Err(module_error(
            "AE-MOD-002",
            format!(
                "seed module entry {} is not present in the catalog",
                catalog.entry_key
            ),
        )),
    }
}

/// Frame one project or M22 workspace entry for the general seed module ABI.
///
/// This is deliberately a manifest-and-filesystem boundary, not an Aether
/// source elaborator. It validates caller-selected project identities, path
/// confinement, UTF-8, source-size limits, and direct workspace authority;
/// the verified seed owns import parsing, dependency resolution, namespace
/// rewriting, cycle detection, and source compilation.
pub fn encode_project_seed_module_catalog(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<String, ProjectError> {
    let entry = document
        .units
        .iter()
        .find(|unit| unit.path == entry_path)
        .ok_or_else(|| {
            module_error(
                "AE-MOD-002",
                format!("entry unit {entry_path} is not a project unit"),
            )
        })?;
    if entry.role != ProjectUnitRole::Main && entry.role != ProjectUnitRole::Test {
        return Err(module_error(
            "AE-MOD-006",
            format!("entry unit {entry_path} must have role main or test"),
        ));
    }
    if allowed_packages.len() > SEED_MODULES_MAX_PACKAGES {
        return Err(module_error(
            "AE-MOD-001",
            format!(
                "seed module catalog exceeds the {}-package limit",
                SEED_MODULES_MAX_PACKAGES
            ),
        ));
    }

    let mut units = Vec::with_capacity(document.units.len());
    for unit in &document.units {
        units.push(read_seed_module_catalog_unit(
            project_root,
            &unit.path,
            unit,
            None,
        )?);
    }

    for package_name in allowed_packages {
        if !is_seed_module_package_name(package_name) {
            return Err(module_error(
                "AE-MOD-001",
                format!("workspace package {package_name:?} is not a valid catalog identity"),
            ));
        }
        let package_root = package_roots.get(package_name).ok_or_else(|| {
            module_error(
                "AE-MOD-002",
                format!("workspace package {package_name} is not available"),
            )
        })?;
        let foreign_project = read_seed_module_project_document(package_root, package_name)?;
        for unit in foreign_project
            .units
            .iter()
            .filter(|unit| unit.role == ProjectUnitRole::Lib)
        {
            let key = format!("{package_name}::{}", unit.path);
            units.push(read_seed_module_catalog_unit(
                package_root,
                &key,
                unit,
                Some(package_name),
            )?);
        }
    }

    // The wire format is deterministic independently of manifest ordering.
    // This does not inspect or transform source payloads.
    units.sort_by(|left, right| left.key.cmp(&right.key));
    let packages: Vec<String> = allowed_packages.iter().cloned().collect();
    encode_seed_module_catalog(entry_path, &packages, &units)
}

fn read_seed_module_project_document(
    package_root: &Path,
    package_name: &str,
) -> Result<ProjectDocument, ProjectError> {
    let canonical_root = package_root.canonicalize().map_err(|error| {
        module_error(
            "AE-MOD-002",
            format!("workspace package {package_name} root is unavailable: {error}"),
        )
    })?;
    let project_file = canonical_root.join("aether.project.json");
    let bytes = fs::read(&project_file).map_err(|error| {
        module_error(
            "AE-MOD-002",
            format!("workspace package {package_name} missing aether.project.json: {error}"),
        )
    })?;
    let text = String::from_utf8(bytes).map_err(|_| {
        module_error(
            "AE-MOD-002",
            format!("workspace package {package_name} manifest is not valid UTF-8"),
        )
    })?;
    let document = parse_project_document(&text)?;
    if document.schema != PROJECT_SCHEMA_VERSION {
        return Err(module_error(
            "AE-MOD-002",
            format!("workspace package {package_name} has unsupported project schema"),
        ));
    }
    Ok(document)
}

fn read_seed_module_catalog_unit(
    project_root: &Path,
    key: &str,
    unit: &ProjectUnit,
    package_name: Option<&str>,
) -> Result<SeedModuleUnit, ProjectError> {
    let resolved = resolve_unit_path(project_root, &unit.path)?;
    let bytes = fs::read(&resolved).map_err(|error| {
        let location = match package_name {
            Some(package) => format!("workspace package {package} unit {}", unit.path),
            None => format!("unit {}", unit.path),
        };
        module_error(
            "AE-PROJECT-002",
            format!("could not read {location}: {error}"),
        )
    })?;
    let source = String::from_utf8(bytes).map_err(|_| {
        let location = match package_name {
            Some(package) => format!("workspace package {package} unit {}", unit.path),
            None => format!("unit {}", unit.path),
        };
        module_error("AE-PROJECT-004", format!("{location} is not valid UTF-8"))
    })?;
    Ok(SeedModuleUnit {
        key: key.to_owned(),
        role: unit.role,
        source,
    })
}

fn seed_module_role_name(role: ProjectUnitRole) -> &'static str {
    match role {
        ProjectUnitRole::Main => "main",
        ProjectUnitRole::Lib => "lib",
        ProjectUnitRole::Test => "test",
    }
}

fn parse_seed_module_role(value: &str) -> Result<ProjectUnitRole, ProjectError> {
    match value {
        "main" => Ok(ProjectUnitRole::Main),
        "lib" => Ok(ProjectUnitRole::Lib),
        "test" => Ok(ProjectUnitRole::Test),
        _ => Err(module_error(
            "AE-MOD-001",
            format!("seed module unit role {value:?} is invalid"),
        )),
    }
}

fn parse_seed_module_count(value: &str, label: &str) -> Result<usize, ProjectError> {
    if value.is_empty() || value.len() > 7 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(module_error(
            "AE-MOD-001",
            format!("seed module {label} must be one to seven ASCII digits"),
        ));
    }
    value.parse::<usize>().map_err(|_| {
        module_error(
            "AE-MOD-001",
            format!("seed module {label} is not representable"),
        )
    })
}

fn parse_seed_module_unit_header(
    header: &str,
) -> Result<(String, ProjectUnitRole, usize), ProjectError> {
    let rest = header
        .strip_prefix("unit ")
        .ok_or_else(|| module_error("AE-MOD-001", "seed module catalog expected a unit header"))?;
    let fields: Vec<&str> = rest.split(' ').collect();
    if fields.len() != 3 || fields.iter().any(|field| field.is_empty()) {
        return Err(module_error(
            "AE-MOD-001",
            "seed module unit header requires identity, role, and scalar count",
        ));
    }
    let role = parse_seed_module_role(fields[1])?;
    let scalar_count = parse_seed_module_count(fields[2], "unit scalar count")?;
    Ok((fields[0].to_owned(), role, scalar_count))
}

fn take_seed_module_line<'a>(bundle: &'a str, cursor: &mut usize) -> Result<&'a str, ProjectError> {
    let remaining = bundle.get(*cursor..).ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed module catalog cursor is not on a UTF-8 boundary",
        )
    })?;
    let line_end = remaining.find('\n').ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed module catalog metadata line must end with LF",
        )
    })?;
    let line = &remaining[..line_end];
    *cursor = cursor
        .checked_add(line_end + 1)
        .ok_or_else(|| module_error("AE-MOD-001", "seed module catalog cursor overflow"))?;
    Ok(line)
}

fn take_seed_module_payload(
    bundle: &str,
    cursor: &mut usize,
    scalar_count: usize,
) -> Result<String, ProjectError> {
    let remaining = bundle.get(*cursor..).ok_or_else(|| {
        module_error(
            "AE-MOD-001",
            "seed module catalog payload cursor is not on a UTF-8 boundary",
        )
    })?;
    let mut payload_end = 0_usize;
    let mut observed = 0_usize;
    for (offset, scalar) in remaining.char_indices() {
        if observed == scalar_count {
            break;
        }
        payload_end = offset + scalar.len_utf8();
        observed += 1;
    }
    if observed != scalar_count {
        return Err(module_error(
            "AE-MOD-001",
            "seed module unit payload is shorter than its scalar count",
        ));
    }
    let payload = remaining[..payload_end].to_owned();
    *cursor = cursor
        .checked_add(payload_end)
        .ok_or_else(|| module_error("AE-MOD-001", "seed module catalog payload cursor overflow"))?;
    Ok(payload)
}

fn validate_seed_module_key(key: &str) -> Result<(), ProjectError> {
    if key.chars().count() > SEED_MODULES_MAX_KEY_SCALARS {
        return Err(module_error(
            "AE-MOD-002",
            format!(
                "seed module unit identity exceeds the {}-scalar limit",
                SEED_MODULES_MAX_KEY_SCALARS
            ),
        ));
    }
    if let Some((package, path)) = key.split_once("::") {
        if package.is_empty()
            || path.is_empty()
            || path.contains("::")
            || !is_seed_module_package_name(package)
        {
            return Err(module_error(
                "AE-MOD-002",
                format!("seed module unit identity {key:?} is invalid"),
            ));
        }
        validate_unit_path(path).map_err(|error| {
            module_error(
                "AE-MOD-002",
                format!("seed module foreign unit {key}: {}", error.message),
            )
        })
    } else {
        validate_unit_path(key).map_err(|error| {
            module_error(
                "AE-MOD-002",
                format!("seed module local unit {key}: {}", error.message),
            )
        })
    }
}

fn is_seed_module_package_name(value: &str) -> bool {
    let mut characters = value.chars();
    matches!(characters.next(), Some(first) if first.is_ascii_alphabetic())
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// Encode units as a multi-source forge envelope (JSON Text).
pub fn encode_multi_source_envelope(units: &[(String, String)]) -> Result<String, ProjectError> {
    debug_assert!(
        crate::product_multi_source_forge_envelope(),
        "ADR-075: multi-source envelope tracker"
    );
    if units.is_empty() {
        return Err(module_error(
            "AE-MOD-006",
            "multi-source envelope requires at least one unit",
        ));
    }
    let payload = serde_json::json!({
        "schema": MULTI_SOURCE_ENVELOPE_SCHEMA,
        "units": units.iter().map(|(path, source)| {
            serde_json::json!({ "path": path, "source": source })
        }).collect::<Vec<_>>(),
    });
    serde_json::to_string_pretty(&payload).map_err(|error| {
        module_error(
            "AE-MOD-001",
            format!("could not encode multi-source envelope: {error}"),
        )
    })
}

/// Decode a multi-source forge envelope into (path, source) units.
pub fn decode_multi_source_envelope(text: &str) -> Result<Vec<(String, String)>, ProjectError> {
    let value: serde_json::Value = serde_json::from_str(text).map_err(|error| {
        module_error(
            "AE-MOD-001",
            format!("multi-source envelope is not JSON: {error}"),
        )
    })?;
    let schema = value.get("schema").and_then(|v| v.as_str()).unwrap_or("");
    if schema != MULTI_SOURCE_ENVELOPE_SCHEMA {
        return Err(module_error(
            "AE-MOD-001",
            format!("unsupported multi-source schema {schema:?}"),
        ));
    }
    let units = value
        .get("units")
        .and_then(|v| v.as_array())
        .ok_or_else(|| module_error("AE-MOD-001", "multi-source envelope missing units"))?;
    let mut out = Vec::new();
    for unit in units {
        let path = unit
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| module_error("AE-MOD-001", "unit missing path"))?
            .to_owned();
        let source = unit
            .get("source")
            .and_then(|v| v.as_str())
            .ok_or_else(|| module_error("AE-MOD-001", "unit missing source"))?
            .to_owned();
        if path.is_empty() {
            return Err(module_error("AE-MOD-001", "unit path must be non-empty"));
        }
        out.push((path, source));
    }
    if out.is_empty() {
        return Err(module_error(
            "AE-MOD-006",
            "multi-source envelope requires at least one unit",
        ));
    }
    Ok(out)
}

/// GSM-001 product forge for in-memory local units (no project file).
///
/// The host frames caller-supplied unit identities and opaque source Text; the
/// seed resolves and elaborates the graph. Cross-package imports are rejected
/// here because this API has no explicit package authority set.
pub fn compile_product_multi_unit(
    units: &[(String, String, ProjectUnitRole)],
    entry_path: &str,
) -> Result<Vec<u8>, ProjectError> {
    debug_assert!(
        crate::product_multi_source_forge_envelope(),
        "ADR-075: multi-unit product forge"
    );
    let catalog_units: Vec<SeedModuleUnit> = units
        .iter()
        .map(|(key, source, role)| SeedModuleUnit {
            key: key.clone(),
            role: *role,
            source: source.clone(),
        })
        .collect();
    let catalog = encode_seed_module_catalog(entry_path, &[], &catalog_units)?;
    crate::compile_product_seed_modules(&catalog).map_err(|error| {
        module_error(
            "AE-SEED-001",
            format!("product seed module compile failed: {error}"),
        )
    })
}

/// Elaborate in-memory units (path, source, role) into one single-world program.
pub fn elaborate_in_memory_units(
    units: &[(String, String, ProjectUnitRole)],
    entry_path: &str,
) -> Result<String, ProjectError> {
    if units.is_empty() {
        return Err(module_error(
            "AE-MOD-006",
            "in-memory multi-unit graph is empty",
        ));
    }
    let mut modules = BTreeMap::new();
    for (path, source, role) in units {
        if modules.contains_key(path) {
            return Err(module_error(
                "AE-MOD-005",
                format!("duplicate in-memory unit path {path}"),
            ));
        }
        let parsed = parse_module_source(path, source, *role)?;
        if parsed.imports.iter().any(|import| import.package.is_some()) {
            return Err(module_error(
                "AE-MOD-002",
                "in-memory multi-unit pilot rejects cross-package imports",
            ));
        }
        modules.insert(path.clone(), parsed);
    }
    // Ensure imported same-package paths exist in the provided unit set.
    for module in modules.values() {
        for import in &module.imports {
            if !modules.contains_key(&import.path) {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "import path {} is not a provided in-memory unit",
                        import.path
                    ),
                ));
            }
        }
    }
    elaborate_modules_map(&modules, entry_path)
}

/// ADR-094: multi-source unit inventory surface (host path; seed-native still false).
#[must_use]
pub const fn product_multi_source_unit_surface_api() -> bool {
    true
}

/// ADR-098: multi-source unit digests (sha256 of source text) without forge.
#[must_use]
pub const fn product_multi_source_unit_digests_api() -> bool {
    true
}

/// One unit in a multi-source envelope inventory (ADR-094/098).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSourceUnitSurface {
    pub path: String,
    pub role: ProjectUnitRole,
    pub source_byte_len: usize,
    pub has_world: bool,
    /// SHA-256 hex of unit source bytes (ADR-098).
    pub source_sha256: String,
}

/// Multi-source envelope inventory without forge (ADR-094/098 tooling surface).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSourceEnvelopeSurface {
    pub schema: String,
    pub units: Vec<MultiSourceUnitSurface>,
    pub entry_path: Option<String>,
    pub unit_count: usize,
}

/// Inventory multi-source envelope units without elaborating or forging.
pub fn product_multi_source_unit_surface(
    envelope: &str,
) -> Result<MultiSourceEnvelopeSurface, ProjectError> {
    debug_assert!(
        product_multi_source_unit_surface_api(),
        "ADR-094: multi-source unit surface"
    );
    let decoded = decode_multi_source_envelope(envelope)?;
    let mut units = Vec::new();
    let mut entry_path = None;
    for (path, source) in decoded {
        let role = if path == "main.ae"
            || path.ends_with("/main.ae")
            || path.ends_with("\\main.ae")
            || path.ends_with("/src/main.ae")
        {
            if entry_path.is_some() {
                return Err(module_error(
                    "AE-MOD-006",
                    "multi-source envelope has multiple main units",
                ));
            }
            entry_path = Some(path.clone());
            ProjectUnitRole::Main
        } else {
            ProjectUnitRole::Lib
        };
        let has_world = source
            .lines()
            .any(|line| line.trim_start().starts_with("world "));
        let source_sha256 = crate::sha256_hex(source.as_bytes());
        units.push(MultiSourceUnitSurface {
            path,
            role,
            source_byte_len: source.len(),
            has_world,
            source_sha256,
        });
    }
    let unit_count = units.len();
    Ok(MultiSourceEnvelopeSurface {
        schema: MULTI_SOURCE_ENVELOPE_SCHEMA.to_owned(),
        units,
        entry_path,
        unit_count,
    })
}

/// Host multi-file product forge from a multi-source envelope Text.
///
/// Role heuristic: unit whose path ends with `main.ae` or is named `main.ae` is
/// main; others are lib. Exactly one main is required.
pub fn compile_product_multi_source_envelope(envelope: &str) -> Result<Vec<u8>, ProjectError> {
    let decoded = decode_multi_source_envelope(envelope)?;
    let mut units = Vec::new();
    let mut entry = None;
    for (path, source) in decoded {
        let role = if path == "main.ae"
            || path.ends_with("/main.ae")
            || path.ends_with("\\main.ae")
            || path.ends_with("/src/main.ae")
        {
            if entry.is_some() {
                return Err(module_error(
                    "AE-MOD-006",
                    "multi-source envelope has multiple main units",
                ));
            }
            entry = Some(path.clone());
            ProjectUnitRole::Main
        } else {
            ProjectUnitRole::Lib
        };
        units.push((path, source, role));
    }
    let entry_path = entry.ok_or_else(|| {
        module_error(
            "AE-MOD-006",
            "multi-source envelope requires a main.ae entry unit",
        )
    })?;
    compile_product_multi_unit(&units, &entry_path)
}

fn elaborate_modules_map(
    modules: &BTreeMap<String, ParsedModule>,
    entry_path: &str,
) -> Result<String, ProjectError> {
    if !modules.contains_key(entry_path) {
        return Err(module_error(
            "AE-MOD-002",
            format!("entry unit {entry_path} is not a project unit"),
        ));
    }
    let entry_role = modules[entry_path].role;
    if entry_role != ProjectUnitRole::Main && entry_role != ProjectUnitRole::Test {
        return Err(module_error(
            "AE-MOD-006",
            format!("entry unit {entry_path} must have role main or test"),
        ));
    }
    let order = closed_cone(entry_path, modules)?;

    // World uniqueness
    let mut worlds = BTreeSet::new();
    for path in &order {
        let world = &modules[path].world;
        if !worlds.insert(world.clone()) {
            return Err(module_error(
                "AE-MOD-005",
                format!("duplicate world name {world} in the module graph"),
            ));
        }
    }

    // Build export tables: path -> (weave -> mangled)
    let mut export_tables: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut all_local_maps: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for path in &order {
        let module = &modules[path];
        let mut local = BTreeMap::new();
        let mut exports = BTreeMap::new();
        for (weave, exported) in &module.weaves {
            let mangled = if *weave == "main" {
                "main".to_owned()
            } else {
                mangle_weave(path, weave)
            };
            local.insert(weave.clone(), mangled.clone());
            if *exported {
                exports.insert(weave.clone(), mangled);
            }
        }
        all_local_maps.insert(path.clone(), local);
        export_tables.insert(path.clone(), exports);
    }

    // Alias maps per module: "alias.name" -> mangled, and validate exports
    let mut import_maps: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for path in &order {
        let module = &modules[path];
        let mut aliases = BTreeSet::new();
        let mut map = BTreeMap::new();
        for import in &module.imports {
            if !aliases.insert(import.alias.clone()) {
                return Err(module_error(
                    "AE-MOD-005",
                    format!("module {path} reuses import alias {}", import.alias),
                ));
            }
            let target_key = import.graph_key();
            let exports = export_tables.get(&target_key).ok_or_else(|| {
                module_error(
                    "AE-MOD-002",
                    format!("import {target_key} missing export table"),
                )
            })?;
            for (weave, mangled) in exports {
                map.insert(format!("{}.{}", import.alias, weave), mangled.clone());
            }
        }
        import_maps.insert(path.clone(), map);
    }

    // Detect private access: any call alias.private left unmapped becomes invalid later;
    // proactively scan for call alias.x where x not exported
    for path in &order {
        let module = &modules[path];
        for import in &module.imports {
            let target_key = import.graph_key();
            let exports = &export_tables[&target_key];
            let needle = format!("call {}.", import.alias);
            for line in module.body_source.lines() {
                if let Some(pos) = line.find(&needle) {
                    let after = &line[pos + needle.len()..];
                    let name: String = after
                        .chars()
                        .take_while(|c| c.is_ascii_lowercase() || *c == '_' || c.is_ascii_digit())
                        .collect();
                    if !name.is_empty() && !exports.contains_key(&name) {
                        return Err(module_error(
                            "AE-MOD-003",
                            format!(
                                "module {path} calls {}.{} but that weave is not exported from {}",
                                import.alias, name, import.path
                            ),
                        ));
                    }
                }
            }
        }
    }

    let entry_world = modules[entry_path].world.clone();
    let mut elaborated = format!("world {entry_world}\n");

    // Emit libs first (order is post-order: deps before dependents), then entry last pieces
    // order from dfs is post-order (deps first). Entry is last.
    for path in &order {
        let module = &modules[path];
        let local_map = &all_local_maps[path];
        let import_map = &import_maps[path];
        let mut body = rename_weaves_in_body(&module.body_source, local_map);
        body = rewrite_calls(&body, local_map, import_map);
        if body.trim().is_empty() {
            continue;
        }
        elaborated.push('\n');
        elaborated.push_str(body.trim_end());
        elaborated.push('\n');
    }

    Ok(elaborated)
}

/// Multi-module project build → verified AETH bytes (M11/M22 GSM-001).
///
/// The host frames the manifest-selected source catalog; the seed owns import
/// parsing, graph resolution, namespace elaboration, and AETH emission. The
/// bootstrap elaborator is reference/oracle only (ADR-045).
pub fn compile_project_modules(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<Vec<u8>, ProjectError> {
    compile_project_modules_with_packages(
        project_root,
        document,
        &BTreeMap::new(),
        &BTreeSet::new(),
    )
}

/// Like [`compile_project_modules`] with workspace package roots (M22).
pub fn compile_project_modules_with_packages(
    project_root: &Path,
    document: &ProjectDocument,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<Vec<u8>, ProjectError> {
    let entry = document
        .units
        .iter()
        .find(|unit| unit.role == ProjectUnitRole::Main)
        .ok_or_else(|| module_error("AE-MOD-006", "project has no main unit"))?;
    compile_project_entry_with_packages(
        project_root,
        document,
        &entry.path,
        package_roots,
        allowed_packages,
    )
}

/// Compile a selectable main/test entry on the seed product path (M11b).
pub fn compile_project_entry(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
) -> Result<Vec<u8>, ProjectError> {
    compile_project_entry_with_packages(
        project_root,
        document,
        entry_path,
        &BTreeMap::new(),
        &BTreeSet::new(),
    )
}

/// Compile a selectable entry with workspace package roots.
///
/// Product path: manifest/path/UTF-8 framing plus the verified seed-owned
/// module catalog compiler. No bootstrap dual-compare gate and no Rust Aether
/// source parse or elaboration occur on this route.
pub fn compile_project_entry_with_packages(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<Vec<u8>, ProjectError> {
    let catalog = encode_project_seed_module_catalog(
        project_root,
        document,
        entry_path,
        package_roots,
        allowed_packages,
    )?;
    debug_assert!(
        !crate::product_path_requires_bootstrap_dual_compare(),
        "ADR-045: product multi-module path must not gate on dual-compare"
    );
    debug_assert!(
        !crate::product_multi_module_invokes_bootstrap(),
        "ADR-047: product multi-module path must not invoke bootstrap"
    );
    crate::compile_product_seed_modules(&catalog).map_err(|error| {
        module_error(
            "AE-PROJECT-004",
            format!("seed module project compile failed: {error}"),
        )
    })
}

/// M17b: compile and pure-run every `role: test` unit; require exit code 0.
pub fn run_project_tests(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<ProjectTestReport, ProjectError> {
    run_project_tests_with_grants(project_root, document, HostGrantConfig::default())
}

/// M17b/M17c: compile and run every `role: test` unit with optional grants.
pub fn run_project_tests_with_grants(
    project_root: &Path,
    document: &ProjectDocument,
    grants: HostGrantConfig,
) -> Result<ProjectTestReport, ProjectError> {
    let tests: Vec<_> = document
        .units
        .iter()
        .filter(|unit| unit.role == ProjectUnitRole::Test)
        .collect();
    if tests.is_empty() {
        return Err(module_error(
            "AE-PROJECT-001",
            "project test requires at least one unit with role test",
        ));
    }
    let pure = grants.read_roots.is_empty()
        && grants.write_roots.is_empty()
        && grants.env_names.is_empty();
    let mut results = Vec::new();
    for unit in tests {
        let bytecode = compile_project_entry(project_root, document, &unit.path)?;
        verify_bytecode(&bytecode).map_err(|error| {
            module_error(
                "AE-PROJECT-004",
                format!("test unit {} produced invalid artifact: {error}", unit.path),
            )
        })?;
        let run_result = if pure {
            run_bytecode(&bytecode)
        } else {
            run_bytecode_with_grants(&bytecode, grants.clone())
        };
        match run_result {
            Ok(output) if output.exit_code == 0 => results.push(ProjectTestResult {
                path: unit.path.clone(),
                ok: true,
                detail: "exit 0".to_owned(),
            }),
            Ok(output) => results.push(ProjectTestResult {
                path: unit.path.clone(),
                ok: false,
                detail: format!("exit {}", output.exit_code),
            }),
            Err(error) => results.push(ProjectTestResult {
                path: unit.path.clone(),
                ok: false,
                detail: format!("run failed: {error}"),
            }),
        }
    }
    Ok(ProjectTestReport { results })
}

/// One project test unit outcome (M17b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTestResult {
    pub path: String,
    pub ok: bool,
    pub detail: String,
}

/// Aggregate project test report (M17b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTestReport {
    pub results: Vec<ProjectTestResult>,
}

impl ProjectTestReport {
    #[must_use]
    pub fn all_passed(&self) -> bool {
        !self.results.is_empty() && self.results.iter().all(|result| result.ok)
    }

    #[must_use]
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|result| result.ok).count()
    }

    #[must_use]
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|result| !result.ok).count()
    }
}

/// Human-readable note for CLI.
#[must_use]
pub fn multi_module_authority_note() -> String {
    format!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} multi-module project build frames a bounded source catalog; the verified seed resolves and elaborates the import graph, then emits AETH (GSM-001; no bootstrap on product path; dual-compare is test/oracle only)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{parse_project_document, sha256_hex, ProjectUnitRole};
    use crate::run_bytecode;
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> std::path::PathBuf {
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("aether-modules-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("temp");
        path
    }

    #[test]
    fn project_test_entry_imports_lib_and_requires_exit_zero() {
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).unwrap();
        let math = "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n";
        let main = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n";
        // ADR-070 pattern: truth-choose revises then yields at weave root (product multi-module).
        let test = "world lib_test\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  bind r <- call math.double 21\n  bind mutable code <- 1\n  choose same r 42:\n    revise code <- 0\n  yield code\n";
        fs::write(root.join("lib/math.ae"), math).unwrap();
        fs::write(root.join("main.ae"), main).unwrap();
        fs::write(root.join("lib_test.ae"), test).unwrap();
        let project = r#"{
  "schema": "aether.project/v1",
  "name": "t",
  "version": "1",
  "units": [
    { "path": "main.ae", "role": "main" },
    { "path": "lib/math.ae", "role": "lib" },
    { "path": "lib_test.ae", "role": "test" }
  ]
}"#;
        let document = parse_project_document(project).unwrap();
        let report = run_project_tests(&root, &document).expect("project tests");
        assert!(report.all_passed(), "{:?}", report.results);

        let empty = r#"{
  "schema": "aether.project/v1",
  "name": "t",
  "version": "1",
  "units": [
    { "path": "main.ae", "role": "main" },
    { "path": "lib/math.ae", "role": "lib" }
  ]
}"#;
        let document = parse_project_document(empty).unwrap();
        let err = run_project_tests(&root, &document).expect_err("zero tests fail closed");
        assert!(err.message.contains("role test"), "{err}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn elaborates_import_export_and_runs_exit_42() {
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        let math = "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n\nweave secret [n: Whole] -> Whole:\n  yield sum n 1\n";
        // ADR-070: multi-module product path supports truth-choose + revise + root yield.
        let main = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  bind r <- call math.double 21\n  bind mutable code <- 1\n  choose same r 42:\n    revise code <- 0\n  yield code\n";
        fs::write(root.join("lib/math.ae"), math).unwrap();
        fs::write(root.join("src/main.ae"), main).unwrap();
        let main_hash = sha256_hex(main.as_bytes());
        let math_hash = sha256_hex(math.as_bytes());
        let json = format!(
            r#"{{
  "schema": "aether.project/v1",
  "name": "modules_demo",
  "version": "0.1.0",
  "units": [
    {{ "path": "src/main.ae", "role": "main" }},
    {{ "path": "lib/math.ae", "role": "lib" }}
  ],
  "lock": {{
    "units": [
      {{ "path": "src/main.ae", "sha256": "{main_hash}" }},
      {{ "path": "lib/math.ae", "sha256": "{math_hash}" }}
    ]
  }}
}}"#
        );
        let document = parse_project_document(&json).unwrap();
        let elaborated = elaborate_project_modules(&root, &document).unwrap();
        assert!(elaborated.contains("weave m_lib_math_double"));
        assert!(!elaborated.contains("import unit"));
        let product = compile_project_modules(&root, &document).unwrap();
        let run = run_bytecode(&product).unwrap();
        assert_eq!(run.exit_code, 0);
        // Oracle: product seed bytes dual-compare to bootstrap (tests only).
        let bootstrap = crate::compile_to_bytecode(&elaborated).unwrap();
        assert_eq!(
            product, bootstrap.bytecode,
            "product build must return seed bytes identical to bootstrap"
        );

        let bad = main.replace("math.double", "math.secret");
        fs::write(root.join("src/main.ae"), &bad).unwrap();
        let err = elaborate_project_modules(&root, &document).unwrap_err();
        assert_eq!(err.code, "AE-MOD-003");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn shipped_project_modules_example_seed_matches_bootstrap() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir.join("../../examples/project-modules");
        let project = root.join("aether.project.json");
        if !project.is_file() {
            return;
        }
        let json = fs::read_to_string(&project).unwrap();
        let document = parse_project_document(&json).unwrap();
        let elaborated = elaborate_project_modules(&root, &document).unwrap();
        let bootstrap = crate::compile_to_bytecode(&elaborated).unwrap();
        let seed = crate::compile_with_seed(&elaborated).unwrap();
        assert_eq!(
            bootstrap.bytecode, seed.bytecode,
            "examples/project-modules elaboration must dual-compare"
        );
        let product = compile_project_modules(&root, &document).unwrap();
        assert_eq!(product, seed.bytecode);
        let run = run_bytecode(&product).unwrap();
        assert_eq!(run.exit_code, 42);
    }

    #[test]
    fn project_catalog_framing_reads_manifest_units_without_parsing_source() {
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).expect("library directory");
        fs::create_dir_all(root.join("src")).expect("source directory");
        fs::write(root.join("lib/raw.ae"), "not Aether source: 🙂\n").expect("library source");
        fs::write(root.join("src/main.ae"), "also not Aether source: é\n").expect("entry source");
        let document = parse_project_document(
            r#"{
  "schema": "aether.project/v1",
  "name": "opaque_catalog",
  "version": "0.1.0",
  "units": [
    { "path": "src/main.ae", "role": "main" },
    { "path": "lib/raw.ae", "role": "lib" }
  ]
}"#,
        )
        .expect("project manifest");

        let catalog = encode_project_seed_module_catalog(
            &root,
            &document,
            "src/main.ae",
            &BTreeMap::new(),
            &BTreeSet::new(),
        )
        .expect("manifest/path/UTF-8 framing must remain source-opaque");
        let decoded = decode_seed_module_catalog(&catalog).expect("catalog round trip");
        assert_eq!(decoded.entry_key, "src/main.ae");
        assert!(decoded.allowed_packages.is_empty());
        assert_eq!(
            decoded
                .units
                .iter()
                .map(|unit| unit.key.as_str())
                .collect::<Vec<_>>(),
            ["lib/raw.ae", "src/main.ae"]
        );
        assert_eq!(decoded.units[0].source, "not Aether source: 🙂\n");
        assert_eq!(decoded.units[1].source, "also not Aether source: é\n");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn seed_catalog_elaborates_an_irregular_transitive_graph_byte_identically() {
        let base = "world base\n\nexport weave increment [n: Whole] -> Whole:\n  yield sum n 1\n";
        let scale =
            "world scale\n\nexport weave triple [n: Whole] -> Whole:\n  yield product n 3\n";
        let combine = "world combine\n\nimport unit \"lib/base.ae\" as base\nimport unit \"lib/scale.ae\" as scale\n\nexport weave compose [n: Whole] -> Whole:\n  bind raised <- call base.increment n\n  yield call scale.triple raised\n";
        let relay = "world relay\n\nimport unit \"lib/combine.ae\" as combine\n\nexport weave result [n: Whole] -> Whole:\n  yield call combine.compose n\n";
        let entry = "world app\n\nimport unit \"lib/relay.ae\" as relay\n\nweave main [] -> Whole:\n  yield call relay.result 13\n";
        let unused = "world unused\n\nexport weave ignored [n: Whole] -> Whole:\n  yield n\n";
        let units = vec![
            SeedModuleUnit {
                key: "src/main.ae".to_owned(),
                role: ProjectUnitRole::Main,
                source: entry.to_owned(),
            },
            SeedModuleUnit {
                key: "lib/unused.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: unused.to_owned(),
            },
            SeedModuleUnit {
                key: "lib/relay.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: relay.to_owned(),
            },
            SeedModuleUnit {
                key: "lib/scale.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: scale.to_owned(),
            },
            SeedModuleUnit {
                key: "lib/combine.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: combine.to_owned(),
            },
            SeedModuleUnit {
                key: "lib/base.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: base.to_owned(),
            },
        ];
        let catalog = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect("frame non-profile graph");
        let product = crate::compile_product_seed_modules(&catalog)
            .expect("checked-in seed elaborates general catalog");
        assert_eq!(run_bytecode(&product).expect("product runs").exit_code, 42);

        let reference_units: Vec<(String, String, ProjectUnitRole)> = units
            .iter()
            .map(|unit| (unit.key.clone(), unit.source.clone(), unit.role))
            .collect();
        let elaborated = elaborate_in_memory_units(&reference_units, "src/main.ae")
            .expect("reference elaborates irregular graph");
        let bootstrap = crate::compile_to_bytecode(&elaborated)
            .expect("reference bootstrap compiles")
            .bytecode;
        assert_eq!(
            product, bootstrap,
            "general seed graph must match bootstrap for non-fixed transitive topology"
        );
    }

    #[test]
    fn project_seed_catalog_elaborates_direct_m22_dependency_byte_identically() {
        let root = temp_dir();
        let consumer_root = root.join("consumer");
        let math_root = root.join("math");
        fs::create_dir_all(consumer_root.join("src")).expect("consumer directory");
        fs::create_dir_all(math_root.join("lib")).expect("math library directory");
        fs::create_dir_all(math_root.join("src")).expect("math source directory");

        let math = "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n";
        let math_main = "world math_package\n\nweave main [] -> Whole:\n  yield 0\n";
        let consumer = "world consumer\n\nimport unit \"lib/math.ae\" from package math as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n";
        fs::write(math_root.join("lib/math.ae"), math).expect("math library source");
        fs::write(math_root.join("src/main.ae"), math_main).expect("math main source");
        fs::write(consumer_root.join("src/main.ae"), consumer).expect("consumer source");

        let foreign_manifest = r#"{
  "schema": "aether.project/v1",
  "name": "math_project",
  "version": "0.1.0",
  "units": [
    { "path": "src/main.ae", "role": "main" },
    { "path": "lib/math.ae", "role": "lib" }
  ]
}"#;
        fs::write(math_root.join("aether.project.json"), foreign_manifest).expect("math manifest");
        let consumer_document = parse_project_document(
            r#"{
  "schema": "aether.project/v1",
  "name": "consumer_project",
  "version": "0.1.0",
  "units": [{ "path": "src/main.ae", "role": "main" }]
}"#,
        )
        .expect("consumer manifest");

        let mut roots = BTreeMap::new();
        roots.insert("math".to_owned(), math_root.clone());
        let allowed = BTreeSet::from(["math".to_owned()]);
        let catalog = encode_project_seed_module_catalog(
            &consumer_root,
            &consumer_document,
            "src/main.ae",
            &roots,
            &allowed,
        )
        .expect("direct dependency catalog");
        let decoded = decode_seed_module_catalog(&catalog).expect("catalog decodes");
        assert_eq!(decoded.allowed_packages, ["math"]);
        assert!(decoded
            .units
            .iter()
            .any(|unit| unit.key == "math::lib/math.ae"));
        assert!(!decoded
            .units
            .iter()
            .any(|unit| unit.key == "math::src/main.ae"));

        let product = compile_project_entry_with_packages(
            &consumer_root,
            &consumer_document,
            "src/main.ae",
            &roots,
            &allowed,
        )
        .expect("M22 product build");
        assert_eq!(run_bytecode(&product).expect("product runs").exit_code, 42);
        let reference = elaborate_project_entry_with_packages(
            &consumer_root,
            &consumer_document,
            "src/main.ae",
            &roots,
            &allowed,
        )
        .expect("M22 reference elaboration");
        let bootstrap = crate::compile_to_bytecode(&reference)
            .expect("M22 reference bootstrap")
            .bytecode;
        assert_eq!(product, bootstrap, "M22 seed catalog must match bootstrap");

        let unapproved = compile_project_entry_with_packages(
            &consumer_root,
            &consumer_document,
            "src/main.ae",
            &BTreeMap::new(),
            &BTreeSet::new(),
        )
        .expect_err("seed must reject a package import lacking direct authority");
        assert_eq!(unapproved.code, "AE-PROJECT-004");
        assert!(unapproved.message.contains("AE-SEED-017"), "{unapproved}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_import_cycle_and_lib_main() {
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).unwrap();
        fs::write(
            root.join("lib/a.ae"),
            "world a\n\nimport unit \"lib/b.ae\" as b\n\nexport weave id [n: Whole] -> Whole:\n  yield n\n",
        )
        .unwrap();
        fs::write(
            root.join("lib/b.ae"),
            "world b\n\nimport unit \"lib/a.ae\" as a\n\nexport weave id [n: Whole] -> Whole:\n  yield n\n",
        )
        .unwrap();
        fs::write(
            root.join("main.ae"),
            "world app\n\nimport unit \"lib/a.ae\" as a\n\nweave main [] -> Whole:\n  yield call a.id 1\n",
        )
        .unwrap();
        let json = r#"{
  "schema": "aether.project/v1",
  "name": "cycle",
  "version": "0.1.0",
  "units": [
    { "path": "main.ae", "role": "main" },
    { "path": "lib/a.ae", "role": "lib" },
    { "path": "lib/b.ae", "role": "lib" }
  ]
}"#;
        let document = parse_project_document(json).unwrap();
        let err = elaborate_project_modules(&root, &document).unwrap_err();
        assert_eq!(err.code, "AE-MOD-004");
        let err = compile_project_modules(&root, &document)
            .expect_err("seed catalog product route must reject an import cycle");
        assert_eq!(err.code, "AE-PROJECT-004");
        assert!(err.message.contains("AE-SEED-017"), "{err}");

        let err = parse_module_source(
            "lib/bad.ae",
            "world bad\n\nweave main [] -> Whole:\n  yield 0\n",
            ProjectUnitRole::Lib,
        )
        .unwrap_err();
        assert_eq!(err.code, "AE-MOD-006");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn seed_bundle_framing_is_scalar_exact_bounded_and_source_opaque() {
        let units = [
            ("lib/math.ae".to_owned(), "not Aether: 🙂\n".to_owned()),
            ("src/main.ae".to_owned(), "also opaque: é".to_owned()),
        ];
        let bundle = encode_seed_bundle("src/main.ae", &units)
            .expect("framing must not parse or reject opaque source text");
        assert!(bundle.contains("unit lib/math.ae 14\n"));
        assert!(bundle.contains("unit src/main.ae 14\n"));
        let decoded = decode_seed_bundle(&bundle).expect("Unicode scalar framing must round-trip");
        assert_eq!(decoded.entry_path, "src/main.ae");
        assert_eq!(decoded.units, units);

        let wrong_scalar_count =
            bundle.replacen("unit lib/math.ae 14\n", "unit lib/math.ae 15\n", 1);
        let error = decode_seed_bundle(&wrong_scalar_count)
            .expect_err("a scalar count that crosses the LF separator must fail closed");
        assert_eq!(error.code, "AE-MOD-001");

        let unsafe_path = encode_seed_bundle(
            "src/main.ae",
            &[
                ("../math.ae".to_owned(), "x".to_owned()),
                ("src/main.ae".to_owned(), "y".to_owned()),
            ],
        )
        .expect_err("unsafe seed bundle paths must fail before framing");
        assert_eq!(unsafe_path.code, "AE-MOD-002");

        let wrong_order = encode_seed_bundle(
            "src/main.ae",
            &[
                ("src/main.ae".to_owned(), "x".to_owned()),
                ("lib/math.ae".to_owned(), "y".to_owned()),
            ],
        )
        .expect_err("entry path must identify the final bundle unit");
        assert_eq!(wrong_order.code, "AE-MOD-006");

        let oversize_path = format!("{}.ae", "a".repeat(SEED_BUNDLE_MAX_WIRE_SCALARS));
        let oversize = encode_seed_bundle(
            "src/main.ae",
            &[
                (oversize_path, "x".to_owned()),
                ("src/main.ae".to_owned(), "y".to_owned()),
            ],
        )
        .expect_err("wire framing must cap path-bearing untrusted bundle text");
        assert_eq!(oversize.code, "AE-MOD-001");
        assert!(oversize.message.contains("wire limit"));
    }

    #[test]
    fn seed_bundle_chain_framing_is_scalar_exact_bounded_and_source_opaque() {
        let units = [
            ("lib/base.ae".to_owned(), "🙂".to_owned()),
            ("lib/math.ae".to_owned(), "é".to_owned()),
            ("src/main.ae".to_owned(), "z".to_owned()),
        ];
        let bundle = encode_seed_bundle_chain("src/main.ae", &units)
            .expect("chain framing must not parse or reject opaque source text");
        assert!(bundle.contains("unit lib/base.ae 1\n"));
        assert!(bundle.contains("unit lib/math.ae 1\n"));
        assert!(bundle.contains("unit src/main.ae 1\n"));
        let decoded = decode_seed_bundle_chain(&bundle)
            .expect("Unicode scalar chain framing must round-trip");
        assert_eq!(decoded.entry_path, "src/main.ae");
        assert_eq!(decoded.units, units);

        let wrong_scalar_count = bundle.replacen("unit lib/base.ae 1\n", "unit lib/base.ae 2\n", 1);
        let error = decode_seed_bundle_chain(&wrong_scalar_count)
            .expect_err("a scalar count that crosses the LF separator must fail closed");
        assert_eq!(error.code, "AE-MOD-001");

        let wrong_count = encode_seed_bundle_chain(
            "src/main.ae",
            &[
                ("lib/base.ae".to_owned(), "x".to_owned()),
                ("src/main.ae".to_owned(), "y".to_owned()),
            ],
        )
        .expect_err("chain framing must require its fixed three-unit shape");
        assert_eq!(wrong_count.code, "AE-MOD-006");

        let duplicate = encode_seed_bundle_chain(
            "src/main.ae",
            &[
                ("lib/base.ae".to_owned(), "x".to_owned()),
                ("lib/base.ae".to_owned(), "y".to_owned()),
                ("src/main.ae".to_owned(), "z".to_owned()),
            ],
        )
        .expect_err("chain framing must reject duplicate unit identities");
        assert_eq!(duplicate.code, "AE-MOD-005");

        let oversize_path = format!("{}.ae", "a".repeat(SEED_BUNDLE_CHAIN_MAX_WIRE_SCALARS));
        let oversize = encode_seed_bundle_chain(
            "src/main.ae",
            &[
                (oversize_path, "x".to_owned()),
                ("lib/math.ae".to_owned(), "y".to_owned()),
                ("src/main.ae".to_owned(), "z".to_owned()),
            ],
        )
        .expect_err("chain framing must cap path-bearing untrusted bundle text");
        assert_eq!(oversize.code, "AE-MOD-001");
        assert!(oversize.message.contains("wire limit"));
    }

    #[test]
    fn seed_bundle_fanin_framing_is_scalar_exact_bounded_and_source_opaque() {
        let units = [
            ("lib/base.ae".to_owned(), "🙂".to_owned()),
            ("lib/scale.ae".to_owned(), "é".to_owned()),
            ("lib/combine.ae".to_owned(), "ö".to_owned()),
            ("src/main.ae".to_owned(), "z".to_owned()),
        ];
        let bundle = encode_seed_bundle_fanin("src/main.ae", &units)
            .expect("fan-in framing must not parse or reject opaque source text");
        assert!(bundle.contains("unit lib/base.ae 1\n"));
        assert!(bundle.contains("unit lib/scale.ae 1\n"));
        assert!(bundle.contains("unit lib/combine.ae 1\n"));
        assert!(bundle.contains("unit src/main.ae 1\n"));
        let decoded = decode_seed_bundle_fanin(&bundle)
            .expect("Unicode scalar fan-in framing must round-trip");
        assert_eq!(decoded.entry_path, "src/main.ae");
        assert_eq!(decoded.units, units);

        let wrong_scalar_count = bundle.replacen("unit lib/base.ae 1\n", "unit lib/base.ae 2\n", 1);
        let error = decode_seed_bundle_fanin(&wrong_scalar_count)
            .expect_err("a scalar count that crosses the LF separator must fail closed");
        assert_eq!(error.code, "AE-MOD-001");

        let wrong_count = encode_seed_bundle_fanin(
            "src/main.ae",
            &[
                ("lib/base.ae".to_owned(), "x".to_owned()),
                ("lib/scale.ae".to_owned(), "y".to_owned()),
                ("src/main.ae".to_owned(), "z".to_owned()),
            ],
        )
        .expect_err("fan-in framing must require its fixed four-unit shape");
        assert_eq!(wrong_count.code, "AE-MOD-006");

        let duplicate = encode_seed_bundle_fanin(
            "src/main.ae",
            &[
                ("lib/base.ae".to_owned(), "x".to_owned()),
                ("lib/base.ae".to_owned(), "y".to_owned()),
                ("lib/combine.ae".to_owned(), "z".to_owned()),
                ("src/main.ae".to_owned(), "q".to_owned()),
            ],
        )
        .expect_err("fan-in framing must reject duplicate unit identities");
        assert_eq!(duplicate.code, "AE-MOD-005");

        let oversize_path = format!("{}.ae", "a".repeat(SEED_BUNDLE_FANIN_MAX_WIRE_SCALARS));
        let oversize = encode_seed_bundle_fanin(
            "src/main.ae",
            &[
                (oversize_path, "x".to_owned()),
                ("lib/scale.ae".to_owned(), "y".to_owned()),
                ("lib/combine.ae".to_owned(), "z".to_owned()),
                ("src/main.ae".to_owned(), "q".to_owned()),
            ],
        )
        .expect_err("fan-in framing must cap path-bearing untrusted bundle text");
        assert_eq!(oversize.code, "AE-MOD-001");
        assert!(oversize.message.contains("wire limit"));
    }

    #[test]
    fn seed_module_catalog_framing_is_scalar_exact_closed_and_source_opaque() {
        let units = vec![
            SeedModuleUnit {
                key: "lib/math.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: "not Aether: 🙂\n".to_owned(),
            },
            SeedModuleUnit {
                key: "util::whole.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: "also opaque: é".to_owned(),
            },
            SeedModuleUnit {
                key: "src/main.ae".to_owned(),
                role: ProjectUnitRole::Main,
                source: "entry opaque: ö".to_owned(),
            },
        ];
        let bundle = encode_seed_module_catalog("src/main.ae", &["util".to_owned()], &units)
            .expect("catalog framing must not parse opaque Aether source");
        assert!(bundle.starts_with(
            "aether.seed-modules/v1\nentry src/main.ae\npackages 1\npackage util\nunits 3\n"
        ));
        assert!(bundle.contains("unit lib/math.ae lib 14\n"));
        assert!(bundle.contains("unit util::whole.ae lib 14\n"));
        assert!(bundle.contains("unit src/main.ae main 15\nsource\n"));
        let decoded =
            decode_seed_module_catalog(&bundle).expect("Unicode scalar catalog must round-trip");
        assert_eq!(decoded.entry_key, "src/main.ae");
        assert_eq!(decoded.allowed_packages, ["util"]);
        assert_eq!(decoded.units, units);

        let wrong_scalar_count =
            bundle.replacen("unit lib/math.ae lib 14\n", "unit lib/math.ae lib 15\n", 1);
        let error = decode_seed_module_catalog(&wrong_scalar_count)
            .expect_err("a source scalar count cannot consume the next catalog payload");
        assert_eq!(error.code, "AE-MOD-001");

        let foreign_entry =
            encode_seed_module_catalog("util::whole.ae", &["util".to_owned()], &units)
                .expect_err("catalog entry must remain local to the selected project");
        assert_eq!(foreign_entry.code, "AE-MOD-006");

        let unallowed_foreign = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect_err("foreign source identity requires an explicit allowed package");
        assert_eq!(unallowed_foreign.code, "AE-MOD-002");

        let duplicate = encode_seed_module_catalog(
            "src/main.ae",
            &["util".to_owned()],
            &[
                units[0].clone(),
                units[0].clone(),
                SeedModuleUnit {
                    key: "src/main.ae".to_owned(),
                    role: ProjectUnitRole::Main,
                    source: "entry".to_owned(),
                },
            ],
        )
        .expect_err("catalog must reject duplicate source identities");
        assert_eq!(duplicate.code, "AE-MOD-005");
    }

    #[test]
    fn seed_catalog_validator_accepts_inclusive_unit_and_aggregate_scalar_limits() {
        let seed_source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seed/aether_seed.ae"),
        )
        .expect("read seed source");
        let compiler = crate::compile_to_bytecode(&seed_source)
            .expect("bootstrap-compiles seed catalog scalar-limit validator")
            .bytecode;

        let maximum_source = "🙂".repeat(SEED_MODULES_MAX_UNIT_SCALARS);
        let mut units = Vec::new();
        for index in 0..(SEED_MODULES_MAX_TOTAL_SCALARS / SEED_MODULES_MAX_UNIT_SCALARS) {
            let (key, role) = if index == 0 {
                ("src/main.ae".to_owned(), ProjectUnitRole::Main)
            } else {
                (format!("lib/unit-{index}.ae"), ProjectUnitRole::Lib)
            };
            units.push(SeedModuleUnit {
                key,
                role,
                source: maximum_source.clone(),
            });
        }
        assert_eq!(
            units.len() * SEED_MODULES_MAX_UNIT_SCALARS,
            SEED_MODULES_MAX_TOTAL_SCALARS,
            "test fixture must occupy the documented aggregate boundary exactly"
        );
        let catalog = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect("host framing accepts documented inclusive scalar limits");
        assert!(
            catalog.len() <= crate::MAX_TEXT_BYTES,
            "the worst-case UTF-8 catalog must fit the Aether Text invocation limit"
        );
        let validation = crate::invoke_bytecode(
            &compiler,
            "modules_catalog_valid",
            &[crate::InvocationValue::Text(catalog)],
        )
        .expect("seed catalog boundary validator invokes");
        assert_eq!(
            validation.value,
            crate::InvocationValue::Truth(true),
            "seed must accept exactly the public per-unit and aggregate scalar limits; stdout: {}",
            validation.stdout
        );

        let oversized_source = "🙂".repeat(SEED_MODULES_MAX_UNIT_SCALARS + 1);
        let oversized_catalog = format!(
            "{SEED_MODULES_SCHEMA}\nentry src/main.ae\npackages 0\nunits 1\nunit src/main.ae main {}\nsource\n{oversized_source}",
            SEED_MODULES_MAX_UNIT_SCALARS + 1
        );
        let oversized_validation = crate::invoke_bytecode(
            &compiler,
            "modules_catalog_valid",
            &[crate::InvocationValue::Text(oversized_catalog)],
        )
        .expect("seed oversized catalog validator invokes");
        assert_eq!(
            oversized_validation.value,
            crate::InvocationValue::Truth(false),
            "seed must reject one scalar past the public per-unit limit; stdout: {}",
            oversized_validation.stdout
        );
    }

    #[test]
    fn bootstrap_seed_module_catalog_validator_accepts_closed_local_catalog() {
        let seed_source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seed/aether_seed.ae"),
        )
        .expect("read seed source");
        let compiler = crate::compile_to_bytecode(&seed_source)
            .expect("bootstrap-compiles seed catalog validator")
            .bytecode;
        let units = vec![
            SeedModuleUnit {
                key: "lib/math.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n"
                    .to_owned(),
            },
            SeedModuleUnit {
                key: "src/main.ae".to_owned(),
                role: ProjectUnitRole::Main,
                source: "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n"
                    .to_owned(),
            },
        ];
        let catalog = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect("frame closed local catalog");
        let validation = crate::invoke_bytecode(
            &compiler,
            "modules_catalog_valid",
            &[crate::InvocationValue::Text(catalog)],
        )
        .expect("seed catalog validator invokes");
        assert_eq!(
            validation.value,
            crate::InvocationValue::Truth(true),
            "seed catalog validator stdout: {}",
            validation.stdout
        );
    }

    #[test]
    fn bootstrap_seed_module_graph_orders_closed_local_catalog() {
        let seed_source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seed/aether_seed.ae"),
        )
        .expect("read seed source");
        let compiler = crate::compile_to_bytecode(&seed_source)
            .expect("bootstrap-compiles seed graph elaborator")
            .bytecode;
        let units = vec![
            SeedModuleUnit {
                key: "lib/math.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n"
                    .to_owned(),
            },
            SeedModuleUnit {
                key: "src/main.ae".to_owned(),
                role: ProjectUnitRole::Main,
                source: "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n"
                    .to_owned(),
            },
        ];
        let catalog = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect("frame closed local catalog");
        let graph = crate::invoke_bytecode(
            &compiler,
            "modules_graph_order",
            &[crate::InvocationValue::Text(catalog)],
        )
        .expect("seed graph elaborator invokes");
        assert_eq!(
            graph.value,
            crate::InvocationValue::Text("|lib/math.ae|src/main.ae|".to_owned()),
            "seed graph elaborator stdout: {}",
            graph.stdout
        );
    }

    #[test]
    fn bootstrap_seed_module_import_counter_is_bounded_for_local_import() {
        let seed_source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seed/aether_seed.ae"),
        )
        .expect("read seed source");
        let compiler = crate::compile_to_bytecode(&seed_source)
            .expect("bootstrap-compiles seed import counter")
            .bytecode;
        let entry = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n";
        let imports = crate::invoke_bytecode(
            &compiler,
            "modules_import_count",
            &[crate::InvocationValue::Text(entry.to_owned())],
        )
        .expect("seed import counter invokes");
        assert_eq!(imports.value, crate::InvocationValue::Whole(1));
    }

    #[test]
    fn bootstrap_seed_compile_modules_elaborates_general_local_catalog() {
        let seed_source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seed/aether_seed.ae"),
        )
        .expect("read seed source");
        let compiler = crate::compile_to_bytecode(&seed_source)
            .expect("bootstrap-compiles seed with general module ABI")
            .bytecode;
        let library =
            "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n";
        let entry = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n";
        let units = vec![
            SeedModuleUnit {
                key: "lib/math.ae".to_owned(),
                role: ProjectUnitRole::Lib,
                source: library.to_owned(),
            },
            SeedModuleUnit {
                key: "src/main.ae".to_owned(),
                role: ProjectUnitRole::Main,
                source: entry.to_owned(),
            },
        ];
        let catalog = encode_seed_module_catalog("src/main.ae", &[], &units)
            .expect("frame closed local catalog");
        let catalog_valid = crate::invoke_bytecode(
            &compiler,
            "modules_catalog_valid",
            &[crate::InvocationValue::Text(catalog.clone())],
        )
        .expect("seed catalog framing validator invokes");
        assert_eq!(
            catalog_valid.value,
            crate::InvocationValue::Truth(true),
            "catalog preflight stdout: {}",
            catalog_valid.stdout
        );
        let forged = crate::forge_modules_bytecode(&compiler, &catalog)
            .expect("seed compile_modules accepts general local graph");
        let seed_stdout = forged.stdout.clone();
        let crate::InvocationValue::Bytes(bytecode) = forged.value else {
            panic!("compile_modules must yield Bytes; seed stdout: {seed_stdout}");
        };
        crate::verify_bytecode(&bytecode).unwrap_or_else(|error| {
            panic!("seed module artifact verifies: {error}; seed stdout: {seed_stdout}")
        });
        assert_eq!(
            crate::run_bytecode(&bytecode)
                .expect("seed module artifact runs")
                .exit_code,
            42
        );

        let expected = elaborate_in_memory_units(
            &[
                (
                    "lib/math.ae".to_owned(),
                    library.to_owned(),
                    ProjectUnitRole::Lib,
                ),
                (
                    "src/main.ae".to_owned(),
                    entry.to_owned(),
                    ProjectUnitRole::Main,
                ),
            ],
            "src/main.ae",
        )
        .expect("reference elaboration");
        let bootstrap = crate::compile_to_bytecode(&expected)
            .expect("reference bootstrap")
            .bytecode;
        assert_eq!(
            bytecode, bootstrap,
            "general seed module elaboration must reproduce the established M11 artifact"
        );
    }
}
