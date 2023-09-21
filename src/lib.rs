//! Terribly unsafe per-thread trapping exceptions for floating point operations.

#![feature(asm_const)]
#![feature(sync_unsafe_cell)]
#![deny(clippy::all)]

#[cfg(debug_assertions)]
mod stack;

#[cfg(all(debug_assertions, any(target_arch = "x86", target_arch = "x86_64")))]
mod x86_64;

/// Enable hardware floating point exceptions.
///
/// FPE configuration is only allowed once per-thread; subsequent calls will be a no-op. The
/// configuration is thread-local. This function configures the environment in the following manner:
///
/// - FPU "divide by zero" and "invalid operation" exceptions are enabled
///
/// Specifically, `batman` does not concern itself with details like precision loss, rounding
/// behavior, overflow/underflow, or handling subnormal numbers.
///
/// Threads inherit the FPU configuration from their parent (default disabled). Once enabled,
/// exceptions cannot be disabled on the thread (at least not by `batman`; other `unsafe` code
/// can disable exceptions at any time).
///
/// This function is a no-op when debug assertions are disabled.
///
/// # Safety
///
/// This function mutates global state (namely signal handlers and floating point environment
/// configuration).
pub unsafe fn signal() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    {
        use array_macro::array;
        use backtrace::Frame;
        use log::debug;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::{cell::SyncUnsafeCell, thread};

        #[cfg(windows)]
        use windows_sys::Win32::System::Threading;

        // 200 frames is less than 64 KiB on Windows x86_64, and about 6 KiB on Linux/macOS.
        const MAX_FRAMES: usize = 200;

        // Backtrace frames are statically allocated because heap allocations are not safe within
        // signal handlers.
        // See: https://www.man7.org/linux/man-pages/man7/signal-safety.7.html)
        //
        // # SAFETY:
        //
        // `Frame` implements `Send` and `Sync`. `SyncUnsafeCell` is used for interior mutability.
        // All reads and writes are synchronized through the `FRAMES_AVAILABLE` and `FRAMES_HANDLED`
        // beacons.
        static FRAMES: [SyncUnsafeCell<Option<Frame>>; MAX_FRAMES] =
            array![_ => SyncUnsafeCell::new(None); MAX_FRAMES];

        // These atomics are used as beacons to communicate between the signal handler and the
        // thread that prints the backtrace.
        static FRAMES_AVAILABLE: AtomicBool = AtomicBool::new(false);
        static FRAMES_HANDLED: AtomicBool = AtomicBool::new(false);

        // This atomic makes the signal handler reentrant.
        static HANDLING: AtomicBool = AtomicBool::new(false);

        let id = thread::current().id();
        debug!("Enabling FPU exceptions on thread {id:?}");

        // Spawn a thread that can use the standard library. This thread prints the backtrace that
        // it receives from the signal handler.
        let handle = thread::spawn(|| {
            // We're not running in the signal handler, so we can do anything!
            // However, we do need to ensure we are synchronized with the signal handler.
            // Wait for the signal handler to unpark us AND for frames to be available.
            loop {
                thread::park();

                if FRAMES_AVAILABLE.load(Ordering::Acquire) {
                    break;
                }
            }

            debug!("Received beacon, processing backtrace...");

            // Sanity check: If this beacon is set, we're DOA.
            assert!(!FRAMES_HANDLED.load(Ordering::Acquire));

            // Show a pretty backtrace.
            //
            // Note that it is possible for this to deadlock. E.g., if the signalling thread is
            // holding the stderr lock. That's OK, because the signal handler will abort if we don't
            // send a beacon back in a timely manner.
            let mut frames = vec![];
            for frame in FRAMES.iter() {
                // SAFETY: This is the only thread accessing `FRAMES`, guaranteed by the atomic
                // beacons.
                match unsafe { &frame.get().read() } {
                    Some(frame) => frames.push(frame.clone()),
                    None => break,
                }
            }
            stack::print(frames);

            debug!("Sending beacon and stopping thread...");

            // Send a beacon back to the signal handler.
            FRAMES_HANDLED.store(true, Ordering::Release);

            // All done! We can safely exit this thread now.
        });

        // SAFETY: This is a signal handler. It must be written with extreme care. The primary
        // concerns from a POSIX point of view is that signal handlers are not allowed to touch
        // global state unless it is done through synchronization with atomics, the number of
        // libc/syscalls allowed is very limited, and reentrancy must be handled properly.
        //
        // See: https://www.man7.org/linux/man-pages/man7/signal-safety.7.html)
        signal_hook_registry::register_signal_unchecked(libc::SIGFPE, move || {
            let exch = HANDLING.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire);
            if exch.is_err() {
                // The signal handler is already running and another thread has raised the signal.
                // This handler is made reentrant by pausing the thread forever. Execution will
                // abort eventually anyway.

                #[cfg(unix)]
                loop {
                    libc::pause();
                }

                #[cfg(windows)]
                loop {
                    Threading::Sleep(Threading::INFINITE);
                }

                #[cfg(not(any(unix, windows)))]
                libc::abort();
            }

            let mut i = 0;
            // SAFETY: We are certain that this is the only thread that gets here because of the
            // `HANDLING` atomic.
            //
            // Note that we cannot use `std::backtrace` because it allocates on the heap and uses
            // OS primitive locks (which are explicitly forbidden in signal handlers by POSIX).
            // TODO: Make sure the `backtrace::trace_unsynchronized` does not allocate on the heap.
            backtrace::trace_unsynchronized(|frame| {
                // Cap the number of frames captured to fit in the static allocation.
                if i >= MAX_FRAMES {
                    return false;
                }

                // Insert the frame into the statically-allocated buffer.
                // SAFETY: `i` is guaranteed in-bounds and there are no other readers or writers.
                std::ptr::replace(FRAMES[i].get(), Some(frame.clone()));

                i += 1;

                true
            });

            // Send a beacon to alert the thread that the frames are ready to be consumed.
            let exch =
                FRAMES_AVAILABLE.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire);
            if exch.is_err() {
                // Sanity check: If this beacon is set, we're DOA.
                libc::abort();
            }

            // Unpark the thread.
            // TODO: Make sure this doesn't do anything that is signal-unsafe.
            handle.thread().unpark();

            // Wait for the thread to print the backtrace. This is essentially a naive spinlock that
            // is signal-safe. Timeout occurs after 3 seconds.
            let mut counter = 30;
            loop {
                // Check the beacon from the thread and the counter to handle timeouts.
                if FRAMES_HANDLED.load(Ordering::Acquire) || counter == 0 {
                    break;
                }

                // Sleep the signal handler for 100 ms.
                #[cfg(unix)]
                {
                    let ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 100_000_000,
                    };
                    // SAFETY: `nanosleep` is not explicitly mentioned in POSIX async-signal-safety,
                    // but Linux implements `sleep` (which _is_ signal-safe) via `nanosleep`.
                    // Meanwhile, glibc claims that `sleep` is unsafe and `nanosleep` is safe!
                    // See: https://www.gnu.org/software/libc/manual/html_node/Sleeping.html
                    libc::nanosleep(&ts, std::ptr::null_mut());
                }

                #[cfg(windows)]
                Threading::Sleep(100);

                counter -= 1;
            }

            // I've seen things you people wouldn't believe.
            // Attack ships on fire off the shoulder of Orion.
            // I watched C-beams glitter in the dark near the Tannhauser gate.
            // All those moments will be lost in time.
            // Like tears in rain.
            // Time to die...
            libc::abort();
        })?;

        // Enable floating point exceptions.
        #[cfg(all(debug_assertions, any(target_arch = "x86", target_arch = "x86_64")))]
        x86_64::enable_fp_exceptions();

        #[cfg(all(
            debug_assertions,
            not(any(target_arch = "x86", target_arch = "x86_64"))
        ))]
        compile_error!("Unsupported platform");

        debug!("FPU exceptions enabled on thread {id:?}");
    }

    Ok(())
}
