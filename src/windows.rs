//! Exception handling for Windows systems.

use crate::backtrace::Backtrace;
use std::{
    ffi::CString,
    sync::atomic::{AtomicUsize, Ordering},
};

static HANDLE: AtomicUsize = AtomicUsize::new(0);

/// Installs a global exception handler for catching floating point exceptions.
///
/// This function is guaranteed to be called only once per process.
///
/// # Safety
///
/// This function mutates global state (namely vectored exception handlers).
pub(crate) unsafe fn install_exception_handler() {
    let handle = AddVectoredExceptionHandler(1, exception_handler as usize);
    HANDLE.store(handle, Ordering::Relaxed);
}

extern "C" {
    fn AddVectoredExceptionHandler(first: u32, handler: usize) -> usize;
    fn RemoveVectoredExceptionHandler(handle: usize) -> u32;
}

/// This is the signal handler that we install.
///
/// When this is called, it prints a backtrace (unconditionally) and aborts.
extern "C" fn exception_handler(_info: usize) -> ! {
    // This isn't strictly necessary
    let handle = HANDLE.load(Ordering::Relaxed);
    unsafe { RemoveVectoredExceptionHandler(handle) };

    // TODO: Include some useful info about the exception from the `info` pointer
    let bt = Backtrace::new().as_string();
    let msg = CString::new(format!("Caught exception\n{}", bt)).expect("Unable to create CString");
    unsafe { libc::puts(msg.as_ptr()) };

    std::process::abort();
}
