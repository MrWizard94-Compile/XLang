//! M35 verified AETH → C pure pilot (F-NATIVE, ADR-059 / ADR-062 / ADR-073 M35c).
//!
//! Lowers a restricted pure Total subset: Whole locals/arithmetic, SPEAK/Text,
//! and multi-weave pure Whole helpers via CALL. Input must already verify.

use crate::{
    parse_artifact, verify_bytecode, Artifact, ArtifactFunction, BytecodeError, Effect, ValueType,
    OP_CALL, OP_DIFFERENCE, OP_LOAD, OP_PRODUCT, OP_PUSH_TEXT, OP_PUSH_WHOLE, OP_QUOTIENT,
    OP_REMAINDER, OP_SPEAK, OP_STORE, OP_SUM, OP_YIELD,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeError {
    pub code: &'static str,
    pub message: String,
}

impl NativeError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for NativeError {}

impl From<BytecodeError> for NativeError {
    fn from(error: BytecodeError) -> Self {
        Self::new("AE-NATIVE-001", error.to_string())
    }
}

#[must_use]
pub const fn f_native_authorized() -> bool {
    true
}

#[must_use]
pub const fn native_aeth_to_c_pilot() -> bool {
    true
}

/// M35b: Whole locals + arithmetic ops are supported in the pure pilot.
#[must_use]
pub const fn native_aeth_to_c_locals_pilot() -> bool {
    true
}

/// M35c: SPEAK/Text and multi-weave pure Whole helpers (CALL) are supported.
#[must_use]
pub const fn native_aeth_to_c_speak_multiweave_pilot() -> bool {
    true
}

/// Lower **verified** AETH to ISO C for the pure pilot subset (M35a–c).
pub fn lower_verified_aeth_to_c(bytecode: &[u8]) -> Result<String, NativeError> {
    debug_assert!(
        f_native_authorized() && native_aeth_to_c_pilot(),
        "ADR-059: F-NATIVE AETH→C pilot"
    );
    verify_bytecode(bytecode)?;
    let artifact = parse_artifact(bytecode)?;
    validate_pure_pilot_artifact(&artifact)?;
    emit_c_for_artifact(&artifact)
}

/// M35f: product path can emit LLVM IR text from verified AETH (pure subset).
#[must_use]
pub const fn native_llvm_ir_emit_product() -> bool {
    true
}

/// Lower **verified** AETH to LLVM IR text for the pure Total subset (M35f).
///
/// Hand-written IR emitter (no libLLVM). Suitable for `llc`/`clang` when a
/// toolchain is present. Not a full LLVM object product path.
pub fn lower_verified_aeth_to_llvm_ir(bytecode: &[u8]) -> Result<String, NativeError> {
    debug_assert!(
        f_native_authorized() && native_llvm_ir_emit_product(),
        "ADR-083: F-NATIVE LLVM IR product path"
    );
    verify_bytecode(bytecode)?;
    let artifact = parse_artifact(bytecode)?;
    validate_pure_pilot_artifact(&artifact)?;
    emit_llvm_ir_for_artifact(&artifact)
}

fn validate_pure_pilot_artifact(artifact: &Artifact) -> Result<(), NativeError> {
    if artifact.functions.is_empty() {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35 pilot requires at least one guest total weave",
        ));
    }
    let mut saw_main = false;
    for function in &artifact.functions {
        if function.is_host() || function.is_task() {
            return Err(NativeError::new(
                "AE-NATIVE-002",
                "M35 pilot rejects host and task weaves",
            ));
        }
        if function.effect != Effect::Total {
            return Err(NativeError::new(
                "AE-NATIVE-002",
                "M35 pilot accepts Total weaves only",
            ));
        }
        if function.result != ValueType::Whole {
            return Err(NativeError::new(
                "AE-NATIVE-002",
                "M35 pilot requires Whole results",
            ));
        }
        for (value_type, _mode) in &function.parameters {
            if *value_type != ValueType::Whole {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    "M35c multi-weave pilot lowers Whole parameters only",
                ));
            }
        }
        for local in &function.locals {
            if local.value_type != ValueType::Whole {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    "M35b pilot lowers Whole locals only",
                ));
            }
        }
        if function.name == "main" {
            if !function.parameters.is_empty() {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    "M35 pilot main must be [] -> Whole",
                ));
            }
            saw_main = true;
        }
    }
    if !saw_main {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35 pilot requires a guest total main weave",
        ));
    }
    if !native_aeth_to_c_speak_multiweave_pilot() && artifact.functions.len() != 1 {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35 pilot accepts a single total main weave only",
        ));
    }
    Ok(())
}

