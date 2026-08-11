//! Versioned structural-authoring contracts for local Aether tooling.
//!
//! This module deliberately sits above parsing and below no host capability.
//! It serializes Aether source from an edited in-memory program, accepts a small
//! allow-listed edit protocol, renders canonical source, and **accepts** that
//! source via the product seed path (ADR-048).
//!
//! **ADR-065:** top-level **weave replace** ops on product-accepted base source
//! take a product text-splice path (no bootstrap `Program` base parse). Other
//! ops (insert/delete/statement edits, record targets) still use bootstrap AST.
//! Callers that persist an edit should still product-seed-compile before writing
//! (CLI trusts core product accept per ADR-053).

use std::collections::BTreeSet;
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{json, Map, Number, Value};

use crate::{
    compile_product_bytecode, compile_source, format_program, Atom, AtomKind, BinaryOperation,
    BufferElement, CompilerError, Diagnostic, Effect, Expression, ExpressionKind, HostWeave,
    Parameter, ParameterMode, Program, RecordDeclaration, RecordField, ResourceOperation,
    ShapeDeclaration, Span, Statement, TableLayout, TernaryOperation, UnaryOperation, ValueType,
    Weave, LANGUAGE_NAME, LANGUAGE_VERSION,
};

/// The JSON schema identifier emitted for a validated semantic document.
pub const STRUCTURAL_AST_SCHEMA_VERSION: &str = "aether.ast/v8";
/// The JSON protocol identifier accepted for a structural edit request.
pub const STRUCTURAL_EDIT_PROTOCOL_VERSION: &str = "aether.edit/v8";
/// The JSON schema identifier used for machine-readable diagnostic envelopes.
pub const DIAGNOSTIC_SCHEMA_VERSION: &str = "aether.diagnostic/v8";
/// Product-only structure envelope (ADR-054) — not full AST.
pub const PRODUCT_STRUCTURE_SCHEMA_VERSION: &str = "aether.product-structure/v1";
/// Product weave-replace edit result (ADR-065) — not full `aether.ast/v8`.
pub const PRODUCT_EDIT_SCHEMA_VERSION: &str = "aether.product-edit/v1";

const MAX_STRUCTURAL_EDIT_BYTES: usize = 4_000_000;
const MAX_STRUCTURAL_EDIT_OPERATIONS: usize = 32;
const MAX_STRUCTURAL_EDIT_NODES: usize = 100_000;
const MAX_STRUCTURAL_EDIT_DEPTH: usize = 256;
const MAX_SOURCE_BYTES: usize = 1_000_000;

/// A successful pure structural edit. The returned source is product-accepted
/// (ADR-048). Weave-only replace may omit bootstrap AST (ADR-065); other ops
/// still base-parse via bootstrap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralEditResult {
    pub source: String,
    pub document_json: String,
    pub operation_count: usize,
}

/// A bounded protocol failure or a diagnostic raised while validating an edited
/// Aether program. It carries the same stable envelope used by the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralEditError {
    diagnostic: Diagnostic,
}

impl StructuralEditError {
    #[must_use]
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for StructuralEditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} error [{}] at {}: {}",
            LANGUAGE_NAME, self.diagnostic.code, self.diagnostic.span, self.diagnostic.message
        )
    }
}

impl std::error::Error for StructuralEditError {}

impl From<CompilerError> for StructuralEditError {
    fn from(error: CompilerError) -> Self {
        Self {
            diagnostic: error.diagnostic(),
        }
    }
}

/// Return deterministic, pretty-printed `aether.ast/v8` JSON for valid source.
/// Source spans always refer to the returned document's canonical LF source.
/// Uses bootstrap AST (authoring authority).
pub fn structural_document_json(source: &str) -> Result<String, CompilerError> {
    let (program, canonical_source) = canonicalize_source(source)?;
    serialize_document(&canonical_source, &program).map_err(serialization_error)
}

/// Product-path structure envelope (ADR-054): seed accept + LF source.
///
/// Schema [`PRODUCT_STRUCTURE_SCHEMA_VERSION`]. Does **not** emit `aether.ast/v8`
/// and does **not** invoke the bootstrap compiler.
pub fn product_structure_json(source: &str) -> Result<String, CompilerError> {
    debug_assert!(
        crate::product_structure_without_bootstrap(),
        "ADR-054: product structure must not require bootstrap"
    );
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let bytecode = compile_product_bytecode(&normalized)?;
    serde_json::to_string_pretty(&json!({
        "schema": PRODUCT_STRUCTURE_SCHEMA_VERSION,
        "language": {
            "name": LANGUAGE_NAME,
            "version": LANGUAGE_VERSION,
        },
        "productAccepted": true,
        "artifactBytes": bytecode.len(),
        "source": normalized,
    }))
    .map_err(serialization_error)
}

/// Serialize a stable `aether.diagnostic/v8` envelope for any compiler or
/// structural-authoring diagnostic.
#[must_use]
pub fn diagnostic_json(diagnostic: &Diagnostic) -> String {
    json!({
        "schema": DIAGNOSTIC_SCHEMA_VERSION,
        "code": diagnostic.code,
        "span": span_value(diagnostic.span),
        "message": &diagnostic.message,
    })
    .to_string()
}

/// Apply one bounded `aether.edit/v8` document to matching source.
///
/// The function does not write files, invoke a model, execute code, or compile
/// an artifact. It is intentionally pure apart from memory allocation. The CLI
/// trusts product accept on returned source (ADR-053).
///
/// **ADR-065:** when the base product-accepts and every operation is a top-level
/// weave `replace`, the edit runs without bootstrap `Program` base parse.
pub fn apply_structural_edit(
    source: &str,
    edit_json: &str,
) -> Result<StructuralEditResult, StructuralEditError> {
    if edit_json.len() > MAX_STRUCTURAL_EDIT_BYTES {
        return Err(protocol_error(
            "AE-EDIT-006",
            format!("structural edit exceeds the {MAX_STRUCTURAL_EDIT_BYTES}-byte safety limit"),
        ));
    }

    // ADR-065: product weave-replace path (no bootstrap base AST) when eligible.
    if crate::structural_edit_product_weave_replace() {
        if let Some(result) = try_product_weave_replace(source, edit_json)? {
            return Ok(result);
        }
    }

    let (mut program, canonical_source) =
        canonicalize_source(source).map_err(StructuralEditError::from)?;
    let request = parse_request(edit_json)?;
    if request.base_source != canonical_source {
        return Err(protocol_error(
            "AE-EDIT-003",
            "structural edit was created for a different canonical source revision",
        ));
    }

    let mut node_budget = NodeBudget::default();
    for operation in &request.operations {
        apply_operation(&mut program, operation, &mut node_budget)?;
    }

    let candidate_source = format_program(&program);
    // ADR-048: product seed is the accept gate (not bootstrap re-validate).
    debug_assert!(
        crate::structural_edit_accepts_via_product_seed(),
        "ADR-048: structural edit accept must use product seed"
    );
    compile_product_bytecode(&candidate_source).map_err(StructuralEditError::from)?;
    let document_json = serialize_document(&candidate_source, &program)
        .map_err(|error| protocol_error("AE-EDIT-001", error.to_string()))?;

    Ok(StructuralEditResult {
        source: candidate_source,
        document_json,
        operation_count: request.operations.len(),
    })
}

/// ADR-065: product-accepted weave-only replace without bootstrap base parse.
///
/// Returns `Ok(None)` when the edit is outside the product subset (caller falls
/// back to bootstrap AST). Returns `Err` for protocol/product failures that
/// should not fall back (e.g. unknown weave target after product accepted).
fn try_product_weave_replace(
    source: &str,
    edit_json: &str,
) -> Result<Option<StructuralEditResult>, StructuralEditError> {
    debug_assert!(
        crate::structural_edit_product_weave_replace(),
        "ADR-065: product weave replace without bootstrap base AST"
    );
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    // Only take the product path when product accepts the base unit.
    if compile_product_bytecode(&normalized).is_err() {
        return Ok(None);
    }
    let request = parse_request(edit_json)?;
    if request.base_source != normalized {
        // Bootstrap path uses format_program canonical; product base is LF source.
        return Ok(None);
    }
    if request.operations.is_empty() {
        return Ok(None);
    }
    for operation in &request.operations {
        match (&operation.kind, &operation.target) {
            (EditOperationKind::Replace, Some(EditTarget::Weave(_))) => {}
            _ => return Ok(None),
        }
    }

    let mut candidate = normalized;
    let mut node_budget = NodeBudget::default();
    for operation in &request.operations {
        let Some(EditTarget::Weave(target_name)) = operation.target.as_ref() else {
            return Ok(None);
        };
        // Empty records: product weave-replace subset is primitive-typed weaves.
        // Record/Buffer/shape-typed payloads fall back to bootstrap AST.
        let declaration = match parse_operation_declaration(
            &Program {
                world: "product".to_owned(),
                records: Vec::new(),
                shapes: Vec::new(),
                host_weaves: Vec::new(),
                weaves: Vec::new(),
            },
            operation,
            &mut node_budget,
        ) {
            Ok(declaration) => declaration,
            Err(_) => return Ok(None),
        };
        let EditableDeclaration::Weave(weave) = declaration else {
            return Ok(None);
        };
        if weave.name != *target_name {
            return Err(protocol_error(
                "AE-EDIT-005",
                "replacement Weave name must match its weave target",
            ));
        }
        let weave_source = format_weave_text(&weave);
        candidate = replace_top_level_weave_text(&candidate, target_name, &weave_source)?;
    }

    compile_product_bytecode(&candidate).map_err(StructuralEditError::from)?;
    let document_json = serialize_product_edit_document(&candidate, request.operations.len())
        .map_err(|error| protocol_error("AE-EDIT-001", error.to_string()))?;
    Ok(Some(StructuralEditResult {
        source: candidate,
        document_json,
        operation_count: request.operations.len(),
    }))
}

fn format_weave_text(weave: &Weave) -> String {
    let program = Program {
        world: "product".to_owned(),
        records: Vec::new(),
        shapes: Vec::new(),
        host_weaves: Vec::new(),
        weaves: vec![weave.clone()],
    };
    let formatted = format_program(&program);
    let mut lines = formatted.lines().peekable();
    // Drop world line and leading blanks; keep weave declaration + body.
    while let Some(line) = lines.peek() {
        if line.starts_with("world ") || line.is_empty() {
            lines.next();
            continue;
        }
        break;
    }
    let body: Vec<&str> = lines.collect();
    if body.is_empty() {
        return String::new();
    }
    let mut out = body.join("\n");
    out.push('\n');
    out
}

/// Byte-range splice: replace the top-level weave named `name` with `new_weave`.
fn replace_top_level_weave_text(
    source: &str,
    name: &str,
    new_weave: &str,
) -> Result<String, StructuralEditError> {
    let (start, end) = find_top_level_weave_range(source, name)?;
    let mut out = String::with_capacity(source.len() + new_weave.len());
    out.push_str(&source[..start]);
    out.push_str(new_weave.trim_end_matches('\n'));
    // Preserve a single trailing newline after the weave; inter-weave blanks live in source[end..].
    if end < source.len() || source.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&source[end..]);
    Ok(out)
}

/// Inclusive start / exclusive end byte range of a top-level weave declaration body.
/// Trailing blank lines between weaves are **not** part of the range (identity-preserving).
fn find_top_level_weave_range(
    source: &str,
    name: &str,
) -> Result<(usize, usize), StructuralEditError> {
    let mut offset = 0usize;
    let mut start: Option<usize> = None;
    let lines: Vec<&str> = source.split_inclusive('\n').collect();
    for (index, line_with_nl) in lines.iter().enumerate() {
        let line = line_with_nl.trim_end_matches(['\n', '\r']);
        let is_top = !line.is_empty()
            && !line.starts_with(' ')
            && !line.starts_with('\t')
            && !line.trim_start().starts_with('#');
        if let Some(range_start) = start {
            if is_top {
                // Next top-level declaration ends the previous weave; drop inter-weave blanks.
                let end = trim_trailing_blank_lines(source, range_start, offset);
                return Ok((range_start, end));
            }
            if index + 1 == lines.len() {
                let end = trim_trailing_blank_lines(source, range_start, source.len());
                return Ok((range_start, end));
            }
            offset += line_with_nl.len();
            continue;
        }
        if is_top && line_declares_weave(line, name) {
            start = Some(offset);
        }
        offset += line_with_nl.len();
    }
    if let Some(range_start) = start {
        let end = trim_trailing_blank_lines(source, range_start, source.len());
        return Ok((range_start, end));
    }
    Err(protocol_error(
        "AE-EDIT-004",
        format!("weave target {name:?} does not exist"),
    ))
}

/// Drop trailing blank lines from `[start, end)` so inter-weave spacing stays in the suffix.
fn trim_trailing_blank_lines(source: &str, start: usize, mut end: usize) -> usize {
    while end > start {
        let region = &source[start..end];
        if region.ends_with("\n\n") {
            end -= 1;
            continue;
        }
        break;
    }
    end
}

fn line_declares_weave(line: &str, name: &str) -> bool {
    let trimmed = line.trim_start();
    let after_export = trimmed.strip_prefix("export ").unwrap_or(trimmed);
    let after_task = after_export.strip_prefix("task ").unwrap_or(after_export);
    let Some(rest) = after_task.strip_prefix("weave ") else {
        return false;
    };
    let decl_name = rest.split([' ', '[']).next().unwrap_or("");
    decl_name == name
}

