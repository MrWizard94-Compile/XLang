//! Aether Stage 2 compiler, AETH verifier, and virtual machine.
//!
//! Stage 2 adds bounded binary values and typed weave invocation so an Aether
//! compiler can accept source as Text and return verified AETH bytes without a
//! host compiler participating in its compilation logic.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub const LANGUAGE_NAME: &str = "Aether";
pub const LANGUAGE_VERSION: &str = "0.3.0";

const ARTIFACT_MAGIC: &[u8; 4] = b"AETH";
const ARTIFACT_VERSION: u8 = 3;
const MAX_SOURCE_BYTES: usize = 1_000_000;
const MAX_FUNCTIONS: usize = 256;
const MAX_LOCALS: usize = u16::MAX as usize;
const MAX_TEXT_BYTES: usize = 1_000_000;
const MAX_BYTES: usize = 1_000_000;
const MAX_CALL_DEPTH: usize = 1_024;

const OP_PUSH_TEXT: u8 = 1;
const OP_PUSH_WHOLE: u8 = 2;
const OP_PUSH_TRUTH: u8 = 3;
const OP_STORE: u8 = 4;
const OP_LOAD: u8 = 5;
const OP_MOVE: u8 = 6;
const OP_REVISE: u8 = 7;
const OP_SPEAK: u8 = 8;
const OP_YIELD: u8 = 9;
const OP_SUM: u8 = 10;
const OP_DIFFERENCE: u8 = 11;
const OP_PRODUCT: u8 = 12;
const OP_LESS: u8 = 13;
const OP_SAME: u8 = 14;
const OP_NOT: u8 = 15;
const OP_JOIN: u8 = 16;
const OP_MEASURE: u8 = 17;
const OP_GLYPH: u8 = 18;
const OP_CUT: u8 = 19;
const OP_RENDER: u8 = 20;
const OP_CALL: u8 = 21;
const OP_JUMP_IF_DIM: u8 = 22;
const OP_JUMP: u8 = 23;
const OP_PUSH_BYTES: u8 = 24;
const OP_QUOTIENT: u8 = 25;
const OP_REMAINDER: u8 = 26;
const OP_FUSE: u8 = 27;
const OP_APPEND: u8 = 28;
const OP_EXTENT: u8 = 29;
const OP_OCTET: u8 = 30;
const OP_SLICE: u8 = 31;
const OP_ENCODE: u8 = 32;
const OP_DECODE: u8 = 33;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Text,
    Whole,
    Truth,
    Bytes,
}

impl ValueType {
    fn to_byte(self) -> u8 {
        match self {
            Self::Text => 1,
            Self::Whole => 2,
            Self::Truth => 3,
            Self::Bytes => 4,
        }
    }

    fn from_byte(value: u8, offset: usize) -> Result<Self, BytecodeError> {
        match value {
            1 => Ok(Self::Text),
            2 => Ok(Self::Whole),
            3 => Ok(Self::Truth),
            4 => Ok(Self::Bytes),
            _ => Err(BytecodeError::new(offset, "unknown Aether value type")),
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => formatter.write_str("Text"),
            Self::Whole => formatter.write_str("Whole"),
            Self::Truth => formatter.write_str("Truth"),
            Self::Bytes => formatter.write_str("Bytes"),
        }
    }
}

