# Batman

![Bat signal appears in the sky over Catwoman from the 1989 Batman film](./img/batsignal.jpg)

`batman` enables FPU hardware exceptions (a per-thread configuration) and aborts the process if the
thread performs any floating point calculation that would result in a NaN.

```rust
fn main() -> std::io::Result<()> {
    env_logger::init();

    // Here be dragons!
    unsafe { batman::signal()? };

    let signal = [""; 16].join(&format!("{}", f64::sqrt(50.3 - 50.0 - 0.3)));

    println!("{signal} Batman!");

    Ok(())
}
```

Let's see how it works:

```
$ RUST_LOG=debug cargo run --example batman
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/batman`
[2023-09-21T06:35:26Z DEBUG batman] Enabling FPU exceptions on thread ThreadId(1)
[2023-09-21T06:35:26Z DEBUG batman] FPU exceptions enabled on thread ThreadId(1)
[2023-09-21T06:35:26Z DEBUG batman] Received beacon, processing backtrace...

Floating point exception occurred.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[2023-09-21T06:35:26Z DEBUG batman] Sending beacon and stopping thread...
Aborted
```

This is not terribly interesting, but there is a note to set the `RUST_BACKTRACE=1` environment variable. Let's try it:

```
$ RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example batman
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/batman`
[2023-09-21T06:36:43Z DEBUG batman] Enabling FPU exceptions on thread ThreadId(1)
[2023-09-21T06:36:43Z DEBUG batman] FPU exceptions enabled on thread ThreadId(1)
[2023-09-21T06:36:43Z DEBUG batman] Received beacon, processing backtrace...

Floating point exception occurred.
   0: std::f64::<impl f64>::sqrt
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/f64.rs:397
   1: batman::main
             at examples/batman.rs:7:47
   2: core::ops::function::FnOnce::call_once
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/core/src/ops/function.rs:250:5

note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[2023-09-21T06:36:43Z DEBUG batman] Sending beacon and stopping thread...
Aborted
```

A floating point exception is caught on line 7 in `batman.rs` in the call to `f64::sqrt()`. Use `RUST_BACKTRACE=full` for a more detailed backtrace:

<details><summary>Expand for details</summary>

```
$ RUST_BACKTRACE=full RUST_LOG=debug cargo run --example batman
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/batman`
[2023-09-21T06:37:20Z DEBUG batman] Enabling FPU exceptions on thread ThreadId(1)
[2023-09-21T06:37:20Z DEBUG batman] FPU exceptions enabled on thread ThreadId(1)
[2023-09-21T06:37:20Z DEBUG batman] Received beacon, processing backtrace...

Floating point exception occurred.
   0: backtrace::backtrace::libunwind::trace
             at /home/jay/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.69/src/backtrace/libunwind.rs:93:5
      backtrace::backtrace::trace_unsynchronized
             at /home/jay/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.69/src/backtrace/mod.rs:66:5
   1: batman::signal::{{closure}}
             at src/lib.rs:144:13
   2: signal_hook_registry::register_signal_unchecked::{{closure}}
             at /home/jay/.cargo/registry/src/index.crates.io-6f17d22bba15001f/signal-hook-registry-1.4.1/src/lib.rs:549:50
   3: signal_hook_registry::handler
             at /home/jay/.cargo/registry/src/index.crates.io-6f17d22bba15001f/signal-hook-registry-1.4.1/src/lib.rs:372:13
   4: <unknown>
   5: std::f64::<impl f64>::sqrt
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/f64.rs:397
   6: batman::main
             at examples/batman.rs:7:47
   7: core::ops::function::FnOnce::call_once
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/core/src/ops/function.rs:250:5
   8: std::sys_common::backtrace::__rust_begin_short_backtrace
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/sys_common/backtrace.rs:154:18
   9: std::rt::lang_start::{{closure}}
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/rt.rs:166:18
  10: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/core/src/ops/function.rs:284:13
      std::panicking::try::do_call
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panicking.rs:504:40
      std::panicking::try
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panicking.rs:468:19
      std::panic::catch_unwind
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panic.rs:142:14
      std::rt::lang_start_internal::{{closure}}
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/rt.rs:148:48
      std::panicking::try::do_call
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panicking.rs:504:40
      std::panicking::try
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panicking.rs:468:19
      std::panic::catch_unwind
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/panic.rs:142:14
      std::rt::lang_start_internal
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/rt.rs:148:20
  11: std::rt::lang_start
             at /rustc/3223b0b5e8dadda3f76c3fd1a8d6c5addc09599e/library/std/src/rt.rs:165:17
  12: main
  13: __libc_start_call_main
             at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
  14: __libc_start_main_impl
             at ./csu/../csu/libc-start.c:392:3
  15: _start

[2023-09-21T06:37:20Z DEBUG batman] Sending beacon and stopping thread...
Aborted
```
</details>


## Disabled by default in release builds

`batman` only enables floating point exceptions when debug assertions are enabled:

```
$ RUST_LOG=debug cargo run --example batman --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/examples/batman`
NaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaN Batman!
```

To allow floating point exceptions in release builds, add the following to your project's `Cargo.toml`:

```toml
[profile.release]
debug-assertions = true
```

You will probably want to enable all debug info to get readable backtraces:

```toml
[profile.release]
debug = true
debug-assertions = true
```


## Caveats

- Threads inherit floating point environment configuration from their parent.
- There is no way to turn off exceptions, once enabled (for API simplicity).
- `batman` requires unstable features and only works on nightly compilers.
- Hardware floating point exceptions are unrecoverable. Thus `batman` aborts when the exception is trapped. It cannot be made into an unwinding panic.
- The signal handler should be able to safely get the thread ID, it's just additional state that I haven't captured yet. Could be useful for log correlations in some multi-threaded apps.
- Only `x86_64` is supported at present, and only Windows, Linux, and macOS have been tested.
- On Windows, the error message from Cargo will say `"exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN"`. This is normal behavior for `abort`. See: https://devblogs.microsoft.com/oldnewthing/20190108-00/?p=100655


## Why is it named `batman`?

See: https://www.destroyallsoftware.com/talks/wat

And there's also the bat signal pun. The name was too good to resist!