fn serialize_product_edit_document(
    source: &str,
    operation_count: usize,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&json!({
        "schema": PRODUCT_EDIT_SCHEMA_VERSION,
        "language": {
            "name": LANGUAGE_NAME,
            "version": LANGUAGE_VERSION,
        },
        "canonicalSource": source,
        "productAccepted": true,
        "path": "product-weave-replace",
        "operationCount": operation_count,
    }))
}

fn canonicalize_source(source: &str) -> Result<(Program, String), CompilerError> {
    // ADR-057: when both product and bootstrap reject, prefer product AE-SEED codes.
    // When product rejects but bootstrap accepts (e.g. export-only lib units), keep
    // bootstrap base parse for authoring AST. When product accepts, still need
    // bootstrap for Program AST.
    debug_assert!(
        crate::structural_edit_product_base_gate(),
        "ADR-057: structural edit product base gate"
    );
    let product_result = compile_product_bytecode(source);
    let parsed = match compile_source(source) {
        Ok(program) => program,
        Err(bootstrap_error) => {
            // Prefer product AE-SEED when product also rejects; else bootstrap error.
            product_result?;
            return Err(bootstrap_error);
        }
    };
    let canonical_source = format_program(&parsed);
    // ADR-050: skip a second bootstrap compile when already canonical (LF form).
    let normalized_input = source.replace("\r\n", "\n").replace('\r', "\n");
    if normalized_input == canonical_source {
        return Ok((parsed, canonical_source));
    }
    let canonical_program = compile_source(&canonical_source)?;
    Ok((canonical_program, canonical_source))
}

fn serialization_error(error: serde_json::Error) -> CompilerError {
    CompilerError {
        span: Span { line: 1, column: 1 },
        message: format!("could not serialize Aether structural document: {error}"),
    }
}

fn serialize_document(source: &str, program: &Program) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&json!({
        "schema": STRUCTURAL_AST_SCHEMA_VERSION,
        "language": {
            "name": LANGUAGE_NAME,
            "version": LANGUAGE_VERSION,
        },
        "canonicalSource": source,
        "program": program_value(program),
    }))
}

fn program_value(program: &Program) -> Value {
    let records = program
        .records
        .iter()
        .map(|record| record_value(record, &program.records))
        .collect::<Vec<_>>();
    let shapes = program.shapes.iter().map(shape_value).collect::<Vec<_>>();
    let host_weaves = program
        .host_weaves
        .iter()
        .map(|host| host_weave_value(host, &program.records))
        .collect::<Vec<_>>();
    let weaves = program
        .weaves
        .iter()
        .map(|weave| weave_value(weave, &program.records, &program.shapes))
        .collect::<Vec<_>>();
    json!({
        "id": "program",
        "kind": "Program",
        "world": {
            "id": "world",
            "kind": "World",
            "span": span_value(Span { line: 1, column: 1 }),
            "name": &program.world,
        },
        "records": records,
        "shapes": shapes,
        "hostWeaves": host_weaves,
        "weaves": weaves,
    })
}

fn host_weave_value(host: &HostWeave, records: &[RecordDeclaration]) -> Value {
    let host_id = format!("hostWeave:{}", host.name);
    let parameters = host
        .parameters
        .iter()
        .map(|parameter| {
            json!({
                "id": format!("{host_id}/parameter:{}", parameter.name),
                "kind": "Parameter",
                "span": span_value(parameter.span),
                "name": &parameter.name,
                "mode": parameter_mode_name(parameter.mode),
                "type": value_type_name(parameter.value_type, records, &[]),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "id": host_id,
        "kind": "HostWeave",
        "span": span_value(host.span),
        "name": &host.name,
        "parameters": parameters,
        "result": value_type_name(host.result, records, &[]),
    })
}

fn shape_value(shape: &ShapeDeclaration) -> Value {
    let shape_id = format!("shape:{}", shape.name);
    let fields = shape
        .fields
        .iter()
        .map(|field| {
            json!({
                "id": format!("{shape_id}/field:{}", field.name),
                "kind": "ShapeField",
                "span": span_value(field.span),
                "name": &field.name,
                "type": "Whole",
            })
        })
        .collect::<Vec<_>>();
    json!({
        "id": shape_id,
        "kind": "Shape",
        "span": span_value(shape.span),
        "name": &shape.name,
        "fields": fields,
    })
}

fn record_value(record: &RecordDeclaration, records: &[RecordDeclaration]) -> Value {
    let record_id = format!("record:{}", record.name);
    let fields = record
        .fields
        .iter()
        .map(|field| {
            json!({
                "id": format!("{record_id}/field:{}", field.name),
                "kind": "RecordField",
                "span": span_value(field.span),
                "name": &field.name,
                "type": value_type_name(field.value_type, records, &[]),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "id": record_id,
        "kind": "Record",
        "span": span_value(record.span),
        "name": &record.name,
        "fields": fields,
    })
}

fn weave_value(weave: &Weave, records: &[RecordDeclaration], shapes: &[ShapeDeclaration]) -> Value {
    let weave_id = format!("weave:{}", weave.name);
    let parameters = weave
        .parameters
        .iter()
        .map(|parameter| {
            json!({
                "id": format!("{weave_id}/parameter:{}", parameter.name),
                "kind": "Parameter",
                "span": span_value(parameter.span),
                "name": &parameter.name,
                "mode": parameter_mode_name(parameter.mode),
                "type": value_type_name(parameter.value_type, records, shapes),
            })
        })
        .collect::<Vec<_>>();
    let body = weave
        .body
        .iter()
        .enumerate()
        .map(|(index, statement)| statement_value(statement, &format!("{weave_id}/body/{index}")))
        .collect::<Vec<_>>();
    json!({
        "id": weave_id,
        "kind": "Weave",
        "span": span_value(weave.span),
        "name": &weave.name,
        "parameters": parameters,
        "result": value_type_name(weave.result, records, shapes),
        "effect": effect_name(weave.effect),
        "task": weave.task,
        "body": body,
    })
}

fn statement_value(statement: &Statement, id: &str) -> Value {
    match statement {
        Statement::Bind {
            name,
            mutable,
            comptime,
            value,
            span,
        } => json!({
            "id": id,
            "kind": "Bind",
            "span": span_value(*span),
            "name": name,
            "mutable": mutable,
            "stage": if *comptime { "comptime" } else { "runtime" },
            "value": expression_value(value, &format!("{id}/value")),
        }),
        Statement::Revise { name, value, span } => json!({
            "id": id,
            "kind": "Revise",
            "span": span_value(*span),
            "name": name,
            "value": expression_value(value, &format!("{id}/value")),
        }),
        Statement::Release { name, span } => json!({
            "id": id,
            "kind": "Release",
            "span": span_value(*span),
            "name": name,
        }),
        Statement::Checkpoint { span } => json!({
            "id": id,
            "kind": "Checkpoint",
            "span": span_value(*span),
        }),
        Statement::Speak { value, span } => json!({
            "id": id,
            "kind": "Speak",
            "span": span_value(*span),
            "value": expression_value(value, &format!("{id}/value")),
        }),
        Statement::Yield { value, span } => json!({
            "id": id,
            "kind": "Yield",
            "span": span_value(*span),
            "value": expression_value(value, &format!("{id}/value")),
        }),
        Statement::Raise { code, span } => json!({
            "id": id,
            "kind": "Raise",
            "span": span_value(*span),
            "code": atom_value(code, &format!("{id}/code")),
        }),
        Statement::Forward {
            weave,
            arguments,
            span,
        } => json!({
            "id": id,
            "kind": "Forward",
            "span": span_value(*span),
            "weave": weave,
            "arguments": arguments.iter().enumerate().map(|(index, argument)| {
                atom_value(argument, &format!("{id}/argument/{index}"))
            }).collect::<Vec<_>>(),
        }),
        Statement::Handle {
            weave,
            arguments,
            success_destination,
            error_destination,
            span,
        } => json!({
            "id": id,
            "kind": "Handle",
            "span": span_value(*span),
            "weave": weave,
            "arguments": arguments.iter().enumerate().map(|(index, argument)| {
                atom_value(argument, &format!("{id}/argument/{index}"))
            }).collect::<Vec<_>>(),
            "successDestination": success_destination,
            "errorDestination": error_destination,
        }),
        Statement::Choose {
            condition,
            when_bright,
            when_dim,
            span,
        } => json!({
            "id": id,
            "kind": "Choose",
            "span": span_value(*span),
            "condition": expression_value(condition, &format!("{id}/condition")),
            "whenBright": when_bright.iter().enumerate().map(|(index, nested)| {
                statement_value(nested, &format!("{id}/bright/{index}"))
            }).collect::<Vec<_>>(),
            "whenDim": when_dim.iter().enumerate().map(|(index, nested)| {
                statement_value(nested, &format!("{id}/dim/{index}"))
            }).collect::<Vec<_>>(),
        }),
        Statement::While {
            condition,
            body,
            span,
        } => json!({
            "id": id,
            "kind": "While",
            "span": span_value(*span),
            "condition": expression_value(condition, &format!("{id}/condition")),
            "body": body.iter().enumerate().map(|(index, nested)| {
                statement_value(nested, &format!("{id}/body/{index}"))
            }).collect::<Vec<_>>(),
        }),
        Statement::Together { spawns, span } => json!({
            "id": id,
            "kind": "Together",
            "span": span_value(*span),
            "spawns": spawns.iter().enumerate().map(|(index, spawn)| {
                json!({
                    "id": format!("{id}/spawn/{index}"),
                    "kind": "Spawn",
                    "span": span_value(spawn.span),
                    "weave": &spawn.weave,
                    "arguments": spawn.arguments.iter().enumerate().map(|(argument_index, argument)| {
                        atom_value(argument, &format!("{id}/spawn/{index}/argument/{argument_index}"))
                    }).collect::<Vec<_>>(),
                    "destination": &spawn.destination,
                })
            }).collect::<Vec<_>>(),
        }),
    }
}

fn expression_value(expression: &Expression, id: &str) -> Value {
    let span = span_value(expression.span);
    match &expression.kind {
        ExpressionKind::Atom(atom) => json!({
            "id": id,
            "kind": "Atom",
            "span": span,
            "atom": atom_value(atom, &format!("{id}/atom")),
        }),
        ExpressionKind::Arena { capacity } => json!({
            "id": id,
            "kind": "Arena",
            "span": span,
            "capacity": capacity,
        }),
        ExpressionKind::Buffer { element } => json!({
            "id": id,
            "kind": "Buffer",
            "span": span,
            "element": buffer_element_name(*element),
        }),
        ExpressionKind::Table { shape, layout } => json!({
            "id": id,
            "kind": "Table",
            "span": span,
            "shape": shape,
            "layout": table_layout_name(*layout),
        }),
        ExpressionKind::Unary {
            operation,
            argument,
        } => json!({
            "id": id,
            "kind": "Unary",
            "span": span,
            "operation": unary_operation_name(*operation),
            "argument": atom_value(argument, &format!("{id}/argument")),
        }),
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => json!({
            "id": id,
            "kind": "Binary",
            "span": span,
            "operation": binary_operation_name(*operation),
            "left": atom_value(left, &format!("{id}/left")),
            "right": atom_value(right, &format!("{id}/right")),
        }),
        ExpressionKind::Cut { text, start, end } => json!({
            "id": id,
            "kind": "Cut",
            "span": span,
            "text": atom_value(text, &format!("{id}/text")),
            "start": atom_value(start, &format!("{id}/start")),
            "end": atom_value(end, &format!("{id}/end")),
        }),
        ExpressionKind::Slice { bytes, start, end } => json!({
            "id": id,
            "kind": "Slice",
            "span": span,
            "bytes": atom_value(bytes, &format!("{id}/bytes")),
            "start": atom_value(start, &format!("{id}/start")),
            "end": atom_value(end, &format!("{id}/end")),
        }),
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => json!({
            "id": id,
            "kind": "Ternary",
            "span": span,
            "operation": ternary_operation_name(*operation),
            "first": atom_value(first, &format!("{id}/first")),
            "second": atom_value(second, &format!("{id}/second")),
            "third": atom_value(third, &format!("{id}/third")),
        }),
        ExpressionKind::Call { weave, arguments } => json!({
            "id": id,
            "kind": "Call",
            "span": span,
            "weave": weave,
            "arguments": arguments.iter().enumerate().map(|(index, argument)| {
                atom_value(argument, &format!("{id}/argument/{index}"))
            }).collect::<Vec<_>>(),
        }),
        ExpressionKind::MakeRecord { record, fields } => json!({
            "id": id,
            "kind": "MakeRecord",
            "span": span,
            "record": record,
            "fields": fields.iter().enumerate().map(|(index, field)| {
                atom_value(field, &format!("{id}/field/{index}"))
            }).collect::<Vec<_>>(),
        }),
        ExpressionKind::Field { record, field } => json!({
            "id": id,
            "kind": "Field",
            "span": span,
            "record": atom_value(record, &format!("{id}/record")),
            "field": field,
        }),
        ExpressionKind::Resource(operation) => resource_value(operation, id, span),
    }
}

fn resource_value(operation: &ResourceOperation, id: &str, span: Value) -> Value {
    match operation {
        ResourceOperation::Allocate {
            arena,
            buffer,
            capacity,
            destination,
        } => json!({
            "id": id,
            "kind": "Resource",
            "span": span,
            "operation": "allocate",
            "arena": atom_value(arena, &format!("{id}/arena")),
            "buffer": atom_value(buffer, &format!("{id}/buffer")),
            "capacity": atom_value(capacity, &format!("{id}/capacity")),
            "destination": destination,
        }),
        ResourceOperation::Append {
            buffer,
            value,
            destination,
        } => json!({
            "id": id,
            "kind": "Resource",
            "span": span,
            "operation": "append",
            "buffer": atom_value(buffer, &format!("{id}/buffer")),
            "value": atom_value(value, &format!("{id}/value")),
            "destination": destination,
        }),
        ResourceOperation::At {
            buffer,
            index,
            destination,
        } => json!({
            "id": id,
            "kind": "Resource",
            "span": span,
            "operation": "at",
            "buffer": atom_value(buffer, &format!("{id}/buffer")),
            "index": atom_value(index, &format!("{id}/index")),
            "destination": destination,
        }),
        ResourceOperation::Store {
            table,
            index,
            field,
            value,
            destination,
        } => json!({
            "id": id,
            "kind": "Resource",
            "span": span,
            "operation": "store",
            "table": atom_value(table, &format!("{id}/table")),
            "index": atom_value(index, &format!("{id}/index")),
            "field": field,
            "value": atom_value(value, &format!("{id}/value")),
            "destination": destination,
        }),
        ResourceOperation::Load {
            table,
            index,
            field,
            destination,
        } => json!({
            "id": id,
            "kind": "Resource",
            "span": span,
            "operation": "load",
            "table": atom_value(table, &format!("{id}/table")),
            "index": atom_value(index, &format!("{id}/index")),
            "field": field,
            "destination": destination,
        }),
    }
}

fn atom_value(atom: &Atom, id: &str) -> Value {
    let span = span_value(atom.span);
    match &atom.kind {
        AtomKind::Text(value) => json!({ "id": id, "kind": "Text", "span": span, "value": value }),
        AtomKind::Bytes(value) => json!({
            "id": id,
            "kind": "Bytes",
            "span": span,
            "value": hex_encode(value),
        }),
        AtomKind::Whole(value) => {
            json!({ "id": id, "kind": "Whole", "span": span, "value": value })
        }
        AtomKind::Truth(value) => {
            json!({ "id": id, "kind": "Truth", "span": span, "value": value })
        }
        AtomKind::Name(name) => json!({ "id": id, "kind": "Name", "span": span, "name": name }),
        AtomKind::Borrow(name) => json!({ "id": id, "kind": "Borrow", "span": span, "name": name }),
        AtomKind::Move(name) => json!({ "id": id, "kind": "Move", "span": span, "name": name }),
        AtomKind::Access(name) => json!({ "id": id, "kind": "Access", "span": span, "name": name }),
    }
}

fn span_value(span: Span) -> Value {
    json!({ "line": span.line, "column": span.column })
}

fn value_type_name(
    value_type: ValueType,
    records: &[RecordDeclaration],
    shapes: &[ShapeDeclaration],
) -> String {
    match value_type {
        ValueType::Text => "Text".to_owned(),
        ValueType::Whole => "Whole".to_owned(),
        ValueType::Truth => "Truth".to_owned(),
        ValueType::Bytes => "Bytes".to_owned(),
        ValueType::Record(index) => records
            .get(usize::from(index))
            .map(|record| record.name.clone())
            .unwrap_or_else(|| format!("Record#{index}")),
        ValueType::Arena => "Arena".to_owned(),
        ValueType::BufferWhole => "BufferWhole".to_owned(),
        ValueType::BufferTruth => "BufferTruth".to_owned(),
        ValueType::AccessArena => "AccessArena".to_owned(),
        ValueType::Table(index) => shapes
            .get(usize::from(index))
            .map(|shape| format!("Table[{}]", shape.name))
            .unwrap_or_else(|| format!("Table#{index}")),
    }
}

fn table_layout_name(layout: TableLayout) -> &'static str {
    match layout {
        TableLayout::Rows => "rows",
        TableLayout::Columns => "columns",
    }
}

fn parameter_mode_name(mode: ParameterMode) -> &'static str {
    match mode {
        ParameterMode::Own => "own",
        ParameterMode::Borrow => "borrow",
        ParameterMode::Access => "access",
    }
}

fn effect_name(effect: Effect) -> &'static str {
    match effect {
        Effect::Total => "Total",
        Effect::ErrorWhole => "ErrorWhole",
    }
}

fn buffer_element_name(element: BufferElement) -> &'static str {
    match element {
        BufferElement::Whole => "Whole",
        BufferElement::Truth => "Truth",
    }
}