const fn is_unique_value(value_type: ValueType) -> bool {
    matches!(value_type, ValueType::Text | ValueType::Bytes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterMode {
    Own,
    Borrow,
}

impl ParameterMode {
    fn to_byte(self) -> u8 {
        match self {
            Self::Own => 1,
            Self::Borrow => 2,
        }
    }

    fn from_byte(value: u8, offset: usize) -> Result<Self, BytecodeError> {
        match value {
            1 => Ok(Self::Own),
            2 => Ok(Self::Borrow),
            _ => Err(BytecodeError::new(offset, "unknown Aether parameter mode")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub world: String,
    pub weaves: Vec<Weave>,
}

impl Program {
    #[must_use]
    pub fn significant_token_count(&self) -> usize {
        2 + self
            .weaves
            .iter()
            .map(|weave| 4 + weave.parameters.len() * 2 + statement_token_count(&weave.body))
            .sum::<usize>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weave {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub result: ValueType,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub value_type: ValueType,
    pub mode: ParameterMode,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Bind {
        name: String,
        mutable: bool,
        value: Expression,
        span: Span,
    },
    Revise {
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
    Choose {
        condition: Expression,
        when_bright: Vec<Statement>,
        when_dim: Vec<Statement>,
        span: Span,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },
}

impl Statement {
    const fn span(&self) -> Span {
        match self {
            Self::Bind { span, .. }
            | Self::Revise { span, .. }
            | Self::Speak { span, .. }
            | Self::Yield { span, .. }
            | Self::Choose { span, .. }
            | Self::While { span, .. } => *span,
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
    Atom(Atom),
    Unary {
        operation: UnaryOperation,
        argument: Atom,
    },
    Binary {
        operation: BinaryOperation,
        left: Atom,
        right: Atom,
    },
    Cut {
        text: Atom,
        start: Atom,
        end: Atom,
    },
    Slice {
        bytes: Atom,
        start: Atom,
        end: Atom,
    },
    Call {
        weave: String,
        arguments: Vec<Atom>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    Not,
    Measure,
    Render,
    Extent,
    Encode,
    Decode,
}

impl UnaryOperation {
    const fn word(self) -> &'static str {
        match self {
            Self::Not => "not",
            Self::Measure => "measure",
            Self::Render => "render",
            Self::Extent => "extent",
            Self::Encode => "encode",
            Self::Decode => "decode",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    Sum,
    Difference,
    Product,
    Less,
    Same,
    Join,
    Glyph,
    Quotient,
    Remainder,
    Fuse,
    Append,
    Octet,
}

impl BinaryOperation {
    const fn word(self) -> &'static str {
        match self {
            Self::Sum => "sum",
            Self::Difference => "difference",
            Self::Product => "product",
            Self::Less => "less",
            Self::Same => "same",
            Self::Join => "join",
            Self::Glyph => "glyph",
            Self::Quotient => "quotient",
            Self::Remainder => "remainder",
            Self::Fuse => "fuse",
            Self::Append => "append",
            Self::Octet => "octet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    pub kind: AtomKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Text(String),
    Bytes(Vec<u8>),
    Whole(i64),
    Truth(bool),
    Name(String),
    Borrow(String),
    Move(String),
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
pub enum InvocationValue {
    Text(String),
    Whole(i64),
    Truth(bool),
    Bytes(Vec<u8>),
}

impl InvocationValue {
    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        match self {
            Self::Text(_) => ValueType::Text,
            Self::Whole(_) => ValueType::Whole,
            Self::Truth(_) => ValueType::Truth,
            Self::Bytes(_) => ValueType::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationOutput {
    pub stdout: String,
    pub value: InvocationValue,
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
    indentation: usize,
    content: &'source str,
}

impl<'source> SourceLine<'source> {
    const fn span(self, column: usize) -> Span {
        Span::new(self.number, column)
    }
}

#[derive(Clone)]
struct BindingState {
    value_type: ValueType,
    mutable: bool,
    moved: bool,
}

#[derive(Clone)]
struct FunctionSignature {
    parameters: Vec<Parameter>,
    result: ValueType,
}

#[derive(Clone, Copy)]
struct SlotInfo {
    index: u16,
    value_type: ValueType,
    mutable: bool,
}

#[derive(Clone)]
struct LocalDescriptor {
    value_type: ValueType,
    mutable: bool,
}

#[derive(Clone)]
struct ArtifactFunction {
    name: String,
    parameters: Vec<(ValueType, ParameterMode)>,
    result: ValueType,
    locals: Vec<LocalDescriptor>,
    code: Vec<u8>,
}

#[derive(Clone)]
struct Artifact {
    functions: Vec<ArtifactFunction>,
}

#[derive(Clone, PartialEq, Eq)]
struct VerificationState {
    stack: Vec<ValueType>,
    initialized: Vec<bool>,
    moved: Vec<bool>,
}

#[derive(Debug, Clone)]
enum RuntimeValue {
    Text(String),
    Whole(i64),
    Truth(bool),
    Bytes(Vec<u8>),
}

impl RuntimeValue {
    const fn value_type(&self) -> ValueType {
        match self {
            Self::Text(_) => ValueType::Text,
            Self::Whole(_) => ValueType::Whole,
            Self::Truth(_) => ValueType::Truth,
            Self::Bytes(_) => ValueType::Bytes,
        }
    }
}

fn runtime_from_invocation(value: &InvocationValue) -> Result<RuntimeValue, BytecodeError> {
    match value {
        InvocationValue::Text(text) => {
            if text.len() > MAX_TEXT_BYTES {
                return Err(BytecodeError::new(
                    0,
                    "invocation Text exceeds the Aether text safety limit",
                ));
            }
            Ok(RuntimeValue::Text(text.clone()))
        }
        InvocationValue::Whole(value) => Ok(RuntimeValue::Whole(*value)),
        InvocationValue::Truth(value) => Ok(RuntimeValue::Truth(*value)),
        InvocationValue::Bytes(bytes) => {
            if bytes.len() > MAX_BYTES {
                return Err(BytecodeError::new(
                    0,
                    "invocation Bytes exceeds the Aether bytes safety limit",
                ));
            }
            Ok(RuntimeValue::Bytes(bytes.clone()))
        }
    }
}

fn invocation_from_runtime(value: RuntimeValue) -> InvocationValue {
    match value {
        RuntimeValue::Text(value) => InvocationValue::Text(value),
        RuntimeValue::Whole(value) => InvocationValue::Whole(value),
        RuntimeValue::Truth(value) => InvocationValue::Truth(value),
        RuntimeValue::Bytes(value) => InvocationValue::Bytes(value),
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    PushText(String),
    PushBytes(Vec<u8>),
    PushWhole(i64),
    PushTruth(bool),
    Store(usize),
    Load(usize),
    Move(usize),
    Revise(usize),
    Speak,
    Yield,
    Sum,
    Difference,
    Product,
    Less,
    Same,
    Not,
    Join,
    Measure,
    Glyph,
    Cut,
    Quotient,
    Remainder,
    Fuse,
    Append,
    Extent,
    Octet,
    Slice,
    Encode,
    Decode,
    Render,
    Call { function: usize, arguments: usize },
    JumpIfDim(usize),
    Jump(usize),
}

#[derive(Debug, Clone)]
struct DecodedInstruction {
    offset: usize,
    next_offset: usize,
    instruction: Instruction,
}

pub fn compile_source(source: &str) -> Result<Program, CompilerError> {
    if source.is_empty() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "source is empty; an Aether world is required",
        ));
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!("source exceeds the Aether {MAX_SOURCE_BYTES}-byte safety limit"),
        ));
    }
    let lines = source_lines(source)?;
    if lines.len() < 2 {
        return Err(CompilerError::new(
            Span::synthetic(),
            "an Aether program needs a world and at least one weave",
        ));
    }

    let world = parse_world(lines[0])?;
    let mut index = 1;
    let mut weaves = Vec::new();
    while index < lines.len() {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "a weave declaration must begin at indentation level zero",
            ));
        }
        let (name, parameters, result) = parse_weave_header(line)?;
        index += 1;
        let body = parse_block(&lines, &mut index, 1)?;
        if body.is_empty() {
            return Err(CompilerError::new(
                line.span(1),
                "every weave requires at least one body statement",
            ));
        }
        weaves.push(Weave {
            name,
            parameters,
            result,
            body,
            span: line.span(1),
        });
    }

    let program = Program { world, weaves };
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
    let mut formatted = format!("world {}\n", program.world);
    for weave in &program.weaves {
        formatted.push('\n');
        formatted.push_str("weave ");
        formatted.push_str(&weave.name);
        formatted.push_str(" [");
        for (index, parameter) in weave.parameters.iter().enumerate() {
            if index > 0 {
                formatted.push_str(", ");
            }
            if parameter.mode == ParameterMode::Borrow {
                formatted.push_str("borrow ");
            }
            formatted.push_str(&parameter.name);
            formatted.push_str(": ");
            formatted.push_str(&parameter.value_type.to_string());
        }
        formatted.push_str("] -> ");
        formatted.push_str(&weave.result.to_string());
        formatted.push_str(":\n");
        write_block(&weave.body, 1, &mut formatted);
    }
    formatted
}

#[must_use]
pub fn canonical_ast(program: &Program) -> String {
    let mut output = String::from("World(");
    output.push_str(&program.world);
    output.push(')');
    for weave in &program.weaves {
        output.push_str(";Weave(");
        output.push_str(&weave.name);
        output.push_str("->");
        output.push_str(&weave.result.to_string());
        output.push_str(")[");
        for (index, parameter) in weave.parameters.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            if parameter.mode == ParameterMode::Borrow {
                output.push_str("Borrow ");
            }
            output.push_str(&parameter.name);
            output.push(':');
            output.push_str(&parameter.value_type.to_string());
        }
        output.push(']');
        write_ast_block(&weave.body, &mut output);
    }
    output
}

pub fn verify_bytecode(bytecode: &[u8]) -> Result<(), BytecodeError> {
    let artifact = parse_artifact(bytecode)?;
    if artifact.functions.is_empty() {
        return Err(BytecodeError::new(5, "artifact defines no weaves"));
    }

    let mut names = BTreeMap::new();
    let mut main_index = None;
    for (index, function) in artifact.functions.iter().enumerate() {
        if names.insert(function.name.as_str(), index).is_some() {
            return Err(BytecodeError::new(0, "artifact defines a weave name twice"));
        }
        if function.name == "main" {
            main_index = Some(index);
        }
        if function.locals.len() < function.parameters.len() {
            return Err(BytecodeError::new(
                0,
                "artifact local table omits one or more parameters",
            ));
        }
        for (parameter_index, (value_type, _)) in function.parameters.iter().enumerate() {
            if function.locals[parameter_index].value_type != *value_type
                || function.locals[parameter_index].mutable
            {
                return Err(BytecodeError::new(
                    0,
                    "artifact parameter slots must be immutable and match their declared type",
                ));
            }
        }
    }

    let Some(main_index) = main_index else {
        return Err(BytecodeError::new(0, "artifact has no main weave"));
    };
    let main = &artifact.functions[main_index];
    if !main.parameters.is_empty() || main.result != ValueType::Whole {
        return Err(BytecodeError::new(
            0,
            "main must accept no parameters and yield Whole",
        ));
    }

    for (function_index, function) in artifact.functions.iter().enumerate() {
        verify_function(function_index, function, &artifact.functions)?;
    }
    Ok(())
}

pub fn run_bytecode(bytecode: &[u8]) -> Result<RunOutput, BytecodeError> {
    let output = invoke_bytecode(bytecode, "main", &[])?;
    let InvocationValue::Whole(exit_code) = output.value else {
        return Err(BytecodeError::new(0, "main did not yield Whole"));
    };
    Ok(RunOutput {
        stdout: output.stdout,
        exit_code,
    })
}

/// Invokes one verified named Aether weave with values supplied by a trusted host.
///
/// Artifacts must still declare a valid `main` weave so they remain independently
/// runnable and verifiable.
pub fn invoke_bytecode(
    bytecode: &[u8],
    weave_name: &str,
    arguments: &[InvocationValue],
) -> Result<InvocationOutput, BytecodeError> {
    if weave_name.is_empty() {
        return Err(BytecodeError::new(0, "invoked weave name is empty"));
    }
    verify_bytecode(bytecode)?;
    let artifact = parse_artifact(bytecode)?;
    let function_index = artifact
        .functions
        .iter()
        .position(|function| function.name == weave_name)
        .ok_or_else(|| {
            BytecodeError::new(0, format!("artifact has no weave named {weave_name}"))
        })?;
    invoke_artifact(&artifact, function_index, weave_name, arguments)
}

/// Invokes the fixed compiler ABI used by `aether forge`.
///
/// The compiler artifact must expose `compile [borrow source: Text] -> Bytes` and
/// retain a valid runnable `main` weave. The host passes only the source text and
/// writes the returned bytes after independently verifying them.
pub fn forge_bytecode(compiler: &[u8], source: &str) -> Result<InvocationOutput, BytecodeError> {
    verify_bytecode(compiler)?;
    let artifact = parse_artifact(compiler)?;
    let function_index = artifact
        .functions
        .iter()
        .position(|function| function.name == "compile")
        .ok_or_else(|| BytecodeError::new(0, "artifact has no weave named compile"))?;
    let function = &artifact.functions[function_index];
    if function.parameters.len() != 1
        || function.parameters[0] != (ValueType::Text, ParameterMode::Borrow)
        || function.result != ValueType::Bytes
    {
        return Err(BytecodeError::new(
            0,
            "compiler weave must have signature [borrow source: Text] -> Bytes",
        ));
    }
    invoke_artifact(
        &artifact,
        function_index,
        "compile",
        &[InvocationValue::Text(source.to_owned())],
    )
}

fn invoke_artifact(
    artifact: &Artifact,
    function_index: usize,
    weave_name: &str,
    arguments: &[InvocationValue],
) -> Result<InvocationOutput, BytecodeError> {
    let function = artifact
        .functions
        .get(function_index)
        .ok_or_else(|| BytecodeError::new(0, "artifact invocation index is invalid"))?;
    if function.parameters.len() != arguments.len() {
        return Err(BytecodeError::new(
            0,
            format!(
                "weave {weave_name} requires {} argument(s), received {}",
                function.parameters.len(),
                arguments.len()
            ),
        ));
    }
    let mut runtime_arguments = Vec::with_capacity(arguments.len());
    for (index, (argument, (expected, _))) in arguments.iter().zip(&function.parameters).enumerate()
    {
        if argument.value_type() != *expected {
            return Err(BytecodeError::new(
                0,
                format!(
                    "weave {weave_name} argument {} requires {expected}, received {}",
                    index + 1,
                    argument.value_type()
                ),
            ));
        }
        runtime_arguments.push(runtime_from_invocation(argument)?);
    }
    let mut stdout = String::new();
    let value = execute_function(artifact, function_index, runtime_arguments, &mut stdout, 0)?;
    Ok(InvocationOutput {
        stdout,
        value: invocation_from_runtime(value),
    })
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
        if line.is_empty() {
            continue;
        }
        if line.trim().is_empty() {
            return Err(CompilerError::new(
                Span::new(index + 1, 1),
                "blank lines cannot contain whitespace",
            ));
        }
        if line.ends_with(' ') || line.ends_with('\t') {
            return Err(CompilerError::new(
                Span::new(index + 1, line.len()),
                "trailing whitespace is not part of canonical Aether source",
            ));
        }
        if line.contains('\t') {
            return Err(CompilerError::new(
                Span::new(index + 1, 1),
                "tabs are not valid indentation; use two spaces per Aether block level",
            ));
        }
        let indentation = line.bytes().take_while(|byte| *byte == b' ').count();
        if indentation % 2 != 0 {
            return Err(CompilerError::new(
                Span::new(index + 1, indentation + 1),
                "Aether indentation uses exact two-space levels",
            ));
        }
        lines.push(SourceLine {
            number: index + 1,
            indentation: indentation / 2,
            content: &line[indentation..],
        });
    }
    Ok(lines)
}

fn parse_world(line: SourceLine<'_>) -> Result<String, CompilerError> {
    if line.indentation != 0 {
        return Err(CompilerError::new(
            line.span(1),
            "world declarations cannot be indented",
        ));
    }
    let Some(name) = line.content.strip_prefix("world ") else {
        return Err(CompilerError::new(
            line.span(1),
            "the first declaration must be world followed by a lowercase name",
        ));
    };
    validate_name(name, line.span(7), "world name", false)?;
    Ok(name.to_owned())
}

fn parse_weave_header(
    line: SourceLine<'_>,
) -> Result<(String, Vec<Parameter>, ValueType), CompilerError> {
    let Some(without_colon) = line.content.strip_suffix(':') else {
        return Err(CompilerError::new(
            line.span(1),
            "weave declarations must end with a colon",
        ));
    };
    let Some(rest) = without_colon.strip_prefix("weave ") else {
        return Err(CompilerError::new(
            line.span(1),
            "expected weave declaration",
        ));
    };
    let Some(opening) = rest.find('[') else {
        return Err(CompilerError::new(
            line.span(1),
            "weave parameters must be enclosed by square brackets",
        ));
    };
    let name = rest[..opening].trim_end();
    validate_name(name, line.span(7), "weave name", true)?;
    let after_opening = &rest[opening + 1..];
    let Some(closing) = after_opening.find(']') else {
        return Err(CompilerError::new(
            line.span(7 + opening + 1),
            "weave parameter list is missing its closing bracket",
        ));
    };
    let parameters = parse_parameters(&after_opening[..closing], line)?;
    let after_parameters = after_opening[closing + 1..].trim();
    let Some(result_text) = after_parameters.strip_prefix("-> ") else {
        return Err(CompilerError::new(
            line.span(1),
            "weave result must use -> Type",
        ));
    };
    let result = parse_value_type(result_text, line.span(line.content.len()))?;
    Ok((name.to_owned(), parameters, result))
}

fn parse_parameters(source: &str, line: SourceLine<'_>) -> Result<Vec<Parameter>, CompilerError> {
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut parameters = Vec::new();
    for segment in source.split(',') {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            return Err(CompilerError::new(
                line.span(1),
                "weave parameter lists cannot contain an empty entry",
            ));
        }
        let (mode, declaration) = if let Some(rest) = trimmed.strip_prefix("borrow ") {
            (ParameterMode::Borrow, rest)
        } else {
            (ParameterMode::Own, trimmed)
        };
        let Some((name, type_text)) = declaration.split_once(": ") else {
            return Err(CompilerError::new(
                line.span(1),
                "each parameter must use name: Type",
            ));
        };
        validate_name(name, line.span(1), "parameter name", false)?;
        let value_type = parse_value_type(type_text, line.span(1))?;
        if mode == ParameterMode::Borrow && !is_unique_value(value_type) {
            return Err(CompilerError::new(
                line.span(1),
                "borrow parameters are reserved for unique Text or Bytes values",
            ));
        }
        if parameters
            .iter()
            .any(|parameter: &Parameter| parameter.name == name)
        {
            return Err(CompilerError::new(
                line.span(1),
                format!("parameter {name} is declared more than once"),
            ));
        }
        parameters.push(Parameter {
            name: name.to_owned(),
            value_type,
            mode,
            span: line.span(1),
        });
    }
    Ok(parameters)
}

fn parse_value_type(source: &str, span: Span) -> Result<ValueType, CompilerError> {
    match source {
        "Text" => Ok(ValueType::Text),
        "Whole" => Ok(ValueType::Whole),
        "Truth" => Ok(ValueType::Truth),
        "Bytes" => Ok(ValueType::Bytes),
        _ => Err(CompilerError::new(
            span,
            "Aether types are Text, Whole, Truth, or Bytes",
        )),
    }
}

fn parse_block(
    lines: &[SourceLine<'_>],
    index: &mut usize,
    indentation: usize,
) -> Result<Vec<Statement>, CompilerError> {
    let mut statements = Vec::new();
    while *index < lines.len() {
        let line = lines[*index];
        if line.indentation < indentation {
            break;
        }
        if line.indentation > indentation {
            return Err(CompilerError::new(
                line.span(1),
                "this line is more deeply indented than its enclosing Aether block",
            ));
        }
        if line.content == "otherwise:" {
            break;
        }

        if let Some(condition_source) = line
            .content
            .strip_prefix("choose ")
            .and_then(|source| source.strip_suffix(':'))
        {
            let condition = parse_expression(condition_source, line.span(3 + "choose ".len()))?;
            *index += 1;
            let when_bright = parse_block(lines, index, indentation + 1)?;
            if when_bright.is_empty() {
                return Err(CompilerError::new(
                    line.span(1),
                    "choose requires a nonempty bright branch",
                ));
            }
            let mut when_dim = Vec::new();
            if *index < lines.len()
                && lines[*index].indentation == indentation
                && lines[*index].content == "otherwise:"
            {
                let otherwise_line = lines[*index];
                *index += 1;
                when_dim = parse_block(lines, index, indentation + 1)?;
                if when_dim.is_empty() {
                    return Err(CompilerError::new(
                        otherwise_line.span(1),
                        "otherwise requires a nonempty dim branch",
                    ));
                }
            }
            statements.push(Statement::Choose {
                condition,
                when_bright,
                when_dim,
                span: line.span(1),
            });
            continue;
        }

        if let Some(condition_source) = line
            .content
            .strip_prefix("while ")
            .and_then(|source| source.strip_suffix(':'))
        {
            let condition = parse_expression(condition_source, line.span(3 + "while ".len()))?;
            *index += 1;
            let body = parse_block(lines, index, indentation + 1)?;
            if body.is_empty() {
                return Err(CompilerError::new(
                    line.span(1),
                    "while requires a nonempty body",
                ));
            }
            statements.push(Statement::While {
                condition,
                body,
                span: line.span(1),
            });
            continue;
        }

        if line.content.ends_with(':') {
            return Err(CompilerError::new(
                line.span(1),
                "unknown Aether block form",
            ));
        }
        statements.push(parse_plain_statement(line)?);
        *index += 1;
    }
    Ok(statements)
}

fn parse_plain_statement(line: SourceLine<'_>) -> Result<Statement, CompilerError> {
    let span = line.span(line.indentation * 2 + 1);
    if let Some(rest) = line.content.strip_prefix("bind ") {
        let (mutable, rest) = if let Some(remainder) = rest.strip_prefix("mutable ") {
            (true, remainder)
        } else {
            (false, rest)
        };
        let Some((name, expression)) = rest.split_once(" <- ") else {
            return Err(CompilerError::new(
                span,
                "bind requires a name, the <- binder, and one value",
            ));
        };
        validate_name(name, span, "binding name", false)?;
        return Ok(Statement::Bind {
            name: name.to_owned(),
            mutable,
            value: parse_expression(expression, span)?,
            span,
        });
    }
    if let Some(rest) = line.content.strip_prefix("revise ") {
        let Some((name, expression)) = rest.split_once(" <- ") else {
            return Err(CompilerError::new(
                span,
                "revise requires a name, the <- binder, and one value",
            ));
        };
        validate_name(name, span, "binding name", false)?;
        return Ok(Statement::Revise {
            name: name.to_owned(),
            value: parse_expression(expression, span)?,
            span,
        });
    }
    if let Some(expression) = line.content.strip_prefix("speak ") {
        return Ok(Statement::Speak {
            value: parse_expression(expression, span)?,
            span,
        });
    }
    if let Some(expression) = line.content.strip_prefix("yield ") {
        return Ok(Statement::Yield {
            value: parse_expression(expression, span)?,
            span,
        });
    }
    Err(CompilerError::new(
        span,
        "unknown Aether statement; use bind, revise, speak, yield, choose, or while",
    ))
}

fn parse_expression(source: &str, span: Span) -> Result<Expression, CompilerError> {
    let tokens = tokenize_fragment(source, span)?;
    if tokens.is_empty() {
        return Err(CompilerError::new(span, "an expression is required"));
    }
    let first = tokens[0].as_str();
    let mut index = 1;
    let kind = match first {
        "not" => ExpressionKind::Unary {
            operation: UnaryOperation::Not,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "measure" => ExpressionKind::Unary {
            operation: UnaryOperation::Measure,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "render" => ExpressionKind::Unary {
            operation: UnaryOperation::Render,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "extent" => ExpressionKind::Unary {
            operation: UnaryOperation::Extent,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "encode" => ExpressionKind::Unary {
            operation: UnaryOperation::Encode,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "decode" => ExpressionKind::Unary {
            operation: UnaryOperation::Decode,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "sum" => ExpressionKind::Binary {
            operation: BinaryOperation::Sum,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "difference" => ExpressionKind::Binary {
            operation: BinaryOperation::Difference,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "product" => ExpressionKind::Binary {
            operation: BinaryOperation::Product,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "less" => ExpressionKind::Binary {
            operation: BinaryOperation::Less,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "same" => ExpressionKind::Binary {
            operation: BinaryOperation::Same,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "join" => ExpressionKind::Binary {
            operation: BinaryOperation::Join,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "glyph" => ExpressionKind::Binary {
            operation: BinaryOperation::Glyph,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "quotient" => ExpressionKind::Binary {
            operation: BinaryOperation::Quotient,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "remainder" => ExpressionKind::Binary {
            operation: BinaryOperation::Remainder,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "fuse" => ExpressionKind::Binary {
            operation: BinaryOperation::Fuse,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "append" => ExpressionKind::Binary {
            operation: BinaryOperation::Append,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "octet" => ExpressionKind::Binary {
            operation: BinaryOperation::Octet,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "cut" => ExpressionKind::Cut {
            text: parse_atom_from(&tokens, &mut index, span)?,
            start: parse_atom_from(&tokens, &mut index, span)?,
            end: parse_atom_from(&tokens, &mut index, span)?,
        },
        "slice" => ExpressionKind::Slice {
            bytes: parse_atom_from(&tokens, &mut index, span)?,
            start: parse_atom_from(&tokens, &mut index, span)?,
            end: parse_atom_from(&tokens, &mut index, span)?,
        },
        "call" => {
            let Some(weave) = tokens.get(index) else {
                return Err(CompilerError::new(span, "call requires a weave name"));
            };
            validate_name(weave, span, "called weave name", true)?;
            index += 1;
            let mut arguments = Vec::new();
            while index < tokens.len() {
                arguments.push(parse_atom_from(&tokens, &mut index, span)?);
            }
            ExpressionKind::Call {
                weave: weave.clone(),
                arguments,
            }
        }
        _ => {
            index = 0;
            ExpressionKind::Atom(parse_atom_from(&tokens, &mut index, span)?)
        }
    };
    if index != tokens.len() {
        return Err(CompilerError::new(
            span,
            "an Aether expression has unexpected trailing terms",
        ));
    }
    Ok(Expression { kind, span })
}

fn tokenize_fragment(source: &str, span: Span) -> Result<Vec<String>, CompilerError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        while index < bytes.len() && bytes[index] == b' ' {
            index += 1;
        }
        if index == bytes.len() {
            break;
        }
        let start = index;
        if bytes[index] == b'"' {
            index += 1;
            let mut escaped = false;
            let mut closed = false;
            while index < bytes.len() {
                let byte = bytes[index];
                index += 1;
                if escaped {
                    escaped = false;
                    continue;
                }
                if byte == b'\\' {
                    escaped = true;
                    continue;
                }
                if byte == b'"' {
                    closed = true;
                    break;
                }
            }
            if !closed {
                return Err(CompilerError::new(
                    span,
                    "text values must use one closed double-quoted literal",
                ));
            }
            if index < bytes.len() && bytes[index] != b' ' {
                return Err(CompilerError::new(
                    span,
                    "text literal must be followed by a space or the end of its expression",
                ));
            }
            tokens.push(source[start..index].to_owned());
            continue;
        }
        while index < bytes.len() && bytes[index] != b' ' {
            index += 1;
        }
        tokens.push(source[start..index].to_owned());
    }
    Ok(tokens)
}

fn parse_atom_from(
    tokens: &[String],
    index: &mut usize,
    span: Span,
) -> Result<Atom, CompilerError> {
    let Some(token) = tokens.get(*index) else {
        return Err(CompilerError::new(span, "expression is missing an operand"));
    };
    if token == "borrow" || token == "move" {
        let mode = token.as_str();
        *index += 1;
        let Some(name) = tokens.get(*index) else {
            return Err(CompilerError::new(
                span,
                format!("{mode} requires a bound value name"),
            ));
        };
        validate_name(name, span, "value name", false)?;
        *index += 1;
        return Ok(Atom {
            kind: if mode == "borrow" {
                AtomKind::Borrow(name.clone())
            } else {
                AtomKind::Move(name.clone())
            },
            span,
        });
    }
    if token == "bytes" {
        *index += 1;
        let Some(literal) = tokens.get(*index) else {
            return Err(CompilerError::new(
                span,
                "bytes requires one double-quoted hexadecimal literal",
            ));
        };
        *index += 1;
        return Ok(Atom {
            kind: AtomKind::Bytes(parse_bytes_literal(literal, span)?),
            span,
        });
    }
    *index += 1;
    if token.starts_with('"') {
        return Ok(Atom {
            kind: AtomKind::Text(parse_text_literal(token, span)?),
            span,
        });
    }
    if token == "bright" {
        return Ok(Atom {
            kind: AtomKind::Truth(true),
            span,
        });
    }
    if token == "dim" {
        return Ok(Atom {
            kind: AtomKind::Truth(false),
            span,
        });
    }
    if is_whole_literal(token) {
        let value = token.parse::<i64>().map_err(|_| {
            CompilerError::new(
                span,
                "whole literal is outside the supported signed 64-bit range",
            )
        })?;
        return Ok(Atom {
            kind: AtomKind::Whole(value),
            span,
        });
    }
    validate_name(token, span, "value name", false)?;
    Ok(Atom {
        kind: AtomKind::Name(token.clone()),
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
    if value.len() > MAX_TEXT_BYTES {
        return Err(CompilerError::new(
            span,
            format!("text literal exceeds the Aether {MAX_TEXT_BYTES}-byte safety limit"),
        ));
    }
    Ok(value)
}

fn parse_bytes_literal(source: &str, span: Span) -> Result<Vec<u8>, CompilerError> {
    let encoded = parse_text_literal(source, span)?;
    if encoded.len() % 2 != 0 || !encoded.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(CompilerError::new(
            span,
            "bytes literals require an even count of ASCII hexadecimal digits",
        ));
    }
    let length = encoded.len() / 2;
    if length > MAX_BYTES {
        return Err(CompilerError::new(
            span,
            format!("bytes literal exceeds the Aether {MAX_BYTES}-byte safety limit"),
        ));
    }
    let mut value = Vec::with_capacity(length);
    for pair in encoded.as_bytes().chunks_exact(2) {
        let high = hex_nibble(pair[0]).ok_or_else(|| {
            CompilerError::new(
                span,
                "bytes literals require an even count of ASCII hexadecimal digits",
            )
        })?;
        let low = hex_nibble(pair[1]).ok_or_else(|| {
            CompilerError::new(
                span,
                "bytes literals require an even count of ASCII hexadecimal digits",
            )
        })?;
        value.push((high << 4) | low);
    }
    Ok(value)
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn validate_program(program: &Program) -> Result<(), CompilerError> {
    if program.weaves.is_empty() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "an Aether program must declare at least one weave",
        ));
    }
    if program.weaves.len() > MAX_FUNCTIONS {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!("Aether supports at most {MAX_FUNCTIONS} weaves per artifact"),
        ));
    }

    let mut signatures = BTreeMap::new();
    for weave in &program.weaves {
        if signatures
            .insert(
                weave.name.clone(),
                FunctionSignature {
                    parameters: weave.parameters.clone(),
                    result: weave.result,
                },
            )
            .is_some()
        {
            return Err(CompilerError::new(
                weave.span,
                format!("weave {} is declared more than once", weave.name),
            ));
        }
    }

    let Some(main) = program.weaves.iter().find(|weave| weave.name == "main") else {
        return Err(CompilerError::new(
            Span::synthetic(),
            "an Aether program must declare weave main [] -> Whole:",
        ));
    };
    if !main.parameters.is_empty() || main.result != ValueType::Whole {
        return Err(CompilerError::new(
            main.span,
            "main must use the declaration weave main [] -> Whole:",
        ));
    }

    for weave in &program.weaves {
        let mut scope = BTreeMap::new();
        for parameter in &weave.parameters {
            scope.insert(
                parameter.name.clone(),
                BindingState {
                    value_type: parameter.value_type,
                    mutable: false,
                    moved: false,
                },
            );
        }
        validate_block(&weave.body, &mut scope, &signatures, weave, true)?;
    }
    Ok(())
}

fn validate_block(
    statements: &[Statement],
    scope: &mut BTreeMap<String, BindingState>,
    signatures: &BTreeMap<String, FunctionSignature>,
    weave: &Weave,
    root: bool,
) -> Result<(), CompilerError> {
    for (index, statement) in statements.iter().enumerate() {
        match statement {
            Statement::Bind {
                name,
                mutable,
                value,
                span,
            } => {
                if !root {
                    return Err(CompilerError::new(
                        *span,
                        "bind is only allowed in a weave root; use a mutable root binding with revise inside blocks",
                    ));
                }
                if scope.contains_key(name) {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} already exists in this weave"),
                    ));
                }
                let value_type = expression_type(value, scope, signatures)?;
                scope.insert(
                    name.clone(),
                    BindingState {
                        value_type,
                        mutable: *mutable,
                        moved: false,
                    },
                );
            }
            Statement::Revise { name, value, span } => {
                let Some(binding) = scope.get(name) else {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} has not been introduced"),
                    ));
                };
                if !binding.mutable {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} is immutable; declare it with bind mutable"),
                    ));
                }
                if binding.moved {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} was moved and cannot be revised"),
                    ));
                }
                let expected = binding.value_type;
                let actual = expression_type(value, scope, signatures)?;
                require_source_type(actual, expected, value.span, "revise")?;
            }
            Statement::Speak { value, .. } => {
                let value_type = expression_type(value, scope, signatures)?;
                require_source_type(value_type, ValueType::Text, value.span, "speak")?;
            }
            Statement::Yield { value, span } => {
                if !root || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        "yield is allowed only as the final statement of a weave root",
                    ));
                }
                let value_type = expression_type(value, scope, signatures)?;
                require_source_type(value_type, weave.result, value.span, "yield")?;
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                let condition_type = expression_type(condition, scope, signatures)?;
                require_source_type(
                    condition_type,
                    ValueType::Truth,
                    condition.span,
                    "choose condition",
                )?;
                let original = scope.clone();
                let mut bright_scope = original.clone();
                validate_block(when_bright, &mut bright_scope, signatures, weave, false)?;
                let mut dim_scope = original.clone();
                if !when_dim.is_empty() {
                    validate_block(when_dim, &mut dim_scope, signatures, weave, false)?;
                }
                merge_scope(scope, &bright_scope, &dim_scope, statement.span())?;
            }
            Statement::While {
                condition, body, ..
            } => {
                let condition_type = expression_type(condition, scope, signatures)?;
                require_source_type(
                    condition_type,
                    ValueType::Truth,
                    condition.span,
                    "while condition",
                )?;
                let before_loop = scope.clone();
                let mut body_scope = before_loop.clone();
                validate_block(body, &mut body_scope, signatures, weave, false)?;
                merge_scope(scope, &before_loop, &body_scope, statement.span())?;
            }
        }
    }
    if root && !matches!(statements.last(), Some(Statement::Yield { .. })) {
        return Err(CompilerError::new(
            weave.span,
            "every weave must end with yield",
        ));
    }
    Ok(())
}

fn merge_scope(
    destination: &mut BTreeMap<String, BindingState>,
    left: &BTreeMap<String, BindingState>,
    right: &BTreeMap<String, BindingState>,
    span: Span,
) -> Result<(), CompilerError> {
    if left.len() != right.len() || left.keys().ne(right.keys()) {
        return Err(CompilerError::new(
            span,
            "Aether blocks cannot introduce a binding conditionally",
        ));
    }
    for (name, left_binding) in left {
        let Some(right_binding) = right.get(name) else {
            return Err(CompilerError::new(
                span,
                "Aether block binding state is inconsistent",
            ));
        };
        if left_binding.value_type != right_binding.value_type
            || left_binding.mutable != right_binding.mutable
        {
            return Err(CompilerError::new(
                span,
                format!("binding {name} changes its declared shape across control flow"),
            ));
        }
        destination.insert(
            name.clone(),
            BindingState {
                value_type: left_binding.value_type,
                mutable: left_binding.mutable,
                moved: left_binding.moved || right_binding.moved,
            },
        );
    }
    Ok(())
}

fn expression_type(
    expression: &Expression,
    scope: &mut BTreeMap<String, BindingState>,
    signatures: &BTreeMap<String, FunctionSignature>,
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => atom_type(atom, scope),
        ExpressionKind::Unary {
            operation,
            argument,
        } => {
            let argument_type = atom_type(argument, scope)?;
            match operation {
                UnaryOperation::Not => {
                    require_source_type(argument_type, ValueType::Truth, argument.span, "not")?;
                    Ok(ValueType::Truth)
                }
                UnaryOperation::Measure => {
                    require_source_type(argument_type, ValueType::Text, argument.span, "measure")?;
                    Ok(ValueType::Whole)
                }
                UnaryOperation::Render => {
                    if argument_type == ValueType::Bytes {
                        return Err(CompilerError::new(
                            argument.span,
                            "render does not accept Bytes; inspect bytes with extent, octet, or decode",
                        ));
                    }
                    Ok(ValueType::Text)
                }
                UnaryOperation::Extent => {
                    require_source_type(argument_type, ValueType::Bytes, argument.span, "extent")?;
                    Ok(ValueType::Whole)
                }
                UnaryOperation::Encode => {
                    require_source_type(argument_type, ValueType::Text, argument.span, "encode")?;
                    Ok(ValueType::Bytes)
                }
                UnaryOperation::Decode => {
                    require_source_type(argument_type, ValueType::Bytes, argument.span, "decode")?;
                    Ok(ValueType::Text)
                }
            }
        }
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            let left_type = atom_type(left, scope)?;
            let right_type = atom_type(right, scope)?;
            match operation {
                BinaryOperation::Sum
                | BinaryOperation::Difference
                | BinaryOperation::Product
                | BinaryOperation::Quotient
                | BinaryOperation::Remainder => {
                    require_source_type(left_type, ValueType::Whole, left.span, operation.word())?;
                    require_source_type(
                        right_type,
                        ValueType::Whole,
                        right.span,
                        operation.word(),
                    )?;
                    Ok(ValueType::Whole)
                }
                BinaryOperation::Less => {
                    require_source_type(left_type, ValueType::Whole, left.span, "less")?;
                    require_source_type(right_type, ValueType::Whole, right.span, "less")?;
                    Ok(ValueType::Truth)
                }
                BinaryOperation::Same => {
                    if left_type != right_type {
                        return Err(CompilerError::new(
                            expression.span,
                            format!("same requires equal value shapes, not {left_type} and {right_type}"),
                        ));
                    }
                    Ok(ValueType::Truth)
                }
                BinaryOperation::Join => {
                    require_source_type(left_type, ValueType::Text, left.span, "join")?;
                    require_source_type(right_type, ValueType::Text, right.span, "join")?;
                    Ok(ValueType::Text)
                }
                BinaryOperation::Glyph => {
                    require_source_type(left_type, ValueType::Text, left.span, "glyph")?;
                    require_source_type(right_type, ValueType::Whole, right.span, "glyph")?;
                    Ok(ValueType::Whole)
                }
                BinaryOperation::Fuse => {
                    require_source_type(left_type, ValueType::Bytes, left.span, "fuse")?;
                    require_source_type(right_type, ValueType::Bytes, right.span, "fuse")?;
                    Ok(ValueType::Bytes)
                }
                BinaryOperation::Append => {
                    require_source_type(left_type, ValueType::Bytes, left.span, "append")?;
                    require_source_type(right_type, ValueType::Whole, right.span, "append")?;
                    Ok(ValueType::Bytes)
                }
                BinaryOperation::Octet => {
                    require_source_type(left_type, ValueType::Bytes, left.span, "octet")?;
                    require_source_type(right_type, ValueType::Whole, right.span, "octet")?;
                    Ok(ValueType::Whole)
                }
            }
        }
        ExpressionKind::Cut { text, start, end } => {
            require_source_type(atom_type(text, scope)?, ValueType::Text, text.span, "cut")?;
            require_source_type(
                atom_type(start, scope)?,
                ValueType::Whole,
                start.span,
                "cut",
            )?;
            require_source_type(atom_type(end, scope)?, ValueType::Whole, end.span, "cut")?;
            Ok(ValueType::Text)
        }
        ExpressionKind::Slice { bytes, start, end } => {
            require_source_type(
                atom_type(bytes, scope)?,
                ValueType::Bytes,
                bytes.span,
                "slice",
            )?;
            require_source_type(
                atom_type(start, scope)?,
                ValueType::Whole,
                start.span,
                "slice",
            )?;
            require_source_type(atom_type(end, scope)?, ValueType::Whole, end.span, "slice")?;
            Ok(ValueType::Bytes)
        }
        ExpressionKind::Call { weave, arguments } => {
            let Some(signature) = signatures.get(weave) else {
                return Err(CompilerError::new(
                    expression.span,
                    format!("weave {weave} has not been declared"),
                ));
            };
            if signature.parameters.len() != arguments.len() {
                return Err(CompilerError::new(
                    expression.span,
                    format!(
                        "call {weave} requires {} argument(s), received {}",
                        signature.parameters.len(),
                        arguments.len()
                    ),
                ));
            }
            for (argument, parameter) in arguments.iter().zip(&signature.parameters) {
                let argument_type = atom_type(argument, scope)?;
                require_source_type(
                    argument_type,
                    parameter.value_type,
                    argument.span,
                    "call argument",
                )?;
                let direct_owned_literal = matches!(
                    (&argument.kind, parameter.value_type),
                    (AtomKind::Text(_), ValueType::Text) | (AtomKind::Bytes(_), ValueType::Bytes)
                );
                if parameter.mode == ParameterMode::Own
                    && is_unique_value(parameter.value_type)
                    && !direct_owned_literal
                    && !matches!(argument.kind, AtomKind::Move(_))
                {
                    return Err(CompilerError::new(
                        argument.span,
                        format!(
                            "call {weave} consumes {} parameter {}; use move name or a matching literal",
                            parameter.value_type, parameter.name
                        ),
                    ));
                }
            }
            Ok(signature.result)
        }
    }
}

fn atom_type(
    atom: &Atom,
    scope: &mut BTreeMap<String, BindingState>,
) -> Result<ValueType, CompilerError> {
    match &atom.kind {
        AtomKind::Text(_) => Ok(ValueType::Text),
        AtomKind::Bytes(_) => Ok(ValueType::Bytes),
        AtomKind::Whole(_) => Ok(ValueType::Whole),
        AtomKind::Truth(_) => Ok(ValueType::Truth),
        AtomKind::Name(name) => {
            let Some(binding) = scope.get(name) else {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} has not been bound in this weave"),
                ));
            };
            if binding.moved {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} was moved and cannot be read"),
                ));
            }
            if is_unique_value(binding.value_type) {
                return Err(CompilerError::new(
                    atom.span,
                    format!(
                        "{} value {name} requires explicit borrow or move",
                        binding.value_type
                    ),
                ));
            }
            Ok(binding.value_type)
        }
        AtomKind::Borrow(name) => {
            let Some(binding) = scope.get(name) else {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} has not been bound in this weave"),
                ));
            };
            if binding.moved {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} was moved and cannot be borrowed"),
                ));
            }
            Ok(binding.value_type)
        }
        AtomKind::Move(name) => {
            let Some(binding) = scope.get_mut(name) else {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} has not been bound in this weave"),
                ));
            };
            if binding.moved {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} was already moved"),
                ));
            }
            binding.moved = true;
            Ok(binding.value_type)
        }
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
    let mut weave_indices = BTreeMap::new();
    let mut weave_results = BTreeMap::new();
    for (index, weave) in program.weaves.iter().enumerate() {
        weave_indices.insert(weave.name.clone(), index);
        weave_results.insert(weave.name.clone(), weave.result);
    }

    let mut compiled = Vec::new();
    for weave in &program.weaves {
        let layout = slot_layout(weave, &weave_results)?;
        let mut code = Vec::new();
        emit_block(&weave.body, &layout, &weave_indices, &mut code)?;
        let mut locals = vec![
            LocalDescriptor {
                value_type: ValueType::Whole,
                mutable: false,
            };
            layout.len()
        ];
        for slot in layout.values() {
            locals[usize::from(slot.index)] = LocalDescriptor {
                value_type: slot.value_type,
                mutable: slot.mutable,
            };
        }
        compiled.push((weave, locals, code));
    }

    let mut bytecode = Vec::from(&ARTIFACT_MAGIC[..]);
    bytecode.push(ARTIFACT_VERSION);
    write_u16(
        &mut bytecode,
        u16::try_from(compiled.len()).map_err(|_| {
            CompilerError::new(
                Span::synthetic(),
                "artifact contains too many weaves for AETH",
            )
        })?,
    );
    for (weave, locals, code) in compiled {
        let name_length = u8::try_from(weave.name.len()).map_err(|_| {
            CompilerError::new(weave.span, "weave name exceeds the AETH name limit")
        })?;
        bytecode.push(name_length);
        bytecode.extend_from_slice(weave.name.as_bytes());
        let parameter_count = u8::try_from(weave.parameters.len()).map_err(|_| {
            CompilerError::new(weave.span, "weave has too many parameters for AETH")
        })?;
        bytecode.push(parameter_count);
        for parameter in &weave.parameters {
            bytecode.push(parameter.value_type.to_byte());
            bytecode.push(parameter.mode.to_byte());
        }
        bytecode.push(weave.result.to_byte());
        write_u16(
            &mut bytecode,
            u16::try_from(locals.len()).map_err(|_| {
                CompilerError::new(weave.span, "weave has too many local bindings for AETH")
            })?,
        );
        for local in locals {
            bytecode.push(local.value_type.to_byte());
            bytecode.push(u8::from(local.mutable));
        }
        write_u32(
            &mut bytecode,
            u32::try_from(code.len()).map_err(|_| {
                CompilerError::new(weave.span, "weave bytecode exceeds the AETH code limit")
            })?,
        );
        bytecode.extend_from_slice(&code);
    }
    Ok(bytecode)
}

