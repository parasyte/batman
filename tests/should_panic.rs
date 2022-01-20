//! These test that `batman` causes panics for some floating point operations.
//!
//! TODO: Replace panicking tests with child processes that checks the return value and stdout.

#![feature(bench_black_box)]

use std::hint::black_box;

#[test]
#[should_panic]
fn test_panic_divide_by_zero() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(1.0) / black_box(0.0)
    );
}

#[test]
#[should_panic]
fn test_panic_zero_div_zero() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(0.0) / black_box(0.0)
    );
}

#[test]
#[should_panic]
fn test_panic_inf_div_inf() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(f32::INFINITY) / black_box(f32::INFINITY)
    );
}

#[test]
#[should_panic]
fn test_panic_mod_by_zero() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(1.0) % black_box(0.0)
    );
}

#[test]
#[should_panic]
fn test_panic_inf_mod() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(f32::INFINITY) % black_box(1.0)
    );
}

#[test]
#[should_panic]
fn test_panic_zero_times_inf() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(0.0) * black_box(f32::INFINITY)
    );
}

#[test]
#[should_panic]
fn test_panic_neg_sqrt() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(-1.0_f32).sqrt()
    );
}

#[test]
#[should_panic]
fn test_panic_inf_minus_inf() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(f32::INFINITY) - black_box(f32::INFINITY)
    );
}
