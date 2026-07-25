//! Aether Stage 0 compiler, bytecode verifier, and virtual machine.
//!
//! This crate is a host bootstrap only. It parses Aether source, emits the
//! Aether-owned AETH artifact format, verifies it, and executes it in the
//! Aether VM. It does not emit C, Rust, JavaScript, LLVM, or another language.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

pub const LANGUAGE_NAME: &str = "Aether";
pub const LANGUAGE_VERSION: &str = "0.1.0";

const ARTIFACT_MAGIC: &[u8; 4] = b"AETH";
const ARTIFACT_VERSION: u8 = 1;

const OP_PUSH_TEXT: u8 = 1;
const OP_PUSH_WHOLE: u8 = 2;
const OP_STORE: u8 = 3;
const OP_LOAD: u8 = 4;
const OP_SPEAK: u8 = 5;
const OP_YIELD: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    const fn synthetic() -> Self {
        Self::new(1, 1)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerError {
    pub span: Span,
    pub message: String,
}

impl CompilerError {
    fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} error at {}: {}",
            LANGUAGE_NAME, self.span, self.message
        )
    }
}

impl std::error::Error for CompilerError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub world: String,
    pub entry: Entry,
    pub statements: Vec<Statement>,
}

impl Program {
    #[must_use]
    pub fn significant_token_count(&self) -> usize {
        4 + self.statements.len() * 2
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub result: ValueType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Bind {
        name: String,
        value: Expression,
        span: Span,
    },
    Speak {
        value: Expression,
        span: Span,
    },
    Yield {
        value: Expression,
        span: Span,
    },
}

impl Statement {
    const fn span(&self) -> Span {
        match self {
            Self::Bind { span, .. } | Self::Speak { span, .. } | Self::Yield { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Text(String),
    Whole(i64),
    Name(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Text,
    Whole,
}

impl fmt::Display for ValueType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => formatter.write_str("Text"),
            Self::Whole => formatter.write_str("Whole"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileOutput {
    pub program: Program,
    pub bytecode: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutput {
    pub stdout: String,
    pub exit_code: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeError {
    pub offset: usize,
    pub message: String,
}

impl BytecodeError {
    fn new(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }
}

impl fmt::Display for BytecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Aether artifact error at byte {}: {}",
            self.offset, self.message
        )
    }
}

impl std::error::Error for BytecodeError {}

#[derive(Clone, Copy)]
struct SourceLine<'source> {
    number: usize,
    text: &'source str,
}

impl<'source> SourceLine<'source> {
    const fn span(self, column: usize) -> Span {
        Span::new(self.number, column)
    }
}

pub fn compile_source(source: &str) -> Result<Program, CompilerError> {
    if source.is_empty() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "source is empty; an Aether world is required",
        ));
    }

    if !source.is_ascii() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "Aether 0.1 source is ASCII-only so canonical source and bytecode agree",
        ));
    }

    let lines = source_lines(source)?;
    let mut meaningful = Vec::new();

    for line in lines {
        if line.text.is_empty() {
            continue;
        }
        if line.text.trim().is_empty() {
            return Err(CompilerError::new(
                line.span(1),
                "blank lines cannot contain whitespace",
            ));
        }
        if line.text.ends_with(' ') || line.text.ends_with('\t') {
            return Err(CompilerError::new(
                line.span(line.text.len()),
                "trailing whitespace is not part of canonical Aether source",
            ));
        }
        if line.text.contains('\t') {
            return Err(CompilerError::new(
                line.span(1),
                "tabs are not valid indentation; use two spaces per Aether block level",
            ));
        }
        meaningful.push(line);
    }

    if meaningful.len() < 3 {
        return Err(CompilerError::new(
            Span::synthetic(),
            "an Aether program needs a world, the main weave, and a yielding body",
        ));
    }

    let world = parse_world(meaningful[0])?;
    let entry = parse_entry(meaningful[1])?;
    let mut statements = Vec::new();
    for line in &meaningful[2..] {
        let Some(body) = line.text.strip_prefix("  ") else {
            return Err(CompilerError::new(
                line.span(1),
                "a weave body line must begin with exactly two spaces",
            ));
        };
        if body.starts_with(' ') {
            return Err(CompilerError::new(
                line.span(3),
                "Aether 0.1 has one block level; deeper indentation is not valid here",
            ));
        }
        statements.push(parse_statement(*line, body)?);
    }

    let program = Program {
        world,
        entry,
        statements,
    };
    validate_program(&program)?;
    Ok(program)
}

