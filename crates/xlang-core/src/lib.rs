//! Aether bootstrap compiler, AETH verifier, VM, and seed-hosted compile path.
//!
//! The Rust core remains the diagnostic bootstrap and the only way to rebuild
//! the checked-in seed compiler artifact. Default program compilation uses that
//! Aether-written seed artifact through the forge ABI (`compile_with_seed`).

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, VecDeque};
use std::fmt;

mod authoring;

pub use authoring::{
    apply_structural_edit, diagnostic_json, structural_document_json, StructuralEditError,
    StructuralEditResult, DIAGNOSTIC_SCHEMA_VERSION, STRUCTURAL_AST_SCHEMA_VERSION,
    STRUCTURAL_EDIT_PROTOCOL_VERSION,
};

pub const LANGUAGE_NAME: &str = "Aether";
pub const LANGUAGE_VERSION: &str = "0.8.0";

/// Checked-in Aether-written seed compiler artifact (AETH v8).
pub const SEED_COMPILER_ARTIFACT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../seed/aether_seed.aeth"
));

const ARTIFACT_MAGIC: &[u8; 4] = b"AETH";
const ARTIFACT_VERSION_V4: u8 = 4;
const ARTIFACT_VERSION_V5: u8 = 5;
const ARTIFACT_VERSION_V6: u8 = 6;
const ARTIFACT_VERSION_V7: u8 = 7;
const ARTIFACT_VERSION_V8: u8 = 8;
const MAX_SOURCE_BYTES: usize = 1_000_000;
const MAX_FUNCTIONS: usize = 256;
const MAX_LOCALS: usize = u16::MAX as usize;
const MAX_RECORDS: usize = 256;
const MAX_RECORD_FIELDS: usize = 64;
const MAX_TEXT_BYTES: usize = 1_000_000;
const MAX_BYTES: usize = 1_000_000;
const MAX_RECORD_BYTES: usize = 1_000_000;
const MAX_CALL_DEPTH: usize = 1_024;
const MAX_ARENA_BYTES: u32 = 1_000_000;
const BUFFER_METADATA_BYTES: usize = 16;
const MAX_COMPTIME_BINDINGS: usize = 1_024;

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
const OP_SEEK: u8 = 34;
const OP_NUMBER: u8 = 35;
const OP_PACK16: u8 = 36;
const OP_PACK32: u8 = 37;
const OP_UNPACK16: u8 = 38;
const OP_UNPACK32: u8 = 39;
const OP_POKE: u8 = 40;
const OP_POKE32: u8 = 41;
const OP_PACK64: u8 = 42;
const OP_MAKE_RECORD: u8 = 43;
const OP_FIELD: u8 = 44;
const OP_ARENA: u8 = 45;
const OP_BUFFER: u8 = 46;
const OP_ACCESS: u8 = 47;
const OP_ALLOCATE: u8 = 48;
const OP_BUFFER_APPEND: u8 = 49;
const OP_BUFFER_AT: u8 = 50;
const OP_COUNT: u8 = 52;
const OP_RAISE: u8 = 53;
const OP_FORWARD_CALL: u8 = 54;
const OP_HANDLE_CALL: u8 = 55;
const OP_COMPTIME_WHOLE: u8 = 56;

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

/// A stable, machine-readable envelope for a compiler or authoring diagnostic.
/// The code identifies a documented category; the message retains the precise
/// human-facing explanation for the current source construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
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

    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            code: diagnostic_code(&self.message),
            span: self.span,
            message: self.message.clone(),
        }
    }
}

fn diagnostic_code(message: &str) -> &'static str {
    let normalized = message.to_ascii_lowercase();
    if normalized.starts_with("source is empty") || normalized.contains("source exceeds") {
        "AE-SOURCE-001"
    } else if normalized.contains("ae-comptime-003") {
        "AE-COMPTIME-003"
    } else if normalized.contains("ae-comptime-002") {
        "AE-COMPTIME-002"
    } else if normalized.contains("ae-comptime-001") || normalized.contains("comptime") {
        "AE-COMPTIME-001"
    } else if normalized.contains("ae-effect-004") {
        "AE-EFFECT-004"
    } else if normalized.contains("ae-effect-003") {
        "AE-EFFECT-003"
    } else if normalized.contains("ae-effect-002") {
        "AE-EFFECT-002"
    } else if normalized.contains("ae-effect-001") || normalized.contains("raises whole") {
        "AE-EFFECT-001"
    } else if normalized.contains("arena")
        || normalized.contains("buffer")
        || normalized.contains("resource outcome")
        || normalized.contains("resource operation")
    {
        "AE-RESOURCE-001"
    } else if normalized.contains("moved")
        || normalized.contains("borrow")
        || normalized.contains("access")
        || normalized.contains("mutable")
        || normalized.contains("revise")
    {
        "AE-OWNERSHIP-001"
    } else if normalized.contains("duplicate")
        || normalized.contains("unknown name")
        || normalized.contains("undefined")
        || normalized.contains("name must")
    {
        "AE-NAME-001"
    } else if normalized.contains("type")
        || normalized.contains("whole")
        || normalized.contains("truth")
        || normalized.contains("text")
        || normalized.contains("bytes")
    {
        "AE-TYPE-001"
    } else if normalized.contains("indent")
        || normalized.contains("line")
        || normalized.contains("expected")
        || normalized.contains("trailing whitespace")
        || normalized.contains("tab")
        || normalized.contains("syntax")
        || normalized.contains("literal")
    {
        "AE-SYNTAX-001"
    } else {
        "AE-SEMANTIC-001"
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
    Record(u16),
    Arena,
    BufferWhole,
    BufferTruth,
    /// An internal verifier stack marker. It is never serializable, bindable,
    /// returnable, or visible in Aether source type syntax.
    AccessArena,
}

impl ValueType {
    fn to_tag(self) -> u8 {
        match self {
            Self::Text => 1,
            Self::Whole => 2,
            Self::Truth => 3,
            Self::Bytes => 4,
            Self::Record(_) => 5,
            Self::Arena => 6,
            Self::BufferWhole => 7,
            Self::BufferTruth => 8,
            Self::AccessArena => 9,
        }
    }

    fn from_primitive_tag(value: u8, offset: usize) -> Result<Self, BytecodeError> {
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
            Self::Record(record) => write!(formatter, "Record#{record}"),
            Self::Arena => formatter.write_str("Arena"),
            Self::BufferWhole => formatter.write_str("BufferWhole"),
            Self::BufferTruth => formatter.write_str("BufferTruth"),
            Self::AccessArena => formatter.write_str("exclusive Arena access"),
        }
    }
}

