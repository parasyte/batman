//! Per-thread tests

#![feature(bench_black_box)]

use std::{hint::black_box, thread};

#[test]
fn test_pass_per_thread_sync_zero_div_zero() {
    let handle = thread::spawn(|| {
        unsafe { batman::signal() };
    });
    let _ = handle.join();

    assert!((black_box(0.0_f32) / black_box(0.0)).is_nan());
}

#[test]
fn test_pass_per_thread_racy_zero_div_zero() {
    let handle = thread::spawn(|| {
        unsafe { batman::signal() };
    });

    assert!((black_box(0.0_f32) / black_box(0.0)).is_nan());

    let _ = handle.join();
}