pub fn compile_to_bytecode(source: &str) -> Result<CompileOutput, CompilerError> {
    let program = compile_source(source)?;
    let bytecode = emit_bytecode(&program)?;
    verify_bytecode(&bytecode).map_err(|error| {
        CompilerError::new(
            Span::synthetic(),
            format!("compiler produced an invalid Aether artifact: {error}"),
        )
    })?;
    Ok(CompileOutput { program, bytecode })
}

#[must_use]
pub fn format_program(program: &Program) -> String {
    let mut formatted = format!(
        "world {}\n\nweave {} [] -> {}:\n",
        program.world, program.entry.name, program.entry.result
    );
    for statement in &program.statements {
        formatted.push_str("  ");
        match statement {
            Statement::Bind { name, value, .. } => {
                formatted.push_str("bind ");
                formatted.push_str(name);
                formatted.push_str(" <- ");
                write_expression(value, &mut formatted);
            }
            Statement::Speak { value, .. } => {
                formatted.push_str("speak ");
                write_expression(value, &mut formatted);
            }
            Statement::Yield { value, .. } => {
                formatted.push_str("yield ");
                write_expression(value, &mut formatted);
            }
        }
        formatted.push('\n');
    }
    formatted
}

#[must_use]
pub fn canonical_ast(program: &Program) -> String {
    let mut ast = format!(
        "World({});Entry({}->{})",
        program.world, program.entry.name, program.entry.result
    );
    for statement in &program.statements {
        ast.push(';');
        match statement {
            Statement::Bind { name, value, .. } => {
                ast.push_str("Bind(");
                ast.push_str(name);
                ast.push(',');
                write_ast_expression(value, &mut ast);
                ast.push(')');
            }
            Statement::Speak { value, .. } => {
                ast.push_str("Speak(");
                write_ast_expression(value, &mut ast);
                ast.push(')');
            }
            Statement::Yield { value, .. } => {
                ast.push_str("Yield(");
                write_ast_expression(value, &mut ast);
                ast.push(')');
            }
        }
    }
    ast
}

pub fn verify_bytecode(bytecode: &[u8]) -> Result<(), BytecodeError> {
    verify_header(bytecode)?;
    let mut position = ARTIFACT_MAGIC.len() + 1;
    let mut stack = Vec::new();
    let mut locals = Vec::new();
    let mut yielded = false;

    while position < bytecode.len() {
        if yielded {
            return Err(BytecodeError::new(
                position,
                "instructions appear after the terminating yield",
            ));
        }
        let offset = position;
        let opcode = read_byte(bytecode, &mut position)?;
        match opcode {
            OP_PUSH_TEXT => {
                let _ = read_text(bytecode, &mut position)?;
                stack.push(ValueType::Text);
            }
            OP_PUSH_WHOLE => {
                let _ = read_i64(bytecode, &mut position)?;
                stack.push(ValueType::Whole);
            }
            OP_STORE => {
                let index = usize::from(read_byte(bytecode, &mut position)?);
                if index != locals.len() {
                    return Err(BytecodeError::new(
                        offset,
                        "store slots must be introduced sequentially",
                    ));
                }
                locals.push(pop_value_type(&mut stack, offset, "store")?);
            }
            OP_LOAD => {
                let index = usize::from(read_byte(bytecode, &mut position)?);
                let Some(value_type) = locals.get(index) else {
                    return Err(BytecodeError::new(
                        offset,
                        "load references an unknown local slot",
                    ));
                };
                stack.push(*value_type);
            }
            OP_SPEAK => {
                require_value_type(
                    pop_value_type(&mut stack, offset, "speak")?,
                    ValueType::Text,
                    offset,
                    "speak",
                )?;
            }
            OP_YIELD => {
                require_value_type(
                    pop_value_type(&mut stack, offset, "yield")?,
                    ValueType::Whole,
                    offset,
                    "yield",
                )?;
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        offset,
                        "yield must leave an empty operand stack",
                    ));
                }
                yielded = true;
            }
            _ => return Err(BytecodeError::new(offset, "unknown Aether opcode")),
        }
    }

    if !yielded {
        return Err(BytecodeError::new(
            bytecode.len(),
            "artifact has no terminating yield instruction",
        ));
    }
    Ok(())
}