fn slot_layout(
    weave: &Weave,
    weave_results: &BTreeMap<String, ValueType>,
) -> Result<BTreeMap<String, SlotInfo>, CompilerError> {
    let mut layout = BTreeMap::new();
    let mut next_slot = 0_u16;
    for parameter in &weave.parameters {
        layout.insert(
            parameter.name.clone(),
            SlotInfo {
                index: next_slot,
                value_type: parameter.value_type,
                mutable: false,
            },
        );
        next_slot = next_slot.checked_add(1).ok_or_else(|| {
            CompilerError::new(parameter.span, "weave has too many local bindings")
        })?;
    }
    for statement in &weave.body {
        if let Statement::Bind {
            name,
            mutable,
            value,
            span,
        } = statement
        {
            let value_type = static_expression_type(value, &layout, weave_results)?;
            if layout.len() >= MAX_LOCALS {
                return Err(CompilerError::new(
                    *span,
                    "weave has too many local bindings",
                ));
            }
            layout.insert(
                name.clone(),
                SlotInfo {
                    index: next_slot,
                    value_type,
                    mutable: *mutable,
                },
            );
            next_slot = next_slot
                .checked_add(1)
                .ok_or_else(|| CompilerError::new(*span, "weave has too many local bindings"))?;
        }
    }
    Ok(layout)
}