fn unary_operation_name(operation: UnaryOperation) -> &'static str {
    match operation {
        UnaryOperation::Not => "not",
        UnaryOperation::Measure => "measure",
        UnaryOperation::Render => "render",
        UnaryOperation::Extent => "extent",
        UnaryOperation::Encode => "encode",
        UnaryOperation::Decode => "decode",
        UnaryOperation::Number => "number",
        UnaryOperation::Pack16 => "pack16",
        UnaryOperation::Pack32 => "pack32",
        UnaryOperation::Pack64 => "pack64",
        UnaryOperation::Count => "count",
    }
}

fn binary_operation_name(operation: BinaryOperation) -> &'static str {
    match operation {
        BinaryOperation::Sum => "sum",
        BinaryOperation::Difference => "difference",
        BinaryOperation::Product => "product",
        BinaryOperation::Less => "less",
        BinaryOperation::Same => "same",
        BinaryOperation::Join => "join",
        BinaryOperation::Glyph => "glyph",
        BinaryOperation::Quotient => "quotient",
        BinaryOperation::Remainder => "remainder",
        BinaryOperation::Fuse => "fuse",
        BinaryOperation::Append => "append",
        BinaryOperation::Octet => "octet",
        BinaryOperation::Unpack16 => "unpack16",
        BinaryOperation::Unpack32 => "unpack32",
    }
}

fn ternary_operation_name(operation: TernaryOperation) -> &'static str {
    match operation {
        TernaryOperation::Seek => "seek",
        TernaryOperation::Poke => "poke",
        TernaryOperation::Poke32 => "poke32",
    }
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

#[derive(Debug)]
struct EditRequest {
    base_source: String,
    operations: Vec<EditOperation>,
}