pub fn run_bytecode(bytecode: &[u8]) -> Result<RunOutput, BytecodeError> {
    verify_bytecode(bytecode)?;
    let mut position = ARTIFACT_MAGIC.len() + 1;
    let mut stack = Vec::new();
    let mut locals = Vec::new();
    let mut stdout = String::new();

    while position < bytecode.len() {
        let offset = position;
        let opcode = read_byte(bytecode, &mut position)?;
        match opcode {
            OP_PUSH_TEXT => stack.push(RuntimeValue::Text(read_text(bytecode, &mut position)?)),
            OP_PUSH_WHOLE => stack.push(RuntimeValue::Whole(read_i64(bytecode, &mut position)?)),
            OP_STORE => {
                let index = usize::from(read_byte(bytecode, &mut position)?);
                if index != locals.len() {
                    return Err(BytecodeError::new(offset, "invalid store slot"));
                }
                locals.push(pop_runtime_value(&mut stack, offset, "store")?);
            }
            OP_LOAD => {
                let index = usize::from(read_byte(bytecode, &mut position)?);
                let Some(value) = locals.get(index) else {
                    return Err(BytecodeError::new(offset, "invalid load slot"));
                };
                stack.push(value.clone());
            }
            OP_SPEAK => match pop_runtime_value(&mut stack, offset, "speak")? {
                RuntimeValue::Text(value) => stdout.push_str(&value),
                RuntimeValue::Whole(_) => {
                    return Err(BytecodeError::new(offset, "speak received a Whole value"));
                }
            },
            OP_YIELD => match pop_runtime_value(&mut stack, offset, "yield")? {
                RuntimeValue::Whole(exit_code) => return Ok(RunOutput { stdout, exit_code }),
                RuntimeValue::Text(_) => {
                    return Err(BytecodeError::new(offset, "yield received a Text value"));
                }
            },
            _ => return Err(BytecodeError::new(offset, "unknown Aether opcode")),
        }
    }

    Err(BytecodeError::new(
        bytecode.len(),
        "artifact ended without yield",
    ))
}

fn source_lines(source: &str) -> Result<Vec<SourceLine<'_>>, CompilerError> {
    let mut lines = Vec::new();
    for (index, raw_line) in source.split('\n').enumerate() {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.contains('\r') {
            return Err(CompilerError::new(
                Span::new(index + 1, 1),
                "carriage returns are only valid as Windows line endings",
            ));
        }
        lines.push(SourceLine {
            number: index + 1,
            text: line,
        });
    }
    Ok(lines)
}

fn parse_world(line: SourceLine<'_>) -> Result<String, CompilerError> {
    if line.text.starts_with(' ') {
        return Err(CompilerError::new(
            line.span(1),
            "world declarations cannot be indented",
        ));
    }
    let Some(name) = line.text.strip_prefix("world ") else {
        return Err(CompilerError::new(
            line.span(1),
            "the first declaration must be world followed by a lowercase name",
        ));
    };
    validate_name(name, line.span(7), "world name")?;
    Ok(name.to_owned())
}

fn parse_entry(line: SourceLine<'_>) -> Result<Entry, CompilerError> {
    if line.text.starts_with(' ') {
        return Err(CompilerError::new(
            line.span(1),
            "weave declarations cannot be indented",
        ));
    }
    if line.text != "weave main [] -> Whole:" {
        return Err(CompilerError::new(
            line.span(1),
            "Aether 0.1 requires the entry declaration weave main [] -> Whole:",
        ));
    }
    Ok(Entry {
        name: "main".to_owned(),
        result: ValueType::Whole,
    })
}