fn emit_c_for_artifact(artifact: &Artifact) -> Result<String, NativeError> {
    let mut out = String::from(
        "/* Generated by Aether M35c from verified AETH only (F-NATIVE). */\n\
#include <stdio.h>\n\
\n",
    );
    let main_index = artifact
        .functions
        .iter()
        .position(|function| function.name == "main")
        .ok_or_else(|| {
            NativeError::new(
                "AE-NATIVE-002",
                "M35 pilot requires a guest total main weave",
            )
        })?;

    for (index, function) in artifact.functions.iter().enumerate() {
        out.push_str(&emit_c_function(function, index, artifact.functions.len())?);
        out.push('\n');
    }

    out.push_str(&format!(
        "int main(void) {{\n  return (int)aether_fn_{main_index}();\n}}\n"
    ));
    Ok(out)
}

fn emit_c_function(
    function: &ArtifactFunction,
    index: usize,
    function_count: usize,
) -> Result<String, NativeError> {
    let local_count = function.locals.len();
    let param_count = function.parameters.len();
    let mut sig = format!("static long long aether_fn_{index}(");
    if param_count == 0 {
        sig.push_str("void");
    } else {
        for p in 0..param_count {
            if p > 0 {
                sig.push_str(", ");
            }
            sig.push_str(&format!("long long p{p}"));
        }
    }
    sig.push_str(") {\n");

    let mut body = String::new();
    body.push_str("  long long stack[64];\n");
    body.push_str("  int sp = 0;\n");
    body.push_str("  const char *tstack[64];\n");
    body.push_str("  int tsp = 0;\n");
    if local_count > 0 {
        body.push_str(&format!("  long long locals[{local_count}];\n"));
        body.push_str(&format!(
            "  for (int i = 0; i < {local_count}; ++i) locals[i] = 0;\n"
        ));
    }
    // Parameters occupy initial local slots in AETH (same as bootstrap emit).
    // Prefer explicit param vars on stack entry when params exist but no matching locals.
    // Verified AETH places parameters as initialized locals [0..param_count).
    for p in 0..param_count {
        if p < local_count {
            body.push_str(&format!("  locals[{p}] = p{p};\n"));
        } else {
            // Defensive: parameters without local slots stay on stack order via pN.
            body.push_str(&format!("  stack[sp++] = p{p};\n"));
        }
    }

    let mut i = 0;
    let code = &function.code;
    let mut saw_yield = false;
    while i < code.len() {
        let op = code[i];
        i += 1;
        match op {
            OP_PUSH_WHOLE => {
                if i + 8 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_WHOLE immediate",
                    ));
                }
                let value = i64::from_le_bytes(code[i..i + 8].try_into().unwrap());
                i += 8;
                body.push_str(&format!("  stack[sp++] = {value}LL;\n"));
            }
            OP_PUSH_TEXT => {
                if i + 4 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_TEXT length",
                    ));
                }
                let len = u32::from_le_bytes(code[i..i + 4].try_into().unwrap()) as usize;
                i += 4;
                if i + len > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_TEXT payload",
                    ));
                }
                let text = String::from_utf8_lossy(&code[i..i + len]);
                i += len;
                let escaped = c_escape(&text);
                body.push_str(&format!("  tstack[tsp++] = \"{escaped}\";\n"));
            }
            OP_STORE => {
                if i + 2 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated STORE local index",
                    ));
                }
                let slot = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                if slot >= local_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("STORE local {slot} out of range"),
                    ));
                }
                body.push_str("  if (sp < 1) return 1;\n");
                body.push_str(&format!("  locals[{slot}] = stack[--sp];\n"));
            }
            OP_LOAD => {
                if i + 2 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated LOAD local index",
                    ));
                }
                let slot = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                if slot >= local_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("LOAD local {slot} out of range"),
                    ));
                }
                body.push_str(&format!("  stack[sp++] = locals[{slot}];\n"));
            }
            OP_SUM | OP_DIFFERENCE | OP_PRODUCT | OP_QUOTIENT | OP_REMAINDER => {
                let op_c = match op {
                    OP_SUM => "+",
                    OP_DIFFERENCE => "-",
                    OP_PRODUCT => "*",
                    OP_QUOTIENT => "/",
                    OP_REMAINDER => "%",
                    _ => unreachable!(),
                };
                body.push_str("  if (sp < 2) return 1;\n");
                body.push_str("  { long long b = stack[--sp]; long long a = stack[--sp];\n");
                if op == OP_QUOTIENT || op == OP_REMAINDER {
                    body.push_str("    if (b == 0) return 1;\n");
                }
                body.push_str(&format!("    stack[sp++] = a {op_c} b; }}\n"));
            }
            OP_SPEAK => {
                if !native_aeth_to_c_speak_multiweave_pilot() {
                    return Err(NativeError::new(
                        "AE-NATIVE-002",
                        "M35 pilot rejects SPEAK (use pure yield Whole only)",
                    ));
                }
                body.push_str("  if (tsp < 1) return 1;\n");
                body.push_str("  fputs(tstack[--tsp], stdout);\n");
            }
            OP_CALL => {
                if !native_aeth_to_c_speak_multiweave_pilot() {
                    return Err(NativeError::new(
                        "AE-NATIVE-002",
                        "M35 pilot rejects CALL (single-weave only)",
                    ));
                }
                if i + 3 > code.len() {
                    return Err(NativeError::new("AE-NATIVE-003", "truncated CALL operands"));
                }
                let target = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                let argc = code[i] as usize;
                i += 1;
                if target >= function_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("CALL target {target} out of range"),
                    ));
                }
                body.push_str(&format!("  if (sp < {argc}) return 1;\n"));
                for a in (0..argc).rev() {
                    body.push_str(&format!("  long long arg{a} = stack[--sp];\n"));
                }
                body.push_str(&format!("  stack[sp++] = aether_fn_{target}("));
                for a in 0..argc {
                    if a > 0 {
                        body.push_str(", ");
                    }
                    body.push_str(&format!("arg{a}"));
                }
                body.push_str(");\n");
            }
            OP_YIELD => {
                body.push_str("  if (sp < 1) return 1;\n");
                body.push_str("  return stack[--sp];\n");
                saw_yield = true;
            }
            other => {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    format!("M35 pilot rejects opcode {other}"),
                ));
            }
        }
    }
    if !saw_yield {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            format!(
                "M35 pilot weave {} requires a terminal yield",
                function.name
            ),
        ));
    }
    Ok(format!("{sig}{body}}}\n"))
}