fn static_expression_type(
    expression: &Expression,
    layout: &BTreeMap<String, SlotInfo>,
    weave_results: &BTreeMap<String, ValueType>,
) -> Result<ValueType, CompilerError> {
    let atom_type = |atom: &Atom| -> Result<ValueType, CompilerError> {
        match &atom.kind {
            AtomKind::Text(_) => Ok(ValueType::Text),
            AtomKind::Bytes(_) => Ok(ValueType::Bytes),
            AtomKind::Whole(_) => Ok(ValueType::Whole),
            AtomKind::Truth(_) => Ok(ValueType::Truth),
            AtomKind::Name(name) | AtomKind::Borrow(name) | AtomKind::Move(name) => {
                layout.get(name).map(|slot| slot.value_type).ok_or_else(|| {
                    CompilerError::new(
                        atom.span,
                        format!("value {name} has not been introduced before this binding"),
                    )
                })
            }
        }
    };
    match &expression.kind {
        ExpressionKind::Atom(atom) => atom_type(atom),
        ExpressionKind::Unary {
            operation,
            argument,
        } => match operation {
            UnaryOperation::Not => Ok(ValueType::Truth),
            UnaryOperation::Measure => Ok(ValueType::Whole),
            UnaryOperation::Render => {
                let _ = atom_type(argument)?;
                Ok(ValueType::Text)
            }
            UnaryOperation::Extent => Ok(ValueType::Whole),
            UnaryOperation::Encode => Ok(ValueType::Bytes),
            UnaryOperation::Decode => Ok(ValueType::Text),
        },
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            let _ = atom_type(left)?;
            let _ = atom_type(right)?;
            match operation {
                BinaryOperation::Sum
                | BinaryOperation::Difference
                | BinaryOperation::Product
                | BinaryOperation::Glyph
                | BinaryOperation::Quotient
                | BinaryOperation::Remainder
                | BinaryOperation::Octet => Ok(ValueType::Whole),
                BinaryOperation::Less | BinaryOperation::Same => Ok(ValueType::Truth),
                BinaryOperation::Join => Ok(ValueType::Text),
                BinaryOperation::Fuse | BinaryOperation::Append => Ok(ValueType::Bytes),
            }
        }
        ExpressionKind::Cut { text, start, end } => {
            let _ = atom_type(text)?;
            let _ = atom_type(start)?;
            let _ = atom_type(end)?;
            Ok(ValueType::Text)
        }
        ExpressionKind::Slice { bytes, start, end } => {
            let _ = atom_type(bytes)?;
            let _ = atom_type(start)?;
            let _ = atom_type(end)?;
            Ok(ValueType::Bytes)
        }
        ExpressionKind::Call { weave: called, .. } => {
            weave_results.get(called).copied().ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    "internal compiler could not resolve a called weave",
                )
            })
        }
    }
}

fn emit_block(
    statements: &[Statement],
    layout: &BTreeMap<String, SlotInfo>,
    weave_indices: &BTreeMap<String, usize>,
    code: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    for statement in statements {
        match statement {
            Statement::Bind { name, value, .. } => {
                emit_expression(value, layout, weave_indices, code)?;
                code.push(OP_STORE);
                write_u16(
                    code,
                    layout
                        .get(name)
                        .ok_or_else(|| {
                            CompilerError::new(
                                statement.span(),
                                "internal compiler could not resolve bind slot",
                            )
                        })?
                        .index,
                );
            }
            Statement::Revise { name, value, .. } => {
                emit_expression(value, layout, weave_indices, code)?;
                code.push(OP_REVISE);
                write_u16(
                    code,
                    layout
                        .get(name)
                        .ok_or_else(|| {
                            CompilerError::new(
                                statement.span(),
                                "internal compiler could not resolve revise slot",
                            )
                        })?
                        .index,
                );
            }
            Statement::Speak { value, .. } => {
                emit_expression(value, layout, weave_indices, code)?;
                code.push(OP_SPEAK);
            }
            Statement::Yield { value, .. } => {
                emit_expression(value, layout, weave_indices, code)?;
                code.push(OP_YIELD);
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                emit_expression(condition, layout, weave_indices, code)?;
                code.push(OP_JUMP_IF_DIM);
                let dim_target = reserve_u32(code);
                emit_block(when_bright, layout, weave_indices, code)?;
                if when_dim.is_empty() {
                    let continuation = code.len();
                    patch_u32(code, dim_target, continuation)?;
                } else {
                    code.push(OP_JUMP);
                    let end_target = reserve_u32(code);
                    let dim_branch = code.len();
                    patch_u32(code, dim_target, dim_branch)?;
                    emit_block(when_dim, layout, weave_indices, code)?;
                    let continuation = code.len();
                    patch_u32(code, end_target, continuation)?;
                }
            }
            Statement::While {
                condition, body, ..
            } => {
                let loop_start = code.len();
                emit_expression(condition, layout, weave_indices, code)?;
                code.push(OP_JUMP_IF_DIM);
                let loop_end = reserve_u32(code);
                emit_block(body, layout, weave_indices, code)?;
                code.push(OP_JUMP);
                write_u32(
                    code,
                    u32::try_from(loop_start).map_err(|_| {
                        CompilerError::new(
                            statement.span(),
                            "loop start is outside the AETH jump range",
                        )
                    })?,
                );
                let continuation = code.len();
                patch_u32(code, loop_end, continuation)?;
            }
        }
    }
    Ok(())
}

fn emit_expression(
    expression: &Expression,
    layout: &BTreeMap<String, SlotInfo>,
    weave_indices: &BTreeMap<String, usize>,
    code: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => emit_atom(atom, layout, code)?,
        ExpressionKind::Unary {
            operation,
            argument,
        } => {
            emit_atom(argument, layout, code)?;
            code.push(match operation {
                UnaryOperation::Not => OP_NOT,
                UnaryOperation::Measure => OP_MEASURE,
                UnaryOperation::Render => OP_RENDER,
                UnaryOperation::Extent => OP_EXTENT,
                UnaryOperation::Encode => OP_ENCODE,
                UnaryOperation::Decode => OP_DECODE,
            });
        }
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            emit_atom(left, layout, code)?;
            emit_atom(right, layout, code)?;
            code.push(match operation {
                BinaryOperation::Sum => OP_SUM,
                BinaryOperation::Difference => OP_DIFFERENCE,
                BinaryOperation::Product => OP_PRODUCT,
                BinaryOperation::Less => OP_LESS,
                BinaryOperation::Same => OP_SAME,
                BinaryOperation::Join => OP_JOIN,
                BinaryOperation::Glyph => OP_GLYPH,
                BinaryOperation::Quotient => OP_QUOTIENT,
                BinaryOperation::Remainder => OP_REMAINDER,
                BinaryOperation::Fuse => OP_FUSE,
                BinaryOperation::Append => OP_APPEND,
                BinaryOperation::Octet => OP_OCTET,
            });
        }
        ExpressionKind::Cut { text, start, end } => {
            emit_atom(text, layout, code)?;
            emit_atom(start, layout, code)?;
            emit_atom(end, layout, code)?;
            code.push(OP_CUT);
        }
        ExpressionKind::Slice { bytes, start, end } => {
            emit_atom(bytes, layout, code)?;
            emit_atom(start, layout, code)?;
            emit_atom(end, layout, code)?;
            code.push(OP_SLICE);
        }
        ExpressionKind::Call { weave, arguments } => {
            for argument in arguments {
                emit_atom(argument, layout, code)?;
            }
            let function = weave_indices.get(weave).ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    format!("internal compiler could not resolve weave {weave}"),
                )
            })?;
            code.push(OP_CALL);
            write_u16(
                code,
                u16::try_from(*function).map_err(|_| {
                    CompilerError::new(expression.span, "weave index is outside the AETH range")
                })?,
            );
            code.push(u8::try_from(arguments.len()).map_err(|_| {
                CompilerError::new(expression.span, "call has too many AETH arguments")
            })?);
        }
    }
    Ok(())
}