const fn is_unique_value(value_type: ValueType) -> bool {
    matches!(
        value_type,
        ValueType::Text
            | ValueType::Bytes
            | ValueType::Record(_)
            | ValueType::BufferWhole
            | ValueType::BufferTruth
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterMode {
    Own,
    Borrow,
    Access,
}

/// The bounded, statically declared control behavior of a weave.
///
/// Aether admits only a total weave or the abortive `Error[Whole]` effect.
/// It deliberately has no inferred row, ambient handler, or resumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Total,
    ErrorWhole,
}

impl Effect {
    const fn to_byte(self) -> u8 {
        match self {
            Self::Total => 0,
            Self::ErrorWhole => 1,
        }
    }

    fn from_byte(value: u8, offset: usize) -> Result<Self, BytecodeError> {
        match value {
            0 => Ok(Self::Total),
            1 => Ok(Self::ErrorWhole),
            _ => Err(BytecodeError::new(offset, "unknown Aether effect tag")),
        }
    }
}

impl ParameterMode {
    fn to_byte(self) -> u8 {
        match self {
            Self::Own => 1,
            Self::Borrow => 2,
            Self::Access => 3,
        }
    }

    fn from_byte(value: u8, offset: usize, version: u8) -> Result<Self, BytecodeError> {
        match value {
            1 => Ok(Self::Own),
            2 => Ok(Self::Borrow),
            3 if matches!(
                version,
                ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
            ) =>
            {
                Ok(Self::Access)
            }
            3 => Err(BytecodeError::new(
                offset,
                "access parameters are valid only in AETH v6, v7, or v8 artifacts",
            )),
            _ => Err(BytecodeError::new(offset, "unknown Aether parameter mode")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub world: String,
    pub records: Vec<RecordDeclaration>,
    pub weaves: Vec<Weave>,
}

impl Program {
    #[must_use]
    pub fn significant_token_count(&self) -> usize {
        2 + self
            .records
            .iter()
            .map(|record| 2 + record.fields.len() * 2)
            .sum::<usize>()
            + self
                .weaves
                .iter()
                .map(|weave| 4 + weave.parameters.len() * 2 + statement_token_count(&weave.body))
                .sum::<usize>()
    }
}

/// An immutable nominal aggregate declaration. Its index in [`Program::records`]
/// is the record identifier used by AETH v5 type references and opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordDeclaration {
    pub name: String,
    pub fields: Vec<RecordField>,
    pub span: Span,
}

/// One primitive-valued field in a [`RecordDeclaration`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub name: String,
    pub value_type: ValueType,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weave {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub result: ValueType,
    pub effect: Effect,
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
        comptime: bool,
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
    Raise {
        code: Atom,
        span: Span,
    },
    Forward {
        weave: String,
        arguments: Vec<Atom>,
        span: Span,
    },
    Handle {
        weave: String,
        arguments: Vec<Atom>,
        success_destination: String,
        error_destination: String,
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
            | Self::Raise { span, .. }
            | Self::Forward { span, .. }
            | Self::Handle { span, .. }
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
    /// The one VM-private, named capability declaration permitted per
    /// invocation. The capacity is copied into the AETH v6/v7/v8 resource plan.
    Arena {
        capacity: u32,
    },
    /// An unallocated owner placeholder. A resource `choose` condition moves
    /// it into an allocation attempt and restores either the allocated buffer
    /// or the unchanged placeholder before a branch begins.
    Buffer {
        element: BufferElement,
    },
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
    Ternary {
        operation: TernaryOperation,
        first: Atom,
        second: Atom,
        third: Atom,
    },
    Call {
        weave: String,
        arguments: Vec<Atom>,
    },
    MakeRecord {
        record: String,
        fields: Vec<Atom>,
    },
    Field {
        record: Atom,
        field: String,
    },
    /// A closed resource outcome. It is intentionally valid only as the
    /// condition of a terminal `choose`, where both alternatives are handled
    /// explicitly and the destination owner is restored before either branch.
    Resource(ResourceOperation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferElement {
    Whole,
    Truth,
}

impl BufferElement {
    const fn value_type(self) -> ValueType {
        match self {
            Self::Whole => ValueType::BufferWhole,
            Self::Truth => ValueType::BufferTruth,
        }
    }

    const fn element_type(self) -> ValueType {
        match self {
            Self::Whole => ValueType::Whole,
            Self::Truth => ValueType::Truth,
        }
    }

    const fn type_tag(self) -> u8 {
        match self {
            Self::Whole => 2,
            Self::Truth => 3,
        }
    }

    fn from_type_tag(value: u8, offset: usize) -> Result<Self, BytecodeError> {
        match value {
            2 => Ok(Self::Whole),
            3 => Ok(Self::Truth),
            _ => Err(BytecodeError::new(
                offset,
                "buffer element kind must be Whole or Truth",
            )),
        }
    }

    const fn word(self) -> &'static str {
        match self {
            Self::Whole => "Whole",
            Self::Truth => "Truth",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceOperation {
    Allocate {
        arena: Atom,
        buffer: Atom,
        capacity: Atom,
        destination: String,
    },
    Append {
        buffer: Atom,
        value: Atom,
        destination: String,
    },
    At {
        buffer: Atom,
        index: Atom,
        destination: String,
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
    Number,
    Pack16,
    Pack32,
    Pack64,
    Count,
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
            Self::Number => "number",
            Self::Pack16 => "pack16",
            Self::Pack32 => "pack32",
            Self::Pack64 => "pack64",
            Self::Count => "count",
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
    Unpack16,
    Unpack32,
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
            Self::Unpack16 => "unpack16",
            Self::Unpack32 => "unpack32",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryOperation {
    Seek,
    Poke,
    Poke32,
}

impl TernaryOperation {
    const fn word(self) -> &'static str {
        match self {
            Self::Seek => "seek",
            Self::Poke => "poke",
            Self::Poke32 => "poke32",
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
    Access(String),
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
    resource: ResourceBindingState,
}

type BindingScope = BTreeMap<String, BindingState>;
type ResourceOutcomeScopes = (BindingScope, BindingScope);

/// The immutable context shared by recursive M2 resource-outcome validation.
/// Keeping it together prevents terminal branch validation from drifting away
/// from the enclosing weave's type and result contract.
struct ResourceValidationContext<'a> {
    signatures: &'a BTreeMap<String, FunctionSignature>,
    records: &'a [RecordDeclaration],
    weave: &'a Weave,
}

/// Typed resource facts produced by source validation and consumed directly by
/// AETH v6/v7/v8 lowering. This is deliberately separate from the parsed AST: it
/// records the resolved lexical region, owner place, element type, and stable
/// replacement destination for every closed M2 outcome.
#[derive(Clone)]
struct SemanticResourcePlan {
    arena: Option<SemanticArena>,
    outcomes: BTreeMap<(usize, usize), SemanticResourceOperation>,
}

#[derive(Clone)]
struct SemanticArena {
    name: String,
    capacity: u32,
    span: Span,
}

#[derive(Clone)]
enum SemanticResourceOperation {
    Allocate {
        region: String,
        owner: String,
        destination: String,
        element: BufferElement,
    },
    Append {
        region: String,
        owner: String,
        destination: String,
        element: BufferElement,
    },
    At {
        region: String,
        owner: String,
        destination: String,
        element: BufferElement,
    },
}

impl SemanticResourcePlan {
    const fn empty() -> Self {
        Self {
            arena: None,
            outcomes: BTreeMap::new(),
        }
    }

    const fn arena_capacity(&self) -> u32 {
        match &self.arena {
            Some(arena) => arena.capacity,
            None => 0,
        }
    }

    fn record_outcome(
        &mut self,
        span: Span,
        operation: SemanticResourceOperation,
    ) -> Result<(), CompilerError> {
        if self
            .outcomes
            .insert((span.line, span.column), operation)
            .is_some()
        {
            return Err(CompilerError::new(
                span,
                "internal compiler found two resource operations at one source location",
            ));
        }
        Ok(())
    }

    fn outcome(&self, span: Span) -> Result<&SemanticResourceOperation, CompilerError> {
        self.outcomes.get(&(span.line, span.column)).ok_or_else(|| {
            CompilerError::new(
                span,
                "internal compiler could not resolve the typed resource operation",
            )
        })
    }
}

impl SemanticResourceOperation {
    fn region(&self) -> &str {
        match self {
            Self::Allocate { region, .. }
            | Self::Append { region, .. }
            | Self::At { region, .. } => region,
        }
    }

    fn destination(&self) -> &str {
        match self {
            Self::Allocate { destination, .. }
            | Self::Append { destination, .. }
            | Self::At { destination, .. } => destination,
        }
    }

    const fn element(&self) -> BufferElement {
        match self {
            Self::Allocate { element, .. }
            | Self::Append { element, .. }
            | Self::At { element, .. } => *element,
        }
    }

    fn matches_source(&self, operation: &ResourceOperation) -> bool {
        match (self, operation) {
            (
                Self::Allocate {
                    owner, destination, ..
                },
                ResourceOperation::Allocate {
                    buffer,
                    destination: source_destination,
                    ..
                },
            )
            | (
                Self::Append {
                    owner, destination, ..
                },
                ResourceOperation::Append {
                    buffer,
                    destination: source_destination,
                    ..
                },
            ) => {
                matches!(&buffer.kind, AtomKind::Move(name) if name == owner)
                    && source_destination == destination
            }
            (
                Self::At {
                    owner, destination, ..
                },
                ResourceOperation::At {
                    buffer,
                    destination: source_destination,
                    ..
                },
            ) => {
                matches!(&buffer.kind, AtomKind::Borrow(name) if name == owner)
                    && source_destination == destination
            }
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum ResourceBindingState {
    Plain,
    Arena {
        name: String,
    },
    Buffer {
        element: BufferElement,
        arena: Option<String>,
        allocated: bool,
    },
}

impl ResourceBindingState {
    const fn plain() -> Self {
        Self::Plain
    }
}

#[derive(Clone)]
struct FunctionSignature {
    parameters: Vec<Parameter>,
    result: ValueType,
    effect: Effect,
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
    effect: Effect,
    locals: Vec<LocalDescriptor>,
    code: Vec<u8>,
}

#[derive(Clone)]
struct Artifact {
    version: u8,
    arena_capacity: u32,
    records: Vec<ArtifactRecord>,
    functions: Vec<ArtifactFunction>,
}

#[derive(Clone)]
struct ArtifactRecord {
    name: String,
    fields: Vec<ArtifactRecordField>,
}

#[derive(Clone)]
struct ArtifactRecordField {
    name: String,
    value_type: ValueType,
}

#[derive(Clone, PartialEq, Eq)]
struct VerificationState {
    stack: VerificationStack,
    initialized: Vec<bool>,
    moved: Vec<bool>,
}

/// Immutable verifier inputs shared by every instruction in one weave.
/// Grouping these prevents the instruction checker from gaining a fragile,
/// ever-growing parameter list as the verified instruction set evolves.
struct VerificationContext<'a> {
    function_index: usize,
    function: &'a ArtifactFunction,
    functions: &'a [ArtifactFunction],
    records: &'a [ArtifactRecord],
}

/// The verifier records where a buffer value on the transient operand stack
/// came from.  A raw placeholder, a read loan, a moved local owner, and an
/// owned result from a checked call are not interchangeable.  This prevents a
/// forged v6 instruction stream from substituting a fresh `buffer` placeholder
/// when an operation promises to restore the specific owner it just moved.
#[derive(Clone, PartialEq, Eq)]
struct VerificationStack {
    values: Vec<VerificationStackValue>,
}

#[derive(Clone, PartialEq, Eq)]
struct VerificationStackValue {
    value_type: ValueType,
    buffer_provenance: BufferProvenance,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BufferProvenance {
    NotBuffer,
    Placeholder,
    Borrowed,
    MovedLocal(usize),
}

impl VerificationStack {
    const fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn push(&mut self, value_type: ValueType) {
        self.values.push(VerificationStackValue {
            value_type,
            buffer_provenance: if is_buffer_type(value_type) {
                BufferProvenance::Borrowed
            } else {
                BufferProvenance::NotBuffer
            },
        });
    }

    fn push_buffer_placeholder(&mut self, element: BufferElement) {
        self.values.push(VerificationStackValue {
            value_type: element.value_type(),
            buffer_provenance: BufferProvenance::Placeholder,
        });
    }

    fn push_moved_local(&mut self, value_type: ValueType, slot: usize) {
        self.values.push(VerificationStackValue {
            value_type,
            buffer_provenance: if is_buffer_type(value_type) {
                BufferProvenance::MovedLocal(slot)
            } else {
                BufferProvenance::NotBuffer
            },
        });
    }

    fn pop(&mut self) -> Option<VerificationStackValue> {
        self.values.pop()
    }

    const fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone)]
enum RuntimeValue {
    Text(String),
    Whole(i64),
    Truth(bool),
    Bytes(Vec<u8>),
    Record {
        record: u16,
        fields: Vec<RuntimeValue>,
    },
    Arena,
    AccessArena,
    Buffer {
        element: BufferElement,
        allocated: bool,
        offset: usize,
        len: usize,
        capacity: usize,
    },
}

enum RuntimeExit {
    Return(RuntimeValue),
    ErrorWhole(i64),
}

impl RuntimeValue {
    const fn value_type(&self) -> ValueType {
        match self {
            Self::Text(_) => ValueType::Text,
            Self::Whole(_) => ValueType::Whole,
            Self::Truth(_) => ValueType::Truth,
            Self::Bytes(_) => ValueType::Bytes,
            Self::Record { record, .. } => ValueType::Record(*record),
            Self::Arena => ValueType::Arena,
            Self::AccessArena => ValueType::AccessArena,
            Self::Buffer { element, .. } => element.value_type(),
        }
    }
}

struct RuntimeState {
    arena: ArenaState,
}

struct ArenaState {
    bytes: Vec<u8>,
    used: usize,
}

impl RuntimeState {
    fn new(capacity: u32) -> Result<Self, BytecodeError> {
        let capacity = usize::try_from(capacity)
            .map_err(|_| BytecodeError::new(0, "arena capacity is outside platform limits"))?;
        let mut bytes = Vec::new();
        if capacity > 0 {
            bytes.try_reserve_exact(capacity).map_err(|_| {
                BytecodeError::new(0, "Aether arena admission failed before guest execution")
            })?;
            bytes.resize(capacity, 0);
        }
        Ok(Self {
            arena: ArenaState { bytes, used: 0 },
        })
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

fn invocation_from_runtime(value: RuntimeValue) -> Result<InvocationValue, BytecodeError> {
    match value {
        RuntimeValue::Text(value) => Ok(InvocationValue::Text(value)),
        RuntimeValue::Whole(value) => Ok(InvocationValue::Whole(value)),
        RuntimeValue::Truth(value) => Ok(InvocationValue::Truth(value)),
        RuntimeValue::Bytes(value) => Ok(InvocationValue::Bytes(value)),
        RuntimeValue::Record { .. } => Err(BytecodeError::new(
            0,
            "host invocation cannot return a record; project a primitive field inside Aether",
        )),
        RuntimeValue::Arena | RuntimeValue::AccessArena | RuntimeValue::Buffer { .. } => Err(
            BytecodeError::new(0, "host invocation cannot return an Aether resource value"),
        ),
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    PushText(String),
    PushBytes(Vec<u8>),
    PushWhole(i64),
    ComptimeWhole(i64),
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
    Seek,
    Number,
    Pack16,
    Pack32,
    Unpack16,
    Unpack32,
    Poke,
    Poke32,
    Pack64,
    Render,
    MakeRecord(u16),
    Field {
        record: u16,
        field: usize,
    },
    Arena,
    Buffer(BufferElement),
    Access(usize),
    Allocate {
        element: BufferElement,
        destination: usize,
    },
    BufferAppend {
        element: BufferElement,
        destination: usize,
    },
    BufferAt {
        element: BufferElement,
        destination: usize,
    },
    Count,
    Raise,
    ForwardCall {
        function: usize,
        arguments: usize,
    },
    HandleCall {
        function: usize,
        arguments: usize,
        success_destination: usize,
        error_destination: usize,
        success_target: usize,
        error_target: usize,
    },
    Call {
        function: usize,
        arguments: usize,
    },
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
    let mut records = Vec::new();
    while index < lines.len() && lines[index].content.starts_with("record ") {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "a record declaration must begin at indentation level zero",
            ));
        }
        records.push(parse_record_declaration(line)?);
        index += 1;
    }
    let record_types = record_type_map(&records)?;
    let mut weaves = Vec::new();
    while index < lines.len() {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "a weave declaration must begin at indentation level zero",
            ));
        }
        if line.content.starts_with("record ") {
            return Err(CompilerError::new(
                line.span(1),
                "record declarations must appear after world and before every weave",
            ));
        }
        let (name, parameters, result, effect) = parse_weave_header(line, &record_types)?;
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
            effect,
            body,
            span: line.span(1),
        });
    }

    let program = Program {
        world,
        records,
        weaves,
    };
    validate_program(&program)?;
    Ok(program)
}

pub fn compile_to_bytecode(source: &str) -> Result<CompileOutput, CompilerError> {
    let program = compile_source(source)?;
    let resource_plan = validate_program(&program)?;
    let bytecode = emit_bytecode_with_resource_plan(&program, &resource_plan)?;
    verify_bytecode(&bytecode).map_err(|error| {
        CompilerError::new(
            Span::synthetic(),
            format!("compiler produced an invalid Aether artifact: {error}"),
        )
    })?;
    Ok(CompileOutput { program, bytecode })
}

/// Compile source with the Aether-written seed compiler (default product path).
///
/// Uses [`SEED_COMPILER_ARTIFACT`] through the forge ABI. The returned
/// `program` AST still comes from the Rust bootstrap for tooling/diagnostics;
/// the `bytecode` is seed-produced and must match bootstrap for Seed Profile
/// programs covered by self-host tests.
pub fn compile_with_seed(source: &str) -> Result<CompileOutput, CompilerError> {
    let program = compile_source(source)?;
    let forged = forge_bytecode(SEED_COMPILER_ARTIFACT, source).map_err(|error| {
        CompilerError::new(Span::synthetic(), format!("seed compiler failed: {error}"))
    })?;
    let InvocationValue::Bytes(bytecode) = forged.value else {
        return Err(CompilerError::new(
            Span::synthetic(),
            "seed compiler must yield Bytes",
        ));
    };
    verify_bytecode(&bytecode).map_err(|error| {
        CompilerError::new(
            Span::synthetic(),
            format!("seed compiler produced an invalid Aether artifact: {error}"),
        )
    })?;
    Ok(CompileOutput { program, bytecode })
}

#[must_use]
pub fn format_program(program: &Program) -> String {
    let mut formatted = format!("world {}\n", program.world);
    for record in &program.records {
        formatted.push('\n');
        formatted.push_str("record ");
        formatted.push_str(&record.name);
        formatted.push_str(" [");
        for (index, field) in record.fields.iter().enumerate() {
            if index > 0 {
                formatted.push_str(", ");
            }
            formatted.push_str(&field.name);
            formatted.push_str(": ");
            formatted.push_str(&format_value_type(field.value_type, &program.records));
        }
        formatted.push_str("]\n");
    }
    for weave in &program.weaves {
        formatted.push('\n');
        formatted.push_str("weave ");
        formatted.push_str(&weave.name);
        formatted.push_str(" [");
        for (index, parameter) in weave.parameters.iter().enumerate() {
            if index > 0 {
                formatted.push_str(", ");
            }
            match parameter.mode {
                ParameterMode::Own => {}
                ParameterMode::Borrow => formatted.push_str("borrow "),
                ParameterMode::Access => formatted.push_str("access "),
            }
            formatted.push_str(&parameter.name);
            formatted.push_str(": ");
            formatted.push_str(&format_value_type(parameter.value_type, &program.records));
        }
        formatted.push_str("] -> ");
        formatted.push_str(&format_value_type(weave.result, &program.records));
        if weave.effect == Effect::ErrorWhole {
            formatted.push_str(" raises Whole");
        }
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
    for record in &program.records {
        output.push_str(";Record(");
        output.push_str(&record.name);
        output.push_str(")[");
        for (index, field) in record.fields.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(&field.name);
            output.push(':');
            output.push_str(&format_value_type(field.value_type, &program.records));
        }
        output.push(']');
    }
    for weave in &program.weaves {
        output.push_str(";Weave(");
        output.push_str(&weave.name);
        output.push_str("->");
        output.push_str(&format_value_type(weave.result, &program.records));
        output.push('#');
        output.push_str(match weave.effect {
            Effect::Total => "Total",
            Effect::ErrorWhole => "ErrorWhole",
        });
        output.push_str(")[");
        for (index, parameter) in weave.parameters.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            match parameter.mode {
                ParameterMode::Own => {}
                ParameterMode::Borrow => output.push_str("Borrow "),
                ParameterMode::Access => output.push_str("Access "),
            }
            output.push_str(&parameter.name);
            output.push(':');
            output.push_str(&format_value_type(parameter.value_type, &program.records));
        }
        output.push(']');
        write_ast_block(&weave.body, &mut output);
    }
    output
}

fn format_value_type(value_type: ValueType, records: &[RecordDeclaration]) -> String {
    match value_type {
        ValueType::Record(record_id) => records
            .get(usize::from(record_id))
            .map_or_else(|| value_type.to_string(), |record| record.name.clone()),
        _ => value_type.to_string(),
    }
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
        validate_artifact_resource_signature(function, artifact.version)?;
        validate_artifact_effect_signature(function, artifact.version)?;
    }

    let Some(main_index) = main_index else {
        return Err(BytecodeError::new(0, "artifact has no main weave"));
    };
    let main = &artifact.functions[main_index];
    if !main.parameters.is_empty()
        || main.result != ValueType::Whole
        || main.effect != Effect::Total
    {
        return Err(BytecodeError::new(
            0,
            "main must accept no parameters, remain total, and yield Whole",
        ));
    }

    verify_resource_plan(&artifact, main_index)?;

    for (function_index, function) in artifact.functions.iter().enumerate() {
        verify_function(
            function_index,
            function,
            &artifact.functions,
            &artifact.records,
            artifact.version,
        )?;
    }
    Ok(())
}

fn validate_artifact_resource_signature(
    function: &ArtifactFunction,
    version: u8,
) -> Result<(), BytecodeError> {
    let access_count = function
        .parameters
        .iter()
        .filter(|(_, mode)| *mode == ParameterMode::Access)
        .count();
    let has_buffer = function
        .parameters
        .iter()
        .any(|(value_type, _)| is_buffer_type(*value_type));
    for (value_type, mode) in &function.parameters {
        if *mode == ParameterMode::Access && *value_type != ValueType::Arena {
            return Err(BytecodeError::new(
                0,
                "artifact access parameters must use Arena",
            ));
        }
        if *value_type == ValueType::Arena && *mode != ParameterMode::Access {
            return Err(BytecodeError::new(
                0,
                "artifact Arena parameters must use access",
            ));
        }
        if *value_type == ValueType::AccessArena {
            return Err(BytecodeError::new(
                0,
                "artifact cannot serialize an access loan parameter",
            ));
        }
    }
    if matches!(function.result, ValueType::Arena | ValueType::AccessArena) {
        return Err(BytecodeError::new(
            0,
            "artifact cannot return an Arena or access loan",
        ));
    }
    if is_buffer_type(function.result) {
        return Err(BytecodeError::new(
            0,
            "AETH v6 M2 does not permit Buffer values as weave results",
        ));
    }
    if has_buffer
        && (!matches!(
            version,
            ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
        ) || access_count != 1)
    {
        return Err(BytecodeError::new(
            0,
            "artifact Buffer signatures require exactly one AETH v6/v7/v8 access Arena parameter",
        ));
    }
    if function
        .locals
        .iter()
        .any(|local| local.value_type == ValueType::AccessArena)
    {
        return Err(BytecodeError::new(
            0,
            "artifact cannot serialize an access loan local",
        ));
    }
    Ok(())
}

fn validate_artifact_effect_signature(
    function: &ArtifactFunction,
    version: u8,
) -> Result<(), BytecodeError> {
    if !matches!(version, ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8)
        && function.effect != Effect::Total
    {
        return Err(BytecodeError::new(
            0,
            "pre-v7 artifacts cannot declare an Aether error effect",
        ));
    }
    if function.effect != Effect::ErrorWhole {
        return Ok(());
    }
    if function.result != ValueType::Whole {
        return Err(BytecodeError::new(
            0,
            "AETH Error[Whole] functions must return Whole for the bounded terminal handler form",
        ));
    }
    if function.parameters.iter().any(|(value_type, mode)| {
        *mode != ParameterMode::Own || !matches!(value_type, ValueType::Whole | ValueType::Truth)
    }) {
        return Err(BytecodeError::new(
            0,
            "AETH Error[Whole] functions accept only ordinary Whole or Truth copy parameters",
        ));
    }
    Ok(())
}

fn verify_resource_plan(artifact: &Artifact, main_index: usize) -> Result<(), BytecodeError> {
    let mut arena_declarations = 0_usize;
    let mut resource_instruction_seen = false;
    for (function_index, function) in artifact.functions.iter().enumerate() {
        for instruction in decode_code(&function.code, artifact.version)? {
            match instruction.instruction {
                Instruction::Arena => {
                    if function_index != main_index {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "the AETH v6 arena declaration must occur in main",
                        ));
                    }
                    arena_declarations += 1;
                    resource_instruction_seen = true;
                }
                Instruction::Buffer(_)
                | Instruction::Access(_)
                | Instruction::Allocate { .. }
                | Instruction::BufferAppend { .. }
                | Instruction::BufferAt { .. }
                | Instruction::Count => resource_instruction_seen = true,
                _ => {}
            }
        }
    }
    if !matches!(
        artifact.version,
        ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
    ) {
        if artifact.arena_capacity != 0 || resource_instruction_seen {
            return Err(BytecodeError::new(
                0,
                "pre-v6 artifacts cannot contain a resource plan or resource instructions",
            ));
        }
        return Ok(());
    }
    if artifact.arena_capacity == 0 && resource_instruction_seen {
        return Err(BytecodeError::new(
            0,
            "AETH v6/v7/v8 resource instructions require a nonzero arena resource plan",
        ));
    }
    if artifact.arena_capacity > 0 && arena_declarations != 1 {
        return Err(BytecodeError::new(
            0,
            "AETH v6/v7/v8 nonzero arena plan requires exactly one main arena declaration",
        ));
    }
    if artifact.arena_capacity == 0 && arena_declarations != 0 {
        return Err(BytecodeError::new(
            0,
            "AETH v6/v7/v8 zero arena plan cannot declare an Arena capability",
        ));
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
    if function.effect != Effect::Total {
        return Err(BytecodeError::new(
            0,
            format!(
                "host invocation refuses weave {weave_name} because Error[Whole] must be handled inside Aether"
            ),
        ));
    }
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
    for (index, (argument, (expected, mode))) in
        arguments.iter().zip(&function.parameters).enumerate()
    {
        if matches!(
            expected,
            ValueType::Record(_)
                | ValueType::Arena
                | ValueType::BufferWhole
                | ValueType::BufferTruth
                | ValueType::AccessArena
        ) || *mode == ParameterMode::Access
        {
            return Err(BytecodeError::new(
                0,
                format!(
                    "weave {weave_name} argument {} is a resource or record; host invocation accepts only primitive Aether values",
                    index + 1
                ),
            ));
        }
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
    let mut runtime_state = RuntimeState::new(artifact.arena_capacity)?;
    let exit = execute_function(
        artifact,
        function_index,
        runtime_arguments,
        &mut stdout,
        &mut runtime_state,
        0,
    )?;
    let RuntimeExit::Return(value) = exit else {
        return Err(BytecodeError::new(
            0,
            "a total host invocation reached an unhandled Aether Error[Whole] exit",
        ));
    };
    Ok(InvocationOutput {
        stdout,
        value: invocation_from_runtime(value)?,
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
    record_types: &BTreeMap<String, u16>,
) -> Result<(String, Vec<Parameter>, ValueType, Effect), CompilerError> {
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
    let parameters = parse_parameters(&after_opening[..closing], line, record_types)?;
    let after_parameters = after_opening[closing + 1..].trim();
    let Some(result_text) = after_parameters.strip_prefix("-> ") else {
        return Err(CompilerError::new(
            line.span(1),
            "weave result must use -> Type",
        ));
    };
    let (result_text, effect) = if let Some(result) = result_text.strip_suffix(" raises Whole") {
        (result, Effect::ErrorWhole)
    } else if result_text.contains(" raises ") {
        return Err(CompilerError::new(
            line.span(line.content.len()),
            "Aether M4 supports only the typed effect phrase raises Whole",
        ));
    } else {
        (result_text, Effect::Total)
    };
    let result = parse_value_type(result_text, line.span(line.content.len()), record_types)?;
    Ok((name.to_owned(), parameters, result, effect))
}

fn parse_parameters(
    source: &str,
    line: SourceLine<'_>,
    record_types: &BTreeMap<String, u16>,
) -> Result<Vec<Parameter>, CompilerError> {
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
        } else if let Some(rest) = trimmed.strip_prefix("access ") {
            (ParameterMode::Access, rest)
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
        let value_type = parse_value_type(type_text, line.span(1), record_types)?;
        if mode == ParameterMode::Borrow && !is_unique_value(value_type) {
            return Err(CompilerError::new(
                line.span(1),
                "borrow parameters are reserved for unique Text, Bytes, or record values",
            ));
        }
        if mode == ParameterMode::Access && value_type != ValueType::Arena {
            return Err(CompilerError::new(
                line.span(1),
                "access parameters require the Arena capability type",
            ));
        }
        if value_type == ValueType::Arena && mode != ParameterMode::Access {
            return Err(CompilerError::new(
                line.span(1),
                "Arena parameters must use access; arenas cannot be owned or borrowed",
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

fn parse_value_type(
    source: &str,
    span: Span,
    record_types: &BTreeMap<String, u16>,
) -> Result<ValueType, CompilerError> {
    match source {
        "Text" => Ok(ValueType::Text),
        "Whole" => Ok(ValueType::Whole),
        "Truth" => Ok(ValueType::Truth),
        "Bytes" => Ok(ValueType::Bytes),
        "Arena" => Ok(ValueType::Arena),
        "BufferWhole" => Ok(ValueType::BufferWhole),
        "BufferTruth" => Ok(ValueType::BufferTruth),
        _ => record_types
            .get(source)
            .copied()
            .map(ValueType::Record)
            .ok_or_else(|| {
                CompilerError::new(
                    span,
                    "Aether types are Text, Whole, Truth, Bytes, Arena, BufferWhole, BufferTruth, or a declared record",
                )
            }),
    }
}

fn parse_primitive_value_type(source: &str, span: Span) -> Result<ValueType, CompilerError> {
    match source {
        "Text" => Ok(ValueType::Text),
        "Whole" => Ok(ValueType::Whole),
        "Truth" => Ok(ValueType::Truth),
        "Bytes" => Ok(ValueType::Bytes),
        _ => Err(CompilerError::new(
            span,
            "record fields may use only Text, Whole, Truth, or Bytes",
        )),
    }
}

fn parse_record_declaration(line: SourceLine<'_>) -> Result<RecordDeclaration, CompilerError> {
    let Some(rest) = line.content.strip_prefix("record ") else {
        return Err(CompilerError::new(
            line.span(1),
            "expected record declaration",
        ));
    };
    let Some(opening) = rest.find('[') else {
        return Err(CompilerError::new(
            line.span(1),
            "record fields must be enclosed by square brackets",
        ));
    };
    let name = rest[..opening].trim_end();
    validate_name(name, line.span(8), "record name", false)?;
    let after_opening = &rest[opening + 1..];
    let Some(closing) = after_opening.find(']') else {
        return Err(CompilerError::new(
            line.span(8 + opening + 1),
            "record field list is missing its closing bracket",
        ));
    };
    if !after_opening[closing + 1..].is_empty() {
        return Err(CompilerError::new(
            line.span(8 + opening + closing + 2),
            "record declarations cannot contain text after the closing bracket",
        ));
    }
    let fields_source = &after_opening[..closing];
    if fields_source.trim().is_empty() {
        return Err(CompilerError::new(
            line.span(1),
            "records require at least one field",
        ));
    }

    let mut fields = Vec::new();
    for segment in fields_source.split(',') {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            return Err(CompilerError::new(
                line.span(1),
                "record field lists cannot contain an empty entry",
            ));
        }
        let Some((field_name, field_type)) = trimmed.split_once(": ") else {
            return Err(CompilerError::new(
                line.span(1),
                "each record field must use name: Type",
            ));
        };
        validate_name(field_name, line.span(1), "record field name", false)?;
        if fields
            .iter()
            .any(|field: &RecordField| field.name == field_name)
        {
            return Err(CompilerError::new(
                line.span(1),
                format!("record field {field_name} is declared more than once"),
            ));
        }
        if fields.len() >= MAX_RECORD_FIELDS {
            return Err(CompilerError::new(
                line.span(1),
                format!("records support at most {MAX_RECORD_FIELDS} fields"),
            ));
        }
        fields.push(RecordField {
            name: field_name.to_owned(),
            value_type: parse_primitive_value_type(field_type, line.span(1))?,
            span: line.span(1),
        });
    }
    Ok(RecordDeclaration {
        name: name.to_owned(),
        fields,
        span: line.span(1),
    })
}

fn record_type_map(records: &[RecordDeclaration]) -> Result<BTreeMap<String, u16>, CompilerError> {
    if records.len() > MAX_RECORDS {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!("Aether supports at most {MAX_RECORDS} records per artifact"),
        ));
    }
    let mut record_types = BTreeMap::new();
    for (index, record) in records.iter().enumerate() {
        let identifier = u16::try_from(index).map_err(|_| {
            CompilerError::new(record.span, "record identifier is outside the AETH range")
        })?;
        if record_types
            .insert(record.name.clone(), identifier)
            .is_some()
        {
            return Err(CompilerError::new(
                record.span,
                format!("record {} is declared more than once", record.name),
            ));
        }
    }
    Ok(record_types)
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
    if let Some(rest) = line.content.strip_prefix("comptime bind ") {
        return parse_bind_statement(rest, span, true);
    }
    if let Some(rest) = line.content.strip_prefix("bind ") {
        return parse_bind_statement(rest, span, false);
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
    if let Some(code) = line.content.strip_prefix("raise ") {
        return Ok(Statement::Raise {
            code: parse_single_atom(code, span, "raise")?,
            span,
        });
    }
    if let Some(call_source) = line.content.strip_prefix("forward ") {
        let (weave, arguments) = parse_effect_call(call_source, span, "forward")?;
        return Ok(Statement::Forward {
            weave,
            arguments,
            span,
        });
    }
    if let Some(handle_source) = line.content.strip_prefix("handle ") {
        let (weave, arguments, success_destination, error_destination) =
            parse_handle_header(handle_source, span)?;
        return Ok(Statement::Handle {
            weave,
            arguments,
            success_destination,
            error_destination,
            span,
        });
    }
    Err(CompilerError::new(
        span,
        "unknown Aether statement; use comptime bind, bind, revise, speak, yield, raise, forward, choose, while, or handle",
    ))
}

fn parse_bind_statement(
    source: &str,
    span: Span,
    comptime: bool,
) -> Result<Statement, CompilerError> {
    let (mutable, source) = if let Some(remainder) = source.strip_prefix("mutable ") {
        (true, remainder)
    } else {
        (false, source)
    };
    let Some((name, expression)) = source.split_once(" <- ") else {
        return Err(CompilerError::new(
            span,
            if comptime {
                "AE-COMPTIME-001: comptime bind requires a name, the <- binder, and one literal Whole arithmetic expression"
            } else {
                "bind requires a name, the <- binder, and one value"
            },
        ));
    };
    validate_name(name, span, "binding name", false)?;
    Ok(Statement::Bind {
        name: name.to_owned(),
        mutable,
        comptime,
        value: parse_expression(expression, span)?,
        span,
    })
}

fn parse_single_atom(source: &str, span: Span, operation: &str) -> Result<Atom, CompilerError> {
    let tokens = tokenize_fragment(source, span)?;
    if tokens.len() != 1 {
        return Err(CompilerError::new(
            span,
            format!("{operation} requires exactly one atom"),
        ));
    }
    let mut index = 0;
    parse_atom_from(&tokens, &mut index, span)
}

fn parse_effect_call(
    source: &str,
    span: Span,
    operation: &str,
) -> Result<(String, Vec<Atom>), CompilerError> {
    let expression = parse_expression(source, span)?;
    let ExpressionKind::Call { weave, arguments } = expression.kind else {
        return Err(CompilerError::new(
            span,
            format!("{operation} requires call followed by a weave name"),
        ));
    };
    Ok((weave, arguments))
}

fn parse_handle_header(
    source: &str,
    span: Span,
) -> Result<(String, Vec<Atom>, String, String), CompilerError> {
    let Some((success_source, error_destination)) = source.rsplit_once(" otherwise error into ")
    else {
        return Err(CompilerError::new(
            span,
            "handle requires call weave arguments into a success destination otherwise error into an error destination",
        ));
    };
    let Some((call_source, success_destination)) = success_source.rsplit_once(" into ") else {
        return Err(CompilerError::new(
            span,
            "handle requires call weave arguments into a mutable destination",
        ));
    };
    validate_name(
        success_destination,
        span,
        "handle success destination",
        false,
    )?;
    validate_name(error_destination, span, "handle error destination", false)?;
    let (weave, arguments) = parse_effect_call(call_source, span, "handle")?;
    Ok((
        weave,
        arguments,
        success_destination.to_owned(),
        error_destination.to_owned(),
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
        "arena" if tokens.len() == 2 => {
            let capacity = parse_atom_from(&tokens, &mut index, span)?;
            let AtomKind::Whole(capacity) = capacity.kind else {
                return Err(CompilerError::new(
                    capacity.span,
                    "arena requires one nonnegative Whole literal capacity",
                ));
            };
            let capacity = u32::try_from(capacity).map_err(|_| {
                CompilerError::new(
                    span,
                    format!("arena capacity must be between 0 and {MAX_ARENA_BYTES}"),
                )
            })?;
            if capacity == 0 || capacity > MAX_ARENA_BYTES {
                return Err(CompilerError::new(
                    span,
                    format!("arena capacity must be between 1 and {MAX_ARENA_BYTES}"),
                ));
            }
            ExpressionKind::Arena { capacity }
        }
        "buffer" if tokens.len() == 2 => {
            let Some(element) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "buffer requires the element type Whole or Truth",
                ));
            };
            index += 1;
            ExpressionKind::Buffer {
                element: parse_buffer_element(element, span)?,
            }
        }
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
        "number" => ExpressionKind::Unary {
            operation: UnaryOperation::Number,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "pack16" => ExpressionKind::Unary {
            operation: UnaryOperation::Pack16,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "pack32" => ExpressionKind::Unary {
            operation: UnaryOperation::Pack32,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "pack64" => ExpressionKind::Unary {
            operation: UnaryOperation::Pack64,
            argument: parse_atom_from(&tokens, &mut index, span)?,
        },
        "count" if tokens.len() > 1 => ExpressionKind::Unary {
            operation: UnaryOperation::Count,
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
        "append" => {
            if tokens.iter().any(|token| token == "into") {
                let buffer = parse_atom_from(&tokens, &mut index, span)?;
                let value = parse_atom_from(&tokens, &mut index, span)?;
                let destination = parse_resource_destination(&tokens, &mut index, span)?;
                ExpressionKind::Resource(ResourceOperation::Append {
                    buffer,
                    value,
                    destination,
                })
            } else {
                ExpressionKind::Binary {
                    operation: BinaryOperation::Append,
                    left: parse_atom_from(&tokens, &mut index, span)?,
                    right: parse_atom_from(&tokens, &mut index, span)?,
                }
            }
        }
        "allocate" if tokens.len() >= 6 => {
            let arena = parse_atom_from(&tokens, &mut index, span)?;
            let buffer = parse_atom_from(&tokens, &mut index, span)?;
            let capacity = parse_atom_from(&tokens, &mut index, span)?;
            let destination = parse_resource_destination(&tokens, &mut index, span)?;
            ExpressionKind::Resource(ResourceOperation::Allocate {
                arena,
                buffer,
                capacity,
                destination,
            })
        }
        "at" if tokens.iter().any(|token| token == "into") => {
            let buffer = parse_atom_from(&tokens, &mut index, span)?;
            let index_value = parse_atom_from(&tokens, &mut index, span)?;
            let destination = parse_resource_destination(&tokens, &mut index, span)?;
            ExpressionKind::Resource(ResourceOperation::At {
                buffer,
                index: index_value,
                destination,
            })
        }
        "octet" => ExpressionKind::Binary {
            operation: BinaryOperation::Octet,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "unpack16" => ExpressionKind::Binary {
            operation: BinaryOperation::Unpack16,
            left: parse_atom_from(&tokens, &mut index, span)?,
            right: parse_atom_from(&tokens, &mut index, span)?,
        },
        "unpack32" => ExpressionKind::Binary {
            operation: BinaryOperation::Unpack32,
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
        "seek" => ExpressionKind::Ternary {
            operation: TernaryOperation::Seek,
            first: parse_atom_from(&tokens, &mut index, span)?,
            second: parse_atom_from(&tokens, &mut index, span)?,
            third: parse_atom_from(&tokens, &mut index, span)?,
        },
        "poke" => ExpressionKind::Ternary {
            operation: TernaryOperation::Poke,
            first: parse_atom_from(&tokens, &mut index, span)?,
            second: parse_atom_from(&tokens, &mut index, span)?,
            third: parse_atom_from(&tokens, &mut index, span)?,
        },
        "poke32" => ExpressionKind::Ternary {
            operation: TernaryOperation::Poke32,
            first: parse_atom_from(&tokens, &mut index, span)?,
            second: parse_atom_from(&tokens, &mut index, span)?,
            third: parse_atom_from(&tokens, &mut index, span)?,
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
        "make" => {
            let Some(record) = tokens.get(index) else {
                return Err(CompilerError::new(span, "make requires a record name"));
            };
            validate_name(record, span, "record name", false)?;
            index += 1;
            let mut fields = Vec::new();
            while index < tokens.len() {
                fields.push(parse_atom_from(&tokens, &mut index, span)?);
            }
            ExpressionKind::MakeRecord {
                record: record.clone(),
                fields,
            }
        }
        "field" => {
            let record = parse_atom_from(&tokens, &mut index, span)?;
            if !matches!(record.kind, AtomKind::Borrow(_)) {
                return Err(CompilerError::new(
                    record.span,
                    "field requires borrow followed by a record binding name",
                ));
            }
            let Some(field) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "field requires a record field name",
                ));
            };
            validate_name(field, span, "record field name", false)?;
            index += 1;
            ExpressionKind::Field {
                record,
                field: field.clone(),
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

fn parse_buffer_element(source: &str, span: Span) -> Result<BufferElement, CompilerError> {
    match source {
        "Whole" => Ok(BufferElement::Whole),
        "Truth" => Ok(BufferElement::Truth),
        _ => Err(CompilerError::new(
            span,
            "M2 buffers may contain only Whole or Truth elements",
        )),
    }
}

fn parse_resource_destination(
    tokens: &[String],
    index: &mut usize,
    span: Span,
) -> Result<String, CompilerError> {
    if tokens.get(*index).is_none_or(|token| token != "into") {
        return Err(CompilerError::new(
            span,
            "resource outcomes require `into <mutable-root-binding>`",
        ));
    }
    *index += 1;
    let Some(destination) = tokens.get(*index) else {
        return Err(CompilerError::new(
            span,
            "resource outcomes require a destination binding after into",
        ));
    };
    validate_name(destination, span, "resource destination", false)?;
    *index += 1;
    Ok(destination.clone())
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
    if token == "borrow" || token == "move" || token == "access" {
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
            kind: match mode {
                "borrow" => AtomKind::Borrow(name.clone()),
                "move" => AtomKind::Move(name.clone()),
                "access" => AtomKind::Access(name.clone()),
                _ => unreachable!("accepted access mode must be known"),
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

fn validate_program(program: &Program) -> Result<SemanticResourcePlan, CompilerError> {
    validate_record_declarations(&program.records)?;
    let mut resource_plan = validate_resource_plan(program)?;
    validate_comptime_budget(program)?;
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
                    effect: weave.effect,
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
    if main.effect != Effect::Total {
        return Err(CompilerError::new(
            main.span,
            "AE-EFFECT-001: main must remain total; handle Error[Whole] before the program entry boundary",
        ));
    }
    if !main.parameters.is_empty() || main.result != ValueType::Whole {
        return Err(CompilerError::new(
            main.span,
            "main must use the total declaration weave main [] -> Whole:",
        ));
    }

    for weave in &program.weaves {
        validate_value_type(weave.result, &program.records, weave.span)?;
        validate_resource_signature(weave)?;
        validate_effect_signature(weave)?;
        if weave_uses_resource(weave) && weave_uses_effect_control(weave) {
            return Err(CompilerError::new(
                weave.span,
                "AE-EFFECT-003: Aether M4 error control cannot share a weave with M2 arena, Buffer, access, or resource outcomes",
            ));
        }
        let access_arena = weave
            .parameters
            .iter()
            .find(|parameter| parameter.mode == ParameterMode::Access)
            .map(|parameter| parameter.name.clone());
        let mut scope = BTreeMap::new();
        for parameter in &weave.parameters {
            validate_value_type(parameter.value_type, &program.records, parameter.span)?;
            scope.insert(
                parameter.name.clone(),
                BindingState {
                    value_type: parameter.value_type,
                    mutable: false,
                    moved: false,
                    resource: resource_parameter_state(parameter, access_arena.as_deref()),
                },
            );
        }
        validate_block(
            &weave.body,
            &mut scope,
            &signatures,
            &program.records,
            weave,
            true,
            &mut resource_plan,
        )?;
    }
    Ok(resource_plan)
}

fn validate_comptime_budget(program: &Program) -> Result<(), CompilerError> {
    fn count(statements: &[Statement]) -> usize {
        statements
            .iter()
            .map(|statement| match statement {
                Statement::Bind { comptime, .. } => usize::from(*comptime),
                Statement::Choose {
                    when_bright,
                    when_dim,
                    ..
                } => count(when_bright) + count(when_dim),
                Statement::While { body, .. } => count(body),
                Statement::Revise { .. }
                | Statement::Speak { .. }
                | Statement::Yield { .. }
                | Statement::Raise { .. }
                | Statement::Forward { .. }
                | Statement::Handle { .. } => 0,
            })
            .sum()
    }

    let mut total = 0_usize;
    for weave in &program.weaves {
        total = total.checked_add(count(&weave.body)).ok_or_else(|| {
            CompilerError::new(
                weave.span,
                "AE-COMPTIME-003: compile-time directive count overflowed the fixed evaluation budget",
            )
        })?;
        if total > MAX_COMPTIME_BINDINGS {
            return Err(CompilerError::new(
                weave.span,
                format!(
                    "AE-COMPTIME-003: Aether permits at most {MAX_COMPTIME_BINDINGS} comptime bind directives per source program"
                ),
            ));
        }
    }
    Ok(())
}

fn evaluate_comptime_whole(expression: &Expression) -> Result<i64, CompilerError> {
    let ExpressionKind::Binary {
        operation,
        left,
        right,
    } = &expression.kind
    else {
        return Err(CompilerError::new(
            expression.span,
            "AE-COMPTIME-001: comptime bind requires exactly one literal Whole sum, difference, product, quotient, or remainder expression",
        ));
    };
    let (AtomKind::Whole(left), AtomKind::Whole(right)) = (&left.kind, &right.kind) else {
        return Err(CompilerError::new(
            expression.span,
            "AE-COMPTIME-001: comptime bind accepts only signed Whole literals, never names, owners, or other value shapes",
        ));
    };
    match operation {
        BinaryOperation::Sum => left.checked_add(*right).ok_or_else(|| {
            CompilerError::new(
                expression.span,
                "AE-COMPTIME-002: comptime sum overflowed Whole",
            )
        }),
        BinaryOperation::Difference => left.checked_sub(*right).ok_or_else(|| {
            CompilerError::new(
                expression.span,
                "AE-COMPTIME-002: comptime difference overflowed Whole",
            )
        }),
        BinaryOperation::Product => left.checked_mul(*right).ok_or_else(|| {
            CompilerError::new(
                expression.span,
                "AE-COMPTIME-002: comptime product overflowed Whole",
            )
        }),
        BinaryOperation::Quotient => {
            if *right == 0 {
                return Err(CompilerError::new(
                    expression.span,
                    "AE-COMPTIME-002: comptime quotient cannot divide by zero",
                ));
            }
            left.checked_div(*right).ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    "AE-COMPTIME-002: comptime quotient overflowed Whole",
                )
            })
        }
        BinaryOperation::Remainder => {
            if *right == 0 {
                return Err(CompilerError::new(
                    expression.span,
                    "AE-COMPTIME-002: comptime remainder cannot divide by zero",
                ));
            }
            left.checked_rem(*right).ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    "AE-COMPTIME-002: comptime remainder overflowed Whole",
                )
            })
        }
        _ => Err(CompilerError::new(
            expression.span,
            format!(
                "AE-COMPTIME-001: comptime bind does not admit {} in the bounded M5 evaluator",
                operation.word()
            ),
        )),
    }
}

fn validate_effect_signature(weave: &Weave) -> Result<(), CompilerError> {
    if weave.effect == Effect::Total {
        return Ok(());
    }
    if weave.name == "main" {
        return Err(CompilerError::new(
            weave.span,
            "main must remain total; handle Error[Whole] before the program entry boundary",
        ));
    }
    if weave.result != ValueType::Whole {
        return Err(CompilerError::new(
            weave.span,
            "AE-EFFECT-003: a weave that raises Whole must return Whole so a terminal handler can preserve a total result",
        ));
    }
    for parameter in &weave.parameters {
        if parameter.mode != ParameterMode::Own
            || !matches!(parameter.value_type, ValueType::Whole | ValueType::Truth)
        {
            return Err(CompilerError::new(
                parameter.span,
                "AE-EFFECT-003: a weave that raises Whole accepts only ordinary Whole or Truth copy parameters",
            ));
        }
    }
    Ok(())
}

fn validate_resource_signature(weave: &Weave) -> Result<(), CompilerError> {
    let access_count = weave
        .parameters
        .iter()
        .filter(|parameter| parameter.mode == ParameterMode::Access)
        .count();
    let has_buffer = weave
        .parameters
        .iter()
        .any(|parameter| is_buffer_type(parameter.value_type));

    for parameter in &weave.parameters {
        if parameter.mode == ParameterMode::Access && parameter.value_type != ValueType::Arena {
            return Err(CompilerError::new(
                parameter.span,
                "access parameters require Arena",
            ));
        }
        if parameter.value_type == ValueType::Arena && parameter.mode != ParameterMode::Access {
            return Err(CompilerError::new(
                parameter.span,
                "Arena parameters require access and cannot be moved or borrowed",
            ));
        }
    }
    if weave.result == ValueType::Arena || weave.result == ValueType::AccessArena {
        return Err(CompilerError::new(
            weave.span,
            "Aether M2 does not permit an Arena or access loan as a weave result",
        ));
    }
    if is_buffer_type(weave.result) {
        return Err(CompilerError::new(
            weave.span,
            "M2 Buffer ownership cannot yet cross a weave result; handle its closed outcome in the defining weave",
        ));
    }
    if has_buffer && access_count != 1 {
        return Err(CompilerError::new(
            weave.span,
            "a weave that accepts or returns a Buffer requires exactly one access Arena parameter",
        ));
    }
    Ok(())
}

fn resource_parameter_state(
    parameter: &Parameter,
    access_arena: Option<&str>,
) -> ResourceBindingState {
    match parameter.value_type {
        ValueType::Arena => ResourceBindingState::Arena {
            name: parameter.name.clone(),
        },
        ValueType::BufferWhole => ResourceBindingState::Buffer {
            element: BufferElement::Whole,
            arena: access_arena.map(str::to_owned),
            allocated: true,
        },
        ValueType::BufferTruth => ResourceBindingState::Buffer {
            element: BufferElement::Truth,
            arena: access_arena.map(str::to_owned),
            allocated: true,
        },
        _ => ResourceBindingState::plain(),
    }
}

fn resource_state_for_expression(
    expression: &Expression,
    scope: &BTreeMap<String, BindingState>,
) -> Result<ResourceBindingState, CompilerError> {
    match &expression.kind {
        ExpressionKind::Arena { .. } => Ok(ResourceBindingState::Arena {
            name: String::new(),
        }),
        ExpressionKind::Buffer { element } => Ok(ResourceBindingState::Buffer {
            element: *element,
            arena: None,
            allocated: false,
        }),
        ExpressionKind::Atom(Atom {
            kind: AtomKind::Move(name),
            span,
        }) => scope
            .get(name)
            .map(|binding| binding.resource.clone())
            .ok_or_else(|| {
                CompilerError::new(
                    *span,
                    format!("value {name} has not been bound in this weave"),
                )
            }),
        ExpressionKind::Call { .. } => Ok(ResourceBindingState::plain()),
        _ => Ok(ResourceBindingState::plain()),
    }
}

fn validate_resource_plan(program: &Program) -> Result<SemanticResourcePlan, CompilerError> {
    let mut arenas = Vec::new();
    let mut resource_operations = false;
    for weave in &program.weaves {
        collect_resource_declarations(
            &weave.body,
            &weave.name,
            &mut arenas,
            &mut resource_operations,
        );
    }
    if arenas.len() > 1 {
        return Err(CompilerError::new(
            arenas[1].1.span,
            "Aether M2 permits exactly one arena declaration per invocation",
        ));
    }
    let mut plan = SemanticResourcePlan::empty();
    if let Some((weave, arena)) = arenas.first() {
        if weave != "main" {
            return Err(CompilerError::new(
                arena.span,
                "the M2 arena declaration must be a root binding in weave main",
            ));
        }
        if arena.name.is_empty() {
            return Err(CompilerError::new(
                arena.span,
                "the M2 arena declaration requires a named root binding",
            ));
        }
        plan.arena = Some(arena.clone());
    }
    if resource_operations && arenas.len() != 1 {
        return Err(CompilerError::new(
            Span::synthetic(),
            "M2 resource operations require one named arena declaration in weave main",
        ));
    }
    Ok(plan)
}

fn collect_resource_declarations(
    statements: &[Statement],
    weave: &str,
    arenas: &mut Vec<(String, SemanticArena)>,
    resource_operations: &mut bool,
) {
    for statement in statements {
        match statement {
            Statement::Bind {
                name, value, span, ..
            } => {
                if let ExpressionKind::Arena { capacity } = &value.kind {
                    arenas.push((
                        weave.to_owned(),
                        SemanticArena {
                            name: name.clone(),
                            capacity: *capacity,
                            span: *span,
                        },
                    ));
                }
                *resource_operations |= expression_uses_resource(value);
            }
            Statement::Revise { value, .. }
            | Statement::Speak { value, .. }
            | Statement::Yield { value, .. } => {
                *resource_operations |= expression_uses_resource(value);
            }
            Statement::Raise { .. } | Statement::Forward { .. } => {}
            Statement::Handle { .. } => {}
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                if matches!(condition.kind, ExpressionKind::Resource(_)) {
                    *resource_operations = true;
                }
                collect_resource_declarations(when_bright, weave, arenas, resource_operations);
                collect_resource_declarations(when_dim, weave, arenas, resource_operations);
            }
            Statement::While { body, .. } => {
                collect_resource_declarations(body, weave, arenas, resource_operations);
            }
        }
    }
}

fn expression_uses_resource(expression: &Expression) -> bool {
    matches!(
        &expression.kind,
        ExpressionKind::Arena { .. }
            | ExpressionKind::Buffer { .. }
            | ExpressionKind::Resource(_)
            | ExpressionKind::Unary {
                operation: UnaryOperation::Count,
                ..
            }
    )
}

fn weave_uses_resource(weave: &Weave) -> bool {
    fn block_uses_resource(statements: &[Statement]) -> bool {
        statements.iter().any(|statement| match statement {
            Statement::Bind { value, .. }
            | Statement::Revise { value, .. }
            | Statement::Speak { value, .. }
            | Statement::Yield { value, .. } => expression_uses_resource(value),
            Statement::Raise { .. } | Statement::Forward { .. } => false,
            Statement::Handle { .. } => false,
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                expression_uses_resource(condition)
                    || block_uses_resource(when_bright)
                    || block_uses_resource(when_dim)
            }
            Statement::While {
                condition, body, ..
            } => expression_uses_resource(condition) || block_uses_resource(body),
        })
    }
    block_uses_resource(&weave.body)
}

fn weave_uses_effect_control(weave: &Weave) -> bool {
    fn block_uses_effect_control(statements: &[Statement]) -> bool {
        statements.iter().any(|statement| match statement {
            Statement::Raise { .. } | Statement::Forward { .. } | Statement::Handle { .. } => true,
            Statement::Choose {
                when_bright,
                when_dim,
                ..
            } => block_uses_effect_control(when_bright) || block_uses_effect_control(when_dim),
            Statement::While { body, .. } => block_uses_effect_control(body),
            Statement::Bind { .. }
            | Statement::Revise { .. }
            | Statement::Speak { .. }
            | Statement::Yield { .. } => false,
        })
    }
    block_uses_effect_control(&weave.body)
}

fn is_buffer_type(value_type: ValueType) -> bool {
    matches!(value_type, ValueType::BufferWhole | ValueType::BufferTruth)
}

fn buffer_element_from_value_type(value_type: ValueType) -> Option<BufferElement> {
    match value_type {
        ValueType::BufferWhole => Some(BufferElement::Whole),
        ValueType::BufferTruth => Some(BufferElement::Truth),
        _ => None,
    }
}

fn validate_block(
    statements: &[Statement],
    scope: &mut BTreeMap<String, BindingState>,
    signatures: &BTreeMap<String, FunctionSignature>,
    records: &[RecordDeclaration],
    weave: &Weave,
    root: bool,
    resource_plan: &mut SemanticResourcePlan,
) -> Result<(), CompilerError> {
    for (index, statement) in statements.iter().enumerate() {
        match statement {
            Statement::Bind {
                name,
                mutable,
                comptime,
                value,
                span,
            } => {
                if !root {
                    return Err(CompilerError::new(
                        *span,
                        if *comptime {
                            "AE-COMPTIME-001: comptime bind is allowed only in a weave root"
                        } else {
                            "bind is only allowed in a weave root; use a mutable root binding with revise inside blocks"
                        },
                    ));
                }
                if *comptime {
                    if *mutable {
                        return Err(CompilerError::new(
                            *span,
                            "AE-COMPTIME-001: comptime bind must be immutable",
                        ));
                    }
                    let _ = evaluate_comptime_whole(value)?;
                }
                if scope.contains_key(name) {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} already exists in this weave"),
                    ));
                }
                let mut resource = resource_state_for_expression(value, scope)?;
                if matches!(value.kind, ExpressionKind::Arena { .. }) {
                    if weave.name != "main" {
                        return Err(CompilerError::new(
                            *span,
                            "the M2 arena declaration must appear in weave main",
                        ));
                    }
                    if let ResourceBindingState::Arena { name: arena_name } = &mut resource {
                        *arena_name = name.clone();
                    }
                }
                let value_type = expression_type(value, scope, signatures, records)?;
                if matches!(value_type, ValueType::AccessArena) {
                    return Err(CompilerError::new(
                        value.span,
                        "an access loan must be consumed by one resource operation or access parameter call",
                    ));
                }
                scope.insert(
                    name.clone(),
                    BindingState {
                        value_type,
                        mutable: *mutable,
                        moved: false,
                        resource,
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
                if is_buffer_type(binding.value_type) || binding.value_type == ValueType::Arena {
                    return Err(CompilerError::new(
                        *span,
                        "M2 resource owners are replaced only by their closed resource outcomes, never revise",
                    ));
                }
                if expression_moves_name(value, name) {
                    return Err(CompilerError::new(
                        value.span,
                        "revise cannot move its own target while that replacement is reserved",
                    ));
                }
                let expected = binding.value_type;
                let actual = expression_type(value, scope, signatures, records)?;
                require_source_type(actual, expected, value.span, "revise")?;
            }
            Statement::Speak { value, .. } => {
                let value_type = expression_type(value, scope, signatures, records)?;
                require_source_type(value_type, ValueType::Text, value.span, "speak")?;
            }
            Statement::Yield { value, span } => {
                if !root || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        "yield is allowed only as the final statement of a weave root",
                    ));
                }
                let value_type = expression_type(value, scope, signatures, records)?;
                require_source_type(value_type, weave.result, value.span, "yield")?;
            }
            Statement::Raise { code, span } => {
                if !root || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        "raise is allowed only as the final statement of an erroring weave root",
                    ));
                }
                if weave.effect != Effect::ErrorWhole {
                    return Err(CompilerError::new(
                        *span,
                        "AE-EFFECT-001: raise requires the weave signature phrase raises Whole",
                    ));
                }
                validate_effect_boundary(scope, *span)?;
                validate_effect_code(code, scope)?;
            }
            Statement::Forward {
                weave: called,
                arguments,
                span,
            } => {
                if !root || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        "forward is allowed only as the final statement of an erroring weave root",
                    ));
                }
                if weave.effect != Effect::ErrorWhole {
                    return Err(CompilerError::new(
                        *span,
                        "AE-EFFECT-001: forward requires the weave signature phrase raises Whole",
                    ));
                }
                validate_effect_boundary(scope, *span)?;
                let signature =
                    validate_effect_call(called, arguments, scope, signatures, *span, "forward")?;
                if signature.result != weave.result {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-EFFECT-002: forward call {called} returns {}, but this weave returns {}",
                            signature.result, weave.result
                        ),
                    ));
                }
            }
            Statement::Handle {
                weave: called,
                arguments,
                success_destination,
                error_destination,
                span,
            } => {
                if !root || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        "handle is allowed only as the final statement of a total weave root",
                    ));
                }
                if weave.effect != Effect::Total {
                    return Err(CompilerError::new(
                        *span,
                        "AE-EFFECT-002: handle discharges Error[Whole], so its enclosing weave must be total",
                    ));
                }
                validate_effect_boundary(scope, *span)?;
                let signature =
                    validate_effect_call(called, arguments, scope, signatures, *span, "handle")?;
                if weave.result != ValueType::Whole || signature.result != ValueType::Whole {
                    return Err(CompilerError::new(
                        *span,
                        "AE-EFFECT-002: the terminal M4 handle form requires both caller and callee results to be Whole",
                    ));
                }
                validate_effect_destination(
                    scope,
                    success_destination,
                    signature.result,
                    *span,
                    "handle success",
                )?;
                if success_destination == error_destination {
                    return Err(CompilerError::new(
                        *span,
                        "AE-EFFECT-002: handle success and error destinations must be distinct mutable root bindings",
                    ));
                }
                validate_effect_destination(
                    scope,
                    error_destination,
                    ValueType::Whole,
                    *span,
                    "handle error",
                )?;
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                if matches!(condition.kind, ExpressionKind::Resource(_)) {
                    if !root || index + 1 != statements.len() {
                        return Err(CompilerError::new(
                            condition.span,
                            "a resource outcome choose must be the final statement of a weave root",
                        ));
                    }
                    let context = ResourceValidationContext {
                        signatures,
                        records,
                        weave,
                    };
                    validate_resource_choose(
                        condition,
                        when_bright,
                        when_dim,
                        scope,
                        &context,
                        resource_plan,
                    )?;
                    continue;
                }
                let condition_type = expression_type(condition, scope, signatures, records)?;
                require_source_type(
                    condition_type,
                    ValueType::Truth,
                    condition.span,
                    "choose condition",
                )?;
                let original = scope.clone();
                let mut bright_scope = original.clone();
                validate_block(
                    when_bright,
                    &mut bright_scope,
                    signatures,
                    records,
                    weave,
                    false,
                    resource_plan,
                )?;
                let mut dim_scope = original.clone();
                if !when_dim.is_empty() {
                    validate_block(
                        when_dim,
                        &mut dim_scope,
                        signatures,
                        records,
                        weave,
                        false,
                        resource_plan,
                    )?;
                }
                merge_scope(scope, &bright_scope, &dim_scope, statement.span())?;
            }
            Statement::While {
                condition, body, ..
            } => {
                let condition_type = expression_type(condition, scope, signatures, records)?;
                require_source_type(
                    condition_type,
                    ValueType::Truth,
                    condition.span,
                    "while condition",
                )?;
                let before_loop = scope.clone();
                let mut body_scope = before_loop.clone();
                validate_block(
                    body,
                    &mut body_scope,
                    signatures,
                    records,
                    weave,
                    false,
                    resource_plan,
                )?;
                merge_scope(scope, &before_loop, &body_scope, statement.span())?;
            }
        }
    }
    if root
        && !matches!(
            statements.last(),
            Some(
                Statement::Yield { .. }
                    | Statement::Raise { .. }
                    | Statement::Forward { .. }
                    | Statement::Handle { .. }
                    | Statement::Choose {
                        condition: Expression {
                            kind: ExpressionKind::Resource(_),
                            ..
                        },
                        ..
                    }
            )
        )
    {
        return Err(CompilerError::new(
            weave.span,
            "every weave must end with yield",
        ));
    }
    Ok(())
}

