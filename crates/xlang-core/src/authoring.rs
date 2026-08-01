//! Versioned structural-authoring contracts for local Aether tooling.
//!
//! This module deliberately sits above parsing and below no host capability.
//! It serializes only validated Aether source, accepts a small allow-listed
//! edit protocol, renders canonical source, and validates that source again.
//! Callers that persist an edit must still use the ordinary seed-hosted product
//! compiler path before writing it.

use std::collections::BTreeSet;
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{json, Map, Number, Value};

use crate::{
    compile_source, format_program, Atom, AtomKind, BinaryOperation, BufferElement, CompilerError,
    Diagnostic, Effect, Expression, ExpressionKind, Parameter, ParameterMode, Program,
    RecordDeclaration, RecordField, ResourceOperation, Span, Statement, TernaryOperation,
    UnaryOperation, ValueType, Weave, LANGUAGE_NAME, LANGUAGE_VERSION,
};

/// The JSON schema identifier emitted for a validated semantic document.
pub const STRUCTURAL_AST_SCHEMA_VERSION: &str = "aether.ast/v2";
/// The JSON protocol identifier accepted for a structural edit request.
pub const STRUCTURAL_EDIT_PROTOCOL_VERSION: &str = "aether.edit/v2";
/// The JSON schema identifier used for machine-readable diagnostic envelopes.
pub const DIAGNOSTIC_SCHEMA_VERSION: &str = "aether.diagnostic/v2";

const MAX_STRUCTURAL_EDIT_BYTES: usize = 4_000_000;
const MAX_STRUCTURAL_EDIT_OPERATIONS: usize = 32;
const MAX_STRUCTURAL_EDIT_NODES: usize = 100_000;
const MAX_STRUCTURAL_EDIT_DEPTH: usize = 256;
const MAX_SOURCE_BYTES: usize = 1_000_000;

/// A successful pure structural edit. The returned source is canonical and has
/// passed the normal bootstrap parser and semantic validator.
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

/// Return deterministic, pretty-printed `aether.ast/v2` JSON for valid source.
/// Source spans always refer to the returned document's canonical LF source.
pub fn structural_document_json(source: &str) -> Result<String, CompilerError> {
    let (program, canonical_source) = canonicalize_source(source)?;
    serialize_document(&canonical_source, &program).map_err(serialization_error)
}

/// Serialize a stable `aether.diagnostic/v2` envelope for any compiler or
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

/// Apply one bounded `aether.edit/v2` document to matching source.
///
/// The function does not write files, invoke a model, execute code, or compile
/// an artifact. It is intentionally pure apart from memory allocation. The CLI
/// seed-compiles returned source before writing it to its requested output path.
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
    let validated_program = compile_source(&candidate_source).map_err(StructuralEditError::from)?;
    let canonical_candidate = format_program(&validated_program);
    let document_json = serialize_document(&canonical_candidate, &validated_program)
        .map_err(|error| protocol_error("AE-EDIT-001", error.to_string()))?;

    Ok(StructuralEditResult {
        source: canonical_candidate,
        document_json,
        operation_count: request.operations.len(),
    })
}

fn canonicalize_source(source: &str) -> Result<(Program, String), CompilerError> {
    let parsed = compile_source(source)?;
    let canonical_source = format_program(&parsed);
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
    let weaves = program
        .weaves
        .iter()
        .map(|weave| weave_value(weave, &program.records))
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
        "weaves": weaves,
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
                "type": value_type_name(field.value_type, records),
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

fn weave_value(weave: &Weave, records: &[RecordDeclaration]) -> Value {
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
                "type": value_type_name(parameter.value_type, records),
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
        "result": value_type_name(weave.result, records),
        "effect": effect_name(weave.effect),
        "body": body,
    })
}

