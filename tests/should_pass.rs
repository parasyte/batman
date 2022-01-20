//! These tests do _not_ use `batman`. All operations should succeed without panicking.
//!
//! This just tests our assumptions that nothing fishy is happening with floating point operations.

#![feature(bench_black_box)]

use std::hint::black_box;

#[test]
fn test_pass_divide_by_zero() {
    assert!((black_box(1.0_f32) / black_box(0.0)).is_infinite());
}

#[test]
fn test_pass_zero_div_zero() {
    assert!((black_box(0.0_f32) / black_box(0.0)).is_nan());
}

#[test]
fn test_pass_inf_div_inf() {
    assert!((black_box(f32::INFINITY) / black_box(f32::INFINITY)).is_nan());
}

#[test]
fn test_pass_mod_by_zero() {
    assert!((black_box(1.0_f32) % black_box(0.0)).is_nan());
}

#[test]
fn test_pass_inf_mod() {
    assert!((black_box(f32::INFINITY) % black_box(1.0)).is_nan());
}

#[test]
fn test_pass_zero_times_inf() {
    assert!((black_box(0.0) * black_box(f32::INFINITY)).is_nan());
}

#[test]
fn test_pass_neg_sqrt() {
    assert!((black_box(-1.0_f32).sqrt()).is_nan());
}

#[test]
fn test_pass_inf_minus_inf() {
    assert!((black_box(f32::INFINITY) - black_box(f32::INFINITY)).is_nan());
}

#[test]
fn test_pass_inf_plus_inf() {
    assert!((black_box(f32::INFINITY) + black_box(f32::INFINITY)).is_infinite());
}

#[test]
fn test_pass_finite_division() {
    assert!(black_box(20.0) / black_box(5.0) - 4.0 <= f32::EPSILON);
}

#[test]
fn test_pass_sqrt() {
    assert!(black_box(25.0_f32).sqrt() - 5.0 <= f32::EPSILON);
}

#[test]
fn test_pass_nothing_up_our_sleeves_inf_plus_inf() {
    unsafe { batman::signal() };

    // Nothing up our sleeves: This won't panic!
    assert!((black_box(f32::INFINITY) + black_box(f32::INFINITY)).is_infinite());
}

#[test]
fn test_pass_nothing_up_our_sleeves_finite_division() {
    unsafe { batman::signal() };

    // Nothing up our sleeves: This won't panic!
    assert!(black_box(20.0) / black_box(5.0) - 4.0 <= f32::EPSILON);
}

#[test]
fn test_pass_nothing_up_our_sleeves_sqrt() {
    unsafe { batman::signal() };

    // Nothing up our sleeves: This won't panic!
    assert!(black_box(25.0_f32).sqrt() - 5.0 <= f32::EPSILON);
}