fn validate_effect_boundary(scope: &BindingScope, span: Span) -> Result<(), CompilerError> {
    let Some((name, _)) = scope.iter().find(|(_, binding)| {
        !binding.moved
            && (is_unique_value(binding.value_type)
                || binding.value_type == ValueType::Arena
                || !matches!(binding.resource, ResourceBindingState::Plain))
    }) else {
        return Ok(());
    };
    Err(CompilerError::new(
        span,
        format!(
            "AE-EFFECT-003: effect control cannot cross live owner, loan, arena, or Buffer binding {name}"
        ),
    ))
}

fn validate_effect_code(code: &Atom, scope: &mut BindingScope) -> Result<(), CompilerError> {
    if !matches!(code.kind, AtomKind::Whole(_) | AtomKind::Name(_)) {
        return Err(CompilerError::new(
            code.span,
            "AE-EFFECT-004: raise requires a Whole literal or copy binding name",
        ));
    }
    let value_type = atom_type(code, scope)?;
    if value_type != ValueType::Whole {
        return Err(CompilerError::new(
            code.span,
            "AE-EFFECT-004: raise requires a Whole literal or copy binding name",
        ));
    }
    Ok(())
}

fn validate_effect_call(
    called: &str,
    arguments: &[Atom],
    scope: &mut BindingScope,
    signatures: &BTreeMap<String, FunctionSignature>,
    span: Span,
    operation: &str,
) -> Result<FunctionSignature, CompilerError> {
    let Some(signature) = signatures.get(called) else {
        return Err(CompilerError::new(
            span,
            format!("weave {called} has not been declared"),
        ));
    };
    if signature.effect != Effect::ErrorWhole {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-EFFECT-002: {operation} call {called} must target a weave that raises Whole"
            ),
        ));
    }
    if signature.parameters.len() != arguments.len() {
        return Err(CompilerError::new(
            span,
            format!(
                "{operation} call {called} requires {} argument(s), received {}",
                signature.parameters.len(),
                arguments.len()
            ),
        ));
    }
    for (argument, parameter) in arguments.iter().zip(&signature.parameters) {
        if parameter.mode != ParameterMode::Own
            || !matches!(parameter.value_type, ValueType::Whole | ValueType::Truth)
        {
            return Err(CompilerError::new(
                span,
                format!(
                    "AE-EFFECT-003: {operation} call {called} cannot cross a non-copy parameter boundary"
                ),
            ));
        }
        if !matches!(
            argument.kind,
            AtomKind::Whole(_) | AtomKind::Truth(_) | AtomKind::Name(_)
        ) {
            return Err(CompilerError::new(
                argument.span,
                format!(
                    "AE-EFFECT-003: {operation} call arguments must be copy literals or copy binding names"
                ),
            ));
        }
        require_source_type(
            atom_type(argument, scope)?,
            parameter.value_type,
            argument.span,
            "effect call argument",
        )?;
    }
    Ok(signature.clone())
}