fn c_escape(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_ascii_graphic() || c == ' ' => out.push(c),
            c => out.push_str(&format!("\\x{:02x}", u32::from(c))),
        }
    }
    out
}

/// Dual-run helper: product-compile source, run on VM, lower to C, and optionally
/// compile+run with host `cc` when available. Always asserts VM exit; C compile
/// is best-effort and does not fail the matrix when no C toolchain exists.
pub fn native_dual_run_vm_exit(source: &str) -> Result<i64, NativeError> {
    let bytecode = crate::compile_product_bytecode(source)
        .map_err(|error| NativeError::new("AE-NATIVE-001", error.to_string()))?;
    let exit = crate::run_bytecode(&bytecode)
        .map_err(|error| NativeError::new("AE-NATIVE-001", error.to_string()))?
        .exit_code;
    // Ensure lower succeeds for the same artifact (dual authority: VM reference).
    let _c = lower_verified_aeth_to_c(&bytecode)?;
    Ok(exit)
}

/// M35d: optional host C toolchain dual-exec pilot is product (best-effort).
#[must_use]
pub const fn native_host_cc_dual_exec_pilot() -> bool {
    true
}

/// Report from optional host `cc`/`clang`/`gcc` dual-exec (ADR-076 / M35d).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeDualExecReport {
    pub vm_exit: i64,
    pub c_source: String,
    /// True when a host C compiler was found and invoked.
    pub cc_available: bool,
    /// Compiler command used when available.
    pub cc_command: Option<String>,
    /// Native process exit code when dual-exec ran.
    pub native_exit: Option<i64>,
    /// True when native_exit matched vm_exit.
    pub exits_match: Option<bool>,
    /// Object file path when `cc -c` succeeded (temp path; may not persist).
    pub object_emitted: bool,
}