fn emit_atom(
    atom: &Atom,
    layout: &BTreeMap<String, SlotInfo>,
    code: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    match &atom.kind {
        AtomKind::Text(value) => {
            let length = u32::try_from(value.len()).map_err(|_| {
                CompilerError::new(
                    atom.span,
                    "Aether text literals cannot exceed the AETH 32-bit length limit",
                )
            })?;
            code.push(OP_PUSH_TEXT);
            write_u32(code, length);
            code.extend_from_slice(value.as_bytes());
        }
        AtomKind::Bytes(value) => {
            if value.len() > MAX_BYTES {
                return Err(CompilerError::new(
                    atom.span,
                    "bytes literal exceeds the Aether safety limit",
                ));
            }
            let length = u32::try_from(value.len()).map_err(|_| {
                CompilerError::new(
                    atom.span,
                    "Aether bytes literals cannot exceed the AETH 32-bit length limit",
                )
            })?;
            code.push(OP_PUSH_BYTES);
            write_u32(code, length);
            code.extend_from_slice(value);
        }
        AtomKind::Whole(value) => {
            code.push(OP_PUSH_WHOLE);
            code.extend_from_slice(&value.to_le_bytes());
        }
        AtomKind::Truth(value) => {
            code.push(OP_PUSH_TRUTH);
            code.push(u8::from(*value));
        }
        AtomKind::Name(name) | AtomKind::Borrow(name) => {
            let slot = layout.get(name).ok_or_else(|| {
                CompilerError::new(
                    atom.span,
                    format!("internal compiler could not resolve value {name}"),
                )
            })?;
            code.push(OP_LOAD);
            write_u16(code, slot.index);
        }
        AtomKind::Move(name) => {
            let slot = layout.get(name).ok_or_else(|| {
                CompilerError::new(
                    atom.span,
                    format!("internal compiler could not resolve value {name}"),
                )
            })?;
            code.push(OP_MOVE);
            write_u16(code, slot.index);
        }
    }
    Ok(())
}

fn parse_artifact(bytecode: &[u8]) -> Result<Artifact, BytecodeError> {
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
    let mut position = ARTIFACT_MAGIC.len() + 1;
    let function_count = usize::from(read_u16(bytecode, &mut position)?);
    if function_count == 0 || function_count > MAX_FUNCTIONS {
        return Err(BytecodeError::new(
            position,
            "artifact function count is outside the Aether limit",
        ));
    }
    let mut functions = Vec::with_capacity(function_count);
    for _ in 0..function_count {
        let name_length = usize::from(read_byte(bytecode, &mut position)?);
        let name = read_ascii(bytecode, &mut position, name_length, "weave name")?;
        let parameter_count = usize::from(read_byte(bytecode, &mut position)?);
        let mut parameters = Vec::with_capacity(parameter_count);
        for _ in 0..parameter_count {
            let value_type = ValueType::from_byte(read_byte(bytecode, &mut position)?, position)?;
            let mode = ParameterMode::from_byte(read_byte(bytecode, &mut position)?, position)?;
            parameters.push((value_type, mode));
        }
        let result = ValueType::from_byte(read_byte(bytecode, &mut position)?, position)?;
        let local_count = usize::from(read_u16(bytecode, &mut position)?);
        if local_count > MAX_LOCALS {
            return Err(BytecodeError::new(
                position,
                "artifact local count exceeds the Aether limit",
            ));
        }
        let mut locals = Vec::with_capacity(local_count);
        for _ in 0..local_count {
            let value_type = ValueType::from_byte(read_byte(bytecode, &mut position)?, position)?;
            let mutable = match read_byte(bytecode, &mut position)? {
                0 => false,
                1 => true,
                _ => {
                    return Err(BytecodeError::new(
                        position,
                        "artifact local mutability is invalid",
                    ));
                }
            };
            locals.push(LocalDescriptor {
                value_type,
                mutable,
            });
        }
        let code_length = usize::try_from(read_u32(bytecode, &mut position)?).map_err(|_| {
            BytecodeError::new(position, "artifact code length is outside platform limits")
        })?;
        let end = position
            .checked_add(code_length)
            .ok_or_else(|| BytecodeError::new(position, "artifact code length overflowed"))?;
        let Some(code) = bytecode.get(position..end) else {
            return Err(BytecodeError::new(position, "artifact code is truncated"));
        };
        position = end;
        functions.push(ArtifactFunction {
            name,
            parameters,
            result,
            locals,
            code: code.to_vec(),
        });
    }
    if position != bytecode.len() {
        return Err(BytecodeError::new(
            position,
            "artifact has trailing bytes after its weave table",
        ));
    }
    Ok(Artifact { functions })
}

fn verify_function(
    function_index: usize,
    function: &ArtifactFunction,
    functions: &[ArtifactFunction],
) -> Result<(), BytecodeError> {
    let decoded = decode_code(&function.code)?;
    if decoded.is_empty() {
        return Err(BytecodeError::new(0, "weave contains no instructions"));
    }
    let mut instruction_indices = BTreeMap::new();
    for (index, instruction) in decoded.iter().enumerate() {
        if instruction_indices
            .insert(instruction.offset, index)
            .is_some()
        {
            return Err(BytecodeError::new(
                instruction.offset,
                "weave instruction offset is duplicated",
            ));
        }
    }
    for instruction in &decoded {
        for target in jump_targets(&instruction.instruction) {
            if !instruction_indices.contains_key(&target) {
                return Err(BytecodeError::new(
                    instruction.offset,
                    "jump target does not begin an Aether instruction",
                ));
            }
        }
    }

    let mut states = BTreeMap::new();
    let initial = VerificationState {
        stack: Vec::new(),
        initialized: function
            .locals
            .iter()
            .enumerate()
            .map(|(index, _)| index < function.parameters.len())
            .collect(),
        moved: vec![false; function.locals.len()],
    };
    states.insert(0_usize, initial);
    let mut queue = VecDeque::from([0_usize]);
    let mut yielded = false;

    while let Some(offset) = queue.pop_front() {
        let state = states
            .get(&offset)
            .cloned()
            .ok_or_else(|| BytecodeError::new(offset, "verifier lost control-flow state"))?;
        let instruction = decoded[*instruction_indices
            .get(&offset)
            .ok_or_else(|| BytecodeError::new(offset, "instruction offset is invalid"))?]
        .clone();
        let successors = verify_instruction(
            function_index,
            function,
            functions,
            &instruction,
            state,
            &mut yielded,
        )?;
        for (target, next_state) in successors {
            merge_verifier_state(
                target,
                next_state,
                &mut states,
                &mut queue,
                &instruction_indices,
            )?;
        }
    }
    if !yielded {
        return Err(BytecodeError::new(0, "weave has no reachable yield"));
    }
    if states.len() != decoded.len() {
        return Err(BytecodeError::new(
            0,
            "weave contains unreachable Aether instructions",
        ));
    }
    Ok(())
}