fn validate_effect_destination(
    scope: &BindingScope,
    destination: &str,
    expected: ValueType,
    span: Span,
    operation: &str,
) -> Result<(), CompilerError> {
    let Some(binding) = scope.get(destination) else {
        return Err(CompilerError::new(
            span,
            format!("AE-EFFECT-002: {operation} destination {destination} has not been introduced"),
        ));
    };
    if !binding.mutable || binding.moved || binding.value_type != expected {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-EFFECT-004: {operation} destination {destination} must be a live mutable {expected} root binding"
            ),
        ));
    }
    Ok(())
}

fn validate_resource_choose(
    condition: &Expression,
    when_bright: &[Statement],
    when_dim: &[Statement],
    scope: &BindingScope,
    context: &ResourceValidationContext<'_>,
    resource_plan: &mut SemanticResourcePlan,
) -> Result<(), CompilerError> {
    if when_dim.is_empty() {
        return Err(CompilerError::new(
            condition.span,
            "resource outcomes require an explicit otherwise branch for the dim outcome",
        ));
    }
    let ExpressionKind::Resource(operation) = &condition.kind else {
        return Err(CompilerError::new(
            condition.span,
            "internal compiler expected a resource outcome condition",
        ));
    };
    let (mut bright_scope, mut dim_scope) =
        resource_outcome_scopes(operation, scope, condition.span, resource_plan)?;
    validate_resource_terminal_block(when_bright, &mut bright_scope, context, resource_plan)?;
    validate_resource_terminal_block(when_dim, &mut dim_scope, context, resource_plan)
}

fn validate_resource_terminal_block(
    statements: &[Statement],
    scope: &mut BindingScope,
    context: &ResourceValidationContext<'_>,
    resource_plan: &mut SemanticResourcePlan,
) -> Result<(), CompilerError> {
    if statements.len() != 1 {
        return Err(CompilerError::new(
            statements
                .first()
                .map_or(context.weave.span, Statement::span),
            "each M2 resource outcome branch must contain exactly one terminal yield or resource choose",
        ));
    }
    match &statements[0] {
        Statement::Yield { value, .. } => {
            let value_type = expression_type(value, scope, context.signatures, context.records)?;
            require_source_type(value_type, context.weave.result, value.span, "yield")
        }
        Statement::Choose {
            condition,
            when_bright,
            when_dim,
            ..
        } if matches!(condition.kind, ExpressionKind::Resource(_)) => validate_resource_choose(
            condition,
            when_bright,
            when_dim,
            scope,
            context,
            resource_plan,
        ),
        statement => Err(CompilerError::new(
            statement.span(),
            "each M2 resource outcome branch must end in yield or another resource choose",
        )),
    }
}

fn resource_outcome_scopes(
    operation: &ResourceOperation,
    scope: &BindingScope,
    span: Span,
    resource_plan: &mut SemanticResourcePlan,
) -> Result<ResourceOutcomeScopes, CompilerError> {
    match operation {
        ResourceOperation::Allocate {
            arena,
            buffer,
            capacity,
            destination,
        } => {
            let arena_name = validate_access_arena(arena, scope)?;
            let (element, original) = validate_buffer_move_destination(buffer, destination, scope)?;
            let mut capacity_scope = scope.clone();
            let capacity_type = atom_type(capacity, &mut capacity_scope)?;
            require_source_type(capacity_type, ValueType::Whole, capacity.span, "allocate")?;
            resource_plan.record_outcome(
                span,
                SemanticResourceOperation::Allocate {
                    region: arena_name.clone(),
                    owner: destination.clone(),
                    destination: destination.clone(),
                    element,
                },
            )?;
            let mut bright = scope.clone();
            let mut dim = scope.clone();
            let Some(bright_target) = bright.get_mut(destination) else {
                return Err(CompilerError::new(
                    span,
                    "resource destination has not been introduced",
                ));
            };
            bright_target.moved = false;
            bright_target.resource = ResourceBindingState::Buffer {
                element,
                arena: Some(arena_name),
                allocated: true,
            };
            let Some(dim_target) = dim.get_mut(destination) else {
                return Err(CompilerError::new(
                    span,
                    "resource destination has not been introduced",
                ));
            };
            dim_target.moved = false;
            dim_target.resource = original.resource;
            Ok((bright, dim))
        }
        ResourceOperation::Append {
            buffer,
            value,
            destination,
        } => {
            let (element, original) = validate_buffer_move_destination(buffer, destination, scope)?;
            let ResourceBindingState::Buffer {
                arena: Some(region),
                allocated: true,
                ..
            } = &original.resource
            else {
                return Err(CompilerError::new(
                    buffer.span,
                    "append requires a buffer that was successfully allocated or received through an access-boundary weave",
                ));
            };
            let mut value_scope = scope.clone();
            let value_type = atom_type(value, &mut value_scope)?;
            require_source_type(value_type, element.element_type(), value.span, "append")?;
            resource_plan.record_outcome(
                span,
                SemanticResourceOperation::Append {
                    region: region.clone(),
                    owner: destination.clone(),
                    destination: destination.clone(),
                    element,
                },
            )?;
            let mut bright = scope.clone();
            let mut dim = scope.clone();
            for outcome_scope in [&mut bright, &mut dim] {
                let Some(target) = outcome_scope.get_mut(destination) else {
                    return Err(CompilerError::new(
                        span,
                        "resource destination has not been introduced",
                    ));
                };
                target.moved = false;
                target.resource = original.resource.clone();
            }
            Ok((bright, dim))
        }
        ResourceOperation::At {
            buffer,
            index,
            destination,
        } => {
            let buffer_binding = validate_borrowed_allocated_buffer(buffer, scope)?;
            let element =
                buffer_element_from_value_type(buffer_binding.value_type).ok_or_else(|| {
                    CompilerError::new(
                        buffer.span,
                        "at requires a BufferWhole or BufferTruth owner",
                    )
                })?;
            let ResourceBindingState::Buffer {
                arena: Some(region),
                allocated: true,
                ..
            } = &buffer_binding.resource
            else {
                return Err(CompilerError::new(
                    buffer.span,
                    "at requires a buffer with a named Arena region",
                ));
            };
            let mut index_scope = scope.clone();
            let index_type = atom_type(index, &mut index_scope)?;
            require_source_type(index_type, ValueType::Whole, index.span, "at")?;
            let destination_binding = scope.get(destination).ok_or_else(|| {
                CompilerError::new(
                    span,
                    format!("resource destination {destination} has not been introduced"),
                )
            })?;
            if !destination_binding.mutable {
                return Err(CompilerError::new(
                    span,
                    format!("resource destination {destination} must be a mutable root binding"),
                ));
            }
            require_source_type(
                destination_binding.value_type,
                element.element_type(),
                span,
                "at destination",
            )?;
            let AtomKind::Borrow(owner) = &buffer.kind else {
                return Err(CompilerError::new(
                    buffer.span,
                    "at requires borrow followed by an allocated buffer binding",
                ));
            };
            resource_plan.record_outcome(
                span,
                SemanticResourceOperation::At {
                    region: region.clone(),
                    owner: owner.clone(),
                    destination: destination.clone(),
                    element,
                },
            )?;
            Ok((scope.clone(), scope.clone()))
        }
    }
}