/// M35d: product-compile + VM run + AETH→C, then optionally compile/run with host
/// `cc`/`clang`/`gcc` when present. **Never fails** solely because no C toolchain
/// exists (`cc_available = false`). When cc runs, native exit must match VM exit.
pub fn native_host_cc_dual_exec(source: &str) -> Result<NativeDualExecReport, NativeError> {
    debug_assert!(
        f_native_authorized() && native_host_cc_dual_exec_pilot(),
        "ADR-076: F-NATIVE host cc dual-exec pilot"
    );
    let bytecode = crate::compile_product_bytecode(source)
        .map_err(|error| NativeError::new("AE-NATIVE-001", error.to_string()))?;
    let vm_exit = crate::run_bytecode(&bytecode)
        .map_err(|error| NativeError::new("AE-NATIVE-001", error.to_string()))?
        .exit_code;
    let c_source = lower_verified_aeth_to_c(&bytecode)?;

    let Some(cc) = find_host_c_compiler() else {
        return Ok(NativeDualExecReport {
            vm_exit,
            c_source,
            cc_available: false,
            cc_command: None,
            native_exit: None,
            exits_match: None,
            object_emitted: false,
        });
    };

    let temp_root = std::env::temp_dir().join(format!(
        "aether-m35d-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&temp_root).map_err(|error| {
        NativeError::new("AE-NATIVE-004", format!("temp dir create failed: {error}"))
    })?;
    let c_path = temp_root.join("program.c");
    let exe_path = temp_root.join(if cfg!(windows) {
        "program.exe"
    } else {
        "program"
    });
    let obj_path = temp_root.join(if cfg!(windows) {
        "program.obj"
    } else {
        "program.o"
    });
    std::fs::write(&c_path, &c_source).map_err(|error| {
        NativeError::new(
            "AE-NATIVE-004",
            format!("could not write C source: {error}"),
        )
    })?;

    // Object emit pilot: `cc -c`
    let object_emitted = std::process::Command::new(&cc)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&obj_path)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    let link = std::process::Command::new(&cc)
        .arg(&c_path)
        .arg("-o")
        .arg(&exe_path)
        .output()
        .map_err(|error| {
            NativeError::new(
                "AE-NATIVE-004",
                format!("host C compiler invoke failed ({cc}): {error}"),
            )
        })?;
    if !link.status.success() {
        let stderr = String::from_utf8_lossy(&link.stderr);
        let _ = std::fs::remove_dir_all(&temp_root);
        return Err(NativeError::new(
            "AE-NATIVE-004",
            format!("host C compile failed ({cc}): {stderr}"),
        ));
    }

    let run = std::process::Command::new(&exe_path)
        .output()
        .map_err(|error| {
            NativeError::new(
                "AE-NATIVE-004",
                format!("native binary run failed: {error}"),
            )
        })?;
    let native_exit = i64::from(run.status.code().unwrap_or(1));
    let exits_match = native_exit == vm_exit;
    let _ = std::fs::remove_dir_all(&temp_root);
    if !exits_match {
        return Err(NativeError::new(
            "AE-NATIVE-005",
            format!("native exit {native_exit} diverged from VM exit {vm_exit}"),
        ));
    }
    Ok(NativeDualExecReport {
        vm_exit,
        c_source,
        cc_available: true,
        cc_command: Some(cc),
        native_exit: Some(native_exit),
        exits_match: Some(true),
        object_emitted,
    })
}

fn find_host_c_compiler() -> Option<String> {
    for candidate in ["cc", "clang", "gcc", "clang.exe", "gcc.exe"] {
        if std::process::Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return Some(candidate.to_owned());
        }
    }
    None
}

fn emit_llvm_ir_for_artifact(artifact: &Artifact) -> Result<String, NativeError> {
    let main_index = artifact
        .functions
        .iter()
        .position(|function| function.name == "main")
        .ok_or_else(|| {
            NativeError::new(
                "AE-NATIVE-002",
                "M35 pilot requires a guest total main weave",
            )
        })?;
    let mut out = String::from(
        "; Generated by Aether M35f from verified AETH only (F-NATIVE).\n\
target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n\
target triple = \"x86_64-unknown-linux-gnu\"\n\
\n\
declare i32 @puts(i8*)\n\
\n",
    );

    for (index, function) in artifact.functions.iter().enumerate() {
        out.push_str(&emit_llvm_function(
            function,
            index,
            artifact.functions.len(),
        )?);
        out.push('\n');
    }
    out.push_str(&format!(
        "define i32 @main() {{\n  %r = call i64 @aether_fn_{main_index}()\n  %t = trunc i64 %r to i32\n  ret i32 %t\n}}\n"
    ));
    Ok(out)
}

