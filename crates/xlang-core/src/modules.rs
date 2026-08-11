//! M11a language modules: import unit / export weave / project build.
//!
//! Multi-module programs are **host-elaborated** into one single-world Aether
//! program (ADR-015 / M11a), then **seed-emitted** on the product path (M11b /
//! ADR-056). Seed does **not** natively elaborate multi-file graphs (no multi-file
//! forge ABI). Bootstrap dual-compare is test/oracle only (ADR-045).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::project::{
    resolve_unit_path, validate_unit_path, ProjectDocument, ProjectError, ProjectUnitRole,
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
    use crate::project::{parse_project_document, PROJECT_SCHEMA_VERSION};
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
/// Host authority (ADR-056). Product emission of the result uses seed only
/// ([`crate::compile_product_bytecode`]); dual-compare remains test/oracle.
pub fn elaborate_project_modules(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<String, ProjectError> {
    debug_assert!(
        crate::host_elaborates_modules_seed_emits(),
        "ADR-056: host elaborates; seed emits"
    );
    debug_assert!(
        !crate::seed_native_multi_module_elaboration(),
        "ADR-056 honesty: seed does not elaborate multi-module natively"
    );
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

/// ADR-075 host multi-file product forge: in-memory units (no project file), host
/// elaborate + seed emit. Cross-package imports are not supported on this path.
pub fn compile_product_multi_unit(
    units: &[(String, String, ProjectUnitRole)],
    entry_path: &str,
) -> Result<Vec<u8>, ProjectError> {
    debug_assert!(
        crate::product_multi_source_forge_envelope(),
        "ADR-075: multi-unit product forge"
    );
    let elaborated = elaborate_in_memory_units(units, entry_path)?;
    crate::compile_product_bytecode(&elaborated).map_err(|error| {
        module_error(
            "AE-SEED-001",
            format!("product multi-unit seed emit failed: {error}"),
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

/// Multi-module project build → verified AETH bytes (M11b).
///
/// Host elaborates the import DAG, then **seed product emit only** (ADR-047:
/// no bootstrap parse/emit). Dual-compare is test/oracle (ADR-045).
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
/// Product path: host elaboration + [`compile_product_bytecode`] only (ADR-047).
/// No bootstrap dual-compare gate (ADR-045); no bootstrap AST parse.
pub fn compile_project_entry_with_packages(
    project_root: &Path,
    document: &ProjectDocument,
    entry_path: &str,
    package_roots: &BTreeMap<String, PathBuf>,
    allowed_packages: &BTreeSet<String>,
) -> Result<Vec<u8>, ProjectError> {
    let source = elaborate_project_entry_with_packages(
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
    compile_product_bytecode(&source).map_err(|error| {
        module_error(
            "AE-PROJECT-004",
            format!("module project seed compile failed: {error}"),
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
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} multi-module project build elaborates the import graph then seed-emits (M11b; no bootstrap on product path, ADR-047; dual-compare is test/oracle only)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{parse_project_document, sha256_hex, ProjectUnitRole};
    use crate::run_bytecode;
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

        let err = parse_module_source(
            "lib/bad.ae",
            "world bad\n\nweave main [] -> Whole:\n  yield 0\n",
            ProjectUnitRole::Lib,
        )
        .unwrap_err();
        assert_eq!(err.code, "AE-MOD-006");
        let _ = fs::remove_dir_all(&root);
    }
}