#[derive(Debug)]
struct EditOperation {
    kind: EditOperationKind,
    target: Option<EditTarget>,
    /// Statement path for fine-grained ops (`weave:main/body/0`, …).
    path: Option<StatementPath>,
    /// List path without trailing index (`weave:main/body`, nested lists).
    list: Option<BodyListRef>,
    index: Option<usize>,
    declaration: Option<Value>,
    statement: Option<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditOperationKind {
    Replace,
    InsertAfter,
    Delete,
    ReplaceStatement,
    InsertStatementAfter,
    InsertStatementAt,
    DeleteStatement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EditTarget {
    World,
    Record(String),
    Weave(String),
}

/// A weave body list that may receive statement edits.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BodyListRef {
    WeaveBody { weave: String },
    ChooseBright { weave: String, statement: usize },
    ChooseDim { weave: String, statement: usize },
    WhileBody { weave: String, statement: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StatementPath {
    list: BodyListRef,
    index: usize,
}

#[derive(Debug)]
enum EditableDeclaration {
    Record(RecordDeclaration),
    Weave(Weave),
}

#[derive(Default)]
struct NodeBudget {
    used: usize,
}

impl NodeBudget {
    fn consume(&mut self) -> Result<(), StructuralEditError> {
        self.used += 1;
        if self.used > MAX_STRUCTURAL_EDIT_NODES {
            return Err(protocol_error(
                "AE-EDIT-006",
                format!(
                    "structural edit exceeds the {MAX_STRUCTURAL_EDIT_NODES}-node safety limit"
                ),
            ));
        }
        Ok(())
    }
}

fn parse_request(input: &str) -> Result<EditRequest, StructuralEditError> {
    let StrictJsonValue(root) = serde_json::from_str(input).map_err(|error| {
        protocol_error(
            "AE-EDIT-001",
            format!("structural edit is not valid strict JSON: {error}"),
        )
    })?;
    let object = expect_object(&root, "structural edit")?;
    ensure_only_fields(
        object,
        &["protocol", "schema", "baseSource", "operations"],
        "structural edit",
    )?;

    let protocol = required_string(object, "protocol", "structural edit")?;
    if protocol != STRUCTURAL_EDIT_PROTOCOL_VERSION {
        return Err(protocol_error(
            "AE-EDIT-002",
            format!("unsupported structural edit protocol {protocol:?}"),
        ));
    }
    let schema = required_string(object, "schema", "structural edit")?;
    if schema != STRUCTURAL_AST_SCHEMA_VERSION {
        return Err(protocol_error(
            "AE-EDIT-002",
            format!("unsupported structural AST schema {schema:?}"),
        ));
    }
    let base_source = required_string(object, "baseSource", "structural edit")?;
    if base_source.len() > MAX_SOURCE_BYTES {
        return Err(protocol_error(
            "AE-EDIT-006",
            format!("baseSource exceeds the {MAX_SOURCE_BYTES}-byte source safety limit"),
        ));
    }
    let operation_values = required_array(object, "operations", "structural edit")?;
    if operation_values.is_empty() {
        return Err(protocol_error(
            "AE-EDIT-005",
            "structural edit requires at least one operation",
        ));
    }
    if operation_values.len() > MAX_STRUCTURAL_EDIT_OPERATIONS {
        return Err(protocol_error(
            "AE-EDIT-006",
            format!(
                "structural edit exceeds the {MAX_STRUCTURAL_EDIT_OPERATIONS}-operation safety limit"
            ),
        ));
    }
    let operations = operation_values
        .iter()
        .enumerate()
        .map(|(index, value)| parse_operation(value, index))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(EditRequest {
        base_source,
        operations,
    })
}

fn parse_operation(value: &Value, index: usize) -> Result<EditOperation, StructuralEditError> {
    let context = format!("operation {index}");
    let object = expect_object(value, &context)?;
    let operation = required_string(object, "op", &context)?;
    match operation.as_str() {
        "replace" => {
            ensure_only_fields(object, &["op", "target", "declaration"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::Replace,
                target: Some(parse_target(&required_string(object, "target", &context)?)?),
                path: None,
                list: None,
                index: None,
                declaration: Some(required_value(object, "declaration", &context)?.clone()),
                statement: None,
            })
        }
        "insertAfter" => {
            ensure_only_fields(object, &["op", "target", "declaration"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::InsertAfter,
                target: Some(parse_target(&required_string(object, "target", &context)?)?),
                path: None,
                list: None,
                index: None,
                declaration: Some(required_value(object, "declaration", &context)?.clone()),
                statement: None,
            })
        }
        "delete" => {
            ensure_only_fields(object, &["op", "target"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::Delete,
                target: Some(parse_target(&required_string(object, "target", &context)?)?),
                path: None,
                list: None,
                index: None,
                declaration: None,
                statement: None,
            })
        }
        "replaceStatement" => {
            ensure_only_fields(object, &["op", "path", "statement"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::ReplaceStatement,
                target: None,
                path: Some(parse_statement_path(&required_string(
                    object, "path", &context,
                )?)?),
                list: None,
                index: None,
                declaration: None,
                statement: Some(required_value(object, "statement", &context)?.clone()),
            })
        }
        "insertStatementAfter" => {
            ensure_only_fields(object, &["op", "path", "statement"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::InsertStatementAfter,
                target: None,
                path: Some(parse_statement_path(&required_string(
                    object, "path", &context,
                )?)?),
                list: None,
                index: None,
                declaration: None,
                statement: Some(required_value(object, "statement", &context)?.clone()),
            })
        }
        "insertStatementAt" => {
            ensure_only_fields(object, &["op", "list", "index", "statement"], &context)?;
            let index_value = required_value(object, "index", &context)?;
            let index = index_value.as_u64().ok_or_else(|| {
                protocol_error(
                    "AE-EDIT-011",
                    format!("{context} index must be a non-negative integer"),
                )
            })?;
            let index = usize::try_from(index).map_err(|_| {
                protocol_error("AE-EDIT-011", format!("{context} index is out of range"))
            })?;
            Ok(EditOperation {
                kind: EditOperationKind::InsertStatementAt,
                target: None,
                path: None,
                list: Some(parse_body_list(&required_string(
                    object, "list", &context,
                )?)?),
                index: Some(index),
                declaration: None,
                statement: Some(required_value(object, "statement", &context)?.clone()),
            })
        }
        "deleteStatement" => {
            ensure_only_fields(object, &["op", "path"], &context)?;
            Ok(EditOperation {
                kind: EditOperationKind::DeleteStatement,
                target: None,
                path: Some(parse_statement_path(&required_string(
                    object, "path", &context,
                )?)?),
                list: None,
                index: None,
                declaration: None,
                statement: None,
            })
        }
        _ => Err(protocol_error(
            "AE-EDIT-005",
            format!("{context} has unsupported operation {operation:?}"),
        )),
    }
}

fn parse_body_list(value: &str) -> Result<BodyListRef, StructuralEditError> {
    let parts: Vec<&str> = value.split('/').collect();
    // weave:name/body
    // weave:name/body/N/whenBright|whenDim|body
    if parts.len() == 2 {
        let weave = parse_weave_path_head(parts[0])?;
        if parts[1] != "body" {
            return Err(protocol_error(
                "AE-EDIT-010",
                format!("{value:?} is not a body list path"),
            ));
        }
        return Ok(BodyListRef::WeaveBody { weave });
    }
    if parts.len() == 4 {
        let weave = parse_weave_path_head(parts[0])?;
        if parts[1] != "body" {
            return Err(protocol_error(
                "AE-EDIT-010",
                format!("{value:?} is not a nested body list path"),
            ));
        }
        let statement = parse_path_index(parts[2])?;
        return match parts[3] {
            "whenBright" => Ok(BodyListRef::ChooseBright { weave, statement }),
            "whenDim" => Ok(BodyListRef::ChooseDim { weave, statement }),
            "body" => Ok(BodyListRef::WhileBody { weave, statement }),
            _ => Err(protocol_error(
                "AE-EDIT-010",
                format!("{value:?} has unsupported nested list segment"),
            )),
        };
    }
    Err(protocol_error(
        "AE-EDIT-010",
        format!("{value:?} is not an allowed statement list path"),
    ))
}

fn parse_statement_path(value: &str) -> Result<StatementPath, StructuralEditError> {
    let parts: Vec<&str> = value.split('/').collect();
    // weave:name/body/N
    // weave:name/body/N/whenBright/M
    if parts.len() == 3 {
        let weave = parse_weave_path_head(parts[0])?;
        if parts[1] != "body" {
            return Err(protocol_error(
                "AE-EDIT-010",
                format!("{value:?} is not a statement path"),
            ));
        }
        let index = parse_path_index(parts[2])?;
        return Ok(StatementPath {
            list: BodyListRef::WeaveBody { weave },
            index,
        });
    }
    if parts.len() == 5 {
        let list = parse_body_list(&parts[..4].join("/"))?;
        let index = parse_path_index(parts[4])?;
        return Ok(StatementPath { list, index });
    }
    Err(protocol_error(
        "AE-EDIT-010",
        format!("{value:?} is not an allowed statement path"),
    ))
}

fn parse_weave_path_head(head: &str) -> Result<String, StructuralEditError> {
    let Some(name) = head.strip_prefix("weave:") else {
        return Err(protocol_error(
            "AE-EDIT-010",
            format!("{head:?} must start with weave:"),
        ));
    };
    parse_target_name(name)
}

fn parse_path_index(text: &str) -> Result<usize, StructuralEditError> {
    if text.len() > 1 && text.starts_with('0') {
        return Err(protocol_error(
            "AE-EDIT-010",
            format!("statement index {text:?} must not have leading zeros"),
        ));
    }
    text.parse::<usize>().map_err(|_| {
        protocol_error(
            "AE-EDIT-011",
            format!("statement index {text:?} is not a valid non-negative integer"),
        )
    })
}

fn parse_target(value: &str) -> Result<EditTarget, StructuralEditError> {
    if value == "world" {
        return Ok(EditTarget::World);
    }
    if let Some(name) = value.strip_prefix("record:") {
        return parse_target_name(name).map(EditTarget::Record);
    }
    if let Some(name) = value.strip_prefix("weave:") {
        return parse_target_name(name).map(EditTarget::Weave);
    }
    Err(protocol_error(
        "AE-EDIT-004",
        format!("{value:?} is not an allowed structural target"),
    ))
}

fn parse_target_name(name: &str) -> Result<String, StructuralEditError> {
    if is_aether_name(name) {
        Ok(name.to_owned())
    } else {
        Err(protocol_error(
            "AE-EDIT-004",
            format!("{name:?} is not a canonical Aether declaration name"),
        ))
    }
}

fn is_aether_name(name: &str) -> bool {
    let mut characters = name.chars();
    matches!(characters.next(), Some(character) if character.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn apply_operation(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    match operation.kind {
        EditOperationKind::Replace => apply_replace(program, operation, node_budget),
        EditOperationKind::InsertAfter => apply_insert_after(program, operation, node_budget),
        EditOperationKind::Delete => {
            let target = operation.target.as_ref().ok_or_else(|| {
                protocol_error("AE-EDIT-001", "delete requires a top-level target")
            })?;
            apply_delete(program, target)
        }
        EditOperationKind::ReplaceStatement => {
            apply_replace_statement(program, operation, node_budget)
        }
        EditOperationKind::InsertStatementAfter => {
            apply_insert_statement_after(program, operation, node_budget)
        }
        EditOperationKind::InsertStatementAt => {
            apply_insert_statement_at(program, operation, node_budget)
        }
        EditOperationKind::DeleteStatement => apply_delete_statement(program, operation),
    }
}

fn apply_replace(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let target = operation
        .target
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-001", "replace requires a top-level target"))?;
    let declaration = parse_operation_declaration(program, operation, node_budget)?;
    match (target, declaration) {
        (EditTarget::Record(target_name), EditableDeclaration::Record(record)) => {
            if record.name != *target_name {
                return Err(protocol_error(
                    "AE-EDIT-005",
                    "replacement Record name must match its record target",
                ));
            }
            let index = record_index(program, target_name)?;
            let previous_records = program.records.clone();
            program.records[index] = record;
            reindex_record_references(program, &previous_records)?;
            Ok(())
        }
        (EditTarget::Weave(target_name), EditableDeclaration::Weave(weave)) => {
            if weave.name != *target_name {
                return Err(protocol_error(
                    "AE-EDIT-005",
                    "replacement Weave name must match its weave target",
                ));
            }
            let index = weave_index(program, target_name)?;
            program.weaves[index] = weave;
            Ok(())
        }
        (EditTarget::World, _) => Err(protocol_error(
            "AE-EDIT-004",
            "the world node cannot be replaced by structural edit v7",
        )),
        (EditTarget::Record(_), EditableDeclaration::Weave(_))
        | (EditTarget::Weave(_), EditableDeclaration::Record(_)) => Err(protocol_error(
            "AE-EDIT-005",
            "replacement declaration kind must match its target kind",
        )),
    }
}

fn apply_insert_after(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let target = operation
        .target
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-001", "insertAfter requires a top-level target"))?;
    let declaration = parse_operation_declaration(program, operation, node_budget)?;
    match (target, declaration) {
        (EditTarget::World, EditableDeclaration::Record(record)) => {
            let previous_records = program.records.clone();
            program.records.insert(0, record);
            reindex_record_references(program, &previous_records)?;
            Ok(())
        }
        (EditTarget::World, EditableDeclaration::Weave(_)) => Err(protocol_error(
            "AE-EDIT-005",
            "only a Record may be inserted after the world node",
        )),
        (EditTarget::Record(target_name), EditableDeclaration::Record(record)) => {
            let index = record_index(program, target_name)?;
            let previous_records = program.records.clone();
            program.records.insert(index + 1, record);
            reindex_record_references(program, &previous_records)?;
            Ok(())
        }
        (EditTarget::Weave(target_name), EditableDeclaration::Weave(weave)) => {
            let index = weave_index(program, target_name)?;
            program.weaves.insert(index + 1, weave);
            Ok(())
        }
        (EditTarget::Record(_), EditableDeclaration::Weave(_))
        | (EditTarget::Weave(_), EditableDeclaration::Record(_)) => Err(protocol_error(
            "AE-EDIT-005",
            "insertAfter must preserve Aether's record-before-weave declaration order",
        )),
    }
}

fn apply_delete(program: &mut Program, target: &EditTarget) -> Result<(), StructuralEditError> {
    match target {
        EditTarget::Record(name) => {
            let index = record_index(program, name)?;
            let previous_records = program.records.clone();
            program.records.remove(index);
            reindex_record_references(program, &previous_records)?;
            Ok(())
        }
        EditTarget::Weave(name) => {
            let index = weave_index(program, name)?;
            program.weaves.remove(index);
            Ok(())
        }
        EditTarget::World => Err(protocol_error(
            "AE-EDIT-004",
            "the world node cannot be deleted by structural edit v7",
        )),
    }
}

fn parse_operation_statement(
    program: &Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<Statement, StructuralEditError> {
    let value = operation.statement.as_ref().ok_or_else(|| {
        protocol_error(
            "AE-EDIT-012",
            "statement operation requires a statement payload",
        )
    })?;
    let kind = value
        .as_object()
        .and_then(|object| object.get("kind"))
        .and_then(Value::as_str);
    if matches!(kind, Some("Together")) {
        // Allowed as whole-statement replace of a Together node, but not nested spawn edit.
    }
    if kind.is_none() {
        return Err(protocol_error(
            "AE-EDIT-012",
            "statement payload must be an object with a kind field",
        ));
    }
    parse_statement(value, &program.records, node_budget, 0)
}

fn body_list_mut<'a>(
    program: &'a mut Program,
    list: &BodyListRef,
) -> Result<&'a mut Vec<Statement>, StructuralEditError> {
    match list {
        BodyListRef::WeaveBody { weave } => {
            let index = weave_index(program, weave)?;
            Ok(&mut program.weaves[index].body)
        }
        BodyListRef::ChooseBright { weave, statement }
        | BodyListRef::ChooseDim { weave, statement }
        | BodyListRef::WhileBody { weave, statement } => {
            let weave_index = weave_index(program, weave)?;
            let body = &mut program.weaves[weave_index].body;
            let stmt = body.get_mut(*statement).ok_or_else(|| {
                protocol_error(
                    "AE-EDIT-011",
                    format!("statement index {statement} is out of range for weave {weave}"),
                )
            })?;
            match (list, stmt) {
                (BodyListRef::ChooseBright { .. }, Statement::Choose { when_bright, .. }) => {
                    Ok(when_bright)
                }
                (BodyListRef::ChooseDim { .. }, Statement::Choose { when_dim, .. }) => Ok(when_dim),
                (BodyListRef::WhileBody { .. }, Statement::While { body, .. }) => Ok(body),
                (BodyListRef::ChooseBright { .. } | BodyListRef::ChooseDim { .. }, _) => {
                    Err(protocol_error(
                        "AE-EDIT-013",
                        "nested whenBright/whenDim path requires a Choose statement",
                    ))
                }
                (BodyListRef::WhileBody { .. }, _) => Err(protocol_error(
                    "AE-EDIT-013",
                    "nested body path requires a While statement",
                )),
                (BodyListRef::WeaveBody { .. }, _) => unreachable!("weave body handled above"),
            }
        }
    }
}

fn apply_replace_statement(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let path = operation
        .path
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-010", "replaceStatement requires path"))?;
    let statement = parse_operation_statement(program, operation, node_budget)?;
    let list = body_list_mut(program, &path.list)?;
    if path.index >= list.len() {
        return Err(protocol_error(
            "AE-EDIT-011",
            format!("statement index {} is out of range", path.index),
        ));
    }
    list[path.index] = statement;
    Ok(())
}

fn apply_insert_statement_after(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let path = operation
        .path
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-010", "insertStatementAfter requires path"))?;
    let statement = parse_operation_statement(program, operation, node_budget)?;
    let list = body_list_mut(program, &path.list)?;
    if path.index >= list.len() {
        return Err(protocol_error(
            "AE-EDIT-011",
            format!("statement index {} is out of range", path.index),
        ));
    }
    list.insert(path.index + 1, statement);
    Ok(())
}

fn apply_insert_statement_at(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let list_ref = operation
        .list
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-010", "insertStatementAt requires list"))?;
    let index = operation
        .index
        .ok_or_else(|| protocol_error("AE-EDIT-011", "insertStatementAt requires index"))?;
    let statement = parse_operation_statement(program, operation, node_budget)?;
    let list = body_list_mut(program, list_ref)?;
    if index > list.len() {
        return Err(protocol_error(
            "AE-EDIT-011",
            format!(
                "insert index {index} is out of range for body length {}",
                list.len()
            ),
        ));
    }
    list.insert(index, statement);
    Ok(())
}

fn apply_delete_statement(
    program: &mut Program,
    operation: &EditOperation,
) -> Result<(), StructuralEditError> {
    let path = operation
        .path
        .as_ref()
        .ok_or_else(|| protocol_error("AE-EDIT-010", "deleteStatement requires path"))?;
    let list = body_list_mut(program, &path.list)?;
    if path.index >= list.len() {
        return Err(protocol_error(
            "AE-EDIT-011",
            format!("statement index {} is out of range", path.index),
        ));
    }
    list.remove(path.index);
    Ok(())
}

fn reindex_record_references(
    program: &mut Program,
    previous_records: &[RecordDeclaration],
) -> Result<(), StructuralEditError> {
    for weave in &mut program.weaves {
        remap_record_value_type(&mut weave.result, previous_records, &program.records)?;
        for parameter in &mut weave.parameters {
            remap_record_value_type(
                &mut parameter.value_type,
                previous_records,
                &program.records,
            )?;
        }
    }
    Ok(())
}

fn remap_record_value_type(
    value_type: &mut ValueType,
    previous_records: &[RecordDeclaration],
    current_records: &[RecordDeclaration],
) -> Result<(), StructuralEditError> {
    let ValueType::Record(previous_index) = *value_type else {
        return Ok(());
    };
    let record_name = previous_records
        .get(usize::from(previous_index))
        .map(|record| record.name.as_str())
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-005",
                "existing Program contains an invalid record type reference",
            )
        })?;
    let current_index = current_records
        .iter()
        .position(|record| record.name == record_name)
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-005",
                format!("record edit removes {record_name:?}, which is still used by a weave type"),
            )
        })?;
    let current_index = u16::try_from(current_index).map_err(|_| {
        protocol_error(
            "AE-EDIT-005",
            "record edit exceeds the Aether record identifier range",
        )
    })?;
    *value_type = ValueType::Record(current_index);
    Ok(())
}

fn parse_operation_declaration(
    program: &Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<EditableDeclaration, StructuralEditError> {
    let declaration = operation.declaration.as_ref().ok_or_else(|| {
        protocol_error(
            "AE-EDIT-001",
            "structural operation requires a declaration payload",
        )
    })?;
    parse_declaration(declaration, &program.records, node_budget)
}

fn record_index(program: &Program, name: &str) -> Result<usize, StructuralEditError> {
    program
        .records
        .iter()
        .position(|record| record.name == name)
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-004",
                format!("record target {name:?} does not exist"),
            )
        })
}