fn validate_access_arena(
    arena: &Atom,
    scope: &BTreeMap<String, BindingState>,
) -> Result<String, CompilerError> {
    let AtomKind::Access(name) = &arena.kind else {
        return Err(CompilerError::new(
            arena.span,
            "allocate requires access followed by the named Arena capability",
        ));
    };
    let binding = scope.get(name).ok_or_else(|| {
        CompilerError::new(
            arena.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.value_type != ValueType::Arena || binding.moved {
        return Err(CompilerError::new(
            arena.span,
            "access requires a live Arena capability",
        ));
    }
    match &binding.resource {
        ResourceBindingState::Arena { name } => Ok(name.clone()),
        _ => Err(CompilerError::new(
            arena.span,
            "access requires a named Arena capability",
        )),
    }
}

fn validate_buffer_move_destination(
    buffer: &Atom,
    destination: &str,
    scope: &BTreeMap<String, BindingState>,
) -> Result<(BufferElement, BindingState), CompilerError> {
    let AtomKind::Move(name) = &buffer.kind else {
        return Err(CompilerError::new(
            buffer.span,
            "resource replacement requires move followed by its buffer destination name",
        ));
    };
    if name != destination {
        return Err(CompilerError::new(
            buffer.span,
            "M2 resource outcomes must restore the same Buffer binding named after into",
        ));
    }
    let binding = scope.get(name).cloned().ok_or_else(|| {
        CompilerError::new(
            buffer.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.moved {
        return Err(CompilerError::new(
            buffer.span,
            format!("value {name} was already moved"),
        ));
    }
    if !binding.mutable {
        return Err(CompilerError::new(
            buffer.span,
            format!("resource destination {name} must be a mutable root binding"),
        ));
    }
    let Some(element) = buffer_element_from_value_type(binding.value_type) else {
        return Err(CompilerError::new(
            buffer.span,
            "resource replacement requires a BufferWhole or BufferTruth owner",
        ));
    };
    Ok((element, binding))
}

fn validate_borrowed_allocated_buffer<'a>(
    buffer: &Atom,
    scope: &'a BTreeMap<String, BindingState>,
) -> Result<&'a BindingState, CompilerError> {
    let AtomKind::Borrow(name) = &buffer.kind else {
        return Err(CompilerError::new(
            buffer.span,
            "at requires borrow followed by an allocated buffer binding",
        ));
    };
    let binding = scope.get(name).ok_or_else(|| {
        CompilerError::new(
            buffer.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.moved {
        return Err(CompilerError::new(
            buffer.span,
            format!("value {name} was moved and cannot be borrowed"),
        ));
    }
    if !is_buffer_type(binding.value_type)
        || !matches!(
            binding.resource,
            ResourceBindingState::Buffer {
                allocated: true,
                ..
            }
        )
    {
        return Err(CompilerError::new(
            buffer.span,
            "at requires a buffer that was successfully allocated or received through an access-boundary weave",
        ));
    }
    Ok(binding)
}

fn expression_moves_name(expression: &Expression, target: &str) -> bool {
    fn atom_moves_name(atom: &Atom, target: &str) -> bool {
        matches!(&atom.kind, AtomKind::Move(name) if name == target)
    }

    match &expression.kind {
        ExpressionKind::Atom(atom) | ExpressionKind::Unary { argument: atom, .. } => {
            atom_moves_name(atom, target)
        }
        ExpressionKind::Arena { .. } | ExpressionKind::Buffer { .. } => false,
        ExpressionKind::Binary { left, right, .. } => {
            atom_moves_name(left, target) || atom_moves_name(right, target)
        }
        ExpressionKind::Cut { text, start, end }
        | ExpressionKind::Slice {
            bytes: text,
            start,
            end,
        } => {
            atom_moves_name(text, target)
                || atom_moves_name(start, target)
                || atom_moves_name(end, target)
        }
        ExpressionKind::Ternary {
            first,
            second,
            third,
            ..
        } => {
            atom_moves_name(first, target)
                || atom_moves_name(second, target)
                || atom_moves_name(third, target)
        }
        ExpressionKind::Call { arguments, .. }
        | ExpressionKind::MakeRecord {
            fields: arguments, ..
        } => arguments
            .iter()
            .any(|argument| atom_moves_name(argument, target)),
        ExpressionKind::Field { record, .. } => atom_moves_name(record, target),
        ExpressionKind::Resource(ResourceOperation::Allocate {
            arena,
            buffer,
            capacity,
            ..
        }) => {
            atom_moves_name(arena, target)
                || atom_moves_name(buffer, target)
                || atom_moves_name(capacity, target)
        }
        ExpressionKind::Resource(ResourceOperation::Append { buffer, value, .. }) => {
            atom_moves_name(buffer, target) || atom_moves_name(value, target)
        }
        ExpressionKind::Resource(ResourceOperation::At { buffer, index, .. }) => {
            atom_moves_name(buffer, target) || atom_moves_name(index, target)
        }
    }
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
                resource: if left_binding.resource == right_binding.resource {
                    left_binding.resource.clone()
                } else {
                    ResourceBindingState::Plain
                },
            },
        );
    }
    Ok(())
}

fn expression_type(
    expression: &Expression,
    scope: &mut BTreeMap<String, BindingState>,
    signatures: &BTreeMap<String, FunctionSignature>,
    records: &[RecordDeclaration],
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => atom_type(atom, scope),
        ExpressionKind::Arena { .. } => Ok(ValueType::Arena),
        ExpressionKind::Buffer { element } => Ok(element.value_type()),
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
                    if matches!(argument_type, ValueType::Bytes | ValueType::Record(_)) {
                        return Err(CompilerError::new(
                            argument.span,
                            "render does not accept Bytes or records; project a record field first",
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
                UnaryOperation::Number => {
                    require_source_type(argument_type, ValueType::Text, argument.span, "number")?;
                    Ok(ValueType::Whole)
                }
                UnaryOperation::Pack16 | UnaryOperation::Pack32 | UnaryOperation::Pack64 => {
                    require_source_type(
                        argument_type,
                        ValueType::Whole,
                        argument.span,
                        operation.word(),
                    )?;
                    Ok(ValueType::Bytes)
                }
                UnaryOperation::Count => {
                    let Some(_) = buffer_element_from_value_type(argument_type) else {
                        return Err(CompilerError::new(
                            argument.span,
                            "count requires borrow followed by an allocated BufferWhole or BufferTruth",
                        ));
                    };
                    let _ = validate_borrowed_allocated_buffer(argument, scope)?;
                    Ok(ValueType::Whole)
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
                    if matches!(
                        left_type,
                        ValueType::Arena
                            | ValueType::BufferWhole
                            | ValueType::BufferTruth
                            | ValueType::AccessArena
                    ) {
                        return Err(CompilerError::new(
                            expression.span,
                            "same does not compare Aether resource values",
                        ));
                    }
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
                BinaryOperation::Unpack16 | BinaryOperation::Unpack32 => {
                    require_source_type(left_type, ValueType::Bytes, left.span, operation.word())?;
                    require_source_type(
                        right_type,
                        ValueType::Whole,
                        right.span,
                        operation.word(),
                    )?;
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
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => match operation {
            TernaryOperation::Seek => {
                require_source_type(
                    atom_type(first, scope)?,
                    ValueType::Text,
                    first.span,
                    "seek",
                )?;
                require_source_type(
                    atom_type(second, scope)?,
                    ValueType::Text,
                    second.span,
                    "seek",
                )?;
                require_source_type(
                    atom_type(third, scope)?,
                    ValueType::Whole,
                    third.span,
                    "seek",
                )?;
                Ok(ValueType::Whole)
            }
            TernaryOperation::Poke | TernaryOperation::Poke32 => {
                require_source_type(
                    atom_type(first, scope)?,
                    ValueType::Bytes,
                    first.span,
                    operation.word(),
                )?;
                require_source_type(
                    atom_type(second, scope)?,
                    ValueType::Whole,
                    second.span,
                    operation.word(),
                )?;
                require_source_type(
                    atom_type(third, scope)?,
                    ValueType::Whole,
                    third.span,
                    operation.word(),
                )?;
                Ok(ValueType::Bytes)
            }
        },
        ExpressionKind::Call { weave, arguments } => {
            let Some(signature) = signatures.get(weave) else {
                return Err(CompilerError::new(
                    expression.span,
                    format!("weave {weave} has not been declared"),
                ));
            };
            if signature.effect != Effect::Total {
                return Err(CompilerError::new(
                    expression.span,
                    format!(
                        "AE-EFFECT-001: call {weave} raises Whole; use terminal handle or forward instead of an ordinary call"
                    ),
                ));
            }
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
                let expected = if parameter.mode == ParameterMode::Access {
                    ValueType::AccessArena
                } else {
                    parameter.value_type
                };
                require_source_type(argument_type, expected, argument.span, "call argument")?;
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
        ExpressionKind::MakeRecord { record, fields } => {
            let (record_id, declaration) = record_by_name(records, record, expression.span)?;
            if fields.len() != declaration.fields.len() {
                return Err(CompilerError::new(
                    expression.span,
                    format!(
                        "make {record} requires {} field value(s), received {}",
                        declaration.fields.len(),
                        fields.len()
                    ),
                ));
            }
            for (value, field) in fields.iter().zip(&declaration.fields) {
                let actual = atom_type(value, scope)?;
                require_source_type(actual, field.value_type, value.span, "record field")?;
            }
            Ok(ValueType::Record(record_id))
        }
        ExpressionKind::Field { record, field } => {
            if !matches!(record.kind, AtomKind::Borrow(_)) {
                return Err(CompilerError::new(
                    record.span,
                    "field requires borrow followed by a record binding name",
                ));
            }
            let record_type = atom_type(record, scope)?;
            let ValueType::Record(record_id) = record_type else {
                return Err(CompilerError::new(
                    record.span,
                    "field requires a declared record value",
                ));
            };
            let declaration = record_by_id(records, record_id, record.span)?;
            let Some(declaration_field) = declaration
                .fields
                .iter()
                .find(|candidate| candidate.name == *field)
            else {
                return Err(CompilerError::new(
                    expression.span,
                    format!("record {} has no field named {field}", declaration.name),
                ));
            };
            Ok(declaration_field.value_type)
        }
        ExpressionKind::Resource(_) => Err(CompilerError::new(
            expression.span,
            "a resource outcome must be handled immediately as a terminal choose condition",
        )),
    }
}

fn validate_record_declarations(records: &[RecordDeclaration]) -> Result<(), CompilerError> {
    let _ = record_type_map(records)?;
    for record in records {
        validate_name(&record.name, record.span, "record name", false)?;
        if record.fields.is_empty() {
            return Err(CompilerError::new(
                record.span,
                "records require at least one field",
            ));
        }
        if record.fields.len() > MAX_RECORD_FIELDS {
            return Err(CompilerError::new(
                record.span,
                format!("records support at most {MAX_RECORD_FIELDS} fields"),
            ));
        }
        let mut fields = BTreeMap::new();
        for field in &record.fields {
            validate_name(&field.name, field.span, "record field name", false)?;
            if fields.insert(field.name.as_str(), ()).is_some() {
                return Err(CompilerError::new(
                    field.span,
                    format!("record field {} is declared more than once", field.name),
                ));
            }
            if matches!(
                field.value_type,
                ValueType::Record(_)
                    | ValueType::Arena
                    | ValueType::BufferWhole
                    | ValueType::BufferTruth
                    | ValueType::AccessArena
            ) {
                return Err(CompilerError::new(
                    field.span,
                    "record fields may use only Text, Whole, Truth, or Bytes",
                ));
            }
        }
    }
    Ok(())
}

fn validate_value_type(
    value_type: ValueType,
    records: &[RecordDeclaration],
    span: Span,
) -> Result<(), CompilerError> {
    if let ValueType::Record(record_id) = value_type {
        let _ = record_by_id(records, record_id, span)?;
    }
    Ok(())
}

fn record_by_name<'a>(
    records: &'a [RecordDeclaration],
    name: &str,
    span: Span,
) -> Result<(u16, &'a RecordDeclaration), CompilerError> {
    let Some((index, declaration)) = records
        .iter()
        .enumerate()
        .find(|(_, declaration)| declaration.name == name)
    else {
        return Err(CompilerError::new(
            span,
            format!("record {name} has not been declared"),
        ));
    };
    let record_id = u16::try_from(index)
        .map_err(|_| CompilerError::new(span, "record identifier is outside the AETH range"))?;
    Ok((record_id, declaration))
}

fn record_by_id(
    records: &[RecordDeclaration],
    record_id: u16,
    span: Span,
) -> Result<&RecordDeclaration, CompilerError> {
    records.get(usize::from(record_id)).ok_or_else(|| {
        CompilerError::new(
            span,
            "record type identifier is outside the declared record table",
        )
    })
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
            if binding.value_type == ValueType::Arena {
                return Err(CompilerError::new(
                    atom.span,
                    format!("Arena capability {name} requires explicit access"),
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
            if binding.value_type == ValueType::Arena {
                return Err(CompilerError::new(
                    atom.span,
                    "Arena capabilities cannot be borrowed; use access for one operation or call",
                ));
            }
            if matches!(
                binding.resource,
                ResourceBindingState::Buffer {
                    allocated: false,
                    ..
                }
            ) {
                return Err(CompilerError::new(
                    atom.span,
                    format!("buffer {name} is unallocated and cannot be borrowed"),
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
        AtomKind::Access(name) => {
            let Some(binding) = scope.get(name) else {
                return Err(CompilerError::new(
                    atom.span,
                    format!("value {name} has not been bound in this weave"),
                ));
            };
            if binding.value_type != ValueType::Arena || binding.moved {
                return Err(CompilerError::new(
                    atom.span,
                    format!("access requires the live Arena capability {name}"),
                ));
            }
            Ok(ValueType::AccessArena)
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

fn emit_bytecode_with_resource_plan(
    program: &Program,
    resource_plan: &SemanticResourcePlan,
) -> Result<Vec<u8>, CompilerError> {
    let mut weave_indices = BTreeMap::new();
    let mut weave_results = BTreeMap::new();
    for (index, weave) in program.weaves.iter().enumerate() {
        weave_indices.insert(weave.name.clone(), index);
        weave_results.insert(weave.name.clone(), weave.result);
    }

    let mut compiled = Vec::new();
    for weave in &program.weaves {
        let layout = slot_layout(weave, &weave_results, &program.records)?;
        let mut code = Vec::new();
        emit_block(
            &weave.body,
            &layout,
            &weave_indices,
            &program.records,
            resource_plan,
            &mut code,
        )?;
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
    bytecode.push(ARTIFACT_VERSION_V8);
    write_u32(&mut bytecode, resource_plan.arena_capacity());
    write_u16(
        &mut bytecode,
        u16::try_from(program.records.len()).map_err(|_| {
            CompilerError::new(
                Span::synthetic(),
                "artifact contains too many records for AETH",
            )
        })?,
    );
    if !program.records.is_empty() {
        for record in &program.records {
            let name_length = u8::try_from(record.name.len()).map_err(|_| {
                CompilerError::new(record.span, "record name exceeds the AETH name limit")
            })?;
            bytecode.push(name_length);
            bytecode.extend_from_slice(record.name.as_bytes());
            bytecode.push(u8::try_from(record.fields.len()).map_err(|_| {
                CompilerError::new(record.span, "record has too many fields for AETH")
            })?);
            for field in &record.fields {
                let field_name_length = u8::try_from(field.name.len()).map_err(|_| {
                    CompilerError::new(field.span, "record field name exceeds the AETH name limit")
                })?;
                bytecode.push(field_name_length);
                bytecode.extend_from_slice(field.name.as_bytes());
                write_primitive_value_type(&mut bytecode, field.value_type, field.span)?;
            }
        }
    }
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
            write_value_type(&mut bytecode, parameter.value_type);
            bytecode.push(parameter.mode.to_byte());
        }
        write_value_type(&mut bytecode, weave.result);
        bytecode.push(weave.effect.to_byte());
        write_u16(
            &mut bytecode,
            u16::try_from(locals.len()).map_err(|_| {
                CompilerError::new(weave.span, "weave has too many local bindings for AETH")
            })?,
        );
        for local in locals {
            write_value_type(&mut bytecode, local.value_type);
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
    records: &[RecordDeclaration],
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
            ..
        } = statement
        {
            let value_type = static_expression_type(value, &layout, weave_results, records)?;
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
    records: &[RecordDeclaration],
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => static_atom_type(atom, layout),
        ExpressionKind::Arena { .. } => Ok(ValueType::Arena),
        ExpressionKind::Buffer { element } => Ok(element.value_type()),
        ExpressionKind::Unary {
            operation,
            argument,
        } => match operation {
            UnaryOperation::Not => Ok(ValueType::Truth),
            UnaryOperation::Measure => Ok(ValueType::Whole),
            UnaryOperation::Render => {
                let _ = static_atom_type(argument, layout)?;
                Ok(ValueType::Text)
            }
            UnaryOperation::Extent => Ok(ValueType::Whole),
            UnaryOperation::Encode => Ok(ValueType::Bytes),
            UnaryOperation::Decode => Ok(ValueType::Text),
            UnaryOperation::Number => Ok(ValueType::Whole),
            UnaryOperation::Pack16 | UnaryOperation::Pack32 | UnaryOperation::Pack64 => {
                Ok(ValueType::Bytes)
            }
            UnaryOperation::Count => Ok(ValueType::Whole),
        },
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => {
            let _ = static_atom_type(left, layout)?;
            let _ = static_atom_type(right, layout)?;
            match operation {
                BinaryOperation::Sum
                | BinaryOperation::Difference
                | BinaryOperation::Product
                | BinaryOperation::Glyph
                | BinaryOperation::Quotient
                | BinaryOperation::Remainder
                | BinaryOperation::Octet
                | BinaryOperation::Unpack16
                | BinaryOperation::Unpack32 => Ok(ValueType::Whole),
                BinaryOperation::Less | BinaryOperation::Same => Ok(ValueType::Truth),
                BinaryOperation::Join => Ok(ValueType::Text),
                BinaryOperation::Fuse | BinaryOperation::Append => Ok(ValueType::Bytes),
            }
        }
        ExpressionKind::Cut { text, start, end } => {
            let _ = static_atom_type(text, layout)?;
            let _ = static_atom_type(start, layout)?;
            let _ = static_atom_type(end, layout)?;
            Ok(ValueType::Text)
        }
        ExpressionKind::Slice { bytes, start, end } => {
            let _ = static_atom_type(bytes, layout)?;
            let _ = static_atom_type(start, layout)?;
            let _ = static_atom_type(end, layout)?;
            Ok(ValueType::Bytes)
        }
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => {
            let _ = static_atom_type(first, layout)?;
            let _ = static_atom_type(second, layout)?;
            let _ = static_atom_type(third, layout)?;
            match operation {
                TernaryOperation::Seek => Ok(ValueType::Whole),
                TernaryOperation::Poke | TernaryOperation::Poke32 => Ok(ValueType::Bytes),
            }
        }
        ExpressionKind::Call { weave: called, .. } => {
            weave_results.get(called).copied().ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    "internal compiler could not resolve a called weave",
                )
            })
        }
        ExpressionKind::MakeRecord { record, fields } => {
            for field in fields {
                let _ = static_atom_type(field, layout)?;
            }
            let (record_id, _) = record_by_name(records, record, expression.span)?;
            Ok(ValueType::Record(record_id))
        }
        ExpressionKind::Field { record, field } => {
            let ValueType::Record(record_id) = static_atom_type(record, layout)? else {
                return Err(CompilerError::new(
                    record.span,
                    "internal compiler expected a record field projection",
                ));
            };
            let declaration = record_by_id(records, record_id, record.span)?;
            declaration
                .fields
                .iter()
                .find(|candidate| candidate.name == *field)
                .map(|candidate| candidate.value_type)
                .ok_or_else(|| {
                    CompilerError::new(
                        expression.span,
                        "internal compiler could not resolve a record field",
                    )
                })
        }
        ExpressionKind::Resource(_) => Ok(ValueType::Truth),
    }
}

fn static_atom_type(
    atom: &Atom,
    layout: &BTreeMap<String, SlotInfo>,
) -> Result<ValueType, CompilerError> {
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
        AtomKind::Access(name) => layout.get(name).map_or_else(
            || {
                Err(CompilerError::new(
                    atom.span,
                    format!("value {name} has not been introduced before this binding"),
                ))
            },
            |_| Ok(ValueType::AccessArena),
        ),
    }
}

fn emit_block(
    statements: &[Statement],
    layout: &BTreeMap<String, SlotInfo>,
    weave_indices: &BTreeMap<String, usize>,
    records: &[RecordDeclaration],
    resource_plan: &SemanticResourcePlan,
    code: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    for statement in statements {
        match statement {
            Statement::Bind {
                name,
                comptime,
                value,
                ..
            } => {
                if *comptime {
                    code.push(OP_COMPTIME_WHOLE);
                    write_i64(code, evaluate_comptime_whole(value)?);
                } else {
                    emit_expression(value, layout, weave_indices, records, resource_plan, code)?;
                }
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
                emit_expression(value, layout, weave_indices, records, resource_plan, code)?;
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
                emit_expression(value, layout, weave_indices, records, resource_plan, code)?;
                code.push(OP_SPEAK);
            }
            Statement::Yield { value, .. } => {
                emit_expression(value, layout, weave_indices, records, resource_plan, code)?;
                code.push(OP_YIELD);
            }
            Statement::Raise { code: value, .. } => {
                emit_atom(value, layout, code)?;
                code.push(OP_RAISE);
            }
            Statement::Forward {
                weave, arguments, ..
            } => {
                for argument in arguments {
                    emit_atom(argument, layout, code)?;
                }
                let function = weave_indices.get(weave).ok_or_else(|| {
                    CompilerError::new(
                        statement.span(),
                        format!("internal compiler could not resolve weave {weave}"),
                    )
                })?;
                code.push(OP_FORWARD_CALL);
                write_u16(
                    code,
                    u16::try_from(*function).map_err(|_| {
                        CompilerError::new(
                            statement.span(),
                            "weave index is outside the AETH range",
                        )
                    })?,
                );
                code.push(u8::try_from(arguments.len()).map_err(|_| {
                    CompilerError::new(statement.span(), "call has too many AETH arguments")
                })?);
            }
            Statement::Handle {
                weave,
                arguments,
                success_destination,
                error_destination,
                ..
            } => {
                for argument in arguments {
                    emit_atom(argument, layout, code)?;
                }
                let function = weave_indices.get(weave).ok_or_else(|| {
                    CompilerError::new(
                        statement.span(),
                        format!("internal compiler could not resolve weave {weave}"),
                    )
                })?;
                let success_slot = layout.get(success_destination).ok_or_else(|| {
                    CompilerError::new(
                        statement.span(),
                        "internal compiler could not resolve handle success destination",
                    )
                })?;
                let error_slot = layout.get(error_destination).ok_or_else(|| {
                    CompilerError::new(
                        statement.span(),
                        "internal compiler could not resolve handle error destination",
                    )
                })?;
                code.push(OP_HANDLE_CALL);
                write_u16(
                    code,
                    u16::try_from(*function).map_err(|_| {
                        CompilerError::new(
                            statement.span(),
                            "weave index is outside the AETH range",
                        )
                    })?,
                );
                code.push(u8::try_from(arguments.len()).map_err(|_| {
                    CompilerError::new(statement.span(), "call has too many AETH arguments")
                })?);
                write_u16(code, success_slot.index);
                write_u16(code, error_slot.index);
                let success_target = reserve_u32(code);
                let error_target = reserve_u32(code);
                let success_start = code.len();
                patch_u32(code, success_target, success_start)?;
                code.push(OP_LOAD);
                write_u16(code, success_slot.index);
                code.push(OP_YIELD);
                let error_start = code.len();
                patch_u32(code, error_target, error_start)?;
                code.push(OP_LOAD);
                write_u16(code, error_slot.index);
                code.push(OP_YIELD);
            }
            Statement::Choose {
                condition,
                when_bright,
                when_dim,
                ..
            } => {
                if matches!(condition.kind, ExpressionKind::Resource(_)) {
                    emit_expression(
                        condition,
                        layout,
                        weave_indices,
                        records,
                        resource_plan,
                        code,
                    )?;
                    code.push(OP_JUMP_IF_DIM);
                    let dim_target = reserve_u32(code);
                    emit_block(
                        when_bright,
                        layout,
                        weave_indices,
                        records,
                        resource_plan,
                        code,
                    )?;
                    let dim_branch = code.len();
                    patch_u32(code, dim_target, dim_branch)?;
                    emit_block(
                        when_dim,
                        layout,
                        weave_indices,
                        records,
                        resource_plan,
                        code,
                    )?;
                    continue;
                }
                emit_expression(
                    condition,
                    layout,
                    weave_indices,
                    records,
                    resource_plan,
                    code,
                )?;
                code.push(OP_JUMP_IF_DIM);
                let dim_target = reserve_u32(code);
                emit_block(
                    when_bright,
                    layout,
                    weave_indices,
                    records,
                    resource_plan,
                    code,
                )?;
                if when_dim.is_empty() {
                    let continuation = code.len();
                    patch_u32(code, dim_target, continuation)?;
                } else {
                    code.push(OP_JUMP);
                    let end_target = reserve_u32(code);
                    let dim_branch = code.len();
                    patch_u32(code, dim_target, dim_branch)?;
                    emit_block(
                        when_dim,
                        layout,
                        weave_indices,
                        records,
                        resource_plan,
                        code,
                    )?;
                    let continuation = code.len();
                    patch_u32(code, end_target, continuation)?;
                }
            }
            Statement::While {
                condition, body, ..
            } => {
                let loop_start = code.len();
                emit_expression(
                    condition,
                    layout,
                    weave_indices,
                    records,
                    resource_plan,
                    code,
                )?;
                code.push(OP_JUMP_IF_DIM);
                let loop_end = reserve_u32(code);
                emit_block(body, layout, weave_indices, records, resource_plan, code)?;
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
    records: &[RecordDeclaration],
    resource_plan: &SemanticResourcePlan,
    code: &mut Vec<u8>,
) -> Result<(), CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => emit_atom(atom, layout, code)?,
        ExpressionKind::Arena { .. } => code.push(OP_ARENA),
        ExpressionKind::Buffer { element } => {
            code.push(OP_BUFFER);
            code.push(element.type_tag());
        }
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
                UnaryOperation::Number => OP_NUMBER,
                UnaryOperation::Pack16 => OP_PACK16,
                UnaryOperation::Pack32 => OP_PACK32,
                UnaryOperation::Pack64 => OP_PACK64,
                UnaryOperation::Count => OP_COUNT,
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
                BinaryOperation::Unpack16 => OP_UNPACK16,
                BinaryOperation::Unpack32 => OP_UNPACK32,
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
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => {
            emit_atom(first, layout, code)?;
            emit_atom(second, layout, code)?;
            emit_atom(third, layout, code)?;
            code.push(match operation {
                TernaryOperation::Seek => OP_SEEK,
                TernaryOperation::Poke => OP_POKE,
                TernaryOperation::Poke32 => OP_POKE32,
            });
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
        ExpressionKind::MakeRecord { record, fields } => {
            for field in fields {
                emit_atom(field, layout, code)?;
            }
            let (record_id, _) = record_by_name(records, record, expression.span)?;
            code.push(OP_MAKE_RECORD);
            write_u16(code, record_id);
        }
        ExpressionKind::Field { record, field } => {
            emit_atom(record, layout, code)?;
            let ValueType::Record(record_id) = static_atom_type(record, layout)? else {
                return Err(CompilerError::new(
                    record.span,
                    "internal compiler expected a record field projection",
                ));
            };
            let declaration = record_by_id(records, record_id, record.span)?;
            let field_index = declaration
                .fields
                .iter()
                .position(|candidate| candidate.name == *field)
                .ok_or_else(|| {
                    CompilerError::new(
                        expression.span,
                        "internal compiler could not resolve a record field",
                    )
                })?;
            code.push(OP_FIELD);
            write_u16(code, record_id);
            code.push(u8::try_from(field_index).map_err(|_| {
                CompilerError::new(
                    expression.span,
                    "record field index is outside the AETH range",
                )
            })?);
        }
        ExpressionKind::Resource(operation) => {
            let semantic = resource_plan.outcome(expression.span)?;
            if semantic.region().is_empty() || !semantic.matches_source(operation) {
                return Err(CompilerError::new(
                    expression.span,
                    "internal compiler found a resource AST/semantic-plan disagreement",
                ));
            }
            let destination_slot = layout.get(semantic.destination()).ok_or_else(|| {
                CompilerError::new(
                    expression.span,
                    format!(
                        "internal compiler could not resolve resource destination {}",
                        semantic.destination()
                    ),
                )
            })?;
            let destination_index = destination_slot.index;

            match operation {
                ResourceOperation::Allocate {
                    arena,
                    buffer,
                    capacity,
                    ..
                } => {
                    emit_atom(arena, layout, code)?;
                    emit_atom(buffer, layout, code)?;
                    emit_atom(capacity, layout, code)?;
                    code.push(OP_ALLOCATE);
                    code.push(semantic.element().type_tag());
                    write_u16(code, destination_index);
                }
                ResourceOperation::Append { buffer, value, .. } => {
                    emit_atom(buffer, layout, code)?;
                    emit_atom(value, layout, code)?;
                    code.push(OP_BUFFER_APPEND);
                    code.push(semantic.element().type_tag());
                    write_u16(code, destination_index);
                }
                ResourceOperation::At { buffer, index, .. } => {
                    emit_atom(buffer, layout, code)?;
                    emit_atom(index, layout, code)?;
                    code.push(OP_BUFFER_AT);
                    code.push(semantic.element().type_tag());
                    write_u16(code, destination_index);
                }
            }
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
        AtomKind::Access(name) => {
            let slot = layout.get(name).ok_or_else(|| {
                CompilerError::new(
                    atom.span,
                    format!("internal compiler could not resolve Arena capability {name}"),
                )
            })?;
            code.push(OP_ACCESS);
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
    let version = bytecode[ARTIFACT_MAGIC.len()];
    if !matches!(
        version,
        ARTIFACT_VERSION_V4
            | ARTIFACT_VERSION_V5
            | ARTIFACT_VERSION_V6
            | ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
    ) {
        return Err(BytecodeError::new(
            ARTIFACT_MAGIC.len(),
            "artifact version is not supported by this Aether VM",
        ));
    }
    let mut position = ARTIFACT_MAGIC.len() + 1;
    let arena_capacity = if matches!(
        version,
        ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
    ) {
        let capacity = read_u32(bytecode, &mut position)?;
        if capacity > MAX_ARENA_BYTES {
            return Err(BytecodeError::new(
                position,
                "AETH v6/v7/v8 arena capacity exceeds the M2 safety limit",
            ));
        }
        capacity
    } else {
        0
    };
    let mut records = Vec::new();
    if matches!(
        version,
        ARTIFACT_VERSION_V5 | ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
    ) {
        let record_count = usize::from(read_u16(bytecode, &mut position)?);
        if (version == ARTIFACT_VERSION_V5 && record_count == 0) || record_count > MAX_RECORDS {
            return Err(BytecodeError::new(
                position,
                "artifact record count is outside the Aether limit",
            ));
        }
        records.reserve(record_count);
        for _ in 0..record_count {
            let name_offset = position;
            let name_length = usize::from(read_byte(bytecode, &mut position)?);
            let name = read_ascii(bytecode, &mut position, name_length, "record name")?;
            validate_artifact_name(&name, name_offset, "record name", false)?;
            if records
                .iter()
                .any(|record: &ArtifactRecord| record.name == name)
            {
                return Err(BytecodeError::new(
                    name_offset,
                    "artifact defines a record name twice",
                ));
            }
            let field_count = usize::from(read_byte(bytecode, &mut position)?);
            if field_count == 0 || field_count > MAX_RECORD_FIELDS {
                return Err(BytecodeError::new(
                    position,
                    "artifact record field count is outside the Aether limit",
                ));
            }
            let mut fields = Vec::with_capacity(field_count);
            for _ in 0..field_count {
                let field_offset = position;
                let field_name_length = usize::from(read_byte(bytecode, &mut position)?);
                let field_name = read_ascii(
                    bytecode,
                    &mut position,
                    field_name_length,
                    "record field name",
                )?;
                validate_artifact_name(&field_name, field_offset, "record field name", false)?;
                if fields
                    .iter()
                    .any(|field: &ArtifactRecordField| field.name == field_name)
                {
                    return Err(BytecodeError::new(
                        field_offset,
                        "artifact record defines a field name twice",
                    ));
                }
                let value_type =
                    ValueType::from_primitive_tag(read_byte(bytecode, &mut position)?, position)?;
                fields.push(ArtifactRecordField {
                    name: field_name,
                    value_type,
                });
            }
            records.push(ArtifactRecord { name, fields });
        }
    }
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
            let value_type = read_value_type(bytecode, &mut position, version, records.len())?;
            let mode =
                ParameterMode::from_byte(read_byte(bytecode, &mut position)?, position, version)?;
            parameters.push((value_type, mode));
        }
        let result = read_value_type(bytecode, &mut position, version, records.len())?;
        let effect = if matches!(version, ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8) {
            Effect::from_byte(read_byte(bytecode, &mut position)?, position)?
        } else {
            Effect::Total
        };
        let local_count = usize::from(read_u16(bytecode, &mut position)?);
        if local_count > MAX_LOCALS {
            return Err(BytecodeError::new(
                position,
                "artifact local count exceeds the Aether limit",
            ));
        }
        let mut locals = Vec::with_capacity(local_count);
        for _ in 0..local_count {
            let value_type = read_value_type(bytecode, &mut position, version, records.len())?;
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
            effect,
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
    Ok(Artifact {
        version,
        arena_capacity,
        records,
        functions,
    })
}

fn verify_function(
    function_index: usize,
    function: &ArtifactFunction,
    functions: &[ArtifactFunction],
    records: &[ArtifactRecord],
    version: u8,
) -> Result<(), BytecodeError> {
    let decoded = decode_code(&function.code, version)?;
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
        stack: VerificationStack::new(),
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
    let mut effect_exited = false;
    let context = VerificationContext {
        function_index,
        function,
        functions,
        records,
    };

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
            &context,
            &instruction,
            state,
            &mut yielded,
            &mut effect_exited,
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
    if !(yielded || function.effect == Effect::ErrorWhole && effect_exited) {
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
    context: &VerificationContext<'_>,
    decoded: &DecodedInstruction,
    mut state: VerificationState,
    yielded: &mut bool,
    effect_exited: &mut bool,
) -> Result<Vec<(usize, VerificationState)>, BytecodeError> {
    let function_index = context.function_index;
    let function = context.function;
    let functions = context.functions;
    let records = context.records;
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
        Instruction::PushWhole(_) | Instruction::ComptimeWhole(_) => {
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::PushTruth(_) => {
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Arena => {
            state.stack.push(ValueType::Arena);
            continue_with(state)
        }
        Instruction::Buffer(element) => {
            state.stack.push_buffer_placeholder(*element);
            continue_with(state)
        }
        Instruction::Access(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            if local.value_type != ValueType::Arena {
                return Err(BytecodeError::new(
                    offset,
                    "access targets a local that is not an Arena capability",
                ));
            }
            ensure_readable(&state, *slot, offset, "access")?;
            state.stack.push(ValueType::AccessArena);
            continue_with(state)
        }
        Instruction::Allocate {
            element,
            destination,
        } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || local.value_type != element.value_type() {
                return Err(BytecodeError::new(
                    offset,
                    "allocate destination must be its matching mutable Buffer slot",
                ));
            }
            if !state.initialized[*destination] || !state.moved[*destination] {
                return Err(BytecodeError::new(
                    offset,
                    "allocate requires its destination Buffer slot to be moved",
                ));
            }
            pop_type(&mut state.stack, ValueType::Whole, offset, "allocate")?;
            let buffer = pop_type(&mut state.stack, element.value_type(), offset, "allocate")?;
            require_moved_buffer_from(&buffer, *destination, offset, "allocate")?;
            pop_type(&mut state.stack, ValueType::AccessArena, offset, "allocate")?;
            state.moved[*destination] = false;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::BufferAppend {
            element,
            destination,
        } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || local.value_type != element.value_type() {
                return Err(BytecodeError::new(
                    offset,
                    "buffer append destination must be its matching mutable Buffer slot",
                ));
            }
            if !state.initialized[*destination] || !state.moved[*destination] {
                return Err(BytecodeError::new(
                    offset,
                    "buffer append requires its destination Buffer slot to be moved",
                ));
            }
            pop_type(
                &mut state.stack,
                element.element_type(),
                offset,
                "buffer append",
            )?;
            let buffer = pop_type(
                &mut state.stack,
                element.value_type(),
                offset,
                "buffer append",
            )?;
            require_moved_buffer_from(&buffer, *destination, offset, "buffer append")?;
            state.moved[*destination] = false;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::BufferAt {
            element,
            destination,
        } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || local.value_type != element.element_type() {
                return Err(BytecodeError::new(
                    offset,
                    "buffer lookup destination must be its matching mutable value slot",
                ));
            }
            ensure_readable(&state, *destination, offset, "buffer lookup destination")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "buffer at")?;
            let buffer = pop_type(&mut state.stack, element.value_type(), offset, "buffer at")?;
            require_borrowed_buffer(&buffer, offset, "buffer at")?;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::Count => {
            let buffer = pop_any_type(&mut state.stack, offset, "count")?;
            if !is_buffer_type(buffer.value_type) {
                return Err(BytecodeError::new(
                    offset,
                    "count requires a BufferWhole or BufferTruth value",
                ));
            }
            require_borrowed_buffer(&buffer, offset, "count")?;
            state.stack.push(ValueType::Whole);
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
            let value = pop_type(&mut state.stack, local.value_type, offset, "store")?;
            if is_buffer_type(local.value_type) {
                require_storable_buffer(&value, offset, "store")?;
            }
            state.initialized[*slot] = true;
            state.moved[*slot] = false;
            continue_with(state)
        }
        Instruction::Load(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            if local.value_type == ValueType::Arena {
                return Err(BytecodeError::new(
                    offset,
                    "Arena capabilities must be used through access, not load",
                ));
            }
            ensure_readable(&state, *slot, offset, "load")?;
            state.stack.push(local.value_type);
            continue_with(state)
        }
        Instruction::Move(slot) => {
            let local = local_descriptor(function, *slot, offset)?;
            if local.value_type == ValueType::Arena {
                return Err(BytecodeError::new(
                    offset,
                    "Arena capabilities cannot be moved",
                ));
            }
            ensure_readable(&state, *slot, offset, "move")?;
            state.moved[*slot] = true;
            state.stack.push_moved_local(local.value_type, *slot);
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
            if is_buffer_type(local.value_type) || local.value_type == ValueType::Arena {
                return Err(BytecodeError::new(
                    offset,
                    "AETH v6 resource owners are replaced only by closed resource instructions",
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
            let value = pop_type(&mut state.stack, function.result, offset, "yield")?;
            if is_buffer_type(function.result) {
                require_yieldable_buffer(&value, offset)?;
            }
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
            if left.value_type != right.value_type {
                return Err(BytecodeError::new(
                    offset,
                    "same requires two values with the same type",
                ));
            }
            if matches!(
                left.value_type,
                ValueType::Arena
                    | ValueType::BufferWhole
                    | ValueType::BufferTruth
                    | ValueType::AccessArena
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "same does not compare Aether resource values",
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
        Instruction::Seek => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "seek")?;
            pop_type(&mut state.stack, ValueType::Text, offset, "seek")?;
            pop_type(&mut state.stack, ValueType::Text, offset, "seek")?;
            state.stack.push(ValueType::Whole);
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
        Instruction::Number => {
            pop_type(&mut state.stack, ValueType::Text, offset, "number")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Pack16 | Instruction::Pack32 | Instruction::Pack64 => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "pack")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Unpack16 | Instruction::Unpack32 => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "unpack")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "unpack")?;
            state.stack.push(ValueType::Whole);
            continue_with(state)
        }
        Instruction::Poke | Instruction::Poke32 => {
            pop_type(&mut state.stack, ValueType::Whole, offset, "poke")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "poke")?;
            pop_type(&mut state.stack, ValueType::Bytes, offset, "poke")?;
            state.stack.push(ValueType::Bytes);
            continue_with(state)
        }
        Instruction::Render => {
            let value_type = pop_any_type(&mut state.stack, offset, "render")?;
            if matches!(
                value_type.value_type,
                ValueType::Bytes
                    | ValueType::Record(_)
                    | ValueType::Arena
                    | ValueType::BufferWhole
                    | ValueType::BufferTruth
                    | ValueType::AccessArena
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "render does not accept Bytes or records; project a record field first",
                ));
            }
            state.stack.push(ValueType::Text);
            continue_with(state)
        }
        Instruction::MakeRecord(record_id) => {
            let record = artifact_record(records, *record_id, offset)?;
            for field in record.fields.iter().rev() {
                pop_type(&mut state.stack, field.value_type, offset, "make record")?;
            }
            state.stack.push(ValueType::Record(*record_id));
            continue_with(state)
        }
        Instruction::Field {
            record: record_id,
            field,
        } => {
            let record = artifact_record(records, *record_id, offset)?;
            let field = record.fields.get(*field).ok_or_else(|| {
                BytecodeError::new(offset, "field references an unknown record field")
            })?;
            pop_type(
                &mut state.stack,
                ValueType::Record(*record_id),
                offset,
                "field",
            )?;
            state.stack.push(field.value_type);
            continue_with(state)
        }
        Instruction::Raise => {
            if function.effect != Effect::ErrorWhole {
                return Err(BytecodeError::new(
                    offset,
                    "AETH RAISE requires an Error[Whole] function signature",
                ));
            }
            pop_type(&mut state.stack, ValueType::Whole, offset, "raise")?;
            require_verifier_effect_boundary(function, &state, offset)?;
            *effect_exited = true;
            Ok(Vec::new())
        }
        Instruction::ForwardCall {
            function: called,
            arguments,
        } => {
            if function.effect != Effect::ErrorWhole {
                return Err(BytecodeError::new(
                    offset,
                    "AETH FORWARD_CALL requires an Error[Whole] function signature",
                ));
            }
            let Some(called_function) = functions.get(*called) else {
                return Err(BytecodeError::new(
                    offset,
                    "forward call references an unknown weave",
                ));
            };
            verify_effect_call_signature(function, called_function, *arguments, offset, "forward")?;
            pop_verifier_effect_arguments(&mut state.stack, called_function, offset, "forward")?;
            require_verifier_effect_boundary(function, &state, offset)?;
            *effect_exited = true;
            Ok(Vec::new())
        }
        Instruction::HandleCall {
            function: called,
            arguments,
            success_destination,
            error_destination,
            success_target,
            error_target,
        } => {
            if function.effect != Effect::Total {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL must occur in a total function",
                ));
            }
            let Some(called_function) = functions.get(*called) else {
                return Err(BytecodeError::new(
                    offset,
                    "handle call references an unknown weave",
                ));
            };
            verify_effect_call_signature(function, called_function, *arguments, offset, "handle")?;
            if function.result != ValueType::Whole || called_function.result != ValueType::Whole {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL requires Whole caller and callee results for the bounded terminal handler form",
                ));
            }
            pop_verifier_effect_arguments(&mut state.stack, called_function, offset, "handle")?;
            require_verifier_effect_boundary(function, &state, offset)?;
            if success_destination == error_destination {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL requires distinct success and error destination slots",
                ));
            }
            let success_local = local_descriptor(function, *success_destination, offset)?;
            if !success_local.mutable
                || success_local.value_type != called_function.result
                || !state.initialized[*success_destination]
                || state.moved[*success_destination]
            {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL success destination must be a live mutable matching local",
                ));
            }
            let error_local = local_descriptor(function, *error_destination, offset)?;
            if !error_local.mutable
                || error_local.value_type != ValueType::Whole
                || !state.initialized[*error_destination]
                || state.moved[*error_destination]
            {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL error destination must be a live mutable Whole local",
                ));
            }
            Ok(vec![
                (*success_target, state.clone()),
                (*error_target, state),
            ])
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
            if called_function.effect != Effect::Total {
                return Err(BytecodeError::new(
                    offset,
                    "ordinary AETH CALL cannot invoke an Error[Whole] weave; use HANDLE_CALL or FORWARD_CALL",
                ));
            }
            if called_function.parameters.len() != *arguments {
                return Err(BytecodeError::new(
                    offset,
                    "call argument count disagrees with its weave signature",
                ));
            }
            for (expected, mode) in called_function.parameters.iter().rev() {
                let expected = if *mode == ParameterMode::Access {
                    ValueType::AccessArena
                } else {
                    *expected
                };
                let value = pop_type(&mut state.stack, expected, offset, "call")?;
                if is_buffer_type(expected) {
                    match mode {
                        ParameterMode::Own => {
                            require_moved_buffer(&value, offset, "call")?;
                        }
                        ParameterMode::Borrow => {
                            require_borrowed_buffer(&value, offset, "call")?;
                        }
                        ParameterMode::Access => unreachable!("access parameters use AccessArena"),
                    }
                }
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

fn require_verifier_effect_boundary(
    function: &ArtifactFunction,
    state: &VerificationState,
    offset: usize,
) -> Result<(), BytecodeError> {
    if !state.stack.is_empty() {
        return Err(BytecodeError::new(
            offset,
            "AETH effect control requires an empty operand stack after its arguments",
        ));
    }
    if function.locals.iter().enumerate().any(|(index, local)| {
        state.initialized[index]
            && !state.moved[index]
            && (is_unique_value(local.value_type)
                || local.value_type == ValueType::Arena
                || is_buffer_type(local.value_type))
    }) {
        return Err(BytecodeError::new(
            offset,
            "AETH effect control cannot cross a live owner, Arena, or Buffer local",
        ));
    }
    Ok(())
}

fn verify_effect_call_signature(
    caller: &ArtifactFunction,
    callee: &ArtifactFunction,
    arguments: usize,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if callee.effect != Effect::ErrorWhole {
        return Err(BytecodeError::new(
            offset,
            format!("AETH {operation} call must target an Error[Whole] weave"),
        ));
    }
    if callee.parameters.len() != arguments {
        return Err(BytecodeError::new(
            offset,
            format!("AETH {operation} call argument count disagrees with its weave signature"),
        ));
    }
    if callee.parameters.iter().any(|(value_type, mode)| {
        *mode != ParameterMode::Own || !matches!(value_type, ValueType::Whole | ValueType::Truth)
    }) {
        return Err(BytecodeError::new(
            offset,
            format!("AETH {operation} call cannot cross a non-copy parameter boundary"),
        ));
    }
    if operation == "forward"
        && (caller.effect != Effect::ErrorWhole || caller.result != callee.result)
    {
        return Err(BytecodeError::new(
            offset,
            "AETH FORWARD_CALL requires matching Error[Whole] caller and callee result signatures",
        ));
    }
    Ok(())
}

fn pop_verifier_effect_arguments(
    stack: &mut VerificationStack,
    callee: &ArtifactFunction,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    for (expected, _) in callee.parameters.iter().rev() {
        pop_type(stack, *expected, offset, operation)?;
    }
    Ok(())
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
        Instruction::HandleCall {
            success_target,
            error_target,
            ..
        } => vec![*success_target, *error_target],
        _ => Vec::new(),
    }
}

fn execute_function(
    artifact: &Artifact,
    function_index: usize,
    arguments: Vec<RuntimeValue>,
    stdout: &mut String,
    runtime_state: &mut RuntimeState,
    depth: usize,
) -> Result<RuntimeExit, BytecodeError> {
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
        let (expected, mode) = function.parameters[index];
        let argument = if mode == ParameterMode::Access {
            if !matches!(argument, RuntimeValue::AccessArena) {
                return Err(BytecodeError::new(
                    0,
                    "runtime access parameter did not receive an exclusive Arena access loan",
                ));
            }
            RuntimeValue::Arena
        } else {
            argument
        };
        if argument.value_type() != expected {
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
        let decoded = decode_instruction(&function.code, &mut position, artifact.version)?;
        match decoded.instruction {
            Instruction::PushText(value) => stack.push(RuntimeValue::Text(value)),
            Instruction::PushBytes(value) => stack.push(RuntimeValue::Bytes(value)),
            Instruction::PushWhole(value) | Instruction::ComptimeWhole(value) => {
                stack.push(RuntimeValue::Whole(value));
            }
            Instruction::PushTruth(value) => stack.push(RuntimeValue::Truth(value)),
            Instruction::Arena => stack.push(RuntimeValue::Arena),
            Instruction::Buffer(element) => stack.push(RuntimeValue::Buffer {
                element,
                allocated: false,
                offset: 0,
                len: 0,
                capacity: 0,
            }),
            Instruction::Access(slot) => {
                let value = read_local(&locals, slot, decoded.offset, "access")?;
                if !matches!(value, RuntimeValue::Arena) {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime access targets a local that is not an Arena",
                    ));
                }
                stack.push(RuntimeValue::AccessArena);
            }
            Instruction::Allocate {
                element,
                destination,
            } => {
                let capacity = pop_whole(&mut stack, decoded.offset, "allocate")?;
                let buffer = pop_buffer(&mut stack, element, decoded.offset, "allocate")?;
                match pop_runtime(&mut stack, decoded.offset, "allocate")? {
                    RuntimeValue::AccessArena => {}
                    value => {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            format!(
                                "allocate received {}, expected exclusive Arena access",
                                value.value_type()
                            ),
                        ))
                    }
                }
                let (buffer, allocated) =
                    runtime_allocate_buffer(buffer, capacity, runtime_state, decoded.offset)?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime allocate destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != element.value_type() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime allocate destination is not its matching mutable Buffer slot",
                    ));
                }
                if locals[destination].is_some() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime allocate destination Buffer was not moved",
                    ));
                }
                locals[destination] = Some(buffer);
                stack.push(RuntimeValue::Truth(allocated));
            }
            Instruction::BufferAppend {
                element,
                destination,
            } => {
                let value =
                    pop_buffer_element(&mut stack, element, decoded.offset, "buffer append")?;
                let buffer = pop_buffer(&mut stack, element, decoded.offset, "buffer append")?;
                let (buffer, appended) =
                    runtime_append_buffer(buffer, value, runtime_state, decoded.offset)?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime buffer append destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != element.value_type() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime buffer append destination is not its matching mutable Buffer slot",
                    ));
                }
                if locals[destination].is_some() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime buffer append destination Buffer was not moved",
                    ));
                }
                locals[destination] = Some(buffer);
                stack.push(RuntimeValue::Truth(appended));
            }
            Instruction::BufferAt {
                element,
                destination,
            } => {
                let index = pop_whole(&mut stack, decoded.offset, "buffer at")?;
                let buffer = pop_buffer(&mut stack, element, decoded.offset, "buffer at")?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime buffer lookup destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != element.element_type() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime buffer lookup destination is not its matching mutable value slot",
                    ));
                }
                let fallback = read_local(
                    &locals,
                    destination,
                    decoded.offset,
                    "buffer lookup destination",
                )?
                .clone();
                let (value, found) =
                    runtime_buffer_at(buffer, index, fallback, runtime_state, decoded.offset)?;
                locals[destination] = Some(value);
                stack.push(RuntimeValue::Truth(found));
            }
            Instruction::Count => {
                let buffer = pop_runtime(&mut stack, decoded.offset, "count")?;
                let RuntimeValue::Buffer { len, allocated, .. } = buffer else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "count received a non-buffer value",
                    ));
                };
                if !allocated {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "count received an unallocated buffer",
                    ));
                }
                stack.push(RuntimeValue::Whole(i64::try_from(len).map_err(|_| {
                    BytecodeError::new(decoded.offset, "buffer count is outside the Whole range")
                })?));
            }
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
                return Ok(RuntimeExit::Return(value));
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
            Instruction::Seek => {
                let start = pop_whole(&mut stack, decoded.offset, "seek")?;
                let needle = pop_text(&mut stack, decoded.offset, "seek")?;
                let text = pop_text(&mut stack, decoded.offset, "seek")?;
                let length = i64::try_from(text.chars().count()).map_err(|_| {
                    BytecodeError::new(decoded.offset, "text length is outside Whole range")
                })?;
                let start = start.clamp(0, length);
                let start = usize::try_from(start)
                    .map_err(|_| BytecodeError::new(decoded.offset, "seek start is invalid"))?;
                let start_byte = scalar_byte_offset(&text, start);
                let found = text[start_byte..].find(&needle).map_or(-1_i64, |offset| {
                    i64::try_from(text[..start_byte + offset].chars().count()).unwrap_or(-1)
                });
                stack.push(RuntimeValue::Whole(found));
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
            Instruction::Number => {
                let text = pop_text(&mut stack, decoded.offset, "number")?;
                if !is_whole_literal(&text) {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "number requires one canonical Whole text value",
                    ));
                }
                let value = text.parse::<i64>().map_err(|_| {
                    BytecodeError::new(decoded.offset, "number is outside the Whole range")
                })?;
                stack.push(RuntimeValue::Whole(value));
            }
            Instruction::Pack16 => {
                let value = pop_whole(&mut stack, decoded.offset, "pack16")?;
                let value = u16::try_from(value).map_err(|_| {
                    BytecodeError::new(
                        decoded.offset,
                        "pack16 requires a Whole between 0 and 65535",
                    )
                })?;
                stack.push(RuntimeValue::Bytes(value.to_le_bytes().to_vec()));
            }
            Instruction::Pack32 => {
                let value = pop_whole(&mut stack, decoded.offset, "pack32")?;
                let value = u32::try_from(value).map_err(|_| {
                    BytecodeError::new(
                        decoded.offset,
                        "pack32 requires a Whole between 0 and 4294967295",
                    )
                })?;
                stack.push(RuntimeValue::Bytes(value.to_le_bytes().to_vec()));
            }
            Instruction::Pack64 => {
                let value = pop_whole(&mut stack, decoded.offset, "pack64")?;
                stack.push(RuntimeValue::Bytes(value.to_le_bytes().to_vec()));
            }
            Instruction::Unpack16 => {
                let start = pop_whole(&mut stack, decoded.offset, "unpack16")?;
                let bytes = pop_bytes(&mut stack, decoded.offset, "unpack16")?;
                let start = checked_bytes_index(start, 2, bytes.len(), decoded.offset, "unpack16")?;
                let value = u16::from_le_bytes([bytes[start], bytes[start + 1]]);
                stack.push(RuntimeValue::Whole(i64::from(value)));
            }
            Instruction::Unpack32 => {
                let start = pop_whole(&mut stack, decoded.offset, "unpack32")?;
                let bytes = pop_bytes(&mut stack, decoded.offset, "unpack32")?;
                let start = checked_bytes_index(start, 4, bytes.len(), decoded.offset, "unpack32")?;
                let value = u32::from_le_bytes([
                    bytes[start],
                    bytes[start + 1],
                    bytes[start + 2],
                    bytes[start + 3],
                ]);
                stack.push(RuntimeValue::Whole(i64::from(value)));
            }
            Instruction::Poke => {
                let value = pop_whole(&mut stack, decoded.offset, "poke")?;
                let index = pop_whole(&mut stack, decoded.offset, "poke")?;
                let mut bytes = pop_bytes(&mut stack, decoded.offset, "poke")?;
                let index = checked_bytes_index(index, 1, bytes.len(), decoded.offset, "poke")?;
                let value = u8::try_from(value).map_err(|_| {
                    BytecodeError::new(decoded.offset, "poke requires a Whole between 0 and 255")
                })?;
                bytes[index] = value;
                stack.push(RuntimeValue::Bytes(bytes));
            }
            Instruction::Poke32 => {
                let value = pop_whole(&mut stack, decoded.offset, "poke32")?;
                let index = pop_whole(&mut stack, decoded.offset, "poke32")?;
                let mut bytes = pop_bytes(&mut stack, decoded.offset, "poke32")?;
                let index = checked_bytes_index(index, 4, bytes.len(), decoded.offset, "poke32")?;
                let value = u32::try_from(value).map_err(|_| {
                    BytecodeError::new(
                        decoded.offset,
                        "poke32 requires a Whole between 0 and 4294967295",
                    )
                })?;
                bytes[index..index + 4].copy_from_slice(&value.to_le_bytes());
                stack.push(RuntimeValue::Bytes(bytes));
            }
            Instruction::Render => {
                let value = pop_runtime(&mut stack, decoded.offset, "render")?;
                let text =
                    match value {
                        RuntimeValue::Text(value) => value,
                        RuntimeValue::Whole(value) => value.to_string(),
                        RuntimeValue::Truth(true) => "bright".to_owned(),
                        RuntimeValue::Truth(false) => "dim".to_owned(),
                        RuntimeValue::Bytes(_) => return Err(BytecodeError::new(
                            decoded.offset,
                            "render does not accept Bytes or records; project a record field first",
                        )),
                        RuntimeValue::Record { .. } => return Err(BytecodeError::new(
                            decoded.offset,
                            "render does not accept Bytes or records; project a record field first",
                        )),
                        RuntimeValue::Arena
                        | RuntimeValue::AccessArena
                        | RuntimeValue::Buffer { .. } => {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "render does not accept Aether resource values",
                            ))
                        }
                    };
                stack.push(RuntimeValue::Text(text));
            }
            Instruction::MakeRecord(record_id) => {
                let record = artifact_record(&artifact.records, record_id, decoded.offset)?;
                let mut fields = Vec::with_capacity(record.fields.len());
                for field in record.fields.iter().rev() {
                    let value = pop_runtime(&mut stack, decoded.offset, "make record")?;
                    require_runtime_type(&value, field.value_type, decoded.offset, "make record")?;
                    fields.push(value);
                }
                fields.reverse();
                ensure_record_size(&fields, decoded.offset)?;
                stack.push(RuntimeValue::Record {
                    record: record_id,
                    fields,
                });
            }
            Instruction::Field {
                record: record_id,
                field,
            } => {
                let record = artifact_record(&artifact.records, record_id, decoded.offset)?;
                let definition = record.fields.get(field).ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "field references an unknown record field")
                })?;
                let value = pop_runtime(&mut stack, decoded.offset, "field")?;
                let RuntimeValue::Record {
                    record: actual_record,
                    fields,
                } = value
                else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "field received a non-record value",
                    ));
                };
                if actual_record != record_id || fields.len() != record.fields.len() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "field received a record that disagrees with its declared schema",
                    ));
                }
                let value = fields.get(field).cloned().ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "field references an unknown record field")
                })?;
                require_runtime_type(&value, definition.value_type, decoded.offset, "field")?;
                stack.push(value);
            }
            Instruction::Raise => {
                if function.effect != Effect::ErrorWhole {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime raise occurred in a total weave",
                    ));
                }
                let code = pop_whole(&mut stack, decoded.offset, "raise")?;
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime raise left values on the operand stack",
                    ));
                }
                return Ok(RuntimeExit::ErrorWhole(code));
            }
            Instruction::ForwardCall {
                function: called,
                arguments,
            } => {
                if function.effect != Effect::ErrorWhole {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime forward occurred in a total weave",
                    ));
                }
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime forward references an unknown weave",
                    )
                })?;
                if called_function.effect != Effect::ErrorWhole
                    || called_function.result != function.result
                    || called_function.parameters.len() != arguments
                {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime forward signature is invalid",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, mode) in called_function.parameters.iter().rev() {
                    if *mode != ParameterMode::Own
                        || !matches!(value_type, ValueType::Whole | ValueType::Truth)
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime forward crosses a non-copy parameter boundary",
                        ));
                    }
                    let value = pop_runtime(&mut stack, decoded.offset, "forward")?;
                    require_runtime_type(&value, *value_type, decoded.offset, "forward")?;
                    values.push(value);
                }
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime forward left values on the operand stack",
                    ));
                }
                values.reverse();
                return execute_function(
                    artifact,
                    called,
                    values,
                    stdout,
                    runtime_state,
                    depth + 1,
                );
            }
            Instruction::HandleCall {
                function: called,
                arguments,
                success_destination,
                error_destination,
                success_target,
                error_target,
            } => {
                if function.effect != Effect::Total {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime handle occurred in an erroring weave",
                    ));
                }
                if success_destination == error_destination {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime handle requires distinct destinations",
                    ));
                }
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "runtime handle references an unknown weave")
                })?;
                if called_function.effect != Effect::ErrorWhole
                    || called_function.parameters.len() != arguments
                {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime handle signature is invalid",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, mode) in called_function.parameters.iter().rev() {
                    if *mode != ParameterMode::Own
                        || !matches!(value_type, ValueType::Whole | ValueType::Truth)
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime handle crosses a non-copy parameter boundary",
                        ));
                    }
                    let value = pop_runtime(&mut stack, decoded.offset, "handle")?;
                    require_runtime_type(&value, *value_type, decoded.offset, "handle")?;
                    values.push(value);
                }
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime handle left values on the operand stack",
                    ));
                }
                values.reverse();
                match execute_function(artifact, called, values, stdout, runtime_state, depth + 1)?
                {
                    RuntimeExit::Return(value) => {
                        let local = function.locals.get(success_destination).ok_or_else(|| {
                            BytecodeError::new(
                                decoded.offset,
                                "runtime handle success destination is invalid",
                            )
                        })?;
                        if !local.mutable || local.value_type != called_function.result {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime handle success destination is invalid",
                            ));
                        }
                        require_runtime_type(&value, local.value_type, decoded.offset, "handle")?;
                        if locals.get(success_destination).is_none()
                            || locals[success_destination].is_none()
                        {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime handle success destination is not live",
                            ));
                        }
                        locals[success_destination] = Some(value);
                        position = success_target;
                    }
                    RuntimeExit::ErrorWhole(code) => {
                        let local = function.locals.get(error_destination).ok_or_else(|| {
                            BytecodeError::new(
                                decoded.offset,
                                "runtime handle error destination is invalid",
                            )
                        })?;
                        if !local.mutable || local.value_type != ValueType::Whole {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime handle error destination is invalid",
                            ));
                        }
                        if locals.get(error_destination).is_none()
                            || locals[error_destination].is_none()
                        {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime handle error destination is not live",
                            ));
                        }
                        locals[error_destination] = Some(RuntimeValue::Whole(code));
                        position = error_target;
                    }
                }
            }
            Instruction::Call {
                function: called,
                arguments,
            } => {
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "runtime call references an unknown weave")
                })?;
                if called_function.effect != Effect::Total {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime ordinary call cannot invoke an Error[Whole] weave",
                    ));
                }
                if called_function.parameters.len() != arguments {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime call has an invalid argument count",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, mode) in called_function.parameters.iter().rev() {
                    let value = pop_runtime(&mut stack, decoded.offset, "call")?;
                    if *mode == ParameterMode::Access {
                        if !matches!(value, RuntimeValue::AccessArena) {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime call access argument is not an exclusive Arena access loan",
                            ));
                        }
                    } else {
                        require_runtime_type(&value, *value_type, decoded.offset, "call")?;
                    }
                    values.push(value);
                }
                values.reverse();
                let RuntimeExit::Return(value) =
                    execute_function(artifact, called, values, stdout, runtime_state, depth + 1)?
                else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime total call reached an Error[Whole] exit",
                    ));
                };
                stack.push(value);
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