fn parse_statement(line: SourceLine<'_>, body: &str) -> Result<Statement, CompilerError> {
    let span = line.span(3);
    if let Some(rest) = body.strip_prefix("bind ") {
        let Some((name, expression)) = rest.split_once(" <- ") else {
            return Err(CompilerError::new(
                span,
                "bind requires a name, the <- binder, and one value",
            ));
        };
        validate_name(name, line.span(8), "binding name")?;
        let expression_column = 3 + "bind ".len() + name.len() + " <- ".len();
        return Ok(Statement::Bind {
            name: name.to_owned(),
            value: parse_expression(expression, line.span(expression_column))?,
            span,
        });
    }
    if let Some(expression) = body.strip_prefix("speak ") {
        return Ok(Statement::Speak {
            value: parse_expression(expression, line.span(3 + "speak ".len()))?,
            span,
        });
    }
    if let Some(expression) = body.strip_prefix("yield ") {
        return Ok(Statement::Yield {
            value: parse_expression(expression, line.span(3 + "yield ".len()))?,
            span,
        });
    }
    Err(CompilerError::new(
        span,
        "unknown Aether statement; use bind, speak, or yield",
    ))
}

fn parse_expression(source: &str, span: Span) -> Result<Expression, CompilerError> {
    if source.is_empty() {
        return Err(CompilerError::new(span, "an expression is required"));
    }
    if source.starts_with('"') {
        return Ok(Expression {
            kind: ExpressionKind::Text(parse_text_literal(source, span)?),
            span,
        });
    }
    if is_whole_literal(source) {
        let value = source.parse::<i64>().map_err(|_| {
            CompilerError::new(
                span,
                "whole literal is outside the supported signed 64-bit range",
            )
        })?;
        return Ok(Expression {
            kind: ExpressionKind::Whole(value),
            span,
        });
    }
    validate_name(source, span, "value name")?;
    Ok(Expression {
        kind: ExpressionKind::Name(source.to_owned()),
        span,
    })
}

fn parse_text_literal(source: &str, span: Span) -> Result<String, CompilerError> {
    if source.len() < 2 || !source.ends_with('"') {
        return Err(CompilerError::new(
            span,
            "text values must use one closed double-quoted literal",
        ));
    }
    let mut value = String::new();
    let mut characters = source[1..source.len() - 1].chars();
    while let Some(character) = characters.next() {
        if character == '"' {
            return Err(CompilerError::new(
                span,
                "double quotes inside text must use the quote escape",
            ));
        }
        if character != '\\' {
            if character.is_control() {
                return Err(CompilerError::new(
                    span,
                    "control characters inside text must use an escape",
                ));
            }
            value.push(character);
            continue;
        }
        let Some(escaped) = characters.next() else {
            return Err(CompilerError::new(
                span,
                "text ends with an incomplete escape",
            ));
        };
        match escaped {
            '\\' => value.push('\\'),
            '"' => value.push('"'),
            'n' => value.push('\n'),
            'r' => value.push('\r'),
            't' => value.push('\t'),
            _ => {
                return Err(CompilerError::new(
                    span,
                    "supported text escapes are \\\\, \\\", \\n, \\r, and \\t",
                ));
            }
        }
    }
    Ok(value)
}

fn validate_program(program: &Program) -> Result<(), CompilerError> {
    let mut bindings = BTreeMap::new();
    let mut has_yielded = false;
    for statement in &program.statements {
        if has_yielded {
            return Err(CompilerError::new(
                statement.span(),
                "yield terminates a weave and must be its final statement",
            ));
        }
        match statement {
            Statement::Bind { name, value, span } => {
                if bindings.contains_key(name) {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} already exists in this weave"),
                    ));
                }
                let value_type = expression_type(value, &bindings)?;
                bindings.insert(name.clone(), value_type);
            }
            Statement::Speak { value, .. } => {
                let value_type = expression_type(value, &bindings)?;
                require_source_type(value_type, ValueType::Text, value.span, "speak")?;
            }
            Statement::Yield { value, .. } => {
                let value_type = expression_type(value, &bindings)?;
                require_source_type(value_type, ValueType::Whole, value.span, "yield")?;
                has_yielded = true;
            }
        }
    }
    if !has_yielded {
        return Err(CompilerError::new(
            Span::synthetic(),
            "every Aether weave must end with yield",
        ));
    }
    Ok(())
}

