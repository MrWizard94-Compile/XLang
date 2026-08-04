//! M11a language modules: import unit / export weave / project build.
//!
//! Multi-module programs are **bootstrap-compiled** via deterministic elaboration
//! into one single-world Aether program (ADR-015 phase M11a). Seed multi-module
//! authority is deferred to M11b.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::project::{
    resolve_unit_path, validate_unit_path, ProjectDocument, ProjectError, ProjectUnitRole,
};
use crate::{compile_to_bytecode, CompileOutput, LANGUAGE_NAME, LANGUAGE_VERSION};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleImport {
    path: String,
    alias: String,
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
#[must_use]
pub fn mangle_weave(unit_path: &str, weave: &str) -> String {
    let stem = unit_path
        .strip_suffix(".ae")
        .unwrap_or(unit_path)
        .replace('/', "_");
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
        ProjectUnitRole::Main => {
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
    let tail = after[end + 1..].trim_start();
    let Some(alias_part) = tail.strip_prefix("as ") else {
        return Err(module_error(
            "AE-MOD-001",
            format!("import unit in {module_path} requires `as <alias>`"),
        ));
    };
    let alias = alias_part.trim();
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
    // Bootstrap-compile with a synthetic total main to catch invalid weave bodies.
    let body = strip_export_keyword(&parsed.body_source);
    let probe = format!(
        "world {}\n\n{}\nweave main [] -> Whole:\n  yield 0\n",
        parsed.world,
        body.trim_end()
    );
    compile_to_bytecode(&probe).map_err(|error| {
        module_error(
            "AE-PROJECT-004",
            format!("lib unit {path} failed validation compile: {error}"),
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
) -> Result<BTreeMap<String, ParsedModule>, ProjectError> {
    let mut by_path = BTreeMap::new();
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
        by_path.insert(unit.path.clone(), parsed);
    }
    Ok(by_path)
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
            if !modules.contains_key(&import.path) {
                return Err(module_error(
                    "AE-MOD-002",
                    format!(
                        "module {} imports {}, which is not listed in the project",
                        path, import.path
                    ),
                ));
            }
            let target = &modules[&import.path];
            if target.role == ProjectUnitRole::Main {
                return Err(module_error(
                    "AE-MOD-002",
                    format!("module {path} cannot import the main unit {}", import.path),
                ));
            }
            dfs(&import.path, modules, visiting, visited, order)?;
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

/// Elaborate the main unit's import cone into one bootstrap-compilable source.
pub fn elaborate_project_modules(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<String, ProjectError> {
    let modules = load_graph(project_root, document)?;
    let entry = document
        .units
        .iter()
        .find(|unit| unit.role == ProjectUnitRole::Main)
        .ok_or_else(|| module_error("AE-MOD-006", "project has no main unit"))?;
    let order = closed_cone(&entry.path, &modules)?;

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
            let exports = export_tables.get(&import.path).ok_or_else(|| {
                module_error(
                    "AE-MOD-002",
                    format!("import {} missing export table", import.path),
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
            let exports = &export_tables[&import.path];
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

    let entry_world = modules[&entry.path].world.clone();
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

/// Bootstrap multi-source project build → one verified AETH (M11a).
pub fn compile_project_modules(
    project_root: &Path,
    document: &ProjectDocument,
) -> Result<CompileOutput, ProjectError> {
    let source = elaborate_project_modules(project_root, document)?;
    compile_to_bytecode(&source).map_err(|error| {
        module_error(
            "AE-PROJECT-004",
            format!("module project bootstrap compile failed: {error}"),
        )
    })
}

/// Human-readable note for CLI.
#[must_use]
pub fn multi_module_authority_note() -> String {
    format!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} multi-module project build uses bootstrap compilation (M11a); seed dual-compare is M11b"
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
    fn elaborates_import_export_and_runs_exit_42() {
        let root = temp_dir();
        fs::create_dir_all(root.join("lib")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        let math = "world math\n\nexport weave double [n: Whole] -> Whole:\n  yield product n 2\n\nweave secret [n: Whole] -> Whole:\n  yield sum n 1\n";
        let main = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield call math.double 21\n";
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
        let compiled = compile_project_modules(&root, &document).unwrap();
        let run = run_bytecode(&compiled.bytecode).unwrap();
        assert_eq!(run.exit_code, 42);

        let bad = main.replace("math.double", "math.secret");
        fs::write(root.join("src/main.ae"), &bad).unwrap();
        let err = elaborate_project_modules(&root, &document).unwrap_err();
        assert_eq!(err.code, "AE-MOD-003");
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