fn decode_code(code: &[u8], version: u8) -> Result<Vec<DecodedInstruction>, BytecodeError> {
    let mut position = 0;
    let mut instructions = Vec::new();
    while position < code.len() {
        instructions.push(decode_instruction(code, &mut position, version)?);
    }
    Ok(instructions)
}

fn decode_instruction(
    code: &[u8],
    position: &mut usize,
    version: u8,
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
        OP_COMPTIME_WHOLE => {
            require_v8_instruction(version, offset, "compile-time Whole provenance")?;
            Instruction::ComptimeWhole(read_i64(code, position)?)
        }
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
        OP_SEEK => Instruction::Seek,
        OP_NUMBER => Instruction::Number,
        OP_PACK16 => Instruction::Pack16,
        OP_PACK32 => Instruction::Pack32,
        OP_UNPACK16 => Instruction::Unpack16,
        OP_UNPACK32 => Instruction::Unpack32,
        OP_POKE => Instruction::Poke,
        OP_POKE32 => Instruction::Poke32,
        OP_PACK64 => Instruction::Pack64,
        OP_MAKE_RECORD => {
            if !matches!(
                version,
                ARTIFACT_VERSION_V5
                    | ARTIFACT_VERSION_V6
                    | ARTIFACT_VERSION_V7
                    | ARTIFACT_VERSION_V8
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record construction is valid only in AETH v5, v6, v7, or v8 artifacts",
                ));
            }
            Instruction::MakeRecord(read_u16(code, position)?)
        }
        OP_FIELD => {
            if !matches!(
                version,
                ARTIFACT_VERSION_V5
                    | ARTIFACT_VERSION_V6
                    | ARTIFACT_VERSION_V7
                    | ARTIFACT_VERSION_V8
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record field projection is valid only in AETH v5, v6, v7, or v8 artifacts",
                ));
            }
            Instruction::Field {
                record: read_u16(code, position)?,
                field: usize::from(read_byte(code, position)?),
            }
        }
        OP_ARENA => {
            require_v6_instruction(version, offset, "arena declaration")?;
            Instruction::Arena
        }
        OP_BUFFER => {
            require_v6_instruction(version, offset, "buffer placeholder")?;
            Instruction::Buffer(BufferElement::from_type_tag(
                read_byte(code, position)?,
                offset,
            )?)
        }
        OP_ACCESS => {
            require_v6_instruction(version, offset, "arena access")?;
            Instruction::Access(usize::from(read_u16(code, position)?))
        }
        OP_ALLOCATE => {
            require_v6_instruction(version, offset, "buffer allocation")?;
            Instruction::Allocate {
                element: BufferElement::from_type_tag(read_byte(code, position)?, offset)?,
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_BUFFER_APPEND => {
            require_v6_instruction(version, offset, "buffer append")?;
            Instruction::BufferAppend {
                element: BufferElement::from_type_tag(read_byte(code, position)?, offset)?,
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_BUFFER_AT => {
            require_v6_instruction(version, offset, "buffer lookup")?;
            Instruction::BufferAt {
                element: BufferElement::from_type_tag(read_byte(code, position)?, offset)?,
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_COUNT => {
            require_v6_instruction(version, offset, "buffer count")?;
            Instruction::Count
        }
        OP_RAISE => {
            require_v7_instruction(version, offset, "raise")?;
            Instruction::Raise
        }
        OP_FORWARD_CALL => {
            require_v7_instruction(version, offset, "forward call")?;
            Instruction::ForwardCall {
                function: usize::from(read_u16(code, position)?),
                arguments: usize::from(read_byte(code, position)?),
            }
        }
        OP_HANDLE_CALL => {
            require_v7_instruction(version, offset, "handle call")?;
            Instruction::HandleCall {
                function: usize::from(read_u16(code, position)?),
                arguments: usize::from(read_byte(code, position)?),
                success_destination: usize::from(read_u16(code, position)?),
                error_destination: usize::from(read_u16(code, position)?),
                success_target: read_usize_u32(code, position)?,
                error_target: read_usize_u32(code, position)?,
            }
        }
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

fn require_v6_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(
        version,
        ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v6, v7, or v8 artifacts"),
        ))
    }
}

fn require_v7_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(version, ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v7 or v8 artifacts"),
        ))
    }
}

fn require_v8_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if version == ARTIFACT_VERSION_V8 {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v8 artifacts"),
        ))
    }
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

