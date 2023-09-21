//! Per-thread tests

use std::{any::Any, hint::black_box, thread};

#[test]
fn test_pass_per_thread_sync_zero_div_zero() -> Result<(), Box<dyn Any + Send>> {
    let handle = thread::spawn(|| {
        unsafe { batman::signal().unwrap() };
    });
    handle.join()?;

    assert!((black_box(0.0_f32) / black_box(0.0)).is_nan());

    Ok(())
}

#[test]
fn test_pass_per_thread_racy_zero_div_zero() -> Result<(), Box<dyn Any + Send>> {
    let handle = thread::spawn(|| {
        unsafe { batman::signal().unwrap() };
    });

    assert!((black_box(0.0_f32) / black_box(0.0)).is_nan());

    handle.join()?;

    Ok(())
}