fn statement_value(statement: &Statement, id: &str) -> Value {
    match statement {
        Statement::Bind {
            name,
            mutable,
            value,
            span,
        } => json!({
            "id": id,
            "kind": "Bind",
            "span": span_value(*span),
            "name": name,
            "mutable": mutable,
            "value": expression_value(value, &format!("{id}/value")),
        }),
        Statement::Revise { name, value, span } => json!({
            "id": id,
            "kind": "Revise",
            "span": span_value(*span),
            "name": name,
            "value": expression_value(value, &format!("{id}/value")),
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

fn value_type_name(value_type: ValueType, records: &[RecordDeclaration]) -> String {
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
    target: EditTarget,
    declaration: Option<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditOperationKind {
    Replace,
    InsertAfter,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EditTarget {
    World,
    Record(String),
    Weave(String),
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
    let (kind, requires_declaration) = match operation.as_str() {
        "replace" => (EditOperationKind::Replace, true),
        "insertAfter" => (EditOperationKind::InsertAfter, true),
        "delete" => (EditOperationKind::Delete, false),
        _ => {
            return Err(protocol_error(
                "AE-EDIT-005",
                format!("{context} has unsupported operation {operation:?}"),
            ));
        }
    };
    let allowed = if requires_declaration {
        &["op", "target", "declaration"][..]
    } else {
        &["op", "target"][..]
    };
    ensure_only_fields(object, allowed, &context)?;
    let target = parse_target(&required_string(object, "target", &context)?)?;
    let declaration = if requires_declaration {
        Some(required_value(object, "declaration", &context)?.clone())
    } else {
        None
    };
    Ok(EditOperation {
        kind,
        target,
        declaration,
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
        EditOperationKind::Delete => apply_delete(program, &operation.target),
    }
}

fn apply_replace(
    program: &mut Program,
    operation: &EditOperation,
    node_budget: &mut NodeBudget,
) -> Result<(), StructuralEditError> {
    let declaration = parse_operation_declaration(program, operation, node_budget)?;
    match (&operation.target, declaration) {
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
            "the world node cannot be replaced by structural edit v2",
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
    let declaration = parse_operation_declaration(program, operation, node_budget)?;
    match (&operation.target, declaration) {
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
            "the world node cannot be deleted by structural edit v2",
        )),
    }
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
        &["kind", "name", "parameters", "result", "effect", "body"],
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
                &["kind", "name", "mutable", "value"],
                "Bind statement",
            )?;
            Ok(Statement::Bind {
                name: required_string(object, "name", "Bind statement")?,
                mutable: required_bool(object, "mutable", "Bind statement")?,
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
        _ => Err(protocol_error(
            "AE-EDIT-001",
            format!("statement has unsupported kind {kind:?}"),
        )),
    }
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
  "protocol": "aether.edit/v2",
  "schema": "aether.ast/v2",
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

    fn shipped_examples() -> [(&'static str, &'static str); 14] {
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
                "error-effect",
                include_str!("../../../examples/error-effect.ae"),
            ),
        ]
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
        let malformed = r#"{"protocol":"aether.edit/v2","schema":"aether.ast/v2","baseSource":"x","operations":[],"extra":true}"#;
        let malformed_error =
            apply_structural_edit(BASE_SOURCE, malformed).expect_err("unknown field must fail");
        assert_eq!(malformed_error.diagnostic().code, "AE-EDIT-001");

        let duplicate = r#"{"protocol":"aether.edit/v2","protocol":"aether.edit/v2","schema":"aether.ast/v2","baseSource":"x","operations":[]}"#;
        let duplicate_error = apply_structural_edit(BASE_SOURCE, duplicate)
            .expect_err("duplicate JSON key must fail");
        assert_eq!(duplicate_error.diagnostic().code, "AE-EDIT-001");
        assert!(duplicate_error.diagnostic().message.contains("duplicate"));
    }

    #[test]
    fn insert_and_delete_are_structural_top_level_operations() {
        let insert = format!(
            r#"{{
  "protocol": "aether.edit/v2",
  "schema": "aether.ast/v2",
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
      "body": [{{"kind":"Yield","value":{{"kind":"Atom","atom":{{"kind":"Whole","value":1}}}}}}]
    }}
  }}]
}}"#,
            serde_json::to_string(BASE_SOURCE).expect("source string must serialize")
        );
        let inserted = apply_structural_edit(BASE_SOURCE, &insert).expect("insert should apply");
        assert!(inserted.source.contains("weave helper [] -> Whole:"));

        let delete = format!(
            r#"{{"protocol":"aether.edit/v2","schema":"aether.ast/v2","baseSource":{},"operations":[{{"op":"delete","target":"weave:helper"}}]}}"#,
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
            r#"{{"protocol":"aether.edit/v2","schema":"aether.ast/v2","baseSource":{},"operations":[{{"op":"insertAfter","target":"world","declaration":{{"kind":"Record","name":"badge","fields":[{{"kind":"RecordField","name":"rank","type":"Whole"}}]}}}}]}}"#,
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
            "../../../schemas/aether-diagnostic-v2.schema.json"
        ))
        .expect("diagnostic schema must be JSON");
        assert_eq!(envelope["schema"], DIAGNOSTIC_SCHEMA_VERSION);
        assert_eq!(envelope["code"], "AE-TYPE-001");
        assert_eq!(envelope["span"]["line"], 4);
        assert_eq!(
            schema["$id"],
            "https://aether.local/schemas/aether-diagnostic-v2.schema.json"
        );
        assert!(schema["required"]
            .as_array()
            .is_some_and(|fields| fields.iter().any(|field| field == "span")));
    }
}