fn artifact_record(
    records: &[ArtifactRecord],
    record_id: u16,
    offset: usize,
) -> Result<&ArtifactRecord, BytecodeError> {
    records.get(usize::from(record_id)).ok_or_else(|| {
        BytecodeError::new(
            offset,
            "record identifier is outside the artifact record table",
        )
    })
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
    stack: &mut VerificationStack,
    expected: ValueType,
    offset: usize,
    operation: &str,
) -> Result<VerificationStackValue, BytecodeError> {
    let actual = pop_any_type(stack, offset, operation)?;
    if actual.value_type == expected {
        Ok(actual)
    } else {
        Err(BytecodeError::new(
            offset,
            format!(
                "{operation} requires {expected}, but artifact stack has {}",
                actual.value_type
            ),
        ))
    }
}

fn pop_any_type(
    stack: &mut VerificationStack,
    offset: usize,
    operation: &str,
) -> Result<VerificationStackValue, BytecodeError> {
    stack.pop().ok_or_else(|| {
        BytecodeError::new(
            offset,
            format!("{operation} would underflow the operand stack"),
        )
    })
}

fn require_moved_buffer_from(
    value: &VerificationStackValue,
    destination: usize,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if value.buffer_provenance == BufferProvenance::MovedLocal(destination) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} must receive the Buffer owner moved from its destination slot"),
        ))
    }
}

fn require_moved_buffer(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if matches!(value.buffer_provenance, BufferProvenance::MovedLocal(_)) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires a Buffer owner moved from a local slot"),
        ))
    }
}

fn require_borrowed_buffer(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if value.buffer_provenance == BufferProvenance::Borrowed {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires a transient borrowed Buffer value"),
        ))
    }
}

fn require_storable_buffer(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if matches!(
        value.buffer_provenance,
        BufferProvenance::Placeholder | BufferProvenance::MovedLocal(_)
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} cannot persist a borrowed Buffer value"),
        ))
    }
}

fn require_yieldable_buffer(
    value: &VerificationStackValue,
    offset: usize,
) -> Result<(), BytecodeError> {
    if matches!(value.buffer_provenance, BufferProvenance::MovedLocal(_)) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            "yield cannot expose a placeholder or borrowed Buffer value",
        ))
    }
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

fn pop_buffer(
    stack: &mut Vec<RuntimeValue>,
    element: BufferElement,
    offset: usize,
    operation: &str,
) -> Result<RuntimeValue, BytecodeError> {
    let value = pop_runtime(stack, offset, operation)?;
    match &value {
        RuntimeValue::Buffer {
            element: actual, ..
        } if *actual == element => Ok(value),
        _ => Err(BytecodeError::new(
            offset,
            format!(
                "{operation} received {}, expected {}",
                value.value_type(),
                element.value_type()
            ),
        )),
    }
}

fn pop_buffer_element(
    stack: &mut Vec<RuntimeValue>,
    element: BufferElement,
    offset: usize,
    operation: &str,
) -> Result<RuntimeValue, BytecodeError> {
    let value = pop_runtime(stack, offset, operation)?;
    require_runtime_type(&value, element.element_type(), offset, operation)?;
    Ok(value)
}

fn runtime_allocate_buffer(
    buffer: RuntimeValue,
    capacity: i64,
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Buffer {
        element, allocated, ..
    } = buffer
    else {
        return Err(BytecodeError::new(
            offset,
            "allocate received a non-buffer placeholder",
        ));
    };
    if allocated {
        return Err(BytecodeError::new(
            offset,
            "allocate requires an unallocated buffer placeholder",
        ));
    }
    let count = match usize::try_from(capacity) {
        Ok(value) => value,
        Err(_) => {
            return Ok((
                RuntimeValue::Buffer {
                    element,
                    allocated: false,
                    offset: 0,
                    len: 0,
                    capacity: 0,
                },
                false,
            ))
        }
    };
    let stride = buffer_element_stride(element);
    let payload = match count.checked_mul(stride) {
        Some(value) => value,
        None => {
            return Ok((
                RuntimeValue::Buffer {
                    element,
                    allocated: false,
                    offset: 0,
                    len: 0,
                    capacity: 0,
                },
                false,
            ))
        }
    };
    let required = match BUFFER_METADATA_BYTES.checked_add(payload) {
        Some(value) => value,
        None => {
            return Ok((
                RuntimeValue::Buffer {
                    element,
                    allocated: false,
                    offset: 0,
                    len: 0,
                    capacity: 0,
                },
                false,
            ))
        }
    };
    let Some(end) = runtime_state.arena.used.checked_add(required) else {
        return Ok((
            RuntimeValue::Buffer {
                element,
                allocated: false,
                offset: 0,
                len: 0,
                capacity: 0,
            },
            false,
        ));
    };
    if end > runtime_state.arena.bytes.len() {
        return Ok((
            RuntimeValue::Buffer {
                element,
                allocated: false,
                offset: 0,
                len: 0,
                capacity: 0,
            },
            false,
        ));
    }
    let offset = runtime_state
        .arena
        .used
        .checked_add(BUFFER_METADATA_BYTES)
        .ok_or_else(|| BytecodeError::new(offset, "buffer data offset overflowed"))?;
    runtime_state.arena.used = end;
    Ok((
        RuntimeValue::Buffer {
            element,
            allocated: true,
            offset,
            len: 0,
            capacity: count,
        },
        true,
    ))
}

fn runtime_append_buffer(
    buffer: RuntimeValue,
    value: RuntimeValue,
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Buffer {
        element,
        allocated,
        offset: data_offset,
        len,
        capacity,
    } = buffer
    else {
        return Err(BytecodeError::new(
            offset,
            "buffer append received a non-buffer value",
        ));
    };
    if !allocated || len >= capacity {
        return Ok((
            RuntimeValue::Buffer {
                element,
                allocated,
                offset: data_offset,
                len,
                capacity,
            },
            false,
        ));
    }
    let stride = buffer_element_stride(element);
    let write_offset = data_offset
        .checked_add(
            len.checked_mul(stride)
                .ok_or_else(|| BytecodeError::new(offset, "buffer append index overflowed"))?,
        )
        .ok_or_else(|| BytecodeError::new(offset, "buffer append offset overflowed"))?;
    let end = write_offset
        .checked_add(stride)
        .ok_or_else(|| BytecodeError::new(offset, "buffer append range overflowed"))?;
    let bytes = runtime_state
        .arena
        .bytes
        .get_mut(write_offset..end)
        .ok_or_else(|| {
            BytecodeError::new(offset, "buffer append range is outside the reserved arena")
        })?;
    match (element, value) {
        (BufferElement::Whole, RuntimeValue::Whole(value)) => {
            bytes.copy_from_slice(&value.to_le_bytes());
        }
        (BufferElement::Truth, RuntimeValue::Truth(value)) => {
            bytes[0] = u8::from(value);
        }
        (_, value) => {
            return Err(BytecodeError::new(
                offset,
                format!(
                    "buffer append received {}, expected {}",
                    value.value_type(),
                    element.element_type()
                ),
            ))
        }
    }
    Ok((
        RuntimeValue::Buffer {
            element,
            allocated: true,
            offset: data_offset,
            len: len + 1,
            capacity,
        },
        true,
    ))
}

fn runtime_buffer_at(
    buffer: RuntimeValue,
    index: i64,
    fallback: RuntimeValue,
    runtime_state: &RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Buffer {
        element,
        allocated,
        offset: data_offset,
        len,
        ..
    } = buffer
    else {
        return Err(BytecodeError::new(
            offset,
            "buffer at received a non-buffer value",
        ));
    };
    let Ok(index) = usize::try_from(index) else {
        return Ok((fallback, false));
    };
    if !allocated || index >= len {
        return Ok((fallback, false));
    }
    let stride = buffer_element_stride(element);
    let read_offset = data_offset
        .checked_add(
            index
                .checked_mul(stride)
                .ok_or_else(|| BytecodeError::new(offset, "buffer at index overflowed"))?,
        )
        .ok_or_else(|| BytecodeError::new(offset, "buffer at offset overflowed"))?;
    let end = read_offset
        .checked_add(stride)
        .ok_or_else(|| BytecodeError::new(offset, "buffer at range overflowed"))?;
    let bytes = runtime_state
        .arena
        .bytes
        .get(read_offset..end)
        .ok_or_else(|| {
            BytecodeError::new(offset, "buffer at range is outside the reserved arena")
        })?;
    let value = match element {
        BufferElement::Whole => {
            let mut data = [0_u8; 8];
            data.copy_from_slice(bytes);
            RuntimeValue::Whole(i64::from_le_bytes(data))
        }
        BufferElement::Truth => match bytes[0] {
            0 => RuntimeValue::Truth(false),
            1 => RuntimeValue::Truth(true),
            _ => {
                return Err(BytecodeError::new(
                    offset,
                    "buffer truth element is malformed inside the arena",
                ))
            }
        },
    };
    Ok((value, true))
}

const fn buffer_element_stride(element: BufferElement) -> usize {
    match element {
        BufferElement::Whole => std::mem::size_of::<i64>(),
        BufferElement::Truth => 1,
    }
}

fn runtime_values_equal(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match (left, right) {
        (RuntimeValue::Text(left), RuntimeValue::Text(right)) => left == right,
        (RuntimeValue::Whole(left), RuntimeValue::Whole(right)) => left == right,
        (RuntimeValue::Truth(left), RuntimeValue::Truth(right)) => left == right,
        (RuntimeValue::Bytes(left), RuntimeValue::Bytes(right)) => left == right,
        (
            RuntimeValue::Record {
                record: left_record,
                fields: left_fields,
            },
            RuntimeValue::Record {
                record: right_record,
                fields: right_fields,
            },
        ) => {
            left_record == right_record
                && left_fields.len() == right_fields.len()
                && left_fields
                    .iter()
                    .zip(right_fields)
                    .all(|(left, right)| runtime_values_equal(left, right))
        }
        (
            RuntimeValue::Buffer {
                element: left_element,
                allocated: left_allocated,
                offset: left_offset,
                len: left_len,
                capacity: left_capacity,
            },
            RuntimeValue::Buffer {
                element: right_element,
                allocated: right_allocated,
                offset: right_offset,
                len: right_len,
                capacity: right_capacity,
            },
        ) => {
            left_element == right_element
                && left_allocated == right_allocated
                && left_offset == right_offset
                && left_len == right_len
                && left_capacity == right_capacity
        }
        (RuntimeValue::Arena, RuntimeValue::Arena)
        | (RuntimeValue::AccessArena, RuntimeValue::AccessArena) => true,
        _ => false,
    }
}

fn ensure_record_size(fields: &[RuntimeValue], offset: usize) -> Result<(), BytecodeError> {
    let mut size = 0_usize;
    for field in fields {
        size = size
            .checked_add(runtime_value_size(field, offset)?)
            .ok_or_else(|| {
                BytecodeError::new(offset, "record size overflowed the runtime safety limit")
            })?;
        if size > MAX_RECORD_BYTES {
            return Err(BytecodeError::new(
                offset,
                "record construction exceeds the Aether runtime safety limit",
            ));
        }
    }
    Ok(())
}