fn verify_instruction(
    function_index: usize,
    function: &ArtifactFunction,
    functions: &[ArtifactFunction],
    decoded: &DecodedInstruction,
    mut state: VerificationState,
    yielded: &mut bool,
) -> Result<Vec<(usize, VerificationState)>, BytecodeError> {
    let offset = decoded.offset;
    let next = decoded.next_offset;
    let continue_with = |state: VerificationState| Ok(vec![(next, state)]);
    match &decoded.instruction {
        Instruction::PushText(value) => {
            if value.len() > MAX_TEXT_BYTES {
                return Err(BytecodeError::new(
                    offset,
                    "text constant exceeds the Aether limit",
                ));
            }
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::PushBytes(value) => {
            if value.len() > MAX_BYTES {
                return Err(BytecodeError::new(
                    offset,
                    "bytes constant exceeds the Aether limit",
                ));
            }
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::PushWhole(_) => {
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::PushTruth(_) => {
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Store(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            if state.initialized[*slot] {
                return Err(BytecodeError::new(
                    offset,
                    "store may only initialize an unbound local slot",
                ));
            }
            pop_type(&mut state.stack, local.value_type, offset, "store")?;
            state.initialized[*slot] = true;
            state.moved[*slot] = false;
            continue_with(state)
        }
        Instruction::Load(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            ensure_readable(&state, *slot, offset, "load")?;
            state.stack.push(local.value_type);
            continue_with(state)
        }
        Instruction::Move(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            ensure_readable(&state, *slot, offset, "move")?;
            state.moved[*slot] = true;
            state.stack.push(local.value_type);
            continue_with(state)
        }
        Instruction::Revise(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            if !local.mutable {
                return Err(BytecodeError::new(
                    offset,
                    "revise targets an immutable local slot",
                ));
            }
            ensure_readable(&state, *slot, offset, "revise")?;
            pop_type(&mut state.stack, local.value_type, offset, "revise")?;
            continue_with(state)
        }
        Instruction::Speak => {
            pop_type(&mut state.stack, ValueType::Text, offset, "speak")?;
            continue_with(state)
        }
        Instruction::Yield => {
            pop_type(&mut state.stack, function.result, offset, "yield")?;
            if !state.stack.is_empty() {
                return Err(BytecodeError::new(
                    offset,
                    "yield must leave an empty operand stack",
                ));
            }
            *yielded = true;
            Ok(Vec::new())
        }
        Instruction::Sum
        | Instruction::Difference
        | Instruction::Product
        | Instruction::Quotient
        | Instruction::Remainder => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "arithmetic")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "arithmetic")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Less => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "less")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "less")?;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Same => {
            let right = pop_any_type(&mut state.stack, offset, "same")?;
            let left = pop_any_type(&mut state.stack, offset, "same")?;
            if left != right {
                return Err(BytecodeError::new(
                    offset,
                    "same requires two values with the same type",
                ));
            }
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Not => {
            pop_type(&mut state.stack, ValueType::Truth, offset, "not")?;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Join => {
            pop_type(&mut state.stack, ValueType::Text, offset, "join")?;
            pop_type(&mut state.stack, ValueType::Text, offset, "join")?;
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::Fuse => {
            pop_type(&mut state.stack, ValueType::Bytes, offset, "fuse")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "fuse")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Append => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "append")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "append")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Measure => {
            pop_type(&mut state.stack, ValueType::Text, offset, "measure")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Extent => {
            pop_type(&mut state.stack, ValueType::Bytes, offset, "extent")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Glyph => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "glyph")?;
            pop_type(&mut state.stack, ValueType::Text, offset, "glyph")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Cut => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "cut")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "cut")?;
            pop_type(&mut state.stack, ValueType::Text, offset, "cut")?;
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::Octet => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "octet")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "octet")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Slice => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "slice")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "slice")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "slice")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Encode => {
            pop_type(&mut state.stack, ValueType::Text, offset, "encode")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Decode => {
            pop_type(&mut state.stack, ValueType::Bytes, offset, "decode")?;
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::Render => {
            let value_type = pop_any_type(&mut state.stack, offset, "render")?;
            if value_type == ValueType::Bytes {
                return Err(BytecodeError::new(
                    offset,
                    "render does not accept Bytes; inspect bytes with extent, octet, or decode",
                ));
            }
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::Call {
            function: called,
            arguments,
        } => {
            let Some(called_function) = functions.get(*called) else {
                return Err(BytecodeError::new(
                    offset,
                    "call references an unknown weave",
                ));
            };
            if called_function.parameters.len() != *arguments {
                return Err(BytecodeError::new(
                    offset,
                    "call argument count disagrees with its weave signature",
                ));
            }
            for (expected, _) in called_function.parameters.iter().rev() {
                pop_type(&mut state.stack, *expected, offset, "call")?;
            }
            state.stack.push(called_function.result);
            let _ = function_index;
            continue_with(state)
        }
        Instruction::JumpIfDim(target) => {
            pop_type(
                &mut state.stack,
                ValueType::Truth,
                offset,
                "conditional jump",
            )?;
            Ok(vec![(next, state.clone()), (*target, state)])
        }
        Instruction::Jump(target) => Ok(vec![(*target, state)]),
    }
}

fn merge_verifier_state(
    target: usize,
    next: VerificationState,
    states: &mut BTreeMap<usize, VerificationState>,
    queue: &mut VecDeque<usize>,
    instruction_indices: &BTreeMap<usize, usize>,
) -> Result<(), BytecodeError> {
    if !instruction_indices.contains_key(&target) {
        return Err(BytecodeError::new(
            target,
            "control flow reaches the end of a weave without yield",
        ));
    }
    match states.get(&target) {
        Some(existing) if existing == &next => Ok(()),
        Some(_) => Err(BytecodeError::new(
            target,
            "control-flow paths disagree about stack or move state",
        )),
        None => {
            states.insert(target, next);
            queue.push_back(target);
            Ok(())
        }
    }
}

fn jump_targets(instruction: &Instruction) -> Vec<usize> {
    match instruction {
        Instruction::JumpIfDim(target) | Instruction::Jump(target) => vec![*target],
        _ => Vec::new(),
    }
}

fn execute_function(
    artifact: &Artifact,
    function_index: usize,
    arguments: Vec<RuntimeValue>,
    stdout: &mut String,
    depth: usize,
) -> Result<RuntimeValue, BytecodeError> {
    if depth >= MAX_CALL_DEPTH {
        return Err(BytecodeError::new(
            0,
            "Aether call depth exceeded the deterministic safety limit",
        ));
    }
    let function = artifact
        .functions
        .get(function_index)
        .ok_or_else(|| BytecodeError::new(0, "runtime call references an unknown weave"))?;
    if function.parameters.len() != arguments.len() {
        return Err(BytecodeError::new(
            0,
            "runtime call argument count is invalid",
        ));
    }
    let mut locals = vec![None; function.locals.len()];
    for (index, argument) in arguments.into_iter().enumerate() {
        if argument.value_type() != function.parameters[index].0 {
            return Err(BytecodeError::new(
                0,
                "runtime call argument type is invalid",
            ));
        }
        locals[index] = Some(argument);
    }
    let mut stack = Vec::new();
    let mut position = 0;
    while position < function.code.len() {
        let decoded = decode_instruction(&function.code, &mut position)?;
        match decoded.instruction {
            Instruction::PushText(value) => stack.push(RuntimeValue::Text(value)),
            Instruction::PushBytes(value) => stack.push(RuntimeValue::Bytes(value)),
            Instruction::PushWhole(value) => stack.push(RuntimeValue::Whole(value)),
            Instruction::PushTruth(value) => stack.push(RuntimeValue::Truth(value)),
            Instruction::Store(slot) => {
                if slot >= locals.len() || locals[slot].is_some() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime store slot is invalid",
                    ));
                }
                let value = pop_runtime(&mut stack, decoded.offset, "store")?;
                require_runtime_type(
                    &value,
                    function.locals[slot].value_type,
                    decoded.offset,
                    "store",
                )?;
                locals[slot] = Some(value);
            }
            Instruction::Load(slot) => {
                let value = read_local(&locals, slot, decoded.offset, "load")?;
                stack.push(value.clone());
            }
            Instruction::Move(slot) => {
                let value = locals
                    .get_mut(slot)
                    .ok_or_else(|| {
                        BytecodeError::new(decoded.offset, "runtime move slot is invalid")
                    })?
                    .take()
                    .ok_or_else(|| {
                        BytecodeError::new(decoded.offset, "runtime move reads an empty slot")
                    })?;
                stack.push(value);
            }
            Instruction::Revise(slot) => {
                if slot >= locals.len() || !function.locals[slot].mutable || locals[slot].is_none()
                {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime revise slot is invalid",
                    ));
                }
                let value = pop_runtime(&mut stack, decoded.offset, "revise")?;
                require_runtime_type(
                    &value,
                    function.locals[slot].value_type,
                    decoded.offset,
                    "revise",
                )?;
                locals[slot] = Some(value);
            }
            Instruction::Speak => match pop_runtime(&mut stack, decoded.offset, "speak")? {
                RuntimeValue::Text(value) => {
                    ensure_text_limit(stdout.len(), value.len(), decoded.offset)?;
                    stdout.push_str(&value);
                }
                _ => {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "speak received a non-Text value",
                    ))
                }
            },
            Instruction::Yield => {
                let value = pop_runtime(&mut stack, decoded.offset, "yield")?;
                require_runtime_type(&value, function.result, decoded.offset, "yield")?;
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "yield left values on the runtime stack",
                    ));
                }
                return Ok(value);
            }
            Instruction::Sum => {
                let right = pop_whole(&mut stack, decoded.offset, "sum")?;
                let left = pop_whole(&mut stack, decoded.offset, "sum")?;
                stack.push(RuntimeValue::Whole(left.checked_add(right).ok_or_else(
                    || BytecodeError::new(decoded.offset, "sum overflowed Whole"),
                )?));
            }
            Instruction::Difference => {
                let right = pop_whole(&mut stack, decoded.offset, "difference")?;
                let left = pop_whole(&mut stack, decoded.offset, "difference")?;
                stack.push(RuntimeValue::Whole(left.checked_sub(right).ok_or_else(
                    || BytecodeError::new(decoded.offset, "difference overflowed Whole"),
                )?));
            }
            Instruction::Product => {
                let right = pop_whole(&mut stack, decoded.offset, "product")?;
                let left = pop_whole(&mut stack, decoded.offset, "product")?;
                stack.push(RuntimeValue::Whole(left.checked_mul(right).ok_or_else(
                    || BytecodeError::new(decoded.offset, "product overflowed Whole"),
                )?));
            }
            Instruction::Quotient => {
                let right = pop_whole(&mut stack, decoded.offset, "quotient")?;
                let left = pop_whole(&mut stack, decoded.offset, "quotient")?;
                if right == 0 {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "quotient cannot divide by zero",
                    ));
                }
                stack.push(RuntimeValue::Whole(left.checked_div(right).ok_or_else(
                    || BytecodeError::new(decoded.offset, "quotient overflowed Whole"),
                )?));
            }
            Instruction::Remainder => {
                let right = pop_whole(&mut stack, decoded.offset, "remainder")?;
                let left = pop_whole(&mut stack, decoded.offset, "remainder")?;
                if right == 0 {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "remainder cannot divide by zero",
                    ));
                }
                stack.push(RuntimeValue::Whole(left.checked_rem(right).ok_or_else(
                    || BytecodeError::new(decoded.offset, "remainder overflowed Whole"),
                )?));
            }
            Instruction::Less => {
                let right = pop_whole(&mut stack, decoded.offset, "less")?;
                let left = pop_whole(&mut stack, decoded.offset, "less")?;
                stack.push(RuntimeValue::Truth(left < right));
            }
            Instruction::Same => {
                let right = pop_runtime(&mut stack, decoded.offset, "same")?;
                let left = pop_runtime(&mut stack, decoded.offset, "same")?;
                if left.value_type() != right.value_type() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "same received different value types",
                    ));
                }
                stack.push(RuntimeValue::Truth(runtime_values_equal(&left, &right)));
            }
            Instruction::Not => {
                let value = pop_truth(&mut stack, decoded.offset, "not")?;
                stack.push(RuntimeValue::Truth(!value));
            }
            Instruction::Join => {
                let right = pop_text(&mut stack, decoded.offset, "join")?;
                let left = pop_text(&mut stack, decoded.offset, "join")?;
                ensure_text_limit(left.len(), right.len(), decoded.offset)?;
                stack.push(RuntimeValue::Text(left + &right));
            }
            Instruction::Fuse => {
                let right = pop_bytes(&mut stack, decoded.offset, "fuse")?;
                let mut left = pop_bytes(&mut stack, decoded.offset, "fuse")?;
                ensure_bytes_limit(left.len(), right.len(), decoded.offset)?;
                left.extend_from_slice(&right);
                stack.push(RuntimeValue::Bytes(left));
            }
            Instruction::Append => {
                let octet = pop_whole(&mut stack, decoded.offset, "append")?;
                let mut bytes = pop_bytes(&mut stack, decoded.offset, "append")?;
                let octet = u8::try_from(octet).map_err(|_| {
                    BytecodeError::new(decoded.offset, "append requires a Whole between 0 and 255")
                })?;
                ensure_bytes_limit(bytes.len(), 1, decoded.offset)?;
                bytes.push(octet);
                stack.push(RuntimeValue::Bytes(bytes));
            }
            Instruction::Measure => {
                let text = pop_text(&mut stack, decoded.offset, "measure")?;
                stack.push(RuntimeValue::Whole(
                    i64::try_from(text.chars().count()).map_err(|_| {
                        BytecodeError::new(decoded.offset, "text length is outside Whole range")
                    })?,
                ));
            }
            Instruction::Extent => {
                let bytes = pop_bytes(&mut stack, decoded.offset, "extent")?;
                stack.push(RuntimeValue::Whole(i64::try_from(bytes.len()).map_err(
                    |_| BytecodeError::new(decoded.offset, "bytes length is outside Whole range"),
                )?));
            }
            Instruction::Glyph => {
                let index = pop_whole(&mut stack, decoded.offset, "glyph")?;
                let text = pop_text(&mut stack, decoded.offset, "glyph")?;
                let value = usize::try_from(index)
                    .ok()
                    .and_then(|index| text.chars().nth(index))
                    .map_or(-1_i64, |character| i64::from(u32::from(character)));
                stack.push(RuntimeValue::Whole(value));
            }
            Instruction::Cut => {
                let end = pop_whole(&mut stack, decoded.offset, "cut")?;
                let start = pop_whole(&mut stack, decoded.offset, "cut")?;
                let text = pop_text(&mut stack, decoded.offset, "cut")?;
                let length = i64::try_from(text.chars().count()).map_err(|_| {
                    BytecodeError::new(decoded.offset, "text length is outside Whole range")
                })?;
                let start = start.clamp(0, length);
                let end = end.clamp(start, length);
                let start = usize::try_from(start)
                    .map_err(|_| BytecodeError::new(decoded.offset, "cut start is invalid"))?;
                let end = usize::try_from(end)
                    .map_err(|_| BytecodeError::new(decoded.offset, "cut end is invalid"))?;
                let start = scalar_byte_offset(&text, start);
                let end = scalar_byte_offset(&text, end);
                let slice = text
                    .get(start..end)
                    .ok_or_else(|| BytecodeError::new(decoded.offset, "cut range is invalid"))?;
                stack.push(RuntimeValue::Text(slice.to_owned()));
            }
            Instruction::Octet => {
                let index = pop_whole(&mut stack, decoded.offset, "octet")?;
                let bytes = pop_bytes(&mut stack, decoded.offset, "octet")?;
                let value = usize::try_from(index)
                    .ok()
                    .and_then(|index| bytes.get(index))
                    .map_or(-1_i64, |byte| i64::from(*byte));
                stack.push(RuntimeValue::Whole(value));
            }
            Instruction::Slice => {
                let end = pop_whole(&mut stack, decoded.offset, "slice")?;
                let start = pop_whole(&mut stack, decoded.offset, "slice")?;
                let bytes = pop_bytes(&mut stack, decoded.offset, "slice")?;
                let length = i64::try_from(bytes.len()).map_err(|_| {
                    BytecodeError::new(decoded.offset, "bytes length is outside Whole range")
                })?;
                let start = start.clamp(0, length);
                let end = end.clamp(start, length);
                let start = usize::try_from(start)
                    .map_err(|_| BytecodeError::new(decoded.offset, "slice start is invalid"))?;
                let end = usize::try_from(end)
                    .map_err(|_| BytecodeError::new(decoded.offset, "slice end is invalid"))?;
                stack.push(RuntimeValue::Bytes(bytes[start..end].to_vec()));
            }
            Instruction::Encode => {
                let text = pop_text(&mut stack, decoded.offset, "encode")?;
                ensure_bytes_limit(0, text.len(), decoded.offset)?;
                stack.push(RuntimeValue::Bytes(text.into_bytes()));
            }
            Instruction::Decode => {
                let bytes = pop_bytes(&mut stack, decoded.offset, "decode")?;
                let text = String::from_utf8(bytes).map_err(|_| {
                    BytecodeError::new(decoded.offset, "decode received invalid UTF-8")
                })?;
                ensure_text_limit(0, text.len(), decoded.offset)?;
                stack.push(RuntimeValue::Text(text));
            }
            Instruction::Render => {
                let value = pop_runtime(&mut stack, decoded.offset, "render")?;
                let text = match value {
                    RuntimeValue::Text(value) => value,
                    RuntimeValue::Whole(value) => value.to_string(),
                    RuntimeValue::Truth(true) => "bright".to_owned(),
                    RuntimeValue::Truth(false) => "dim".to_owned(),
                    RuntimeValue::Bytes(_) => return Err(BytecodeError::new(
                        decoded.offset,
                        "render does not accept Bytes; inspect bytes with extent, octet, or decode",
                    )),
                };
                stack.push(RuntimeValue::Text(text));
            }
            Instruction::Call {
                function: called,
                arguments,
            } => {
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "runtime call references an unknown weave")
                })?;
                if called_function.parameters.len() != arguments {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime call has an invalid argument count",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, _) in called_function.parameters.iter().rev() {
                    let value = pop_runtime(&mut stack, decoded.offset, "call")?;
                    require_runtime_type(&value, *value_type, decoded.offset, "call")?;
                    values.push(value);
                }
                values.reverse();
                stack.push(execute_function(
                    artifact,
                    called,
                    values,
                    stdout,
                    depth + 1,
                )?);
            }
            Instruction::JumpIfDim(target) => {
                if !pop_truth(&mut stack, decoded.offset, "conditional jump")? {
                    position = target;
                }
            }
            Instruction::Jump(target) => position = target,
        }
    }
    Err(BytecodeError::new(
        function.code.len(),
        "runtime reached the end of a weave without yield",
    ))
}

fn decode_code(code: &[u8]) -> Result<Vec<DecodedInstruction>, BytecodeError> {
    let mut position = 0;
    let mut instructions = Vec::new();
    while position < code.len() {
        instructions.push(decode_instruction(code, &mut position)?);
    }
    Ok(instructions)
}

fn decode_instruction(
    code: &[u8],
    position: &mut usize,
) -> Result<DecodedInstruction, BytecodeError> {
    let offset = *position;
    let opcode = read_byte(code, position)?;
    let instruction = match opcode {
        OP_PUSH_TEXT => {
            let length = usize::try_from(read_u32(code, position)?).map_err(|_| {
                BytecodeError::new(*position, "text constant length is outside platform limits")
            })?;
            Instruction::PushText(read_utf8(code, position, length, "text constant")?)
        }
        OP_PUSH_BYTES => {
            let length = usize::try_from(read_u32(code, position)?).map_err(|_| {
                BytecodeError::new(
                    *position,
                    "bytes constant length is outside platform limits",
                )
            })?;
            Instruction::PushBytes(read_raw_bytes(code, position, length, "bytes constant")?)
        }
        OP_PUSH_WHOLE => Instruction::PushWhole(read_i64(code, position)?),
        OP_PUSH_TRUTH => match read_byte(code, position)? {
            0 => Instruction::PushTruth(false),
            1 => Instruction::PushTruth(true),
            _ => return Err(BytecodeError::new(offset, "truth constant is invalid")),
        },
        OP_STORE => Instruction::Store(usize::from(read_u16(code, position)?)),
        OP_LOAD => Instruction::Load(usize::from(read_u16(code, position)?)),
        OP_MOVE => Instruction::Move(usize::from(read_u16(code, position)?)),
        OP_REVISE => Instruction::Revise(usize::from(read_u16(code, position)?)),
        OP_SPEAK => Instruction::Speak,
        OP_YIELD => Instruction::Yield,
        OP_SUM => Instruction::Sum,
        OP_DIFFERENCE => Instruction::Difference,
        OP_PRODUCT => Instruction::Product,
        OP_LESS => Instruction::Less,
        OP_SAME => Instruction::Same,
        OP_NOT => Instruction::Not,
        OP_JOIN => Instruction::Join,
        OP_MEASURE => Instruction::Measure,
        OP_GLYPH => Instruction::Glyph,
        OP_CUT => Instruction::Cut,
        OP_RENDER => Instruction::Render,
        OP_QUOTIENT => Instruction::Quotient,
        OP_REMAINDER => Instruction::Remainder,
        OP_FUSE => Instruction::Fuse,
        OP_APPEND => Instruction::Append,
        OP_EXTENT => Instruction::Extent,
        OP_OCTET => Instruction::Octet,
        OP_SLICE => Instruction::Slice,
        OP_ENCODE => Instruction::Encode,
        OP_DECODE => Instruction::Decode,
        OP_CALL => Instruction::Call {
            function: usize::from(read_u16(code, position)?),
            arguments: usize::from(read_byte(code, position)?),
        },
        OP_JUMP_IF_DIM => Instruction::JumpIfDim(read_usize_u32(code, position)?),
        OP_JUMP => Instruction::Jump(read_usize_u32(code, position)?),
        _ => return Err(BytecodeError::new(offset, "unknown Aether opcode")),
    };
    Ok(DecodedInstruction {
        offset,
        next_offset: *position,
        instruction,
    })
}