fn expression_type(
    expression: &Expression,
    bindings: &BTreeMap<String, ValueType>,
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Text(_) => Ok(ValueType::Text),
        ExpressionKind::Whole(_) => Ok(ValueType::Whole),
        ExpressionKind::Name(name) => bindings.get(name).copied().ok_or_else(|| {
            CompilerError::new(
                expression.span,
                format!("value {name} has not been bound in this weave"),
            )
        }),
    }
}

fn require_source_type(
    actual: ValueType,
    expected: ValueType,
    span: Span,
    operation: &str,
) -> Result<(), CompilerError> {
    if actual == expected {
        Ok(())
    } else {
        Err(CompilerError::new(
            span,
            format!("{operation} requires {expected}, but this expression is {actual}"),
        ))
    }
}

fn emit_bytecode(program: &Program) -> Result<Vec<u8>, CompilerError> {
    let mut bytecode = Vec::from(&ARTIFACT_MAGIC[..]);
    bytecode.push(ARTIFACT_VERSION);
    let mut local_types = BTreeMap::new();
    let mut local_slots = BTreeMap::new();

    for statement in &program.statements {
        match statement {
            Statement::Bind { name, value, span } => {
                emit_expression(value, &local_slots, &mut bytecode)?;
                let index = u8::try_from(local_slots.len()).map_err(|_| {
                    CompilerError::new(*span, "Aether 0.1 supports at most 256 local bindings")
                })?;
                bytecode.push(OP_STORE);
                bytecode.push(index);
                let value_type = expression_type(value, &local_types)?;
                local_types.insert(name.clone(), value_type);
                local_slots.insert(name.clone(), index);
            }
            Statement::Speak { value, .. } => {
                emit_expression(value, &local_slots, &mut bytecode)?;
                bytecode.push(OP_SPEAK);
            }
            Statement::Yield { value, .. } => {
                emit_expression(value, &local_slots, &mut bytecode)?;
                bytecode.push(OP_YIELD);
            }
        }
    }
    Ok(bytecode)
}

fn emit_expression(
    expression: &Expression,
    local_slots: &BTreeMap<String, u8>,
    bytecode: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    match &expression.kind {
        ExpressionKind::Text(value) => {
            let length = u16::try_from(value.len()).map_err(|_| {
                CompilerError::new(
                    expression.span,
                    "Aether 0.1 text literals cannot exceed 65,535 bytes",
                )
            })?;
            bytecode.push(OP_PUSH_TEXT);
            bytecode.extend_from_slice(&length.to_le_bytes());
            bytecode.extend_from_slice(value.as_bytes());
        }
        ExpressionKind::Whole(value) => {
            bytecode.push(OP_PUSH_WHOLE);
            bytecode.extend_from_slice(&value.to_le_bytes());
        }
        ExpressionKind::Name(name) => {
            let Some(slot) = local_slots.get(name) else {
                return Err(CompilerError::new(
                    expression.span,
                    format!("value {name} has not been bound in this weave"),
                ));
            };
            bytecode.push(OP_LOAD);
            bytecode.push(*slot);
        }
    }
    Ok(())
}

fn verify_header(bytecode: &[u8]) -> Result<(), BytecodeError> {
    if bytecode.len() < ARTIFACT_MAGIC.len() + 1 {
        return Err(BytecodeError::new(0, "artifact is shorter than its header"));
    }
    if bytecode[..ARTIFACT_MAGIC.len()] != ARTIFACT_MAGIC[..] {
        return Err(BytecodeError::new(0, "artifact magic is not AETH"));
    }
    if bytecode[ARTIFACT_MAGIC.len()] != ARTIFACT_VERSION {
        return Err(BytecodeError::new(
            ARTIFACT_MAGIC.len(),
            "artifact version is not supported by this Aether VM",
        ));
    }
    Ok(())
}

fn read_byte(bytecode: &[u8], position: &mut usize) -> Result<u8, BytecodeError> {
    let offset = *position;
    let Some(value) = bytecode.get(offset) else {
        return Err(BytecodeError::new(offset, "artifact ended unexpectedly"));
    };
    *position += 1;
    Ok(*value)
}