fn runtime_value_size(value: &RuntimeValue, offset: usize) -> Result<usize, BytecodeError> {
    match value {
        RuntimeValue::Text(value) => Ok(value.len()),
        RuntimeValue::Whole(_) => Ok(std::mem::size_of::<i64>()),
        RuntimeValue::Truth(_) => Ok(1),
        RuntimeValue::Bytes(value) => Ok(value.len()),
        RuntimeValue::Record { fields, .. } => {
            let mut size = 0_usize;
            for field in fields {
                size = size
                    .checked_add(runtime_value_size(field, offset)?)
                    .ok_or_else(|| {
                        BytecodeError::new(
                            offset,
                            "record size overflowed the runtime safety limit",
                        )
                    })?;
                if size > MAX_RECORD_BYTES {
                    return Err(BytecodeError::new(
                        offset,
                        "record exceeds the Aether runtime safety limit",
                    ));
                }
            }
            Ok(size)
        }
        RuntimeValue::Arena | RuntimeValue::AccessArena | RuntimeValue::Buffer { .. } => {
            Err(BytecodeError::new(
                offset,
                "Aether resource values cannot be measured as record payloads",
            ))
        }
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

fn read_value_type(
    bytes: &[u8],
    position: &mut usize,
    version: u8,
    record_count: usize,
) -> Result<ValueType, BytecodeError> {
    let offset = *position;
    let tag = read_byte(bytes, position)?;
    match tag {
        1..=4 => ValueType::from_primitive_tag(tag, offset),
        5 => {
            if !matches!(
                version,
                ARTIFACT_VERSION_V5
                    | ARTIFACT_VERSION_V6
                    | ARTIFACT_VERSION_V7
                    | ARTIFACT_VERSION_V8
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record types are valid only in AETH v5, v6, v7, or v8 artifacts",
                ));
            }
            let record_id = read_u16(bytes, position)?;
            if usize::from(record_id) >= record_count {
                return Err(BytecodeError::new(
                    offset,
                    "record type identifier is outside the artifact record table",
                ));
            }
            Ok(ValueType::Record(record_id))
        }
        6 if matches!(
            version,
            ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
        ) =>
        {
            Ok(ValueType::Arena)
        }
        7 if matches!(
            version,
            ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
        ) =>
        {
            Ok(ValueType::BufferWhole)
        }
        8 if matches!(
            version,
            ARTIFACT_VERSION_V6 | ARTIFACT_VERSION_V7 | ARTIFACT_VERSION_V8
        ) =>
        {
            Ok(ValueType::BufferTruth)
        }
        9 => Err(BytecodeError::new(
            offset,
            "access loans are verifier-internal and cannot be serialized",
        )),
        6..=8 => Err(BytecodeError::new(
            offset,
            "resource value types are valid only in AETH v6, v7, or v8 artifacts",
        )),
        _ => Err(BytecodeError::new(offset, "unknown Aether value type")),
    }
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

fn checked_bytes_index(
    index: i64,
    width: usize,
    length: usize,
    offset: usize,
    operation: &str,
) -> Result<usize, BytecodeError> {
    let index = usize::try_from(index)
        .map_err(|_| BytecodeError::new(offset, format!("{operation} index is invalid")))?;
    let end = index
        .checked_add(width)
        .ok_or_else(|| BytecodeError::new(offset, format!("{operation} index overflowed")))?;
    if end > length {
        return Err(BytecodeError::new(
            offset,
            format!("{operation} range is outside the Bytes value"),
        ));
    }
    Ok(index)
}

fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_value_type(bytes: &mut Vec<u8>, value_type: ValueType) {
    debug_assert_ne!(value_type, ValueType::AccessArena);
    bytes.push(value_type.to_tag());
    if let ValueType::Record(record_id) = value_type {
        write_u16(bytes, record_id);
    }
}

fn write_primitive_value_type(
    bytes: &mut Vec<u8>,
    value_type: ValueType,
    span: Span,
) -> Result<(), CompilerError> {
    if matches!(
        value_type,
        ValueType::Record(_)
            | ValueType::Arena
            | ValueType::BufferWhole
            | ValueType::BufferTruth
            | ValueType::AccessArena
    ) {
        return Err(CompilerError::new(
            span,
            "record fields may use only Text, Whole, Truth, or Bytes",
        ));
    }
    bytes.push(value_type.to_tag());
    Ok(())
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
                comptime,
                value,
                ..
            } => {
                if *comptime {
                    output.push_str("comptime bind ");
                } else {
                    output.push_str("bind ");
                }
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
            Statement::Raise { code, .. } => {
                output.push_str("raise ");
                write_atom(code, output);
                output.push('\n');
            }
            Statement::Forward {
                weave, arguments, ..
            } => {
                output.push_str("forward call ");
                output.push_str(weave);
                for argument in arguments {
                    output.push(' ');
                    write_atom(argument, output);
                }
                output.push('\n');
            }
            Statement::Handle {
                weave,
                arguments,
                success_destination,
                error_destination,
                ..
            } => {
                output.push_str("handle call ");
                output.push_str(weave);
                for argument in arguments {
                    output.push(' ');
                    write_atom(argument, output);
                }
                output.push_str(" into ");
                output.push_str(success_destination);
                output.push_str(" otherwise error into ");
                output.push_str(error_destination);
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
        ExpressionKind::Arena { capacity } => {
            output.push_str("arena ");
            output.push_str(&capacity.to_string());
        }
        ExpressionKind::Buffer { element } => {
            output.push_str("buffer ");
            output.push_str(element.word());
        }
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
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => {
            output.push_str(operation.word());
            output.push(' ');
            write_atom(first, output);
            output.push(' ');
            write_atom(second, output);
            output.push(' ');
            write_atom(third, output);
        }
        ExpressionKind::Call { weave, arguments } => {
            output.push_str("call ");
            output.push_str(weave);
            for argument in arguments {
                output.push(' ');
                write_atom(argument, output);
            }
        }
        ExpressionKind::MakeRecord { record, fields } => {
            output.push_str("make ");
            output.push_str(record);
            for field in fields {
                output.push(' ');
                write_atom(field, output);
            }
        }
        ExpressionKind::Field { record, field } => {
            output.push_str("field ");
            write_atom(record, output);
            output.push(' ');
            output.push_str(field);
        }
        ExpressionKind::Resource(operation) => write_resource_operation(operation, output),
    }
}

fn write_resource_operation(operation: &ResourceOperation, output: &mut String) {
    match operation {
        ResourceOperation::Allocate {
            arena,
            buffer,
            capacity,
            destination,
        } => {
            output.push_str("allocate ");
            write_atom(arena, output);
            output.push(' ');
            write_atom(buffer, output);
            output.push(' ');
            write_atom(capacity, output);
            output.push_str(" into ");
            output.push_str(destination);
        }
        ResourceOperation::Append {
            buffer,
            value,
            destination,
        } => {
            output.push_str("append ");
            write_atom(buffer, output);
            output.push(' ');
            write_atom(value, output);
            output.push_str(" into ");
            output.push_str(destination);
        }
        ResourceOperation::At {
            buffer,
            index,
            destination,
        } => {
            output.push_str("at ");
            write_atom(buffer, output);
            output.push(' ');
            write_atom(index, output);
            output.push_str(" into ");
            output.push_str(destination);
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
        AtomKind::Access(name) => {
            output.push_str("access ");
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
                comptime,
                value,
                ..
            } => {
                output.push_str(if *comptime {
                    "ComptimeBind("
                } else if *mutable {
                    "BindMutable("
                } else {
                    "Bind("
                });
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
            Statement::Raise { code, .. } => {
                output.push_str("Raise(");
                write_ast_atom(code, output);
                output.push(')');
            }
            Statement::Forward {
                weave, arguments, ..
            } => {
                output.push_str("Forward(");
                output.push_str(weave);
                for argument in arguments {
                    output.push(',');
                    write_ast_atom(argument, output);
                }
                output.push(')');
            }
            Statement::Handle {
                weave,
                arguments,
                success_destination,
                error_destination,
                ..
            } => {
                output.push_str("Handle(");
                output.push_str(weave);
                output.push(',');
                output.push_str(success_destination);
                output.push(',');
                output.push_str(error_destination);
                for argument in arguments {
                    output.push(',');
                    write_ast_atom(argument, output);
                }
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
        ExpressionKind::Arena { capacity } => {
            output.push_str("Arena(");
            output.push_str(&capacity.to_string());
            output.push(')');
        }
        ExpressionKind::Buffer { element } => {
            output.push_str("Buffer(");
            output.push_str(element.word());
            output.push(')');
        }
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
        ExpressionKind::Ternary {
            operation,
            first,
            second,
            third,
        } => {
            output.push_str(operation.word());
            output.push('(');
            write_ast_atom(first, output);
            output.push(',');
            write_ast_atom(second, output);
            output.push(',');
            write_ast_atom(third, output);
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
        ExpressionKind::MakeRecord { record, fields } => {
            output.push_str("make(");
            output.push_str(record);
            for field in fields {
                output.push(',');
                write_ast_atom(field, output);
            }
            output.push(')');
        }
        ExpressionKind::Field { record, field } => {
            output.push_str("field(");
            write_ast_atom(record, output);
            output.push(',');
            output.push_str(field);
            output.push(')');
        }
        ExpressionKind::Resource(operation) => {
            output.push_str("Outcome(");
            match operation {
                ResourceOperation::Allocate {
                    arena,
                    buffer,
                    capacity,
                    destination,
                } => {
                    output.push_str("allocate,");
                    write_ast_atom(arena, output);
                    output.push(',');
                    write_ast_atom(buffer, output);
                    output.push(',');
                    write_ast_atom(capacity, output);
                    output.push(',');
                    output.push_str(destination);
                }
                ResourceOperation::Append {
                    buffer,
                    value,
                    destination,
                } => {
                    output.push_str("append,");
                    write_ast_atom(buffer, output);
                    output.push(',');
                    write_ast_atom(value, output);
                    output.push(',');
                    output.push_str(destination);
                }
                ResourceOperation::At {
                    buffer,
                    index,
                    destination,
                } => {
                    output.push_str("at,");
                    write_ast_atom(buffer, output);
                    output.push(',');
                    write_ast_atom(index, output);
                    output.push(',');
                    output.push_str(destination);
                }
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
        AtomKind::Access(name) => {
            output.push_str("Access(");
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
            Statement::Raise { .. } | Statement::Forward { .. } => 2,
            Statement::Handle { .. } => 4,
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
            | "record"
            | "bind"
            | "mutable"
            | "revise"
            | "speak"
            | "yield"
            | "raise"
            | "forward"
            | "handle"
            | "raises"
            | "error"
            | "into"
            | "choose"
            | "otherwise"
            | "while"
            | "call"
            | "make"
            | "field"
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
            | "seek"
            | "number"
            | "pack16"
            | "pack32"
            | "pack64"
            | "unpack16"
            | "unpack32"
            | "poke"
            | "poke32"
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

fn validate_artifact_name(
    name: &str,
    offset: usize,
    subject: &str,
    allow_main: bool,
) -> Result<(), BytecodeError> {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return Err(BytecodeError::new(
            offset,
            format!("artifact {subject} is required"),
        ));
    };
    if !first.is_ascii_lowercase()
        || !characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
    {
        return Err(BytecodeError::new(
            offset,
            format!("artifact {subject} is not a canonical Aether name"),
        ));
    }
    let reserved = matches!(
        name,
        "world"
            | "weave"
            | "record"
            | "bind"
            | "mutable"
            | "revise"
            | "speak"
            | "yield"
            | "raise"
            | "forward"
            | "handle"
            | "raises"
            | "error"
            | "into"
            | "choose"
            | "otherwise"
            | "while"
            | "call"
            | "make"
            | "field"
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
            | "seek"
            | "number"
            | "pack16"
            | "pack32"
            | "pack64"
            | "unpack16"
            | "unpack32"
            | "poke"
            | "poke32"
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
        return Err(BytecodeError::new(
            offset,
            format!("artifact {subject} uses a reserved Aether word"),
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
    fn compiles_runs_and_formats_legacy_source_in_current_aeth_v8() {
        let output = compile_to_bytecode(HELLO).expect("Aether source should compile");
        let run = run_bytecode(&output.bytecode).expect("Aether artifact should run");
        assert_eq!(run.stdout, "Hello from Aether\n");
        assert_eq!(run.exit_code, 0);
        assert_eq!(format_program(&output.program), HELLO);
        assert!(canonical_ast(&output.program).contains("Borrow(greeting)"));
        assert_eq!(&output.bytecode[..5], b"AETH\x08");
    }

    #[test]
    fn runs_bounded_search_packing_and_binary_patching_primitives() {
        let source = "world forge_tools\n\nweave main [] -> Whole:\n  bind sample <- \"aéabc\"\n  bind found <- seek borrow sample \"abc\" 0\n  bind parsed <- number \"42\"\n  bind packed16 <- pack16 4660\n  bind unpacked16 <- unpack16 borrow packed16 0\n  bind packed32 <- pack32 16909060\n  bind patched <- poke borrow packed32 1 255\n  bind patched_value <- unpack32 borrow patched 0\n  bind restored <- poke32 borrow patched 0 16909060\n  bind restored_value <- unpack32 borrow restored 0\n  bind score <- sum found parsed\n  bind score2 <- sum score unpacked16\n  bind score3 <- sum score2 patched_value\n  yield sum score3 restored_value\n";
        let output = compile_to_bytecode(source).expect("current primitives should compile");
        let run = run_bytecode(&output.bytecode).expect("current primitive artifact should run");
        assert_eq!(run.exit_code, 33_887_336);
    }

    #[test]
    fn rejects_invalid_numeric_and_binary_ranges_at_runtime() {
        let invalid_number = "world invalid\n\nweave main [] -> Whole:\n  bind value <- number \"01\"\n  yield value\n";
        let artifact = compile_to_bytecode(invalid_number)
            .expect("number operand shape is checked at runtime")
            .bytecode;
        let error = run_bytecode(&artifact).expect_err("noncanonical numeric text must fail");
        assert!(error.message.contains("canonical Whole"));

        let invalid_pack = "world invalid\n\nweave main [] -> Whole:\n  bind value <- pack16 65536\n  bind length <- extent borrow value\n  yield length\n";
        let artifact = compile_to_bytecode(invalid_pack)
            .expect("pack operand shape is checked at runtime")
            .bytecode;
        let error = run_bytecode(&artifact).expect_err("out-of-range pack16 must fail");
        assert!(error.message.contains("between 0 and 65535"));

        let invalid_poke = "world invalid\n\nweave main [] -> Whole:\n  bind value <- poke bytes \"00\" 1 0\n  bind length <- extent borrow value\n  yield length\n";
        let artifact = compile_to_bytecode(invalid_poke)
            .expect("poke operand shapes should compile")
            .bytecode;
        let error = run_bytecode(&artifact).expect_err("out-of-range poke must fail");
        assert!(error.message.contains("outside the Bytes value"));
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
        assert_eq!(&first.bytecode[..5], b"AETH\x08");
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

        let record_source = "world records\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card 7\n  bind score <- field borrow value score\n  yield score\n";
        let mut artifact = compile_to_bytecode(record_source)
            .expect("record source should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V5;
        artifact.drain(5..9).for_each(drop);
        // The fixture has one primitive record and one parameterless `main`.
        // v5 has the same record/function layout as v8 except that it has no
        // arena header and no per-weave effect byte.
        let effect_offset =
            5 + 2 + 1 + "card".len() + 1 + 1 + "score".len() + 1 + 2 + 1 + "main".len() + 1 + 1;
        assert_eq!(artifact[effect_offset], 0, "fixture main must be total");
        artifact.remove(effect_offset);
        verify_bytecode(&artifact).expect("the translated v5 compatibility fixture should verify");
        let field_offset = artifact
            .iter()
            .position(|byte| *byte == OP_FIELD)
            .expect("fixture should contain a record field opcode");
        artifact[field_offset] = OP_ARENA;
        let error = verify_bytecode(&artifact)
            .expect_err("v5 artifacts must reject v6-only resource instructions");
        assert!(error.message.contains("valid only in AETH v6"));
    }

    #[test]
    fn accepts_windows_line_endings_but_formats_canonically() {
        let windows_source = HELLO.replace('\n', "\r\n");
        let program = compile_source(&windows_source).expect("CRLF source should compile");
        assert_eq!(format_program(&program), HELLO);
    }

    #[test]
    fn compiles_runs_and_formats_immutable_records_in_current_aeth_v8() {
        let source = "world records\n\nrecord card [label: Text, score: Whole, payload: Bytes, active: Truth]\n\nweave inspect [borrow value: card] -> Whole:\n  bind score <- field borrow value score\n  yield score\n\nweave main [] -> Whole:\n  bind card_value <- make card \"Aether\" 7 bytes \"0102\" bright\n  bind label <- field borrow card_value label\n  speak borrow label\n  bind score <- call inspect borrow card_value\n  bind duplicate <- make card \"Aether\" 7 bytes \"0102\" bright\n  bind equal <- same borrow card_value borrow duplicate\n  bind mutable result <- score\n  choose equal:\n    revise result <- sum result 1\n  yield result\n";
        let output = compile_to_bytecode(source).expect("record source should compile");
        let run = run_bytecode(&output.bytecode).expect("record artifact should run");
        assert_eq!(run.stdout, "Aether");
        assert_eq!(run.exit_code, 8);
        assert_eq!(&output.bytecode[..5], b"AETH\x08");
        assert_eq!(format_program(&output.program), source);
        assert!(canonical_ast(&output.program).contains("Record(card)[label:Text"));

        let host_error = invoke_bytecode(&output.bytecode, "inspect", &[InvocationValue::Whole(7)])
            .expect_err("hosts must not synthesize Aether record arguments");
        assert!(host_error
            .message
            .contains("host invocation accepts only primitive"));
    }

    #[test]
    fn rejects_invalid_record_source_and_malformed_v5_field_projection() {
        let nested = "world invalid\n\nrecord inner [value: Whole]\nrecord outer [item: inner]\n\nweave main [] -> Whole:\n  yield 0\n";
        let error =
            compile_source(nested).expect_err("record fields are deliberately primitive-only");
        assert!(error.message.contains("record fields may use only"));

        let implicit_projection = "world invalid\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card 7\n  bind score <- field value score\n  yield score\n";
        let error = compile_source(implicit_projection)
            .expect_err("record fields require an explicit borrow projection");
        assert!(error.message.contains("field requires borrow"));

        let source = "world invalid\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card 7\n  bind score <- field borrow value score\n  yield score\n";
        let mut artifact = compile_to_bytecode(source)
            .expect("record fixture should compile")
            .bytecode;
        let field_opcode = artifact
            .iter()
            .position(|byte| *byte == OP_FIELD)
            .expect("fixture should contain OP_FIELD");
        artifact[field_opcode + 1] = u8::MAX;
        artifact[field_opcode + 2] = u8::MAX;
        let error = verify_bytecode(&artifact)
            .expect_err("a projection must reference a declared record identifier");
        assert!(error.message.contains("record identifier"));
    }

    #[test]
    fn runs_bounded_arena_buffer_operations() {
        let source = "world arena_buffer\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  bind mutable observed <- 0\n  choose allocate access memory move values 2 into values:\n    choose append move values 7 into values:\n      choose at borrow values 0 into observed:\n        yield observed\n      otherwise:\n        yield -3\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
        let output = compile_to_bytecode(source).expect("arena-buffer source should compile");
        assert_eq!(&output.bytecode[..5], b"AETH\x08");
        assert_eq!(
            u32::from_le_bytes(output.bytecode[5..9].try_into().expect("v8 capacity bytes")),
            64
        );
        let run = run_bytecode(&output.bytecode).expect("arena-buffer artifact should run");
        assert_eq!(run.exit_code, 7);

        let count_source = "world buffer_count\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 2 into values:\n    choose append move values 7 into values:\n      yield count borrow values\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
        let count_artifact = compile_to_bytecode(count_source)
            .expect("count source should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&count_artifact)
                .expect("count artifact should run")
                .exit_code,
            1
        );
    }

    #[test]
    fn resource_outcomes_preserve_owners_and_copy_lookup_fallbacks() {
        let exhausted = "world exhausted\n\nweave main [] -> Whole:\n  bind memory <- arena 31\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 2 into values:\n    yield 1\n  otherwise:\n    yield -1\n";
        let exhausted_artifact = compile_to_bytecode(exhausted)
            .expect("exhaustion fixture should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&exhausted_artifact)
                .expect("exhaustion fixture should run")
                .exit_code,
            -1
        );

        let full = "world full\n\nweave main [] -> Whole:\n  bind memory <- arena 24\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 1 into values:\n    choose append move values 7 into values:\n      choose append move values 8 into values:\n        yield 1\n      otherwise:\n        yield -2\n    otherwise:\n      yield -3\n  otherwise:\n    yield -1\n";
        let full_artifact = compile_to_bytecode(full)
            .expect("full-buffer fixture should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&full_artifact)
                .expect("full-buffer fixture should run")
                .exit_code,
            -2
        );

        let absent = "world absent\n\nweave main [] -> Whole:\n  bind memory <- arena 24\n  bind mutable values <- buffer Whole\n  bind mutable observed <- 99\n  choose allocate access memory move values 1 into values:\n    choose append move values 7 into values:\n      choose at borrow values 1 into observed:\n        yield -4\n      otherwise:\n        yield observed\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
        let absent_artifact = compile_to_bytecode(absent)
            .expect("absent-lookup fixture should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&absent_artifact)
                .expect("absent-lookup fixture should run")
                .exit_code,
            99
        );

        let truth = "world truth_buffer\n\nweave main [] -> Whole:\n  bind memory <- arena 17\n  bind mutable flags <- buffer Truth\n  bind mutable observed <- dim\n  choose allocate access memory move flags 1 into flags:\n    choose append move flags bright into flags:\n      choose at borrow flags 0 into observed:\n        yield 1\n      otherwise:\n        yield -3\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
        let truth_artifact = compile_to_bytecode(truth)
            .expect("truth-buffer fixture should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&truth_artifact)
                .expect("truth-buffer fixture should run")
                .exit_code,
            1
        );
    }

    #[test]
    fn passes_arena_access_through_a_named_weave() {
        let source = "world access_call\n\nweave provision [access memory: Arena] -> Whole:\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 1 into values:\n    choose append move values 42 into values:\n      yield 1\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n\nweave main [] -> Whole:\n  bind memory <- arena 24\n  yield call provision access memory\n";
        let output = compile_to_bytecode(source).expect("access-call source should compile");
        let run = run_bytecode(&output.bytecode).expect("access-call artifact should run");
        assert_eq!(run.exit_code, 1);
    }

    #[test]
    fn rejects_invalid_resource_source_shapes() {
        let oversized_arena =
            "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 1000001\n  yield 0\n";
        let error = compile_source(oversized_arena).expect_err("arena capacity must be bounded");
        assert!(error.message.contains("between 1"));

        let unsupported_element = "world invalid\n\nweave main [] -> Whole:\n  bind mutable values <- buffer Text\n  yield 0\n";
        let error = compile_source(unsupported_element)
            .expect_err("buffers may contain only Whole or Truth values");
        assert!(error.message.contains("only Whole or Truth"));

        let mismatched_destination = "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  bind mutable other <- buffer Whole\n  choose allocate access memory move values 1 into other:\n    yield 1\n  otherwise:\n    yield 0\n";
        let error = compile_source(mismatched_destination)
            .expect_err("allocation must restore the moved owner into itself");
        assert!(error.message.contains("same Buffer binding"));

        let unallocated_borrow = "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  bind size <- count borrow values\n  yield size\n";
        let error =
            compile_source(unallocated_borrow).expect_err("unallocated buffers cannot be borrowed");
        assert!(error.message.contains("unallocated"));

        let nonterminal_outcome = "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 1 into values:\n    yield 1\n  otherwise:\n    yield 0\n  yield 2\n";
        let error = compile_source(nonterminal_outcome)
            .expect_err("resource outcomes must be handled as terminal choices");
        assert!(error.message.contains("final statement"));

        let helper_arena = "world invalid\n\nweave helper [] -> Whole:\n  bind memory <- arena 64\n  yield 0\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(helper_arena)
            .expect_err("a local helper cannot manufacture an escaping arena region");
        assert!(error.message.contains("weave main"));

        let revise_buffer = "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable values <- buffer Whole\n  revise values <- buffer Whole\n  yield 0\n";
        let error = compile_source(revise_buffer)
            .expect_err("buffer owners have only closed replacement operations in M2");
        assert!(error.message.contains("closed resource outcomes"));

        let buffer_result = "world invalid\n\nweave create [access memory: Arena] -> BufferWhole:\n  bind mutable values <- buffer Whole\n  choose allocate access memory move values 1 into values:\n    yield move values\n  otherwise:\n    yield move values\n\nweave main [] -> Whole:\n  bind memory <- arena 24\n  yield 0\n";
        let error = compile_source(buffer_result)
            .expect_err("buffer ownership cannot cross a result before outcome propagation exists");
        assert!(error.message.contains("cannot yet cross a weave result"));
    }

    #[test]
    fn verifier_rejects_hostile_resource_artifacts() {
        let source = include_str!("../../../examples/arena-buffer.ae");
        let artifact = compile_to_bytecode(source)
            .expect("resource source should compile")
            .bytecode;

        let mut oversized = artifact.clone();
        oversized[5..9].copy_from_slice(&(MAX_ARENA_BYTES + 1).to_le_bytes());
        let error = verify_bytecode(&oversized)
            .expect_err("an oversized resource plan must fail before execution");
        assert!(error.message.contains("safety limit"));

        let mut invalid_destination = artifact.clone();
        let allocate_offset = invalid_destination
            .iter()
            .position(|byte| *byte == OP_ALLOCATE)
            .expect("fixture should contain a buffer allocation opcode");
        invalid_destination[allocate_offset + 2] = u8::MAX;
        invalid_destination[allocate_offset + 3] = u8::MAX;
        let error = verify_bytecode(&invalid_destination)
            .expect_err("a resource destination must be a declared local slot");
        assert!(error.message.contains("outside the local table"));

        let mut unplanned = compile_to_bytecode(HELLO)
            .expect("legacy fixture should compile")
            .bytecode;
        let yield_offset = unplanned
            .iter()
            .rposition(|byte| *byte == OP_YIELD)
            .expect("fixture should contain a yield opcode");
        unplanned[yield_offset] = OP_ARENA;
        let error = verify_bytecode(&unplanned)
            .expect_err("a zero-capacity artifact cannot introduce arena capability bytecode");
        assert!(error.message.contains("nonzero arena resource plan"));

        let mut forged_owner = Vec::from(&ARTIFACT_MAGIC[..]);
        forged_owner.push(ARTIFACT_VERSION_V6);
        write_u32(&mut forged_owner, 64);
        write_u16(&mut forged_owner, 0);
        write_u16(&mut forged_owner, 1);
        forged_owner.push(4);
        forged_owner.extend_from_slice(b"main");
        forged_owner.push(0);
        forged_owner.push(ValueType::Whole.to_tag());
        write_u16(&mut forged_owner, 3);
        forged_owner.push(ValueType::Arena.to_tag());
        forged_owner.push(0);
        forged_owner.push(ValueType::BufferWhole.to_tag());
        forged_owner.push(1);
        forged_owner.push(ValueType::BufferWhole.to_tag());
        forged_owner.push(0);

        let mut code = vec![OP_ARENA, OP_STORE];
        write_u16(&mut code, 0);
        code.extend_from_slice(&[OP_BUFFER, BufferElement::Whole.type_tag(), OP_STORE]);
        write_u16(&mut code, 1);
        code.push(OP_MOVE);
        write_u16(&mut code, 1);
        code.push(OP_STORE);
        write_u16(&mut code, 2);
        code.push(OP_ACCESS);
        write_u16(&mut code, 0);
        code.extend_from_slice(&[OP_BUFFER, BufferElement::Whole.type_tag(), OP_PUSH_WHOLE]);
        code.extend_from_slice(&1_i64.to_le_bytes());
        code.extend_from_slice(&[OP_ALLOCATE, BufferElement::Whole.type_tag()]);
        write_u16(&mut code, 1);
        code.push(OP_JUMP_IF_DIM);
        let dim_target = reserve_u32(&mut code);
        code.push(OP_PUSH_WHOLE);
        code.extend_from_slice(&1_i64.to_le_bytes());
        code.push(OP_YIELD);
        let dim_offset = code.len();
        patch_u32(&mut code, dim_target, dim_offset)
            .expect("crafted jump target should fit in the test artifact");
        code.push(OP_PUSH_WHOLE);
        code.extend_from_slice(&0_i64.to_le_bytes());
        code.push(OP_YIELD);
        write_u32(
            &mut forged_owner,
            u32::try_from(code.len()).expect("crafted code length should fit"),
        );
        forged_owner.extend_from_slice(&code);

        let error = verify_bytecode(&forged_owner)
            .expect_err("a forged placeholder cannot replace the moved Buffer owner");
        assert!(error.message.contains("moved from its destination slot"));
    }

    #[test]
    fn seed_matches_bootstrap_for_the_canonical_m2_resource_corpus() {
        let corpus = [
            (
                "arena-buffer",
                include_str!("../../../examples/arena-buffer.ae"),
                7,
            ),
            (
                "arena-exhausted",
                include_str!("../../../examples/arena-exhausted.ae"),
                -1,
            ),
            (
                "arena-full",
                include_str!("../../../examples/arena-full.ae"),
                -2,
            ),
            (
                "arena-lookup-fallback",
                include_str!("../../../examples/arena-lookup-fallback.ae"),
                99,
            ),
            (
                "arena-truth-buffer",
                include_str!("../../../examples/arena-truth-buffer.ae"),
                1,
            ),
            (
                "arena-access-weave",
                include_str!("../../../examples/arena-access-weave.ae"),
                1,
            ),
        ];
        let compiler = compile_to_bytecode(include_str!("../../../seed/aether_seed.ae"))
            .expect("the seed source should bootstrap")
            .bytecode;
        for (name, source, expected_exit_code) in corpus {
            let forged = forge_bytecode(&compiler, source)
                .unwrap_or_else(|error| panic!("the seed compiler must forge {name}: {error}"));
            let InvocationValue::Bytes(generated) = forged.value else {
                panic!("the seed compiler must return AETH bytes for {name}");
            };
            let bootstrap = compile_to_bytecode(source)
                .unwrap_or_else(|error| panic!("the bootstrap must compile {name}: {error}"))
                .bytecode;
            assert_eq!(generated, bootstrap, "seed and bootstrap differ for {name}");
            verify_bytecode(&generated).unwrap_or_else(|error| {
                panic!("the seed-produced {name} artifact must verify: {error}")
            });
            assert_eq!(
                run_bytecode(&generated)
                    .unwrap_or_else(|error| panic!(
                        "the seed-produced {name} artifact must run: {error}"
                    ))
                    .exit_code,
                expected_exit_code,
                "the canonical M2 fixture {name} returned the wrong outcome"
            );
        }
    }

    #[test]
    fn compiles_verifies_and_runs_the_bounded_m5_comptime_bindings() {
        let source = "world comptime_math\n\nweave main [] -> Whole:\n  comptime bind table_width <- product 16 8\n  comptime bind header_size <- sum 12 4\n  comptime bind word_count <- quotient 144 12\n  comptime bind remainder_value <- remainder 17 5\n  comptime bind signed_delta <- difference 5 13\n  bind first <- sum table_width header_size\n  bind second <- sum word_count remainder_value\n  bind third <- sum first second\n  yield sum third signed_delta\n";
        let output = compile_to_bytecode(source).expect("M5 comptime source should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V8);
        assert!(
            output.bytecode.contains(&OP_COMPTIME_WHOLE),
            "M5 artifacts must retain compile-time provenance in AETH v8"
        );
        assert_eq!(format_program(&output.program), source);
        assert!(canonical_ast(&output.program).contains("ComptimeBind(table_width"));
        verify_bytecode(&output.bytecode).expect("M5 artifact should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M5 artifact should run")
                .exit_code,
            150
        );
    }

    #[test]
    fn rejects_nonconstant_or_unbounded_m5_comptime_shapes() {
        let mutable = "world invalid\n\nweave main [] -> Whole:\n  comptime bind mutable value <- sum 1 2\n  yield value\n";
        let error = compile_source(mutable).expect_err("comptime bindings must remain immutable");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-001");

        let nonliteral = "world invalid\n\nweave main [] -> Whole:\n  bind source <- 1\n  comptime bind value <- sum source 2\n  yield value\n";
        let error = compile_source(nonliteral)
            .expect_err("comptime bindings must not read runtime bindings");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-001");

        let nested = "world invalid\n\nweave main [] -> Whole:\n  choose bright:\n    comptime bind value <- sum 1 2\n  otherwise:\n    yield 0\n  yield 1\n";
        let error = compile_source(nested).expect_err("comptime bindings must be root-only");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-001");

        let divide_by_zero = "world invalid\n\nweave main [] -> Whole:\n  comptime bind value <- quotient 1 0\n  yield value\n";
        let error = compile_source(divide_by_zero)
            .expect_err("compile-time division by zero must be rejected deterministically");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-002");

        let overflow = "world invalid\n\nweave main [] -> Whole:\n  comptime bind value <- sum 9223372036854775807 1\n  yield value\n";
        let error = compile_source(overflow)
            .expect_err("compile-time Whole overflow must be rejected deterministically");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-002");

        let mut over_budget = String::from("world budget\n\nweave main [] -> Whole:\n");
        for index in 0..=MAX_COMPTIME_BINDINGS {
            over_budget.push_str(&format!("  comptime bind value_{index} <- sum 1 1\n"));
        }
        over_budget.push_str("  yield value_0\n");
        let error = compile_source(&over_budget)
            .expect_err("the fixed M5 directive budget must reject excess work");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-003");
    }

    #[test]
    fn verifier_rejects_comptime_provenance_outside_aeth_v8() {
        let source = "world provenance\n\nweave main [] -> Whole:\n  comptime bind value <- sum 1 2\n  yield value\n";
        let mut artifact = compile_to_bytecode(source)
            .expect("M5 provenance fixture should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V7;
        let error = verify_bytecode(&artifact)
            .expect_err("AETH v7 must not reinterpret AETH v8 comptime provenance");
        assert!(error.message.contains("valid only in AETH v8"));
    }

    #[test]
    fn compiles_verifies_and_runs_the_bounded_m4_error_effect() {
        let source = "world effects\n\nweave leaf [value: Whole] -> Whole raises Whole:\n  raise value\n\nweave forwarded [value: Whole] -> Whole raises Whole:\n  forward call leaf value\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call forwarded 17 into success otherwise error into code\n";
        let output = compile_to_bytecode(source).expect("M4 handled source should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V8);
        assert_eq!(format_program(&output.program), source);
        verify_bytecode(&output.bytecode).expect("M4 artifact should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M4 artifact should run")
                .exit_code,
            17
        );

        let mut v7_compatibility = output.bytecode.clone();
        v7_compatibility[4] = ARTIFACT_VERSION_V7;
        verify_bytecode(&v7_compatibility)
            .expect("the unchanged M4 payload must remain a valid AETH v7 artifact");
        assert_eq!(
            run_bytecode(&v7_compatibility)
                .expect("the AETH v7 M4 compatibility artifact should run")
                .exit_code,
            17
        );
    }

    #[test]
    fn handles_the_normal_exit_of_a_may_error_weave() {
        let source = "world effects\n\nweave value [] -> Whole raises Whole:\n  yield 23\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call value into success otherwise error into code\n";
        let bytecode = compile_to_bytecode(source)
            .expect("M4 normal source should compile")
            .bytecode;
        assert_eq!(
            run_bytecode(&bytecode)
                .expect("M4 normal artifact should run")
                .exit_code,
            23
        );
    }

    #[test]
    fn rejects_unhandled_or_unsafe_m4_source_shapes() {
        let unhandled = "world invalid\n\nweave leaf [] -> Whole raises Whole:\n  raise 1\n\nweave main [] -> Whole:\n  bind value <- call leaf\n  yield value\n";
        let error = compile_source(unhandled).expect_err("ordinary calls cannot hide effects");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-001");

        let owner_boundary = "world invalid\n\nweave leaf [] -> Whole raises Whole:\n  bind label <- \"owned\"\n  raise 1\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call leaf into success otherwise error into code\n";
        let error =
            compile_source(owner_boundary).expect_err("M4 control cannot cross a live owner");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-003");

        let resource_mix = "world invalid\n\nweave leaf [] -> Whole raises Whole:\n  raise 1\n\nweave main [] -> Whole:\n  bind memory <- arena 8\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call leaf into success otherwise error into code\n";
        let error = compile_source(resource_mix)
            .expect_err("M4 control cannot share a weave with resources");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-003");

        let erroring_main = "world invalid\n\nweave main [] -> Whole raises Whole:\n  raise 1\n";
        let error = compile_source(erroring_main).expect_err("main must remain total");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-001");

        let wrong_result = "world invalid\n\nweave leaf [] -> Truth raises Whole:\n  yield bright\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(wrong_result)
            .expect_err("an Error[Whole] weave must retain the terminal Whole result");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-003");

        let wrong_code = "world invalid\n\nweave leaf [] -> Whole raises Whole:\n  raise bright\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(wrong_code)
            .expect_err("raise must carry a Whole literal or copy binding");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-004");
    }

    #[test]
    fn verifier_rejects_effect_opcodes_or_metadata_outside_v7_or_v8() {
        let source = "world effects\n\nweave leaf [] -> Whole raises Whole:\n  raise 1\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call leaf into success otherwise error into code\n";
        let mut artifact = compile_to_bytecode(source)
            .expect("M4 artifact source should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V6;
        let error = verify_bytecode(&artifact)
            .expect_err("v6 must not reinterpret v7 function effect metadata");
        assert!(
            error.message.contains("artifact local count")
                || error.message.contains("trailing bytes")
                || error.message.contains("unknown")
        );

        let mut totalized_leaf = compile_to_bytecode(source)
            .expect("M4 artifact source should compile")
            .bytecode;
        // v8 header + empty record table + function count + leaf descriptor:
        // name length/name, parameter count, result tag, then effect tag.
        let leaf_effect_offset = 4 + 1 + 4 + 2 + 2 + 1 + "leaf".len() + 1 + 1;
        totalized_leaf[leaf_effect_offset] = 0;
        let error = verify_bytecode(&totalized_leaf)
            .expect_err("RAISE must not be accepted under a forged total signature");
        assert!(error.message.contains("RAISE requires an Error[Whole]"));

        let total_main = "world total\n\nweave main [] -> Whole:\n  yield 0\n";
        let mut effectful_main = compile_to_bytecode(total_main)
            .expect("total main source should compile")
            .bytecode;
        let main_effect_offset = 4 + 1 + 4 + 2 + 2 + 1 + "main".len() + 1 + 1;
        effectful_main[main_effect_offset] = 1;
        let error = verify_bytecode(&effectful_main)
            .expect_err("a forged Error[Whole] entry weave must be rejected");
        assert!(error
            .message
            .contains("main must accept no parameters, remain total, and yield Whole"));
    }

    fn hex_encode(bytes: &[u8]) -> String {
        let mut output = String::with_capacity(bytes.len() * 2);
        write_hex_bytes(bytes, &mut output);
        output
    }
}
