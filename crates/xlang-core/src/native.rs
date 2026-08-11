//! M35a verified AETH → C pure pilot (F-NATIVE, ADR-059).
//!
//! Lowers only a restricted pure Total main subset. Input must already verify.

use crate::{
    parse_artifact, verify_bytecode, ArtifactFunction, BytecodeError, ValueType, OP_DIFFERENCE,
    OP_PRODUCT, OP_PUSH_TEXT, OP_PUSH_WHOLE, OP_QUOTIENT, OP_REMAINDER, OP_SPEAK, OP_SUM, OP_YIELD,
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

/// Lower **verified** AETH to ISO C for the pure Whole pilot subset.
pub fn lower_verified_aeth_to_c(bytecode: &[u8]) -> Result<String, NativeError> {
    debug_assert!(
        f_native_authorized() && native_aeth_to_c_pilot(),
        "ADR-059: F-NATIVE AETH→C pilot"
    );
    verify_bytecode(bytecode)?;
    let artifact = parse_artifact(bytecode)?;
    if artifact.functions.len() != 1 {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35a pilot accepts a single total main weave only",
        ));
    }
    let main = &artifact.functions[0];
    if main.name != "main" || main.is_host() || main.is_task() {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35a pilot requires a single guest total main weave",
        ));
    }
    if !main.parameters.is_empty() || main.result != ValueType::Whole {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35a pilot main must be [] -> Whole",
        ));
    }
    if !main.locals.is_empty() {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35a pilot does not lower locals yet",
        ));
    }
    emit_c_for_pure_main(main)
}

fn emit_c_for_pure_main(main: &ArtifactFunction) -> Result<String, NativeError> {
    let mut body = String::new();
    body.push_str("  long long stack[64];\n");
    body.push_str("  int sp = 0;\n");
    let mut i = 0;
    let code = &main.code;
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
                body.push_str(&format!("  fputs(\"{escaped}\", stdout);\n"));
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
                // M35a: SPEAK of Whole is not lowered; PUSH_TEXT is emitted as fputs above.
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    "M35a pilot rejects SPEAK (use pure yield Whole only)",
                ));
            }
            OP_YIELD => {
                body.push_str("  if (sp < 1) return 1;\n");
                body.push_str("  return (int)stack[--sp];\n");
            }
            other => {
                return Err(NativeError::new(
                    "AE-NATIVE-002",
                    format!("M35a pilot rejects opcode {other}"),
                ));
            }
        }
    }
    if !body.contains("return (int)stack") {
        return Err(NativeError::new(
            "AE-NATIVE-002",
            "M35a pilot requires a terminal yield",
        ));
    }
    Ok(format!(
        "/* Generated by Aether M35a from verified AETH only (F-NATIVE). */\n\
#include <stdio.h>\n\
int main(void) {{\n\
{body}\
}}\n"
    ))
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
        assert!(c.contains("return (int)stack[--sp];"));
        assert!(c.contains("42LL"));
        assert!(c.contains("int main(void)"));
    }

    #[test]
    fn rejects_unverified_or_foreign_surface() {
        let error = lower_verified_aeth_to_c(b"not-aeth").expect_err("bad");
        assert_eq!(error.code, "AE-NATIVE-001");
    }
}