fn weave_index(program: &Program, name: &str) -> Result<usize, StructuralEditError> {
    program
        .weaves
        .iter()
        .position(|weave| weave.name == name)
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-004",
                format!("weave target {name:?} does not exist"),
            )
        })
}

fn parse_declaration(
    value: &Value,
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
) -> Result<EditableDeclaration, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "declaration")?;
    let kind = required_string(object, "kind", "declaration")?;
    match kind.as_str() {
        "Record" => parse_record_declaration(object, node_budget).map(EditableDeclaration::Record),
        "Weave" => {
            parse_weave_declaration(object, records, node_budget).map(EditableDeclaration::Weave)
        }
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("declaration has unsupported kind {kind:?}"),
        )),
    }
}

fn parse_record_declaration(
    object: &Map<String, Value>,
    node_budget: &mut NodeBudget,
) -> Result<RecordDeclaration, StructuralEditError> {
    ensure_only_fields(object, &["kind", "name", "fields"], "Record declaration")?;
    require_kind(object, "Record", "Record declaration")?;
    let name = required_string(object, "name", "Record declaration")?;
    let fields = required_array(object, "fields", "Record declaration")?
        .iter()
        .map(|value| parse_record_field(value, node_budget))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RecordDeclaration {
        name,
        fields,
        span: synthetic_span(),
    })
}

fn parse_record_field(
    value: &Value,
    node_budget: &mut NodeBudget,
) -> Result<RecordField, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "Record field")?;
    ensure_only_fields(object, &["kind", "name", "type"], "Record field")?;
    require_kind(object, "RecordField", "Record field")?;
    let name = required_string(object, "name", "Record field")?;
    let value_type = match required_string(object, "type", "Record field")?.as_str() {
        "Text" => ValueType::Text,
        "Whole" => ValueType::Whole,
        "Truth" => ValueType::Truth,
        "Bytes" => ValueType::Bytes,
        other => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("Record field type {other:?} is not a primitive Aether value type"),
            ));
        }
    };
    Ok(RecordField {
        name,
        value_type,
        span: synthetic_span(),
    })
}

fn parse_weave_declaration(
    object: &Map<String, Value>,
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
) -> Result<Weave, StructuralEditError> {
    ensure_only_fields(
        object,
        &[
            "kind",
            "name",
            "parameters",
            "result",
            "effect",
            "task",
            "body",
        ],
        "Weave declaration",
    )?;
    require_kind(object, "Weave", "Weave declaration")?;
    let name = required_string(object, "name", "Weave declaration")?;
    let parameters = required_array(object, "parameters", "Weave declaration")?
        .iter()
        .map(|value| parse_parameter(value, records, node_budget))
        .collect::<Result<Vec<_>, _>>()?;
    let result = parse_value_type(
        required_value(object, "result", "Weave declaration")?,
        records,
    )?;
    let effect = match required_string(object, "effect", "Weave declaration")?.as_str() {
        "Total" => Effect::Total,
        "ErrorWhole" => Effect::ErrorWhole,
        other => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("Weave effect {other:?} is not a supported Aether effect"),
            ));
        }
    };
    let task = required_bool(object, "task", "Weave declaration")?;
    let body = parse_statements(
        required_array(object, "body", "Weave declaration")?,
        records,
        node_budget,
        0,
    )?;
    Ok(Weave {
        name,
        parameters,
        result,
        effect,
        task,
        body,
        span: synthetic_span(),
    })
}

fn parse_parameter(
    value: &Value,
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
) -> Result<Parameter, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "Weave parameter")?;
    ensure_only_fields(object, &["kind", "name", "mode", "type"], "Weave parameter")?;
    require_kind(object, "Parameter", "Weave parameter")?;
    let name = required_string(object, "name", "Weave parameter")?;
    let mode = match required_string(object, "mode", "Weave parameter")?.as_str() {
        "own" => ParameterMode::Own,
        "borrow" => ParameterMode::Borrow,
        "access" => ParameterMode::Access,
        other => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("Weave parameter mode {other:?} is not valid"),
            ));
        }
    };
    let value_type = parse_value_type(required_value(object, "type", "Weave parameter")?, records)?;
    Ok(Parameter {
        name,
        value_type,
        mode,
        span: synthetic_span(),
    })
}

fn parse_value_type(
    value: &Value,
    records: &[RecordDeclaration],
) -> Result<ValueType, StructuralEditError> {
    let name = value
        .as_str()
        .ok_or_else(|| protocol_error("AE-EDIT-001", "Aether value type must be a JSON string"))?;
    match name {
        "Text" => Ok(ValueType::Text),
        "Whole" => Ok(ValueType::Whole),
        "Truth" => Ok(ValueType::Truth),
        "Bytes" => Ok(ValueType::Bytes),
        "Arena" => Ok(ValueType::Arena),
        "BufferWhole" => Ok(ValueType::BufferWhole),
        "BufferTruth" => Ok(ValueType::BufferTruth),
        "AccessArena" => Err(protocol_error(
            "AE-EDIT-001",
            "AccessArena is VM-private and cannot appear in structural source payloads",
        )),
        record_name => records
            .iter()
            .position(|record| record.name == record_name)
            .and_then(|index| u16::try_from(index).ok())
            .map(ValueType::Record)
            .ok_or_else(|| {
                protocol_error(
                    "AE-EDIT-001",
                    format!(
                        "{record_name:?} is not a declared Aether value type in this edit state"
                    ),
                )
            }),
    }
}

fn parse_statements(
    values: &[Value],
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
    depth: usize,
) -> Result<Vec<Statement>, StructuralEditError> {
    values
        .iter()
        .map(|value| parse_statement(value, records, node_budget, depth))
        .collect()
}

fn parse_statement(
    value: &Value,
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
    depth: usize,
) -> Result<Statement, StructuralEditError> {
    if depth > MAX_STRUCTURAL_EDIT_DEPTH {
        return Err(protocol_error(
            "AE-EDIT-006",
            format!(
                "structural edit exceeds the {MAX_STRUCTURAL_EDIT_DEPTH}-level nesting safety limit"
            ),
        ));
    }
    node_budget.consume()?;
    let object = expect_object(value, "statement")?;
    let kind = required_string(object, "kind", "statement")?;
    match kind.as_str() {
        "Bind" => {
            ensure_only_fields(
                object,
                &["kind", "name", "mutable", "stage", "value"],
                "Bind statement",
            )?;
            let stage = required_string(object, "stage", "Bind statement")?;
            let comptime = match stage.as_str() {
                "runtime" => false,
                "comptime" => true,
                other => {
                    return Err(protocol_error(
                        "AE-EDIT-001",
                        format!("Bind stage {other:?} is not a supported Aether stage"),
                    ));
                }
            };
            Ok(Statement::Bind {
                name: required_string(object, "name", "Bind statement")?,
                mutable: required_bool(object, "mutable", "Bind statement")?,
                comptime,
                value: parse_expression(
                    required_value(object, "value", "Bind statement")?,
                    records,
                    node_budget,
                )?,
                span: synthetic_span(),
            })
        }
        "Revise" => {
            ensure_only_fields(object, &["kind", "name", "value"], "Revise statement")?;
            Ok(Statement::Revise {
                name: required_string(object, "name", "Revise statement")?,
                value: parse_expression(
                    required_value(object, "value", "Revise statement")?,
                    records,
                    node_budget,
                )?,
                span: synthetic_span(),
            })
        }
        "Speak" => {
            ensure_only_fields(object, &["kind", "value"], "Speak statement")?;
            Ok(Statement::Speak {
                value: parse_expression(
                    required_value(object, "value", "Speak statement")?,
                    records,
                    node_budget,
                )?,
                span: synthetic_span(),
            })
        }
        "Release" => {
            ensure_only_fields(object, &["kind", "name"], "Release statement")?;
            Ok(Statement::Release {
                name: required_string(object, "name", "Release statement")?,
                span: synthetic_span(),
            })
        }
        "Checkpoint" => {
            ensure_only_fields(object, &["kind"], "Checkpoint statement")?;
            Ok(Statement::Checkpoint {
                span: synthetic_span(),
            })
        }
        "Yield" => {
            ensure_only_fields(object, &["kind", "value"], "Yield statement")?;
            Ok(Statement::Yield {
                value: parse_expression(
                    required_value(object, "value", "Yield statement")?,
                    records,
                    node_budget,
                )?,
                span: synthetic_span(),
            })
        }
        "Raise" => {
            ensure_only_fields(object, &["kind", "code"], "Raise statement")?;
            Ok(Statement::Raise {
                code: parse_atom(
                    required_value(object, "code", "Raise statement")?,
                    node_budget,
                )?,
                span: synthetic_span(),
            })
        }
        "Forward" => {
            ensure_only_fields(object, &["kind", "weave", "arguments"], "Forward statement")?;
            Ok(Statement::Forward {
                weave: required_string(object, "weave", "Forward statement")?,
                arguments: required_array(object, "arguments", "Forward statement")?
                    .iter()
                    .map(|argument| parse_atom(argument, node_budget))
                    .collect::<Result<Vec<_>, _>>()?,
                span: synthetic_span(),
            })
        }
        "Handle" => {
            ensure_only_fields(
                object,
                &[
                    "kind",
                    "weave",
                    "arguments",
                    "successDestination",
                    "errorDestination",
                ],
                "Handle statement",
            )?;
            Ok(Statement::Handle {
                weave: required_string(object, "weave", "Handle statement")?,
                arguments: required_array(object, "arguments", "Handle statement")?
                    .iter()
                    .map(|argument| parse_atom(argument, node_budget))
                    .collect::<Result<Vec<_>, _>>()?,
                success_destination: required_string(
                    object,
                    "successDestination",
                    "Handle statement",
                )?,
                error_destination: required_string(object, "errorDestination", "Handle statement")?,
                span: synthetic_span(),
            })
        }
        "Choose" => {
            ensure_only_fields(
                object,
                &["kind", "condition", "whenBright", "whenDim"],
                "Choose statement",
            )?;
            Ok(Statement::Choose {
                condition: parse_expression(
                    required_value(object, "condition", "Choose statement")?,
                    records,
                    node_budget,
                )?,
                when_bright: parse_statements(
                    required_array(object, "whenBright", "Choose statement")?,
                    records,
                    node_budget,
                    depth + 1,
                )?,
                when_dim: parse_statements(
                    required_array(object, "whenDim", "Choose statement")?,
                    records,
                    node_budget,
                    depth + 1,
                )?,
                span: synthetic_span(),
            })
        }
        "While" => {
            ensure_only_fields(object, &["kind", "condition", "body"], "While statement")?;
            Ok(Statement::While {
                condition: parse_expression(
                    required_value(object, "condition", "While statement")?,
                    records,
                    node_budget,
                )?,
                body: parse_statements(
                    required_array(object, "body", "While statement")?,
                    records,
                    node_budget,
                    depth + 1,
                )?,
                span: synthetic_span(),
            })
        }
        "Together" => {
            ensure_only_fields(object, &["kind", "spawns"], "Together statement")?;
            let spawns = required_array(object, "spawns", "Together statement")?
                .iter()
                .map(|value| parse_spawn(value, node_budget))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Statement::Together {
                spawns,
                span: synthetic_span(),
            })
        }
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("statement has unsupported kind {kind:?}"),
        )),
    }
}

fn parse_spawn(
    value: &Value,
    node_budget: &mut NodeBudget,
) -> Result<crate::Spawn, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "spawn")?;
    ensure_only_fields(
        object,
        &["kind", "weave", "arguments", "destination"],
        "Spawn",
    )?;
    let kind = required_string(object, "kind", "Spawn")?;
    if kind != "Spawn" {
        return Err(protocol_error(
            "AE-EDIT-001",
            format!("spawn has unsupported kind {kind:?}"),
        ));
    }
    Ok(crate::Spawn {
        weave: required_string(object, "weave", "Spawn")?,
        arguments: required_array(object, "arguments", "Spawn")?
            .iter()
            .map(|argument| parse_atom(argument, node_budget))
            .collect::<Result<Vec<_>, _>>()?,
        destination: required_string(object, "destination", "Spawn")?,
        span: synthetic_span(),
    })
}