fn read_i64(bytecode: &[u8], position: &mut usize) -> Result<i64, BytecodeError> {
    let offset = *position;
    let end = offset
        .checked_add(8)
        .ok_or_else(|| BytecodeError::new(offset, "whole literal length overflowed"))?;
    let Some(bytes) = bytecode.get(offset..end) else {
        return Err(BytecodeError::new(offset, "whole literal is truncated"));
    };
    *position = end;
    let mut buffer = [0_u8; 8];
    buffer.copy_from_slice(bytes);
    Ok(i64::from_le_bytes(buffer))
}

fn read_text(bytecode: &[u8], position: &mut usize) -> Result<String, BytecodeError> {
    let offset = *position;
    let low = read_byte(bytecode, position)?;
    let high = read_byte(bytecode, position)?;
    let length = usize::from(u16::from_le_bytes([low, high]));
    let end = (*position)
        .checked_add(length)
        .ok_or_else(|| BytecodeError::new(offset, "text literal length overflowed"))?;
    let Some(bytes) = bytecode.get(*position..end) else {
        return Err(BytecodeError::new(offset, "text literal is truncated"));
    };
    let value = std::str::from_utf8(bytes)
        .map_err(|_| BytecodeError::new(offset, "text literal is not UTF-8"))?;
    if !value.is_ascii() {
        return Err(BytecodeError::new(
            offset,
            "text literal is not valid Aether 0.1 ASCII text",
        ));
    }
    *position = end;
    Ok(value.to_owned())
}

fn pop_value_type(
    stack: &mut Vec<ValueType>,
    offset: usize,
    operation: &str,
) -> Result<ValueType, BytecodeError> {
    stack.pop().ok_or_else(|| {
        BytecodeError::new(
            offset,
            format!("{operation} would underflow the operand stack"),
        )
    })
}

fn require_value_type(
    actual: ValueType,
    expected: ValueType,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if actual == expected {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires {expected}, but artifact stack has {actual}"),
        ))
    }
}

#[derive(Debug, Clone)]
enum RuntimeValue {
    Text(String),
    Whole(i64),
}

fn pop_runtime_value(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<RuntimeValue, BytecodeError> {
    stack.pop().ok_or_else(|| {
        BytecodeError::new(
            offset,
            format!("{operation} would underflow the operand stack"),
        )
    })
}

fn write_expression(expression: &Expression, output: &mut String) {
    match &expression.kind {
        ExpressionKind::Text(value) => {
            output.push('"');
            for character in value.chars() {
                match character {
                    '\\' => output.push_str("\\\\"),
                    '"' => output.push_str("\\\""),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    _ => output.push(character),
                }
            }
            output.push('"');
        }
        ExpressionKind::Whole(value) => output.push_str(&value.to_string()),
        ExpressionKind::Name(name) => output.push_str(name),
    }
}

fn write_ast_expression(expression: &Expression, output: &mut String) {
    match &expression.kind {
        ExpressionKind::Text(value) => {
            output.push_str("Text(");
            output.push_str(&value.escape_default().to_string());
            output.push(')');
        }
        ExpressionKind::Whole(value) => {
            output.push_str("Whole(");
            output.push_str(&value.to_string());
            output.push(')');
        }
        ExpressionKind::Name(name) => {
            output.push_str("Name(");
            output.push_str(name);
            output.push(')');
        }
    }
}

fn validate_name(name: &str, span: Span, subject: &str) -> Result<(), CompilerError> {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return Err(CompilerError::new(span, format!("{subject} is required")));
    };
    if !first.is_ascii_lowercase() {
        return Err(CompilerError::new(
            span,
            format!("{subject} must begin with a lowercase ASCII letter"),
        ));
    }
    if !characters.all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
    }) {
        return Err(CompilerError::new(
            span,
            format!("{subject} may only use lowercase ASCII letters, digits, and underscores"),
        ));
    }
    if matches!(
        name,
        "world" | "weave" | "bind" | "speak" | "yield" | "main" | "whole" | "text"
    ) {
        return Err(CompilerError::new(
            span,
            format!("{subject} uses the reserved Aether word {name}"),
        ));
    }
    Ok(())
}