fn local_descriptor(
    function: &ArtifactFunction,
    slot: usize,
    offset: usize,
) -> Result<&LocalDescriptor, BytecodeError> {
    function
        .locals
        .get(slot)
        .ok_or_else(|| BytecodeError::new(offset, "local slot is outside the local table"))
}

fn ensure_readable(
    state: &VerificationState,
    slot: usize,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if !state.initialized[slot] {
        return Err(BytecodeError::new(
            offset,
            format!("{operation} reads an uninitialized local slot"),
        ));
    }
    if state.moved[slot] {
        return Err(BytecodeError::new(
            offset,
            format!("{operation} reads a moved local slot"),
        ));
    }
    Ok(())
}

fn pop_type(
    stack: &mut Vec<ValueType>,
    expected: ValueType,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    let actual = pop_any_type(stack, offset, operation)?;
    if actual == expected {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires {expected}, but artifact stack has {actual}"),
        ))
    }
}

fn pop_any_type(
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

fn read_local<'a>(
    locals: &'a [Option<RuntimeValue>],
    slot: usize,
    offset: usize,
    operation: &str,
) -> Result<&'a RuntimeValue, BytecodeError> {
    locals
        .get(slot)
        .ok_or_else(|| BytecodeError::new(offset, format!("runtime {operation} slot is invalid")))?
        .as_ref()
        .ok_or_else(|| {
            BytecodeError::new(offset, format!("runtime {operation} reads an empty slot"))
        })
}

fn pop_runtime(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<RuntimeValue, BytecodeError> {
    stack.pop().ok_or_else(|| {
        BytecodeError::new(
            offset,
            format!("{operation} would underflow the runtime stack"),
        )
    })
}

fn require_runtime_type(
    value: &RuntimeValue,
    expected: ValueType,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if value.value_type() == expected {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!(
                "{operation} received {}, expected {expected}",
                value.value_type()
            ),
        ))
    }
}

fn pop_whole(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<i64, BytecodeError> {
    match pop_runtime(stack, offset, operation)? {
        RuntimeValue::Whole(value) => Ok(value),
        value => Err(BytecodeError::new(
            offset,
            format!(
                "{operation} received {}, expected Whole",
                value.value_type()
            ),
        )),
    }
}

fn pop_truth(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<bool, BytecodeError> {
    match pop_runtime(stack, offset, operation)? {
        RuntimeValue::Truth(value) => Ok(value),
        value => Err(BytecodeError::new(
            offset,
            format!(
                "{operation} received {}, expected Truth",
                value.value_type()
            ),
        )),
    }
}

fn pop_text(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<String, BytecodeError> {
    match pop_runtime(stack, offset, operation)? {
        RuntimeValue::Text(value) => Ok(value),
        value => Err(BytecodeError::new(
            offset,
            format!("{operation} received {}, expected Text", value.value_type()),
        )),
    }
}

fn pop_bytes(
    stack: &mut Vec<RuntimeValue>,
    offset: usize,
    operation: &str,
) -> Result<Vec<u8>, BytecodeError> {
    match pop_runtime(stack, offset, operation)? {
        RuntimeValue::Bytes(value) => Ok(value),
        value => Err(BytecodeError::new(
            offset,
            format!(
                "{operation} received {}, expected Bytes",
                value.value_type()
            ),
        )),
    }
}

fn runtime_values_equal(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match (left, right) {
        (RuntimeValue::Text(left), RuntimeValue::Text(right)) => left == right,
        (RuntimeValue::Whole(left), RuntimeValue::Whole(right)) => left == right,
        (RuntimeValue::Truth(left), RuntimeValue::Truth(right)) => left == right,
        (RuntimeValue::Bytes(left), RuntimeValue::Bytes(right)) => left == right,
        _ => false,
    }
}

fn ensure_text_limit(left: usize, right: usize, offset: usize) -> Result<(), BytecodeError> {
    let length = left
        .checked_add(right)
        .ok_or_else(|| BytecodeError::new(offset, "text size overflowed"))?;
    if length > MAX_TEXT_BYTES {
        return Err(BytecodeError::new(
            offset,
            "text operation exceeds the Aether runtime safety limit",
        ));
    }
    Ok(())
}

fn ensure_bytes_limit(left: usize, right: usize, offset: usize) -> Result<(), BytecodeError> {
    let length = left
        .checked_add(right)
        .ok_or_else(|| BytecodeError::new(offset, "bytes size overflowed"))?;
    if length > MAX_BYTES {
        return Err(BytecodeError::new(
            offset,
            "bytes operation exceeds the Aether runtime safety limit",
        ));
    }
    Ok(())
}

fn read_byte(bytes: &[u8], position: &mut usize) -> Result<u8, BytecodeError> {
    let offset = *position;
    let Some(value) = bytes.get(offset) else {
        return Err(BytecodeError::new(offset, "artifact ended unexpectedly"));
    };
    *position += 1;
    Ok(*value)
}

fn read_u16(bytes: &[u8], position: &mut usize) -> Result<u16, BytecodeError> {
    let low = read_byte(bytes, position)?;
    let high = read_byte(bytes, position)?;
    Ok(u16::from_le_bytes([low, high]))
}

fn read_u32(bytes: &[u8], position: &mut usize) -> Result<u32, BytecodeError> {
    let mut buffer = [0_u8; 4];
    for byte in &mut buffer {
        *byte = read_byte(bytes, position)?;
    }
    Ok(u32::from_le_bytes(buffer))
}

fn read_usize_u32(bytes: &[u8], position: &mut usize) -> Result<usize, BytecodeError> {
    usize::try_from(read_u32(bytes, position)?)
        .map_err(|_| BytecodeError::new(*position, "jump target is outside platform limits"))
}

fn read_i64(bytes: &[u8], position: &mut usize) -> Result<i64, BytecodeError> {
    let mut buffer = [0_u8; 8];
    for byte in &mut buffer {
        *byte = read_byte(bytes, position)?;
    }
    Ok(i64::from_le_bytes(buffer))
}

fn read_ascii(
    bytes: &[u8],
    position: &mut usize,
    length: usize,
    subject: &str,
) -> Result<String, BytecodeError> {
    let offset = *position;
    let end = offset
        .checked_add(length)
        .ok_or_else(|| BytecodeError::new(offset, format!("{subject} length overflowed")))?;
    let Some(slice) = bytes.get(offset..end) else {
        return Err(BytecodeError::new(
            offset,
            format!("{subject} is truncated"),
        ));
    };
    if !slice.is_ascii() {
        return Err(BytecodeError::new(
            offset,
            format!("{subject} is not ASCII"),
        ));
    }
    let value = std::str::from_utf8(slice)
        .map_err(|_| BytecodeError::new(offset, format!("{subject} is not UTF-8")))?;
    *position = end;
    Ok(value.to_owned())
}

fn read_utf8(
    bytes: &[u8],
    position: &mut usize,
    length: usize,
    subject: &str,
) -> Result<String, BytecodeError> {
    if length > MAX_TEXT_BYTES {
        return Err(BytecodeError::new(
            *position,
            format!("{subject} exceeds the Aether text safety limit"),
        ));
    }
    let offset = *position;
    let end = offset
        .checked_add(length)
        .ok_or_else(|| BytecodeError::new(offset, format!("{subject} length overflowed")))?;
    let Some(slice) = bytes.get(offset..end) else {
        return Err(BytecodeError::new(
            offset,
            format!("{subject} is truncated"),
        ));
    };
    let value = std::str::from_utf8(slice)
        .map_err(|_| BytecodeError::new(offset, format!("{subject} is not valid UTF-8")))?;
    *position = end;
    Ok(value.to_owned())
}

fn read_raw_bytes(
    bytes: &[u8],
    position: &mut usize,
    length: usize,
    subject: &str,
) -> Result<Vec<u8>, BytecodeError> {
    if length > MAX_BYTES {
        return Err(BytecodeError::new(
            *position,
            format!("{subject} exceeds the Aether bytes safety limit"),
        ));
    }
    let offset = *position;
    let end = offset
        .checked_add(length)
        .ok_or_else(|| BytecodeError::new(offset, format!("{subject} length overflowed")))?;
    let Some(slice) = bytes.get(offset..end) else {
        return Err(BytecodeError::new(
            offset,
            format!("{subject} is truncated"),
        ));
    };
    *position = end;
    Ok(slice.to_vec())
}

fn scalar_byte_offset(text: &str, scalar_index: usize) -> usize {
    text.char_indices()
        .nth(scalar_index)
        .map_or(text.len(), |(offset, _)| offset)
}

fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn reserve_u32(bytes: &mut Vec<u8>) -> usize {
    let offset = bytes.len();
    bytes.extend_from_slice(&[0_u8; 4]);
    offset
}

fn patch_u32(bytes: &mut [u8], offset: usize, target: usize) -> Result<(), CompilerError> {
    let target = u32::try_from(target)
        .map_err(|_| CompilerError::new(Span::synthetic(), "jump target exceeds AETH limits"))?;
    let end = offset
        .checked_add(4)
        .ok_or_else(|| CompilerError::new(Span::synthetic(), "jump patch offset overflowed"))?;
    let Some(destination) = bytes.get_mut(offset..end) else {
        return Err(CompilerError::new(
            Span::synthetic(),
            "internal compiler jump patch is outside emitted code",
        ));
    };
    destination.copy_from_slice(&target.to_le_bytes());
    Ok(())
}

fn write_block(statements: &[Statement], indentation: usize, output: &mut String) {
    for statement in statements {
        output.push_str(&"  ".repeat(indentation));
        match statement {
            Statement::Bind {
                name,
                mutable,
                value,
                ..
            } => {
                output.push_str("bind ");
                if *mutable {
                    output.push_str("mutable ");
                }
                output.push_str(name);
                output.push_str(" <- ");
                write_expression(value, output);
                output.push('\n');
            }
            Statement::Revise { name, value, .. } => {
                output.push_str("revise ");
                output.push_str(name);
                output.push_str(" <- ");
                write_expression(value, output);
                output.push('\n');
            }
            Statement::Speak { value, .. } => {
                output.push_str("speak ");
                write_expression(value, output);
                output.push('\n');
            }
            Statement::Yield { value, .. } => {
                output.push_str("yield ");
                write_expression(value, output);
                output.push('\n');
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                output.push_str("choose ");
                write_expression(condition, output);
                output.push_str(":\n");
                write_block(when_bright, indentation + 1, output);
                if !when_dim.is_empty() {
                    output.push_str(&"  ".repeat(indentation));
                    output.push_str("otherwise:\n");
                    write_block(when_dim, indentation + 1, output);
                }
            }
            Statement::While {
                condition, body, ..
            } => {
                output.push_str("while ");
                write_expression(condition, output);
                output.push_str(":\n");
                write_block(body, indentation + 1, output);
            }
        }
    }
}

fn write_expression(expression: &Expression, output: &mut String) {
    match &expression.kind {
        ExpressionKind::Atom(atom) => write_atom(atom, output),
        ExpressionKind::Unary {
            operation,
            argument,
        } => {
            output.push_str(operation.word());
            output.push(' ');
            write_atom(argument, output);
        }
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            output.push_str(operation.word());
            output.push(' ');
            write_atom(left, output);
            output.push(' ');
            write_atom(right, output);
        }
        ExpressionKind::Cut { text, start, end } => {
            output.push_str("cut ");
            write_atom(text, output);
            output.push(' ');
            write_atom(start, output);
            output.push(' ');
            write_atom(end, output);
        }
        ExpressionKind::Slice { bytes, start, end } => {
            output.push_str("slice ");
            write_atom(bytes, output);
            output.push(' ');
            write_atom(start, output);
            output.push(' ');
            write_atom(end, output);
        }
        ExpressionKind::Call { weave, arguments } => {
            output.push_str("call ");
            output.push_str(weave);
            for argument in arguments {
                output.push(' ');
                write_atom(argument, output);
            }
        }
    }
}

fn write_atom(atom: &Atom, output: &mut String) {
    match &atom.kind {
        AtomKind::Text(value) => {
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
        AtomKind::Bytes(value) => {
            output.push_str("bytes \"");
            write_hex_bytes(value, output);
            output.push('"');
        }
        AtomKind::Whole(value) => output.push_str(&value.to_string()),
        AtomKind::Truth(true) => output.push_str("bright"),
        AtomKind::Truth(false) => output.push_str("dim"),
        AtomKind::Name(name) => output.push_str(name),
        AtomKind::Borrow(name) => {
            output.push_str("borrow ");
            output.push_str(name);
        }
        AtomKind::Move(name) => {
            output.push_str("move ");
            output.push_str(name);
        }
    }
}

fn write_hex_bytes(bytes: &[u8], output: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0F)]));
    }
}

fn write_ast_block(statements: &[Statement], output: &mut String) {
    output.push('{');
    for (index, statement) in statements.iter().enumerate() {
        if index > 0 {
            output.push(';');
        }
        match statement {
            Statement::Bind {
                name,
                mutable,
                value,
                ..
            } => {
                output.push_str(if *mutable { "BindMutable(" } else { "Bind(" });
                output.push_str(name);
                output.push(',');
                write_ast_expression(value, output);
                output.push(')');
            }
            Statement::Revise { name, value, .. } => {
                output.push_str("Revise(");
                output.push_str(name);
                output.push(',');
                write_ast_expression(value, output);
                output.push(')');
            }
            Statement::Speak { value, .. } => {
                output.push_str("Speak(");
                write_ast_expression(value, output);
                output.push(')');
            }
            Statement::Yield { value, .. } => {
                output.push_str("Yield(");
                write_ast_expression(value, output);
                output.push(')');
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                output.push_str("Choose(");
                write_ast_expression(condition, output);
                write_ast_block(when_bright, output);
                write_ast_block(when_dim, output);
                output.push(')');
            }
            Statement::While {
                condition, body, ..
            } => {
                output.push_str("While(");
                write_ast_expression(condition, output);
                write_ast_block(body, output);
                output.push(')');
            }
        }
    }
    output.push('}');
}

