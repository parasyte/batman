//! x87 and SSE FP exceptions can be enabled by "unmasking".
//! See: AMD64 Architecture Programmer’s Manual: Volume 2:
//!
//! # 8.1.5 Masking Floating-Point and Media Instructions
//!
//! Any x87 floating-point exceptions can be masked and reported later using bits in the x87
//! floating-point status register (FSW) and the x87 floating-point control register (FCW). The
//! floating-point exception-pending exception is used for unmasked x87 floating-point exceptions
//! (see Section “#MF—x87 Floating-Point Exception-Pending (Vector 16)” on page 248).
//!
//! The SIMD floating-point exception is used for unmasked SSE floating-point exceptions (see Section
//! “#XF—SIMD Floating-Point Exception (Vector 19)” on page 250). SSE floating-point exceptions are
//! masked using the MXCSR register. The exception mechanism is not triggered when these exceptions
//! are masked. Instead, the processor handles the exceptions in a default manner
//!
//! ----
//!
//! 3DNow! does not support FP exceptions (but also does not produce NaNs)
//! See: AMD64 Architecture Programmer’s Manual: Volume 1:
//!
//! # 5.5.6.4 No Support for Infinities, NaNs, and Denormals
//!
//! 64-bit media floating-point instructions support only normalized numbers. They do not support
//! infinity, NaN, and denormalized number representations. Operations on such numbers produce
//! undefined results, and no exceptions are generated. If all source operands are normalized
//! numbers, these instructions never produce infinities, NaNs, or denormalized numbers as results.
//!
//! This aspect of 64-bit media floating-point operations does not comply with the IEEE 754 standard.
//! Software must use only normalized operands and ensure that computations remain within valid
//! normalized-number ranges.
//!
//! # 5.5.6.5 No Support for Floating-Point Exceptions
//!
//! The 64-bit media floating-point instructions do not generate floating-point exceptions. Software
//! must ensure that in-range operands are provided to these instructions.

// This is not actually dead code?
// Needed because cargo check doesn't notice we are using the constants in global_asm!()
#![allow(dead_code)]

use core::arch::global_asm;

/// Zero-Divide Exception Mask (x87)
const FCW_ZM: u16 = 1 << 2;
/// Invalid Operation Exception Mask (x87)
const FCW_IM: u16 = 1 << 0;
/// Unmask (enable) x87 exceptions
const FCW_UNMASK: u16 = !(FCW_ZM | FCW_IM);

/// Zero-Divide Exception Mask (SSE)
const MXCSR_ZM: u32 = 1 << 9;
/// Invalid Operation Exception Mask (SSE)
const MXCSR_IM: u32 = 1 << 7;
/// Zero-Divide Exception (SSE)
const MXCSR_ZE: u32 = 1 << 2;
/// Invalid Operation Exception (SSE)
const MXCSR_IE: u32 = 1 << 0;
/// Clear and unmask (enable) SSE FP exceptions
const MXCSR_UNMASK: u32 = !(MXCSR_ZM | MXCSR_IM | MXCSR_ZE | MXCSR_IE);

global_asm!(
    ".global enable_fp_exceptions",
    "enable_fp_exceptions:",

    // Clear and enable SSE FP exceptions
    "stmxcsr    dword ptr [rsp-4]",
    "and        dword ptr [rsp-4], {mxcsr}",
    "ldmxcsr    dword ptr [rsp-4]",

    // Clear and enable x87 exceptions
    "fclex",
    "fstcw      word ptr [rsp-8]",
    "and        word ptr [rsp-8], {fcw}",
    "fldcw      word ptr [rsp-8]",

    "ret",

    mxcsr = const MXCSR_UNMASK,
    fcw = const FCW_UNMASK,
);

extern "C" {
    /// Enable floating point exceptions on the current thread.
    pub fn enable_fp_exceptions();
}