fn is_whole_literal(source: &str) -> bool {
    let digits = source.strip_prefix('-').unwrap_or(source);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    if digits.len() > 1 && digits.starts_with('0') {
        return false;
    }
    source != "-0"
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO: &str = "world genesis\n\nweave main [] -> Whole:\n  bind greeting <- \"Hello from Aether\\n\"\n  speak greeting\n  yield 0\n";

    #[test]
    fn compiles_runs_and_formats_aether_source() {
        let output = compile_to_bytecode(HELLO).expect("Aether source should compile");
        let run = run_bytecode(&output.bytecode).expect("Aether artifact should run");
        assert_eq!(run.stdout, "Hello from Aether\n");
        assert_eq!(run.exit_code, 0);
        assert_eq!(format_program(&output.program), HELLO);
        assert_eq!(
            canonical_ast(&output.program),
            "World(genesis);Entry(main->Whole);Bind(greeting,Text(Hello from Aether\\n));Speak(Name(greeting));Yield(Whole(0))"
        );
    }

    #[test]
    fn supports_whole_bindings_for_exit_status() {
        let source = "world arithmetic\n\nweave main [] -> Whole:\n  bind status <- 42\n  speak \"ready\\n\"\n  yield status\n";
        let output = compile_to_bytecode(source).expect("source should compile");
        let run = run_bytecode(&output.bytecode).expect("artifact should run");
        assert_eq!(run.stdout, "ready\n");
        assert_eq!(run.exit_code, 42);
    }

    #[test]
    fn preserves_source_order_for_local_slots() {
        let source = "world slots\n\nweave main [] -> Whole:\n  bind zeta <- \"z\"\n  bind alpha <- \"a\"\n  speak zeta\n  speak alpha\n  yield 0\n";
        let output = compile_to_bytecode(source).expect("source should compile");
        let run = run_bytecode(&output.bytecode).expect("artifact should run");

        assert_eq!(run.stdout, "za");
        assert_eq!(run.exit_code, 0);
    }

    #[test]
    fn rejects_legacy_syntax() {
        let error =
            compile_source("fn main() -> Int { return 0; }").expect_err("legacy source must fail");
        assert!(error.message.contains("world"));
    }

    #[test]
    fn rejects_noncanonical_indentation() {
        let source = "world broken\nweave main [] -> Whole:\n    yield 0\n";
        let error = compile_source(source).expect_err("four spaces must fail");
        assert!(error.message.contains("deeper indentation"));
    }

    #[test]
    fn rejects_unbound_values() {
        let source = "world broken\nweave main [] -> Whole:\n  speak greeting\n  yield 0\n";
        let error = compile_source(source).expect_err("unbound name must fail");
        assert!(error.message.contains("has not been bound"));
    }

    #[test]
    fn rejects_wrong_effect_types() {
        let source = "world broken\nweave main [] -> Whole:\n  speak 7\n  yield 0\n";
        let error = compile_source(source).expect_err("speak must require Text");
        assert!(error.message.contains("requires Text"));
    }

    #[test]
    fn rejects_statements_after_yield() {
        let source = "world broken\nweave main [] -> Whole:\n  yield 0\n  speak \"late\"\n";
        let error = compile_source(source).expect_err("yield must be terminal");
        assert!(error.message.contains("final statement"));
    }

    #[test]
    fn bytecode_is_deterministic_and_verifiable() {
        let first = compile_to_bytecode(HELLO).expect("first compilation should work");
        let second = compile_to_bytecode(HELLO).expect("second compilation should work");
        assert_eq!(first.bytecode, second.bytecode);
        verify_bytecode(&first.bytecode).expect("compiler artifact must verify");
    }

    #[test]
    fn verifier_rejects_bad_magic_and_stack_underflow() {
        let mut artifact = compile_to_bytecode(HELLO)
            .expect("source should compile")
            .bytecode;
        artifact[0] = b'X';
        assert!(verify_bytecode(&artifact).is_err());

        let underflow = [b'A', b'E', b'T', b'H', ARTIFACT_VERSION, OP_SPEAK, OP_YIELD];
        assert!(verify_bytecode(&underflow).is_err());
    }

    #[test]
    fn accepts_windows_line_endings_but_formats_canonically() {
        let windows_source = HELLO.replace('\n', "\r\n");
        let program = compile_source(&windows_source).expect("CRLF source should compile");
        assert_eq!(format_program(&program), HELLO);
    }
}