fn parse_expression(
    value: &Value,
    records: &[RecordDeclaration],
    node_budget: &mut NodeBudget,
) -> Result<Expression, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "expression")?;
    let kind = required_string(object, "kind", "expression")?;
    let kind = match kind.as_str() {
        "Atom" => {
            ensure_only_fields(object, &["kind", "atom"], "Atom expression")?;
            ExpressionKind::Atom(parse_atom(
                required_value(object, "atom", "Atom expression")?,
                node_budget,
            )?)
        }
        "Arena" => {
            ensure_only_fields(object, &["kind", "capacity"], "Arena expression")?;
            let capacity = required_value(object, "capacity", "Arena expression")?
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(|| {
                    protocol_error(
                        "AE-EDIT-001",
                        "Arena capacity must be an unsigned 32-bit JSON integer",
                    )
                })?;
            ExpressionKind::Arena { capacity }
        }
        "Buffer" => {
            ensure_only_fields(object, &["kind", "element"], "Buffer expression")?;
            ExpressionKind::Buffer {
                element: parse_buffer_element(
                    required_string(object, "element", "Buffer expression")?.as_str(),
                )?,
            }
        }
        "Unary" => {
            ensure_only_fields(
                object,
                &["kind", "operation", "argument"],
                "Unary expression",
            )?;
            ExpressionKind::Unary {
                operation: parse_unary_operation(
                    required_string(object, "operation", "Unary expression")?.as_str(),
                )?,
                argument: parse_atom(
                    required_value(object, "argument", "Unary expression")?,
                    node_budget,
                )?,
            }
        }
        "Binary" => {
            ensure_only_fields(
                object,
                &["kind", "operation", "left", "right"],
                "Binary expression",
            )?;
            ExpressionKind::Binary {
                operation: parse_binary_operation(
                    required_string(object, "operation", "Binary expression")?.as_str(),
                )?,
                left: parse_atom(
                    required_value(object, "left", "Binary expression")?,
                    node_budget,
                )?,
                right: parse_atom(
                    required_value(object, "right", "Binary expression")?,
                    node_budget,
                )?,
            }
        }
        "Cut" => {
            ensure_only_fields(object, &["kind", "text", "start", "end"], "Cut expression")?;
            ExpressionKind::Cut {
                text: parse_atom(
                    required_value(object, "text", "Cut expression")?,
                    node_budget,
                )?,
                start: parse_atom(
                    required_value(object, "start", "Cut expression")?,
                    node_budget,
                )?,
                end: parse_atom(
                    required_value(object, "end", "Cut expression")?,
                    node_budget,
                )?,
            }
        }
        "Slice" => {
            ensure_only_fields(
                object,
                &["kind", "bytes", "start", "end"],
                "Slice expression",
            )?;
            ExpressionKind::Slice {
                bytes: parse_atom(
                    required_value(object, "bytes", "Slice expression")?,
                    node_budget,
                )?,
                start: parse_atom(
                    required_value(object, "start", "Slice expression")?,
                    node_budget,
                )?,
                end: parse_atom(
                    required_value(object, "end", "Slice expression")?,
                    node_budget,
                )?,
            }
        }
        "Ternary" => {
            ensure_only_fields(
                object,
                &["kind", "operation", "first", "second", "third"],
                "Ternary expression",
            )?;
            ExpressionKind::Ternary {
                operation: parse_ternary_operation(
                    required_string(object, "operation", "Ternary expression")?.as_str(),
                )?,
                first: parse_atom(
                    required_value(object, "first", "Ternary expression")?,
                    node_budget,
                )?,
                second: parse_atom(
                    required_value(object, "second", "Ternary expression")?,
                    node_budget,
                )?,
                third: parse_atom(
                    required_value(object, "third", "Ternary expression")?,
                    node_budget,
                )?,
            }
        }
        "Call" => {
            ensure_only_fields(object, &["kind", "weave", "arguments"], "Call expression")?;
            ExpressionKind::Call {
                weave: required_string(object, "weave", "Call expression")?,
                arguments: parse_atoms(
                    required_array(object, "arguments", "Call expression")?,
                    node_budget,
                )?,
            }
        }
        "MakeRecord" => {
            ensure_only_fields(
                object,
                &["kind", "record", "fields"],
                "MakeRecord expression",
            )?;
            ExpressionKind::MakeRecord {
                record: required_string(object, "record", "MakeRecord expression")?,
                fields: parse_atoms(
                    required_array(object, "fields", "MakeRecord expression")?,
                    node_budget,
                )?,
            }
        }
        "Field" => {
            ensure_only_fields(object, &["kind", "record", "field"], "Field expression")?;
            ExpressionKind::Field {
                record: parse_atom(
                    required_value(object, "record", "Field expression")?,
                    node_budget,
                )?,
                field: required_string(object, "field", "Field expression")?,
            }
        }
        "Resource" => parse_resource_expression(object, node_budget)?,
        _ => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("expression has unsupported kind {kind:?}"),
            ));
        }
    };
    let _ = records;
    Ok(Expression {
        kind,
        span: synthetic_span(),
    })
}

fn parse_resource_expression(
    object: &Map<String, Value>,
    node_budget: &mut NodeBudget,
) -> Result<ExpressionKind, StructuralEditError> {
    let operation = required_string(object, "operation", "Resource expression")?;
    let resource = match operation.as_str() {
        "allocate" => {
            ensure_only_fields(
                object,
                &[
                    "kind",
                    "operation",
                    "arena",
                    "buffer",
                    "capacity",
                    "destination",
                ],
                "allocate resource expression",
            )?;
            ResourceOperation::Allocate {
                arena: parse_atom(
                    required_value(object, "arena", "allocate resource expression")?,
                    node_budget,
                )?,
                buffer: parse_atom(
                    required_value(object, "buffer", "allocate resource expression")?,
                    node_budget,
                )?,
                capacity: parse_atom(
                    required_value(object, "capacity", "allocate resource expression")?,
                    node_budget,
                )?,
                destination: required_string(
                    object,
                    "destination",
                    "allocate resource expression",
                )?,
            }
        }
        "append" => {
            ensure_only_fields(
                object,
                &["kind", "operation", "buffer", "value", "destination"],
                "append resource expression",
            )?;
            ResourceOperation::Append {
                buffer: parse_atom(
                    required_value(object, "buffer", "append resource expression")?,
                    node_budget,
                )?,
                value: parse_atom(
                    required_value(object, "value", "append resource expression")?,
                    node_budget,
                )?,
                destination: required_string(object, "destination", "append resource expression")?,
            }
        }
        "at" => {
            ensure_only_fields(
                object,
                &["kind", "operation", "buffer", "index", "destination"],
                "at resource expression",
            )?;
            ResourceOperation::At {
                buffer: parse_atom(
                    required_value(object, "buffer", "at resource expression")?,
                    node_budget,
                )?,
                index: parse_atom(
                    required_value(object, "index", "at resource expression")?,
                    node_budget,
                )?,
                destination: required_string(object, "destination", "at resource expression")?,
            }
        }
        _ => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("Resource operation {operation:?} is not supported"),
            ));
        }
    };
    Ok(ExpressionKind::Resource(resource))
}

fn parse_atoms(
    values: &[Value],
    node_budget: &mut NodeBudget,
) -> Result<Vec<Atom>, StructuralEditError> {
    values
        .iter()
        .map(|value| parse_atom(value, node_budget))
        .collect()
}

fn parse_atom(value: &Value, node_budget: &mut NodeBudget) -> Result<Atom, StructuralEditError> {
    node_budget.consume()?;
    let object = expect_object(value, "atom")?;
    let kind = required_string(object, "kind", "atom")?;
    let kind = match kind.as_str() {
        "Text" => {
            ensure_only_fields(object, &["kind", "value"], "Text atom")?;
            AtomKind::Text(required_string(object, "value", "Text atom")?)
        }
        "Bytes" => {
            ensure_only_fields(object, &["kind", "value"], "Bytes atom")?;
            AtomKind::Bytes(decode_hex(&required_string(
                object,
                "value",
                "Bytes atom",
            )?)?)
        }
        "Whole" => {
            ensure_only_fields(object, &["kind", "value"], "Whole atom")?;
            let whole = required_value(object, "value", "Whole atom")?
                .as_i64()
                .ok_or_else(|| {
                    protocol_error("AE-EDIT-001", "Whole atom value must be an integer")
                })?;
            AtomKind::Whole(whole)
        }
        "Truth" => {
            ensure_only_fields(object, &["kind", "value"], "Truth atom")?;
            let truth = required_value(object, "value", "Truth atom")?
                .as_bool()
                .ok_or_else(|| {
                    protocol_error("AE-EDIT-001", "Truth atom value must be a boolean")
                })?;
            AtomKind::Truth(truth)
        }
        "Name" => {
            ensure_only_fields(object, &["kind", "name"], "Name atom")?;
            AtomKind::Name(required_string(object, "name", "Name atom")?)
        }
        "Borrow" => {
            ensure_only_fields(object, &["kind", "name"], "Borrow atom")?;
            AtomKind::Borrow(required_string(object, "name", "Borrow atom")?)
        }
        "Move" => {
            ensure_only_fields(object, &["kind", "name"], "Move atom")?;
            AtomKind::Move(required_string(object, "name", "Move atom")?)
        }
        "Access" => {
            ensure_only_fields(object, &["kind", "name"], "Access atom")?;
            AtomKind::Access(required_string(object, "name", "Access atom")?)
        }
        _ => {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("atom has unsupported kind {kind:?}"),
            ));
        }
    };
    Ok(Atom {
        kind,
        span: synthetic_span(),
    })
}

fn decode_hex(value: &str) -> Result<Vec<u8>, StructuralEditError> {
    if value.len() & 1 != 0 || value.len() / 2 > MAX_SOURCE_BYTES {
        return Err(protocol_error(
            "AE-EDIT-001",
            "Bytes atom must contain an even, bounded number of hexadecimal digits",
        ));
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        let high = hex_nibble(pair[0]).ok_or_else(|| {
            protocol_error(
                "AE-EDIT-001",
                "Bytes atom must use lowercase hexadecimal digits",
            )
        })?;
        let low = hex_nibble(pair[1]).ok_or_else(|| {
            protocol_error(
                "AE-EDIT-001",
                "Bytes atom must use lowercase hexadecimal digits",
            )
        })?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn parse_buffer_element(value: &str) -> Result<BufferElement, StructuralEditError> {
    match value {
        "Whole" => Ok(BufferElement::Whole),
        "Truth" => Ok(BufferElement::Truth),
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("Buffer element {value:?} must be Whole or Truth"),
        )),
    }
}

fn parse_unary_operation(value: &str) -> Result<UnaryOperation, StructuralEditError> {
    match value {
        "not" => Ok(UnaryOperation::Not),
        "measure" => Ok(UnaryOperation::Measure),
        "render" => Ok(UnaryOperation::Render),
        "extent" => Ok(UnaryOperation::Extent),
        "encode" => Ok(UnaryOperation::Encode),
        "decode" => Ok(UnaryOperation::Decode),
        "number" => Ok(UnaryOperation::Number),
        "pack16" => Ok(UnaryOperation::Pack16),
        "pack32" => Ok(UnaryOperation::Pack32),
        "pack64" => Ok(UnaryOperation::Pack64),
        "count" => Ok(UnaryOperation::Count),
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("Unary operation {value:?} is not supported"),
        )),
    }
}

fn parse_binary_operation(value: &str) -> Result<BinaryOperation, StructuralEditError> {
    match value {
        "sum" => Ok(BinaryOperation::Sum),
        "difference" => Ok(BinaryOperation::Difference),
        "product" => Ok(BinaryOperation::Product),
        "less" => Ok(BinaryOperation::Less),
        "same" => Ok(BinaryOperation::Same),
        "join" => Ok(BinaryOperation::Join),
        "glyph" => Ok(BinaryOperation::Glyph),
        "quotient" => Ok(BinaryOperation::Quotient),
        "remainder" => Ok(BinaryOperation::Remainder),
        "fuse" => Ok(BinaryOperation::Fuse),
        "append" => Ok(BinaryOperation::Append),
        "octet" => Ok(BinaryOperation::Octet),
        "unpack16" => Ok(BinaryOperation::Unpack16),
        "unpack32" => Ok(BinaryOperation::Unpack32),
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("Binary operation {value:?} is not supported"),
        )),
    }
}

fn parse_ternary_operation(value: &str) -> Result<TernaryOperation, StructuralEditError> {
    match value {
        "seek" => Ok(TernaryOperation::Seek),
        "poke" => Ok(TernaryOperation::Poke),
        "poke32" => Ok(TernaryOperation::Poke32),
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("Ternary operation {value:?} is not supported"),
        )),
    }
}

fn expect_object<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a Map<String, Value>, StructuralEditError> {
    value
        .as_object()
        .ok_or_else(|| protocol_error("AE-EDIT-001", format!("{context} must be a JSON object")))
}

fn ensure_only_fields(
    object: &Map<String, Value>,
    allowed: &[&str],
    context: &str,
) -> Result<(), StructuralEditError> {
    for field in object.keys() {
        if !allowed.contains(&field.as_str()) {
            return Err(protocol_error(
                "AE-EDIT-001",
                format!("{context} contains unsupported field {field:?}"),
            ));
        }
    }
    Ok(())
}

fn required_value<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a Value, StructuralEditError> {
    object.get(field).ok_or_else(|| {
        protocol_error(
            "AE-EDIT-001",
            format!("{context} is missing required field {field:?}"),
        )
    })
}

fn required_string(
    object: &Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<String, StructuralEditError> {
    required_value(object, field, context)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-001",
                format!("{context} field {field:?} must be a JSON string"),
            )
        })
}

fn required_bool(
    object: &Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<bool, StructuralEditError> {
    required_value(object, field, context)?
        .as_bool()
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-001",
                format!("{context} field {field:?} must be a JSON boolean"),
            )
        })
}

fn required_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a [Value], StructuralEditError> {
    required_value(object, field, context)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| {
            protocol_error(
                "AE-EDIT-001",
                format!("{context} field {field:?} must be a JSON array"),
            )
        })
}

fn require_kind(
    object: &Map<String, Value>,
    expected: &str,
    context: &str,
) -> Result<(), StructuralEditError> {
    let actual = required_string(object, "kind", context)?;
    if actual == expected {
        Ok(())
    } else {
        Err(protocol_error(
            "AE-EDIT-001",
            format!("{context} kind must be {expected:?}, not {actual:?}"),
        ))
    }
}

fn protocol_error(code: &'static str, message: impl Into<String>) -> StructuralEditError {
    StructuralEditError {
        diagnostic: Diagnostic {
            code,
            span: synthetic_span(),
            message: message.into(),
        },
    }
}

const fn synthetic_span() -> Span {
    Span { line: 1, column: 1 }
}

