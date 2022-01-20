# Batman

![Bat signal appears in the sky over Catwoman from the 1989 Batman film](./img/batsignal.jpg)

Have you ever written a function like this that was hard to debug?

```rust
fn main() {
    let signal = [""; 16].join(&format!("{}", f64::sqrt(50.3 - 50.0 - 0.3)));

    println!("{signal} Batman!");
}
```

Well now you don't have to be confused any more! Just let `batman` configure FPU hardware exceptions and your days of debugging garbage floating point calculations will be over!

```rust
fn main() {
    // Here be dragons!
    unsafe { batman::signal() };

    let signal = [""; 16].join(&format!("{}", f64::sqrt(50.3 - 50.0 - 0.3)));

    println!("{signal} Batman!");
}
```

Let's see how it works:

```
$ cargo run --example batman
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/batman`
Caught signal 8 (SIGFPE): Floating point exception
   0: batman::backtrace::Backtrace::new
             at ./src/backtrace.rs:14:20
   1: batman::unix::sigfpe_handler
             at ./src/unix.rs:25:14
   2: <unknown>
   3: std::f64::<impl f64>::sqrt
             at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/std/src/f64.rs:345
   4: batman::main
             at ./examples/batman.rs:5:47
   5: core::ops::function::FnOnce::call_once
             at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/ops/function.rs:227:5

Aborted
```

Signal `SIGFPE` is caught on line 5 in `batman.rs` in the call to `f64::sqrt()`! Multiply this by several thousand lines of various linear algebra calculations used in games and scientific computing, and `batman` will have your back.

> But floating point exceptions are slow and they crash my program!

That's why `batman` only enables them when debug assertions are enabled.

```
$ cargo run --example batman --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/examples/batman`
NaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNaN Batman!
```

> But setting signal and exception handlers mutates global state!

Yeah, configuring FPU exceptions was made `unsafe` because this can be a real problem. On the bright side, enabling exceptions themselves is thread-local state, so you can decide to raise exceptions on a per-thread basis. But be aware that threads inherit FPU exceptions from their parent, and there is no way to turn it off (for API simplicity).

# Caveats

`batman` requires unstable features and only works on nightly compilers. Since you're obviously adventurous anyway, you can use it on stable with [`nightly-crimes`](https://crates.io/crates/nightly-crimes).

# Why is it named `batman`?

If the joke wasn't clear by now, I humbly refer you to https://www.destroyallsoftware.com/talks/wat. Combine this with the bat signal implication, and you'll agree it's the best name.