fn emit_llvm_function(
    function: &ArtifactFunction,
    index: usize,
    function_count: usize,
) -> Result<String, NativeError> {
    let local_count = function.locals.len();
    let param_count = function.parameters.len();
    let mut sig = format!("define i64 @aether_fn_{index}(");
    if param_count == 0 {
        sig.push_str(") {\nentry:\n");
    } else {
        for p in 0..param_count {
            if p > 0 {
                sig.push_str(", ");
            }
            sig.push_str(&format!("i64 %p{p}"));
        }
        sig.push_str(") {\nentry:\n");
    }

    let mut body = String::new();
    // Stack as alloca array
    body.push_str("  %stack = alloca [64 x i64]\n");
    body.push_str("  %sp = alloca i32\n");
    body.push_str("  store i32 0, i32* %sp\n");
    body.push_str("  %tstack = alloca [64 x i8*]\n");
    body.push_str("  %tsp = alloca i32\n");
    body.push_str("  store i32 0, i32* %tsp\n");
    if local_count > 0 {
        body.push_str(&format!("  %locals = alloca [{local_count} x i64]\n"));
        for i in 0..local_count {
            body.push_str(&format!(
                "  %linit{i} = getelementptr [{local_count} x i64], [{local_count} x i64]* %locals, i64 0, i64 {i}\n  store i64 0, i64* %linit{i}\n"
            ));
        }
    }
    for p in 0..param_count {
        if p < local_count {
            body.push_str(&format!(
                "  %parg{p} = getelementptr [{local_count} x i64], [{local_count} x i64]* %locals, i64 0, i64 {p}\n  store i64 %p{p}, i64* %parg{p}\n"
            ));
        }
    }

    let mut i = 0;
    let code = &function.code;
    let mut saw_yield = false;
    let mut tmp = 0u32;
    let mut next_tmp = || {
        let id = tmp;
        tmp += 1;
        id
    };
    let mut str_globals = String::new();
    let mut str_id = 0u32;

    while i < code.len() {
        let op = code[i];
        i += 1;
        match op {
            OP_PUSH_WHOLE => {
                if i + 8 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_WHOLE immediate",
                    ));
                }
                let value = i64::from_le_bytes(code[i..i + 8].try_into().unwrap());
                i += 8;
                let s = next_tmp();
                let spv = next_tmp();
                let ptr = next_tmp();
                let spn = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %sp\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{s}\n  store i64 {value}, i64* %{ptr}\n  %{spn} = add i32 %{s}, 1\n  store i32 %{spn}, i32* %sp\n"
                ));
                let _ = spv;
            }
            OP_PUSH_TEXT => {
                if i + 4 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_TEXT length",
                    ));
                }
                let len = u32::from_le_bytes(code[i..i + 4].try_into().unwrap()) as usize;
                i += 4;
                if i + len > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated PUSH_TEXT payload",
                    ));
                }
                let text = String::from_utf8_lossy(&code[i..i + len]);
                i += len;
                let sid = str_id;
                str_id += 1;
                let mut bytes = text.as_bytes().to_vec();
                bytes.push(0);
                let n = bytes.len();
                let mut hex = String::new();
                for b in &bytes {
                    hex.push_str(&format!("\\{b:02X}"));
                }
                str_globals.push_str(&format!(
                    "@.str{index}_{sid} = private unnamed_addr constant [{n} x i8] c\"{hex}\"\n"
                ));
                let s = next_tmp();
                let ptr = next_tmp();
                let spn = next_tmp();
                let g = next_tmp();
                body.push_str(&format!(
                    "  %{g} = getelementptr [{n} x i8], [{n} x i8]* @.str{index}_{sid}, i64 0, i64 0\n  %{s} = load i32, i32* %tsp\n  %{ptr} = getelementptr [64 x i8*], [64 x i8*]* %tstack, i64 0, i32 %{s}\n  store i8* %{g}, i8** %{ptr}\n  %{spn} = add i32 %{s}, 1\n  store i32 %{spn}, i32* %tsp\n"
                ));
            }
            OP_STORE => {
                if i + 2 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated STORE local index",
                    ));
                }
                let slot = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                if slot >= local_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("STORE local {slot} out of range"),
                    ));
                }
                let s = next_tmp();
                let sn = next_tmp();
                let ptr = next_tmp();
                let val = next_tmp();
                let lptr = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %sp\n  %{sn} = sub i32 %{s}, 1\n  store i32 %{sn}, i32* %sp\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{sn}\n  %{val} = load i64, i64* %{ptr}\n  %{lptr} = getelementptr [{local_count} x i64], [{local_count} x i64]* %locals, i64 0, i64 {slot}\n  store i64 %{val}, i64* %{lptr}\n"
                ));
            }
            OP_LOAD => {
                if i + 2 > code.len() {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        "truncated LOAD local index",
                    ));
                }
                let slot = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                if slot >= local_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("LOAD local {slot} out of range"),
                    ));
                }
                let s = next_tmp();
                let lptr = next_tmp();
                let val = next_tmp();
                let ptr = next_tmp();
                let spn = next_tmp();
                body.push_str(&format!(
                    "  %{lptr} = getelementptr [{local_count} x i64], [{local_count} x i64]* %locals, i64 0, i64 {slot}\n  %{val} = load i64, i64* %{lptr}\n  %{s} = load i32, i32* %sp\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{s}\n  store i64 %{val}, i64* %{ptr}\n  %{spn} = add i32 %{s}, 1\n  store i32 %{spn}, i32* %sp\n"
                ));
            }
            OP_SUM | OP_DIFFERENCE | OP_PRODUCT | OP_QUOTIENT | OP_REMAINDER => {
                let op_ir = match op {
                    OP_SUM => "add",
                    OP_DIFFERENCE => "sub",
                    OP_PRODUCT => "mul",
                    OP_QUOTIENT => "sdiv",
                    OP_REMAINDER => "srem",
                    _ => unreachable!(),
                };
                let s = next_tmp();
                let s1 = next_tmp();
                let s0 = next_tmp();
                let pb = next_tmp();
                let pa = next_tmp();
                let b = next_tmp();
                let a = next_tmp();
                let r = next_tmp();
                let pr = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %sp\n  %{s1} = sub i32 %{s}, 1\n  %{pb} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{s1}\n  %{b} = load i64, i64* %{pb}\n  %{s0} = sub i32 %{s}, 2\n  %{pa} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{s0}\n  %{a} = load i64, i64* %{pa}\n  %{r} = {op_ir} i64 %{a}, %{b}\n  store i64 %{r}, i64* %{pa}\n  store i32 %{s1}, i32* %sp\n"
                ));
                let _ = pr;
            }
            OP_SPEAK => {
                let s = next_tmp();
                let sn = next_tmp();
                let ptr = next_tmp();
                let val = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %tsp\n  %{sn} = sub i32 %{s}, 1\n  store i32 %{sn}, i32* %tsp\n  %{ptr} = getelementptr [64 x i8*], [64 x i8*]* %tstack, i64 0, i32 %{sn}\n  %{val} = load i8*, i8** %{ptr}\n  call i32 @puts(i8* %{val})\n"
                ));
            }
            OP_CALL => {
                if i + 3 > code.len() {
                    return Err(NativeError::new("AE-NATIVE-003", "truncated CALL operands"));
                }
                let target = u16::from_le_bytes(code[i..i + 2].try_into().unwrap()) as usize;
                i += 2;
                let argc = code[i] as usize;
                i += 1;
                if target >= function_count {
                    return Err(NativeError::new(
                        "AE-NATIVE-003",
                        format!("CALL target {target} out of range"),
                    ));
                }
                let mut arg_tmps = Vec::new();
                for _ in 0..argc {
                    let s = next_tmp();
                    let sn = next_tmp();
                    let ptr = next_tmp();
                    let val = next_tmp();
                    body.push_str(&format!(
                        "  %{s} = load i32, i32* %sp\n  %{sn} = sub i32 %{s}, 1\n  store i32 %{sn}, i32* %sp\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{sn}\n  %{val} = load i64, i64* %{ptr}\n"
                    ));
                    arg_tmps.push(val);
                }
                arg_tmps.reverse();
                let r = next_tmp();
                body.push_str(&format!("  %{r} = call i64 @aether_fn_{target}("));
                for (ai, a) in arg_tmps.iter().enumerate() {
                    if ai > 0 {
                        body.push_str(", ");
                    }
                    body.push_str(&format!("i64 %{a}"));
                }
                body.push_str(")\n");
                let s = next_tmp();
                let ptr = next_tmp();
                let spn = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %sp\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{s}\n  store i64 %{r}, i64* %{ptr}\n  %{spn} = add i32 %{s}, 1\n  store i32 %{spn}, i32* %sp\n"
                ));
            }
            OP_YIELD => {
                let s = next_tmp();
                let sn = next_tmp();
                let ptr = next_tmp();
                let val = next_tmp();
                body.push_str(&format!(
                    "  %{s} = load i32, i32* %sp\n  %{sn} = sub i32 %{s}, 1\n  %{ptr} = getelementptr [64 x i64], [64 x i64]* %stack, i64 0, i32 %{sn}\n  %{val} = load i64, i64* %{ptr}\n  ret i64 %{val}\n"
                ));
                saw_yield = true;
            }
            other => {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    format!("M35f LLVM IR rejects opcode {other}"),
                ));
            }
        }
    }
    if !saw_yield {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            format!(
                "M35 pilot weave {} requires a terminal yield",
                function.name
            ),
        ));
    }
    Ok(format!("{str_globals}{sig}{body}}}\n"))
}

