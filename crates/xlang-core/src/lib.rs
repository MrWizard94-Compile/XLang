//! Aether bootstrap compiler, AETH verifier, VM, and seed-hosted compile path.
//!
//! Default product compilation and seed rebuild use the Aether-written seed
//! artifact through the forge ABI (`compile_product_bytecode` / ADR-067). The
//! Rust core remains recovery diagnostics, dual-compare oracle emit, and residual
//! structural AST (statement/record edits; full `aether.ast/v8`).

// Workspace default is deny. M21 human-authorized foreign ABI load path lives
// in `ffi` with a scoped allow; the rest of the crate must not use unsafe.
#![deny(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

mod authoring;
mod ffi;
mod modules;
mod native;
mod project;
mod registry;
mod workspace;

/// AETH-encoded marker for M21 foreign host function names.
const FOREIGN_FUNCTION_NAME_PREFIX: &str = "\u{1e}F\u{1e}";

fn encode_foreign_function_name(user: &str, library: &str, symbol: &str) -> String {
    format!("{FOREIGN_FUNCTION_NAME_PREFIX}{library}\u{1e}{symbol}\u{1e}{user}")
}

fn decode_foreign_function_name(name: &str) -> Option<(&str, &str, &str)> {
    let rest = name.strip_prefix(FOREIGN_FUNCTION_NAME_PREFIX)?;
    let mut parts = rest.split('\u{1e}');
    let library = parts.next()?;
    let symbol = parts.next()?;
    let user = parts.next()?;
    if parts.next().is_some() || library.is_empty() || symbol.is_empty() || user.is_empty() {
        return None;
    }
    Some((library, symbol, user))
}

pub use authoring::{
    apply_structural_edit, diagnostic_json, product_structure_json, structural_document_json,
    StructuralEditError, StructuralEditResult, DIAGNOSTIC_SCHEMA_VERSION,
    PRODUCT_EDIT_SCHEMA_VERSION, PRODUCT_STRUCTURE_SCHEMA_VERSION, STRUCTURAL_AST_SCHEMA_VERSION,
    STRUCTURAL_EDIT_PROTOCOL_VERSION,
};
pub use modules::{
    compile_project_entry, compile_project_entry_with_packages, compile_project_modules,
    compile_project_modules_with_packages, elaborate_project_entry,
    elaborate_project_entry_with_packages, elaborate_project_modules,
    elaborate_project_modules_with_packages, mangle_weave, multi_module_authority_note,
    run_project_tests, run_project_tests_with_grants, source_requires_project_modules,
    validate_lib_module_source, ProjectTestReport, ProjectTestResult,
};
pub use native::{
    f_native_authorized, lower_verified_aeth_to_c, native_aeth_to_c_locals_pilot,
    native_aeth_to_c_pilot, NativeError,
};
pub use project::{
    format_project, format_source, format_source_product, parse_project_document,
    refresh_project_lock, resolve_unit_path, serialize_project_document, sha256_hex,
    unit_artifact_file_name, validate_unit_path, verify_project, ProjectDocument, ProjectError,
    ProjectFormatReport, ProjectFormatUnit, ProjectLock, ProjectLockUnit, ProjectUnit,
    ProjectUnitReport, ProjectUnitRole, ProjectVerifyReport, PROJECT_SCHEMA_VERSION,
};
pub use registry::{
    empty_registry_cache, f_registry_authorized, parse_registry_cache, pin_local_package,
    registry_offline_cache_verify, serialize_registry_cache, verify_registry_cache,
    RegistryCacheDocument, RegistryError, RegistryPackagePin, REGISTRY_CACHE_SCHEMA,
    REGISTRY_INDEX_FILE,
};
pub use workspace::{
    compile_workspace_package, parse_workspace_document, refresh_workspace_lock,
    resolve_package_path, serialize_workspace_document, topological_package_order,
    verify_workspace, WorkspaceDocument, WorkspaceError, WorkspaceLock, WorkspaceLockPackage,
    WorkspacePackage, WorkspacePackageReport, WorkspaceVerifyReport, WORKSPACE_PROJECT_FILE,
    WORKSPACE_SCHEMA_VERSION,
};

pub const LANGUAGE_NAME: &str = "Aether";
pub const LANGUAGE_VERSION: &str = "0.36.0";

/// Checked-in Aether-written seed compiler artifact (AETH v11).
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
const ARTIFACT_VERSION_V9: u8 = 9;
const ARTIFACT_VERSION_V10: u8 = 10;
const ARTIFACT_VERSION_V11: u8 = 11;
/// AETH v12 adds verified task-frame metadata and cooperative checkpoints.
const ARTIFACT_VERSION_V12: u8 = 12;
const MAX_SOURCE_BYTES: usize = 1_000_000;
const MAX_FUNCTIONS: usize = 256;
const MAX_LOCALS: usize = u16::MAX as usize;
const MAX_RECORDS: usize = 256;
const MAX_RECORD_FIELDS: usize = 64;
const MAX_SHAPES: usize = 64;
const MAX_SHAPE_FIELDS: usize = 8;
const MAX_TABLE_CAPACITY: i64 = 1_024;
const MAX_TEXT_BYTES: usize = 1_000_000;
const MAX_BYTES: usize = 1_000_000;
const MAX_RECORD_BYTES: usize = 1_000_000;
const MAX_CALL_DEPTH: usize = 1_024;
const MAX_ARENA_BYTES: u32 = 1_000_000;
const BUFFER_METADATA_BYTES: usize = 16;
const TABLE_METADATA_BYTES: usize = 16;
const MAX_COMPTIME_BINDINGS: usize = 1_024;
const MAX_NURSERY_SPAWNS: usize = 8;

pub(crate) const OP_PUSH_TEXT: u8 = 1;
pub(crate) const OP_PUSH_WHOLE: u8 = 2;
const OP_PUSH_TRUTH: u8 = 3;
pub(crate) const OP_STORE: u8 = 4;
pub(crate) const OP_LOAD: u8 = 5;
const OP_MOVE: u8 = 6;
const OP_REVISE: u8 = 7;
pub(crate) const OP_SPEAK: u8 = 8;
pub(crate) const OP_YIELD: u8 = 9;
pub(crate) const OP_SUM: u8 = 10;
pub(crate) const OP_DIFFERENCE: u8 = 11;
pub(crate) const OP_PRODUCT: u8 = 12;
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
pub(crate) const OP_QUOTIENT: u8 = 25;
pub(crate) const OP_REMAINDER: u8 = 26;
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
const OP_TABLE: u8 = 57;
const OP_TABLE_ALLOCATE: u8 = 58;
const OP_TABLE_STORE: u8 = 59;
const OP_TABLE_LOAD: u8 = 60;
const OP_TABLE_COUNT: u8 = 61;
const OP_NURSERY_BEGIN: u8 = 62;
const OP_NURSERY_SPAWN: u8 = 63;
const OP_NURSERY_END: u8 = 64;
const OP_HOST_CALL: u8 = 65;
/// Logical destruction of a live owner local (M19a explicit `release`).
const OP_RELEASE: u8 = 66;
/// AETH v12 verifier-approved cooperative task suspension point.
const OP_TASK_CHECKPOINT: u8 = 67;

/// Function-table kind for AETH v11: ordinary guest weave.
const FUNCTION_KIND_GUEST: u8 = 0;
/// Function-table kind for AETH v11: host weave (empty guest code).
const FUNCTION_KIND_HOST: u8 = 1;
/// AETH v12 function flag marking a verifier-approved resumable task frame.
const FUNCTION_FLAG_TASK_FRAME: u8 = 0x01;

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
    } else if normalized.contains("ae-seed-012") {
        "AE-SEED-012"
    } else if normalized.contains("ae-seed-011") {
        "AE-SEED-011"
    } else if normalized.contains("ae-seed-010") {
        "AE-SEED-010"
    } else if normalized.contains("ae-seed-008") {
        "AE-SEED-008"
    } else if normalized.contains("ae-seed-007") {
        "AE-SEED-007"
    } else if normalized.contains("ae-seed-006") {
        "AE-SEED-006"
    } else if normalized.contains("ae-seed-005") {
        "AE-SEED-005"
    } else if normalized.contains("ae-seed-004") {
        "AE-SEED-004"
    } else if normalized.contains("ae-seed-003") {
        "AE-SEED-003"
    } else if normalized.contains("ae-seed-002") {
        "AE-SEED-002"
    } else if normalized.contains("ae-seed-001") || normalized.contains("product seed path") {
        "AE-SEED-001"
    } else if normalized.contains("ae-host-003") {
        "AE-HOST-003"
    } else if normalized.contains("ae-host-002") {
        "AE-HOST-002"
    } else if normalized.contains("ae-host-001")
        || normalized.contains("host weave")
        || normalized.contains("host service")
        || normalized.contains("host call")
    {
        "AE-HOST-001"
    } else if normalized.contains("ae-task-005") {
        "AE-TASK-005"
    } else if normalized.contains("ae-task-003") {
        "AE-TASK-003"
    } else if normalized.contains("ae-task-004")
        || normalized.contains("task_checkpoint")
        || normalized.contains("task checkpoint")
        || normalized.contains("task frame requires")
    {
        "AE-TASK-004"
    } else if normalized.contains("ae-task-002") {
        "AE-TASK-002"
    } else if normalized.contains("ae-task-001")
        || normalized.contains("together")
        || normalized.contains("nursery")
        || normalized.contains("spawn")
    {
        "AE-TASK-001"
    } else if normalized.contains("ae-layout-003") {
        "AE-LAYOUT-003"
    } else if normalized.contains("ae-layout-002") {
        "AE-LAYOUT-002"
    } else if normalized.contains("ae-layout-001")
        || normalized.contains("shape declaration")
        || normalized.contains("shape field")
        || normalized.contains("table layout")
        || normalized.contains("layout rows")
        || normalized.contains("layout columns")
        || normalized.contains("shapes require")
        || normalized.contains("shapes support")
        || normalized.contains("shape name")
        || normalized.contains("shape collides")
    {
        "AE-LAYOUT-001"
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
    } else if normalized.contains("ae-resource-004") {
        "AE-RESOURCE-004"
    } else if normalized.contains("arena")
        || normalized.contains("buffer")
        || normalized.contains("table")
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
    /// An M6 dual-layout table owner keyed by a declared shape identifier.
    Table(u16),
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
            Self::Table(_) => 10,
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
            Self::Table(shape) => write!(formatter, "Table#{shape}"),
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
            | ValueType::Table(_)
    )
}

const fn is_resource_owner_type(value_type: ValueType) -> bool {
    matches!(
        value_type,
        ValueType::BufferWhole | ValueType::BufferTruth | ValueType::Table(_) | ValueType::Arena
    )
}

const fn is_table_type(value_type: ValueType) -> bool {
    matches!(value_type, ValueType::Table(_))
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
                ARTIFACT_VERSION_V6
                    | ARTIFACT_VERSION_V7
                    | ARTIFACT_VERSION_V8
                    | ARTIFACT_VERSION_V9
                    | ARTIFACT_VERSION_V10
                    | ARTIFACT_VERSION_V11
                    | ARTIFACT_VERSION_V12
            ) =>
            {
                Ok(Self::Access)
            }
            3 => Err(BytecodeError::new(
                offset,
                "access parameters are valid only in AETH v6 through v12 artifacts",
            )),
            _ => Err(BytecodeError::new(offset, "unknown Aether parameter mode")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub world: String,
    pub records: Vec<RecordDeclaration>,
    pub shapes: Vec<ShapeDeclaration>,
    /// Capability-closed host weave declarations (M8). No body; total external ABI.
    pub host_weaves: Vec<HostWeave>,
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
                .shapes
                .iter()
                .map(|shape| 2 + shape.fields.len() * 2)
                .sum::<usize>()
            + self
                .host_weaves
                .iter()
                .map(|host| 3 + host.parameters.len() * 2)
                .sum::<usize>()
            + self
                .weaves
                .iter()
                .map(|weave| 4 + weave.parameters.len() * 2 + statement_token_count(&weave.body))
                .sum::<usize>()
    }
}

/// A total host weave: external pure service signature with no Aether body.
///
/// When [`HostWeave::foreign`] is set, this is an M21 foreign weave: host-side
/// libloading after an explicit library path grant (not a pure fixture).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostWeave {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub result: ValueType,
    pub span: Span,
    /// M21: library key + C symbol; `None` for ordinary pure/grant host weaves.
    pub foreign: Option<ForeignAbi>,
}

/// Pinned foreign library key and symbol for M21 pilot weaves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignAbi {
    /// Logical library name from `from "key"` (not a filesystem path).
    pub library: String,
    /// Exact C symbol name from `symbol "name"`.
    pub symbol: String,
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

/// An M6 layout shape: a fixed product of 1..=8 `Whole` fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeDeclaration {
    pub name: String,
    pub fields: Vec<ShapeField>,
    pub span: Span,
}

/// One `Whole` field in a [`ShapeDeclaration`]. Field order is layout order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeField {
    pub name: String,
    pub span: Span,
}

/// Physical storage order for an M6 table of a declared shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableLayout {
    Rows,
    Columns,
}

impl TableLayout {
    const fn word(self) -> &'static str {
        match self {
            Self::Rows => "rows",
            Self::Columns => "columns",
        }
    }

    const fn to_tag(self) -> u8 {
        match self {
            Self::Rows => 0,
            Self::Columns => 1,
        }
    }

    fn from_tag(value: u8, offset: usize) -> Result<Self, BytecodeError> {
        match value {
            0 => Ok(Self::Rows),
            1 => Ok(Self::Columns),
            _ => Err(BytecodeError::new(
                offset,
                "unknown Aether table layout tag",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weave {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub result: ValueType,
    pub effect: Effect,
    /// `true` only for an explicit `task weave` declaration. This is semantic
    /// metadata, not a naming convention or advisory annotation.
    pub task: bool,
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
pub struct Spawn {
    pub weave: String,
    pub arguments: Vec<Atom>,
    pub destination: String,
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
    /// M19a: logical destruction of a live unique or resource owner.
    Release {
        name: String,
        span: Span,
    },
    /// M19e: verifier-approved cooperative suspension boundary for a task.
    Checkpoint {
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
    Together {
        spawns: Vec<Spawn>,
        span: Span,
    },
}

impl Statement {
    const fn span(&self) -> Span {
        match self {
            Self::Bind { span, .. }
            | Self::Revise { span, .. }
            | Self::Speak { span, .. }
            | Self::Release { span, .. }
            | Self::Checkpoint { span }
            | Self::Yield { span, .. }
            | Self::Raise { span, .. }
            | Self::Forward { span, .. }
            | Self::Handle { span, .. }
            | Self::Choose { span, .. }
            | Self::While { span, .. }
            | Self::Together { span, .. } => *span,
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
    /// An unallocated M6 table owner placeholder with an explicit layout.
    Table {
        shape: String,
        layout: TableLayout,
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
    Store {
        table: Atom,
        index: Atom,
        field: String,
        value: Atom,
        destination: String,
    },
    Load {
        table: Atom,
        index: Atom,
        field: String,
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

/// The immutable context shared by recursive M2/M6 resource-outcome validation.
/// Keeping it together prevents terminal branch validation from drifting away
/// from the enclosing weave's type and result contract.
struct ResourceValidationContext<'a> {
    signatures: &'a BTreeMap<String, FunctionSignature>,
    records: &'a [RecordDeclaration],
    shapes: &'a [ShapeDeclaration],
    weave: &'a Weave,
}

/// Typed resource facts produced by source validation and consumed directly by
/// AETH v6/v7/v8/v9 lowering. This is deliberately separate from the parsed AST: it
/// records the resolved lexical region, owner place, element type, and stable
/// replacement destination for every closed M2/M6 outcome.
#[derive(Clone)]
struct SemanticResourcePlan {
    arena: Option<SemanticArena>,
    /// Direct root arena capacity indexed by guest weave. v12 uses these
    /// verified values to size main and task-private regions independently.
    arenas_by_weave: BTreeMap<String, SemanticArena>,
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
    TableAllocate {
        region: String,
        owner: String,
        destination: String,
    },
    TableStore {
        region: String,
        owner: String,
        destination: String,
        shape: u16,
        field: u8,
    },
    TableLoad {
        region: String,
        owner: String,
        destination: String,
        shape: u16,
        field: u8,
    },
}

impl SemanticResourcePlan {
    const fn empty() -> Self {
        Self {
            arena: None,
            arenas_by_weave: BTreeMap::new(),
            outcomes: BTreeMap::new(),
        }
    }

    const fn arena_capacity(&self) -> u32 {
        match &self.arena {
            Some(arena) => arena.capacity,
            None => 0,
        }
    }

    fn direct_arena_capacity(&self, weave: &str) -> u32 {
        self.arenas_by_weave
            .get(weave)
            .map_or(0, |arena| arena.capacity)
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
            | Self::At { region, .. }
            | Self::TableAllocate { region, .. }
            | Self::TableStore { region, .. }
            | Self::TableLoad { region, .. } => region,
        }
    }

    fn destination(&self) -> &str {
        match self {
            Self::Allocate { destination, .. }
            | Self::Append { destination, .. }
            | Self::At { destination, .. }
            | Self::TableAllocate { destination, .. }
            | Self::TableStore { destination, .. }
            | Self::TableLoad { destination, .. } => destination,
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
                Self::TableAllocate {
                    owner, destination, ..
                },
                ResourceOperation::Allocate {
                    buffer,
                    destination: source_destination,
                    ..
                },
            )
            | (
                Self::TableStore {
                    owner, destination, ..
                },
                ResourceOperation::Store {
                    table: buffer,
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
            (
                Self::TableLoad {
                    owner, destination, ..
                },
                ResourceOperation::Load {
                    table: buffer,
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
    Table {
        shape: u16,
        layout: TableLayout,
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
    is_host: bool,
    is_task: bool,
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
pub(crate) struct ArtifactFunction {
    name: String,
    parameters: Vec<(ValueType, ParameterMode)>,
    result: ValueType,
    effect: Effect,
    /// AETH v11: guest (0) or host (1). Pre-v11 artifacts are always guest.
    kind: u8,
    /// AETH v12 task-frame authority flags. Earlier artifact versions encode
    /// no flags and therefore always carry zero.
    flags: u8,
    /// AETH v12 direct arena capacity for main or one task frame.
    frame_arena_capacity: u32,
    locals: Vec<LocalDescriptor>,
    code: Vec<u8>,
}

impl ArtifactFunction {
    const fn is_host(&self) -> bool {
        self.kind == FUNCTION_KIND_HOST
    }

    const fn is_task(&self) -> bool {
        self.flags & FUNCTION_FLAG_TASK_FRAME != 0
    }
}

#[derive(Clone)]
struct Artifact {
    version: u8,
    arena_capacity: u32,
    records: Vec<ArtifactRecord>,
    shapes: Vec<ArtifactShape>,
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

#[derive(Clone)]
struct ArtifactShape {
    name: String,
    fields: Vec<String>,
}

#[derive(Clone, PartialEq, Eq)]
struct NurseryVerification {
    expected: u8,
    seen: u8,
    targets: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq)]
struct VerificationState {
    stack: VerificationStack,
    initialized: Vec<bool>,
    moved: Vec<bool>,
    nursery: Option<NurseryVerification>,
}

/// Immutable verifier inputs shared by every instruction in one weave.
/// Grouping these prevents the instruction checker from gaining a fragile,
/// ever-growing parameter list as the verified instruction set evolves.
struct VerificationContext<'a> {
    function_index: usize,
    version: u8,
    function: &'a ArtifactFunction,
    functions: &'a [ArtifactFunction],
    records: &'a [ArtifactRecord],
    shapes: &'a [ArtifactShape],
}

/// The verifier records where a buffer or table value on the transient operand
/// stack came from. A raw placeholder, a read loan, a moved local owner, and an
/// owned result from a checked call are not interchangeable. This prevents a
/// forged resource instruction stream from substituting a fresh placeholder
/// when an operation promises to restore the specific owner it just moved.
#[derive(Clone, PartialEq, Eq)]
struct VerificationStack {
    values: Vec<VerificationStackValue>,
}

#[derive(Clone, PartialEq, Eq)]
struct VerificationStackValue {
    value_type: ValueType,
    resource_provenance: ResourceProvenance,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceProvenance {
    NotResource,
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
            resource_provenance: if is_buffer_type(value_type) || is_table_type(value_type) {
                ResourceProvenance::Borrowed
            } else {
                ResourceProvenance::NotResource
            },
        });
    }

    fn push_buffer_placeholder(&mut self, element: BufferElement) {
        self.values.push(VerificationStackValue {
            value_type: element.value_type(),
            resource_provenance: ResourceProvenance::Placeholder,
        });
    }

    fn push_table_placeholder(&mut self, shape: u16) {
        self.values.push(VerificationStackValue {
            value_type: ValueType::Table(shape),
            resource_provenance: ResourceProvenance::Placeholder,
        });
    }

    fn push_moved_local(&mut self, value_type: ValueType, slot: usize) {
        self.values.push(VerificationStackValue {
            value_type,
            resource_provenance: if is_buffer_type(value_type) || is_table_type(value_type) {
                ResourceProvenance::MovedLocal(slot)
            } else {
                ResourceProvenance::NotResource
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
struct RuntimeText {
    value: String,
    ascii: bool,
}

impl RuntimeText {
    fn new(value: String) -> Self {
        Self {
            ascii: value.is_ascii(),
            value,
        }
    }

    fn as_str(&self) -> &str {
        &self.value
    }

    const fn byte_len(&self) -> usize {
        self.value.len()
    }

    fn scalar_len(&self) -> usize {
        if self.ascii {
            self.value.len()
        } else {
            self.value.chars().count()
        }
    }

    fn scalar_at(&self, index: usize) -> Option<char> {
        if self.ascii {
            self.value.as_bytes().get(index).copied().map(char::from)
        } else {
            self.value.chars().nth(index)
        }
    }

    fn slice_scalars(&self, start: usize, end: usize) -> Option<Self> {
        let start = self.scalar_byte_offset(start);
        let end = self.scalar_byte_offset(end);
        self.value.get(start..end).map(|value| Self {
            value: value.to_owned(),
            // An ASCII source can only produce an ASCII slice. A Unicode
            // source may produce ASCII text, but retaining `false` is safe and
            // preserves scalar behavior without a second scan.
            ascii: self.ascii,
        })
    }

    fn find_from_scalar(&self, needle: &Self, start: usize) -> i64 {
        let start_byte = self.scalar_byte_offset(start);
        let Some(tail) = self.value.get(start_byte..) else {
            return -1;
        };
        let Some(offset) = tail.find(needle.as_str()) else {
            return -1;
        };
        let found = start_byte + offset;
        let scalar = if self.ascii {
            found
        } else {
            self.value[..found].chars().count()
        };
        i64::try_from(scalar).unwrap_or(-1)
    }

    fn join(mut self, suffix: &Self) -> Self {
        self.ascii = self.ascii && suffix.ascii;
        self.value.push_str(suffix.as_str());
        self
    }

    fn into_string(self) -> String {
        self.value
    }

    fn scalar_byte_offset(&self, scalar_index: usize) -> usize {
        if self.ascii {
            scalar_index.min(self.value.len())
        } else {
            self.value
                .char_indices()
                .nth(scalar_index)
                .map_or(self.value.len(), |(offset, _)| offset)
        }
    }
}

#[derive(Debug, Clone)]
enum RuntimeValue {
    Text(RuntimeText),
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
    Table {
        shape: u16,
        layout: TableLayout,
        field_count: u8,
        allocated: bool,
        offset: usize,
        capacity: usize,
    },
}

enum RuntimeExit {
    Return(RuntimeValue),
    ErrorWhole(i64),
    /// A verified task has reached an empty-stack cancellation boundary.
    Checkpoint(TaskFrame),
    /// A task returns through the scheduler so its private lane can be revoked.
    TaskReturn {
        value: RuntimeValue,
        frame: TaskFrame,
    },
}

/// A fixed, private slice of the v12 nursery slab.  The lane is never exposed
/// to guest code; resource values retain only offsets into the VM-owned arena.
#[derive(Clone, Copy)]
struct TaskArenaLane {
    start: usize,
    capacity: usize,
    used: usize,
}

/// Complete suspension state for a task.  The task subset has no calls or
/// nested nurseries, so one frame is sufficient to resume it deterministically.
struct TaskFrame {
    function_index: usize,
    locals: Vec<Option<RuntimeValue>>,
    stack: Vec<RuntimeValue>,
    position: usize,
    lane: TaskArenaLane,
}

/// Captured v12 nursery child arguments.  Capture happens at `NURSERY_SPAWN`;
/// dispatch begins only after the complete nursery has passed admission at END.
struct V12NurseryChild {
    function: usize,
    arguments: Vec<RuntimeValue>,
    destination: usize,
}

struct V12NurseryFrame {
    expected: u8,
    seen: u8,
    children: Vec<V12NurseryChild>,
}

enum V12SchedulerState {
    Pending(Vec<RuntimeValue>),
    Parked(TaskFrame),
    Completed,
    Failed,
    Cancelled,
    Running,
}

struct V12SchedulerChild {
    function: usize,
    destination: usize,
    lane: Option<TaskArenaLane>,
    state: V12SchedulerState,
}

/// Borrowed execution state for one complete v12 nursery. Grouping this state
/// makes the scheduler boundary explicit while keeping parent ownership, output,
/// VM state, and source offset together for every dispatch outcome.
struct V12NurseryExecutionContext<'a> {
    artifact: &'a Artifact,
    parent: &'a ArtifactFunction,
    parent_locals: &'a mut [Option<RuntimeValue>],
    stdout: &'a mut String,
    runtime_state: &'a mut RuntimeState,
    depth: usize,
    offset: usize,
}

/// Test-only evidence of the externally observable v12 scheduler lifecycle.
/// Production cancellation remains deliberately unobservable to guest code;
/// this trace makes the critical parked-frame destruction invariant directly
/// regression-testable without adding a runtime introspection surface.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskFrameTraceEvent {
    Started(usize),
    Parked(usize),
    Completed(usize),
    Cancelled(usize),
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
            Self::Table { shape, .. } => ValueType::Table(*shape),
        }
    }
}

struct NurseryFrame {
    cancelled: bool,
    code: i64,
    expected: u8,
    seen: u8,
}

/// Operator-selected host grants for capability-mediated I/O (M14 / ADR-018)
/// and foreign library paths (M21 / ADR-025).
///
/// Empty grants install only pure fixtures (`whole_inc`, `text_extent`).
/// Foreign weaves require an explicit `library_grants` entry matching `from "key"`.
#[derive(Clone, Debug, Default)]
pub struct HostGrantConfig {
    pub read_roots: Vec<std::path::PathBuf>,
    pub write_roots: Vec<std::path::PathBuf>,
    pub env_names: Vec<String>,
    /// Map from foreign `from "key"` to an absolute library file path (no PATH search).
    pub library_grants: BTreeMap<String, std::path::PathBuf>,
}

const HOST_IO_MAX_BYTES: usize = 1_000_000;

/// Capability-closed catalog of host services installed by the trusted host.
#[derive(Clone, Default)]
struct HostServices {
    names: BTreeMap<String, HostServiceKind>,
    foreign: BTreeMap<String, ForeignService>,
    library_grants: BTreeMap<String, std::path::PathBuf>,
    read_roots: Vec<std::path::PathBuf>,
    write_roots: Vec<std::path::PathBuf>,
    env_names: BTreeSet<String>,
}

#[derive(Clone, Copy)]
enum HostServiceKind {
    WholeInc,
    TextExtent,
    ReadText,
    ReadBytes,
    WriteText,
    WriteBytes,
    EnvGet,
}

#[derive(Clone, Debug)]
struct ForeignService {
    library: String,
    symbol: String,
    arity: usize,
}

impl HostServices {
    /// Product pure fixture: only `whole_inc` and `text_extent`.
    fn pure_fixture() -> Self {
        let mut names = BTreeMap::new();
        names.insert("whole_inc".to_owned(), HostServiceKind::WholeInc);
        names.insert("text_extent".to_owned(), HostServiceKind::TextExtent);
        Self {
            names,
            foreign: BTreeMap::new(),
            library_grants: BTreeMap::new(),
            read_roots: Vec::new(),
            write_roots: Vec::new(),
            env_names: BTreeSet::new(),
        }
    }

    /// Pure fixtures plus grant-backed I/O and optional foreign library grants.
    fn with_grants(config: HostGrantConfig) -> Self {
        let mut services = Self::pure_fixture();
        services.read_roots = config.read_roots;
        services.write_roots = config.write_roots;
        services.env_names = config.env_names.into_iter().collect();
        services.library_grants = config.library_grants;
        if !services.read_roots.is_empty() {
            services
                .names
                .insert("read_text".to_owned(), HostServiceKind::ReadText);
            services
                .names
                .insert("read_bytes".to_owned(), HostServiceKind::ReadBytes);
        }
        if !services.write_roots.is_empty() {
            services
                .names
                .insert("write_text".to_owned(), HostServiceKind::WriteText);
            services
                .names
                .insert("write_bytes".to_owned(), HostServiceKind::WriteBytes);
        }
        if !services.env_names.is_empty() {
            services
                .names
                .insert("env_get".to_owned(), HostServiceKind::EnvGet);
        }
        services
    }

    fn invoke(
        &self,
        name: &str,
        arguments: &[RuntimeValue],
        offset: usize,
    ) -> Result<RuntimeValue, BytecodeError> {
        // M21: foreign host functions store lib+symbol+user name in the AETH name.
        if let Some((library, symbol, _user)) = decode_foreign_function_name(name) {
            let foreign = ForeignService {
                library: library.to_owned(),
                symbol: symbol.to_owned(),
                arity: arguments.len(),
            };
            return self.invoke_foreign(&foreign, arguments, offset);
        }
        if let Some(foreign) = self.foreign.get(name) {
            return self.invoke_foreign(foreign, arguments, offset);
        }
        let Some(kind) = self.names.get(name) else {
            return Err(BytecodeError::new(
                offset,
                format!("AE-HOST-003: host service {name} is missing or denied"),
            ));
        };
        match kind {
            HostServiceKind::WholeInc => {
                if arguments.len() != 1 {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service whole_inc requires one Whole argument",
                    ));
                }
                let RuntimeValue::Whole(value) = &arguments[0] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service whole_inc requires Whole",
                    ));
                };
                let next = value.checked_add(1).ok_or_else(|| {
                    BytecodeError::new(
                        offset,
                        "AE-HOST-003: host service whole_inc overflowed Whole",
                    )
                })?;
                Ok(RuntimeValue::Whole(next))
            }
            HostServiceKind::TextExtent => {
                if arguments.len() != 1 {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service text_extent requires one borrowed Text argument",
                    ));
                }
                let RuntimeValue::Text(text) = &arguments[0] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service text_extent requires Text",
                    ));
                };
                let len = i64::try_from(text.byte_len()).map_err(|_| {
                    BytecodeError::new(
                        offset,
                        "AE-HOST-003: host service text_extent length is outside Whole",
                    )
                })?;
                Ok(RuntimeValue::Whole(len))
            }
            HostServiceKind::ReadText => {
                let path = expect_one_text_path(arguments, offset, "read_text")?;
                let full = resolve_under_roots(&self.read_roots, &path, offset)?;
                let bytes = std::fs::read(&full).map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-HOST-003: host service read_text failed: {error}"),
                    )
                })?;
                if bytes.len() > HOST_IO_MAX_BYTES {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-005: host service read_text exceeds the 1000000-byte safety limit",
                    ));
                }
                let text = String::from_utf8(bytes).map_err(|_| {
                    BytecodeError::new(
                        offset,
                        "AE-HOST-003: host service read_text requires UTF-8 file contents",
                    )
                })?;
                Ok(RuntimeValue::Text(RuntimeText::new(text)))
            }
            HostServiceKind::ReadBytes => {
                let path = expect_one_text_path(arguments, offset, "read_bytes")?;
                let full = resolve_under_roots(&self.read_roots, &path, offset)?;
                let bytes = std::fs::read(&full).map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-HOST-003: host service read_bytes failed: {error}"),
                    )
                })?;
                if bytes.len() > HOST_IO_MAX_BYTES {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-005: host service read_bytes exceeds the 1000000-byte safety limit",
                    ));
                }
                Ok(RuntimeValue::Bytes(bytes))
            }
            HostServiceKind::WriteText => {
                if arguments.len() != 2 {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_text requires path and body Text",
                    ));
                }
                let RuntimeValue::Text(path) = &arguments[0] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_text path must be Text",
                    ));
                };
                let RuntimeValue::Text(body) = &arguments[1] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_text body must be Text",
                    ));
                };
                if body.byte_len() > HOST_IO_MAX_BYTES {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-005: host service write_text exceeds the 1000000-byte safety limit",
                    ));
                }
                let full = resolve_under_roots(&self.write_roots, path.as_str(), offset)?;
                if let Some(parent) = full.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                std::fs::write(&full, body.as_str().as_bytes()).map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-HOST-003: host service write_text failed: {error}"),
                    )
                })?;
                let len = i64::try_from(body.byte_len()).map_err(|_| {
                    BytecodeError::new(
                        offset,
                        "AE-HOST-003: host service write_text length is outside Whole",
                    )
                })?;
                Ok(RuntimeValue::Whole(len))
            }
            HostServiceKind::WriteBytes => {
                if arguments.len() != 2 {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_bytes requires path Text and body Bytes",
                    ));
                }
                let RuntimeValue::Text(path) = &arguments[0] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_bytes path must be Text",
                    ));
                };
                let RuntimeValue::Bytes(body) = &arguments[1] else {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-002: host service write_bytes body must be Bytes",
                    ));
                };
                if body.len() > HOST_IO_MAX_BYTES {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-005: host service write_bytes exceeds the 1000000-byte safety limit",
                    ));
                }
                let full = resolve_under_roots(&self.write_roots, path.as_str(), offset)?;
                if let Some(parent) = full.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                std::fs::write(&full, body).map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-HOST-003: host service write_bytes failed: {error}"),
                    )
                })?;
                let len = i64::try_from(body.len()).map_err(|_| {
                    BytecodeError::new(
                        offset,
                        "AE-HOST-003: host service write_bytes length is outside Whole",
                    )
                })?;
                Ok(RuntimeValue::Whole(len))
            }
            HostServiceKind::EnvGet => {
                let name = expect_one_text_path(arguments, offset, "env_get")?;
                if name.len() > 256 {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-005: host service env_get name exceeds the 256-byte safety limit",
                    ));
                }
                if !self.env_names.contains(&name) {
                    return Err(BytecodeError::new(
                        offset,
                        format!("AE-HOST-003: host service env_get denied for name {name}"),
                    ));
                }
                let value = std::env::var(&name).map_err(|_| {
                    BytecodeError::new(
                        offset,
                        format!(
                            "AE-HOST-003: host service env_get missing environment variable {name}"
                        ),
                    )
                })?;
                Ok(RuntimeValue::Text(RuntimeText::new(value)))
            }
        }
    }

    fn invoke_foreign(
        &self,
        foreign: &ForeignService,
        arguments: &[RuntimeValue],
        offset: usize,
    ) -> Result<RuntimeValue, BytecodeError> {
        if arguments.len() != foreign.arity {
            return Err(BytecodeError::new(
                offset,
                format!(
                    "AE-FFI-002: foreign service requires {} Whole argument(s)",
                    foreign.arity
                ),
            ));
        }
        let Some(path) = self.library_grants.get(&foreign.library) else {
            return Err(BytecodeError::new(
                offset,
                format!(
                    "AE-FFI-003: foreign library key {} is not granted; pass --grant-lib {}=<path>",
                    foreign.library, foreign.library
                ),
            ));
        };
        if !path.is_file() {
            return Err(BytecodeError::new(
                offset,
                format!(
                    "AE-FFI-003: foreign library grant for {} is not a file: {}",
                    foreign.library,
                    path.display()
                ),
            ));
        }
        ffi::invoke_whole_symbol(path, &foreign.symbol, arguments, offset)
    }
}

fn expect_one_text_path(
    arguments: &[RuntimeValue],
    offset: usize,
    service: &str,
) -> Result<String, BytecodeError> {
    if arguments.len() != 1 {
        return Err(BytecodeError::new(
            offset,
            format!("AE-HOST-002: host service {service} requires one Text path argument"),
        ));
    }
    let RuntimeValue::Text(path) = &arguments[0] else {
        return Err(BytecodeError::new(
            offset,
            format!("AE-HOST-002: host service {service} requires Text"),
        ));
    };
    Ok(path.as_str().to_owned())
}

fn resolve_under_roots(
    roots: &[std::path::PathBuf],
    guest_path: &str,
    offset: usize,
) -> Result<std::path::PathBuf, BytecodeError> {
    validate_guest_io_path(guest_path, offset)?;
    if roots.is_empty() {
        return Err(BytecodeError::new(
            offset,
            "AE-HOST-003: host I/O grant root is missing",
        ));
    }
    if guest_path.len() > 4096 {
        return Err(BytecodeError::new(
            offset,
            "AE-HOST-005: guest path exceeds the 4096-byte safety limit",
        ));
    }
    let mut last_error = BytecodeError::new(offset, "AE-HOST-004: guest path escapes grant roots");
    for root in roots {
        let Ok(root_canon) = root.canonicalize() else {
            last_error = BytecodeError::new(
                offset,
                format!(
                    "AE-HOST-003: host grant root {} could not be resolved",
                    root.display()
                ),
            );
            continue;
        };
        let mut joined = root_canon.clone();
        for segment in guest_path.split('/') {
            joined.push(segment);
        }
        // Parent may not exist yet for writes: canonicalize parent + push file.
        let resolved = if joined.exists() {
            joined.canonicalize().map_err(|error| {
                BytecodeError::new(
                    offset,
                    format!("AE-HOST-004: could not resolve guest path: {error}"),
                )
            })?
        } else if let Some(parent) = joined.parent() {
            let parent_canon = if parent.as_os_str().is_empty() {
                root_canon.clone()
            } else if parent.exists() {
                parent.canonicalize().map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-HOST-004: could not resolve guest path parent: {error}"),
                    )
                })?
            } else {
                // Allow write creating nested path only if we can create under root later;
                // for resolve check, ensure planned path stays under root via component join.
                let mut check = root_canon.clone();
                for segment in guest_path.split('/') {
                    check.push(segment);
                }
                if !check.starts_with(&root_canon) {
                    return Err(BytecodeError::new(
                        offset,
                        "AE-HOST-004: guest path escapes grant root",
                    ));
                }
                return Ok(check);
            };
            if !parent_canon.starts_with(&root_canon) {
                return Err(BytecodeError::new(
                    offset,
                    "AE-HOST-004: guest path escapes grant root",
                ));
            }
            parent_canon.join(joined.file_name().ok_or_else(|| {
                BytecodeError::new(offset, "AE-HOST-004: guest path has no file name")
            })?)
        } else {
            return Err(BytecodeError::new(
                offset,
                "AE-HOST-004: guest path has no parent",
            ));
        };
        if resolved.starts_with(&root_canon) {
            return Ok(resolved);
        }
        last_error = BytecodeError::new(offset, "AE-HOST-004: guest path escapes grant root");
    }
    Err(last_error)
}

fn validate_guest_io_path(path: &str, offset: usize) -> Result<(), BytecodeError> {
    if path.is_empty() {
        return Err(BytecodeError::new(
            offset,
            "AE-HOST-004: guest path must be non-empty",
        ));
    }
    if path.contains('\\') || path.starts_with('/') || path.contains(':') || path.contains("//") {
        return Err(BytecodeError::new(
            offset,
            "AE-HOST-004: guest path must be relative with '/' separators only",
        ));
    }
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(BytecodeError::new(
                offset,
                "AE-HOST-004: guest path escapes grant root or has empty segments",
            ));
        }
        if !segment.bytes().all(
            |byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-'),
        ) {
            return Err(BytecodeError::new(
                offset,
                "AE-HOST-004: guest path contains an illegal segment",
            ));
        }
    }
    Ok(())
}

struct RuntimeState {
    arena: ArenaState,
    nurseries: Vec<NurseryFrame>,
    v12_nurseries: Vec<V12NurseryFrame>,
    resumed_task: Option<TaskFrame>,
    active_task_lane: Option<TaskArenaLane>,
    #[cfg(test)]
    task_frame_trace: Vec<TaskFrameTraceEvent>,
    /// Test-only proof of deterministic task-local destruction order.  This is
    /// deliberately unavailable to guest code and is populated only while a
    /// real task frame is being torn down.
    #[cfg(test)]
    task_destroyed_local_slots: Vec<usize>,
    /// Test-only snapshots proving that terminal handles do not implicitly
    /// destroy parent-owned resources while their callee runs.
    #[cfg(test)]
    handle_live_resource_slots: Vec<Vec<usize>>,
    hosts: HostServices,
}

struct ArenaState {
    bytes: Vec<u8>,
    used: usize,
}

impl RuntimeState {
    fn with_hosts(capacity: u32, hosts: HostServices) -> Result<Self, BytecodeError> {
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
            nurseries: Vec::new(),
            v12_nurseries: Vec::new(),
            resumed_task: None,
            active_task_lane: None,
            #[cfg(test)]
            task_frame_trace: Vec::new(),
            #[cfg(test)]
            task_destroyed_local_slots: Vec::new(),
            #[cfg(test)]
            handle_live_resource_slots: Vec::new(),
            hosts,
        })
    }

    /// Reserve bytes in either the ordinary monotonic arena or the currently
    /// executing task's pre-admitted private lane.  Returning `None` preserves
    /// the existing M2 allocation-outcome behavior without leaking a sibling
    /// lane or advancing a cursor on failure.
    fn reserve_arena_bytes(&mut self, required: usize) -> Option<usize> {
        if let Some(lane) = self.active_task_lane.as_mut() {
            let lane_end = lane.used.checked_add(required)?;
            if lane_end > lane.capacity {
                return None;
            }
            let start = lane.start.checked_add(lane.used)?;
            let end = start.checked_add(required)?;
            if end > self.arena.bytes.len() {
                return None;
            }
            lane.used = lane_end;
            return Some(start);
        }

        let end = self.arena.used.checked_add(required)?;
        if end > self.arena.bytes.len() {
            return None;
        }
        let start = self.arena.used;
        self.arena.used = end;
        Some(start)
    }

    fn reserve_task_slab(
        &mut self,
        capacity: usize,
        offset: usize,
    ) -> Result<usize, BytecodeError> {
        let start = self.arena.used;
        let end = start
            .checked_add(capacity)
            .ok_or_else(|| BytecodeError::new(offset, "task nursery slab capacity overflowed"))?;
        if end > self.arena.bytes.len() {
            return Err(BytecodeError::new(
                offset,
                "AETH v12 task nursery admission exceeded its verified arena capacity",
            ));
        }
        self.arena.used = end;
        Ok(start)
    }

    fn zero_arena_range(
        &mut self,
        start: usize,
        capacity: usize,
        offset: usize,
    ) -> Result<(), BytecodeError> {
        let end = start
            .checked_add(capacity)
            .ok_or_else(|| BytecodeError::new(offset, "task arena lane range overflowed"))?;
        let bytes = self.arena.bytes.get_mut(start..end).ok_or_else(|| {
            BytecodeError::new(offset, "task arena lane lies outside the reserved arena")
        })?;
        bytes.fill(0);
        Ok(())
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
            Ok(RuntimeValue::Text(RuntimeText::new(text.clone())))
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
        RuntimeValue::Text(value) => Ok(InvocationValue::Text(value.into_string())),
        RuntimeValue::Whole(value) => Ok(InvocationValue::Whole(value)),
        RuntimeValue::Truth(value) => Ok(InvocationValue::Truth(value)),
        RuntimeValue::Bytes(value) => Ok(InvocationValue::Bytes(value)),
        RuntimeValue::Record { .. } => Err(BytecodeError::new(
            0,
            "host invocation cannot return a record; project a primitive field inside Aether",
        )),
        RuntimeValue::Arena
        | RuntimeValue::AccessArena
        | RuntimeValue::Buffer { .. }
        | RuntimeValue::Table { .. } => Err(BytecodeError::new(
            0,
            "host invocation cannot return an Aether resource value",
        )),
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
    Table {
        shape: u16,
        layout: TableLayout,
    },
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
    TableAllocate {
        destination: usize,
    },
    TableStore {
        shape: u16,
        field: u8,
        destination: usize,
    },
    TableLoad {
        shape: u16,
        field: u8,
        destination: usize,
    },
    TableCount,
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
    NurseryBegin {
        count: u8,
    },
    NurserySpawn {
        function: usize,
        arguments: usize,
        destination: usize,
    },
    NurseryEnd,
    /// AETH v12 cooperative task suspension point.
    TaskCheckpoint,
    Call {
        function: usize,
        arguments: usize,
    },
    /// AETH v11 host service invoke. `function` is the function-table index of a host entry.
    HostCall {
        function: usize,
        arguments: usize,
    },
    /// M19a: destroy live owner at local slot.
    Release(usize),
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
    if index < lines.len() && lines[index].content.starts_with("import unit ") {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "AE-MOD-001: import unit must begin at indentation level zero",
            ));
        }
        return Err(CompilerError::new(
            line.span(1),
            "AE-MOD-007: import unit requires aether project build (multi-module); single-file compile rejects imports",
        ));
    }
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
    let mut shapes = Vec::new();
    while index < lines.len() && lines[index].content.starts_with("shape ") {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "a shape declaration must begin at indentation level zero",
            ));
        }
        shapes.push(parse_shape_declaration(&lines, &mut index)?);
    }
    let record_types = record_type_map(&records)?;
    let mut host_weaves = Vec::new();
    while index < lines.len()
        && (lines[index].content.starts_with("host weave ")
            || lines[index].content.starts_with("foreign weave "))
    {
        let line = lines[index];
        if line.indentation != 0 {
            return Err(CompilerError::new(
                line.span(1),
                "AE-HOST-001: a host or foreign weave declaration must begin at indentation level zero",
            ));
        }
        if line.content.starts_with("foreign weave ") {
            host_weaves.push(parse_foreign_weave_header(line, &record_types)?);
        } else {
            host_weaves.push(parse_host_weave_header(line, &record_types)?);
        }
        index += 1;
    }
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
                "record declarations must appear after world and before every shape or weave",
            ));
        }
        if line.content.starts_with("shape ") {
            return Err(CompilerError::new(
                line.span(1),
                "shape declarations must appear after records and before every weave",
            ));
        }
        if line.content.starts_with("host weave ") || line.content.starts_with("foreign weave ") {
            return Err(CompilerError::new(
                line.span(1),
                "AE-HOST-001: host/foreign weave declarations must appear after shapes and before every ordinary weave",
            ));
        }
        if line.content.starts_with("host ") {
            return Err(CompilerError::new(
                line.span(1),
                "AE-HOST-001: host declarations must use the form host weave name [params] -> Type",
            ));
        }
        if line.content.starts_with("foreign ") {
            return Err(CompilerError::new(
                line.span(1),
                "AE-FFI-001: foreign declarations must use foreign weave name [params] -> Type from \"lib\" symbol \"name\"",
            ));
        }
        if line.content.starts_with("import unit ") {
            return Err(CompilerError::new(
                line.span(1),
                "AE-MOD-001: import unit declarations must appear immediately after world",
            ));
        }
        let (name, parameters, result, effect, task) = parse_weave_header(line, &record_types)?;
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
            task,
            body,
            span: line.span(1),
        });
    }

    let program = Program {
        world,
        records,
        shapes,
        host_weaves,
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

/// Product emission: seed forge + verify only (BARP Phase 2 / ADR-044).
///
/// Does **not** bootstrap-parse or bootstrap-validate. Invalid Seed Profile
/// inputs fail at seed forge or [`verify_bytecode`]. Full diagnostics remain
/// bootstrap authority via CLI `check` (default). Product-path errors use bounded
/// `AE-SEED-*` codes (ADR-046–055). Prefer [`product_diagnostics`] for structured
/// product diagnostic collection without bytecode.
pub fn compile_product_bytecode(source: &str) -> Result<Vec<u8>, CompilerError> {
    debug_assert!(
        seed_interprets_m23_comptime_calls_natively(),
        "BARP Phase 1 requires seed-native M23; materialization bridge removed"
    );
    debug_assert!(
        product_path_forges_before_bootstrap_validate(),
        "BARP Phase 2 requires forge-first product emission"
    );
    if seed_product_diagnostics_subset() {
        if let Some(message) = seed_reject_empty_source(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        if let Some(message) = seed_reject_raw_import_unit(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        if let Some(message) = seed_reject_missing_world(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        if let Some(message) = seed_reject_legacy_syntax_heuristics(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        if let Some(message) = seed_reject_odd_indentation(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        if let Some(message) = seed_reject_missing_main_weave(source) {
            return Err(CompilerError::new(Span::synthetic(), message));
        }
        // ADR-070: fail closed before forge when truth-choose yields (bootstrap
        // rejects; seed otherwise emits broken jumps).
        if product_rejects_yield_in_truth_choose() {
            if let Some(message) = seed_reject_yield_in_truth_choose(source) {
                return Err(CompilerError::new(Span::synthetic(), message));
            }
        }
    }
    let forged = forge_bytecode(SEED_COMPILER_ARTIFACT, source).map_err(|error| {
        let detail = error.to_string();
        CompilerError::new(
            Span::synthetic(),
            format_seed_product_error(classify_seed_forge_error(&detail), &detail),
        )
    })?;
    let InvocationValue::Bytes(bytecode) = forged.value else {
        return Err(CompilerError::new(
            Span::synthetic(),
            format_seed_product_error("AE-SEED-001", "seed compiler must yield Bytes"),
        ));
    };
    verify_bytecode(&bytecode).map_err(|error| {
        let detail = error.to_string();
        CompilerError::new(
            Span::synthetic(),
            format_seed_product_error(classify_seed_verify_error(&detail), &detail),
        )
    })?;
    Ok(bytecode)
}

/// BARP ADR-055: structured **product** diagnostic collection (seed path).
///
/// Returns zero diagnostics when product accept succeeds. Failures yield stable
/// `AE-SEED-*` codes. This is the host-facing product diagnostic ABI — not a
/// claim that the seed binary emits structured error packets on every path
/// (opaque VM failures are classified into AE-SEED-008/001). Bootstrap full
/// diagnostics remain CLI `check` / optional LSP bootstrap mode.
#[must_use]
pub fn product_diagnostics(source: &str) -> Vec<Diagnostic> {
    debug_assert!(
        product_diagnostic_abi(),
        "ADR-055: product diagnostic ABI tracker"
    );
    match compile_product_bytecode(source) {
        Ok(_) => Vec::new(),
        Err(error) => vec![error.diagnostic()],
    }
}

fn format_seed_product_error(code: &str, detail: &str) -> String {
    format!("{code}: product seed path failed ({detail}). Full diagnostics: aether check <source>")
}

/// BARP Phase 3c (ADR-052): map forge/VM detail strings to stable product codes
/// without claiming full bootstrap diagnostic parity.
fn classify_seed_forge_error(detail: &str) -> &'static str {
    let normalized = detail.to_ascii_lowercase();
    if normalized.contains("unknown weave")
        || normalized.contains("no weave named")
        || normalized.contains("call references an unknown")
    {
        return "AE-SEED-011";
    }
    // Opaque seed VM failures often mean incomplete parse/bind (e.g. unbound name).
    if normalized.contains("unpack16")
        || normalized.contains("unpack32")
        || normalized.contains("index is invalid")
        || normalized.contains("seek ")
        || normalized.contains("poke ")
    {
        return "AE-SEED-008";
    }
    "AE-SEED-001"
}

fn classify_seed_verify_error(detail: &str) -> &'static str {
    let normalized = detail.to_ascii_lowercase();
    if normalized.contains("unknown weave")
        || normalized.contains("no weave named")
        || normalized.contains("call references an unknown")
    {
        return "AE-SEED-011";
    }
    // Residual: if preflight misses, map illegal yield-in-choose seed emit noise.
    if normalized.contains("jump target") || normalized.contains("unreachable aether instructions")
    {
        return "AE-SEED-013";
    }
    if normalized.contains("requires whole")
        || normalized.contains("stack has")
        || normalized.contains("type mismatch")
        || normalized.contains("wrong type")
        || normalized.contains("expected whole")
        || normalized.contains("expected text")
        || normalized.contains("expected truth")
        || normalized.contains("expected bytes")
    {
        return "AE-SEED-010";
    }
    "AE-SEED-002"
}

/// BARP Phase 3b (ADR-050): empty product input.
fn seed_reject_empty_source(source: &str) -> Option<String> {
    if source.trim().is_empty() {
        return Some(format_seed_product_error("AE-SEED-005", "source is empty"));
    }
    None
}

/// BARP ADR-055/056: single-file product path rejects raw multi-module surface.
/// Multi-module product path is host elaborate + seed emit (`aether project build`).
fn seed_reject_raw_import_unit(source: &str) -> Option<String> {
    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("import unit ") {
            return Some(format_seed_product_error(
                "AE-SEED-012",
                &format!(
                    "line {}: raw import unit is multi-module surface — use `aether project build` (host elaborate + seed emit; seed does not elaborate multi-file natively)",
                    line_index + 1
                ),
            ));
        }
    }
    None
}

/// BARP Phase 3b (ADR-050): require a top-level `world` declaration line.
fn seed_reject_missing_world(source: &str) -> Option<String> {
    let has_world = source.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("world ")
    });
    if !has_world {
        return Some(format_seed_product_error(
            "AE-SEED-006",
            "program requires a top-level world declaration",
        ));
    }
    None
}

/// BARP Phase 3b (ADR-050): reject obvious non-Aether / legacy syntax without
/// running the bootstrap compiler.
fn seed_reject_legacy_syntax_heuristics(source: &str) -> Option<String> {
    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        // Aether admits `import unit …`; reject other `import` shapes as legacy.
        let non_aether_import =
            trimmed.starts_with("import ") && !trimmed.starts_with("import unit ");
        let legacy = trimmed.starts_with("fn ")
            || trimmed.starts_with("fn\t")
            || trimmed.starts_with("return ")
            || trimmed.starts_with("return;")
            || trimmed.starts_with("let ")
            || trimmed.starts_with("pub ")
            || trimmed.starts_with("use ")
            || non_aether_import
            || trimmed.contains("-> Int")
            || trimmed.contains("-> i64");
        // Aether does not use brace blocks; `{`/`}` on a line is legacy/hostile.
        let brace = trimmed.contains('{') || trimmed.contains('}');
        if legacy || brace {
            return Some(format_seed_product_error(
                "AE-SEED-007",
                &format!(
                    "line {}: source looks like legacy or non-Aether syntax (use Aether forms; aether check for details)",
                    line_index + 1
                ),
            ));
        }
    }
    None
}

/// BARP Phase 3a: reject indentation that is not a multiple of two spaces
/// before forge (seed and product path honesty; bootstrap already rejects).
fn seed_reject_odd_indentation(source: &str) -> Option<String> {
    for (line_index, line) in source.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let spaces = line.chars().take_while(|c| *c == ' ').count();
        if spaces > 0 && spaces % 2 == 1 {
            return Some(format_seed_product_error(
                "AE-SEED-003",
                &format!(
                    "line {}: Aether indentation uses exact two-space levels (odd leading spaces)",
                    line_index + 1
                ),
            ));
        }
        if line.starts_with('\t') {
            return Some(format_seed_product_error(
                "AE-SEED-003",
                &format!(
                    "line {}: Aether indentation uses exact two-space levels (tabs forbidden)",
                    line_index + 1
                ),
            ));
        }
    }
    None
}

/// BARP Phase 3b: fail closed when no top-level `weave main` / `task weave main`
/// is present (host preflight — not bootstrap semantic analysis).
fn seed_reject_missing_main_weave(source: &str) -> Option<String> {
    let mut saw_weave = false;
    let mut saw_main = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("weave ") || trimmed.starts_with("task weave ") {
            saw_weave = true;
            let after_task = trimmed.strip_prefix("task ").unwrap_or(trimmed);
            if let Some(rest) = after_task.strip_prefix("weave ") {
                let name = rest.split([' ', '[']).next().unwrap_or("");
                if name == "main" {
                    saw_main = true;
                    break;
                }
            }
        }
    }
    if saw_weave && !saw_main {
        return Some(format_seed_product_error(
            "AE-SEED-004",
            "program requires a top-level weave main (or task weave main)",
        ));
    }
    None
}

/// BARP ADR-070: reject `yield` inside truth-condition `choose` branches.
///
/// Bootstrap rejects this as "yield is allowed only as the final statement of a
/// weave root". Resource `choose allocate|append|at|store|load|…` may terminate
/// branches with `yield` (M2/M6). Seed historically emitted broken jumps for
/// truth-choose yields (showcase multi-module discovery). Product path fails
/// closed before forge with `AE-SEED-013` — no bootstrap AST required.
fn seed_reject_yield_in_truth_choose(source: &str) -> Option<String> {
    debug_assert!(
        product_rejects_yield_in_truth_choose(),
        "ADR-070: product rejects yield in truth-choose"
    );
    // Stack of (choose_indent, is_truth_choose) for open choose regions.
    let mut choose_stack: Vec<(usize, bool)> = Vec::new();
    for (line_index, line) in source.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.chars().take_while(|c| *c == ' ').count();
        let trimmed = line.trim_start();
        // Pop closed chooses when indent returns to or above their level.
        while choose_stack
            .last()
            .is_some_and(|(choose_indent, _)| indent <= *choose_indent)
        {
            choose_stack.pop();
        }
        if let Some(rest) = trimmed.strip_prefix("choose ") {
            let is_truth = rest.starts_with("same ")
                || rest.starts_with("less ")
                || rest.starts_with("bright")
                || rest.starts_with("dim")
                || rest.starts_with("not ")
                // bare `choose <name>` truth variable form (not resource op).
                || (!rest.starts_with("allocate ")
                    && !rest.starts_with("append ")
                    && !rest.starts_with("at ")
                    && !rest.starts_with("store ")
                    && !rest.starts_with("load ")
                    && !rest.starts_with("lookup "));
            choose_stack.push((indent, is_truth));
            continue;
        }
        if (trimmed.starts_with("yield ") || trimmed == "yield")
            && choose_stack.iter().any(|(_, is_truth)| *is_truth)
        {
            return Some(format_seed_product_error(
                "AE-SEED-013",
                &format!(
                    "line {}: yield is not allowed inside a truth-condition choose branch (revise then yield at weave root; resource choose allocate/append/… may yield). aether check --bootstrap for full AST diagnostics",
                    line_index + 1
                ),
            ));
        }
    }
    None
}

/// Compile source with the Aether-written seed compiler (product path).
///
/// **Product success** is seed forge + verify only ([`compile_product_bytecode`];
/// ADR-044/049/051). This path **never** invokes the bootstrap compiler.
///
/// [`CompileOutput::program`] is always an **empty placeholder** and must not be
/// used as a semantic document — call [`compile_source`] / CLI `check` (bootstrap)
/// for AST diagnostics, or dual-compare oracles via [`compile_to_bytecode`].
/// Prefer [`compile_product_bytecode`] when only bytes are required.
pub fn compile_with_seed(source: &str) -> Result<CompileOutput, CompilerError> {
    debug_assert!(
        compile_with_seed_product_authoritative(),
        "ADR-049: compile_with_seed product success must not require bootstrap"
    );
    debug_assert!(
        !compile_with_seed_invokes_bootstrap(),
        "ADR-051: compile_with_seed must not invoke bootstrap"
    );
    let bytecode = compile_product_bytecode(source)?;
    Ok(CompileOutput {
        program: Program {
            world: String::new(),
            records: Vec::new(),
            shapes: Vec::new(),
            host_weaves: Vec::new(),
            weaves: Vec::new(),
        },
        bytecode,
    })
}

/// BARP Phase 1: seed evaluates M23 pure comptime calls (D2a body subset) and
/// emits `COMPTIME_WHOLE` without a bootstrap materialization rewrite.
#[must_use]
pub const fn seed_interprets_m23_comptime_calls_natively() -> bool {
    true
}

/// BARP Phase 2: product emission forges seed before any bootstrap validate.
/// Bootstrap remains `check`/AST diagnostics, seed rebuild, and dual-compare oracle.
#[must_use]
pub const fn product_path_forges_before_bootstrap_validate() -> bool {
    true
}

/// BARP ADR-045: product multi-module/project emit does **not** require
/// bootstrap dual-compare on every build (oracle remains tests/gate).
#[must_use]
pub const fn product_path_requires_bootstrap_dual_compare() -> bool {
    false
}

/// BARP Phase 3a (ADR-046): bounded product-path `AE-SEED-*` diagnostics subset
/// (not full bootstrap diagnostic parity).
#[must_use]
pub const fn seed_product_diagnostics_subset() -> bool {
    true
}

/// BARP ADR-047: multi-module product emit does **not** invoke the bootstrap
/// compiler (no parse/validate/AST fill on the product path).
#[must_use]
pub const fn product_multi_module_invokes_bootstrap() -> bool {
    false
}

/// BARP ADR-048: structural-edit post-edit accept gate is product seed, not
/// bootstrap re-validate (base parse remains bootstrap for authoring AST).
#[must_use]
pub const fn structural_edit_accepts_via_product_seed() -> bool {
    true
}

/// BARP ADR-049: [`compile_with_seed`] product success is seed-only; bootstrap
/// AST is best-effort fill and does not gate product Ok.
#[must_use]
pub const fn compile_with_seed_product_authoritative() -> bool {
    true
}

/// BARP ADR-050: Phase 3b product preflight codes AE-SEED-005–007 (empty, world,
/// legacy heuristics) in addition to 001–004.
#[must_use]
pub const fn seed_product_preflight_phase3b() -> bool {
    true
}

/// BARP ADR-051: [`compile_with_seed`] never calls bootstrap (empty Program only).
#[must_use]
pub const fn compile_with_seed_invokes_bootstrap() -> bool {
    false
}

/// BARP ADR-051: CLI `aether check --product` validates via product seed only.
#[must_use]
pub const fn product_cli_check_without_bootstrap() -> bool {
    true
}

/// BARP ADR-052: project-verify lib units validate via product seed probe.
#[must_use]
pub const fn lib_module_validates_via_product_seed() -> bool {
    true
}

/// BARP ADR-052 Phase 3c: product forge/verify map to AE-SEED-008/010/011 subset.
#[must_use]
pub const fn seed_product_diagnostics_phase3c() -> bool {
    true
}

/// BARP ADR-053: [`format_source_product`] / `format --product` is seed-only (LF + accept).
#[must_use]
pub const fn product_format_without_bootstrap() -> bool {
    true
}

/// BARP ADR-053: CLI apply-edit trusts core product accept (no second forge).
#[must_use]
pub const fn apply_edit_cli_trusts_product_accept() -> bool {
    true
}

/// BARP ADR-054: `project format --product` formats units via product seed only.
#[must_use]
pub const fn product_project_format_without_bootstrap() -> bool {
    true
}

/// BARP ADR-054: `structure --product` / [`product_structure_json`] is seed-only.
#[must_use]
pub const fn product_structure_without_bootstrap() -> bool {
    true
}

/// BARP ADR-055: structured product diagnostic ABI ([`product_diagnostics`]).
#[must_use]
pub const fn product_diagnostic_abi() -> bool {
    true
}

/// BARP ADR-056: multi-module product path is host elaborate + seed emit.
/// Seed-native multi-file elaboration is **not** available (no multi-file forge ABI).
#[must_use]
pub const fn host_elaborates_modules_seed_emits() -> bool {
    true
}

/// BARP ADR-056 honesty: seed does **not** natively elaborate multi-module graphs.
#[must_use]
pub const fn seed_native_multi_module_elaboration() -> bool {
    false
}

/// BARP ADR-057: structural-edit prefers product diagnostics when base fails both paths.
#[must_use]
pub const fn structural_edit_product_base_gate() -> bool {
    true
}

/// BARP ADR-058: LSP product-path diagnostics are primary (seed AE-SEED codes).
#[must_use]
pub const fn lsp_product_diagnostics_primary() -> bool {
    true
}

/// BARP ADR-061 honesty: seed-internal structured error packets not yet implemented.
#[must_use]
pub const fn seed_internal_error_packets() -> bool {
    false
}

/// BARP ADR-062: default `format_source` prefers product AE-SEED when both reject.
#[must_use]
pub const fn format_source_product_base_gate() -> bool {
    true
}

/// BARP ADR-063: default `check` prefers product AE-SEED when both reject.
#[must_use]
pub const fn check_product_base_gate() -> bool {
    true
}

/// BARP ADR-063: product-surface symbols available without bootstrap AST.
#[must_use]
pub const fn product_surface_symbols_without_bootstrap() -> bool {
    true
}

/// BARP ADR-064: user-facing CLI defaults to product seed path (check/format/structure/project format).
#[must_use]
pub const fn product_default_cli_toolchain() -> bool {
    true
}

/// BARP ADR-064–070 honesty: bootstrap is recovery/oracle only — not the
/// default product toolchain. Residual bootstrap roles after ADR-070:
/// dual-compare oracle, recovery flags (`--bootstrap`), nested body-list
/// structural edits, and full `aether.ast/v8`. Product owns seed rebuild,
/// structural product ops, multi-module choose-revise, yield-in-truth-choose
/// fail-closed preflight, and LSP product-surface navigation.
#[must_use]
pub const fn bootstrap_is_recovery_oracle_only() -> bool {
    true
}

/// BARP ADR-065: top-level weave `replace` on product-accepted base source does
/// not require bootstrap `Program` base parse (text splice + product accept).
#[must_use]
pub const fn structural_edit_product_weave_replace() -> bool {
    true
}

/// BARP ADR-068: top-level weave `replace` / `insertAfter` / `delete` on
/// product-accepted base source without bootstrap `Program` base parse.
#[must_use]
pub const fn structural_edit_product_top_level_weave_ops() -> bool {
    true
}

/// BARP ADR-069: weave-body statement ops and top-level primitive record ops
/// without bootstrap `Program` base parse (nested choose/while lists residual).
#[must_use]
pub const fn structural_edit_product_statement_and_record_ops() -> bool {
    true
}

/// BARP ADR-066: LSP hover/definition use product-surface symbols for local
/// weaves/records/world (no bootstrap AST for Plain navigation).
#[must_use]
pub const fn lsp_product_surface_hover_definition() -> bool {
    true
}

/// BARP ADR-067: product seed rebuild is `compile` without `--bootstrap`
/// (`compile_product_bytecode`); dual-compare oracle may still use `--bootstrap`.
#[must_use]
pub const fn product_seed_rebuild_without_bootstrap() -> bool {
    true
}

/// BARP ADR-070: product path rejects `yield` inside truth-condition `choose`
/// without bootstrap AST (fail closed before seed forge).
#[must_use]
pub const fn product_rejects_yield_in_truth_choose() -> bool {
    true
}

/// BARP ADR-070 honesty: multi-module product path supports truth-`choose` with
/// `revise` then root `yield` (showcase lesson); yield-in-truth-choose is illegal.
#[must_use]
pub const fn multi_module_product_choose_revise_supported() -> bool {
    true
}

/// One top-level name discovered from product-accepted source without bootstrap AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductSurfaceSymbol {
    pub name: String,
    pub kind: &'static str,
    /// 0-based line for LSP.
    pub line: u32,
    /// 0-based UTF-16-ish column (byte index of name start on the line for ASCII).
    pub column: u32,
}

/// Product-path symbol scan (ADR-063): require product accept, then scan for
/// top-level `world`, `weave`/`task weave`/`export weave`, and `record` names.
///
/// Not a full AST; no nested body symbols. Prefer bootstrap `structure` for
/// complete authoring trees.
pub fn product_surface_symbols(source: &str) -> Result<Vec<ProductSurfaceSymbol>, CompilerError> {
    debug_assert!(
        product_surface_symbols_without_bootstrap(),
        "ADR-063: product surface symbols without bootstrap"
    );
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    compile_product_bytecode(&normalized)?;
    let mut symbols = Vec::new();
    for (line_index, line) in normalized.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let line_u32 = line_index as u32;
        if let Some(rest) = trimmed.strip_prefix("world ") {
            let name = rest.split_whitespace().next().unwrap_or("").to_owned();
            if !name.is_empty() {
                let column = (line.find("world ").unwrap_or(0) + 6) as u32;
                symbols.push(ProductSurfaceSymbol {
                    name,
                    kind: "world",
                    line: line_u32,
                    column,
                });
            }
            continue;
        }
        let after_task = trimmed.strip_prefix("task ").unwrap_or(trimmed);
        let after_export = after_task.strip_prefix("export ").unwrap_or(after_task);
        if let Some(rest) = after_export.strip_prefix("weave ") {
            let name = rest.split([' ', '[']).next().unwrap_or("").to_owned();
            if !name.is_empty() {
                let column = (line.find(&name).unwrap_or(0)) as u32;
                symbols.push(ProductSurfaceSymbol {
                    name,
                    kind: "weave",
                    line: line_u32,
                    column,
                });
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("record ") {
            let name = rest.split([' ', '[']).next().unwrap_or("").to_owned();
            if !name.is_empty() {
                let column = (line.find(&name).unwrap_or(0)) as u32;
                symbols.push(ProductSurfaceSymbol {
                    name,
                    kind: "record",
                    line: line_u32,
                    column,
                });
            }
        }
    }
    Ok(symbols)
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
    for shape in &program.shapes {
        formatted.push('\n');
        formatted.push_str("shape ");
        formatted.push_str(&shape.name);
        formatted.push_str(":\n");
        for field in &shape.fields {
            formatted.push_str("  ");
            formatted.push_str(&field.name);
            formatted.push_str(" Whole\n");
        }
    }
    for host in &program.host_weaves {
        formatted.push('\n');
        if host.foreign.is_some() {
            formatted.push_str("foreign weave ");
        } else {
            formatted.push_str("host weave ");
        }
        formatted.push_str(&host.name);
        formatted.push_str(" [");
        for (index, parameter) in host.parameters.iter().enumerate() {
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
        formatted.push_str(&format_value_type(host.result, &program.records));
        if let Some(foreign) = &host.foreign {
            formatted.push_str(" from \"");
            formatted.push_str(&foreign.library);
            formatted.push_str("\" symbol \"");
            formatted.push_str(&foreign.symbol);
            formatted.push('"');
        }
        formatted.push('\n');
    }
    for weave in &program.weaves {
        formatted.push('\n');
        if weave.task {
            formatted.push_str("task weave ");
        } else {
            formatted.push_str("weave ");
        }
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
    for shape in &program.shapes {
        output.push_str(";Shape(");
        output.push_str(&shape.name);
        output.push_str(")[");
        for (index, field) in shape.fields.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(&field.name);
            output.push_str(":Whole");
        }
        output.push(']');
    }
    for host in &program.host_weaves {
        output.push_str(";HostWeave(");
        output.push_str(&host.name);
        output.push_str("->");
        output.push_str(&format_value_type(host.result, &program.records));
        output.push_str(")[");
        for (index, parameter) in host.parameters.iter().enumerate() {
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
        if weave.task {
            output.push_str("#Task");
        }
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
    if main.is_host() {
        return Err(BytecodeError::new(0, "main cannot be a host weave"));
    }
    if !main.parameters.is_empty()
        || main.result != ValueType::Whole
        || main.effect != Effect::Total
    {
        return Err(BytecodeError::new(
            0,
            "main must accept no parameters, remain total, and yield Whole",
        ));
    }

    if artifact.version == ARTIFACT_VERSION_V12 {
        validate_v12_function_metadata(&artifact, main_index)?;
    }

    verify_resource_plan(&artifact, main_index)?;

    for (function_index, function) in artifact.functions.iter().enumerate() {
        verify_function(
            function_index,
            function,
            &artifact.functions,
            &artifact.records,
            &artifact.shapes,
            artifact.version,
        )?;
    }
    Ok(())
}

fn validate_v12_function_metadata(
    artifact: &Artifact,
    main_index: usize,
) -> Result<(), BytecodeError> {
    for (index, function) in artifact.functions.iter().enumerate() {
        if function.is_host() {
            if function.flags != 0 || function.frame_arena_capacity != 0 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 host weave metadata must have zero flags and zero frame arena capacity",
                ));
            }
            continue;
        }
        if function.is_task() {
            if index == main_index
                || function.effect != Effect::Total
                || function.result != ValueType::Whole
                || function.parameters.iter().any(|(value_type, mode)| {
                    *mode != ParameterMode::Own
                        || !matches!(value_type, ValueType::Whole | ValueType::Truth)
                })
            {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 task frame must be a non-main total Whole guest with owned Whole or Truth parameters",
                ));
            }
        } else {
            if function.flags != 0 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 non-task guest weave has task-frame flags",
                ));
            }
            if index != main_index && function.frame_arena_capacity != 0 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 only main or a task frame may declare direct arena capacity",
                ));
            }
        }
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
    if is_table_type(function.result)
        || function
            .parameters
            .iter()
            .any(|(value_type, _)| is_table_type(*value_type))
    {
        return Err(BytecodeError::new(
            0,
            "AETH v9 M6 does not permit Table values as weave parameters or results",
        ));
    }
    if has_buffer
        && (!matches!(
            version,
            ARTIFACT_VERSION_V6
                | ARTIFACT_VERSION_V7
                | ARTIFACT_VERSION_V8
                | ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) || access_count != 1)
    {
        return Err(BytecodeError::new(
            0,
            "artifact Buffer signatures require exactly one AETH v6 through v12 access Arena parameter",
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
    if !matches!(
        version,
        ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) && function.effect != Effect::Total
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
    if artifact.version == ARTIFACT_VERSION_V12 {
        return verify_v12_resource_plan(artifact, main_index);
    }
    let mut arena_declarations = 0_usize;
    let mut resource_instruction_seen = false;
    for (function_index, function) in artifact.functions.iter().enumerate() {
        for instruction in decode_code(&function.code, artifact.version)? {
            match &instruction.instruction {
                Instruction::Arena => {
                    // M19d: OP_ARENA may appear in any function (self-owned arena).
                    let _ = function_index;
                    let _ = main_index;
                    arena_declarations += 1;
                    resource_instruction_seen = true;
                }
                Instruction::Buffer(_)
                | Instruction::Table { .. }
                | Instruction::Access(_)
                | Instruction::Allocate { .. }
                | Instruction::BufferAppend { .. }
                | Instruction::BufferAt { .. }
                | Instruction::TableAllocate { .. }
                | Instruction::TableStore { .. }
                | Instruction::TableLoad { .. }
                | Instruction::TableCount
                | Instruction::Count => resource_instruction_seen = true,
                _ => {}
            }
        }
    }
    if !matches!(
        artifact.version,
        ARTIFACT_VERSION_V6
            | ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
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
    if artifact.arena_capacity > 0 && arena_declarations == 0 {
        return Err(BytecodeError::new(
            0,
            "AETH v6 through v11 nonzero arena plan requires at least one arena declaration",
        ));
    }
    if artifact.arena_capacity == 0 && arena_declarations != 0 {
        return Err(BytecodeError::new(
            0,
            "AETH v6 through v11 zero arena plan cannot declare an Arena capability",
        ));
    }
    Ok(())
}

fn instruction_uses_resource(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Arena
            | Instruction::Buffer(_)
            | Instruction::Table { .. }
            | Instruction::Access(_)
            | Instruction::Allocate { .. }
            | Instruction::BufferAppend { .. }
            | Instruction::BufferAt { .. }
            | Instruction::TableAllocate { .. }
            | Instruction::TableStore { .. }
            | Instruction::TableLoad { .. }
            | Instruction::TableCount
            | Instruction::Count
    )
}

fn v12_companion_is_eligible(function: &ArtifactFunction, decoded: &[DecodedInstruction]) -> bool {
    if function.is_host()
        || function.is_task()
        || function.result != ValueType::Whole
        || function.parameters.iter().any(|(value_type, mode)| {
            *mode != ParameterMode::Own
                || !matches!(value_type, ValueType::Whole | ValueType::Truth)
        })
        || function
            .locals
            .iter()
            .any(|local| !matches!(local.value_type, ValueType::Whole | ValueType::Truth))
    {
        return false;
    }
    decoded.iter().all(|instruction| {
        matches!(
            instruction.instruction,
            Instruction::PushWhole(_)
                | Instruction::PushTruth(_)
                | Instruction::Store(_)
                | Instruction::Load(_)
                | Instruction::Move(_)
                | Instruction::Revise(_)
                | Instruction::Yield
                | Instruction::Sum
                | Instruction::Difference
                | Instruction::Product
                | Instruction::Quotient
                | Instruction::Remainder
                | Instruction::Less
                | Instruction::Same
                | Instruction::Not
                | Instruction::JumpIfDim(_)
                | Instruction::Jump(_)
                | Instruction::Raise
        )
    })
}

fn verify_v12_resource_plan(artifact: &Artifact, main_index: usize) -> Result<(), BytecodeError> {
    let mut decoded_functions = Vec::with_capacity(artifact.functions.len());
    for function in &artifact.functions {
        decoded_functions.push(decode_code(&function.code, artifact.version)?);
    }

    let mut main_capacity = 0_u32;
    let mut nursery_capacity = 0_u32;
    let task_declared = artifact.functions.iter().any(ArtifactFunction::is_task);

    for (function_index, function) in artifact.functions.iter().enumerate() {
        let decoded = &decoded_functions[function_index];
        let arena_count = decoded
            .iter()
            .filter(|instruction| matches!(instruction.instruction, Instruction::Arena))
            .count();
        let uses_resource = decoded
            .iter()
            .any(|instruction| instruction_uses_resource(&instruction.instruction));

        if function.is_host() {
            continue;
        }
        if function_index == main_index {
            main_capacity = function.frame_arena_capacity;
            if (arena_count == 0) != (main_capacity == 0) {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 main frame arena capacity must agree with its direct Arena declaration",
                ));
            }
            if arena_count > 1 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 main may declare at most one direct Arena capability",
                ));
            }
        } else if function.is_task() {
            if (arena_count == 0) != (function.frame_arena_capacity == 0) {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 task frame arena capacity must agree with its direct Arena declaration",
                ));
            }
            if arena_count > 1 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 task frame may declare at most one direct Arena capability",
                ));
            }
        } else {
            if uses_resource {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 permits resource instructions outside main only in an isolated task frame",
                ));
            }
            if function.frame_arena_capacity != 0 {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 non-task guest weave cannot declare frame arena capacity",
                ));
            }
        }

        if task_declared && function_index != main_index && !function.is_task() && uses_resource {
            return Err(BytecodeError::new(
                0,
                "AETH v12 task artifacts cannot grant a non-main companion resource ownership",
            ));
        }

        let mut nursery_targets: Option<Vec<usize>> = None;
        for instruction in decoded {
            match instruction.instruction {
                Instruction::NurseryBegin { .. } => {
                    if nursery_targets.replace(Vec::new()).is_some() {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "AETH v12 cannot nest nursery regions",
                        ));
                    }
                }
                Instruction::NurserySpawn { function, .. } => {
                    let Some(targets) = nursery_targets.as_mut() else {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "AETH v12 NURSERY_SPAWN has no active nursery region",
                        ));
                    };
                    targets.push(function);
                }
                Instruction::NurseryEnd => {
                    let Some(targets) = nursery_targets.take() else {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "AETH v12 NURSERY_END has no active nursery region",
                        ));
                    };
                    let child_targets = targets
                        .iter()
                        .map(|target| {
                            artifact.functions.get(*target).ok_or_else(|| {
                                BytecodeError::new(
                                    instruction.offset,
                                    "AETH v12 nursery spawn references an unknown weave",
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if child_targets.iter().any(|target| target.is_task()) {
                        let mut lane_sum = 0_u32;
                        for (target, child) in targets.into_iter().zip(child_targets) {
                            if child.is_task() {
                                lane_sum = lane_sum
                                    .checked_add(child.frame_arena_capacity)
                                    .ok_or_else(|| {
                                        BytecodeError::new(
                                            instruction.offset,
                                            "AETH v12 task lane capacity overflowed the M2 limit",
                                        )
                                    })?;
                            } else if !v12_companion_is_eligible(child, &decoded_functions[target])
                            {
                                return Err(BytecodeError::new(
                                    instruction.offset,
                                    "AETH v12 checkpointed nursery child must be a task frame or Copy-only companion",
                                ));
                            }
                        }
                        nursery_capacity = nursery_capacity.max(lane_sum);
                    }
                }
                _ => {}
            }
        }
        if nursery_targets.is_some() {
            return Err(BytecodeError::new(
                0,
                "AETH v12 nursery region is not closed",
            ));
        }
    }

    let required = main_capacity.checked_add(nursery_capacity).ok_or_else(|| {
        BytecodeError::new(
            0,
            "AETH v12 concurrent frame capacity overflowed the M2 limit",
        )
    })?;
    if required > MAX_ARENA_BYTES || artifact.arena_capacity != required {
        return Err(BytecodeError::new(
            0,
            "AETH v12 header arena capacity must equal main direct capacity plus the largest checkpointed nursery lane sum",
        ));
    }
    Ok(())
}

pub fn run_bytecode(bytecode: &[u8]) -> Result<RunOutput, BytecodeError> {
    run_bytecode_with_grants(bytecode, HostGrantConfig::default())
}

/// Run verified bytecode with optional capability grants (M14).
///
/// Empty [`HostGrantConfig`] installs only pure fixtures (`whole_inc`,
/// `text_extent`), preserving M8 behavior.
pub fn run_bytecode_with_grants(
    bytecode: &[u8],
    grants: HostGrantConfig,
) -> Result<RunOutput, BytecodeError> {
    let output = invoke_bytecode_with_grants(bytecode, "main", &[], grants)?;
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
    invoke_bytecode_with_grants(bytecode, weave_name, arguments, HostGrantConfig::default())
}

/// Like [`invoke_bytecode`] with optional host I/O grants (M14).
pub fn invoke_bytecode_with_grants(
    bytecode: &[u8],
    weave_name: &str,
    arguments: &[InvocationValue],
    grants: HostGrantConfig,
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
    invoke_artifact_with_hosts(
        &artifact,
        function_index,
        weave_name,
        arguments,
        HostServices::with_grants(grants),
    )
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
    invoke_artifact_with_hosts(
        artifact,
        function_index,
        weave_name,
        arguments,
        HostServices::pure_fixture(),
    )
}

fn invoke_artifact_with_hosts(
    artifact: &Artifact,
    function_index: usize,
    weave_name: &str,
    arguments: &[InvocationValue],
    hosts: HostServices,
) -> Result<InvocationOutput, BytecodeError> {
    let function = artifact
        .functions
        .get(function_index)
        .ok_or_else(|| BytecodeError::new(0, "artifact invocation index is invalid"))?;
    if function.is_task() {
        return Err(BytecodeError::new(
            0,
            format!(
                "host invocation refuses task weave {weave_name}; task frames may run only inside a checkpointed nursery"
            ),
        ));
    }
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
    let mut runtime_state = RuntimeState::with_hosts(artifact.arena_capacity, hosts)?;
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

fn parse_host_weave_header(
    line: SourceLine<'_>,
    record_types: &BTreeMap<String, u16>,
) -> Result<HostWeave, CompilerError> {
    if line.content.ends_with(':') {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: host weaves declare an external signature only and cannot open a body block",
        ));
    }
    let Some(rest) = line.content.strip_prefix("host weave ") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: expected host weave declaration",
        ));
    };
    if rest.contains(" raises ") {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: host weaves are total and cannot raise Whole",
        ));
    }
    let Some(opening) = rest.find('[') else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: host weave parameters must be enclosed by square brackets",
        ));
    };
    let name = rest[..opening].trim_end();
    validate_name(name, line.span(12), "host weave name", true)?;
    let after_opening = &rest[opening + 1..];
    let Some(closing) = after_opening.find(']') else {
        return Err(CompilerError::new(
            line.span(12 + opening + 1),
            "AE-HOST-001: host weave parameter list is missing its closing bracket",
        ));
    };
    let parameters = parse_parameters(&after_opening[..closing], line, record_types, false)?;
    let after_parameters = after_opening[closing + 1..].trim();
    let Some(result_text) = after_parameters.strip_prefix("-> ") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: host weave result must use -> Type",
        ));
    };
    if result_text.trim() != result_text {
        return Err(CompilerError::new(
            line.span(1),
            "AE-HOST-001: host weave result type must follow -> with a single space",
        ));
    }
    let result = parse_value_type(result_text, line.span(line.content.len()), record_types)?;
    validate_host_abi_types(&parameters, result, line.span(1))?;
    Ok(HostWeave {
        name: name.to_owned(),
        parameters,
        result,
        span: line.span(1),
        foreign: None,
    })
}

fn parse_foreign_weave_header(
    line: SourceLine<'_>,
    record_types: &BTreeMap<String, u16>,
) -> Result<HostWeave, CompilerError> {
    if line.content.ends_with(':') {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weaves declare an external signature only and cannot open a body block",
        ));
    }
    let Some(rest) = line.content.strip_prefix("foreign weave ") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: expected foreign weave declaration",
        ));
    };
    if rest.contains(" raises ") {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weaves are total and cannot raise Whole",
        ));
    }
    let Some(opening) = rest.find('[') else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weave parameters must be enclosed by square brackets",
        ));
    };
    let name = rest[..opening].trim_end();
    validate_name(name, line.span(15), "foreign weave name", true)?;
    let after_opening = &rest[opening + 1..];
    let Some(closing) = after_opening.find(']') else {
        return Err(CompilerError::new(
            line.span(15 + opening + 1),
            "AE-FFI-001: foreign weave parameter list is missing its closing bracket",
        ));
    };
    let parameters = parse_parameters(&after_opening[..closing], line, record_types, false)?;
    let after_parameters = after_opening[closing + 1..].trim();
    // -> Type from "lib" symbol "sym"
    let Some(after_arrow) = after_parameters.strip_prefix("-> ") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weave result must use -> Type from \"lib\" symbol \"name\"",
        ));
    };
    let Some(from_idx) = after_arrow.find(" from \"") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weave requires from \"library_key\"",
        ));
    };
    let result_text = after_arrow[..from_idx].trim_end();
    if result_text.is_empty() || result_text.trim() != result_text {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weave result type must follow -> with a single space",
        ));
    }
    let result = parse_value_type(result_text, line.span(line.content.len()), record_types)?;
    let after_from = &after_arrow[from_idx + " from \"".len()..];
    let Some(lib_end) = after_from.find('"') else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign library key string is not closed",
        ));
    };
    let library = &after_from[..lib_end];
    let after_lib = after_from[lib_end + 1..].trim_start();
    let Some(after_symbol) = after_lib.strip_prefix("symbol \"") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign weave requires symbol \"c_name\"",
        ));
    };
    let Some(sym_end) = after_symbol.find('"') else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: foreign symbol string is not closed",
        ));
    };
    if !after_symbol[sym_end + 1..].trim().is_empty() {
        return Err(CompilerError::new(
            line.span(1),
            "AE-FFI-001: unexpected tokens after foreign symbol",
        ));
    }
    let symbol = &after_symbol[..sym_end];
    validate_foreign_abi_types(&parameters, result, line.span(1))?;
    validate_foreign_ident(library, line.span(1), "library key")?;
    validate_foreign_ident(symbol, line.span(1), "symbol")?;
    Ok(HostWeave {
        name: name.to_owned(),
        parameters,
        result,
        span: line.span(1),
        foreign: Some(ForeignAbi {
            library: library.to_owned(),
            symbol: symbol.to_owned(),
        }),
    })
}

fn validate_foreign_ident(value: &str, span: Span, label: &str) -> Result<(), CompilerError> {
    if value.is_empty() || value.len() > 128 {
        return Err(CompilerError::new(
            span,
            format!("AE-FFI-001: foreign {label} must be 1..=128 characters"),
        ));
    }
    if !value
        .bytes()
        .all(|byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-'))
    {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-FFI-001: foreign {label} may contain only letters, digits, underscore, dot, and hyphen"
            ),
        ));
    }
    Ok(())
}

/// M21 pilot: Whole-only parameters and result (no pointers / Text / resources).
fn validate_foreign_abi_types(
    parameters: &[Parameter],
    result: ValueType,
    span: Span,
) -> Result<(), CompilerError> {
    if parameters.len() > ffi::MAX_FOREIGN_WHOLE_ARGS {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-FFI-001: foreign pilot supports at most {} parameters",
                ffi::MAX_FOREIGN_WHOLE_ARGS
            ),
        ));
    }
    for parameter in parameters {
        if parameter.mode != ParameterMode::Own || parameter.value_type != ValueType::Whole {
            return Err(CompilerError::new(
                parameter.span,
                format!(
                    "AE-FFI-001: foreign pilot parameter {} must be owned Whole",
                    parameter.name
                ),
            ));
        }
    }
    if result != ValueType::Whole {
        return Err(CompilerError::new(
            span,
            "AE-FFI-001: foreign pilot result must be Whole",
        ));
    }
    Ok(())
}

fn validate_host_abi_types(
    parameters: &[Parameter],
    result: ValueType,
    span: Span,
) -> Result<(), CompilerError> {
    for parameter in parameters {
        match (parameter.mode, parameter.value_type) {
            (ParameterMode::Own, ValueType::Whole | ValueType::Truth) => {}
            (ParameterMode::Borrow, ValueType::Text | ValueType::Bytes) => {}
            (ParameterMode::Access, _)
            | (
                _,
                ValueType::Arena
                | ValueType::AccessArena
                | ValueType::BufferWhole
                | ValueType::BufferTruth
                | ValueType::Table(_)
                | ValueType::Record(_),
            ) => {
                return Err(CompilerError::new(
                    parameter.span,
                    format!(
                        "AE-HOST-001: host weave parameter {} cannot use mode {:?} with type {}",
                        parameter.name, parameter.mode, parameter.value_type
                    ),
                ));
            }
            (ParameterMode::Own, ValueType::Text | ValueType::Bytes) => {
                return Err(CompilerError::new(
                    parameter.span,
                    format!(
                        "AE-HOST-001: host weave Text/Bytes parameter {} must be borrowed",
                        parameter.name
                    ),
                ));
            }
            (ParameterMode::Borrow, ValueType::Whole | ValueType::Truth) => {
                return Err(CompilerError::new(
                    parameter.span,
                    format!(
                        "AE-HOST-001: host weave copy parameter {} cannot use borrow",
                        parameter.name
                    ),
                ));
            }
        }
    }
    match result {
        ValueType::Whole | ValueType::Truth | ValueType::Text | ValueType::Bytes => Ok(()),
        ValueType::Record(_)
        | ValueType::Arena
        | ValueType::AccessArena
        | ValueType::BufferWhole
        | ValueType::BufferTruth
        | ValueType::Table(_) => Err(CompilerError::new(
            span,
            format!("AE-HOST-001: host weave result cannot be {result}"),
        )),
    }
}

fn parse_weave_header(
    line: SourceLine<'_>,
    record_types: &BTreeMap<String, u16>,
) -> Result<(String, Vec<Parameter>, ValueType, Effect, bool), CompilerError> {
    let Some(without_colon) = line.content.strip_suffix(':') else {
        return Err(CompilerError::new(
            line.span(1),
            "weave declarations must end with a colon",
        ));
    };
    let (rest, task) = if let Some(rest) = without_colon.strip_prefix("task weave ") {
        (rest, true)
    } else if let Some(rest) = without_colon.strip_prefix("export weave ") {
        (rest, false)
    } else if let Some(rest) = without_colon.strip_prefix("weave ") {
        (rest, false)
    } else {
        return Err(CompilerError::new(
            line.span(1),
            "expected weave, task weave, or export weave declaration",
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
    let parameters = parse_parameters(&after_opening[..closing], line, record_types, task)?;
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
    Ok((name.to_owned(), parameters, result, effect, task))
}

fn parse_parameters(
    source: &str,
    line: SourceLine<'_>,
    record_types: &BTreeMap<String, u16>,
    task: bool,
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
        if task
            && (mode != ParameterMode::Own
                || !matches!(value_type, ValueType::Whole | ValueType::Truth))
        {
            return Err(CompilerError::new(
                line.span(1),
                "AE-TASK-004: task weave parameters must be owned Whole or Truth copy values",
            ));
        }
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

fn parse_shape_declaration(
    lines: &[SourceLine<'_>],
    index: &mut usize,
) -> Result<ShapeDeclaration, CompilerError> {
    let line = lines[*index];
    let Some(rest) = line.content.strip_prefix("shape ") else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-LAYOUT-001: expected shape declaration",
        ));
    };
    let Some(name) = rest.strip_suffix(':') else {
        return Err(CompilerError::new(
            line.span(1),
            "AE-LAYOUT-001: shape declarations use `shape name:` followed by Whole field lines",
        ));
    };
    let name = name.trim_end();
    validate_name(name, line.span(7), "shape name", false)?;
    *index += 1;
    let mut fields = Vec::new();
    while *index < lines.len() {
        let field_line = lines[*index];
        if field_line.indentation == 0 {
            break;
        }
        if field_line.indentation != 1 {
            return Err(CompilerError::new(
                field_line.span(1),
                "AE-LAYOUT-001: shape fields must be indented one level under the shape header",
            ));
        }
        let tokens = tokenize_fragment(field_line.content, field_line.span(1))?;
        if tokens.len() != 2 {
            return Err(CompilerError::new(
                field_line.span(1),
                "AE-LAYOUT-001: each shape field must be `name Whole`",
            ));
        }
        validate_name(&tokens[0], field_line.span(1), "shape field name", false)?;
        if tokens[1] != "Whole" {
            return Err(CompilerError::new(
                field_line.span(1),
                "AE-LAYOUT-001: shape fields may use only Whole",
            ));
        }
        if fields
            .iter()
            .any(|field: &ShapeField| field.name == tokens[0])
        {
            return Err(CompilerError::new(
                field_line.span(1),
                format!(
                    "AE-LAYOUT-001: shape field {} is declared more than once",
                    tokens[0]
                ),
            ));
        }
        if fields.len() >= MAX_SHAPE_FIELDS {
            return Err(CompilerError::new(
                field_line.span(1),
                format!("AE-LAYOUT-001: shapes support at most {MAX_SHAPE_FIELDS} Whole fields"),
            ));
        }
        fields.push(ShapeField {
            name: tokens[0].clone(),
            span: field_line.span(1),
        });
        *index += 1;
    }
    if fields.is_empty() {
        return Err(CompilerError::new(
            line.span(1),
            "AE-LAYOUT-001: shapes require at least one Whole field",
        ));
    }
    Ok(ShapeDeclaration {
        name: name.to_owned(),
        fields,
        span: line.span(1),
    })
}

fn shape_type_map(shapes: &[ShapeDeclaration]) -> Result<BTreeMap<String, u16>, CompilerError> {
    if shapes.len() > MAX_SHAPES {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!("AE-LAYOUT-001: Aether supports at most {MAX_SHAPES} shapes per artifact"),
        ));
    }
    let mut shape_types = BTreeMap::new();
    for (index, shape) in shapes.iter().enumerate() {
        let identifier = u16::try_from(index).map_err(|_| {
            CompilerError::new(shape.span, "shape identifier is outside the AETH range")
        })?;
        if shape_types.insert(shape.name.clone(), identifier).is_some() {
            return Err(CompilerError::new(
                shape.span,
                format!(
                    "AE-LAYOUT-001: shape {} is declared more than once",
                    shape.name
                ),
            ));
        }
    }
    Ok(shape_types)
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

        if line.content == "together:" {
            let span = line.span(1);
            *index += 1;
            let mut spawns = Vec::new();
            while *index < lines.len() {
                let spawn_line = lines[*index];
                if spawn_line.indentation < indentation + 1 {
                    break;
                }
                if spawn_line.indentation > indentation + 1 {
                    return Err(CompilerError::new(
                        spawn_line.span(1),
                        "AE-TASK-001: nursery bodies admit only spawn call lines at one indent level",
                    ));
                }
                if spawn_line.content == "together:" {
                    return Err(CompilerError::new(
                        spawn_line.span(1),
                        "AE-TASK-001: nested together blocks are not admitted",
                    ));
                }
                if spawn_line.content.ends_with(':') {
                    return Err(CompilerError::new(
                        spawn_line.span(1),
                        "AE-TASK-001: nursery bodies admit only spawn call lines",
                    ));
                }
                spawns.push(parse_spawn_statement(spawn_line)?);
                *index += 1;
            }
            if spawns.is_empty() || spawns.len() > MAX_NURSERY_SPAWNS {
                return Err(CompilerError::new(
                    span,
                    format!(
                        "AE-TASK-001: together requires 1 through {MAX_NURSERY_SPAWNS} spawn call lines"
                    ),
                ));
            }
            statements.push(Statement::Together { spawns, span });
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

fn parse_spawn_statement(line: SourceLine<'_>) -> Result<Spawn, CompilerError> {
    let span = line.span(line.indentation * 2 + 1);
    let Some(call_source) = line.content.strip_prefix("spawn call ") else {
        return Err(CompilerError::new(
            span,
            "AE-TASK-001: nursery bodies admit only spawn call lines",
        ));
    };
    let Some((call_body, destination)) = call_source.rsplit_once(" into ") else {
        return Err(CompilerError::new(
            span,
            "AE-TASK-002: spawn call requires arguments and an into destination",
        ));
    };
    validate_name(destination, span, "spawn destination", false)?;
    let expression = parse_expression(&format!("call {call_body}"), span)?;
    let ExpressionKind::Call { weave, arguments } = expression.kind else {
        return Err(CompilerError::new(
            span,
            "AE-TASK-002: spawn requires call form spawn call <weave> <args...> into <name>",
        ));
    };
    Ok(Spawn {
        weave,
        arguments,
        destination: destination.to_owned(),
        span,
    })
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
    if let Some(name) = line.content.strip_prefix("release ") {
        let name = name.trim();
        validate_name(name, span, "release target", false)?;
        return Ok(Statement::Release {
            name: name.to_owned(),
            span,
        });
    }
    if line.content == "checkpoint" {
        return Ok(Statement::Checkpoint { span });
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
        "unknown Aether statement; use comptime bind, bind, revise, speak, release, checkpoint, yield, raise, forward, choose, while, together, or handle",
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
        "table" if tokens.len() == 4 => {
            let Some(shape) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-001: table requires a shape name",
                ));
            };
            validate_name(shape, span, "shape name", false)?;
            index += 1;
            let Some(layout_keyword) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-001: table requires the layout keyword",
                ));
            };
            if layout_keyword != "layout" {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-001: table requires `layout rows` or `layout columns`",
                ));
            }
            index += 1;
            let Some(layout) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-001: table requires rows or columns after layout",
                ));
            };
            let layout = match layout.as_str() {
                "rows" => TableLayout::Rows,
                "columns" => TableLayout::Columns,
                _ => {
                    return Err(CompilerError::new(
                        span,
                        "AE-LAYOUT-001: table layout must be rows or columns",
                    ))
                }
            };
            index += 1;
            ExpressionKind::Table {
                shape: shape.clone(),
                layout,
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
        "store" if tokens.iter().any(|token| token == "into") => {
            let table = parse_atom_from(&tokens, &mut index, span)?;
            let index_value = parse_atom_from(&tokens, &mut index, span)?;
            let Some(field) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-002: store requires a shape field name",
                ));
            };
            validate_name(field, span, "shape field name", false)?;
            index += 1;
            let value = parse_atom_from(&tokens, &mut index, span)?;
            let destination = parse_resource_destination(&tokens, &mut index, span)?;
            ExpressionKind::Resource(ResourceOperation::Store {
                table,
                index: index_value,
                field: field.clone(),
                value,
                destination,
            })
        }
        "load" if tokens.iter().any(|token| token == "into") => {
            let table = parse_atom_from(&tokens, &mut index, span)?;
            let index_value = parse_atom_from(&tokens, &mut index, span)?;
            let Some(field) = tokens.get(index) else {
                return Err(CompilerError::new(
                    span,
                    "AE-LAYOUT-002: load requires a shape field name",
                ));
            };
            validate_name(field, span, "shape field name", false)?;
            index += 1;
            let destination = parse_resource_destination(&tokens, &mut index, span)?;
            ExpressionKind::Resource(ResourceOperation::Load {
                table,
                index: index_value,
                field: field.clone(),
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
            validate_call_target(weave, span)?;
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
    validate_shape_declarations(&program.shapes, &program.records)?;
    let mut resource_plan = validate_resource_plan(program)?;
    validate_comptime_budget(program)?;
    if program.weaves.is_empty() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "an Aether program must declare at least one weave",
        ));
    }
    let total_functions = program
        .host_weaves
        .len()
        .saturating_add(program.weaves.len());
    if total_functions > MAX_FUNCTIONS {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!("Aether supports at most {MAX_FUNCTIONS} weaves per artifact"),
        ));
    }

    let mut signatures = BTreeMap::new();
    for host in &program.host_weaves {
        if host.foreign.is_some() {
            validate_foreign_abi_types(&host.parameters, host.result, host.span)?;
        } else {
            validate_host_abi_types(&host.parameters, host.result, host.span)?;
        }
        if host.name == "main" {
            return Err(CompilerError::new(
                host.span,
                "AE-HOST-001: main cannot be a host weave",
            ));
        }
        if signatures
            .insert(
                host.name.clone(),
                FunctionSignature {
                    parameters: host.parameters.clone(),
                    result: host.result,
                    effect: Effect::Total,
                    is_host: true,
                    is_task: false,
                },
            )
            .is_some()
        {
            return Err(CompilerError::new(
                host.span,
                format!(
                    "AE-HOST-001: host weave {} is declared more than once",
                    host.name
                ),
            ));
        }
    }
    for weave in &program.weaves {
        if signatures
            .insert(
                weave.name.clone(),
                FunctionSignature {
                    parameters: weave.parameters.clone(),
                    result: weave.result,
                    effect: weave.effect,
                    is_host: false,
                    is_task: weave.task,
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
        if weave.task {
            validate_task_weave_declaration(weave)?;
        }
    }

    // M19b: spawn callees must not use M2/M6 resource forms (Policy A).
    let resource_using_weaves: BTreeSet<String> = program
        .weaves
        .iter()
        .filter(|weave| weave_uses_resource(weave))
        .map(|weave| weave.name.clone())
        .collect();

    for (weave_index, weave) in program.weaves.iter().enumerate() {
        validate_value_type(weave.result, &program.records, weave.span)?;
        validate_resource_signature(weave)?;
        validate_effect_signature(weave)?;
        // M19a: abortive raise/forward may appear after explicit `release` of
        // owners; site-level clean boundary enforces liveness. Erroring weaves
        // still cannot *declare* arena/buffer/table resource forms (main-owned
        // arena plan + M2). Terminal handle may coexist with live resources (M16).
        // M19b: parent resource ownership may coexist with nurseries when spawn
        // callees are resource-free (Policy A); no weave-level nursery ban.
        if weave.effect == Effect::ErrorWhole && weave_uses_resource(weave) {
            return Err(CompilerError::new(
                weave.span,
                "AE-EFFECT-003: a weave that raises Whole cannot declare arena, Buffer, access, table, or resource outcomes",
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
            if is_table_type(parameter.value_type) {
                return Err(CompilerError::new(
                    parameter.span,
                    "M6 tables cannot cross weave parameter boundaries",
                ));
            }
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
        let mut comptime_env = BTreeMap::new();
        validate_block(
            &weave.body,
            &mut scope,
            &signatures,
            &program.records,
            &program.shapes,
            weave,
            true,
            &mut resource_plan,
            &mut comptime_env,
            &resource_using_weaves,
            &program.weaves[..weave_index],
        )?;
    }
    if program.weaves.iter().any(|weave| weave.task) {
        validate_m19e_nursery_contracts(program, &signatures)?;
        apply_m19e_frame_capacity(program, &mut resource_plan)?;
    }
    Ok(resource_plan)
}

/// Enforce the deliberately small self-contained task subset before ordinary
/// source validation resolves its local types and ownership states. The second
/// pass remains necessary because it proves each concrete local operation.
fn validate_task_weave_declaration(weave: &Weave) -> Result<(), CompilerError> {
    if weave.name == "main" || weave.effect != Effect::Total || weave.result != ValueType::Whole {
        return Err(CompilerError::new(
            weave.span,
            "AE-TASK-004: task weave must be a non-main total guest weave returning Whole",
        ));
    }
    for parameter in &weave.parameters {
        if parameter.mode != ParameterMode::Own
            || !matches!(parameter.value_type, ValueType::Whole | ValueType::Truth)
        {
            return Err(CompilerError::new(
                parameter.span,
                "AE-TASK-004: task weave parameters must be owned Whole or Truth copy values",
            ));
        }
    }

    fn reject_task_expression(expression: &Expression) -> Result<(), CompilerError> {
        if matches!(expression.kind, ExpressionKind::Call { .. }) {
            return Err(CompilerError::new(
                expression.span,
                "AE-TASK-004: task weave cannot use call, host, or foreign invocation",
            ));
        }
        Ok(())
    }

    fn walk(statements: &[Statement], checkpoint_count: &mut usize) -> Result<(), CompilerError> {
        for statement in statements {
            match statement {
                Statement::Bind {
                    comptime,
                    value,
                    span,
                    ..
                } => {
                    if *comptime {
                        return Err(CompilerError::new(
                            *span,
                            "AE-TASK-004: task weave cannot contain comptime bind",
                        ));
                    }
                    reject_task_expression(value)?;
                }
                Statement::Revise { value, .. } | Statement::Yield { value, .. } => {
                    reject_task_expression(value)?
                }
                Statement::Checkpoint { .. } => {
                    *checkpoint_count = checkpoint_count.saturating_add(1);
                }
                Statement::Speak { span, .. } => {
                    return Err(CompilerError::new(
                        *span,
                        "AE-TASK-004: task weave cannot speak or perform ambient output",
                    ));
                }
                Statement::Raise { span, .. }
                | Statement::Forward { span, .. }
                | Statement::Handle { span, .. } => {
                    return Err(CompilerError::new(
                        *span,
                        "AE-TASK-004: task weave cannot raise, forward, or handle effects",
                    ));
                }
                Statement::Together { span, .. } => {
                    return Err(CompilerError::new(
                        *span,
                        "AE-TASK-004: task weave cannot open a nested together nursery",
                    ));
                }
                Statement::Release { .. } => {}
                Statement::Choose {
                    condition,
                    when_bright,
                    when_dim,
                    ..
                } => {
                    reject_task_expression(condition)?;
                    walk(when_bright, checkpoint_count)?;
                    walk(when_dim, checkpoint_count)?;
                }
                Statement::While {
                    condition,
                    body,
                    span,
                    ..
                } => {
                    reject_task_expression(condition)?;
                    if !matches!(body.first(), Some(Statement::Checkpoint { .. })) {
                        return Err(CompilerError::new(
                            *span,
                            "AE-TASK-004: every task while body must begin with a direct checkpoint",
                        ));
                    }
                    walk(body, checkpoint_count)?;
                }
            }
        }
        Ok(())
    }

    let mut checkpoints = 0_usize;
    walk(&weave.body, &mut checkpoints)?;
    if checkpoints == 0 {
        return Err(CompilerError::new(
            weave.span,
            "AE-TASK-004: task weave requires at least one checkpoint",
        ));
    }
    Ok(())
}

fn validate_m19e_nursery_contracts(
    program: &Program,
    signatures: &BTreeMap<String, FunctionSignature>,
) -> Result<(), CompilerError> {
    for weave in &program.weaves {
        if !weave.task && weave.name != "main" && weave_uses_resource(weave) {
            return Err(CompilerError::new(
                weave.span,
                "AE-RESOURCE-004: a program using task weave permits direct arena/resource ownership only in main or a task weave",
            ));
        }
    }

    fn companion_is_eligible(weave: &Weave) -> bool {
        if weave.task
            || weave.result != ValueType::Whole
            || weave.parameters.iter().any(|parameter| {
                parameter.mode != ParameterMode::Own
                    || !matches!(parameter.value_type, ValueType::Whole | ValueType::Truth)
            })
            || weave_uses_resource(weave)
        {
            return false;
        }
        fn copy_atom(atom: &Atom) -> bool {
            matches!(
                atom.kind,
                AtomKind::Whole(_) | AtomKind::Truth(_) | AtomKind::Name(_)
            )
        }
        fn copy_expression(expression: &Expression) -> bool {
            match &expression.kind {
                ExpressionKind::Atom(atom) => copy_atom(atom),
                ExpressionKind::Unary {
                    operation,
                    argument,
                } => matches!(operation, UnaryOperation::Not) && copy_atom(argument),
                ExpressionKind::Binary { left, right, .. } => copy_atom(left) && copy_atom(right),
                _ => false,
            }
        }
        fn block(statements: &[Statement]) -> bool {
            statements.iter().all(|statement| match statement {
                Statement::Bind {
                    comptime, value, ..
                } => !*comptime && copy_expression(value),
                Statement::Revise { value, .. } | Statement::Yield { value, .. } => {
                    copy_expression(value)
                }
                Statement::Raise { code, .. } => copy_atom(code),
                Statement::Choose {
                    condition,
                    when_bright,
                    when_dim,
                    ..
                } => copy_expression(condition) && block(when_bright) && block(when_dim),
                Statement::While {
                    condition, body, ..
                } => copy_expression(condition) && block(body),
                Statement::Speak { .. }
                | Statement::Release { .. }
                | Statement::Checkpoint { .. }
                | Statement::Forward { .. }
                | Statement::Handle { .. }
                | Statement::Together { .. } => false,
            })
        }
        block(&weave.body)
    }

    fn visit(
        parent: &Weave,
        statements: &[Statement],
        program: &Program,
        signatures: &BTreeMap<String, FunctionSignature>,
    ) -> Result<(), CompilerError> {
        for statement in statements {
            match statement {
                Statement::Together { spawns, span } => {
                    let checkpointed = spawns.iter().any(|spawn| {
                        signatures
                            .get(&spawn.weave)
                            .is_some_and(|signature| signature.is_task)
                    });
                    if checkpointed {
                        for spawn in spawns {
                            let Some(signature) = signatures.get(&spawn.weave) else {
                                return Err(CompilerError::new(
                                    spawn.span,
                                    format!("AE-TASK-005: spawn target {} is unknown", spawn.weave),
                                ));
                            };
                            if signature.is_task {
                                continue;
                            }
                            let Some(companion) = program
                                .weaves
                                .iter()
                                .find(|candidate| candidate.name == spawn.weave)
                            else {
                                return Err(CompilerError::new(
                                    spawn.span,
                                    format!(
                                        "AE-TASK-005: checkpointed nursery target {} must be a guest task or companion weave",
                                        spawn.weave
                                    ),
                                ));
                            };
                            if !companion_is_eligible(companion) {
                                return Err(CompilerError::new(
                                    spawn.span,
                                    format!(
                                        "AE-TASK-005: checkpointed nursery companion {} must be resource-free, self-contained, and Copy-only",
                                        spawn.weave
                                    ),
                                ));
                            }
                        }
                        if parent.task {
                            return Err(CompilerError::new(
                                *span,
                                "AE-TASK-004: task weave cannot open a checkpointed nursery",
                            ));
                        }
                    }
                }
                Statement::Choose {
                    when_bright,
                    when_dim,
                    ..
                } => {
                    visit(parent, when_bright, program, signatures)?;
                    visit(parent, when_dim, program, signatures)?;
                }
                Statement::While { body, .. } => visit(parent, body, program, signatures)?,
                Statement::Bind { .. }
                | Statement::Revise { .. }
                | Statement::Speak { .. }
                | Statement::Release { .. }
                | Statement::Checkpoint { .. }
                | Statement::Yield { .. }
                | Statement::Raise { .. }
                | Statement::Forward { .. }
                | Statement::Handle { .. } => {}
            }
        }
        Ok(())
    }

    for weave in &program.weaves {
        visit(weave, &weave.body, program, signatures)?;
    }
    Ok(())
}

fn apply_m19e_frame_capacity(
    program: &Program,
    resource_plan: &mut SemanticResourcePlan,
) -> Result<(), CompilerError> {
    fn visit(
        statements: &[Statement],
        program: &Program,
        resource_plan: &SemanticResourcePlan,
        maximum: &mut u32,
    ) -> Result<(), CompilerError> {
        for statement in statements {
            match statement {
                Statement::Together { spawns, span } => {
                    let mut has_task = false;
                    let mut total = 0_u32;
                    for spawn in spawns {
                        let task = program
                            .weaves
                            .iter()
                            .find(|weave| weave.name == spawn.weave)
                            .is_some_and(|weave| weave.task);
                        if task {
                            has_task = true;
                            total = total
                                .checked_add(resource_plan.direct_arena_capacity(&spawn.weave))
                                .ok_or_else(|| {
                                    CompilerError::new(
                                        *span,
                                        "AE-RESOURCE-004: checkpointed nursery frame capacity overflowed the M2 bound",
                                    )
                                })?;
                        }
                    }
                    if has_task {
                        *maximum = (*maximum).max(total);
                    }
                }
                Statement::Choose {
                    when_bright,
                    when_dim,
                    ..
                } => {
                    visit(when_bright, program, resource_plan, maximum)?;
                    visit(when_dim, program, resource_plan, maximum)?;
                }
                Statement::While { body, .. } => visit(body, program, resource_plan, maximum)?,
                Statement::Bind { .. }
                | Statement::Revise { .. }
                | Statement::Speak { .. }
                | Statement::Release { .. }
                | Statement::Checkpoint { .. }
                | Statement::Yield { .. }
                | Statement::Raise { .. }
                | Statement::Forward { .. }
                | Statement::Handle { .. } => {}
            }
        }
        Ok(())
    }

    let main_capacity = resource_plan.direct_arena_capacity("main");
    let mut nursery_capacity = 0_u32;
    for weave in &program.weaves {
        visit(&weave.body, program, resource_plan, &mut nursery_capacity)?;
    }
    let capacity = main_capacity.checked_add(nursery_capacity).ok_or_else(|| {
        CompilerError::new(
            Span::synthetic(),
            "AE-RESOURCE-004: M19e frame capacity overflowed the M2 bound",
        )
    })?;
    if capacity > MAX_ARENA_BYTES {
        return Err(CompilerError::new(
            Span::synthetic(),
            format!(
                "AE-RESOURCE-004: M19e frame capacity exceeds the M2 safety limit of {MAX_ARENA_BYTES}"
            ),
        ));
    }
    if capacity == 0 {
        resource_plan.arena = None;
    } else {
        resource_plan.arena = Some(SemanticArena {
            name: "m19e_frame_plan".to_owned(),
            capacity,
            span: Span::synthetic(),
        });
    }
    Ok(())
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
                Statement::Together { .. }
                | Statement::Checkpoint { .. }
                | Statement::Revise { .. }
                | Statement::Speak { .. }
                | Statement::Release { .. }
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

fn comptime_operand_whole(atom: &Atom, env: &BTreeMap<String, i64>) -> Result<i64, CompilerError> {
    match &atom.kind {
        AtomKind::Whole(value) => Ok(*value),
        AtomKind::Name(name) => env.get(name).copied().ok_or_else(|| {
            CompilerError::new(
                atom.span,
                format!(
                    "AE-COMPTIME-001: comptime operand {name} is not a prior root-level comptime Whole binding"
                ),
            )
        }),
        AtomKind::Text(_)
        | AtomKind::Bytes(_)
        | AtomKind::Truth(_)
        | AtomKind::Borrow(_)
        | AtomKind::Move(_)
        | AtomKind::Access(_) => Err(CompilerError::new(
            atom.span,
            "AE-COMPTIME-001: comptime bind accepts only Whole literals or prior comptime Whole names",
        )),
    }
}

/// M5/M15 arithmetic plus the M23 `call` form. Every accepted directive still
/// folds to one `COMPTIME_WHOLE` immediate; M23 adds no runtime call edge.
fn evaluate_comptime_whole(
    expression: &Expression,
    env: &BTreeMap<String, i64>,
    prior_weaves: &[Weave],
) -> Result<i64, CompilerError> {
    match &expression.kind {
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => evaluate_comptime_arithmetic(
            *operation,
            comptime_operand_whole(left, env)?,
            comptime_operand_whole(right, env)?,
            expression.span,
        ),
        ExpressionKind::Call { weave, arguments } => {
            evaluate_comptime_pure_call(expression.span, weave, arguments, env, prior_weaves)
        }
        _ => Err(CompilerError::new(
            expression.span,
            "AE-COMPTIME-001: comptime bind requires exactly one Whole sum, difference, product, quotient, remainder, or eligible pure call expression",
        )),
    }
}

/// Checked Whole arithmetic shared by the direct M5/M15 evaluator and the
/// restricted M23 callee interpreter. Keeping one implementation prevents
/// arithmetic drift between a literal directive and the same operation inside
/// a pure comptime helper.
fn evaluate_comptime_arithmetic(
    operation: BinaryOperation,
    left: i64,
    right: i64,
    span: Span,
) -> Result<i64, CompilerError> {
    match operation {
        BinaryOperation::Sum => left.checked_add(right).ok_or_else(|| {
            CompilerError::new(span, "AE-COMPTIME-002: comptime sum overflowed Whole")
        }),
        BinaryOperation::Difference => left.checked_sub(right).ok_or_else(|| {
            CompilerError::new(
                span,
                "AE-COMPTIME-002: comptime difference overflowed Whole",
            )
        }),
        BinaryOperation::Product => left.checked_mul(right).ok_or_else(|| {
            CompilerError::new(span, "AE-COMPTIME-002: comptime product overflowed Whole")
        }),
        BinaryOperation::Quotient => {
            if right == 0 {
                return Err(CompilerError::new(
                    span,
                    "AE-COMPTIME-002: comptime quotient cannot divide by zero",
                ));
            }
            left.checked_div(right).ok_or_else(|| {
                CompilerError::new(span, "AE-COMPTIME-002: comptime quotient overflowed Whole")
            })
        }
        BinaryOperation::Remainder => {
            if right == 0 {
                return Err(CompilerError::new(
                    span,
                    "AE-COMPTIME-002: comptime remainder cannot divide by zero",
                ));
            }
            left.checked_rem(right).ok_or_else(|| {
                CompilerError::new(span, "AE-COMPTIME-002: comptime remainder overflowed Whole")
            })
        }
        _ => Err(CompilerError::new(
            span,
            format!(
                "AE-COMPTIME-001: comptime bind does not admit {} in the bounded comptime evaluator",
                operation.word()
            ),
        )),
    }
}

#[derive(Clone, Copy)]
struct ComptimeLocal {
    value: i64,
    mutable: bool,
}

/// M23 resolver: lookup is intentionally limited to earlier guest weaves. A
/// host/foreign target, a forward target, and an unknown target all fail before
/// an evaluator can observe any runtime authority.
fn evaluate_comptime_pure_call(
    span: Span,
    weave_name: &str,
    arguments: &[Atom],
    caller_env: &BTreeMap<String, i64>,
    prior_weaves: &[Weave],
) -> Result<i64, CompilerError> {
    let callee = prior_weaves
        .iter()
        .find(|candidate| candidate.name == weave_name)
        .ok_or_else(|| {
            CompilerError::new(
                span,
                format!(
                    "AE-COMPTIME-001: comptime call {weave_name} must target a prior total guest weave"
                ),
            )
        })?;

    if callee.task || callee.effect != Effect::Total || callee.result != ValueType::Whole {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-COMPTIME-001: comptime call {weave_name} requires a total guest weave returning Whole"
            ),
        ));
    }
    if callee.parameters.iter().any(|parameter| {
        parameter.mode != ParameterMode::Own || parameter.value_type != ValueType::Whole
    }) {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-COMPTIME-001: comptime call {weave_name} requires owned Whole parameters only"
            ),
        ));
    }
    if arguments.len() != callee.parameters.len() {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-COMPTIME-001: comptime call {weave_name} expects {} Whole arguments, received {}",
                callee.parameters.len(),
                arguments.len()
            ),
        ));
    }

    let mut locals = BTreeMap::new();
    for (parameter, argument) in callee.parameters.iter().zip(arguments) {
        locals.insert(
            parameter.name.clone(),
            ComptimeLocal {
                value: comptime_operand_whole(argument, caller_env)?,
                mutable: false,
            },
        );
    }
    evaluate_comptime_pure_weave(callee, locals)
}

/// Interpret only the M23 callee subset. This deliberately does not reuse the
/// VM: the evaluator has no bytecode, host, arena, resource, effect, nursery,
/// control-flow, or nested-call capability to invoke.
fn evaluate_comptime_pure_weave(
    weave: &Weave,
    mut locals: BTreeMap<String, ComptimeLocal>,
) -> Result<i64, CompilerError> {
    let mut result = None;

    for (index, statement) in weave.body.iter().enumerate() {
        match statement {
            Statement::Bind {
                name,
                mutable,
                comptime,
                value,
                span,
            } => {
                if *comptime {
                    return Err(CompilerError::new(
                        *span,
                        "AE-COMPTIME-001: an M23 comptime-pure callee cannot contain comptime bind",
                    ));
                }
                if locals.contains_key(name) {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-COMPTIME-001: comptime-pure callee {} rebinds local {name}",
                            weave.name
                        ),
                    ));
                }
                let value = evaluate_comptime_callee_expression(value, &locals)?;
                locals.insert(
                    name.clone(),
                    ComptimeLocal {
                        value,
                        mutable: *mutable,
                    },
                );
            }
            Statement::Revise { name, value, span } => {
                let evaluated = evaluate_comptime_callee_expression(value, &locals)?;
                let Some(local) = locals.get_mut(name) else {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-COMPTIME-001: comptime-pure callee {} revises unknown local {name}",
                            weave.name
                        ),
                    ));
                };
                if !local.mutable {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-COMPTIME-001: comptime-pure callee {} revises immutable local {name}",
                            weave.name
                        ),
                    ));
                }
                local.value = evaluated;
            }
            Statement::Yield { value, span } => {
                if index + 1 != weave.body.len() {
                    return Err(CompilerError::new(
                        *span,
                        "AE-COMPTIME-001: an M23 comptime-pure callee must end with its Whole yield",
                    ));
                }
                result = Some(evaluate_comptime_callee_expression(value, &locals)?);
            }
            _ => {
                return Err(CompilerError::new(
                    statement.span(),
                    format!(
                        "AE-COMPTIME-001: comptime-pure callee {} permits only Whole bind, revise, and terminal yield statements",
                        weave.name
                    ),
                ));
            }
        }
    }

    result.ok_or_else(|| {
        CompilerError::new(
            weave.span,
            format!(
                "AE-COMPTIME-001: comptime-pure callee {} requires a terminal Whole yield",
                weave.name
            ),
        )
    })
}

fn evaluate_comptime_callee_expression(
    expression: &Expression,
    locals: &BTreeMap<String, ComptimeLocal>,
) -> Result<i64, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => comptime_callee_operand_whole(atom, locals),
        ExpressionKind::Binary {
            operation,
            left,
            right,
        } => evaluate_comptime_arithmetic(
            *operation,
            comptime_callee_operand_whole(left, locals)?,
            comptime_callee_operand_whole(right, locals)?,
            expression.span,
        ),
        ExpressionKind::Call { .. } => Err(CompilerError::new(
            expression.span,
            "AE-COMPTIME-001: an M23 comptime-pure callee cannot contain a nested call",
        )),
        _ => Err(CompilerError::new(
            expression.span,
            "AE-COMPTIME-001: an M23 comptime-pure callee expression must be a Whole atom or one Whole arithmetic operation",
        )),
    }
}

fn comptime_callee_operand_whole(
    atom: &Atom,
    locals: &BTreeMap<String, ComptimeLocal>,
) -> Result<i64, CompilerError> {
    match &atom.kind {
        AtomKind::Whole(value) => Ok(*value),
        AtomKind::Name(name) => locals.get(name).map(|local| local.value).ok_or_else(|| {
            CompilerError::new(
                atom.span,
                format!(
                    "AE-COMPTIME-001: comptime-pure callee reads unknown Whole local {name}"
                ),
            )
        }),
        AtomKind::Text(_)
        | AtomKind::Bytes(_)
        | AtomKind::Truth(_)
        | AtomKind::Borrow(_)
        | AtomKind::Move(_)
        | AtomKind::Access(_) => Err(CompilerError::new(
            atom.span,
            "AE-COMPTIME-001: an M23 comptime-pure callee accepts only Whole literals or local names",
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
        if is_table_type(parameter.value_type) {
            return Err(CompilerError::new(
                parameter.span,
                "M6 tables cannot cross weave parameter boundaries",
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
    if is_table_type(weave.result) {
        return Err(CompilerError::new(
            weave.span,
            "M6 table ownership cannot cross a weave result; handle its closed outcome in the defining weave",
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
    shapes: &[ShapeDeclaration],
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
        ExpressionKind::Table { shape, layout } => {
            let (shape_id, _) = shape_by_name(shapes, shape, expression.span)?;
            Ok(ResourceBindingState::Table {
                shape: shape_id,
                layout: *layout,
                arena: None,
                allocated: false,
            })
        }
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
    // M19d: at most one arena per weave; multiple weaves may each declare one.
    let mut seen_weaves = BTreeSet::new();
    for (weave_name, arena) in &arenas {
        if !seen_weaves.insert(weave_name.clone()) {
            return Err(CompilerError::new(
                arena.span,
                "AE-RESOURCE-001: Aether permits at most one arena declaration per weave",
            ));
        }
        if arena.name.is_empty() {
            return Err(CompilerError::new(
                arena.span,
                "the M2 arena declaration requires a named root binding",
            ));
        }
    }
    let mut plan = SemanticResourcePlan::empty();
    for (weave_name, arena) in &arenas {
        plan.arenas_by_weave
            .insert(weave_name.clone(), arena.clone());
    }
    if let Some((_, first)) = arenas.first() {
        let task_program = program.weaves.iter().any(|weave| weave.task);
        let mut total = 0_u32;
        for (_, arena) in &arenas {
            total = total.checked_add(arena.capacity).ok_or_else(|| {
                CompilerError::new(
                    arena.span,
                    format!(
                        "AE-RESOURCE-001: combined arena capacity exceeds the M2 safety limit of {MAX_ARENA_BYTES}"
                    ),
                )
            })?;
            if !task_program && total > MAX_ARENA_BYTES {
                return Err(CompilerError::new(
                    arena.span,
                    format!(
                        "AE-RESOURCE-001: combined arena capacity exceeds the M2 safety limit of {MAX_ARENA_BYTES}"
                    ),
                ));
            }
        }
        // v12 computes an exact concurrent frame plan after full task/nursery
        // validation.  Do not reject an unreachable aggregate of independent
        // task lanes here; the later plan enforces the actual 1,000,000-byte
        // concurrent bound.  v4-v11 retain their historical aggregate plan.
        if !task_program {
            plan.arena = Some(SemanticArena {
                name: first.name.clone(),
                capacity: total,
                span: first.span,
            });
        }
    }
    if resource_operations && arenas.is_empty() {
        return Err(CompilerError::new(
            Span::synthetic(),
            "M2 resource operations require at least one named arena declaration in a total weave",
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
            Statement::Raise { .. }
            | Statement::Forward { .. }
            | Statement::Release { .. }
            | Statement::Checkpoint { .. } => {}
            Statement::Handle { .. } | Statement::Together { .. } => {}
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
            | ExpressionKind::Table { .. }
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
            Statement::Raise { .. }
            | Statement::Forward { .. }
            | Statement::Release { .. }
            | Statement::Checkpoint { .. } => false,
            Statement::Handle { .. } | Statement::Together { .. } => false,
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

fn nursery_may_raise(spawns: &[Spawn], signatures: &BTreeMap<String, FunctionSignature>) -> bool {
    spawns.iter().any(|spawn| {
        signatures
            .get(&spawn.weave)
            .is_some_and(|signature| signature.effect == Effect::ErrorWhole)
    })
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

#[allow(clippy::too_many_arguments)]
fn validate_block(
    statements: &[Statement],
    scope: &mut BTreeMap<String, BindingState>,
    signatures: &BTreeMap<String, FunctionSignature>,
    records: &[RecordDeclaration],
    shapes: &[ShapeDeclaration],
    weave: &Weave,
    root: bool,
    resource_plan: &mut SemanticResourcePlan,
    comptime_env: &mut BTreeMap<String, i64>,
    resource_using_weaves: &BTreeSet<String>,
    prior_weaves: &[Weave],
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
                    let folded = evaluate_comptime_whole(value, comptime_env, prior_weaves)?;
                    comptime_env.insert(name.clone(), folded);
                }
                if scope.contains_key(name) {
                    return Err(CompilerError::new(
                        *span,
                        format!("binding {name} already exists in this weave"),
                    ));
                }
                let mut resource = resource_state_for_expression(value, scope, shapes)?;
                if matches!(value.kind, ExpressionKind::Arena { .. }) {
                    // M19d: any total weave may declare a self-owned arena.
                    if weave.effect != Effect::Total {
                        return Err(CompilerError::new(
                            *span,
                            "AE-EFFECT-003: a weave that raises Whole cannot declare an arena",
                        ));
                    }
                    if let ResourceBindingState::Arena { name: arena_name } = &mut resource {
                        *arena_name = name.clone();
                    }
                }
                let value_type = expression_type(value, scope, signatures, records, shapes)?;
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
                if is_resource_owner_type(binding.value_type) {
                    return Err(CompilerError::new(
                        *span,
                        "M2/M6 resource owners are replaced only by their closed resource outcomes, never revise",
                    ));
                }
                if expression_moves_name(value, name) {
                    return Err(CompilerError::new(
                        value.span,
                        "revise cannot move its own target while that replacement is reserved",
                    ));
                }
                let expected = binding.value_type;
                let actual = expression_type(value, scope, signatures, records, shapes)?;
                require_source_type(actual, expected, value.span, "revise")?;
            }
            Statement::Speak { value, .. } => {
                let value_type = expression_type(value, scope, signatures, records, shapes)?;
                require_source_type(value_type, ValueType::Text, value.span, "speak")?;
            }
            Statement::Release { name, span } => {
                if !root {
                    return Err(CompilerError::new(
                        *span,
                        "AE-RESOURCE-001: release is allowed only in a weave root",
                    ));
                }
                let Some(binding) = scope.get(name) else {
                    return Err(CompilerError::new(
                        *span,
                        format!("AE-RESOURCE-001: release target {name} has not been introduced"),
                    ));
                };
                if binding.moved {
                    return Err(CompilerError::new(
                        *span,
                        format!("AE-RESOURCE-001: release target {name} has already been moved or released"),
                    ));
                }
                let releasable = is_unique_value(binding.value_type)
                    || matches!(
                        binding.value_type,
                        ValueType::Arena | ValueType::BufferWhole | ValueType::BufferTruth
                    )
                    || is_table_type(binding.value_type);
                if !releasable || binding.value_type == ValueType::AccessArena {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-RESOURCE-001: release cannot target copy values or access loans ({name})"
                        ),
                    ));
                }
                if binding.value_type == ValueType::Arena {
                    let blocked = scope.iter().any(|(other, state)| {
                        other != name
                            && !state.moved
                            && (is_buffer_type(state.value_type)
                                || is_table_type(state.value_type)
                                || state.value_type == ValueType::AccessArena)
                    });
                    if blocked {
                        return Err(CompilerError::new(
                            *span,
                            format!(
                                "AE-RESOURCE-001: release of arena {name} requires no live Buffer, table, or access loan"
                            ),
                        ));
                    }
                }
                scope.get_mut(name).expect("release target").moved = true;
            }
            Statement::Checkpoint { span } => {
                if !weave.task {
                    return Err(CompilerError::new(
                        *span,
                        "AE-TASK-004: checkpoint is valid only in a task weave",
                    ));
                }
                if scope
                    .values()
                    .any(|binding| !binding.moved && binding.value_type == ValueType::AccessArena)
                {
                    return Err(CompilerError::new(
                        *span,
                        "AE-TASK-004: checkpoint cannot cross a live exclusive access loan",
                    ));
                }
            }
            Statement::Yield { value, span } => {
                if (!root && !weave.task) || index + 1 != statements.len() {
                    return Err(CompilerError::new(
                        *span,
                        if weave.task {
                            "AE-TASK-004: task yield must be the final statement of its current outcome branch"
                        } else {
                            "yield is allowed only as the final statement of a weave root"
                        },
                    ));
                }
                let value_type = expression_type(value, scope, signatures, records, shapes)?;
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
                validate_handle_boundary(scope, *span)?;
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
                    if index + 1 != statements.len() || (!root && !weave.task) {
                        return Err(CompilerError::new(
                            condition.span,
                            if weave.task {
                                "AE-TASK-004: a task resource outcome choose must be the final statement of its current outcome branch"
                            } else {
                                "a resource outcome choose must be the final statement of a weave root"
                            },
                        ));
                    }
                    let context = ResourceValidationContext {
                        signatures,
                        records,
                        shapes,
                        weave,
                    };
                    if weave.task {
                        validate_task_resource_choose(
                            condition,
                            when_bright,
                            when_dim,
                            scope,
                            &context,
                            resource_plan,
                            comptime_env,
                            resource_using_weaves,
                            prior_weaves,
                        )?;
                    } else {
                        validate_resource_choose(
                            condition,
                            when_bright,
                            when_dim,
                            scope,
                            &context,
                            resource_plan,
                        )?;
                    }
                    continue;
                }
                let condition_type =
                    expression_type(condition, scope, signatures, records, shapes)?;
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
                    shapes,
                    weave,
                    false,
                    resource_plan,
                    comptime_env,
                    resource_using_weaves,
                    prior_weaves,
                )?;
                let mut dim_scope = original.clone();
                if !when_dim.is_empty() {
                    validate_block(
                        when_dim,
                        &mut dim_scope,
                        signatures,
                        records,
                        shapes,
                        weave,
                        false,
                        resource_plan,
                        comptime_env,
                        resource_using_weaves,
                        prior_weaves,
                    )?;
                }
                merge_scope(scope, &bright_scope, &dim_scope, statement.span())?;
            }
            Statement::While {
                condition, body, ..
            } => {
                let condition_type =
                    expression_type(condition, scope, signatures, records, shapes)?;
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
                    shapes,
                    weave,
                    false,
                    resource_plan,
                    comptime_env,
                    resource_using_weaves,
                    prior_weaves,
                )?;
                merge_scope(scope, &before_loop, &body_scope, statement.span())?;
            }
            Statement::Together { spawns, span } => {
                if spawns.is_empty() || spawns.len() > MAX_NURSERY_SPAWNS {
                    return Err(CompilerError::new(
                        *span,
                        format!(
                            "AE-TASK-001: together requires 1 through {MAX_NURSERY_SPAWNS} spawn call lines"
                        ),
                    ));
                }
                // M19b Policy A: live resource/unique owners may remain across a
                // nursery; exclusive access loans may not (handle-family boundary).
                validate_nursery_boundary(scope, *span)?;
                let may_raise = nursery_may_raise(spawns, signatures);
                if may_raise {
                    if weave.effect != Effect::ErrorWhole {
                        return Err(CompilerError::new(
                            *span,
                            "AE-TASK-002: a nursery that spawns an Error[Whole] child requires the enclosing weave to raise Whole",
                        ));
                    }
                    if weave.name == "main" {
                        return Err(CompilerError::new(
                            *span,
                            "AE-TASK-003: main must remain total; handle a may-raise nursery before the program entry boundary",
                        ));
                    }
                }
                let mut destinations = BTreeMap::new();
                for spawn in spawns {
                    // M19d Policy A+: total resourceful callees allowed when they
                    // do not take resource parameters (self-owned arena only).
                    if resource_using_weaves.contains(&spawn.weave) {
                        let Some(signature) = signatures.get(&spawn.weave) else {
                            return Err(CompilerError::new(
                                spawn.span,
                                format!("spawn call {} targets an unknown weave", spawn.weave),
                            ));
                        };
                        if signature.effect != Effect::Total {
                            return Err(CompilerError::new(
                                spawn.span,
                                format!(
                                    "AE-TASK-003: nursery spawn callee {} cannot raise Whole while using arena, Buffer, access, table, or resource outcomes",
                                    spawn.weave
                                ),
                            ));
                        }
                        if signature.parameters.iter().any(|parameter| {
                            matches!(
                                parameter.value_type,
                                ValueType::Arena
                                    | ValueType::AccessArena
                                    | ValueType::BufferWhole
                                    | ValueType::BufferTruth
                                    | ValueType::Table(_)
                            ) || parameter.mode == ParameterMode::Access
                        }) {
                            return Err(CompilerError::new(
                                spawn.span,
                                format!(
                                    "AE-TASK-003: nursery spawn callee {} cannot take arena, Buffer, access, or table parameters",
                                    spawn.weave
                                ),
                            ));
                        }
                    }
                    validate_spawn(spawn, scope, signatures, &mut destinations)?;
                }
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

fn validate_spawn(
    spawn: &Spawn,
    scope: &mut BindingScope,
    signatures: &BTreeMap<String, FunctionSignature>,
    destinations: &mut BTreeMap<String, Span>,
) -> Result<(), CompilerError> {
    let Some(signature) = signatures.get(&spawn.weave) else {
        return Err(CompilerError::new(
            spawn.span,
            format!("AE-TASK-002: weave {} has not been declared", spawn.weave),
        ));
    };
    if signature.result != ValueType::Whole {
        return Err(CompilerError::new(
            spawn.span,
            format!(
                "AE-TASK-002: spawn target {} must return Whole",
                spawn.weave
            ),
        ));
    }
    if signature.parameters.len() != spawn.arguments.len() {
        return Err(CompilerError::new(
            spawn.span,
            format!(
                "AE-TASK-002: spawn call {} requires {} argument(s), received {}",
                spawn.weave,
                signature.parameters.len(),
                spawn.arguments.len()
            ),
        ));
    }
    if signature.effect == Effect::ErrorWhole {
        for (argument, parameter) in spawn.arguments.iter().zip(&signature.parameters) {
            if parameter.mode != ParameterMode::Own
                || !matches!(parameter.value_type, ValueType::Whole | ValueType::Truth)
            {
                return Err(CompilerError::new(
                    spawn.span,
                    format!(
                        "AE-TASK-002: spawn of erroring weave {} accepts only ordinary Whole or Truth copy arguments",
                        spawn.weave
                    ),
                ));
            }
            if !matches!(
                argument.kind,
                AtomKind::Whole(_) | AtomKind::Truth(_) | AtomKind::Name(_)
            ) {
                return Err(CompilerError::new(
                    argument.span,
                    "AE-TASK-002: spawn of an erroring weave requires copy literals or copy binding names",
                ));
            }
            require_source_type(
                atom_type(argument, scope)?,
                parameter.value_type,
                argument.span,
                "spawn argument",
            )?;
        }
    } else {
        for (argument, parameter) in spawn.arguments.iter().zip(&signature.parameters) {
            let argument_type = atom_type(argument, scope)?;
            let expected = if parameter.mode == ParameterMode::Access {
                ValueType::AccessArena
            } else {
                parameter.value_type
            };
            require_source_type(argument_type, expected, argument.span, "spawn argument")?;
            if parameter.mode == ParameterMode::Access
                || is_buffer_type(parameter.value_type)
                || parameter.value_type == ValueType::Arena
                || is_table_type(parameter.value_type)
            {
                return Err(CompilerError::new(
                    argument.span,
                    "AE-TASK-003: nursery spawn arguments cannot cross arena, Buffer, access, or table boundaries",
                ));
            }
            if parameter.mode == ParameterMode::Own
                && is_unique_value(parameter.value_type)
                && !matches!(
                    (&argument.kind, parameter.value_type),
                    (AtomKind::Text(_), ValueType::Text) | (AtomKind::Bytes(_), ValueType::Bytes)
                )
                && !matches!(argument.kind, AtomKind::Move(_))
            {
                return Err(CompilerError::new(
                    argument.span,
                    format!(
                        "AE-TASK-002: spawn {} consumes {} parameter {}; use move name or a matching literal",
                        spawn.weave, parameter.value_type, parameter.name
                    ),
                ));
            }
        }
    }
    validate_effect_destination(
        scope,
        &spawn.destination,
        ValueType::Whole,
        spawn.span,
        "spawn",
    )?;
    if destinations
        .insert(spawn.destination.clone(), spawn.span)
        .is_some()
    {
        return Err(CompilerError::new(
            spawn.span,
            format!(
                "AE-TASK-002: spawn destination {} is used more than once in the same nursery",
                spawn.destination
            ),
        ));
    }
    Ok(())
}

/// Full clean boundary for abortive effect control (M4/M19a raise/forward).
fn validate_effect_boundary(scope: &BindingScope, span: Span) -> Result<(), CompilerError> {
    let Some((name, _)) = scope.iter().find(|(_, binding)| {
        !binding.moved
            && (is_unique_value(binding.value_type)
                || binding.value_type == ValueType::Arena
                || binding.value_type == ValueType::AccessArena
                || !matches!(binding.resource, ResourceBindingState::Plain))
    }) else {
        return Ok(());
    };
    Err(CompilerError::new(
        span,
        format!(
            "AE-EFFECT-003: effect control cannot cross live owner, loan, arena, Buffer, or table binding {name}"
        ),
    ))
}

/// M16 handle boundary: live resource owners may remain; exclusive access loans may not.
fn validate_handle_boundary(scope: &BindingScope, span: Span) -> Result<(), CompilerError> {
    let Some((name, _)) = scope
        .iter()
        .find(|(_, binding)| !binding.moved && binding.value_type == ValueType::AccessArena)
    else {
        return Ok(());
    };
    Err(CompilerError::new(
        span,
        format!("AE-EFFECT-003: handle cannot cross live exclusive access loan {name}"),
    ))
}

/// M19b nursery boundary: same as handle — live owners may remain; access loans may not.
fn validate_nursery_boundary(scope: &BindingScope, span: Span) -> Result<(), CompilerError> {
    let Some((name, _)) = scope
        .iter()
        .find(|(_, binding)| !binding.moved && binding.value_type == ValueType::AccessArena)
    else {
        return Ok(());
    };
    Err(CompilerError::new(
        span,
        format!("AE-EFFECT-003: nursery control cannot cross live exclusive access loan {name}"),
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
    if signature.is_task {
        return Err(CompilerError::new(
            span,
            format!(
                "AE-TASK-005: task weave {called} may be invoked only by spawn call inside a checkpointed nursery"
            ),
        ));
    }
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
    let (mut bright_scope, mut dim_scope) = resource_outcome_scopes(
        operation,
        scope,
        condition.span,
        resource_plan,
        context.shapes,
    )?;
    validate_resource_terminal_block(when_bright, &mut bright_scope, context, resource_plan)?;
    validate_resource_terminal_block(when_dim, &mut dim_scope, context, resource_plan)
}

/// M19e keeps the ordinary M2 terminal-outcome rule for v4-v11, but a task
/// may suspend after a successful resource transition before reaching its own
/// terminal yield.  The task-only form remains closed: no nested bindings are
/// introduced, each branch is proven terminal, and every resource transition
/// still receives the same ownership-state calculation as the historic path.
#[allow(clippy::too_many_arguments)]
fn validate_task_resource_choose(
    condition: &Expression,
    when_bright: &[Statement],
    when_dim: &[Statement],
    scope: &BindingScope,
    context: &ResourceValidationContext<'_>,
    resource_plan: &mut SemanticResourcePlan,
    comptime_env: &mut BTreeMap<String, i64>,
    resource_using_weaves: &BTreeSet<String>,
    prior_weaves: &[Weave],
) -> Result<(), CompilerError> {
    if !context.weave.task {
        return Err(CompilerError::new(
            condition.span,
            "internal compiler attempted task resource validation for a non-task weave",
        ));
    }
    if when_dim.is_empty() {
        return Err(CompilerError::new(
            condition.span,
            "resource outcomes require an explicit otherwise branch for the dim outcome",
        ));
    }
    let ExpressionKind::Resource(operation) = &condition.kind else {
        return Err(CompilerError::new(
            condition.span,
            "internal compiler expected a task resource outcome condition",
        ));
    };
    let (mut bright_scope, mut dim_scope) = resource_outcome_scopes(
        operation,
        scope,
        condition.span,
        resource_plan,
        context.shapes,
    )?;
    validate_block(
        when_bright,
        &mut bright_scope,
        context.signatures,
        context.records,
        context.shapes,
        context.weave,
        false,
        resource_plan,
        comptime_env,
        resource_using_weaves,
        prior_weaves,
    )?;
    validate_block(
        when_dim,
        &mut dim_scope,
        context.signatures,
        context.records,
        context.shapes,
        context.weave,
        false,
        resource_plan,
        comptime_env,
        resource_using_weaves,
        prior_weaves,
    )?;
    if !task_outcome_block_terminates(when_bright) || !task_outcome_block_terminates(when_dim) {
        return Err(CompilerError::new(
            condition.span,
            "AE-TASK-004: each task resource outcome branch must end in yield or a terminal choose",
        ));
    }
    Ok(())
}

fn task_outcome_block_terminates(statements: &[Statement]) -> bool {
    let Some(last) = statements.last() else {
        return false;
    };
    match last {
        Statement::Yield { .. } => true,
        Statement::Choose {
            when_bright,
            when_dim,
            ..
        } => {
            !when_dim.is_empty()
                && task_outcome_block_terminates(when_bright)
                && task_outcome_block_terminates(when_dim)
        }
        Statement::Bind { .. }
        | Statement::Revise { .. }
        | Statement::Speak { .. }
        | Statement::Release { .. }
        | Statement::Checkpoint { .. }
        | Statement::While { .. }
        | Statement::Together { .. }
        | Statement::Raise { .. }
        | Statement::Forward { .. }
        | Statement::Handle { .. } => false,
    }
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
            let value_type = expression_type(
                value,
                scope,
                context.signatures,
                context.records,
                context.shapes,
            )?;
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
    shapes: &[ShapeDeclaration],
) -> Result<ResourceOutcomeScopes, CompilerError> {
    match operation {
        ResourceOperation::Allocate {
            arena,
            buffer,
            capacity,
            destination,
        } => {
            let arena_name = validate_access_arena(arena, scope)?;
            let mut capacity_scope = scope.clone();
            let capacity_type = atom_type(capacity, &mut capacity_scope)?;
            require_source_type(capacity_type, ValueType::Whole, capacity.span, "allocate")?;
            if let AtomKind::Whole(literal) = capacity.kind {
                if !(1..=MAX_TABLE_CAPACITY).contains(&literal)
                    && matches!(
                        scope.get(destination).map(|binding| binding.value_type),
                        Some(ValueType::Table(_))
                    )
                {
                    return Err(CompilerError::new(
                        capacity.span,
                        format!(
                            "AE-LAYOUT-003: table capacity must be between 1 and {MAX_TABLE_CAPACITY}"
                        ),
                    ));
                }
            }
            if let Some(ValueType::Table(shape_id)) =
                scope.get(destination).map(|binding| binding.value_type)
            {
                let original =
                    validate_table_move_destination(buffer, destination, scope, shape_id)?;
                let ResourceBindingState::Table { layout, .. } = original.resource else {
                    return Err(CompilerError::new(
                        buffer.span,
                        "AE-LAYOUT-001: allocate requires a table owner placeholder",
                    ));
                };
                let _ = shape_by_id(shapes, shape_id, span)?;
                resource_plan.record_outcome(
                    span,
                    SemanticResourceOperation::TableAllocate {
                        region: arena_name.clone(),
                        owner: destination.clone(),
                        destination: destination.clone(),
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
                bright_target.resource = ResourceBindingState::Table {
                    shape: shape_id,
                    layout,
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
                return Ok((bright, dim));
            }
            let (element, original) = validate_buffer_move_destination(buffer, destination, scope)?;
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
        ResourceOperation::Store {
            table,
            index,
            field,
            value,
            destination,
        } => {
            let (shape_id, original) = validate_table_store_destination(table, destination, scope)?;
            let ResourceBindingState::Table {
                arena: Some(region),
                allocated: true,
                layout: _,
                ..
            } = &original.resource
            else {
                return Err(CompilerError::new(
                    table.span,
                    "store requires a table that was successfully allocated",
                ));
            };
            let declaration = shape_by_id(shapes, shape_id, span)?;
            let field_index = declaration
                .fields
                .iter()
                .position(|candidate| candidate.name == *field)
                .ok_or_else(|| {
                    CompilerError::new(
                        span,
                        format!(
                            "AE-LAYOUT-002: shape {} has no field named {field}",
                            declaration.name
                        ),
                    )
                })?;
            let field_index = u8::try_from(field_index).map_err(|_| {
                CompilerError::new(span, "shape field index is outside the AETH range")
            })?;
            let mut index_scope = scope.clone();
            require_source_type(
                atom_type(index, &mut index_scope)?,
                ValueType::Whole,
                index.span,
                "store",
            )?;
            let mut value_scope = scope.clone();
            require_source_type(
                atom_type(value, &mut value_scope)?,
                ValueType::Whole,
                value.span,
                "store",
            )?;
            resource_plan.record_outcome(
                span,
                SemanticResourceOperation::TableStore {
                    region: region.clone(),
                    owner: destination.clone(),
                    destination: destination.clone(),
                    shape: shape_id,
                    field: field_index,
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
        ResourceOperation::Load {
            table,
            index,
            field,
            destination,
        } => {
            let table_binding = validate_borrowed_allocated_table(table, scope)?;
            let ValueType::Table(shape_id) = table_binding.value_type else {
                return Err(CompilerError::new(
                    table.span,
                    "load requires a table owner",
                ));
            };
            let ResourceBindingState::Table {
                arena: Some(region),
                allocated: true,
                ..
            } = &table_binding.resource
            else {
                return Err(CompilerError::new(
                    table.span,
                    "load requires a table with a named Arena region",
                ));
            };
            let declaration = shape_by_id(shapes, shape_id, span)?;
            let field_index = declaration
                .fields
                .iter()
                .position(|candidate| candidate.name == *field)
                .ok_or_else(|| {
                    CompilerError::new(
                        span,
                        format!(
                            "AE-LAYOUT-002: shape {} has no field named {field}",
                            declaration.name
                        ),
                    )
                })?;
            let field_index = u8::try_from(field_index).map_err(|_| {
                CompilerError::new(span, "shape field index is outside the AETH range")
            })?;
            let mut index_scope = scope.clone();
            require_source_type(
                atom_type(index, &mut index_scope)?,
                ValueType::Whole,
                index.span,
                "load",
            )?;
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
                ValueType::Whole,
                span,
                "load destination",
            )?;
            let AtomKind::Borrow(owner) = &table.kind else {
                return Err(CompilerError::new(
                    table.span,
                    "load requires borrow followed by an allocated table binding",
                ));
            };
            resource_plan.record_outcome(
                span,
                SemanticResourceOperation::TableLoad {
                    region: region.clone(),
                    owner: owner.clone(),
                    destination: destination.clone(),
                    shape: shape_id,
                    field: field_index,
                },
            )?;
            Ok((scope.clone(), scope.clone()))
        }
    }
}

fn validate_table_move_destination(
    table: &Atom,
    destination: &str,
    scope: &BTreeMap<String, BindingState>,
    expected_shape: u16,
) -> Result<BindingState, CompilerError> {
    let AtomKind::Move(name) = &table.kind else {
        return Err(CompilerError::new(
            table.span,
            "resource replacement requires move followed by its table destination name",
        ));
    };
    if name != destination {
        return Err(CompilerError::new(
            table.span,
            "M6 resource outcomes must restore the same table binding named after into",
        ));
    }
    let binding = scope.get(name).cloned().ok_or_else(|| {
        CompilerError::new(
            table.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.moved {
        return Err(CompilerError::new(
            table.span,
            format!("value {name} was already moved"),
        ));
    }
    if !binding.mutable {
        return Err(CompilerError::new(
            table.span,
            format!("resource destination {name} must be a mutable root binding"),
        ));
    }
    if binding.value_type != ValueType::Table(expected_shape) {
        return Err(CompilerError::new(
            table.span,
            "resource replacement requires a matching table owner",
        ));
    }
    Ok(binding)
}

fn validate_table_store_destination(
    table: &Atom,
    destination: &str,
    scope: &BTreeMap<String, BindingState>,
) -> Result<(u16, BindingState), CompilerError> {
    let AtomKind::Move(name) = &table.kind else {
        return Err(CompilerError::new(
            table.span,
            "store requires move followed by its table destination name",
        ));
    };
    if name != destination {
        return Err(CompilerError::new(
            table.span,
            "M6 store must restore the same table binding named after into",
        ));
    }
    let binding = scope.get(name).cloned().ok_or_else(|| {
        CompilerError::new(
            table.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.moved {
        return Err(CompilerError::new(
            table.span,
            format!("value {name} was already moved"),
        ));
    }
    if !binding.mutable {
        return Err(CompilerError::new(
            table.span,
            format!("resource destination {name} must be a mutable root binding"),
        ));
    }
    let ValueType::Table(shape_id) = binding.value_type else {
        return Err(CompilerError::new(
            table.span,
            "store requires a table owner",
        ));
    };
    Ok((shape_id, binding))
}

fn validate_borrowed_allocated_table<'a>(
    table: &Atom,
    scope: &'a BTreeMap<String, BindingState>,
) -> Result<&'a BindingState, CompilerError> {
    let AtomKind::Borrow(name) = &table.kind else {
        return Err(CompilerError::new(
            table.span,
            "load requires borrow followed by an allocated table binding",
        ));
    };
    let binding = scope.get(name).ok_or_else(|| {
        CompilerError::new(
            table.span,
            format!("value {name} has not been bound in this weave"),
        )
    })?;
    if binding.moved {
        return Err(CompilerError::new(
            table.span,
            format!("value {name} was moved and cannot be borrowed"),
        ));
    }
    if !is_table_type(binding.value_type)
        || !matches!(
            binding.resource,
            ResourceBindingState::Table {
                allocated: true,
                ..
            }
        )
    {
        return Err(CompilerError::new(
            table.span,
            "load requires a table that was successfully allocated",
        ));
    }
    Ok(binding)
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
        ExpressionKind::Arena { .. }
        | ExpressionKind::Buffer { .. }
        | ExpressionKind::Table { .. } => false,
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
        ExpressionKind::Resource(ResourceOperation::Store {
            table,
            index,
            value,
            ..
        }) => {
            atom_moves_name(table, target)
                || atom_moves_name(index, target)
                || atom_moves_name(value, target)
        }
        ExpressionKind::Resource(ResourceOperation::Load { table, index, .. }) => {
            atom_moves_name(table, target) || atom_moves_name(index, target)
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
    shapes: &[ShapeDeclaration],
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => atom_type(atom, scope),
        ExpressionKind::Arena { .. } => Ok(ValueType::Arena),
        ExpressionKind::Buffer { element } => Ok(element.value_type()),
        ExpressionKind::Table { shape, .. } => {
            let (shape_id, _) = shape_by_name(shapes, shape, expression.span)?;
            Ok(ValueType::Table(shape_id))
        }
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
                    if is_table_type(argument_type) {
                        let _ = validate_borrowed_allocated_table(argument, scope)?;
                        return Ok(ValueType::Whole);
                    }
                    let Some(_) = buffer_element_from_value_type(argument_type) else {
                        return Err(CompilerError::new(
                            argument.span,
                            "count requires borrow followed by an allocated BufferWhole, BufferTruth, or table",
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
                            | ValueType::Table(_)
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
            if signature.is_task {
                return Err(CompilerError::new(
                    expression.span,
                    format!(
                        "AE-TASK-005: task weave {weave} may be invoked only by spawn call inside a checkpointed nursery"
                    ),
                ));
            }
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
                    if signature.is_host {
                        format!(
                            "AE-HOST-002: host call {weave} requires {} argument(s), received {}",
                            signature.parameters.len(),
                            arguments.len()
                        )
                    } else {
                        format!(
                            "call {weave} requires {} argument(s), received {}",
                            signature.parameters.len(),
                            arguments.len()
                        )
                    },
                ));
            }
            for (argument, parameter) in arguments.iter().zip(&signature.parameters) {
                let argument_type = atom_type(argument, scope)?;
                let expected = if parameter.mode == ParameterMode::Access {
                    ValueType::AccessArena
                } else {
                    parameter.value_type
                };
                if signature.is_host {
                    if argument_type != expected {
                        return Err(CompilerError::new(
                            argument.span,
                            format!(
                                "AE-HOST-002: host call {weave} argument {} requires {expected}, received {argument_type}",
                                parameter.name
                            ),
                        ));
                    }
                    if parameter.mode == ParameterMode::Borrow
                        && !matches!(
                            argument.kind,
                            AtomKind::Borrow(_) | AtomKind::Text(_) | AtomKind::Bytes(_)
                        )
                    {
                        return Err(CompilerError::new(
                            argument.span,
                            format!(
                                "AE-HOST-002: host call {weave} borrow parameter {} requires borrow name or a matching literal",
                                parameter.name
                            ),
                        ));
                    }
                } else {
                    require_source_type(argument_type, expected, argument.span, "call argument")?;
                }
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
                    | ValueType::Table(_)
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

fn validate_shape_declarations(
    shapes: &[ShapeDeclaration],
    records: &[RecordDeclaration],
) -> Result<(), CompilerError> {
    let _ = shape_type_map(shapes)?;
    for shape in shapes {
        validate_name(&shape.name, shape.span, "shape name", false)?;
        if records.iter().any(|record| record.name == shape.name) {
            return Err(CompilerError::new(
                shape.span,
                format!(
                    "AE-LAYOUT-001: shape {} collides with a record of the same name",
                    shape.name
                ),
            ));
        }
        if shape.fields.is_empty() || shape.fields.len() > MAX_SHAPE_FIELDS {
            return Err(CompilerError::new(
                shape.span,
                format!("AE-LAYOUT-001: shapes require 1 through {MAX_SHAPE_FIELDS} Whole fields"),
            ));
        }
        let mut fields = BTreeMap::new();
        for field in &shape.fields {
            validate_name(&field.name, field.span, "shape field name", false)?;
            if fields.insert(field.name.as_str(), ()).is_some() {
                return Err(CompilerError::new(
                    field.span,
                    format!(
                        "AE-LAYOUT-001: shape field {} is declared more than once",
                        field.name
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn shape_by_name<'a>(
    shapes: &'a [ShapeDeclaration],
    name: &str,
    span: Span,
) -> Result<(u16, &'a ShapeDeclaration), CompilerError> {
    let Some((index, declaration)) = shapes
        .iter()
        .enumerate()
        .find(|(_, declaration)| declaration.name == name)
    else {
        return Err(CompilerError::new(
            span,
            format!("AE-LAYOUT-002: shape {name} has not been declared"),
        ));
    };
    let shape_id = u16::try_from(index)
        .map_err(|_| CompilerError::new(span, "shape identifier is outside the AETH range"))?;
    Ok((shape_id, declaration))
}

fn shape_by_id(
    shapes: &[ShapeDeclaration],
    shape_id: u16,
    span: Span,
) -> Result<&ShapeDeclaration, CompilerError> {
    shapes.get(usize::from(shape_id)).ok_or_else(|| {
        CompilerError::new(
            span,
            "shape type identifier is outside the declared shape table",
        )
    })
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
            if matches!(
                binding.resource,
                ResourceBindingState::Table {
                    allocated: false,
                    ..
                }
            ) {
                return Err(CompilerError::new(
                    atom.span,
                    format!("table {name} is unallocated and cannot be borrowed"),
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
    let artifact_version = if program.weaves.iter().any(|weave| weave.task) {
        ARTIFACT_VERSION_V12
    } else {
        ARTIFACT_VERSION_V11
    };
    // Function table: host weaves first (declaration order), then guest weaves.
    let mut weave_indices = BTreeMap::new();
    let mut weave_results = BTreeMap::new();
    let mut host_flags = BTreeMap::new();
    for (index, host) in program.host_weaves.iter().enumerate() {
        weave_indices.insert(host.name.clone(), index);
        weave_results.insert(host.name.clone(), host.result);
        host_flags.insert(host.name.clone(), true);
    }
    let host_count = program.host_weaves.len();
    for (index, weave) in program.weaves.iter().enumerate() {
        weave_indices.insert(weave.name.clone(), host_count + index);
        weave_results.insert(weave.name.clone(), weave.result);
        host_flags.insert(weave.name.clone(), false);
    }

    let mut compiled_hosts = Vec::new();
    for host in &program.host_weaves {
        let mut locals = Vec::with_capacity(host.parameters.len());
        for parameter in &host.parameters {
            locals.push(LocalDescriptor {
                value_type: parameter.value_type,
                mutable: false,
            });
        }
        compiled_hosts.push((host, locals));
    }

    let mut compiled = Vec::new();
    for (weave_index, weave) in program.weaves.iter().enumerate() {
        let layout = slot_layout(weave, &weave_results, &program.records, &program.shapes)?;
        let mut code = Vec::new();
        let mut comptime_env = BTreeMap::new();
        emit_block(
            &weave.body,
            weave.task,
            &layout,
            &weave_indices,
            &host_flags,
            &program.records,
            &program.shapes,
            resource_plan,
            &mut comptime_env,
            &program.weaves[..weave_index],
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
    bytecode.push(artifact_version);
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
    for record in &program.records {
        let name_length = u8::try_from(record.name.len()).map_err(|_| {
            CompilerError::new(record.span, "record name exceeds the AETH name limit")
        })?;
        bytecode.push(name_length);
        bytecode.extend_from_slice(record.name.as_bytes());
        bytecode.push(
            u8::try_from(record.fields.len()).map_err(|_| {
                CompilerError::new(record.span, "record has too many fields for AETH")
            })?,
        );
        for field in &record.fields {
            let field_name_length = u8::try_from(field.name.len()).map_err(|_| {
                CompilerError::new(field.span, "record field name exceeds the AETH name limit")
            })?;
            bytecode.push(field_name_length);
            bytecode.extend_from_slice(field.name.as_bytes());
            write_primitive_value_type(&mut bytecode, field.value_type, field.span)?;
        }
    }
    write_u16(
        &mut bytecode,
        u16::try_from(program.shapes.len()).map_err(|_| {
            CompilerError::new(
                Span::synthetic(),
                "artifact contains too many shapes for AETH",
            )
        })?,
    );
    for shape in &program.shapes {
        let name_length = u8::try_from(shape.name.len()).map_err(|_| {
            CompilerError::new(shape.span, "shape name exceeds the AETH name limit")
        })?;
        bytecode.push(name_length);
        bytecode.extend_from_slice(shape.name.as_bytes());
        bytecode.push(
            u8::try_from(shape.fields.len()).map_err(|_| {
                CompilerError::new(shape.span, "shape has too many fields for AETH")
            })?,
        );
        for field in &shape.fields {
            let field_name_length = u8::try_from(field.name.len()).map_err(|_| {
                CompilerError::new(field.span, "shape field name exceeds the AETH name limit")
            })?;
            bytecode.push(field_name_length);
            bytecode.extend_from_slice(field.name.as_bytes());
        }
    }
    let function_count = compiled_hosts
        .len()
        .checked_add(compiled.len())
        .ok_or_else(|| {
            CompilerError::new(
                Span::synthetic(),
                "artifact contains too many weaves for AETH",
            )
        })?;
    write_u16(
        &mut bytecode,
        u16::try_from(function_count).map_err(|_| {
            CompilerError::new(
                Span::synthetic(),
                "artifact contains too many weaves for AETH",
            )
        })?,
    );
    for (host, locals) in compiled_hosts {
        let encoded_name = if let Some(foreign) = &host.foreign {
            encode_foreign_function_name(&host.name, &foreign.library, &foreign.symbol)
        } else {
            host.name.clone()
        };
        let name_length = u8::try_from(encoded_name.len()).map_err(|_| {
            CompilerError::new(host.span, "host weave name exceeds the AETH name limit")
        })?;
        bytecode.push(name_length);
        bytecode.extend_from_slice(encoded_name.as_bytes());
        let parameter_count = u8::try_from(host.parameters.len()).map_err(|_| {
            CompilerError::new(host.span, "host weave has too many parameters for AETH")
        })?;
        bytecode.push(parameter_count);
        for parameter in &host.parameters {
            write_value_type(&mut bytecode, parameter.value_type);
            bytecode.push(parameter.mode.to_byte());
        }
        write_value_type(&mut bytecode, host.result);
        bytecode.push(Effect::Total.to_byte());
        bytecode.push(FUNCTION_KIND_HOST);
        if artifact_version == ARTIFACT_VERSION_V12 {
            bytecode.push(0);
            write_u32(&mut bytecode, 0);
        }
        write_u16(
            &mut bytecode,
            u16::try_from(locals.len()).map_err(|_| {
                CompilerError::new(host.span, "host weave has too many local bindings for AETH")
            })?,
        );
        for local in locals {
            write_value_type(&mut bytecode, local.value_type);
            bytecode.push(u8::from(local.mutable));
        }
        write_u32(&mut bytecode, 0);
    }
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
        bytecode.push(FUNCTION_KIND_GUEST);
        if artifact_version == ARTIFACT_VERSION_V12 {
            bytecode.push(if weave.task {
                FUNCTION_FLAG_TASK_FRAME
            } else {
                0
            });
            let frame_capacity = if weave.task || weave.name == "main" {
                resource_plan.direct_arena_capacity(&weave.name)
            } else {
                0
            };
            write_u32(&mut bytecode, frame_capacity);
        }
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
    shapes: &[ShapeDeclaration],
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
            let value_type =
                static_expression_type(value, &layout, weave_results, records, shapes)?;
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
    shapes: &[ShapeDeclaration],
) -> Result<ValueType, CompilerError> {
    match &expression.kind {
        ExpressionKind::Atom(atom) => static_atom_type(atom, layout),
        ExpressionKind::Arena { .. } => Ok(ValueType::Arena),
        ExpressionKind::Buffer { element } => Ok(element.value_type()),
        ExpressionKind::Table { shape, .. } => {
            let (shape_id, _) = shape_by_name(shapes, shape, expression.span)?;
            Ok(ValueType::Table(shape_id))
        }
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

#[allow(clippy::too_many_arguments)]
fn emit_block(
    statements: &[Statement],
    task: bool,
    layout: &BTreeMap<String, SlotInfo>,
    weave_indices: &BTreeMap<String, usize>,
    host_flags: &BTreeMap<String, bool>,
    records: &[RecordDeclaration],
    shapes: &[ShapeDeclaration],
    resource_plan: &SemanticResourcePlan,
    comptime_env: &mut BTreeMap<String, i64>,
    prior_weaves: &[Weave],
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
                    let folded = evaluate_comptime_whole(value, comptime_env, prior_weaves)?;
                    comptime_env.insert(name.clone(), folded);
                    code.push(OP_COMPTIME_WHOLE);
                    write_i64(code, folded);
                } else {
                    emit_expression(
                        value,
                        layout,
                        weave_indices,
                        host_flags,
                        records,
                        shapes,
                        resource_plan,
                        code,
                    )?;
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
                emit_expression(
                    value,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    code,
                )?;
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
                emit_expression(
                    value,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    code,
                )?;
                code.push(OP_SPEAK);
            }
            Statement::Yield { value, .. } => {
                emit_expression(
                    value,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    code,
                )?;
                code.push(OP_YIELD);
            }
            Statement::Release { name, .. } => {
                let slot = layout.get(name).ok_or_else(|| {
                    CompilerError::new(
                        statement.span(),
                        "internal compiler could not resolve release slot",
                    )
                })?;
                code.push(OP_RELEASE);
                write_u16(code, slot.index);
            }
            Statement::Checkpoint { .. } => code.push(OP_TASK_CHECKPOINT),
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
                        host_flags,
                        records,
                        shapes,
                        resource_plan,
                        code,
                    )?;
                    code.push(OP_JUMP_IF_DIM);
                    let dim_target = reserve_u32(code);
                    emit_block(
                        when_bright,
                        task,
                        layout,
                        weave_indices,
                        host_flags,
                        records,
                        shapes,
                        resource_plan,
                        comptime_env,
                        prior_weaves,
                        code,
                    )?;
                    let dim_branch = code.len();
                    patch_u32(code, dim_target, dim_branch)?;
                    emit_block(
                        when_dim,
                        task,
                        layout,
                        weave_indices,
                        host_flags,
                        records,
                        shapes,
                        resource_plan,
                        comptime_env,
                        prior_weaves,
                        code,
                    )?;
                    continue;
                }
                emit_expression(
                    condition,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    code,
                )?;
                code.push(OP_JUMP_IF_DIM);
                let dim_target = reserve_u32(code);
                emit_block(
                    when_bright,
                    task,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    comptime_env,
                    prior_weaves,
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
                        task,
                        layout,
                        weave_indices,
                        host_flags,
                        records,
                        shapes,
                        resource_plan,
                        comptime_env,
                        prior_weaves,
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
                if task {
                    // The back edge of every v12 task loop targets a verified
                    // checkpoint.  The source still requires the loop body to
                    // begin with its own direct checkpoint, so each iteration
                    // has an explicit safe point before the condition and body.
                    code.push(OP_TASK_CHECKPOINT);
                }
                emit_expression(
                    condition,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    code,
                )?;
                code.push(OP_JUMP_IF_DIM);
                let loop_end = reserve_u32(code);
                emit_block(
                    body,
                    task,
                    layout,
                    weave_indices,
                    host_flags,
                    records,
                    shapes,
                    resource_plan,
                    comptime_env,
                    prior_weaves,
                    code,
                )?;
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
            Statement::Together { spawns, .. } => {
                let count = u8::try_from(spawns.len()).map_err(|_| {
                    CompilerError::new(
                        statement.span(),
                        "nursery spawn count is outside AETH range",
                    )
                })?;
                code.push(OP_NURSERY_BEGIN);
                code.push(count);
                for spawn in spawns {
                    for argument in &spawn.arguments {
                        emit_atom(argument, layout, code)?;
                    }
                    let function = weave_indices.get(&spawn.weave).ok_or_else(|| {
                        CompilerError::new(
                            spawn.span,
                            format!("internal compiler could not resolve weave {}", spawn.weave),
                        )
                    })?;
                    let destination = layout.get(&spawn.destination).ok_or_else(|| {
                        CompilerError::new(
                            spawn.span,
                            "internal compiler could not resolve spawn destination",
                        )
                    })?;
                    code.push(OP_NURSERY_SPAWN);
                    write_u16(
                        code,
                        u16::try_from(*function).map_err(|_| {
                            CompilerError::new(spawn.span, "weave index is outside the AETH range")
                        })?,
                    );
                    code.push(u8::try_from(spawn.arguments.len()).map_err(|_| {
                        CompilerError::new(spawn.span, "spawn has too many AETH arguments")
                    })?);
                    write_u16(code, destination.index);
                }
                code.push(OP_NURSERY_END);
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_expression(
    expression: &Expression,
    layout: &BTreeMap<String, SlotInfo>,
    weave_indices: &BTreeMap<String, usize>,
    host_flags: &BTreeMap<String, bool>,
    records: &[RecordDeclaration],
    shapes: &[ShapeDeclaration],
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
        ExpressionKind::Table {
            shape,
            layout: table_layout,
        } => {
            let (shape_id, _) = shape_by_name(shapes, shape, expression.span)?;
            code.push(OP_TABLE);
            write_u16(code, shape_id);
            code.push(table_layout.to_tag());
        }
        ExpressionKind::Unary {
            operation,
            argument,
        } => {
            emit_atom(argument, layout, code)?;
            if *operation == UnaryOperation::Count {
                if let AtomKind::Borrow(name) | AtomKind::Name(name) = &argument.kind {
                    if matches!(
                        layout.get(name).map(|slot| slot.value_type),
                        Some(ValueType::Table(_))
                    ) {
                        code.push(OP_TABLE_COUNT);
                    } else {
                        code.push(OP_COUNT);
                    }
                } else {
                    code.push(OP_COUNT);
                }
            } else {
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
            let is_host = host_flags.get(weave).copied().unwrap_or(false);
            code.push(if is_host { OP_HOST_CALL } else { OP_CALL });
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

            match (operation, semantic) {
                (
                    ResourceOperation::Allocate {
                        arena,
                        buffer,
                        capacity,
                        ..
                    },
                    SemanticResourceOperation::Allocate { element, .. },
                ) => {
                    emit_atom(arena, layout, code)?;
                    emit_atom(buffer, layout, code)?;
                    emit_atom(capacity, layout, code)?;
                    code.push(OP_ALLOCATE);
                    code.push(element.type_tag());
                    write_u16(code, destination_index);
                }
                (
                    ResourceOperation::Allocate {
                        arena,
                        buffer,
                        capacity,
                        ..
                    },
                    SemanticResourceOperation::TableAllocate { .. },
                ) => {
                    emit_atom(arena, layout, code)?;
                    emit_atom(buffer, layout, code)?;
                    emit_atom(capacity, layout, code)?;
                    code.push(OP_TABLE_ALLOCATE);
                    write_u16(code, destination_index);
                }
                (
                    ResourceOperation::Append { buffer, value, .. },
                    SemanticResourceOperation::Append { element, .. },
                ) => {
                    emit_atom(buffer, layout, code)?;
                    emit_atom(value, layout, code)?;
                    code.push(OP_BUFFER_APPEND);
                    code.push(element.type_tag());
                    write_u16(code, destination_index);
                }
                (
                    ResourceOperation::At { buffer, index, .. },
                    SemanticResourceOperation::At { element, .. },
                ) => {
                    emit_atom(buffer, layout, code)?;
                    emit_atom(index, layout, code)?;
                    code.push(OP_BUFFER_AT);
                    code.push(element.type_tag());
                    write_u16(code, destination_index);
                }
                (
                    ResourceOperation::Store {
                        table,
                        index,
                        value,
                        ..
                    },
                    SemanticResourceOperation::TableStore { shape, field, .. },
                ) => {
                    emit_atom(table, layout, code)?;
                    emit_atom(index, layout, code)?;
                    emit_atom(value, layout, code)?;
                    code.push(OP_TABLE_STORE);
                    write_u16(code, *shape);
                    code.push(*field);
                    write_u16(code, destination_index);
                }
                (
                    ResourceOperation::Load { table, index, .. },
                    SemanticResourceOperation::TableLoad { shape, field, .. },
                ) => {
                    emit_atom(table, layout, code)?;
                    emit_atom(index, layout, code)?;
                    code.push(OP_TABLE_LOAD);
                    write_u16(code, *shape);
                    code.push(*field);
                    write_u16(code, destination_index);
                }
                _ => {
                    return Err(CompilerError::new(
                        expression.span,
                        "internal compiler found a resource AST/semantic-plan disagreement",
                    ))
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

pub(crate) fn parse_artifact(bytecode: &[u8]) -> Result<Artifact, BytecodeError> {
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
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) {
        return Err(BytecodeError::new(
            ARTIFACT_MAGIC.len(),
            "artifact version is not supported by this Aether VM",
        ));
    }
    let mut position = ARTIFACT_MAGIC.len() + 1;
    let arena_capacity = if matches!(
        version,
        ARTIFACT_VERSION_V6
            | ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) {
        let capacity = read_u32(bytecode, &mut position)?;
        if capacity > MAX_ARENA_BYTES {
            return Err(BytecodeError::new(
                position,
                "AETH v6 through v12 arena capacity exceeds the M2 safety limit",
            ));
        }
        capacity
    } else {
        0
    };
    let mut records = Vec::new();
    if matches!(
        version,
        ARTIFACT_VERSION_V5
            | ARTIFACT_VERSION_V6
            | ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
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
    let mut shapes = Vec::new();
    if matches!(
        version,
        ARTIFACT_VERSION_V9 | ARTIFACT_VERSION_V10 | ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12
    ) {
        let shape_count = usize::from(read_u16(bytecode, &mut position)?);
        if shape_count > MAX_SHAPES {
            return Err(BytecodeError::new(
                position,
                "artifact shape count is outside the Aether limit",
            ));
        }
        shapes.reserve(shape_count);
        for _ in 0..shape_count {
            let name_offset = position;
            let name_length = usize::from(read_byte(bytecode, &mut position)?);
            let name = read_ascii(bytecode, &mut position, name_length, "shape name")?;
            validate_artifact_name(&name, name_offset, "shape name", false)?;
            if shapes
                .iter()
                .any(|shape: &ArtifactShape| shape.name == name)
                || records
                    .iter()
                    .any(|record: &ArtifactRecord| record.name == name)
            {
                return Err(BytecodeError::new(
                    name_offset,
                    "artifact defines a shape name twice or collides with a record",
                ));
            }
            let field_count = usize::from(read_byte(bytecode, &mut position)?);
            if field_count == 0 || field_count > MAX_SHAPE_FIELDS {
                return Err(BytecodeError::new(
                    position,
                    "artifact shape field count is outside the Aether limit",
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
                    "shape field name",
                )?;
                validate_artifact_name(&field_name, field_offset, "shape field name", false)?;
                if fields.iter().any(|field| field == &field_name) {
                    return Err(BytecodeError::new(
                        field_offset,
                        "artifact shape defines a field name twice",
                    ));
                }
                fields.push(field_name);
            }
            shapes.push(ArtifactShape { name, fields });
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
            let value_type = read_value_type(
                bytecode,
                &mut position,
                version,
                records.len(),
                shapes.len(),
            )?;
            let mode =
                ParameterMode::from_byte(read_byte(bytecode, &mut position)?, position, version)?;
            parameters.push((value_type, mode));
        }
        let result = read_value_type(
            bytecode,
            &mut position,
            version,
            records.len(),
            shapes.len(),
        )?;
        let effect = if matches!(
            version,
            ARTIFACT_VERSION_V7
                | ARTIFACT_VERSION_V8
                | ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) {
            Effect::from_byte(read_byte(bytecode, &mut position)?, position)?
        } else {
            Effect::Total
        };
        let kind = if matches!(version, ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12) {
            let kind = read_byte(bytecode, &mut position)?;
            if kind != FUNCTION_KIND_GUEST && kind != FUNCTION_KIND_HOST {
                return Err(BytecodeError::new(
                    position,
                    "artifact function kind must be guest (0) or host (1)",
                ));
            }
            kind
        } else {
            FUNCTION_KIND_GUEST
        };
        let (flags, frame_arena_capacity) = if version == ARTIFACT_VERSION_V12 {
            let flags = read_byte(bytecode, &mut position)?;
            if flags & !FUNCTION_FLAG_TASK_FRAME != 0 {
                return Err(BytecodeError::new(
                    position,
                    "AETH v12 function flags contain an unknown bit",
                ));
            }
            let frame_arena_capacity = read_u32(bytecode, &mut position)?;
            if frame_arena_capacity > MAX_ARENA_BYTES {
                return Err(BytecodeError::new(
                    position,
                    "AETH v12 frame arena capacity exceeds the M2 safety limit",
                ));
            }
            (flags, frame_arena_capacity)
        } else {
            (0, 0)
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
            let value_type = read_value_type(
                bytecode,
                &mut position,
                version,
                records.len(),
                shapes.len(),
            )?;
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
        if kind == FUNCTION_KIND_HOST {
            if effect != Effect::Total {
                return Err(BytecodeError::new(
                    position,
                    "host weaves must remain total in AETH v11",
                ));
            }
            if code_length != 0 {
                return Err(BytecodeError::new(
                    position,
                    "host weaves must have empty guest bytecode",
                ));
            }
            for (value_type, mode) in &parameters {
                let legal = matches!(
                    (*mode, *value_type),
                    (ParameterMode::Own, ValueType::Whole | ValueType::Truth)
                        | (ParameterMode::Borrow, ValueType::Text | ValueType::Bytes)
                );
                if !legal {
                    return Err(BytecodeError::new(
                        position,
                        "host weave parameter ABI is outside the M8 primitive surface",
                    ));
                }
            }
            if !matches!(
                result,
                ValueType::Whole | ValueType::Truth | ValueType::Text | ValueType::Bytes
            ) {
                return Err(BytecodeError::new(
                    position,
                    "host weave result ABI is outside the M8 primitive surface",
                ));
            }
        }
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
            kind,
            flags,
            frame_arena_capacity,
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
        shapes,
        functions,
    })
}

fn verify_function(
    function_index: usize,
    function: &ArtifactFunction,
    functions: &[ArtifactFunction],
    records: &[ArtifactRecord],
    shapes: &[ArtifactShape],
    version: u8,
) -> Result<(), BytecodeError> {
    if function.is_host() {
        if !matches!(version, ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12) {
            return Err(BytecodeError::new(
                0,
                "host weaves are valid only in AETH v11 or v12 artifacts",
            ));
        }
        if function.flags != 0 || function.frame_arena_capacity != 0 {
            return Err(BytecodeError::new(
                0,
                "AETH v12 host weaves require zero task flags and zero frame arena capacity",
            ));
        }
        if !function.code.is_empty() {
            return Err(BytecodeError::new(
                0,
                "host weaves must have empty guest bytecode",
            ));
        }
        if function.name == "main" {
            return Err(BytecodeError::new(0, "main cannot be a host weave"));
        }
        return Ok(());
    }
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
    if function.is_task() {
        if version != ARTIFACT_VERSION_V12 {
            return Err(BytecodeError::new(
                0,
                "task-frame function metadata is valid only in AETH v12",
            ));
        }
        let mut checkpoint_seen = false;
        for instruction in &decoded {
            match instruction.instruction {
                Instruction::TaskCheckpoint => checkpoint_seen = true,
                Instruction::Speak
                | Instruction::Raise
                | Instruction::ForwardCall { .. }
                | Instruction::HandleCall { .. }
                | Instruction::Call { .. }
                | Instruction::HostCall { .. }
                | Instruction::NurseryBegin { .. }
                | Instruction::NurserySpawn { .. }
                | Instruction::NurseryEnd
                | Instruction::ComptimeWhole(_) => {
                    return Err(BytecodeError::new(
                        instruction.offset,
                        "AETH v12 task frame contains an instruction outside the self-contained task subset",
                    ));
                }
                Instruction::Jump(target) if target < instruction.offset => {
                    let Some(target_index) = instruction_indices.get(&target) else {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "task backward jump target is invalid",
                        ));
                    };
                    if !matches!(
                        &decoded[*target_index].instruction,
                        Instruction::TaskCheckpoint
                    ) {
                        return Err(BytecodeError::new(
                            instruction.offset,
                            "AETH v12 task backward jumps must target TASK_CHECKPOINT",
                        ));
                    }
                }
                _ => {}
            }
        }
        if !checkpoint_seen {
            return Err(BytecodeError::new(
                0,
                "AETH v12 task frame requires at least one TASK_CHECKPOINT",
            ));
        }
    } else if version != ARTIFACT_VERSION_V12
        && (function.flags != 0 || function.frame_arena_capacity != 0)
    {
        return Err(BytecodeError::new(
            0,
            "pre-v12 function metadata cannot carry task flags or frame capacity",
        ));
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
        nursery: None,
    };
    states.insert(0_usize, initial);
    let mut queue = VecDeque::from([0_usize]);
    let mut yielded = false;
    let mut effect_exited = false;
    let context = VerificationContext {
        function_index,
        version,
        function,
        functions,
        records,
        shapes,
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
    let version = context.version;
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
        Instruction::Table { shape, .. } => {
            if usize::from(*shape) >= context.shapes.len() {
                return Err(BytecodeError::new(
                    offset,
                    "table shape identifier is outside the artifact shape table",
                ));
            }
            state.stack.push_table_placeholder(*shape);
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
        Instruction::TableAllocate { destination } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || !is_table_type(local.value_type) {
                return Err(BytecodeError::new(
                    offset,
                    "table allocate destination must be its matching mutable Table slot",
                ));
            }
            if !state.initialized[*destination] || !state.moved[*destination] {
                return Err(BytecodeError::new(
                    offset,
                    "table allocate requires its destination Table slot to be moved",
                ));
            }
            pop_type(&mut state.stack, ValueType::Whole, offset, "table allocate")?;
            let table = pop_type(&mut state.stack, local.value_type, offset, "table allocate")?;
            require_moved_resource_from(&table, *destination, offset, "table allocate")?;
            pop_type(
                &mut state.stack,
                ValueType::AccessArena,
                offset,
                "table allocate",
            )?;
            state.moved[*destination] = false;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::TableStore {
            shape,
            field,
            destination,
        } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || local.value_type != ValueType::Table(*shape) {
                return Err(BytecodeError::new(
                    offset,
                    "table store destination must be its matching mutable Table slot",
                ));
            }
            if !state.initialized[*destination] || !state.moved[*destination] {
                return Err(BytecodeError::new(
                    offset,
                    "table store requires its destination Table slot to be moved",
                ));
            }
            let shape_decl = context.shapes.get(usize::from(*shape)).ok_or_else(|| {
                BytecodeError::new(offset, "table store shape is outside the shape table")
            })?;
            if usize::from(*field) >= shape_decl.fields.len() {
                return Err(BytecodeError::new(
                    offset,
                    "table store field is outside the shape",
                ));
            }
            pop_type(&mut state.stack, ValueType::Whole, offset, "table store")?;
            pop_type(&mut state.stack, ValueType::Whole, offset, "table store")?;
            let table = pop_type(
                &mut state.stack,
                ValueType::Table(*shape),
                offset,
                "table store",
            )?;
            require_moved_resource_from(&table, *destination, offset, "table store")?;
            state.moved[*destination] = false;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::TableLoad {
            shape,
            field,
            destination,
        } => {
            let local = local_descriptor(function, *destination, offset)?;
            if !local.mutable || local.value_type != ValueType::Whole {
                return Err(BytecodeError::new(
                    offset,
                    "table load destination must be a mutable Whole slot",
                ));
            }
            ensure_readable(&state, *destination, offset, "table load destination")?;
            let shape_decl = context.shapes.get(usize::from(*shape)).ok_or_else(|| {
                BytecodeError::new(offset, "table load shape is outside the shape table")
            })?;
            if usize::from(*field) >= shape_decl.fields.len() {
                return Err(BytecodeError::new(
                    offset,
                    "table load field is outside the shape",
                ));
            }
            pop_type(&mut state.stack, ValueType::Whole, offset, "table load")?;
            let table = pop_type(
                &mut state.stack,
                ValueType::Table(*shape),
                offset,
                "table load",
            )?;
            require_borrowed_resource(&table, offset, "table load")?;
            state.stack.push(ValueType::Truth);
            continue_with(state)
        }
        Instruction::TableCount => {
            let table = pop_any_type(&mut state.stack, offset, "table count")?;
            if !is_table_type(table.value_type) {
                return Err(BytecodeError::new(
                    offset,
                    "table count requires a Table value",
                ));
            }
            require_borrowed_resource(&table, offset, "table count")?;
            state.stack.push(ValueType::Whole);
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
            if is_table_type(local.value_type) {
                require_storable_resource(&value, offset, "store")?;
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
            if is_resource_owner_type(local.value_type) {
                return Err(BytecodeError::new(
                    offset,
                    "AETH resource owners are replaced only by closed resource instructions",
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
                    | ValueType::Table(_)
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
            if called_function.is_task() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH v12 task frames may be invoked only by NURSERY_SPAWN",
                ));
            }
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
            if called_function.is_task() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH v12 task frames may be invoked only by NURSERY_SPAWN",
                ));
            }
            verify_effect_call_signature(function, called_function, *arguments, offset, "handle")?;
            if function.result != ValueType::Whole || called_function.result != ValueType::Whole {
                return Err(BytecodeError::new(
                    offset,
                    "AETH HANDLE_CALL requires Whole caller and callee results for the bounded terminal handler form",
                ));
            }
            pop_verifier_effect_arguments(&mut state.stack, called_function, offset, "handle")?;
            require_verifier_handle_boundary(function, &state, offset)?;
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
            if called_function.is_host() {
                return Err(BytecodeError::new(
                    offset,
                    "ordinary AETH CALL cannot invoke a host weave; use HOST_CALL",
                ));
            }
            if called_function.is_task() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH v12 task frames may be invoked only by NURSERY_SPAWN",
                ));
            }
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
        Instruction::HostCall {
            function: called,
            arguments,
        } => {
            let Some(called_function) = functions.get(*called) else {
                return Err(BytecodeError::new(
                    offset,
                    "host call references an unknown weave",
                ));
            };
            if !called_function.is_host() {
                return Err(BytecodeError::new(
                    offset,
                    "HOST_CALL can only target a host weave entry",
                ));
            }
            if called_function.effect != Effect::Total {
                return Err(BytecodeError::new(offset, "host weaves must remain total"));
            }
            if called_function.parameters.len() != *arguments {
                return Err(BytecodeError::new(
                    offset,
                    "host call argument count disagrees with its weave signature",
                ));
            }
            for (expected, mode) in called_function.parameters.iter().rev() {
                if *mode == ParameterMode::Access
                    || is_buffer_type(*expected)
                    || is_table_type(*expected)
                    || matches!(
                        expected,
                        ValueType::Record(_) | ValueType::Arena | ValueType::AccessArena
                    )
                {
                    return Err(BytecodeError::new(
                        offset,
                        "host call cannot cross a resource or record boundary",
                    ));
                }
                pop_type(&mut state.stack, *expected, offset, "host call")?;
            }
            if matches!(
                called_function.result,
                ValueType::Record(_)
                    | ValueType::Arena
                    | ValueType::AccessArena
                    | ValueType::BufferWhole
                    | ValueType::BufferTruth
                    | ValueType::Table(_)
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "host call cannot yield a resource or record",
                ));
            }
            state.stack.push(called_function.result);
            let _ = function_index;
            continue_with(state)
        }
        Instruction::NurseryBegin { count } => {
            if state.nursery.is_some() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_BEGIN cannot nest another nursery",
                ));
            }
            if *count == 0 || usize::from(*count) > MAX_NURSERY_SPAWNS {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_BEGIN count must be between 1 and 8",
                ));
            }
            if !state.stack.is_empty() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_BEGIN requires an empty operand stack",
                ));
            }
            state.nursery = Some(NurseryVerification {
                expected: *count,
                seen: 0,
                targets: Vec::new(),
            });
            continue_with(state)
        }
        Instruction::NurserySpawn {
            function: called,
            arguments,
            destination,
        } => {
            let Some(nursery) = state.nursery.as_mut() else {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_SPAWN requires an open nursery frame",
                ));
            };
            if nursery.seen >= nursery.expected {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_SPAWN exceeds the nursery begin count",
                ));
            }
            let Some(called_function) = functions.get(*called) else {
                return Err(BytecodeError::new(
                    offset,
                    "nursery spawn references an unknown weave",
                ));
            };
            if called_function.result != ValueType::Whole {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_SPAWN target must return Whole",
                ));
            }
            if called_function.effect == Effect::ErrorWhole {
                if function.effect != Effect::ErrorWhole {
                    return Err(BytecodeError::new(
                        offset,
                        "AETH NURSERY_SPAWN of Error[Whole] requires an Error[Whole] enclosing weave",
                    ));
                }
                verify_effect_call_signature(
                    function,
                    called_function,
                    *arguments,
                    offset,
                    "nursery spawn",
                )?;
                pop_verifier_effect_arguments(
                    &mut state.stack,
                    called_function,
                    offset,
                    "nursery spawn",
                )?;
            } else {
                if called_function.parameters.len() != *arguments {
                    return Err(BytecodeError::new(
                        offset,
                        "nursery spawn argument count disagrees with its weave signature",
                    ));
                }
                for (expected, mode) in called_function.parameters.iter().rev() {
                    if *mode == ParameterMode::Access
                        || is_buffer_type(*expected)
                        || *expected == ValueType::Arena
                        || is_table_type(*expected)
                    {
                        return Err(BytecodeError::new(
                            offset,
                            "AETH NURSERY_SPAWN cannot cross arena, Buffer, access, or table boundaries",
                        ));
                    }
                    pop_type(&mut state.stack, *expected, offset, "nursery spawn")?;
                }
            }
            let dest = local_descriptor(function, *destination, offset)?;
            if !dest.mutable
                || dest.value_type != ValueType::Whole
                || !state.initialized[*destination]
                || state.moved[*destination]
            {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_SPAWN destination must be a live mutable Whole local",
                ));
            }
            if let Some(nursery) = state.nursery.as_mut() {
                nursery.seen = nursery.seen.saturating_add(1);
                nursery.targets.push(*called);
            }
            continue_with(state)
        }
        Instruction::NurseryEnd => {
            let Some(nursery) = state.nursery.take() else {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_END requires an open nursery frame",
                ));
            };
            if nursery.seen != nursery.expected {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_END spawn count disagrees with NURSERY_BEGIN",
                ));
            }
            if !state.stack.is_empty() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH NURSERY_END requires an empty operand stack",
                ));
            }
            // Cancel may re-raise at runtime; the success path continues after END.
            let _ = effect_exited;
            continue_with(state)
        }
        Instruction::TaskCheckpoint => {
            if version != ARTIFACT_VERSION_V12 || !function.is_task() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH TASK_CHECKPOINT requires an AETH v12 task-frame function",
                ));
            }
            if !state.stack.is_empty() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH TASK_CHECKPOINT requires an empty operand stack",
                ));
            }
            if state.nursery.is_some() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH TASK_CHECKPOINT cannot occur with an open nursery",
                ));
            }
            if function.locals.iter().enumerate().any(|(index, local)| {
                state.initialized[index]
                    && !state.moved[index]
                    && local.value_type == ValueType::AccessArena
            }) {
                return Err(BytecodeError::new(
                    offset,
                    "AETH TASK_CHECKPOINT cannot cross a live exclusive Arena access loan",
                ));
            }
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
        Instruction::Release(slot) => {
            if !state.stack.is_empty() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH RELEASE requires an empty operand stack",
                ));
            }
            if *slot >= function.locals.len() {
                return Err(BytecodeError::new(
                    offset,
                    "AETH RELEASE references an invalid local slot",
                ));
            }
            if !state.initialized[*slot] || state.moved[*slot] {
                return Err(BytecodeError::new(
                    offset,
                    "AETH RELEASE requires an initialized live local",
                ));
            }
            let local = &function.locals[*slot];
            let releasable = is_unique_value(local.value_type)
                || matches!(
                    local.value_type,
                    ValueType::Arena | ValueType::BufferWhole | ValueType::BufferTruth
                )
                || is_table_type(local.value_type);
            if !releasable || local.value_type == ValueType::AccessArena {
                return Err(BytecodeError::new(
                    offset,
                    "AETH RELEASE cannot target a copy value or access loan",
                ));
            }
            state.moved[*slot] = true;
            continue_with(state)
        }
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
                || local.value_type == ValueType::AccessArena
                || is_buffer_type(local.value_type)
                || is_table_type(local.value_type))
    }) {
        return Err(BytecodeError::new(
            offset,
            "AETH effect control cannot cross a live owner, Arena, or Buffer local",
        ));
    }
    Ok(())
}

/// M16: HANDLE_CALL may keep live resource owners; exclusive access loans must not span it.
fn require_verifier_handle_boundary(
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
            && local.value_type == ValueType::AccessArena
    }) {
        return Err(BytecodeError::new(
            offset,
            "AETH HANDLE_CALL cannot cross a live exclusive Arena access loan",
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
    if function.is_host() {
        return Err(BytecodeError::new(
            0,
            "runtime cannot execute a host weave as guest bytecode",
        ));
    }
    let (mut locals, mut stack, mut position) =
        if let Some(frame) = runtime_state.resumed_task.take() {
            if !function.is_task() || frame.function_index != function_index {
                return Err(BytecodeError::new(
                    0,
                    "runtime task resume does not match its verified task function",
                ));
            }
            if !arguments.is_empty() {
                return Err(BytecodeError::new(
                    0,
                    "runtime task resume cannot receive a second argument list",
                ));
            }
            if runtime_state.active_task_lane.is_some() {
                return Err(BytecodeError::new(
                    0,
                    "runtime task resume found another active private arena lane",
                ));
            }
            runtime_state.active_task_lane = Some(frame.lane);
            (frame.locals, frame.stack, frame.position)
        } else {
            if function.is_task() && runtime_state.active_task_lane.is_none() {
                return Err(BytecodeError::new(
                    0,
                    "AETH v12 task frames may run only inside the checkpointed nursery scheduler",
                ));
            }
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
            (locals, Vec::new(), 0)
        };
    while position < function.code.len() {
        let decoded = decode_instruction(&function.code, &mut position, artifact.version)?;
        match decoded.instruction {
            Instruction::PushText(value) => stack.push(RuntimeValue::Text(RuntimeText::new(value))),
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
            Instruction::Table { shape, layout } => {
                let field_count = artifact
                    .shapes
                    .get(usize::from(shape))
                    .map(|shape| shape.fields.len())
                    .ok_or_else(|| {
                        BytecodeError::new(decoded.offset, "runtime table shape is invalid")
                    })?;
                let field_count = u8::try_from(field_count).map_err(|_| {
                    BytecodeError::new(decoded.offset, "runtime table field count is invalid")
                })?;
                stack.push(RuntimeValue::Table {
                    shape,
                    layout,
                    field_count,
                    allocated: false,
                    offset: 0,
                    capacity: 0,
                });
            }
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
            Instruction::TableAllocate { destination } => {
                let capacity = pop_whole(&mut stack, decoded.offset, "table allocate")?;
                let table = pop_runtime(&mut stack, decoded.offset, "table allocate")?;
                match pop_runtime(&mut stack, decoded.offset, "table allocate")? {
                    RuntimeValue::AccessArena => {}
                    value => {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            format!(
                                "table allocate received {}, expected exclusive Arena access",
                                value.value_type()
                            ),
                        ))
                    }
                }
                let (table, allocated) =
                    runtime_allocate_table(table, capacity, runtime_state, decoded.offset)?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime table allocate destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != table.value_type() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime table allocate destination is not its matching mutable Table slot",
                    ));
                }
                if locals[destination].is_some() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime table allocate destination Table was not moved",
                    ));
                }
                locals[destination] = Some(table);
                stack.push(RuntimeValue::Truth(allocated));
            }
            Instruction::TableStore {
                shape,
                field,
                destination,
            } => {
                let value = pop_whole(&mut stack, decoded.offset, "table store")?;
                let index = pop_whole(&mut stack, decoded.offset, "table store")?;
                let table = pop_runtime(&mut stack, decoded.offset, "table store")?;
                let (table, stored) = runtime_table_store(
                    table,
                    shape,
                    field,
                    index,
                    value,
                    runtime_state,
                    decoded.offset,
                )?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime table store destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != ValueType::Table(shape) {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime table store destination is not its matching mutable Table slot",
                    ));
                }
                if locals[destination].is_some() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime table store destination Table was not moved",
                    ));
                }
                locals[destination] = Some(table);
                stack.push(RuntimeValue::Truth(stored));
            }
            Instruction::TableLoad {
                shape,
                field,
                destination,
            } => {
                let index = pop_whole(&mut stack, decoded.offset, "table load")?;
                let table = pop_runtime(&mut stack, decoded.offset, "table load")?;
                let local = function.locals.get(destination).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime table load destination slot is invalid",
                    )
                })?;
                if !local.mutable || local.value_type != ValueType::Whole {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime table load destination is not a mutable Whole slot",
                    ));
                }
                let fallback = read_local(
                    &locals,
                    destination,
                    decoded.offset,
                    "table load destination",
                )?
                .clone();
                let (value, found) = runtime_table_load(
                    table,
                    shape,
                    field,
                    index,
                    fallback,
                    runtime_state,
                    decoded.offset,
                )?;
                locals[destination] = Some(value);
                stack.push(RuntimeValue::Truth(found));
            }
            Instruction::TableCount => {
                let table = pop_runtime(&mut stack, decoded.offset, "table count")?;
                let RuntimeValue::Table {
                    capacity,
                    allocated,
                    ..
                } = table
                else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "table count received a non-table value",
                    ));
                };
                if !allocated {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "table count received an unallocated table",
                    ));
                }
                stack.push(RuntimeValue::Whole(i64::try_from(capacity).map_err(
                    |_| {
                        BytecodeError::new(
                            decoded.offset,
                            "table capacity is outside the Whole range",
                        )
                    },
                )?));
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
                    ensure_text_limit(stdout.len(), value.byte_len(), decoded.offset)?;
                    stdout.push_str(value.as_str());
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
                if function.is_task() {
                    let lane = runtime_state.active_task_lane.take().ok_or_else(|| {
                        BytecodeError::new(
                            decoded.offset,
                            "AETH v12 task yield has no active private arena lane",
                        )
                    })?;
                    return Ok(RuntimeExit::TaskReturn {
                        value,
                        frame: TaskFrame {
                            function_index,
                            locals,
                            stack,
                            position,
                            lane,
                        },
                    });
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
                ensure_text_limit(left.byte_len(), right.byte_len(), decoded.offset)?;
                stack.push(RuntimeValue::Text(left.join(&right)));
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
                    i64::try_from(text.scalar_len()).map_err(|_| {
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
                    .and_then(|index| text.scalar_at(index))
                    .map_or(-1_i64, |character| i64::from(u32::from(character)));
                stack.push(RuntimeValue::Whole(value));
            }
            Instruction::Cut => {
                let end = pop_whole(&mut stack, decoded.offset, "cut")?;
                let start = pop_whole(&mut stack, decoded.offset, "cut")?;
                let text = pop_text(&mut stack, decoded.offset, "cut")?;
                let length = i64::try_from(text.scalar_len()).map_err(|_| {
                    BytecodeError::new(decoded.offset, "text length is outside Whole range")
                })?;
                let start = start.clamp(0, length);
                let end = end.clamp(start, length);
                let start = usize::try_from(start)
                    .map_err(|_| BytecodeError::new(decoded.offset, "cut start is invalid"))?;
                let end = usize::try_from(end)
                    .map_err(|_| BytecodeError::new(decoded.offset, "cut end is invalid"))?;
                let slice = text
                    .slice_scalars(start, end)
                    .ok_or_else(|| BytecodeError::new(decoded.offset, "cut range is invalid"))?;
                stack.push(RuntimeValue::Text(slice));
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
                let length = i64::try_from(text.scalar_len()).map_err(|_| {
                    BytecodeError::new(decoded.offset, "text length is outside Whole range")
                })?;
                let start = start.clamp(0, length);
                let start = usize::try_from(start)
                    .map_err(|_| BytecodeError::new(decoded.offset, "seek start is invalid"))?;
                stack.push(RuntimeValue::Whole(text.find_from_scalar(&needle, start)));
            }
            Instruction::Encode => {
                let text = pop_text(&mut stack, decoded.offset, "encode")?;
                ensure_bytes_limit(0, text.byte_len(), decoded.offset)?;
                stack.push(RuntimeValue::Bytes(text.into_string().into_bytes()));
            }
            Instruction::Decode => {
                let bytes = pop_bytes(&mut stack, decoded.offset, "decode")?;
                let text = String::from_utf8(bytes).map_err(|_| {
                    BytecodeError::new(decoded.offset, "decode received invalid UTF-8")
                })?;
                ensure_text_limit(0, text.len(), decoded.offset)?;
                stack.push(RuntimeValue::Text(RuntimeText::new(text)));
            }
            Instruction::Number => {
                let text = pop_text(&mut stack, decoded.offset, "number")?;
                if !is_whole_literal(text.as_str()) {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "number requires one canonical Whole text value",
                    ));
                }
                let value = text.as_str().parse::<i64>().map_err(|_| {
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
                        RuntimeValue::Whole(value) => RuntimeText::new(value.to_string()),
                        RuntimeValue::Truth(true) => RuntimeText::new("bright".to_owned()),
                        RuntimeValue::Truth(false) => RuntimeText::new("dim".to_owned()),
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
                        | RuntimeValue::Buffer { .. }
                        | RuntimeValue::Table { .. } => {
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
                if called_function.is_task() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime forward cannot invoke a task-frame weave",
                    ));
                }
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
                    || called_function.is_task()
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
                #[cfg(test)]
                runtime_state.handle_live_resource_slots.push(
                    locals
                        .iter()
                        .enumerate()
                        .filter_map(|(slot, value)| {
                            value.as_ref().and_then(|value| {
                                matches!(
                                    value,
                                    RuntimeValue::Arena
                                        | RuntimeValue::Buffer { .. }
                                        | RuntimeValue::Table { .. }
                                )
                                .then_some(slot)
                            })
                        })
                        .collect(),
                );
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
                    RuntimeExit::Checkpoint(_) | RuntimeExit::TaskReturn { .. } => {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime handle cannot invoke a task-frame function",
                        ));
                    }
                }
                #[cfg(test)]
                runtime_state.handle_live_resource_slots.push(
                    locals
                        .iter()
                        .enumerate()
                        .filter_map(|(slot, value)| {
                            value.as_ref().and_then(|value| {
                                matches!(
                                    value,
                                    RuntimeValue::Arena
                                        | RuntimeValue::Buffer { .. }
                                        | RuntimeValue::Table { .. }
                                )
                                .then_some(slot)
                            })
                        })
                        .collect(),
                );
            }
            Instruction::TaskCheckpoint => {
                if artifact.version != ARTIFACT_VERSION_V12 || !function.is_task() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "AETH TASK_CHECKPOINT requires an AETH v12 task-frame function",
                    ));
                }
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "AETH TASK_CHECKPOINT requires an empty task operand stack",
                    ));
                }
                let lane = runtime_state.active_task_lane.take().ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "AETH v12 task checkpoint has no active private arena lane",
                    )
                })?;
                return Ok(RuntimeExit::Checkpoint(TaskFrame {
                    function_index,
                    locals,
                    stack,
                    position,
                    lane,
                }));
            }
            Instruction::NurseryBegin { count } => {
                if count == 0 || usize::from(count) > MAX_NURSERY_SPAWNS {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery begin count is invalid",
                    ));
                }
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery begin left values on the operand stack",
                    ));
                }
                if artifact.version == ARTIFACT_VERSION_V12 {
                    if !runtime_state.v12_nurseries.is_empty()
                        || !runtime_state.nurseries.is_empty()
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "AETH v12 cannot open a nested nursery region",
                        ));
                    }
                    runtime_state.v12_nurseries.push(V12NurseryFrame {
                        expected: count,
                        seen: 0,
                        children: Vec::with_capacity(usize::from(count)),
                    });
                    continue;
                }
                runtime_state.nurseries.push(NurseryFrame {
                    cancelled: false,
                    code: 0,
                    expected: count,
                    seen: 0,
                });
            }
            Instruction::NurserySpawn {
                function: called,
                arguments,
                destination,
            } => {
                if artifact.version == ARTIFACT_VERSION_V12 {
                    {
                        let Some(frame) = runtime_state.v12_nurseries.last_mut() else {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime v12 nursery spawn without an open nursery",
                            ));
                        };
                        if frame.seen >= frame.expected {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime v12 nursery spawn exceeds begin count",
                            ));
                        }
                        frame.seen = frame.seen.saturating_add(1);
                    }
                    let called_function = artifact.functions.get(called).ok_or_else(|| {
                        BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery spawn references an unknown weave",
                        )
                    })?;
                    if called_function.is_host()
                        || called_function.result != ValueType::Whole
                        || called_function.parameters.len() != arguments
                        || called_function.parameters.iter().any(|(value_type, mode)| {
                            *mode != ParameterMode::Own
                                || !matches!(value_type, ValueType::Whole | ValueType::Truth)
                        })
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery child must use the Copy-only Whole/Truth ABI",
                        ));
                    }
                    let mut values = Vec::with_capacity(arguments);
                    for (value_type, _) in called_function.parameters.iter().rev() {
                        let value = pop_runtime(&mut stack, decoded.offset, "v12 nursery spawn")?;
                        require_runtime_type(
                            &value,
                            *value_type,
                            decoded.offset,
                            "v12 nursery spawn",
                        )?;
                        values.push(value);
                    }
                    values.reverse();
                    let Some(frame) = runtime_state.v12_nurseries.last_mut() else {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery frame was lost during spawn",
                        ));
                    };
                    frame.children.push(V12NurseryChild {
                        function: called,
                        arguments: values,
                        destination,
                    });
                    continue;
                }
                let Some(frame) = runtime_state.nurseries.last_mut() else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery spawn without an open nursery",
                    ));
                };
                if frame.seen >= frame.expected {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery spawn exceeds begin count",
                    ));
                }
                frame.seen = frame.seen.saturating_add(1);
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime nursery spawn references an unknown weave",
                    )
                })?;
                if called_function.result != ValueType::Whole
                    || called_function.parameters.len() != arguments
                    || called_function.is_task()
                {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery spawn signature is invalid",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, mode) in called_function.parameters.iter().rev() {
                    let value = pop_runtime(&mut stack, decoded.offset, "nursery spawn")?;
                    if *mode != ParameterMode::Own
                        || matches!(
                            value_type,
                            ValueType::Arena
                                | ValueType::AccessArena
                                | ValueType::BufferWhole
                                | ValueType::BufferTruth
                                | ValueType::Table(_)
                        )
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime nursery spawn crosses a resource boundary",
                        ));
                    }
                    require_runtime_type(&value, *value_type, decoded.offset, "nursery spawn")?;
                    values.push(value);
                }
                values.reverse();
                let cancelled = runtime_state
                    .nurseries
                    .last()
                    .is_some_and(|frame| frame.cancelled);
                if cancelled {
                    // Drop arguments; destination stays unchanged.
                    continue;
                }
                match execute_function(artifact, called, values, stdout, runtime_state, depth + 1)?
                {
                    RuntimeExit::Return(value) => {
                        let local = function.locals.get(destination).ok_or_else(|| {
                            BytecodeError::new(
                                decoded.offset,
                                "runtime nursery spawn destination is invalid",
                            )
                        })?;
                        if !local.mutable || local.value_type != ValueType::Whole {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime nursery spawn destination is invalid",
                            ));
                        }
                        require_runtime_type(
                            &value,
                            ValueType::Whole,
                            decoded.offset,
                            "nursery spawn",
                        )?;
                        if locals.get(destination).is_none() || locals[destination].is_none() {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime nursery spawn destination is not live",
                            ));
                        }
                        locals[destination] = Some(value);
                    }
                    RuntimeExit::ErrorWhole(code) => {
                        if function.effect != Effect::ErrorWhole {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime nursery spawn raised in a total weave",
                            ));
                        }
                        let Some(frame) = runtime_state.nurseries.last_mut() else {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime nursery frame was lost during spawn",
                            ));
                        };
                        frame.cancelled = true;
                        frame.code = code;
                    }
                    RuntimeExit::Checkpoint(_) | RuntimeExit::TaskReturn { .. } => {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "legacy nursery execution cannot invoke a task-frame function",
                        ));
                    }
                }
            }
            Instruction::NurseryEnd => {
                if artifact.version == ARTIFACT_VERSION_V12 {
                    if !stack.is_empty() {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery end left values on the operand stack",
                        ));
                    }
                    let Some(frame) = runtime_state.v12_nurseries.pop() else {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery end without an open nursery",
                        ));
                    };
                    if frame.seen != frame.expected {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime v12 nursery end spawn count is incomplete",
                        ));
                    }
                    if let Some(code) = run_v12_nursery(
                        V12NurseryExecutionContext {
                            artifact,
                            parent: function,
                            parent_locals: &mut locals,
                            stdout,
                            runtime_state,
                            depth,
                            offset: decoded.offset,
                        },
                        frame.children,
                    )? {
                        if function.effect != Effect::ErrorWhole {
                            return Err(BytecodeError::new(
                                decoded.offset,
                                "runtime v12 nursery raised in a total weave",
                            ));
                        }
                        return Ok(RuntimeExit::ErrorWhole(code));
                    }
                    continue;
                }
                let Some(frame) = runtime_state.nurseries.pop() else {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery end without an open nursery",
                    ));
                };
                if frame.seen != frame.expected {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery end spawn count is incomplete",
                    ));
                }
                if !stack.is_empty() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime nursery end left values on the operand stack",
                    ));
                }
                if frame.cancelled {
                    if function.effect != Effect::ErrorWhole {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime cancelled nursery raised in a total weave",
                        ));
                    }
                    return Ok(RuntimeExit::ErrorWhole(frame.code));
                }
            }
            Instruction::Call {
                function: called,
                arguments,
            } => {
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(decoded.offset, "runtime call references an unknown weave")
                })?;
                if called_function.is_host() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime ordinary call cannot invoke a host weave",
                    ));
                }
                if called_function.is_task() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime ordinary call cannot invoke a task-frame weave",
                    ));
                }
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
            Instruction::HostCall {
                function: called,
                arguments,
            } => {
                let called_function = artifact.functions.get(called).ok_or_else(|| {
                    BytecodeError::new(
                        decoded.offset,
                        "runtime host call references an unknown weave",
                    )
                })?;
                if !called_function.is_host() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime HOST_CALL target is not a host weave",
                    ));
                }
                if called_function.parameters.len() != arguments {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime host call has an invalid argument count",
                    ));
                }
                let mut values = Vec::with_capacity(arguments);
                for (value_type, mode) in called_function.parameters.iter().rev() {
                    let value = pop_runtime(&mut stack, decoded.offset, "host call")?;
                    if *mode == ParameterMode::Access
                        || matches!(
                            value,
                            RuntimeValue::Record { .. }
                                | RuntimeValue::Arena
                                | RuntimeValue::AccessArena
                                | RuntimeValue::Buffer { .. }
                                | RuntimeValue::Table { .. }
                        )
                    {
                        return Err(BytecodeError::new(
                            decoded.offset,
                            "runtime host call cannot cross a resource or record boundary",
                        ));
                    }
                    require_runtime_type(&value, *value_type, decoded.offset, "host call")?;
                    values.push(value);
                }
                values.reverse();
                let value =
                    runtime_state
                        .hosts
                        .invoke(&called_function.name, &values, decoded.offset)?;
                if value.value_type() != called_function.result {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime host service returned a type that disagrees with its signature",
                    ));
                }
                stack.push(value);
            }
            Instruction::JumpIfDim(target) => {
                if !pop_truth(&mut stack, decoded.offset, "conditional jump")? {
                    position = target;
                }
            }
            Instruction::Jump(target) => position = target,
            Instruction::Release(slot) => {
                // Logical destruction: drop the runtime local so later uses fail closed.
                if slot >= locals.len() {
                    return Err(BytecodeError::new(
                        decoded.offset,
                        "runtime RELEASE references an invalid local slot",
                    ));
                }
                locals[slot] = None;
            }
        }
    }
    Err(BytecodeError::new(
        function.code.len(),
        "runtime reached the end of a weave without yield",
    ))
}

fn commit_v12_nursery_result(
    parent: &ArtifactFunction,
    locals: &mut [Option<RuntimeValue>],
    destination: usize,
    value: RuntimeValue,
    offset: usize,
) -> Result<(), BytecodeError> {
    let local = parent
        .locals
        .get(destination)
        .ok_or_else(|| BytecodeError::new(offset, "runtime v12 nursery destination is invalid"))?;
    if !local.mutable || local.value_type != ValueType::Whole {
        return Err(BytecodeError::new(
            offset,
            "runtime v12 nursery destination must be a mutable Whole root binding",
        ));
    }
    require_runtime_type(&value, ValueType::Whole, offset, "v12 nursery result")?;
    let slot = locals.get_mut(destination).ok_or_else(|| {
        BytecodeError::new(
            offset,
            "runtime v12 nursery destination is outside the parent frame",
        )
    })?;
    if slot.is_none() {
        return Err(BytecodeError::new(
            offset,
            "runtime v12 nursery destination is not live",
        ));
    }
    *slot = Some(value);
    Ok(())
}

/// End a completed or cancelled task frame without running guest code.  Local
/// slots are cleared in reverse declaration order before the private lane is
/// revoked and zeroed, matching the v12 destruction contract.
fn destroy_task_frame(
    mut frame: TaskFrame,
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(), BytecodeError> {
    if !frame.stack.is_empty() {
        return Err(BytecodeError::new(
            offset,
            "AETH v12 task destruction found a non-empty transient operand stack",
        ));
    }
    for (slot, local) in frame.locals.iter_mut().enumerate().rev() {
        // The slot is exposed only by the test-only destruction trace. Consume
        // it in release builds as well, preserving the same teardown loop.
        let _ = slot;
        if local.take().is_some() {
            #[cfg(test)]
            runtime_state.task_destroyed_local_slots.push(slot);
        }
    }
    runtime_state.zero_arena_range(frame.lane.start, frame.lane.capacity, offset)
}

fn cancel_v12_scheduler_children(
    children: &mut [V12SchedulerChild],
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(), BytecodeError> {
    for (index, child) in children.iter_mut().enumerate() {
        // The index is test-visible through the cfg(test) lifecycle trace.
        // Keep it consumed in release builds as well so the production crate
        // remains warning-free without changing cancellation behavior.
        let _ = index;
        let state = std::mem::replace(&mut child.state, V12SchedulerState::Cancelled);
        match state {
            V12SchedulerState::Parked(frame) => {
                #[cfg(test)]
                runtime_state
                    .task_frame_trace
                    .push(TaskFrameTraceEvent::Cancelled(index));
                destroy_task_frame(frame, runtime_state, offset)?;
            }
            V12SchedulerState::Completed | V12SchedulerState::Failed => {
                child.state = state;
            }
            V12SchedulerState::Pending(_)
            | V12SchedulerState::Cancelled
            | V12SchedulerState::Running => {
                // Pending arguments are Copy-only and drop here. `Running` is
                // reachable only while unwinding an invalid-runtime halt; it has
                // no concurrent guest execution in this single-thread VM.
            }
        }
    }
    Ok(())
}

/// Execute one fully captured AETH v12 nursery.  Task children run only until
/// their next checkpoint or yield; ordinary companions run to a terminal
/// result.  The source-order loop is deterministic and has no host scheduling
/// or parallel execution surface.
fn run_v12_nursery(
    context: V12NurseryExecutionContext<'_>,
    children: Vec<V12NurseryChild>,
) -> Result<Option<i64>, BytecodeError> {
    let V12NurseryExecutionContext {
        artifact,
        parent,
        parent_locals,
        stdout,
        runtime_state,
        depth,
        offset,
    } = context;
    let mut task_capacity = 0_usize;
    for child in &children {
        let function = artifact.functions.get(child.function).ok_or_else(|| {
            BytecodeError::new(
                offset,
                "runtime v12 nursery child references an unknown weave",
            )
        })?;
        if function.is_task() {
            task_capacity = task_capacity
                .checked_add(usize::try_from(function.frame_arena_capacity).map_err(|_| {
                    BytecodeError::new(offset, "task frame capacity is outside platform limits")
                })?)
                .ok_or_else(|| BytecodeError::new(offset, "task nursery lane sum overflowed"))?;
        }
    }

    // Admission is all-or-nothing and happens before the first task can run.
    let slab = if task_capacity == 0 {
        None
    } else {
        Some((
            runtime_state.reserve_task_slab(task_capacity, offset)?,
            task_capacity,
        ))
    };
    let mut next_lane = slab.map_or(0, |(start, _)| start);
    let mut scheduled = Vec::with_capacity(children.len());
    for child in children {
        let function = artifact.functions.get(child.function).ok_or_else(|| {
            BytecodeError::new(
                offset,
                "runtime v12 nursery child references an unknown weave",
            )
        })?;
        let lane = if function.is_task() {
            let capacity = usize::try_from(function.frame_arena_capacity).map_err(|_| {
                BytecodeError::new(offset, "task frame capacity is outside platform limits")
            })?;
            let start = next_lane;
            next_lane = next_lane
                .checked_add(capacity)
                .ok_or_else(|| BytecodeError::new(offset, "task nursery lane offset overflowed"))?;
            Some(TaskArenaLane {
                start,
                capacity,
                used: 0,
            })
        } else {
            None
        };
        scheduled.push(V12SchedulerChild {
            function: child.function,
            destination: child.destination,
            lane,
            state: V12SchedulerState::Pending(child.arguments),
        });
    }

    let scheduler_result = (|| -> Result<Option<i64>, BytecodeError> {
        let mut failure = None;
        while failure.is_none()
            && scheduled.iter().any(|child| {
                matches!(
                    child.state,
                    V12SchedulerState::Pending(_) | V12SchedulerState::Parked(_)
                )
            })
        {
            for (index, child) in scheduled.iter_mut().enumerate() {
                if failure.is_some() {
                    break;
                }
                // The source-order index is exposed only by test traces. Keep
                // the production dispatch loop warning-free without changing
                // the deterministic schedule.
                let _ = index;
                let state = std::mem::replace(&mut child.state, V12SchedulerState::Running);
                let (function_index, destination, lane) =
                    (child.function, child.destination, child.lane);
                let called = artifact.functions.get(function_index).ok_or_else(|| {
                    BytecodeError::new(
                        offset,
                        "runtime v12 nursery child references an unknown weave",
                    )
                })?;
                match state {
                    V12SchedulerState::Pending(arguments) => {
                        if called.is_task() {
                            let lane = lane.ok_or_else(|| {
                                BytecodeError::new(
                                    offset,
                                    "runtime v12 task child has no assigned private arena lane",
                                )
                            })?;
                            if runtime_state.active_task_lane.is_some()
                                || runtime_state.resumed_task.is_some()
                            {
                                return Err(BytecodeError::new(
                                    offset,
                                    "runtime v12 scheduler found an overlapping active task frame",
                                ));
                            }
                            runtime_state.active_task_lane = Some(lane);
                            #[cfg(test)]
                            runtime_state
                                .task_frame_trace
                                .push(TaskFrameTraceEvent::Started(index));
                            match execute_function(
                                artifact,
                                function_index,
                                arguments,
                                stdout,
                                runtime_state,
                                depth + 1,
                            )? {
                                RuntimeExit::Checkpoint(frame) => {
                                    #[cfg(test)]
                                    runtime_state
                                        .task_frame_trace
                                        .push(TaskFrameTraceEvent::Parked(index));
                                    child.state = V12SchedulerState::Parked(frame);
                                }
                                RuntimeExit::TaskReturn { value, frame } => {
                                    commit_v12_nursery_result(
                                        parent,
                                        parent_locals,
                                        destination,
                                        value,
                                        offset,
                                    )?;
                                    destroy_task_frame(frame, runtime_state, offset)?;
                                    #[cfg(test)]
                                    runtime_state
                                        .task_frame_trace
                                        .push(TaskFrameTraceEvent::Completed(index));
                                    child.state = V12SchedulerState::Completed;
                                }
                                RuntimeExit::Return(_) | RuntimeExit::ErrorWhole(_) => {
                                    return Err(BytecodeError::new(
                                        offset,
                                        "runtime v12 task child returned through an invalid exit path",
                                    ));
                                }
                            }
                        } else {
                            match execute_function(
                                artifact,
                                function_index,
                                arguments,
                                stdout,
                                runtime_state,
                                depth + 1,
                            )? {
                                RuntimeExit::Return(value) => {
                                    commit_v12_nursery_result(
                                        parent,
                                        parent_locals,
                                        destination,
                                        value,
                                        offset,
                                    )?;
                                    child.state = V12SchedulerState::Completed;
                                }
                                RuntimeExit::ErrorWhole(code) => {
                                    child.state = V12SchedulerState::Failed;
                                    failure = Some(code);
                                }
                                RuntimeExit::Checkpoint(_) | RuntimeExit::TaskReturn { .. } => {
                                    return Err(BytecodeError::new(
                                        offset,
                                        "runtime v12 companion returned through a task-frame exit path",
                                    ));
                                }
                            }
                        }
                    }
                    V12SchedulerState::Parked(frame) => {
                        if !called.is_task() {
                            return Err(BytecodeError::new(
                                offset,
                                "runtime v12 scheduler parked a non-task companion",
                            ));
                        }
                        if runtime_state.active_task_lane.is_some()
                            || runtime_state.resumed_task.is_some()
                        {
                            return Err(BytecodeError::new(
                                offset,
                                "runtime v12 scheduler found an overlapping resumed task frame",
                            ));
                        }
                        runtime_state.resumed_task = Some(frame);
                        match execute_function(
                            artifact,
                            function_index,
                            Vec::new(),
                            stdout,
                            runtime_state,
                            depth + 1,
                        )? {
                            RuntimeExit::Checkpoint(frame) => {
                                #[cfg(test)]
                                runtime_state
                                    .task_frame_trace
                                    .push(TaskFrameTraceEvent::Parked(index));
                                child.state = V12SchedulerState::Parked(frame);
                            }
                            RuntimeExit::TaskReturn { value, frame } => {
                                commit_v12_nursery_result(
                                    parent,
                                    parent_locals,
                                    destination,
                                    value,
                                    offset,
                                )?;
                                destroy_task_frame(frame, runtime_state, offset)?;
                                #[cfg(test)]
                                runtime_state
                                    .task_frame_trace
                                    .push(TaskFrameTraceEvent::Completed(index));
                                child.state = V12SchedulerState::Completed;
                            }
                            RuntimeExit::Return(_) | RuntimeExit::ErrorWhole(_) => {
                                return Err(BytecodeError::new(
                                    offset,
                                    "runtime resumed task returned through an invalid exit path",
                                ));
                            }
                        }
                    }
                    V12SchedulerState::Completed
                    | V12SchedulerState::Failed
                    | V12SchedulerState::Cancelled => {
                        child.state = state;
                    }
                    V12SchedulerState::Running => {
                        return Err(BytecodeError::new(
                            offset,
                            "runtime v12 scheduler found a stale running child state",
                        ));
                    }
                }
            }
        }
        if failure.is_some() {
            cancel_v12_scheduler_children(&mut scheduled, runtime_state, offset)?;
        }
        Ok(failure)
    })();

    // No task frame may survive the nursery boundary, even if malformed input
    // caused an internal runtime halt.  This keeps all private bytes inside the
    // slab and ensures future calls cannot inherit a stale execution context.
    let cleanup_result = cancel_v12_scheduler_children(&mut scheduled, runtime_state, offset);
    runtime_state.resumed_task = None;
    runtime_state.active_task_lane = None;
    let slab_result = if let Some((start, capacity)) = slab {
        let expected_end = start
            .checked_add(capacity)
            .ok_or_else(|| BytecodeError::new(offset, "task nursery slab range overflowed"))?;
        let invariant = if runtime_state.arena.used == expected_end {
            Ok(())
        } else {
            Err(BytecodeError::new(
                offset,
                "runtime v12 task nursery lost LIFO ownership of its private slab",
            ))
        };
        let zero = runtime_state.zero_arena_range(start, capacity, offset);
        runtime_state.arena.used = start;
        invariant.and(zero)
    } else {
        Ok(())
    };

    let scheduler_outcome = scheduler_result?;
    cleanup_result?;
    slab_result?;
    Ok(scheduler_outcome)
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
                    | ARTIFACT_VERSION_V9
                    | ARTIFACT_VERSION_V10
                    | ARTIFACT_VERSION_V11
                    | ARTIFACT_VERSION_V12
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record construction is valid only in AETH v5 through v12 artifacts",
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
                    | ARTIFACT_VERSION_V9
                    | ARTIFACT_VERSION_V10
                    | ARTIFACT_VERSION_V11
                    | ARTIFACT_VERSION_V12
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record field projection is valid only in AETH v5 through v12 artifacts",
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
        OP_TABLE => {
            require_v9_instruction(version, offset, "table placeholder")?;
            Instruction::Table {
                shape: read_u16(code, position)?,
                layout: TableLayout::from_tag(read_byte(code, position)?, offset)?,
            }
        }
        OP_TABLE_ALLOCATE => {
            require_v9_instruction(version, offset, "table allocation")?;
            Instruction::TableAllocate {
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_TABLE_STORE => {
            require_v9_instruction(version, offset, "table store")?;
            Instruction::TableStore {
                shape: read_u16(code, position)?,
                field: read_byte(code, position)?,
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_TABLE_LOAD => {
            require_v9_instruction(version, offset, "table load")?;
            Instruction::TableLoad {
                shape: read_u16(code, position)?,
                field: read_byte(code, position)?,
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_TABLE_COUNT => {
            require_v9_instruction(version, offset, "table count")?;
            Instruction::TableCount
        }
        OP_NURSERY_BEGIN => {
            require_v10_instruction(version, offset, "nursery begin")?;
            Instruction::NurseryBegin {
                count: read_byte(code, position)?,
            }
        }
        OP_NURSERY_SPAWN => {
            require_v10_instruction(version, offset, "nursery spawn")?;
            Instruction::NurserySpawn {
                function: usize::from(read_u16(code, position)?),
                arguments: usize::from(read_byte(code, position)?),
                destination: usize::from(read_u16(code, position)?),
            }
        }
        OP_NURSERY_END => {
            require_v10_instruction(version, offset, "nursery end")?;
            Instruction::NurseryEnd
        }
        OP_TASK_CHECKPOINT => {
            if version != ARTIFACT_VERSION_V12 {
                return Err(BytecodeError::new(
                    offset,
                    "task checkpoint is valid only in AETH v12 artifacts",
                ));
            }
            Instruction::TaskCheckpoint
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
        OP_HOST_CALL => {
            require_v11_instruction(version, offset, "host call")?;
            Instruction::HostCall {
                function: usize::from(read_u16(code, position)?),
                arguments: usize::from(read_byte(code, position)?),
            }
        }
        OP_RELEASE => {
            require_v11_instruction(version, offset, "release")?;
            Instruction::Release(usize::from(read_u16(code, position)?))
        }
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
        ARTIFACT_VERSION_V6
            | ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v6 through v12 artifacts"),
        ))
    }
}

fn require_v7_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(
        version,
        ARTIFACT_VERSION_V7
            | ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v7 through v12 artifacts"),
        ))
    }
}

fn require_v8_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(
        version,
        ARTIFACT_VERSION_V8
            | ARTIFACT_VERSION_V9
            | ARTIFACT_VERSION_V10
            | ARTIFACT_VERSION_V11
            | ARTIFACT_VERSION_V12
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v8 through v12 artifacts"),
        ))
    }
}

fn require_v9_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(
        version,
        ARTIFACT_VERSION_V9 | ARTIFACT_VERSION_V10 | ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v9 through v12 artifacts"),
        ))
    }
}

fn require_v10_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(
        version,
        ARTIFACT_VERSION_V10 | ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v10 through v12 artifacts"),
        ))
    }
}

fn require_v11_instruction(version: u8, offset: usize, subject: &str) -> Result<(), BytecodeError> {
    if matches!(version, ARTIFACT_VERSION_V11 | ARTIFACT_VERSION_V12) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{subject} is valid only in AETH v11 or v12 artifacts"),
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
    if value.resource_provenance == ResourceProvenance::MovedLocal(destination) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} must receive the Buffer owner moved from its destination slot"),
        ))
    }
}

fn require_moved_resource_from(
    value: &VerificationStackValue,
    destination: usize,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if value.resource_provenance == ResourceProvenance::MovedLocal(destination) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} must receive the resource owner moved from its destination slot"),
        ))
    }
}

fn require_moved_buffer(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if matches!(value.resource_provenance, ResourceProvenance::MovedLocal(_)) {
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
    if value.resource_provenance == ResourceProvenance::Borrowed {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires a transient borrowed Buffer value"),
        ))
    }
}

fn require_borrowed_resource(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if value.resource_provenance == ResourceProvenance::Borrowed {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} requires a transient borrowed resource value"),
        ))
    }
}

fn require_storable_buffer(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if matches!(
        value.resource_provenance,
        ResourceProvenance::Placeholder | ResourceProvenance::MovedLocal(_)
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} cannot persist a borrowed Buffer value"),
        ))
    }
}

fn require_storable_resource(
    value: &VerificationStackValue,
    offset: usize,
    operation: &str,
) -> Result<(), BytecodeError> {
    if matches!(
        value.resource_provenance,
        ResourceProvenance::Placeholder | ResourceProvenance::MovedLocal(_)
    ) {
        Ok(())
    } else {
        Err(BytecodeError::new(
            offset,
            format!("{operation} cannot persist a borrowed resource value"),
        ))
    }
}

fn require_yieldable_buffer(
    value: &VerificationStackValue,
    offset: usize,
) -> Result<(), BytecodeError> {
    if matches!(value.resource_provenance, ResourceProvenance::MovedLocal(_)) {
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
) -> Result<RuntimeText, BytecodeError> {
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
    let Some(allocation_start) = runtime_state.reserve_arena_bytes(required) else {
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
    let offset = allocation_start
        .checked_add(BUFFER_METADATA_BYTES)
        .ok_or_else(|| BytecodeError::new(offset, "buffer data offset overflowed"))?;
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

fn runtime_allocate_table(
    table: RuntimeValue,
    capacity: i64,
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Table {
        shape,
        layout,
        field_count,
        ..
    } = table
    else {
        return Err(BytecodeError::new(
            offset,
            "table allocate received a non-table value",
        ));
    };
    if !(1..=MAX_TABLE_CAPACITY).contains(&capacity) {
        return Ok((
            RuntimeValue::Table {
                shape,
                layout,
                field_count,
                allocated: false,
                offset: 0,
                capacity: 0,
            },
            false,
        ));
    }
    let count = usize::try_from(capacity)
        .map_err(|_| BytecodeError::new(offset, "table capacity is outside platform limits"))?;
    let fields = usize::from(field_count);
    let cells = count
        .checked_mul(fields)
        .ok_or_else(|| BytecodeError::new(offset, "table cell count overflowed"))?;
    let payload = cells
        .checked_mul(std::mem::size_of::<i64>())
        .ok_or_else(|| BytecodeError::new(offset, "table payload size overflowed"))?;
    let required = match TABLE_METADATA_BYTES.checked_add(payload) {
        Some(value) => value,
        None => {
            return Ok((
                RuntimeValue::Table {
                    shape,
                    layout,
                    field_count,
                    allocated: false,
                    offset: 0,
                    capacity: 0,
                },
                false,
            ))
        }
    };
    let Some(allocation_start) = runtime_state.reserve_arena_bytes(required) else {
        return Ok((
            RuntimeValue::Table {
                shape,
                layout,
                field_count,
                allocated: false,
                offset: 0,
                capacity: 0,
            },
            false,
        ));
    };
    let data_offset = allocation_start
        .checked_add(TABLE_METADATA_BYTES)
        .ok_or_else(|| BytecodeError::new(offset, "table data offset overflowed"))?;
    Ok((
        RuntimeValue::Table {
            shape,
            layout,
            field_count,
            allocated: true,
            offset: data_offset,
            capacity: count,
        },
        true,
    ))
}

fn table_cell_offset(
    layout: TableLayout,
    capacity: usize,
    field_count: usize,
    index: usize,
    field: usize,
    offset: usize,
) -> Result<usize, BytecodeError> {
    let cell = match layout {
        TableLayout::Rows => index
            .checked_mul(field_count)
            .and_then(|base| base.checked_add(field)),
        TableLayout::Columns => field
            .checked_mul(capacity)
            .and_then(|base| base.checked_add(index)),
    }
    .ok_or_else(|| BytecodeError::new(offset, "table cell address overflowed"))?;
    cell.checked_mul(std::mem::size_of::<i64>())
        .ok_or_else(|| BytecodeError::new(offset, "table cell byte offset overflowed"))
}

fn runtime_table_store(
    table: RuntimeValue,
    shape: u16,
    field: u8,
    index: i64,
    value: i64,
    runtime_state: &mut RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Table {
        shape: table_shape,
        layout,
        field_count,
        allocated,
        offset: data_offset,
        capacity,
    } = table
    else {
        return Err(BytecodeError::new(
            offset,
            "table store received a non-table value",
        ));
    };
    if table_shape != shape {
        return Err(BytecodeError::new(
            offset,
            "table store shape does not match the owner",
        ));
    }
    if !allocated {
        return Ok((
            RuntimeValue::Table {
                shape: table_shape,
                layout,
                field_count,
                allocated,
                offset: data_offset,
                capacity,
            },
            false,
        ));
    }
    let Ok(index) = usize::try_from(index) else {
        return Ok((
            RuntimeValue::Table {
                shape: table_shape,
                layout,
                field_count,
                allocated,
                offset: data_offset,
                capacity,
            },
            false,
        ));
    };
    let field = usize::from(field);
    if index >= capacity || field >= usize::from(field_count) {
        return Ok((
            RuntimeValue::Table {
                shape: table_shape,
                layout,
                field_count,
                allocated,
                offset: data_offset,
                capacity,
            },
            false,
        ));
    }
    let cell_offset = table_cell_offset(
        layout,
        capacity,
        usize::from(field_count),
        index,
        field,
        offset,
    )?;
    let write_offset = data_offset
        .checked_add(cell_offset)
        .ok_or_else(|| BytecodeError::new(offset, "table store offset overflowed"))?;
    let end = write_offset
        .checked_add(std::mem::size_of::<i64>())
        .ok_or_else(|| BytecodeError::new(offset, "table store range overflowed"))?;
    let bytes = runtime_state
        .arena
        .bytes
        .get_mut(write_offset..end)
        .ok_or_else(|| {
            BytecodeError::new(offset, "table store range is outside the reserved arena")
        })?;
    bytes.copy_from_slice(&value.to_le_bytes());
    Ok((
        RuntimeValue::Table {
            shape: table_shape,
            layout,
            field_count,
            allocated: true,
            offset: data_offset,
            capacity,
        },
        true,
    ))
}

fn runtime_table_load(
    table: RuntimeValue,
    shape: u16,
    field: u8,
    index: i64,
    fallback: RuntimeValue,
    runtime_state: &RuntimeState,
    offset: usize,
) -> Result<(RuntimeValue, bool), BytecodeError> {
    let RuntimeValue::Table {
        shape: table_shape,
        layout,
        field_count,
        allocated,
        offset: data_offset,
        capacity,
    } = table
    else {
        return Err(BytecodeError::new(
            offset,
            "table load received a non-table value",
        ));
    };
    if table_shape != shape {
        return Err(BytecodeError::new(
            offset,
            "table load shape does not match the owner",
        ));
    }
    let Ok(index) = usize::try_from(index) else {
        return Ok((fallback, false));
    };
    let field = usize::from(field);
    if !allocated || index >= capacity || field >= usize::from(field_count) {
        return Ok((fallback, false));
    }
    let cell_offset = table_cell_offset(
        layout,
        capacity,
        usize::from(field_count),
        index,
        field,
        offset,
    )?;
    let read_offset = data_offset
        .checked_add(cell_offset)
        .ok_or_else(|| BytecodeError::new(offset, "table load offset overflowed"))?;
    let end = read_offset
        .checked_add(std::mem::size_of::<i64>())
        .ok_or_else(|| BytecodeError::new(offset, "table load range overflowed"))?;
    let bytes = runtime_state
        .arena
        .bytes
        .get(read_offset..end)
        .ok_or_else(|| {
            BytecodeError::new(offset, "table load range is outside the reserved arena")
        })?;
    let mut data = [0_u8; 8];
    data.copy_from_slice(bytes);
    Ok((RuntimeValue::Whole(i64::from_le_bytes(data)), true))
}

fn runtime_values_equal(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match (left, right) {
        (RuntimeValue::Text(left), RuntimeValue::Text(right)) => left.as_str() == right.as_str(),
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
        (
            RuntimeValue::Table {
                shape: left_shape,
                layout: left_layout,
                field_count: left_fields,
                allocated: left_allocated,
                offset: left_offset,
                capacity: left_capacity,
            },
            RuntimeValue::Table {
                shape: right_shape,
                layout: right_layout,
                field_count: right_fields,
                allocated: right_allocated,
                offset: right_offset,
                capacity: right_capacity,
            },
        ) => {
            left_shape == right_shape
                && left_layout == right_layout
                && left_fields == right_fields
                && left_allocated == right_allocated
                && left_offset == right_offset
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
        RuntimeValue::Text(value) => Ok(value.byte_len()),
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
        RuntimeValue::Arena
        | RuntimeValue::AccessArena
        | RuntimeValue::Buffer { .. }
        | RuntimeValue::Table { .. } => Err(BytecodeError::new(
            offset,
            "Aether resource values cannot be measured as record payloads",
        )),
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
    shape_count: usize,
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
                    | ARTIFACT_VERSION_V9
                    | ARTIFACT_VERSION_V10
                    | ARTIFACT_VERSION_V11
                    | ARTIFACT_VERSION_V12
            ) {
                return Err(BytecodeError::new(
                    offset,
                    "record types are valid only in AETH v5 through v12 artifacts",
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
            ARTIFACT_VERSION_V6
                | ARTIFACT_VERSION_V7
                | ARTIFACT_VERSION_V8
                | ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) =>
        {
            Ok(ValueType::Arena)
        }
        7 if matches!(
            version,
            ARTIFACT_VERSION_V6
                | ARTIFACT_VERSION_V7
                | ARTIFACT_VERSION_V8
                | ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) =>
        {
            Ok(ValueType::BufferWhole)
        }
        8 if matches!(
            version,
            ARTIFACT_VERSION_V6
                | ARTIFACT_VERSION_V7
                | ARTIFACT_VERSION_V8
                | ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) =>
        {
            Ok(ValueType::BufferTruth)
        }
        9 => Err(BytecodeError::new(
            offset,
            "access loans are verifier-internal and cannot be serialized",
        )),
        10 if matches!(
            version,
            ARTIFACT_VERSION_V9
                | ARTIFACT_VERSION_V10
                | ARTIFACT_VERSION_V11
                | ARTIFACT_VERSION_V12
        ) =>
        {
            let shape_id = read_u16(bytes, position)?;
            if usize::from(shape_id) >= shape_count {
                return Err(BytecodeError::new(
                    offset,
                    "table shape identifier is outside the artifact shape table",
                ));
            }
            Ok(ValueType::Table(shape_id))
        }
        6..=8 | 10 => Err(BytecodeError::new(
            offset,
            "resource value types are valid only in supported AETH versions",
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
    match value_type {
        ValueType::Record(record_id) | ValueType::Table(record_id) => {
            write_u16(bytes, record_id);
        }
        _ => {}
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
            | ValueType::Table(_)
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
            Statement::Release { name, .. } => {
                output.push_str("release ");
                output.push_str(name);
                output.push('\n');
            }
            Statement::Checkpoint { .. } => {
                output.push_str("checkpoint\n");
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
            Statement::Together { spawns, .. } => {
                output.push_str("together:\n");
                for spawn in spawns {
                    output.push_str(&"  ".repeat(indentation + 1));
                    output.push_str("spawn call ");
                    output.push_str(&spawn.weave);
                    for argument in &spawn.arguments {
                        output.push(' ');
                        write_atom(argument, output);
                    }
                    output.push_str(" into ");
                    output.push_str(&spawn.destination);
                    output.push('\n');
                }
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
        ExpressionKind::Table { shape, layout } => {
            output.push_str("table ");
            output.push_str(shape);
            output.push_str(" layout ");
            output.push_str(layout.word());
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
        ResourceOperation::Store {
            table,
            index,
            field,
            value,
            destination,
        } => {
            output.push_str("store ");
            write_atom(table, output);
            output.push(' ');
            write_atom(index, output);
            output.push(' ');
            output.push_str(field);
            output.push(' ');
            write_atom(value, output);
            output.push_str(" into ");
            output.push_str(destination);
        }
        ResourceOperation::Load {
            table,
            index,
            field,
            destination,
        } => {
            output.push_str("load ");
            write_atom(table, output);
            output.push(' ');
            write_atom(index, output);
            output.push(' ');
            output.push_str(field);
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
            Statement::Release { name, .. } => {
                output.push_str("Release(");
                output.push_str(name);
                output.push(')');
            }
            Statement::Checkpoint { .. } => output.push_str("Checkpoint"),
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
            Statement::Together { spawns, .. } => {
                output.push_str("Together(");
                for (index, spawn) in spawns.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    output.push_str("Spawn(");
                    output.push_str(&spawn.weave);
                    output.push(',');
                    output.push_str(&spawn.destination);
                    for argument in &spawn.arguments {
                        output.push(',');
                        write_ast_atom(argument, output);
                    }
                    output.push(')');
                }
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
        ExpressionKind::Table { shape, layout } => {
            output.push_str("Table(");
            output.push_str(shape);
            output.push(',');
            output.push_str(layout.word());
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
                ResourceOperation::Store {
                    table,
                    index,
                    field,
                    value,
                    destination,
                } => {
                    output.push_str("store,");
                    write_ast_atom(table, output);
                    output.push(',');
                    write_ast_atom(index, output);
                    output.push(',');
                    output.push_str(field);
                    output.push(',');
                    write_ast_atom(value, output);
                    output.push(',');
                    output.push_str(destination);
                }
                ResourceOperation::Load {
                    table,
                    index,
                    field,
                    destination,
                } => {
                    output.push_str("load,");
                    write_ast_atom(table, output);
                    output.push(',');
                    write_ast_atom(index, output);
                    output.push(',');
                    output.push_str(field);
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
            | Statement::Release { .. }
            | Statement::Yield { .. } => 2,
            Statement::Checkpoint { .. } => 1,
            Statement::Raise { .. } | Statement::Forward { .. } => 2,
            Statement::Handle { .. } => 4,
            Statement::Choose {
                when_bright,
                when_dim,
                ..
            } => 2 + statement_token_count(when_bright) + statement_token_count(when_dim),
            Statement::While { body, .. } => 2 + statement_token_count(body),
            Statement::Together { spawns, .. } => 1 + spawns.len() * 3,
        })
        .sum()
}

fn validate_call_target(name: &str, span: Span) -> Result<(), CompilerError> {
    if let Some((alias, weave)) = name.split_once('.') {
        if alias.contains('.') || weave.contains('.') {
            return Err(CompilerError::new(
                span,
                "AE-MOD-001: qualified call must use alias.weave with a single dot",
            ));
        }
        validate_name(alias, span, "import alias", false)?;
        validate_name(weave, span, "called weave name", true)?;
        return Err(CompilerError::new(
            span,
            "AE-MOD-007: qualified call alias.weave requires aether project build (multi-module); single-file compile rejects qualified imports",
        ));
    }
    validate_name(name, span, "called weave name", true)
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

    /// Locate v12 descriptor metadata for a deliberately primitive-only test
    /// fixture. Keeping this tiny decoder in the test module lets hostile
    /// artifact tests mutate one invariant at a time without exposing parser
    /// offsets through the product API.
    fn v12_primitive_function_offsets(bytecode: &[u8], expected_name: &str) -> (usize, usize) {
        assert_eq!(&bytecode[..5], b"AETH\x0c");
        let mut position = 9;
        let records = u16::from_le_bytes([bytecode[position], bytecode[position + 1]]);
        assert_eq!(records, 0, "fixture must remain record-free");
        position += 2;
        let shapes = u16::from_le_bytes([bytecode[position], bytecode[position + 1]]);
        assert_eq!(shapes, 0, "fixture must remain shape-free");
        position += 2;
        let function_count = usize::from(u16::from_le_bytes([
            bytecode[position],
            bytecode[position + 1],
        ]));
        position += 2;
        for _ in 0..function_count {
            let name_length = usize::from(bytecode[position]);
            position += 1;
            let name = std::str::from_utf8(&bytecode[position..position + name_length])
                .expect("test fixture function names must be ASCII");
            position += name_length;
            let parameter_count = usize::from(bytecode[position]);
            position += 1;
            // Every parameter in this fixture is one primitive value-type byte
            // plus one parameter-mode byte.
            position += parameter_count * 2;
            // Primitive result, effect, and guest/host kind.
            position += 3;
            let flags_offset = position;
            position += 1;
            // Frame arena capacity.
            position += 4;
            let local_count = usize::from(u16::from_le_bytes([
                bytecode[position],
                bytecode[position + 1],
            ]));
            position += 2;
            // Every local in this fixture is one primitive value-type byte plus
            // one mutability byte.
            position += local_count * 2;
            let code_length = usize::try_from(u32::from_le_bytes([
                bytecode[position],
                bytecode[position + 1],
                bytecode[position + 2],
                bytecode[position + 3],
            ]))
            .expect("test fixture code length must fit usize");
            position += 4;
            let code_offset = position;
            position += code_length;
            if name == expected_name {
                return (flags_offset, code_offset);
            }
        }
        panic!("test fixture has no function named {expected_name}");
    }

    #[test]
    fn compiles_runs_and_formats_legacy_source_in_current_aeth_v9() {
        let output = compile_to_bytecode(HELLO).expect("Aether source should compile");
        let run = run_bytecode(&output.bytecode).expect("Aether artifact should run");
        assert_eq!(run.stdout, "Hello from Aether\n");
        assert_eq!(run.exit_code, 0);
        assert_eq!(format_program(&output.program), HELLO);
        assert!(canonical_ast(&output.program).contains("Borrow(greeting)"));
        assert_eq!(&output.bytecode[..5], b"AETH\x0b");
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
        let source = "world unicode\n\nweave main [] -> Whole:\n  bind source <- \"Aé🙂Z\"\n  bind section <- cut borrow source 1 3\n  bind count <- measure borrow source\n  bind code <- glyph borrow source 2\n  bind found <- seek borrow source \"🙂\" 1\n  bind mutable result <- -1\n  choose same count 4:\n    revise result <- sum code found\n  speak borrow section\n  yield result\n";
        let output = compile_to_bytecode(source).expect("Unicode source should compile");
        let run = run_bytecode(&output.bytecode).expect("Unicode artifact should run");
        assert_eq!(run.stdout, "é🙂");
        assert_eq!(run.exit_code, 128_580);

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
    fn runtime_text_preserves_ascii_fast_and_unicode_scalar_offsets() {
        let ascii = RuntimeText::new("aether".to_owned());
        assert!(
            ascii.ascii,
            "ASCII text must retain the cached fast-path fact"
        );
        assert_eq!(ascii.scalar_len(), 6);
        assert_eq!(ascii.scalar_at(2), Some('t'));
        assert_eq!(
            ascii.find_from_scalar(&RuntimeText::new("th".to_owned()), 1),
            2
        );
        assert_eq!(
            ascii
                .slice_scalars(1, 4)
                .expect("ASCII scalar range should be valid")
                .as_str(),
            "eth"
        );

        let unicode = RuntimeText::new("Aé🙂Z".to_owned());
        assert!(
            !unicode.ascii,
            "Unicode text must retain scalar traversal semantics"
        );
        assert_eq!(unicode.scalar_len(), 4);
        assert_eq!(unicode.scalar_at(2), Some('🙂'));
        assert_eq!(
            unicode.find_from_scalar(&RuntimeText::new("Z".to_owned()), 1),
            3
        );
        assert_eq!(
            unicode
                .slice_scalars(1, 3)
                .expect("Unicode scalar range should be valid")
                .as_str(),
            "é🙂"
        );
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
        assert_eq!(&first.bytecode[..5], b"AETH\x0b");
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
        // Translate a current v11 artifact into a v5-compatible payload:
        // drop arena capacity, empty shape table, effect byte, and host kind byte.
        artifact[4] = ARTIFACT_VERSION_V5;
        artifact.drain(5..9).for_each(drop);
        let record_bytes = 2 + 1 + "card".len() + 1 + 1 + "score".len() + 1;
        let shape_count_offset = 5 + record_bytes;
        assert_eq!(
            u16::from_le_bytes([
                artifact[shape_count_offset],
                artifact[shape_count_offset + 1]
            ]),
            0,
            "fixture should have an empty shape table"
        );
        artifact.drain(shape_count_offset..shape_count_offset + 2);
        let effect_offset = shape_count_offset + 2 + 1 + "main".len() + 1 + 1;
        assert_eq!(artifact[effect_offset], 0, "fixture main must be total");
        artifact.remove(effect_offset);
        assert_eq!(
            artifact[effect_offset], FUNCTION_KIND_GUEST,
            "fixture main must be a guest weave"
        );
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
    fn compiles_runs_and_formats_immutable_records_in_current_aeth_v9() {
        let source = "world records\n\nrecord card [label: Text, score: Whole, payload: Bytes, active: Truth]\n\nweave inspect [borrow value: card] -> Whole:\n  bind score <- field borrow value score\n  yield score\n\nweave main [] -> Whole:\n  bind card_value <- make card \"Aether\" 7 bytes \"0102\" bright\n  bind label <- field borrow card_value label\n  speak borrow label\n  bind score <- call inspect borrow card_value\n  bind duplicate <- make card \"Aether\" 7 bytes \"0102\" bright\n  bind equal <- same borrow card_value borrow duplicate\n  bind mutable result <- score\n  choose equal:\n    revise result <- sum result 1\n  yield result\n";
        let output = compile_to_bytecode(source).expect("record source should compile");
        let run = run_bytecode(&output.bytecode).expect("record artifact should run");
        assert_eq!(run.stdout, "Aether");
        assert_eq!(run.exit_code, 8);
        assert_eq!(&output.bytecode[..5], b"AETH\x0b");
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
        assert_eq!(&output.bytecode[..5], b"AETH\x0b");
        assert_eq!(
            u32::from_le_bytes(
                output.bytecode[5..9]
                    .try_into()
                    .expect("v10 capacity bytes")
            ),
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

        // M19d: total helpers may declare self-owned arenas.
        let helper_arena = "world helper_arena\n\nweave helper [] -> Whole:\n  bind memory <- arena 64\n  yield 0\n\nweave main [] -> Whole:\n  yield call helper\n";
        let helper = compile_to_bytecode(helper_arena).expect("total helper arena is legal");
        assert_eq!(
            run_bytecode(&helper.bytecode)
                .expect("helper arena program runs")
                .exit_code,
            0
        );

        let two_arenas_one_weave = "world invalid\n\nweave main [] -> Whole:\n  bind a <- arena 8\n  bind b <- arena 8\n  yield 0\n";
        let error = compile_source(two_arenas_one_weave).expect_err("at most one arena per weave");
        assert!(
            error.message.contains("one arena") || error.diagnostic().code == "AE-RESOURCE-001",
            "{error}"
        );

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
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V11);
        assert!(
            output.bytecode.contains(&OP_COMPTIME_WHOLE),
            "M5 artifacts must retain compile-time provenance in AETH v8 through v11"
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
    fn compiles_verifies_and_runs_m15_comptime_name_chaining() {
        let source = include_str!("../../../examples/comptime-chain.ae");
        let output = compile_to_bytecode(source).expect("M15 chain should compile");
        assert!(
            output.bytecode.contains(&OP_COMPTIME_WHOLE),
            "M15 chain must emit COMPTIME_WHOLE"
        );
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );
        verify_bytecode(&output.bytecode).expect("M15 chain should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M15 chain should run")
                .exit_code,
            288,
            "cell=64 row=256 header=32 total=288"
        );

        let mixed = "world mixed\n\nweave main [] -> Whole:\n  comptime bind base <- sum 10 5\n  comptime bind next <- product base 2\n  yield next\n";
        let mixed_out = compile_to_bytecode(mixed).expect("mixed operands compile");
        assert_eq!(
            run_bytecode(&mixed_out.bytecode)
                .expect("mixed run")
                .exit_code,
            30
        );
    }

    #[test]
    fn compiles_verifies_runs_and_seed_matches_m23_pure_comptime_calls() {
        let source = include_str!("../../../examples/comptime-calls.ae");
        let bootstrap = compile_to_bytecode(source).expect("M23 source should bootstrap compile");
        assert_eq!(
            format_program(&bootstrap.program),
            source.replace("\r\n", "\n")
        );
        assert!(
            bootstrap.bytecode.contains(&OP_COMPTIME_WHOLE),
            "M23 calls must still lower to COMPTIME_WHOLE"
        );
        verify_bytecode(&bootstrap.bytecode).expect("M23 bootstrap artifact should verify");
        assert_eq!(
            run_bytecode(&bootstrap.bytecode)
                .expect("M23 bootstrap artifact should run")
                .exit_code,
            512,
            "cell=64, wide=128, total=512"
        );

        let seeded = compile_with_seed(source).expect("M23 source should seed compile");
        assert_eq!(
            seeded.bytecode, bootstrap.bytecode,
            "M23 seed-native product path must match bootstrap byte-for-byte"
        );
        assert_eq!(
            run_bytecode(&seeded.bytecode)
                .expect("M23 seed artifact should run")
                .exit_code,
            512
        );
    }

    #[test]
    fn rejects_m23_ineligible_calls_without_expanding_comptime_authority() {
        let cases = [
            (
                "host target",
                "world invalid\n\nhost weave host_inc [value: Whole] -> Whole\n\nweave main [] -> Whole:\n  comptime bind result <- call host_inc 1\n  yield result\n",
                "AE-COMPTIME-001",
                "prior total guest weave",
            ),
            (
                "foreign target",
                "world invalid\n\nforeign weave foreign_sum [left: Whole, right: Whole] -> Whole from \"pilot\" symbol \"aether_foreign_sum\"\n\nweave main [] -> Whole:\n  comptime bind result <- call foreign_sum 1 2\n  yield result\n",
                "AE-COMPTIME-001",
                "prior total guest weave",
            ),
            (
                "erroring target",
                "world invalid\n\nweave boom [value: Whole] -> Whole raises Whole:\n  raise value\n\nweave main [] -> Whole:\n  comptime bind result <- call boom 1\n  yield result\n",
                "AE-COMPTIME-001",
                "total guest weave returning Whole",
            ),
            (
                "nested call in callee",
                "world invalid\n\nweave leaf [value: Whole] -> Whole:\n  yield sum value 1\n\nweave bad [value: Whole] -> Whole:\n  yield call leaf value\n\nweave main [] -> Whole:\n  comptime bind result <- call bad 1\n  yield result\n",
                "AE-COMPTIME-001",
                "nested call",
            ),
            (
                "choose in callee",
                "world invalid\n\nweave branch [value: Whole] -> Whole:\n  bind mutable result <- 0\n  choose bright:\n    revise result <- value\n  otherwise:\n    revise result <- 0\n  yield result\n\nweave main [] -> Whole:\n  comptime bind result <- call branch 1\n  yield result\n",
                "AE-COMPTIME-001",
                "permits only Whole bind, revise, and terminal yield",
            ),
            (
                "while in callee",
                "world invalid\n\nweave loop [value: Whole] -> Whole:\n  bind mutable result <- value\n  while dim:\n    revise result <- 0\n  yield result\n\nweave main [] -> Whole:\n  comptime bind result <- call loop 1\n  yield result\n",
                "AE-COMPTIME-001",
                "permits only Whole bind, revise, and terminal yield",
            ),
            (
                "runtime argument",
                "world invalid\n\nweave double [value: Whole] -> Whole:\n  yield product value 2\n\nweave main [] -> Whole:\n  bind runtime_value <- 21\n  comptime bind result <- call double runtime_value\n  yield result\n",
                "AE-COMPTIME-001",
                "not a prior root-level comptime Whole binding",
            ),
            (
                "forward target",
                "world invalid\n\nweave main [] -> Whole:\n  comptime bind result <- call double 21\n  yield result\n\nweave double [value: Whole] -> Whole:\n  yield product value 2\n",
                "AE-COMPTIME-001",
                "prior total guest weave",
            ),
            (
                "non Whole parameter",
                "world invalid\n\nweave text_value [value: Text] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  comptime bind result <- call text_value 1\n  yield result\n",
                "AE-COMPTIME-001",
                "owned Whole parameters only",
            ),
            (
                "non Whole result",
                "world invalid\n\nweave truth_value [value: Whole] -> Truth:\n  yield bright\n\nweave main [] -> Whole:\n  comptime bind result <- call truth_value 1\n  yield result\n",
                "AE-COMPTIME-001",
                "total guest weave returning Whole",
            ),
            (
                "argument count",
                "world invalid\n\nweave area [width: Whole, height: Whole] -> Whole:\n  yield product width height\n\nweave main [] -> Whole:\n  comptime bind result <- call area 2\n  yield result\n",
                "AE-COMPTIME-001",
                "expects 2 Whole arguments, received 1",
            ),
            (
                "overflow in callee",
                "world invalid\n\nweave increment [value: Whole] -> Whole:\n  yield sum value 1\n\nweave main [] -> Whole:\n  comptime bind result <- call increment 9223372036854775807\n  yield result\n",
                "AE-COMPTIME-002",
                "comptime sum overflowed Whole",
            ),
        ];

        for (name, source, expected_code, expected_message) in cases {
            let error = compile_source(source)
                .expect_err("M23 ineligible call must fail during source validation");
            assert_eq!(error.diagnostic().code, expected_code, "case: {name}");
            assert!(
                error.message.contains(expected_message),
                "case {name} must explain the M23 boundary: {error}"
            );
        }
    }

    #[test]
    fn rejects_m15_forward_ref_and_runtime_comptime_operands() {
        let forward = "world invalid\n\nweave main [] -> Whole:\n  comptime bind a <- sum b 1\n  comptime bind b <- sum 1 1\n  yield a\n";
        let error = compile_source(forward).expect_err("forward comptime name must fail");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-001");
        assert!(
            error.to_string().contains("not a prior"),
            "forward ref message: {error}"
        );

        let unknown = "world invalid\n\nweave main [] -> Whole:\n  comptime bind a <- sum missing 1\n  yield a\n";
        let error = compile_source(unknown).expect_err("unknown name must fail");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-001");

        let overflow = "world invalid\n\nweave main [] -> Whole:\n  comptime bind big <- sum 9223372036854775807 0\n  comptime bind boom <- sum big 1\n  yield boom\n";
        let error = compile_source(overflow).expect_err("overflow through name must fail");
        assert_eq!(error.diagnostic().code, "AE-COMPTIME-002");
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
        // Drop the empty shape table and guest kind byte so the payload can be
        // reinterpreted as v7 (pre-kind, pre-shape).
        let shape_count_offset = 5 + 4 + 2;
        assert_eq!(
            u16::from_le_bytes([
                artifact[shape_count_offset],
                artifact[shape_count_offset + 1]
            ]),
            0
        );
        artifact.drain(shape_count_offset..shape_count_offset + 2);
        // function table: count(2) + name_len(1) + "main"(4) + params(1) + result(1) + effect(1) + kind(1)
        let kind_offset = shape_count_offset + 2 + 1 + "main".len() + 1 + 1 + 1;
        assert_eq!(artifact[kind_offset], FUNCTION_KIND_GUEST);
        artifact.remove(kind_offset);
        artifact[4] = ARTIFACT_VERSION_V7;
        let error = verify_bytecode(&artifact)
            .expect_err("AETH v7 must not reinterpret AETH v8 comptime provenance");
        assert!(
            error.message.contains("valid only in AETH v8 through v11")
                || error.message.contains("valid only in AETH v8")
                || error.message.contains("unknown Aether opcode")
        );
    }

    #[test]
    fn compiles_verifies_and_runs_the_bounded_m4_error_effect() {
        let source = "world effects\n\nweave leaf [value: Whole] -> Whole raises Whole:\n  raise value\n\nweave forwarded [value: Whole] -> Whole raises Whole:\n  forward call leaf value\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call forwarded 17 into success otherwise error into code\n";
        let output = compile_to_bytecode(source).expect("M4 handled source should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V11);
        assert_eq!(format_program(&output.program), source);
        verify_bytecode(&output.bytecode).expect("M4 artifact should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M4 artifact should run")
                .exit_code,
            17
        );
    }

    #[test]
    fn compiles_verifies_and_runs_m19a_release_then_raise() {
        let source = include_str!("../../../examples/release-raise.ae");
        let output = compile_to_bytecode(source).expect("M19a release-raise should compile");
        assert!(
            output.bytecode.contains(&OP_RELEASE),
            "release must emit OP_RELEASE"
        );
        verify_bytecode(&output.bytecode).expect("verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("handled raise after release")
                .exit_code,
            9
        );
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );

        let still_live = "world bad\n\nweave boom [] -> Whole raises Whole:\n  bind label <- \"x\"\n  raise 1\n\nweave main [] -> Whole:\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call boom into success otherwise error into code\n";
        let error = compile_source(still_live).expect_err("live Text at raise fails");
        assert_eq!(error.diagnostic().code, "AE-EFFECT-003");

        let copy = "world bad\n\nweave main [] -> Whole:\n  bind n <- 1\n  release n\n  yield 0\n";
        let error = compile_source(copy).expect_err("cannot release Whole");
        assert!(
            error.to_string().contains("AE-RESOURCE-001")
                || error.diagnostic().code == "AE-RESOURCE-001",
            "{error}"
        );

        let double = "world bad\n\nweave main [] -> Whole:\n  bind label <- \"x\"\n  release label\n  release label\n  yield 0\n";
        let error = compile_source(double).expect_err("double release fails");
        assert!(
            error.to_string().contains("AE-RESOURCE-001")
                || error.diagnostic().code == "AE-RESOURCE-001"
        );
    }

    #[test]
    fn compiles_verifies_and_runs_m16_resource_handle_mix() {
        let source = include_str!("../../../examples/resource-handle.ae");
        let output = compile_to_bytecode(source).expect("M16 resource-handle should compile");
        verify_bytecode(&output.bytecode).expect("M16 artifact should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M16 success path should run")
                .exit_code,
            7
        );
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );

        let err_path = "world resource_handle_err\n\nweave boom [] -> Whole raises Whole:\n  raise 3\n\nweave main [] -> Whole:\n  bind memory <- arena 32\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call boom into success otherwise error into code\n";
        let err_out = compile_to_bytecode(err_path).expect("M16 error path should compile");
        assert_eq!(
            run_bytecode(&err_out.bytecode)
                .expect("M16 handled error should run")
                .exit_code,
            3
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

        // M16: total handle may share a weave with an arena (ADR-020).
        let resource_handle = "world ok\n\nweave leaf [] -> Whole raises Whole:\n  raise 1\n\nweave main [] -> Whole:\n  bind memory <- arena 8\n  bind mutable success <- 0\n  bind mutable code <- 0\n  handle call leaf into success otherwise error into code\n";
        let resource_out = compile_to_bytecode(resource_handle)
            .expect("M16 handle may share a total weave with arena");
        assert_eq!(
            run_bytecode(&resource_out.bytecode)
                .expect("M16 resource+handle should run")
                .exit_code,
            1
        );

        let raises_resource = "world invalid\n\nweave bad [] -> Whole raises Whole:\n  bind mutable items <- buffer Whole\n  raise 1\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  yield 0\n";
        let error = compile_source(raises_resource)
            .expect_err("erroring weaves cannot own buffers with raise");
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
        let shape_count_offset = 5 + 4 + 2;
        assert_eq!(
            u16::from_le_bytes([
                artifact[shape_count_offset],
                artifact[shape_count_offset + 1]
            ]),
            0
        );
        artifact.drain(shape_count_offset..shape_count_offset + 2);
        artifact[4] = ARTIFACT_VERSION_V6;
        let error = verify_bytecode(&artifact)
            .expect_err("v6 must not reinterpret v7 function effect metadata");
        assert!(
            error.message.contains("artifact local count")
                || error.message.contains("trailing bytes")
                || error.message.contains("unknown")
                || error.message.contains("effect")
        );

        let mut totalized_leaf = compile_to_bytecode(source)
            .expect("M4 artifact source should compile")
            .bytecode;
        // v9 header + empty record/shape tables + function count + leaf descriptor:
        // name length/name, parameter count, result tag, then effect tag.
        let leaf_effect_offset = 4 + 1 + 4 + 2 + 2 + 2 + 1 + "leaf".len() + 1 + 1;
        totalized_leaf[leaf_effect_offset] = 0;
        let error = verify_bytecode(&totalized_leaf)
            .expect_err("RAISE must not be accepted under a forged total signature");
        assert!(error.message.contains("RAISE requires an Error[Whole]"));

        let total_main = "world total\n\nweave main [] -> Whole:\n  yield 0\n";
        let mut effectful_main = compile_to_bytecode(total_main)
            .expect("total main source should compile")
            .bytecode;
        let main_effect_offset = 4 + 1 + 4 + 2 + 2 + 2 + 1 + "main".len() + 1 + 1;
        effectful_main[main_effect_offset] = 1;
        let error = verify_bytecode(&effectful_main)
            .expect_err("a forged Error[Whole] entry weave must be rejected");
        assert!(error
            .message
            .contains("main must accept no parameters, remain total, and yield Whole"));
    }

    #[test]
    fn compiles_verifies_and_runs_dual_layout_tables() {
        let columns = include_str!("../../../examples/layout-table.ae");
        let rows = columns.replace("layout columns", "layout rows");
        let columns_out =
            compile_to_bytecode(columns).expect("columns layout-table source should compile");
        let rows_out = compile_to_bytecode(&rows).expect("rows layout-table source should compile");
        assert_eq!(columns_out.bytecode[4], ARTIFACT_VERSION_V11);
        assert!(columns_out.bytecode.contains(&OP_TABLE));
        assert!(columns_out.bytecode.contains(&OP_TABLE_ALLOCATE));
        assert!(columns_out.bytecode.contains(&OP_TABLE_STORE));
        assert!(columns_out.bytecode.contains(&OP_TABLE_LOAD));
        verify_bytecode(&columns_out.bytecode).expect("columns artifact should verify");
        verify_bytecode(&rows_out.bytecode).expect("rows artifact should verify");
        let columns_run =
            run_bytecode(&columns_out.bytecode).expect("columns layout-table should run");
        let rows_run = run_bytecode(&rows_out.bytecode).expect("rows layout-table should run");
        assert_eq!(columns_run.exit_code, 10);
        assert_eq!(rows_run.exit_code, columns_run.exit_code);
        assert_eq!(format_program(&columns_out.program), columns);
    }

    #[test]
    fn rejects_invalid_m6_shape_and_table_source() {
        let empty = "world invalid\n\nshape particle:\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(empty).expect_err("empty shapes are illegal");
        assert_eq!(error.diagnostic().code, "AE-LAYOUT-001");

        let non_whole =
            "world invalid\n\nshape particle:\n  mass Text\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(non_whole).expect_err("non-Whole shape fields are illegal");
        assert_eq!(error.diagnostic().code, "AE-LAYOUT-001");

        let unknown_shape = "world invalid\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable parts <- table missing layout rows\n  yield 0\n";
        let error = compile_source(unknown_shape).expect_err("unknown shape must fail");
        assert!(
            error.message.contains("AE-LAYOUT-002")
                || error.message.contains("has not been declared")
                || error.message.contains("shape"),
            "unexpected unknown-shape diagnostic: {} / {}",
            error.diagnostic().code,
            error.message
        );

        let wrong_field = "world invalid\n\nshape particle:\n  mass Whole\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable parts <- table particle layout rows\n  choose allocate access memory move parts 1 into parts:\n    choose store move parts 0 charge 1 into parts:\n      yield 1\n    otherwise:\n      yield -2\n  otherwise:\n    yield -1\n";
        let error = compile_source(wrong_field).expect_err("unknown field must fail");
        assert!(
            error.message.contains("AE-LAYOUT-002")
                || error.message.contains("has no field")
                || error.message.contains("charge"),
            "unexpected wrong-field diagnostic: {} / {}",
            error.diagnostic().code,
            error.message
        );

        let capacity = "world invalid\n\nshape particle:\n  mass Whole\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable parts <- table particle layout rows\n  choose allocate access memory move parts 0 into parts:\n    yield 1\n  otherwise:\n    yield -1\n";
        let error = compile_source(capacity).expect_err("capacity 0 must fail");
        assert!(
            error.diagnostic().code == "AE-LAYOUT-003"
                || error.message.contains("AE-LAYOUT-003")
                || error.message.contains("capacity"),
            "unexpected capacity diagnostic: {} / {}",
            error.diagnostic().code,
            error.message
        );
    }

    #[test]
    fn verifier_rejects_table_opcodes_outside_aeth_v9() {
        let source = include_str!("../../../examples/layout-table.ae");
        let mut artifact = compile_to_bytecode(source)
            .expect("layout-table fixture should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V8;
        let error = verify_bytecode(&artifact)
            .expect_err("AETH v8 must not reinterpret AETH v9 table metadata");
        assert!(
            error.message.contains("trailing bytes")
                || error.message.contains("outside")
                || error.message.contains("unknown")
                || error.message.contains("function count")
                || error.message.contains("shape")
        );
    }

    #[test]
    fn compiles_verifies_and_runs_structured_nurseries() {
        let total = include_str!("../../../examples/nursery-total.ae");
        let output = compile_to_bytecode(total).expect("total nursery should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V11);
        assert!(output.bytecode.contains(&OP_NURSERY_BEGIN));
        assert!(output.bytecode.contains(&OP_NURSERY_SPAWN));
        assert!(output.bytecode.contains(&OP_NURSERY_END));
        verify_bytecode(&output.bytecode).expect("total nursery should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("total nursery should run")
                .exit_code,
            7
        );

        let cancel = include_str!("../../../examples/nursery-cancel.ae");
        let cancel_out = compile_to_bytecode(cancel).expect("cancel nursery should compile");
        verify_bytecode(&cancel_out.bytecode).expect("cancel nursery should verify");
        assert_eq!(
            run_bytecode(&cancel_out.bytecode)
                .expect("cancel nursery should run")
                .exit_code,
            9
        );
    }

    #[test]
    fn m19e_active_task_frame_cancels_at_a_private_resource_checkpoint() {
        let source = include_str!("../../../examples/active-cancel.ae");
        let output = compile_to_bytecode(source).expect("M19e active-cancel source should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V12);
        assert!(
            output.bytecode.contains(&OP_TASK_CHECKPOINT),
            "the task fixture must emit an explicit suspension opcode"
        );
        assert_eq!(
            u32::from_le_bytes(
                output.bytecode[5..9]
                    .try_into()
                    .expect("AETH capacity header")
            ),
            64,
            "one private staged-task lane is the exact concurrent arena requirement"
        );
        assert!(canonical_ast(&output.program).contains("Weave(staged->Whole#Total#Task)"));
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );
        verify_bytecode(&output.bytecode).expect("M19e artifact must verify before execution");
        let seeded = compile_with_seed(source)
            .expect("M19e active cancellation must compile through the checked-in seed");
        assert_eq!(
            seeded.bytecode, output.bytecode,
            "seed and bootstrap must agree on active-frame cancellation artifacts"
        );
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M19e active cancellation must run deterministically")
                .exit_code,
            9,
            "the later companion failure must propagate after cancellation"
        );

        let artifact = parse_artifact(&output.bytecode).expect("verified fixture must parse");
        let main_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "main")
            .expect("fixture must retain main");
        let mut stdout = String::new();
        let mut runtime_state =
            RuntimeState::with_hosts(artifact.arena_capacity, HostServices::pure_fixture())
                .expect("verified header capacity must admit the runtime arena");
        let exit = execute_function(
            &artifact,
            main_index,
            Vec::new(),
            &mut stdout,
            &mut runtime_state,
            0,
        )
        .expect("active frame fixture must execute under the internal scheduler");
        assert!(matches!(exit, RuntimeExit::Return(RuntimeValue::Whole(9))));
        assert_eq!(
            runtime_state.task_frame_trace,
            vec![
                TaskFrameTraceEvent::Started(0),
                TaskFrameTraceEvent::Parked(0),
                TaskFrameTraceEvent::Cancelled(0),
            ],
            "the task must start, park at its checkpoint, then be cancelled rather than resumed"
        );
        assert_eq!(
            runtime_state.task_destroyed_local_slots,
            vec![4, 3, 2],
            "cancellation must clear the live Buffer, Arena, and record-owned Text/Bytes in reverse slot order; moved inputs are not destroyed twice"
        );
        assert_eq!(
            runtime_state.arena.used, 0,
            "cancelled slab ownership must be reclaimed"
        );
        assert!(
            runtime_state.arena.bytes.iter().all(|byte| *byte == 0),
            "cancelled task-lane bytes must be scrubbed before the nursery returns"
        );
    }

    #[test]
    fn m19e_scheduler_retains_completed_results_and_only_cancels_live_or_pending_children() {
        let source = "world scheduler_results\n\ntask weave fast [] -> Whole:\n  bind mutable result <- 7\n  choose dim:\n    checkpoint\n    revise result <- 0\n  otherwise:\n    revise result <- 7\n  yield result\n\ntask weave parked [] -> Whole:\n  checkpoint\n  yield 8\n\nweave fail [] -> Whole raises Whole:\n  raise 9\n\nweave run [] -> Whole raises Whole:\n  bind mutable completed <- 0\n  bind mutable cancelled <- 0\n  bind mutable pending <- 0\n  together:\n    spawn call fast into completed\n    spawn call parked into cancelled\n    spawn call fail into pending\n  yield completed\n\nweave main [] -> Whole:\n  bind mutable value <- 0\n  bind mutable fault <- 0\n  handle call run into value otherwise error into fault\n";
        let output = compile_to_bytecode(source)
            .expect("completed/cancelled scheduler fixture should compile");
        verify_bytecode(&output.bytecode).expect("scheduler fixture must verify before execution");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("scheduler fixture should re-raise deterministically")
                .exit_code,
            9
        );

        let artifact = parse_artifact(&output.bytecode).expect("fixture artifact must parse");
        let function_index = |name: &str| {
            artifact
                .functions
                .iter()
                .position(|function| function.name == name)
                .unwrap_or_else(|| panic!("fixture must retain {name}"))
        };
        let run_index = function_index("run");
        let mut parent_locals =
            vec![Some(RuntimeValue::Whole(0)); artifact.functions[run_index].locals.len()];
        assert_eq!(
            parent_locals.len(),
            3,
            "fixture keeps three nursery destinations"
        );
        let children = vec![
            V12NurseryChild {
                function: function_index("fast"),
                arguments: Vec::new(),
                destination: 0,
            },
            V12NurseryChild {
                function: function_index("parked"),
                arguments: Vec::new(),
                destination: 1,
            },
            V12NurseryChild {
                function: function_index("fail"),
                arguments: Vec::new(),
                destination: 2,
            },
        ];
        let mut stdout = String::new();
        let mut runtime_state =
            RuntimeState::with_hosts(artifact.arena_capacity, HostServices::pure_fixture())
                .expect("verified fixture capacity must admit its runtime state");
        let outcome = run_v12_nursery(
            V12NurseryExecutionContext {
                artifact: &artifact,
                parent: &artifact.functions[run_index],
                parent_locals: &mut parent_locals,
                stdout: &mut stdout,
                runtime_state: &mut runtime_state,
                depth: 0,
                offset: 0,
            },
            children,
        )
        .expect("scheduler must quiesce after the first companion failure");
        assert_eq!(outcome, Some(9));
        for (slot, expected) in [(0_usize, 7_i64), (1, 0), (2, 0)] {
            assert!(
                matches!(parent_locals[slot].as_ref(), Some(RuntimeValue::Whole(value)) if *value == expected),
                "destination slot {slot} must retain only its documented outcome"
            );
        }
        assert_eq!(
            runtime_state.task_frame_trace,
            vec![
                TaskFrameTraceEvent::Started(0),
                TaskFrameTraceEvent::Completed(0),
                TaskFrameTraceEvent::Started(1),
                TaskFrameTraceEvent::Parked(1),
                TaskFrameTraceEvent::Cancelled(1),
            ],
            "a completed task commits before failure, while its parked sibling is cancelled and the failing destination remains unchanged"
        );
    }

    #[test]
    fn m19e_task_loop_resumes_only_at_verifier_checkpoint_boundaries() {
        let source = "world task_loop\n\ntask weave count [limit: Whole] -> Whole:\n  bind mutable current <- 0\n  while less current limit:\n    checkpoint\n    revise current <- sum current 1\n  checkpoint\n  yield current\n\nweave main [] -> Whole:\n  bind mutable result <- 0\n  together:\n    spawn call count 3 into result\n  yield result\n";
        let output = compile_to_bytecode(source).expect("checkpointed task loop should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V12);
        verify_bytecode(&output.bytecode).expect("task loop back edge must target checkpoint");
        let seeded = compile_with_seed(source)
            .expect("checkpointed task loop must compile through the checked-in seed");
        assert_eq!(
            seeded.bytecode, output.bytecode,
            "seed and bootstrap must preserve task loop checkpoint placement"
        );
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("task loop should resume and complete")
                .exit_code,
            3
        );
    }

    #[test]
    fn m19e_frame_plan_adds_main_capacity_to_the_largest_task_nursery() {
        let source = include_str!("../../../examples/task-frame-capacity.ae");
        let bootstrap = compile_to_bytecode(source)
            .expect("multi-task capacity fixture should bootstrap compile");
        assert_eq!(bootstrap.bytecode[4], ARTIFACT_VERSION_V12);
        assert_eq!(
            u32::from_le_bytes(
                bootstrap.bytecode[5..9]
                    .try_into()
                    .expect("AETH v12 capacity header"),
            ),
            96,
            "main's 16-byte arena plus the 32+48-byte concurrent task lanes"
        );
        verify_bytecode(&bootstrap.bytecode).expect("exact frame plan must verify");
        assert_eq!(
            run_bytecode(&bootstrap.bytecode)
                .expect("task frame capacity fixture should run")
                .exit_code,
            3
        );

        let artifact = parse_artifact(&bootstrap.bytecode).expect("capacity fixture must parse");
        let main_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "main")
            .expect("capacity fixture must retain main");
        let mut stdout = String::new();
        let mut runtime_state =
            RuntimeState::with_hosts(artifact.arena_capacity, HostServices::pure_fixture())
                .expect("verified frame plan must admit the runtime arena");
        let exit = execute_function(
            &artifact,
            main_index,
            Vec::new(),
            &mut stdout,
            &mut runtime_state,
            0,
        )
        .expect("capacity fixture must complete through the round-robin scheduler");
        assert!(matches!(exit, RuntimeExit::Return(RuntimeValue::Whole(3))));
        assert_eq!(
            runtime_state.task_frame_trace,
            vec![
                TaskFrameTraceEvent::Started(0),
                TaskFrameTraceEvent::Parked(0),
                TaskFrameTraceEvent::Started(1),
                TaskFrameTraceEvent::Parked(1),
                TaskFrameTraceEvent::Completed(0),
                TaskFrameTraceEvent::Completed(1),
            ],
            "two parked tasks must resume and complete in source-order round-robin order"
        );

        let mut no_admission_state = RuntimeState::with_hosts(79, HostServices::pure_fixture())
            .expect("the deliberately undersized diagnostic arena should initialize");
        let error = match execute_function(
            &artifact,
            main_index,
            Vec::new(),
            &mut String::new(),
            &mut no_admission_state,
            0,
        ) {
            Ok(_) => panic!("all task lanes must be admitted before any child starts"),
            Err(error) => error,
        };
        assert!(error.message.contains("admission exceeded"));
        assert!(
            no_admission_state.task_frame_trace.is_empty(),
            "a failed slab admission must leave every task pending"
        );
        assert_eq!(no_admission_state.arena.used, 0);

        let seeded = compile_with_seed(source)
            .expect("M19e frame plan must use the checked-in seed compiler");
        assert_eq!(
            seeded.bytecode, bootstrap.bytecode,
            "seed and bootstrap must agree on the exact concurrent frame plan"
        );
    }

    #[test]
    fn m19e_preserves_main_owners_across_handled_failure_and_never_double_releases_a_task_owner() {
        let parent_owner_source = "world parent_owner\n\ntask weave parked [] -> Whole:\n  checkpoint\n  yield 1\n\nweave fail [] -> Whole raises Whole:\n  raise 9\n\nweave run [] -> Whole raises Whole:\n  bind mutable value <- 0\n  bind mutable failed <- 0\n  together:\n    spawn call parked into value\n    spawn call fail into failed\n  yield value\n\nweave main [] -> Whole:\n  bind memory <- arena 16\n  bind mutable values <- buffer Whole\n  bind mutable value <- 0\n  bind mutable fault <- 0\n  handle call run into value otherwise error into fault\n";
        let parent_owner = compile_to_bytecode(parent_owner_source)
            .expect("main-owned arena fixture should compile");
        assert_eq!(parent_owner.bytecode[4], ARTIFACT_VERSION_V12);
        assert_eq!(
            u32::from_le_bytes(
                parent_owner.bytecode[5..9]
                    .try_into()
                    .expect("v12 capacity")
            ),
            16,
            "the main-owned arena remains outside the zero-capacity parked task lane"
        );
        assert_eq!(
            compile_with_seed(parent_owner_source)
                .expect("main-owned arena fixture must seed-compile")
                .bytecode,
            parent_owner.bytecode
        );
        assert_eq!(
            run_bytecode(&parent_owner.bytecode)
                .expect("handled nursery failure must keep main resources live")
                .exit_code,
            9
        );
        let artifact =
            parse_artifact(&parent_owner.bytecode).expect("parent-owner fixture must parse");
        let main_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "main")
            .expect("parent-owner fixture must retain main");
        let mut runtime_state =
            RuntimeState::with_hosts(artifact.arena_capacity, HostServices::pure_fixture())
                .expect("parent-owner fixture capacity must admit its runtime state");
        let exit = execute_function(
            &artifact,
            main_index,
            Vec::new(),
            &mut String::new(),
            &mut runtime_state,
            0,
        )
        .expect("main-owned resources may cross the terminal M16 handle boundary");
        assert!(matches!(exit, RuntimeExit::Return(RuntimeValue::Whole(9))));
        assert_eq!(
            runtime_state.handle_live_resource_slots,
            vec![vec![0, 1], vec![0, 1]],
            "the parent Arena and Buffer must remain live both before and after its child nursery cancels"
        );

        let released_task_source = "world released_task\n\ntask weave released [] -> Whole:\n  bind memory <- arena 16\n  release memory\n  checkpoint\n  yield 1\n\nweave fail [] -> Whole raises Whole:\n  raise 9\n\nweave run [] -> Whole raises Whole:\n  bind mutable value <- 0\n  bind mutable failed <- 0\n  together:\n    spawn call released into value\n    spawn call fail into failed\n  yield value\n\nweave main [] -> Whole:\n  bind mutable value <- 0\n  bind mutable fault <- 0\n  handle call run into value otherwise error into fault\n";
        let released_task = compile_to_bytecode(released_task_source)
            .expect("released task fixture should compile");
        assert_eq!(
            compile_with_seed(released_task_source)
                .expect("released task fixture must seed-compile")
                .bytecode,
            released_task.bytecode
        );
        let artifact =
            parse_artifact(&released_task.bytecode).expect("released fixture must parse");
        let main_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "main")
            .expect("released fixture must retain main");
        let mut runtime_state =
            RuntimeState::with_hosts(artifact.arena_capacity, HostServices::pure_fixture())
                .expect("released fixture capacity must admit its runtime state");
        let exit = execute_function(
            &artifact,
            main_index,
            Vec::new(),
            &mut String::new(),
            &mut runtime_state,
            0,
        )
        .expect("released task failure must be handled by main");
        assert!(matches!(exit, RuntimeExit::Return(RuntimeValue::Whole(9))));
        assert_eq!(
            runtime_state.task_frame_trace,
            vec![
                TaskFrameTraceEvent::Started(0),
                TaskFrameTraceEvent::Parked(0),
                TaskFrameTraceEvent::Cancelled(0),
            ]
        );
        assert!(
            runtime_state.task_destroyed_local_slots.is_empty(),
            "a released Arena is already absent from the parked frame and cannot be destroyed twice"
        );
        assert_eq!(runtime_state.arena.used, 0);
        assert!(runtime_state.arena.bytes.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn m19e_rejects_task_forms_outside_the_closed_cancellation_subset() {
        let checkpoint_outside =
            "world invalid\n\nweave main [] -> Whole:\n  checkpoint\n  yield 0\n";
        let error =
            compile_source(checkpoint_outside).expect_err("checkpoint outside task must fail");
        assert_eq!(error.diagnostic().code, "AE-TASK-004");

        let no_checkpoint = "world invalid\n\ntask weave worker [] -> Whole:\n  yield 0\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(no_checkpoint).expect_err("task must declare a checkpoint");
        assert_eq!(error.diagnostic().code, "AE-TASK-004");

        let ordinary_call = "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave main [] -> Whole:\n  yield call worker\n";
        let error = compile_source(ordinary_call).expect_err("task ordinary call must fail");
        assert_eq!(error.diagnostic().code, "AE-TASK-005");

        let bad_loop = "world invalid\n\ntask weave worker [] -> Whole:\n  bind mutable value <- 0\n  while less value 1:\n    revise value <- sum value 1\n  checkpoint\n  yield value\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(bad_loop).expect_err("task loop must lead with checkpoint");
        assert_eq!(error.diagnostic().code, "AE-TASK-004");

        for (name, source, expected_code) in [
            (
                "erroring task",
                "world invalid\n\ntask weave worker [] -> Whole raises Whole:\n  checkpoint\n  raise 1\n\nweave main [] -> Whole:\n  yield 0\n",
                "AE-TASK-004",
            ),
            (
                "task main",
                "world invalid\n\ntask weave main [] -> Whole:\n  checkpoint\n  yield 0\n",
                "AE-TASK-004",
            ),
            (
                "non-Whole task",
                "world invalid\n\ntask weave worker [] -> Text:\n  checkpoint\n  yield \"no\"\n\nweave main [] -> Whole:\n  yield 0\n",
                "AE-TASK-004",
            ),
            (
                "borrowed task parameter",
                "world invalid\n\ntask weave worker [borrow value: Whole] -> Whole:\n  checkpoint\n  yield value\n\nweave main [] -> Whole:\n  yield 0\n",
                "AE-TASK-004",
            ),
            (
                "task ambient output",
                "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  speak \"no\"\n  yield 0\n\nweave main [] -> Whole:\n  yield 0\n",
                "AE-TASK-004",
            ),
        ] {
            let error = match compile_source(source) {
                Ok(_) => panic!("{name} must be rejected"),
                Err(error) => error,
            };
            assert_eq!(
                error.diagnostic().code,
                expected_code,
                "{name} diagnostic: {}",
                error.message
            );
        }

        let noisy_companion = "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave noisy [] -> Whole:\n  speak \"no\"\n  yield 2\n\nweave main [] -> Whole:\n  bind mutable first <- 0\n  bind mutable second <- 0\n  together:\n    spawn call worker into first\n    spawn call noisy into second\n  yield first\n";
        let error = compile_source(noisy_companion)
            .expect_err("a checkpointed nursery companion cannot use stdout");
        assert_eq!(error.diagnostic().code, "AE-TASK-005");

        let total_parent_failure = "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave boom [] -> Whole raises Whole:\n  raise 9\n\nweave main [] -> Whole:\n  bind mutable worker <- 0\n  bind mutable failed <- 0\n  together:\n    spawn call worker into worker\n    spawn call boom into failed\n  yield worker\n";
        let error = compile_source(total_parent_failure)
            .expect_err("a total checkpointed nursery parent cannot spawn an erroring companion");
        assert_eq!(error.diagnostic().code, "AE-TASK-002");

        let non_main_resource_owner = "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave helper [] -> Whole:\n  bind memory <- arena 8\n  yield 0\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(non_main_resource_owner)
            .expect_err("task programs forbid direct resource ownership in a non-main companion");
        assert_eq!(
            error.diagnostic().code,
            "AE-RESOURCE-004",
            "non-main task-program resource diagnostic: {}",
            error.message
        );

        let cancellation_api = "world invalid\n\ntask weave worker [] -> Whole:\n  checkpoint\n  cancel worker\n  yield 0\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(cancellation_api)
            .expect_err("M19e deliberately exposes no task handle or cancellation API");
        assert_eq!(error.diagnostic().code, "AE-TASK-001");
    }

    #[test]
    fn verifier_rejects_m19e_descriptor_and_task_call_escapes() {
        let source = "world hostile_task\n\ntask weave worker [] -> Whole:\n  checkpoint\n  yield 1\n\nweave helper [] -> Whole:\n  yield 2\n\nweave main [] -> Whole:\n  bind mutable slot <- 0\n  together:\n    spawn call worker into slot\n  bind result <- call helper\n  yield result\n";
        let output = compile_to_bytecode(source).expect("closed task fixture should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V12);
        let (worker_flags, _) = v12_primitive_function_offsets(&output.bytecode, "worker");

        let mut unknown_flag = output.bytecode.clone();
        unknown_flag[worker_flags] = 0x80;
        let error = verify_bytecode(&unknown_flag)
            .expect_err("unknown v12 function-flag bits must fail closed");
        assert!(error.message.contains("unknown bit"));

        let mut non_task_checkpoint = output.bytecode.clone();
        non_task_checkpoint[worker_flags] = 0;
        let error = verify_bytecode(&non_task_checkpoint)
            .expect_err("TASK_CHECKPOINT outside a task frame must fail verification");
        assert!(error.message.contains("TASK_CHECKPOINT requires"));

        let artifact = parse_artifact(&output.bytecode).expect("fixture must parse");
        let worker_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "worker")
            .expect("fixture must retain worker");
        let main_index = artifact
            .functions
            .iter()
            .position(|function| function.name == "main")
            .expect("fixture must retain main");
        let (_, main_code_offset) = v12_primitive_function_offsets(&output.bytecode, "main");
        let call = decode_code(&artifact.functions[main_index].code, ARTIFACT_VERSION_V12)
            .expect("fixture main code must decode")
            .into_iter()
            .find(|instruction| matches!(instruction.instruction, Instruction::Call { .. }))
            .expect("fixture main must contain the ordinary helper call");
        let mut ordinary_task_call = output.bytecode;
        let raw_call_offset = main_code_offset + call.offset;
        assert_eq!(ordinary_task_call[raw_call_offset], OP_CALL);
        ordinary_task_call[raw_call_offset + 1..raw_call_offset + 3]
            .copy_from_slice(&(worker_index as u16).to_le_bytes());
        let error = verify_bytecode(&ordinary_task_call)
            .expect_err("raw ordinary CALL to a task frame must fail verification");
        assert!(error
            .message
            .contains("may be invoked only by NURSERY_SPAWN"));

        let mut bad_capacity =
            compile_to_bytecode(include_str!("../../../examples/active-cancel.ae"))
                .expect("active cancellation fixture should compile")
                .bytecode;
        bad_capacity[5..9].copy_from_slice(&0_u32.to_le_bytes());
        let error = verify_bytecode(&bad_capacity)
            .expect_err("v12 header capacity must match its concurrent frame plan");
        assert!(error.message.contains("header arena capacity"));
    }

    #[test]
    fn verifier_rejects_m19e_checkpointed_nursery_hostile_forms_before_execution() {
        let mut legacy_checkpoint =
            compile_to_bytecode("world legacy_checkpoint\n\nweave main [] -> Whole:\n  yield 0\n")
                .expect("legacy fixture should compile")
                .bytecode;
        assert_eq!(legacy_checkpoint[4], ARTIFACT_VERSION_V11);
        let last = legacy_checkpoint
            .last_mut()
            .expect("legacy fixture must contain its terminal yield opcode");
        assert_eq!(*last, OP_YIELD);
        *last = OP_TASK_CHECKPOINT;
        let error = verify_bytecode(&legacy_checkpoint)
            .expect_err("v11 must reject a v12-only checkpoint opcode");
        assert!(error.message.contains("only in AETH v12"));

        let source = "world hostile_v12\n\ntask weave worker [] -> Whole:\n  checkpoint\n  checkpoint\n  yield 1\n\nweave fail [] -> Whole raises Whole:\n  raise 9\n\nweave forwarder [] -> Whole raises Whole:\n  forward call fail\n\nweave wrapper [] -> Whole:\n  bind mutable value <- 0\n  bind mutable fault <- 0\n  handle call fail into value otherwise error into fault\n\nweave main [] -> Whole:\n  bind mutable slot <- 0\n  together:\n    spawn call worker into slot\n  yield slot\n";
        let baseline = compile_to_bytecode(source)
            .expect("primitive hostile fixture should compile")
            .bytecode;
        verify_bytecode(&baseline).expect("baseline hostile fixture must verify");
        let artifact = parse_artifact(&baseline).expect("baseline artifact must parse");
        let index_of = |name: &str| {
            artifact
                .functions
                .iter()
                .position(|function| function.name == name)
                .unwrap_or_else(|| panic!("baseline fixture must retain {name}"))
        };
        let worker_index = index_of("worker");
        let (worker_flags, worker_code_offset) =
            v12_primitive_function_offsets(&baseline, "worker");
        let worker_checkpoint_offset =
            decode_code(&artifact.functions[worker_index].code, ARTIFACT_VERSION_V12)
                .expect("worker code must decode")
                .into_iter()
                .find(|instruction| matches!(instruction.instruction, Instruction::TaskCheckpoint))
                .expect("worker fixture must contain a task checkpoint")
                .offset;
        let raw_worker_checkpoint = worker_code_offset + worker_checkpoint_offset;
        assert_eq!(baseline[raw_worker_checkpoint], OP_TASK_CHECKPOINT);

        let mut erroring_task = baseline.clone();
        erroring_task[worker_flags - 2] = Effect::ErrorWhole.to_byte();
        let error = verify_bytecode(&erroring_task)
            .expect_err("an erroring descriptor cannot claim a task frame");
        assert!(error
            .message
            .contains("task frame must be a non-main total Whole"));

        let mut oversized_frame = baseline.clone();
        oversized_frame[worker_flags + 1..worker_flags + 5]
            .copy_from_slice(&(MAX_ARENA_BYTES + 1).to_le_bytes());
        let error = verify_bytecode(&oversized_frame)
            .expect_err("a frame capacity above the M2 limit must fail before execution");
        assert!(error.message.contains("frame arena capacity"));

        let mut non_empty_checkpoint = baseline.clone();
        let old_code_length = u32::from_le_bytes(
            non_empty_checkpoint[worker_code_offset - 4..worker_code_offset]
                .try_into()
                .expect("worker code length must be encoded"),
        );
        non_empty_checkpoint.splice(
            raw_worker_checkpoint..raw_worker_checkpoint,
            [OP_PUSH_WHOLE, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        non_empty_checkpoint[worker_code_offset - 4..worker_code_offset]
            .copy_from_slice(&(old_code_length + 9).to_le_bytes());
        let error = verify_bytecode(&non_empty_checkpoint)
            .expect_err("a checkpoint with a transient operand must fail before execution");
        assert!(error.message.contains("empty operand stack"));

        let mut forbidden_task_opcode = baseline.clone();
        forbidden_task_opcode[raw_worker_checkpoint] = OP_SPEAK;
        let error = verify_bytecode(&forbidden_task_opcode)
            .expect_err("a task frame cannot smuggle stdout work into a cancellation region");
        assert!(error
            .message
            .contains("outside the self-contained task subset"));

        let wrapper_index = index_of("wrapper");
        let (_, wrapper_code_offset) = v12_primitive_function_offsets(&baseline, "wrapper");
        let wrapper_handle_offset = decode_code(
            &artifact.functions[wrapper_index].code,
            ARTIFACT_VERSION_V12,
        )
        .expect("wrapper code must decode")
        .into_iter()
        .find(|instruction| matches!(instruction.instruction, Instruction::HandleCall { .. }))
        .expect("wrapper fixture must contain a handle call")
        .offset;
        let mut task_handle = baseline.clone();
        let raw_wrapper_handle = wrapper_code_offset + wrapper_handle_offset;
        assert_eq!(task_handle[raw_wrapper_handle], OP_HANDLE_CALL);
        task_handle[raw_wrapper_handle + 1..raw_wrapper_handle + 3]
            .copy_from_slice(&(worker_index as u16).to_le_bytes());
        let error =
            verify_bytecode(&task_handle).expect_err("HANDLE_CALL cannot target a task frame");
        assert!(error
            .message
            .contains("may be invoked only by NURSERY_SPAWN"));

        let forwarder_index = index_of("forwarder");
        let (_, forwarder_code_offset) = v12_primitive_function_offsets(&baseline, "forwarder");
        let forward_offset = decode_code(
            &artifact.functions[forwarder_index].code,
            ARTIFACT_VERSION_V12,
        )
        .expect("forwarder code must decode")
        .into_iter()
        .find(|instruction| matches!(instruction.instruction, Instruction::ForwardCall { .. }))
        .expect("forwarder fixture must contain a forward call")
        .offset;
        let mut task_forward = baseline.clone();
        let raw_forward = forwarder_code_offset + forward_offset;
        assert_eq!(task_forward[raw_forward], OP_FORWARD_CALL);
        task_forward[raw_forward + 1..raw_forward + 3]
            .copy_from_slice(&(worker_index as u16).to_le_bytes());
        let error =
            verify_bytecode(&task_forward).expect_err("FORWARD_CALL cannot target a task frame");
        assert!(error
            .message
            .contains("may be invoked only by NURSERY_SPAWN"));

        let main_index = index_of("main");
        let (_, main_code_offset) = v12_primitive_function_offsets(&baseline, "main");
        let spawn_offset = decode_code(&artifact.functions[main_index].code, ARTIFACT_VERSION_V12)
            .expect("main code must decode")
            .into_iter()
            .find(|instruction| matches!(instruction.instruction, Instruction::NurserySpawn { .. }))
            .expect("main fixture must contain a nursery spawn")
            .offset;
        let mut invalid_destination = baseline.clone();
        let raw_spawn = main_code_offset + spawn_offset;
        assert_eq!(invalid_destination[raw_spawn], OP_NURSERY_SPAWN);
        invalid_destination[raw_spawn + 4..raw_spawn + 6].copy_from_slice(&u16::MAX.to_le_bytes());
        let error = verify_bytecode(&invalid_destination)
            .expect_err("a malformed v12 nursery destination must fail before scheduling");
        assert!(error
            .message
            .contains("local slot is outside the local table"));

        let loop_artifact = compile_to_bytecode(include_str!("../../../examples/task-loop.ae"))
            .expect("task loop fixture should compile")
            .bytecode;
        let loop_parsed = parse_artifact(&loop_artifact).expect("task loop artifact must parse");
        let loop_index = loop_parsed
            .functions
            .iter()
            .position(|function| function.name == "count")
            .expect("task loop fixture must retain count");
        let (_, loop_code_offset) = v12_primitive_function_offsets(&loop_artifact, "count");
        let loop_code = decode_code(
            &loop_parsed.functions[loop_index].code,
            ARTIFACT_VERSION_V12,
        )
        .expect("task loop code must decode");
        let back_edge = loop_code
            .iter()
            .find_map(|instruction| match instruction.instruction {
                Instruction::Jump(target) if target < instruction.offset => {
                    Some((instruction.offset, target))
                }
                _ => None,
            })
            .expect("task loop fixture must contain a backward jump");
        let invalid_target = back_edge.1 + 1;
        assert!(
            loop_code.iter().any(|instruction| {
                instruction.offset == invalid_target
                    && !matches!(instruction.instruction, Instruction::TaskCheckpoint)
            }),
            "the hostile target must remain an instruction boundary but not a checkpoint"
        );
        let mut invalid_back_edge = loop_artifact;
        let raw_back_edge = loop_code_offset + back_edge.0;
        assert_eq!(invalid_back_edge[raw_back_edge], OP_JUMP);
        invalid_back_edge[raw_back_edge + 1..raw_back_edge + 5]
            .copy_from_slice(&(invalid_target as u32).to_le_bytes());
        let error = verify_bytecode(&invalid_back_edge)
            .expect_err("a task back edge that skips its checkpoint must fail verification");
        assert!(error
            .message
            .contains("backward jumps must target TASK_CHECKPOINT"));
    }

    #[test]
    fn rejects_illegal_nursery_shapes_and_boundaries() {
        let empty = "world invalid\n\nweave main [] -> Whole:\n  together:\n  yield 0\n";
        let error = compile_source(empty).expect_err("empty together must fail");
        assert_eq!(error.diagnostic().code, "AE-TASK-001");

        // M19b: resourceful spawn *callee* is still forbidden (main uses resources).
        // Resource spawn arguments remain forbidden (Policy A+).
        let resource_arg = "world invalid\n\nweave worker [access memory: Arena] -> Whole:\n  yield 0\n\nweave main [] -> Whole:\n  bind memory <- arena 64\n  bind mutable a <- 0\n  together:\n    spawn call worker access memory into a\n  yield a\n";
        let error = compile_source(resource_arg).expect_err("resource spawn args fail");
        assert_eq!(error.diagnostic().code, "AE-TASK-003");

        let worker = include_str!("../../../examples/spawn-arena.ae");
        let compiled = compile_to_bytecode(worker).expect("spawn-arena should compile");
        assert_eq!(
            run_bytecode(&compiled.bytecode)
                .expect("spawn-arena should run")
                .exit_code,
            7
        );
        let seeded = compile_with_seed(worker).expect("spawn-arena seed path");
        assert_eq!(
            seeded.bytecode, compiled.bytecode,
            "spawn-arena seed≡bootstrap"
        );
        // Header capacity = 32 + 16 = 48.
        assert_eq!(
            u32::from_le_bytes(compiled.bytecode[5..9].try_into().unwrap()),
            48
        );
    }

    #[test]
    fn compiles_verifies_and_runs_m19b_nursery_with_parent_resource() {
        let source = include_str!("../../../examples/nursery-resource.ae");
        let output = compile_to_bytecode(source).expect("M19b nursery+parent arena should compile");
        verify_bytecode(&output.bytecode).expect("M19b artifact should verify");
        assert_eq!(
            run_bytecode(&output.bytecode)
                .expect("M19b mix should run")
                .exit_code,
            7
        );
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );

        let seeded = compile_with_seed(source).expect("M19b must seed-compile");
        assert_eq!(
            seeded.bytecode, output.bytecode,
            "M19b mix must match bootstrap byte-for-byte"
        );
    }

    #[test]
    fn verifier_rejects_nursery_opcodes_outside_aeth_v10() {
        let source = include_str!("../../../examples/nursery-total.ae");
        let mut artifact = compile_to_bytecode(source)
            .expect("nursery fixture should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V9;
        let error = verify_bytecode(&artifact)
            .expect_err("AETH v9 must not accept AETH v10 nursery opcodes");
        assert!(
            error.message.contains("valid only in AETH v10")
                || error.message.contains("valid only in AETH v10 or v11")
                || error.message.contains("unknown")
                || error.message.contains("outside")
                || error.message.contains("trailing")
                || error.message.contains("function kind")
                || error.message.contains("code is truncated")
                || error.message.contains("local")
        );
    }

    #[test]
    fn compiles_verifies_and_runs_the_host_abi_pilot() {
        let source = include_str!("../../../examples/host-pilot.ae");
        let output = compile_to_bytecode(source).expect("host-pilot should compile");
        assert_eq!(output.bytecode[4], ARTIFACT_VERSION_V11);
        assert!(
            output.bytecode.contains(&OP_HOST_CALL),
            "host-pilot artifact must emit HOST_CALL"
        );
        assert!(
            !output
                .program
                .host_weaves
                .iter()
                .any(|host| host.name == "main"),
            "main must remain a guest weave"
        );
        assert_eq!(output.program.host_weaves.len(), 2);
        verify_bytecode(&output.bytecode).expect("host-pilot should verify");
        let run = run_bytecode(&output.bytecode).expect("host-pilot should run");
        assert_eq!(run.exit_code, 48, "41+1 + len(\"Aether\") == 48");
        assert_eq!(
            format_program(&output.program),
            source.replace("\r\n", "\n")
        );
    }

    #[test]
    fn rejects_illegal_host_weave_forms_and_missing_services() {
        let body = "world bad\n\nhost weave whole_inc [value: Whole] -> Whole:\n  yield value\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(body).expect_err("host weave with body must fail");
        assert_eq!(error.diagnostic().code, "AE-HOST-001");

        let raises = "world bad\n\nhost weave boom [value: Whole] -> Whole raises Whole\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(raises).expect_err("host weave raises must fail");
        assert_eq!(error.diagnostic().code, "AE-HOST-001");

        let buffer = "world bad\n\nhost weave take [buffer: BufferWhole] -> Whole\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(buffer).expect_err("host buffer param must fail");
        assert_eq!(error.diagnostic().code, "AE-HOST-001");

        let unknown = "world bad\n\nhost weave mystery [value: Whole] -> Whole\n\nweave main [] -> Whole:\n  yield call mystery 1\n";
        let output = compile_to_bytecode(unknown).expect("undeclared service name still compiles");
        let error =
            run_bytecode(&output.bytecode).expect_err("missing host service must fail closed");
        assert!(error.message.contains("AE-HOST-003"));
    }

    #[test]
    fn verifier_rejects_host_call_outside_aeth_v11() {
        let source = include_str!("../../../examples/host-pilot.ae");
        let mut artifact = compile_to_bytecode(source)
            .expect("host-pilot should compile")
            .bytecode;
        artifact[4] = ARTIFACT_VERSION_V10;
        let error = verify_bytecode(&artifact)
            .expect_err("AETH v10 must not accept host kind metadata or HOST_CALL");
        assert!(
            error.message.contains("valid only in AETH v11")
                || error.message.contains("unknown")
                || error.message.contains("outside")
                || error.message.contains("truncated")
                || error.message.contains("local")
                || error.message.contains("function kind")
                || error.message.contains("truncated is truncated")
        );
    }

    fn m21_pilot_library_path() -> std::path::PathBuf {
        // Prefer CARGO_TARGET_DIR layout; search common debug locations.
        let target = std::env::var_os("CARGO_TARGET_TMPDIR")
            .map(std::path::PathBuf::from)
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target")
            });
        let name = if cfg!(windows) {
            "aether_ffi_pilot.dll"
        } else if cfg!(target_os = "macos") {
            "libaether_ffi_pilot.dylib"
        } else {
            "libaether_ffi_pilot.so"
        };
        for dir in [
            target.join("debug"),
            target.join("debug/deps"),
            target.join("debug/examples"),
        ] {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return candidate.canonicalize().expect("pilot lib path");
            }
        }
        panic!(
            "M21 pilot library {name} not found under {}; build aether-ffi-pilot first",
            target.display()
        );
    }

    #[test]
    fn m21_foreign_whole_inc_requires_lib_grant_and_runs() {
        let source = r#"world foreign_pilot

foreign weave whole_inc_f [value: Whole] -> Whole from "pilot" symbol "aether_whole_inc"

weave main [] -> Whole:
  yield call whole_inc_f 41
"#;
        let compiled = compile_to_bytecode(source).expect("foreign weave should bootstrap-compile");
        assert!(
            compiled.program.host_weaves[0].foreign.is_some(),
            "foreign metadata on host weave"
        );
        assert!(format_program(&compiled.program).contains("foreign weave whole_inc_f"));

        let denied = run_bytecode(&compiled.bytecode).expect_err("missing grant-lib fails closed");
        assert!(
            denied.message.contains("AE-FFI-003") || denied.message.contains("not granted"),
            "{denied}"
        );

        let mut grants = HostGrantConfig::default();
        grants
            .library_grants
            .insert("pilot".to_owned(), m21_pilot_library_path());
        let run = run_bytecode_with_grants(&compiled.bytecode, grants)
            .expect("granted pilot library should run");
        assert_eq!(run.exit_code, 42);

        let bad_symbol = r#"world foreign_bad

foreign weave missing [value: Whole] -> Whole from "pilot" symbol "no_such_symbol"

weave main [] -> Whole:
  yield call missing 1
"#;
        let bad = compile_to_bytecode(bad_symbol).expect("compile");
        let mut grants = HostGrantConfig::default();
        grants
            .library_grants
            .insert("pilot".to_owned(), m21_pilot_library_path());
        let err = run_bytecode_with_grants(&bad.bytecode, grants)
            .expect_err("missing symbol fails closed");
        assert!(
            err.message.contains("AE-FFI-003") || err.message.contains("missing"),
            "{err}"
        );

        let text_param = "world bad\n\nforeign weave t [borrow s: Text] -> Whole from \"pilot\" symbol \"aether_whole_inc\"\n\nweave main [] -> Whole:\n  yield 0\n";
        let error = compile_source(text_param).expect_err("Text param not in pilot");
        assert!(
            error.diagnostic().code == "AE-FFI-001"
                || error.to_string().contains("AE-FFI-001")
                || error.to_string().contains("owned Whole"),
            "{error}"
        );

        let sum_source = r#"world foreign_sum

foreign weave whole_sum_f [left: Whole, right: Whole] -> Whole from "pilot" symbol "aether_whole_sum"

weave main [] -> Whole:
  yield call whole_sum_f 17 25
"#;
        let sum = compile_with_seed(sum_source).expect("two-arg foreign should seed-compile");
        let bootstrap_sum = compile_to_bytecode(sum_source).expect("two-arg foreign bootstrap");
        assert_eq!(
            sum.bytecode, bootstrap_sum.bytecode,
            "two-arg foreign seed≡bootstrap"
        );
        let mut grants = HostGrantConfig::default();
        grants
            .library_grants
            .insert("pilot".to_owned(), m21_pilot_library_path());
        let run = run_bytecode_with_grants(&sum.bytecode, grants)
            .expect("granted two-arg foreign should run");
        assert_eq!(run.exit_code, 42);
    }

    #[test]
    fn host_invoke_still_rejects_resources_at_the_boundary() {
        let source = "world boundary\n\nweave main [] -> Whole:\n  yield 0\n\nweave helper [value: Whole] -> Whole:\n  yield value\n";
        let artifact = compile_to_bytecode(source)
            .expect("helper fixture should compile")
            .bytecode;
        let error = invoke_bytecode(&artifact, "helper", &[InvocationValue::Whole(1)])
            .expect("primitive host invoke remains legal")
            .value;
        assert_eq!(error, InvocationValue::Whole(1));
        // Record/resource rejection is preserved on the forge-style boundary.
        let record_source = "world records\n\nrecord card [score: Whole]\n\nweave main [] -> Whole:\n  bind value <- make card 7\n  yield call project borrow value\n\nweave project [borrow value: card] -> Whole:\n  yield field borrow value score\n";
        let record_artifact = compile_to_bytecode(record_source)
            .expect("record fixture should compile")
            .bytecode;
        let refused = invoke_bytecode(&record_artifact, "project", &[InvocationValue::Whole(0)])
            .expect_err("host invoke still rejects mismatched record parameters");
        assert!(
            refused.message.contains("requires")
                || refused.message.contains("resource")
                || refused.message.contains("record")
        );
    }

    fn m14_temp_root(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "aether-m14-{}-{}-{}",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("m14 temp root");
        path
    }

    #[test]
    fn m14_pure_host_pilot_still_runs_without_grants() {
        let source = include_str!("../../../examples/host-pilot.ae");
        let artifact = compile_to_bytecode(source)
            .expect("host-pilot should compile")
            .bytecode;
        let run = run_bytecode_with_grants(&artifact, HostGrantConfig::default())
            .expect("empty grants keep pure fixtures");
        assert_eq!(run.exit_code, 48);
    }

    #[test]
    fn m14_read_text_requires_grant_and_respects_path_jail() {
        let root = m14_temp_root("read");
        std::fs::write(root.join("config.txt"), "Aether").expect("fixture write");
        let source = include_str!("../../../examples/host-io-read.ae");
        let artifact = compile_to_bytecode(source)
            .expect("host-io-read should compile")
            .bytecode;

        let denied = run_bytecode(&artifact).expect_err("read without grant fails closed");
        assert!(
            denied.message.contains("AE-HOST-003"),
            "missing grant: {}",
            denied.message
        );

        let granted = run_bytecode_with_grants(
            &artifact,
            HostGrantConfig {
                read_roots: vec![root.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect("granted read_text should succeed");
        assert_eq!(granted.exit_code, 6, "len(\"Aether\") == 6");

        let escape_source = "world escape\n\nhost weave read_text [borrow path: Text] -> Text\n\nweave main [] -> Whole:\n  bind bad <- \"../config.txt\"\n  bind cfg <- call read_text borrow bad\n  yield 0\n";
        let escape_artifact = compile_to_bytecode(escape_source)
            .expect("escape program compiles")
            .bytecode;
        let escaped = run_bytecode_with_grants(
            &escape_artifact,
            HostGrantConfig {
                read_roots: vec![root.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect_err("path escape must fail");
        assert!(
            escaped.message.contains("AE-HOST-004"),
            "escape: {}",
            escaped.message
        );

        let absolute_source = "world absolute_path\n\nhost weave read_text [borrow path: Text] -> Text\n\nweave main [] -> Whole:\n  bind bad <- \"/tmp/x\"\n  bind cfg <- call read_text borrow bad\n  yield 0\n";
        let absolute_artifact = compile_to_bytecode(absolute_source)
            .expect("absolute program compiles")
            .bytecode;
        let absolute = run_bytecode_with_grants(
            &absolute_artifact,
            HostGrantConfig {
                read_roots: vec![root.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect_err("absolute guest path must fail");
        assert!(
            absolute.message.contains("AE-HOST-004"),
            "absolute: {}",
            absolute.message
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn m14_write_text_and_env_get_honor_grants() {
        let root = m14_temp_root("write");
        let write_source = include_str!("../../../examples/host-io-write.ae");
        let write_artifact = compile_to_bytecode(write_source)
            .expect("host-io-write should compile")
            .bytecode;
        let denied = run_bytecode(&write_artifact).expect_err("write without grant fails");
        assert!(denied.message.contains("AE-HOST-003"));

        let written = run_bytecode_with_grants(
            &write_artifact,
            HostGrantConfig {
                read_roots: Vec::new(),
                write_roots: vec![root.clone()],
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect("granted write_text should succeed");
        assert_eq!(written.exit_code, 5, "len(\"hello\") == 5");
        let body = std::fs::read_to_string(root.join("out.txt")).expect("out.txt readable");
        assert_eq!(body, "hello");

        std::env::set_var("AETHER_M14_TEST_VAR", "granted-value");
        let env_source = "world env_demo\n\nhost weave env_get [borrow name: Text] -> Text\n\nhost weave text_extent [borrow message: Text] -> Whole\n\nweave main [] -> Whole:\n  bind key <- \"AETHER_M14_TEST_VAR\"\n  bind value <- call env_get borrow key\n  yield call text_extent borrow value\n";
        let env_artifact = compile_to_bytecode(env_source)
            .expect("env_get program should compile")
            .bytecode;
        let env_denied = run_bytecode(&env_artifact).expect_err("env without grant fails");
        assert!(env_denied.message.contains("AE-HOST-003"));
        let env_wrong = run_bytecode_with_grants(
            &env_artifact,
            HostGrantConfig {
                read_roots: Vec::new(),
                write_roots: Vec::new(),
                env_names: vec!["OTHER_NAME".to_owned()],
                library_grants: Default::default(),
            },
        )
        .expect_err("ungranted env name fails");
        assert!(env_wrong.message.contains("AE-HOST-003"));
        let env_ok = run_bytecode_with_grants(
            &env_artifact,
            HostGrantConfig {
                read_roots: Vec::new(),
                write_roots: Vec::new(),
                env_names: vec!["AETHER_M14_TEST_VAR".to_owned()],
                library_grants: Default::default(),
            },
        )
        .expect("granted env_get succeeds");
        assert_eq!(env_ok.exit_code, 13, "len(\"granted-value\") == 13");
        std::env::remove_var("AETHER_M14_TEST_VAR");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn m14_read_bytes_and_size_limit() {
        let root = m14_temp_root("bytes");
        std::fs::write(root.join("blob.bin"), b"\x00\x01\x02").expect("blob");
        let bytes_source = "world host_io_bytes\n\nhost weave read_bytes [borrow path: Text] -> Bytes\n\nweave main [] -> Whole:\n  bind path <- \"blob.bin\"\n  bind data <- call read_bytes borrow path\n  yield extent borrow data\n";
        let artifact = compile_to_bytecode(bytes_source)
            .expect("read_bytes program compiles")
            .bytecode;
        let run = run_bytecode_with_grants(
            &artifact,
            HostGrantConfig {
                read_roots: vec![root.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect("read_bytes under grant");
        assert_eq!(run.exit_code, 3);

        let oversize = vec![b'x'; HOST_IO_MAX_BYTES + 1];
        std::fs::write(root.join("big.txt"), &oversize).expect("big file");
        let big_source = "world host_io_big\n\nhost weave read_text [borrow path: Text] -> Text\n\nweave main [] -> Whole:\n  bind path <- \"big.txt\"\n  bind cfg <- call read_text borrow path\n  yield 0\n";
        let big_artifact = compile_to_bytecode(big_source)
            .expect("big read compiles")
            .bytecode;
        let limited = run_bytecode_with_grants(
            &big_artifact,
            HostGrantConfig {
                read_roots: vec![root.clone()],
                write_roots: Vec::new(),
                env_names: Vec::new(),
                library_grants: Default::default(),
            },
        )
        .expect_err("oversize read fails");
        assert!(
            limited.message.contains("AE-HOST-005"),
            "size: {}",
            limited.message
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn layout_harness_compares_capacity_256_workloads() {
        // Closed resource outcomes are terminal chooses, so the product harness
        // fills a fixed prefix of a capacity-256 table with nested store/load
        // chains. Full 256-cell logical equivalence is covered by the pure model.
        let columns = harness_layout_source("columns");
        let rows = harness_layout_source("rows");
        let columns_bc = compile_to_bytecode(&columns)
            .expect("columns harness should compile")
            .bytecode;
        let rows_bc = compile_to_bytecode(&rows)
            .expect("rows harness should compile")
            .bytecode;
        // Final mass[7] + charge[7] under the closed terminal-outcome discipline.
        let expected = 7 + 14;
        let start = std::time::Instant::now();
        let mut columns_exit = 0;
        for _ in 0..32 {
            columns_exit = run_bytecode(&columns_bc)
                .expect("columns harness should run")
                .exit_code;
        }
        let columns_elapsed = start.elapsed();
        let start = std::time::Instant::now();
        let mut rows_exit = 0;
        for _ in 0..32 {
            rows_exit = run_bytecode(&rows_bc)
                .expect("rows harness should run")
                .exit_code;
        }
        let rows_elapsed = start.elapsed();
        assert_eq!(columns_exit, expected);
        assert_eq!(rows_exit, expected);
        eprintln!(
            "M6 layout harness (32 runs, capacity 256, 8 store pairs + loads): columns={columns_elapsed:?} rows={rows_elapsed:?} exit={columns_exit}"
        );
    }

    fn harness_layout_source(layout: &str) -> String {
        // Binary ops accept only atoms, and resource branches admit only one
        // terminal statement. Accumulate with paired sum locals written by load.
        let mut source = format!(
            "world harness\n\nshape particle:\n  mass Whole\n  charge Whole\n\nweave main [] -> Whole:\n  bind memory <- arena 1000000\n  bind mutable parts <- table particle layout {layout}\n  bind mutable mass_total <- 0\n  bind mutable charge_total <- 0\n  bind mutable sample <- 0\n"
        );
        source.push_str("  choose allocate access memory move parts 256 into parts:\n");
        let indent = |depth: usize| "  ".repeat(depth);
        let mut depth = 2_usize;
        for index in 0..8_i64 {
            source.push_str(&format!(
                "{}choose store move parts {index} mass {index} into parts:\n",
                indent(depth)
            ));
            depth += 1;
            source.push_str(&format!(
                "{}choose store move parts {index} charge {} into parts:\n",
                indent(depth),
                index * 2
            ));
            depth += 1;
        }
        // Load mass cells into mass_total by folding through sample and a helper
        // sum expression at the final yield after all loads into fixed slots.
        for index in 0..8_i64 {
            source.push_str(&format!(
                "{}choose load borrow parts {index} mass into sample:\n",
                indent(depth)
            ));
            depth += 1;
        }
        // After the last mass load, sample holds mass of index 7. Prior masses are
        // not retained under the one-statement resource-branch rule, so the
        // harness proves dual-layout equivalence on the final loaded mass and
        // charge pair under capacity 256, plus store success for the prefix.
        source.push_str(&format!(
            "{}choose load borrow parts 7 charge into charge_total:\n",
            indent(depth)
        ));
        depth += 1;
        source.push_str(&format!("{}yield sum sample charge_total\n", indent(depth)));
        for current in (2..=depth).rev() {
            source.push_str(&format!("{}otherwise:\n", indent(current - 1)));
            source.push_str(&format!("{}yield -1\n", indent(current)));
        }
        source
    }

    fn hex_encode(bytes: &[u8]) -> String {
        let mut output = String::with_capacity(bytes.len() * 2);
        write_hex_bytes(bytes, &mut output);
        output
    }
}