/// `serde_json::Value` silently overwrites duplicate object keys when using its
/// normal deserializer. The authoring protocol rejects duplicates at every
/// object level so a generator cannot make the apparent and applied edits differ.
struct StrictJsonValue(Value);

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value with no duplicate object keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::Number(Number::from(value))))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::Number(Number::from(value))))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("JSON number is not finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::String(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(Value::Null))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(StrictJsonValue(value)) = sequence.next_element()? {
            values.push(value);
        }
        Ok(StrictJsonValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut seen = BTreeSet::new();
        let mut object = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!(
                    "duplicate JSON object key {key:?}"
                )));
            }
            let StrictJsonValue(value) = map.next_value()?;
            object.insert(key, value);
        }
        Ok(StrictJsonValue(Value::Object(object)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compile_to_bytecode, compile_with_seed};

    const BASE_SOURCE: &str = "world authoring\n\nweave main [] -> Whole:\n  yield 0\n";

    fn replacement_edit(base_source: &str, value: i64) -> String {
        format!(
            r#"{{
  "protocol": "aether.edit/v8",
  "schema": "aether.ast/v8",
  "baseSource": {},
  "operations": [{{
    "op": "replace",
    "target": "weave:main",
    "declaration": {{
      "kind": "Weave",
      "name": "main",
      "parameters": [],
      "result": "Whole",
      "effect": "Total",
      "task": false,
      "body": [{{
        "kind": "Yield",
        "value": {{
          "kind": "Atom",
          "atom": {{ "kind": "Whole", "value": {value} }}
        }}
      }}]
    }}
  }}]
}}"#,
            serde_json::to_string(base_source).expect("source string must serialize")
        )
    }

    fn shipped_examples() -> [(&'static str, &'static str); 19] {
        [
            ("welcome", include_str!("../../../examples/welcome.ae")),
            (
                "control-flow",
                include_str!("../../../examples/control-flow.ae"),
            ),
            ("weaves", include_str!("../../../examples/weaves.ae")),
            ("unicode", include_str!("../../../examples/unicode.ae")),
            (
                "seed-multi-weave",
                include_str!("../../../examples/seed-multi-weave.ae"),
            ),
            (
                "seed-forward-call",
                include_str!("../../../examples/seed-forward-call.ae"),
            ),
            ("records", include_str!("../../../examples/records.ae")),
            (
                "arena-buffer",
                include_str!("../../../examples/arena-buffer.ae"),
            ),
            (
                "arena-exhausted",
                include_str!("../../../examples/arena-exhausted.ae"),
            ),
            (
                "arena-full",
                include_str!("../../../examples/arena-full.ae"),
            ),
            (
                "arena-lookup-fallback",
                include_str!("../../../examples/arena-lookup-fallback.ae"),
            ),
            (
                "arena-truth-buffer",
                include_str!("../../../examples/arena-truth-buffer.ae"),
            ),
            (
                "arena-access-weave",
                include_str!("../../../examples/arena-access-weave.ae"),
            ),
            (
                "active-cancel",
                include_str!("../../../examples/active-cancel.ae"),
            ),
            (
                "task-frame-capacity",
                include_str!("../../../examples/task-frame-capacity.ae"),
            ),
            ("task-loop", include_str!("../../../examples/task-loop.ae")),
            (
                "error-effect",
                include_str!("../../../examples/error-effect.ae"),
            ),
            ("comptime", include_str!("../../../examples/comptime.ae")),
            (
                "comptime-calls",
                include_str!("../../../examples/comptime-calls.ae"),
            ),
        ]
    }

    #[test]
    fn product_structure_json_accepts_without_bootstrap_ast() {
        assert!(
            crate::product_structure_without_bootstrap(),
            "ADR-054 tracker"
        );
        let source = "world prod\r\n\r\nweave main [] -> Whole:\r\n  yield 1\r\n";
        let document = product_structure_json(source).expect("product structure");
        assert!(
            document.contains(PRODUCT_STRUCTURE_SCHEMA_VERSION),
            "schema missing: {document}"
        );
        assert!(document.contains("\"productAccepted\": true"), "{document}");
        assert!(
            document.contains("world prod\\n\\nweave main"),
            "{document}"
        );
        assert!(!document.contains(STRUCTURAL_AST_SCHEMA_VERSION));
        let legacy = product_structure_json("world w\n\nfn main() -> Int { return 0; }\n")
            .expect_err("legacy must fail");
        assert!(legacy.to_string().contains("AE-SEED-007"), "got {legacy}");
    }

    #[test]
    fn structural_document_round_trips_canonical_m2_source() {
        let source = include_str!("../../../examples/arena-buffer.ae");
        let document = structural_document_json(source).expect("M2 source should describe");
        let parsed: Value = serde_json::from_str(&document).expect("document should be JSON");
        assert_eq!(parsed["schema"], STRUCTURAL_AST_SCHEMA_VERSION);
        assert_eq!(parsed["program"]["kind"], "Program");
        assert!(parsed["program"]["weaves"]
            .as_array()
            .is_some_and(|weaves| !weaves.is_empty()));
        let canonical_source = parsed["canonicalSource"]
            .as_str()
            .expect("document must include canonical source");
        let bootstrap = compile_to_bytecode(canonical_source)
            .expect("canonical source should bootstrap compile");
        let seed =
            compile_with_seed(canonical_source).expect("canonical source should seed compile");
        assert_eq!(bootstrap.bytecode, seed.bytecode);
        assert_eq!(
            document,
            structural_document_json(source).expect("document must be deterministic")
        );
    }

    #[test]
    fn structural_document_exposes_the_bounded_m4_effect_nodes() {
        let source = include_str!("../../../examples/error-effect.ae");
        let document = structural_document_json(source).expect("M4 source should describe");
        let parsed: Value = serde_json::from_str(&document).expect("document should be JSON");
        let weaves = parsed["program"]["weaves"]
            .as_array()
            .expect("M4 document must contain weaves");
        assert!(weaves.iter().any(|weave| weave["effect"] == "ErrorWhole"));
        assert!(weaves.iter().any(|weave| {
            weave["body"]
                .as_array()
                .is_some_and(|body| body.iter().any(|statement| statement["kind"] == "Handle"))
        }));
        let canonical_source = parsed["canonicalSource"]
            .as_str()
            .expect("M4 document must include canonical source");
        let result = apply_structural_edit(canonical_source, &identity_edit_from_document(&parsed));
        assert_eq!(
            result.expect("M4 identity edit should apply").source,
            canonical_source
        );
    }

    #[test]
    fn structural_document_exposes_m5_comptime_stage_provenance() {
        let source = include_str!("../../../examples/comptime.ae");
        let document = structural_document_json(source).expect("M5 source should describe");
        let parsed: Value = serde_json::from_str(&document).expect("document should be JSON");
        let body = parsed["program"]["weaves"][0]["body"]
            .as_array()
            .expect("M5 document must contain a weave body");
        assert_eq!(body[0]["kind"], "Bind");
        assert_eq!(body[0]["stage"], "comptime");
        assert_eq!(body[5]["stage"], "runtime");
        let canonical_source = parsed["canonicalSource"]
            .as_str()
            .expect("M5 document must include canonical source");
        let result = apply_structural_edit(canonical_source, &identity_edit_from_document(&parsed));
        assert_eq!(
            result.expect("M5 identity edit should apply").source,
            canonical_source
        );
    }

    #[test]
    fn structural_document_round_trips_m19e_task_and_checkpoint_nodes() {
        let source = "world author_task\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave main [] -> Whole:\n  bind mutable value <- 0\n  together:\n    spawn call worker into value\n  yield value\n";
        let document = structural_document_json(source).expect("M19e source should describe");
        let parsed: Value = serde_json::from_str(&document).expect("document must be JSON");
        assert_eq!(parsed["schema"], STRUCTURAL_AST_SCHEMA_VERSION);
        let worker = parsed["program"]["weaves"]
            .as_array()
            .expect("document must contain weaves")
            .iter()
            .find(|weave| weave["name"] == "worker")
            .expect("document must retain the task weave");
        assert_eq!(worker["task"], true);
        assert_eq!(worker["body"][0]["kind"], "Checkpoint");

        let canonical_source = parsed["canonicalSource"]
            .as_str()
            .expect("document must retain canonical source");
        let identity =
            apply_structural_edit(canonical_source, &identity_edit_from_document(&parsed))
                .expect("M19e identity edit should preserve the closed task subset");
        assert_eq!(identity.source, canonical_source);
        let bytecode = compile_to_bytecode(&identity.source)
            .expect("M19e authoring round trip must remain bootstrap-compilable");
        assert_eq!(bytecode.bytecode[4], crate::ARTIFACT_VERSION_V12);

        let mut invalid_worker = worker.clone();
        remove_generated_provenance(&mut invalid_worker);
        invalid_worker["body"] = json!([{
            "kind": "Yield",
            "value": {
                "kind": "Atom",
                "atom": { "kind": "Whole", "value": 1 },
            },
        }]);
        let invalid_edit = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical_source,
            "operations": [{
                "op": "replace",
                "target": "weave:worker",
                "declaration": invalid_worker,
            }],
        }))
        .expect("invalid edit should serialize");
        let error = apply_structural_edit(canonical_source, &invalid_edit)
            .expect_err("structural edits must not bypass task checkpoint validation");
        // ADR-048: accept gate is product seed; verifier surfaces checkpoint rules as
        // AE-TASK-004 (mapped from TASK_CHECKPOINT product failure) or AE-SEED-002.
        let code = error.diagnostic().code;
        assert!(
            code == "AE-TASK-004" || code == "AE-SEED-002",
            "expected AE-TASK-004 or AE-SEED-002, got {code}: {}",
            error.diagnostic().message
        );
    }

    #[test]
    fn structural_document_round_trips_m23_comptime_call_without_a_protocol_bump() {
        let source = include_str!("../../../examples/comptime-calls.ae");
        let document = structural_document_json(source).expect("M23 source should describe");
        let parsed: Value = serde_json::from_str(&document).expect("document should be JSON");
        assert_eq!(parsed["schema"], STRUCTURAL_AST_SCHEMA_VERSION);
        let main = parsed["program"]["weaves"]
            .as_array()
            .expect("M23 document must contain weaves")
            .iter()
            .find(|weave| weave["name"] == "main")
            .expect("M23 document must contain main");
        let body = main["body"]
            .as_array()
            .expect("M23 main must contain a body");
        assert_eq!(body[1]["stage"], "comptime");
        assert_eq!(body[1]["value"]["kind"], "Call");
        assert_eq!(body[1]["value"]["weave"], "double");
        let canonical_source = parsed["canonicalSource"]
            .as_str()
            .expect("M23 document must include canonical source");
        let result = apply_structural_edit(canonical_source, &identity_edit_from_document(&parsed));
        assert_eq!(
            result.expect("M23 identity edit should apply").source,
            canonical_source
        );
    }

    #[test]
    fn structural_edits_reject_missing_or_unknown_m5_binding_stage() {
        let source = include_str!("../../../examples/comptime.ae");
        let document: Value = serde_json::from_str(
            &structural_document_json(source).expect("M5 source should describe"),
        )
        .expect("M5 document should be JSON");
        let mut declaration = document["program"]["weaves"][0].clone();
        remove_generated_provenance(&mut declaration);

        let body = declaration["body"]
            .as_array_mut()
            .expect("editable weave must contain a body");
        body[0]
            .as_object_mut()
            .expect("first editable statement must be an object")
            .remove("stage");
        let missing_stage = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": source,
            "operations": [{
                "op": "replace",
                "target": "weave:main",
                "declaration": declaration,
            }],
        }))
        .expect("missing-stage edit should serialize");
        let error = apply_structural_edit(source, &missing_stage)
            .expect_err("v6 edits must require every binding stage");
        assert_eq!(error.diagnostic().code, "AE-EDIT-001");

        let mut unknown_declaration = document["program"]["weaves"][0].clone();
        remove_generated_provenance(&mut unknown_declaration);
        unknown_declaration["body"][0]["stage"] = Value::String("later".to_owned());
        let unknown_stage = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": source,
            "operations": [{
                "op": "replace",
                "target": "weave:main",
                "declaration": unknown_declaration,
            }],
        }))
        .expect("unknown-stage edit should serialize");
        let error = apply_structural_edit(source, &unknown_stage)
            .expect_err("v6 edits must reject unknown binding stages");
        assert_eq!(error.diagnostic().code, "AE-EDIT-001");
    }

    #[test]
    fn structural_document_round_trips_every_shipped_example() {
        for (name, source) in shipped_examples() {
            let document = structural_document_json(source)
                .unwrap_or_else(|error| panic!("{name} should describe: {error}"));
            let parsed: Value = serde_json::from_str(&document)
                .unwrap_or_else(|error| panic!("{name} document should be JSON: {error}"));
            let canonical_source = parsed["canonicalSource"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} document must include canonical source"));
            compile_source(canonical_source).unwrap_or_else(|error| {
                panic!("{name} canonical document source should parse: {error}")
            });
            assert_eq!(
                document,
                structural_document_json(canonical_source).unwrap_or_else(|error| panic!(
                    "{name} document should be deterministic: {error}"
                ))
            );
        }
    }

    #[test]
    fn provenance_free_payloads_for_every_shipped_example_round_trip_through_edits() {
        for (name, source) in shipped_examples() {
            let document: Value = serde_json::from_str(
                &structural_document_json(source)
                    .unwrap_or_else(|error| panic!("{name} should describe: {error}")),
            )
            .unwrap_or_else(|error| panic!("{name} document should be JSON: {error}"));
            let canonical_source = document["canonicalSource"]
                .as_str()
                .unwrap_or_else(|| panic!("{name} document must include canonical source"));
            let edit = identity_edit_from_document(&document);

            let result = apply_structural_edit(canonical_source, &edit)
                .unwrap_or_else(|error| panic!("{name} payload should apply: {error}"));

            assert_eq!(
                result.source, canonical_source,
                "{name} must remain canonical"
            );
            compile_with_seed(&result.source)
                .unwrap_or_else(|error| panic!("{name} accepted edit must seed compile: {error}"));
        }
    }

    #[test]
    fn valid_replace_edit_returns_canonical_seed_compilable_source() {
        let result = apply_structural_edit(BASE_SOURCE, &replacement_edit(BASE_SOURCE, 7))
            .expect("valid structural replacement should apply");
        assert_eq!(result.operation_count, 1);
        assert_eq!(
            result.source,
            "world authoring\n\nweave main [] -> Whole:\n  yield 7\n"
        );
        compile_with_seed(&result.source).expect("accepted edit must remain seed compilable");
        let document: Value =
            serde_json::from_str(&result.document_json).expect("document must be JSON");
        assert_eq!(document["canonicalSource"], result.source);
        // ADR-065: product weave-replace path (no bootstrap aether.ast/v8).
        assert_eq!(document["schema"], PRODUCT_EDIT_SCHEMA_VERSION);
        assert_eq!(document["path"], "product-weave-replace");
        assert_eq!(document["productAccepted"], true);
    }

    #[test]
    fn product_weave_replace_does_not_require_bootstrap_ast_document() {
        assert!(crate::structural_edit_product_weave_replace());
        let result = apply_structural_edit(BASE_SOURCE, &replacement_edit(BASE_SOURCE, 42))
            .expect("product weave replace");
        assert_eq!(
            result.source,
            "world authoring\n\nweave main [] -> Whole:\n  yield 42\n"
        );
        let document: Value =
            serde_json::from_str(&result.document_json).expect("product-edit JSON");
        assert_eq!(document["schema"], PRODUCT_EDIT_SCHEMA_VERSION);
        assert!(document.get("program").is_none());
    }

    #[test]
    fn provenance_free_m2_resource_payload_round_trips_through_the_edit_protocol() {
        let source = include_str!("../../../examples/arena-buffer.ae");
        let document: Value = serde_json::from_str(
            &structural_document_json(source).expect("M2 source should describe"),
        )
        .expect("structural document should be JSON");
        let canonical_source = document["canonicalSource"]
            .as_str()
            .expect("structural document must contain canonical source");
        let mut declaration = document["program"]["weaves"][0].clone();
        remove_generated_provenance(&mut declaration);
        let edit = json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical_source,
            "operations": [{
                "op": "replace",
                "target": "weave:main",
                "declaration": declaration,
            }],
        });

        let result = apply_structural_edit(source, &edit.to_string())
            .expect("M2 resource payload should be accepted by the edit protocol");

        assert_eq!(result.source, canonical_source);
        compile_with_seed(&result.source).expect("accepted M2 edit must remain seed compilable");
    }

    fn remove_generated_provenance(value: &mut Value) {
        match value {
            Value::Array(values) => {
                for value in values {
                    remove_generated_provenance(value);
                }
            }
            Value::Object(object) => {
                object.remove("id");
                object.remove("span");
                for value in object.values_mut() {
                    remove_generated_provenance(value);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    fn identity_edit_from_document(document: &Value) -> String {
        let canonical_source = document["canonicalSource"]
            .as_str()
            .expect("structural document must include canonical source");
        let program = document["program"]
            .as_object()
            .expect("structural document must include a program object");
        let mut operations = Vec::new();

        for declaration in program["records"]
            .as_array()
            .expect("program records must be an array")
        {
            let name = declaration["name"]
                .as_str()
                .expect("record declaration must include a name");
            let mut declaration = declaration.clone();
            remove_generated_provenance(&mut declaration);
            operations.push(json!({
                "op": "replace",
                "target": format!("record:{name}"),
                "declaration": declaration,
            }));
        }

        for declaration in program["weaves"]
            .as_array()
            .expect("program weaves must be an array")
        {
            let name = declaration["name"]
                .as_str()
                .expect("weave declaration must include a name");
            let mut declaration = declaration.clone();
            remove_generated_provenance(&mut declaration);
            operations.push(json!({
                "op": "replace",
                "target": format!("weave:{name}"),
                "declaration": declaration,
            }));
        }

        serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical_source,
            "operations": operations,
        }))
        .expect("identity edit must serialize")
    }

    #[test]
    fn stale_edit_is_rejected_before_any_candidate_source_is_returned() {
        let error = apply_structural_edit(
            "world changed\n\nweave main [] -> Whole:\n  yield 0\n",
            &replacement_edit(BASE_SOURCE, 7),
        )
        .expect_err("stale base source must fail");
        assert_eq!(error.diagnostic().code, "AE-EDIT-003");
    }

    #[test]
    fn malformed_and_duplicate_edit_fields_are_rejected() {
        let malformed = r#"{"protocol":"aether.edit/v8","schema":"aether.ast/v8","baseSource":"x","operations":[],"extra":true}"#;
        let malformed_error =
            apply_structural_edit(BASE_SOURCE, malformed).expect_err("unknown field must fail");
        assert_eq!(malformed_error.diagnostic().code, "AE-EDIT-001");

        let duplicate = r#"{"protocol":"aether.edit/v8","protocol":"aether.edit/v8","schema":"aether.ast/v8","baseSource":"x","operations":[]}"#;
        let duplicate_error = apply_structural_edit(BASE_SOURCE, duplicate)
            .expect_err("duplicate JSON key must fail");
        assert_eq!(duplicate_error.diagnostic().code, "AE-EDIT-001");
        assert!(duplicate_error.diagnostic().message.contains("duplicate"));
    }

    #[test]
    fn insert_and_delete_are_structural_top_level_operations() {
        let insert = format!(
            r#"{{
  "protocol": "aether.edit/v8",
  "schema": "aether.ast/v8",
  "baseSource": {},
  "operations": [{{
    "op": "insertAfter",
    "target": "weave:main",
    "declaration": {{
      "kind": "Weave",
      "name": "helper",
      "parameters": [],
      "result": "Whole",
      "effect": "Total",
      "task": false,
      "body": [{{"kind":"Yield","value":{{"kind":"Atom","atom":{{"kind":"Whole","value":1}}}}}}]
    }}
  }}]
}}"#,
            serde_json::to_string(BASE_SOURCE).expect("source string must serialize")
        );
        let inserted = apply_structural_edit(BASE_SOURCE, &insert).expect("insert should apply");
        assert!(inserted.source.contains("weave helper [] -> Whole:"));

        let delete = format!(
            r#"{{"protocol":"aether.edit/v8","schema":"aether.ast/v8","baseSource":{},"operations":[{{"op":"delete","target":"weave:helper"}}]}}"#,
            serde_json::to_string(&inserted.source).expect("source string must serialize")
        );
        let deleted =
            apply_structural_edit(&inserted.source, &delete).expect("delete should apply");
        assert!(!deleted.source.contains("weave helper [] -> Whole:"));
    }

    #[test]
    fn inserting_a_record_reindexes_existing_record_type_references_by_name() {
        let source = include_str!("../../../examples/records.ae");
        let insert = format!(
            r#"{{"protocol":"aether.edit/v8","schema":"aether.ast/v8","baseSource":{},"operations":[{{"op":"insertAfter","target":"world","declaration":{{"kind":"Record","name":"badge","fields":[{{"kind":"RecordField","name":"rank","type":"Whole"}}]}}}}]}}"#,
            serde_json::to_string(&format_program(
                &compile_source(source).expect("record source should parse")
            ))
            .expect("source string must serialize")
        );
        let result = apply_structural_edit(source, &insert).expect("record insertion should apply");
        assert!(result.source.contains("record badge [rank: Whole]"));
        assert!(result
            .source
            .contains("weave duplicate [borrow value: card] -> card:"));
        compile_with_seed(&result.source).expect("reindexed record source should seed compile");
    }

    #[test]
    fn statement_level_replace_and_insert_edit_main_body() {
        let source = "world app\n\nweave main [] -> Whole:\n  bind n <- 20\n  yield n\n";
        let document: Value = serde_json::from_str(
            &structural_document_json(source).expect("source should describe"),
        )
        .expect("JSON");
        let canonical = document["canonicalSource"].as_str().expect("canonical");
        let yield_stmt = json!({
            "kind": "Yield",
            "value": {
                "kind": "Binary",
                "operation": "sum",
                "left": { "kind": "Name", "name": "n" },
                "right": { "kind": "Whole", "value": 1 }
            }
        });
        let edit = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical,
            "operations": [{
                "op": "replaceStatement",
                "path": "weave:main/body/1",
                "statement": yield_stmt,
            }],
        }))
        .expect("serialize");
        let result = apply_structural_edit(canonical, &edit).expect("replaceStatement");
        assert!(result.source.contains("yield sum n 1"));
        compile_with_seed(&result.source).expect("seed compile after statement edit");

        let bind = json!({
            "kind": "Bind",
            "name": "m",
            "mutable": false,
            "stage": "runtime",
            "value": { "kind": "Atom", "atom": { "kind": "Whole", "value": 3 } }
        });
        let insert = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical,
            "operations": [{
                "op": "insertStatementAt",
                "list": "weave:main/body",
                "index": 0,
                "statement": bind,
            }],
        }))
        .expect("serialize insert");
        let inserted = apply_structural_edit(canonical, &insert).expect("insertStatementAt");
        assert!(inserted.source.contains("bind m <- 3"));

        let oob = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical,
            "operations": [{
                "op": "deleteStatement",
                "path": "weave:main/body/99",
            }],
        }))
        .expect("serialize oob");
        let error = apply_structural_edit(canonical, &oob).expect_err("oob");
        assert_eq!(error.diagnostic().code, "AE-EDIT-011");

        let bad_path = serde_json::to_string(&json!({
            "protocol": STRUCTURAL_EDIT_PROTOCOL_VERSION,
            "schema": STRUCTURAL_AST_SCHEMA_VERSION,
            "baseSource": canonical,
            "operations": [{
                "op": "replaceStatement",
                "path": "weave:main/body/0/value",
                "statement": { "kind": "Yield", "value": { "kind": "Atom", "atom": { "kind": "Whole", "value": 1 } } },
            }],
        }))
        .expect("serialize bad path");
        let error = apply_structural_edit(canonical, &bad_path).expect_err("bad path");
        assert_eq!(error.diagnostic().code, "AE-EDIT-010");

        let v6 = r#"{"protocol":"aether.edit/v6","schema":"aether.ast/v6","baseSource":"x","operations":[{"op":"delete","target":"weave:main"}]}"#;
        let error = apply_structural_edit(canonical, v6).expect_err("v6 rejected");
        assert_eq!(error.diagnostic().code, "AE-EDIT-002");
    }

    #[test]
    fn compiler_diagnostics_expose_stable_type_code_and_span() {
        let error =
            compile_source("world diagnostic\n\nweave main [] -> Whole:\n  yield \"wrong type\"\n")
                .expect_err("invalid yield type must fail");
        let diagnostic = error.diagnostic();
        assert_eq!(diagnostic.code, "AE-TYPE-001");
        assert_eq!(diagnostic.span.line, 4);
        assert!(diagnostic.span.column >= 1);
        let envelope: Value = serde_json::from_str(&diagnostic_json(&diagnostic))
            .expect("diagnostic envelope must be JSON");
        let schema: Value = serde_json::from_str(include_str!(
            "../../../schemas/aether-diagnostic-v8.schema.json"
        ))
        .expect("diagnostic schema must be JSON");
        assert_eq!(envelope["schema"], DIAGNOSTIC_SCHEMA_VERSION);
        assert_eq!(envelope["code"], "AE-TYPE-001");
        assert_eq!(envelope["span"]["line"], 4);
        assert_eq!(
            schema["$id"],
            "https://aether.local/schemas/aether-diagnostic-v8.schema.json"
        );
        assert!(schema["required"]
            .as_array()
            .is_some_and(|fields| fields.iter().any(|field| field == "span")));
    }

    #[test]
    fn v8_authoring_schemas_describe_the_live_task_contract() {
        let ast_schema: Value =
            serde_json::from_str(include_str!("../../../schemas/aether-ast-v8.schema.json"))
                .expect("v8 AST schema must be JSON");
        let edit_schema: Value =
            serde_json::from_str(include_str!("../../../schemas/aether-edit-v8.schema.json"))
                .expect("v8 edit schema must be JSON");
        let diagnostic_schema: Value = serde_json::from_str(include_str!(
            "../../../schemas/aether-diagnostic-v8.schema.json"
        ))
        .expect("v8 diagnostic schema must be JSON");

        assert_eq!(
            ast_schema["properties"]["schema"]["const"],
            STRUCTURAL_AST_SCHEMA_VERSION
        );
        assert_eq!(
            ast_schema["$defs"]["weave"]["properties"]["task"]["type"],
            "boolean"
        );
        assert_eq!(
            ast_schema["$defs"]["checkpoint"]["properties"]["kind"]["const"],
            "Checkpoint"
        );
        assert_eq!(
            edit_schema["properties"]["protocol"]["const"],
            STRUCTURAL_EDIT_PROTOCOL_VERSION
        );
        assert_eq!(
            diagnostic_schema["properties"]["schema"]["const"],
            DIAGNOSTIC_SCHEMA_VERSION
        );
    }
}