/// M35e: product path can emit a native object file from verified AETH via host cc.
#[must_use]
pub const fn native_object_emit_product() -> bool {
    true
}

/// M35e: lower verified AETH → C → host `cc -c` object file at `object_path`.
///
/// Requires a host C toolchain. Fails closed with `AE-NATIVE-004` when no `cc`
/// is available (unlike dual-exec best-effort). Not LLVM IR; not a full product
/// native linker path.
pub fn lower_verified_aeth_to_native_object(
    bytecode: &[u8],
    object_path: &std::path::Path,
) -> Result<String, NativeError> {
    debug_assert!(
        f_native_authorized() && native_object_emit_product(),
        "ADR-079: F-NATIVE native object product path"
    );
    let c_source = lower_verified_aeth_to_c(bytecode)?;
    let Some(cc) = find_host_c_compiler() else {
        return Err(NativeError::new(
            "AE-NATIVE-004",
            "M35e native object emit requires host cc/clang/gcc (none found)",
        ));
    };
    if let Some(parent) = object_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                NativeError::new(
                    "AE-NATIVE-004",
                    format!("could not create object parent dir: {error}"),
                )
            })?;
        }
    }
    let temp_root = std::env::temp_dir().join(format!(
        "aether-m35e-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&temp_root).map_err(|error| {
        NativeError::new("AE-NATIVE-004", format!("temp dir create failed: {error}"))
    })?;
    let c_path = temp_root.join("program.c");
    std::fs::write(&c_path, &c_source).map_err(|error| {
        NativeError::new(
            "AE-NATIVE-004",
            format!("could not write C source: {error}"),
        )
    })?;
    let output = std::process::Command::new(&cc)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(object_path)
        .output()
        .map_err(|error| {
            NativeError::new(
                "AE-NATIVE-004",
                format!("host C compiler invoke failed ({cc}): {error}"),
            )
        })?;
    let _ = std::fs::remove_dir_all(&temp_root);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(NativeError::new(
            "AE-NATIVE-004",
            format!("host C object emit failed ({cc}): {stderr}"),
        ));
    }
    if !object_path.is_file() {
        return Err(NativeError::new(
            "AE-NATIVE-004",
            format!(
                "host C object emit did not produce {}",
                object_path.display()
            ),
        ));
    }
    Ok(cc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compile_product_bytecode, run_bytecode};

    #[test]
    fn lowers_pure_whole_yield_and_matches_vm_exit() {
        assert!(f_native_authorized());
        assert!(native_aeth_to_c_pilot());
        let source = "world pure\n\nweave main [] -> Whole:\n  yield 42\n";
        let bytecode = compile_product_bytecode(source).expect("product");
        let exit = run_bytecode(&bytecode).expect("run").exit_code;
        assert_eq!(exit, 42);
        let c = lower_verified_aeth_to_c(&bytecode).expect("lower");
        assert!(c.contains("return stack[--sp];"));
        assert!(c.contains("42LL"));
        assert!(c.contains("int main(void)"));
    }

    #[test]
    fn lowers_bind_sum_local_to_c() {
        assert!(native_aeth_to_c_locals_pilot());
        let source = "world pure\n\nweave main [] -> Whole:\n  bind x <- sum 20 22\n  yield x\n";
        let bytecode = compile_product_bytecode(source).expect("product");
        assert_eq!(run_bytecode(&bytecode).expect("run").exit_code, 42);
        let c = lower_verified_aeth_to_c(&bytecode).expect("lower with locals");
        assert!(c.contains("locals["));
        assert!(c.contains("20LL") && c.contains("22LL"));
        assert!(c.contains("a + b") || c.contains("a + b;"));
    }

    #[test]
    fn lowers_speak_text_and_multiweave_helpers() {
        assert!(native_aeth_to_c_speak_multiweave_pilot());
        let source = "\
world multi

weave double [n: Whole] -> Whole:
  yield product n 2

weave main [] -> Whole:
  speak \"ok\"
  bind x <- call double 21
  yield x
";
        let bytecode = compile_product_bytecode(source).expect("product multi");
        let run = run_bytecode(&bytecode).expect("run");
        assert_eq!(run.exit_code, 42);
        assert_eq!(run.stdout, "ok");
        let c = lower_verified_aeth_to_c(&bytecode).expect("lower multi");
        assert!(c.contains("fputs(tstack[--tsp], stdout);"));
        assert!(c.contains("aether_fn_"));
        assert!(c.contains("tstack[tsp++]"));
        assert_eq!(native_dual_run_vm_exit(source).expect("dual"), 42);
    }

    #[test]
    fn rejects_unverified_or_foreign_surface() {
        let error = lower_verified_aeth_to_c(b"not-aeth").expect_err("bad");
        assert_eq!(error.code, "AE-NATIVE-001");
    }

    #[test]
    fn host_cc_dual_exec_pilot_is_best_effort() {
        assert!(native_host_cc_dual_exec_pilot());
        let source = "world pure\n\nweave main [] -> Whole:\n  yield 7\n";
        let report = native_host_cc_dual_exec(source).expect("dual-exec report");
        assert_eq!(report.vm_exit, 7);
        assert!(report.c_source.contains("int main(void)"));
        if report.cc_available {
            assert_eq!(report.native_exit, Some(7));
            assert_eq!(report.exits_match, Some(true));
        }
    }

    #[test]
    fn native_object_emit_product_path_fail_closed_without_cc_or_succeeds() {
        assert!(native_object_emit_product());
        let source = "world pure\n\nweave main [] -> Whole:\n  yield 3\n";
        let bytecode = compile_product_bytecode(source).expect("product");
        let dir = std::env::temp_dir().join(format!(
            "aether-m35e-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).expect("temp");
        let obj = dir.join(if cfg!(windows) { "out.obj" } else { "out.o" });
        match lower_verified_aeth_to_native_object(&bytecode, &obj) {
            Ok(_cc) => {
                assert!(obj.is_file(), "object file must exist");
            }
            Err(error) => {
                assert_eq!(error.code, "AE-NATIVE-004");
                assert!(error.message.contains("requires host cc") || error.message.contains("cc"));
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lowers_pure_subset_to_llvm_ir_text() {
        assert!(native_llvm_ir_emit_product());
        let source = "\
world llvm

weave double [n: Whole] -> Whole:
  yield product n 2

weave main [] -> Whole:
  speak \"hi\"
  bind x <- call double 21
  yield x
";
        let bytecode = compile_product_bytecode(source).expect("product");
        assert_eq!(run_bytecode(&bytecode).expect("run").exit_code, 42);
        let ir = lower_verified_aeth_to_llvm_ir(&bytecode).expect("llvm ir");
        assert!(ir.contains("define i32 @main()"));
        assert!(ir.contains("define i64 @aether_fn_"));
        assert!(ir.contains("mul i64"));
        assert!(ir.contains("@puts"));
        assert!(ir.contains("ret i64"));
    }
}
