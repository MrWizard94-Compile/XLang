//! Pilot C-ABI symbols for Aether M21 foreign-weave tests.
//!
//! These are intentional, tiny, copy-primitive exports only. They are not a
//! general foreign surface and do not claim memory safety for arbitrary native code.

// no_mangle is required for a stable C ABI export used by the M21 pilot.
#![allow(unsafe_code)]

/// Whole -> Whole: add one (wrapping). Symbol name is pinned in foreign weave tests.
#[no_mangle]
pub extern "C" fn aether_whole_inc(value: i64) -> i64 {
    value.wrapping_add(1)
}

/// Whole, Whole -> Whole: sum (wrapping).
#[no_mangle]
pub extern "C" fn aether_whole_sum(left: i64, right: i64) -> i64 {
    left.wrapping_add(right)
}
