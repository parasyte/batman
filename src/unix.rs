//! Signal handling for unix-like systems.

use crate::backtrace::Backtrace;
use libc::{c_int, sighandler_t, strsignal};
use std::ffi::{CStr, CString};

/// Installs a global signal handler for catching floating point exceptions.
///
/// This function is guaranteed to be called only once per process.
///
/// # Safety
///
/// This function mutates global state (namely signal handlers).
pub(crate) unsafe fn install_signal_handler() {
    libc::signal(libc::SIGFPE, sigfpe_handler as sighandler_t);
}

/// This is the signal handler that we install.
///
/// When this is called, it prints a backtrace (unconditionally) and aborts.
extern "C" fn sigfpe_handler(signo: c_int) -> ! {
    let signame = sigabbr(signo);
    let sigdesc = unsafe { CStr::from_ptr(strsignal(signo)) };

    let bt = Backtrace::new().as_string();
    let msg = CString::new(format!(
        "Caught signal {} ({}): {}\n{}",
        signo,
        signame,
        sigdesc.to_str().unwrap(),
        bt,
    ))
    .expect("Unable to create CString");
    unsafe { libc::puts(msg.as_ptr()) };

    std::process::abort();
}

/// Get the abbreviated name for a signal as a string slice.
fn sigabbr(signo: c_int) -> &'static str {
    match signo {
        libc::SIGHUP => "SIGHUP",
        libc::SIGINT => "SIGINT",
        libc::SIGQUIT => "SIGQUIT",
        libc::SIGILL => "SIGILL",
        libc::SIGABRT => "SIGABRT",
        libc::SIGFPE => "SIGFPE",
        libc::SIGSEGV => "SIGSEGV",
        libc::SIGPIPE => "SIGPIPE",
        libc::SIGALRM => "SIGALRM",
        libc::SIGTERM => "SIGTERM",
        _ => "UNKNOWN",
    }
}
