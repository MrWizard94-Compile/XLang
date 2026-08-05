//! M21 foreign ABI pilot — host-side libloading after operator library grant.
//!
//! Residual risk of native code is accepted under ADR-025 + human authorize.
//! This module is the only place `unsafe` is allowed in aether-core.

#![allow(unsafe_code)]

use std::path::Path;

use libloading::{Library, Symbol};

use crate::{BytecodeError, RuntimeValue, ValueType};

/// Max Whole parameters for the pilot (fixed C calling conventions).
pub const MAX_FOREIGN_WHOLE_ARGS: usize = 4;

/// Invoke a pinned C-ABI symbol with Whole-only arguments and Whole result.
pub fn invoke_whole_symbol(
    library_path: &Path,
    symbol: &str,
    arguments: &[RuntimeValue],
    offset: usize,
) -> Result<RuntimeValue, BytecodeError> {
    if arguments.len() > MAX_FOREIGN_WHOLE_ARGS {
        return Err(BytecodeError::new(
            offset,
            format!(
                "AE-FFI-001: foreign pilot supports at most {MAX_FOREIGN_WHOLE_ARGS} Whole arguments"
            ),
        ));
    }
    let mut wholes = Vec::with_capacity(arguments.len());
    for argument in arguments {
        let RuntimeValue::Whole(value) = argument else {
            return Err(BytecodeError::new(
                offset,
                "AE-FFI-001: foreign pilot arguments must be Whole",
            ));
        };
        wholes.push(*value);
    }

    // SAFETY: Operator granted this library path (M21 residual risk accepted).
    // Symbol is caller-pinned in source. Lifetime of Symbol is bounded by Library.
    let library = unsafe { Library::new(library_path) }.map_err(|error| {
        BytecodeError::new(
            offset,
            format!(
                "AE-FFI-003: could not load foreign library {}: {error}",
                library_path.display()
            ),
        )
    })?;

    let symbol_c = format!("{symbol}\0");
    let result = match wholes.len() {
        0 => {
            let func: Symbol<unsafe extern "C" fn() -> i64> =
                unsafe { library.get(symbol_c.as_bytes()) }.map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-FFI-003: missing foreign symbol {symbol}: {error}"),
                    )
                })?;
            unsafe { func() }
        }
        1 => {
            let func: Symbol<unsafe extern "C" fn(i64) -> i64> =
                unsafe { library.get(symbol_c.as_bytes()) }.map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-FFI-003: missing foreign symbol {symbol}: {error}"),
                    )
                })?;
            unsafe { func(wholes[0]) }
        }
        2 => {
            let func: Symbol<unsafe extern "C" fn(i64, i64) -> i64> =
                unsafe { library.get(symbol_c.as_bytes()) }.map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-FFI-003: missing foreign symbol {symbol}: {error}"),
                    )
                })?;
            unsafe { func(wholes[0], wholes[1]) }
        }
        3 => {
            let func: Symbol<unsafe extern "C" fn(i64, i64, i64) -> i64> =
                unsafe { library.get(symbol_c.as_bytes()) }.map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-FFI-003: missing foreign symbol {symbol}: {error}"),
                    )
                })?;
            unsafe { func(wholes[0], wholes[1], wholes[2]) }
        }
        4 => {
            let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> i64> =
                unsafe { library.get(symbol_c.as_bytes()) }.map_err(|error| {
                    BytecodeError::new(
                        offset,
                        format!("AE-FFI-003: missing foreign symbol {symbol}: {error}"),
                    )
                })?;
            unsafe { func(wholes[0], wholes[1], wholes[2], wholes[3]) }
        }
        _ => unreachable!("argument count checked above"),
    };
    let _ = ValueType::Whole;
    Ok(RuntimeValue::Whole(result))
}