fn write_ast_expression(expression: &Expression, output: &mut String) {
    match &expression.kind {
        ExpressionKind::Atom(atom) => write_ast_atom(atom, output),
        ExpressionKind::Unary {
            operation,
            argument,
        } => {
            output.push_str(operation.word());
            output.push('(');
            write_ast_atom(argument, output);
            output.push(')');
        }
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            output.push_str(operation.word());
            output.push('(');
            write_ast_atom(left, output);
            output.push(',');
            write_ast_atom(right, output);
            output.push(')');
        }
        ExpressionKind::Cut { text, start, end } => {
            output.push_str("cut(");
            write_ast_atom(text, output);
            output.push(',');
            write_ast_atom(start, output);
            output.push(',');
            write_ast_atom(end, output);
            output.push(')');
        }
        ExpressionKind::Slice { bytes, start, end } => {
            output.push_str("slice(");
            write_ast_atom(bytes, output);
            output.push(',');
            write_ast_atom(start, output);
            output.push(',');
            write_ast_atom(end, output);
            output.push(')');
        }
        ExpressionKind::Call { weave, arguments } => {
            output.push_str("call(");
            output.push_str(weave);
            for argument in arguments {
                output.push(',');
                write_ast_atom(argument, output);
            }
            output.push(')');
        }
    }
}

fn write_ast_atom(atom: &Atom, output: &mut String) {
    match &atom.kind {
        AtomKind::Text(value) => {
            output.push_str("Text(");
            output.push_str(&value.escape_default().to_string());
            output.push(')');
        }
        AtomKind::Bytes(value) => {
            output.push_str("Bytes(");
            write_hex_bytes(value, output);
            output.push(')');
        }
        AtomKind::Whole(value) => {
            output.push_str("Whole(");
            output.push_str(&value.to_string());
            output.push(')');
        }
        AtomKind::Truth(value) => {
            output.push_str(if *value {
                "Truth(bright)"
            } else {
                "Truth(dim)"
            });
        }
        AtomKind::Name(name) => {
            output.push_str("Name(");
            output.push_str(name);
            output.push(')');
        }
        AtomKind::Borrow(name) => {
            output.push_str("Borrow(");
            output.push_str(name);
            output.push(')');
        }
        AtomKind::Move(name) => {
            output.push_str("Move(");
            output.push_str(name);
            output.push(')');
        }
    }
}

fn statement_token_count(statements: &[Statement]) -> usize {
    statements
        .iter()
        .map(|statement| match statement {
            Statement::Bind { .. }
            | Statement::Revise { .. }
            | Statement::Speak { .. }
            | Statement::Yield { .. } => 2,
            Statement::Choose {
                when_bright,
                when_dim,
                ..
            } => 2 + statement_token_count(when_bright) + statement_token_count(when_dim),
            Statement::While { body, .. } => 2 + statement_token_count(body),
        })
        .sum()
}

fn validate_name(
    name: &str,
    span: Span,
    subject: &str,
    allow_main: bool,
) -> Result<(), CompilerError> {
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
    let reserved = matches!(
        name,
        "world"
            | "weave"
            | "bind"
            | "mutable"
            | "revise"
            | "speak"
            | "yield"
            | "choose"
            | "otherwise"
            | "while"
            | "call"
            | "borrow"
            | "move"
            | "sum"
            | "difference"
            | "product"
            | "less"
            | "same"
            | "not"
            | "join"
            | "measure"
            | "glyph"
            | "cut"
            | "slice"
            | "render"
            | "extent"
            | "encode"
            | "decode"
            | "quotient"
            | "remainder"
            | "fuse"
            | "append"
            | "octet"
            | "bright"
            | "dim"
            | "text"
            | "whole"
            | "truth"
            | "bytes"
    ) || (!allow_main && name == "main");
    if reserved {
        return Err(CompilerError::new(
            span,
            format!("{subject} uses reserved Aether word {name}"),
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

    const HELLO: &str = "world genesis\n\nweave main [] -> Whole:\n  bind greeting <- \"Hello from Aether\\n\"\n  speak borrow greeting\n  yield 0\n";

    #[test]
    fn compiles_runs_and_formats_stage_one_source() {
        let output = compile_to_bytecode(HELLO).expect("Aether source should compile");
        let run = run_bytecode(&output.bytecode).expect("Aether artifact should run");
        assert_eq!(run.stdout, "Hello from Aether\n");
        assert_eq!(run.exit_code, 0);
        assert_eq!(format_program(&output.program), HELLO);
        assert!(canonical_ast(&output.program).contains("Borrow(greeting)"));
        assert_eq!(&output.bytecode[..5], b"AETH\x03");
    }

    #[test]
    fn runs_control_flow_mutation_and_text_primitives() {
        let source = "world loops\n\nweave main [] -> Whole:\n  bind mutable count <- 0\n  bind mutable piece <- \"\"\n  bind mutable output <- \"\"\n  while less count 3:\n    revise piece <- render count\n    revise output <- join borrow output borrow piece\n    revise count <- sum count 1\n  choose same borrow output \"012\":\n    speak borrow output\n  otherwise:\n    speak \"broken\"\n  yield count\n";
        let output = compile_to_bytecode(source).expect("control-flow source should compile");
        let run = run_bytecode(&output.bytecode).expect("control-flow artifact should run");
        assert_eq!(run.stdout, "012");
        assert_eq!(run.exit_code, 3);
    }

    #[test]
    fn runs_named_weaves_with_owned_and_borrowed_text() {
        let source = "world calls\n\nweave echo [borrow value: Text] -> Text:\n  yield borrow value\n\nweave main [] -> Whole:\n  bind seed <- \"Aether\"\n  bind echoed <- call echo borrow seed\n  speak borrow echoed\n  yield 0\n";
        let output = compile_to_bytecode(source).expect("call source should compile");
        let run = run_bytecode(&output.bytecode).expect("call artifact should run");
        assert_eq!(run.stdout, "Aether");
        assert_eq!(run.exit_code, 0);
    }

    #[test]
    fn runs_unicode_text_primitives_on_scalar_boundaries() {
        let source = "world unicode\n\nweave main [] -> Whole:\n  bind source <- \"Aé🙂Z\"\n  bind section <- cut borrow source 1 3\n  bind count <- measure borrow source\n  bind code <- glyph borrow source 2\n  bind mutable result <- -1\n  choose same count 4:\n    revise result <- code\n  speak borrow section\n  yield result\n";
        let output = compile_to_bytecode(source).expect("Unicode source should compile");
        let run = run_bytecode(&output.bytecode).expect("Unicode artifact should run");
        assert_eq!(run.stdout, "é🙂");
        assert_eq!(run.exit_code, 128_578);

        let mut malformed = output.bytecode;
        let text_byte = malformed
            .windows(2)
            .position(|window| window == [0xC3, 0xA9])
            .expect("artifact should contain the UTF-8 text literal");
        malformed[text_byte] = 0xFF;
        let error = verify_bytecode(&malformed)
            .expect_err("invalid UTF-8 artifacts must fail verification");
        assert!(error.message.contains("valid UTF-8"));
    }

    #[test]
    fn supports_text_literals_larger_than_the_legacy_u16_limit() {
        let payload = "x".repeat(70_000);
        let source = format!(
            "world wide\n\nweave main [] -> Whole:\n  bind payload <- \"{payload}\"\n  bind length <- measure borrow payload\n  yield length\n"
        );
        let output = compile_to_bytecode(&source).expect("large bounded text should compile");
        let run = run_bytecode(&output.bytecode).expect("large bounded text should run");
        assert_eq!(run.exit_code, 70_000);
    }

    #[test]
    fn runs_bounded_bytes_primitives_and_preserves_canonical_source() {
        let source = "world binary\n\nweave package [borrow source: Text] -> Bytes:\n  bind encoded <- encode borrow source\n  bind marked <- append move encoded 33\n  bind suffix <- bytes \"ff\"\n  bind payload <- fuse move marked move suffix\n  yield move payload\n\nweave main [] -> Whole:\n  bind payload <- call package \"Aé\"\n  bind length <- extent borrow payload\n  bind first <- octet borrow payload 0\n  bind section <- slice borrow payload 1 4\n  bind recovered <- decode move section\n  speak move recovered\n  bind divided <- quotient length 2\n  bind remainder_value <- remainder length 2\n  bind score <- sum divided remainder_value\n  yield sum score first\n";
        let output = compile_to_bytecode(source).expect("bytes source should compile");
        let run = run_bytecode(&output.bytecode).expect("bytes artifact should run");
        assert_eq!(run.stdout, "é!");
        assert_eq!(run.exit_code, 68);
        assert_eq!(format_program(&output.program), source);
        assert!(canonical_ast(&output.program).contains("Bytes(ff)"));
    }

    #[test]
    fn invokes_a_named_compiler_weave_and_verifies_its_binary_output() {
        let target = compile_to_bytecode(HELLO)
            .expect("target source should compile")
            .bytecode;
        let compiler_source = format!(
            "world forge\n\nweave compile [borrow source: Text] -> Bytes:\n  bind target <- bytes \"{}\"\n  yield move target\n\nweave main [] -> Whole:\n  yield 0\n",
            hex_encode(&target)
        );
        let compiler = compile_to_bytecode(&compiler_source)
            .expect("compiler fixture should compile")
            .bytecode;
        let output = invoke_bytecode(
            &compiler,
            "compile",
            &[InvocationValue::Text("world input\n".to_owned())],
        )
        .expect("compiler weave should accept source and yield bytes");
        assert!(output.stdout.is_empty());
        let InvocationValue::Bytes(generated) = output.value else {
            panic!("compiler weave must yield Bytes");
        };
        assert_eq!(generated, target);
        verify_bytecode(&generated).expect("Aether compiler output must be a verified artifact");
    }

    #[test]
    fn forge_rejects_a_compiler_weave_without_the_required_borrowed_text_abi() {
        let source = "world forge\n\nweave compile [source: Text] -> Bytes:\n  bind artifact <- bytes \"\"\n  yield move artifact\n\nweave main [] -> Whole:\n  yield 0\n";
        let compiler = compile_to_bytecode(source)
            .expect("invalid compiler ABI fixture should compile as general Aether")
            .bytecode;
        let error = forge_bytecode(&compiler, "world supplied\n")
            .expect_err("forge must reject an owned Text compiler parameter");
        assert!(error.message.contains("[borrow source: Text] -> Bytes"));
    }

    #[test]
    fn rejects_invalid_binary_literals_and_implicit_bytes_access() {
        let invalid_literal =
            "world invalid\n\nweave main [] -> Whole:\n  bind payload <- bytes \"0\"\n  yield 0\n";
        let error = compile_source(invalid_literal).expect_err("odd hexadecimal Bytes must fail");
        assert!(error.message.contains("even count"));

        let implicit_access = "world invalid\n\nweave main [] -> Whole:\n  bind payload <- bytes \"41\"\n  bind count <- extent payload\n  yield count\n";
        let error = compile_source(implicit_access)
            .expect_err("Bytes must require explicit borrow or move access");
        assert!(error
            .message
            .contains("Bytes value payload requires explicit borrow or move"));
    }

    #[test]
    fn rejects_invalid_utf8_when_bytes_are_decoded() {
        let source = "world invalid\n\nweave main [] -> Whole:\n  bind decoded <- decode bytes \"ff\"\n  speak move decoded\n  yield 0\n";
        let artifact = compile_to_bytecode(source)
            .expect("binary literals may contain invalid UTF-8")
            .bytecode;
        let error = run_bytecode(&artifact).expect_err("decode must reject invalid UTF-8");
        assert!(error.message.contains("invalid UTF-8"));
    }

    #[test]
    fn tracks_moves_across_control_flow() {
        let source = "world moves\n\nweave main [] -> Whole:\n  bind gift <- \"Aether\"\n  choose bright:\n    speak move gift\n  otherwise:\n    speak \"unused\"\n  speak borrow gift\n  yield 0\n";
        let error = compile_source(source).expect_err("a maybe-moved Text must fail");
        assert!(error.message.contains("was moved"));
    }

    #[test]
    fn rejects_text_without_explicit_borrow_or_move() {
        let source = "world moves\n\nweave main [] -> Whole:\n  bind gift <- \"Aether\"\n  speak gift\n  yield 0\n";
        let error = compile_source(source).expect_err("Text must require explicit access mode");
        assert!(error.message.contains("explicit borrow or move"));
    }

    #[test]
    fn rejects_binding_inside_a_block() {
        let source = "world shape\n\nweave main [] -> Whole:\n  choose bright:\n    bind hidden <- 1\n  yield 0\n";
        let error = compile_source(source).expect_err("conditional binds must fail");
        assert!(error.message.contains("only allowed in a weave root"));
    }

    #[test]
    fn rejects_legacy_syntax() {
        let error =
            compile_source("fn main() -> Int { return 0; }").expect_err("legacy source must fail");
        assert!(error.message.contains("world"));
    }

    #[test]
    fn rejects_noncanonical_indentation() {
        let source = "world broken\nweave main [] -> Whole:\n   yield 0\n";
        let error = compile_source(source).expect_err("odd indentation must fail");
        assert!(error.message.contains("two-space"));
    }

    #[test]
    fn bytecode_is_deterministic_and_verifiable() {
        let first = compile_to_bytecode(HELLO).expect("first compilation should work");
        let second = compile_to_bytecode(HELLO).expect("second compilation should work");
        assert_eq!(first.bytecode, second.bytecode);
        verify_bytecode(&first.bytecode).expect("compiler artifact must verify");
    }

    #[test]
    fn verifier_rejects_bad_magic_and_unknown_opcode() {
        let mut artifact = compile_to_bytecode(HELLO)
            .expect("source should compile")
            .bytecode;
        artifact[0] = b'X';
        assert!(verify_bytecode(&artifact).is_err());

        let mut artifact = compile_to_bytecode(HELLO)
            .expect("source should compile")
            .bytecode;
        let last = artifact.len() - 1;
        artifact[last] = 255;
        assert!(verify_bytecode(&artifact).is_err());
    }

    #[test]
    fn accepts_windows_line_endings_but_formats_canonically() {
        let windows_source = HELLO.replace('\n', "\r\n");
        let program = compile_source(&windows_source).expect("CRLF source should compile");
        assert_eq!(format_program(&program), HELLO);
    }

    fn hex_encode(bytes: &[u8]) -> String {
        let mut output = String::with_capacity(bytes.len() * 2);
        write_hex_bytes(bytes, &mut output);
        output
    }
}
