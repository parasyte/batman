//! These test that `batman` causes panics (aborts) for some floating point operations.

use rusty_forkfork::rusty_fork_test;
use std::hint::black_box;

rusty_fork_test! {
    #[test]
    #[should_panic]
    fn test_panic_divide_by_zero() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(1.0) / black_box(0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_zero_div_zero() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(0.0) / black_box(0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_inf_div_inf() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(f32::INFINITY) / black_box(f32::INFINITY)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_mod_by_zero() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(1.0) % black_box(0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_inf_mod() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(f32::INFINITY) % black_box(1.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_zero_times_inf() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(0.0) * black_box(f32::INFINITY)
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_neg_sqrt() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(-1.0_f32).sqrt()
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_inf_minus_inf() {
        unsafe { batman::signal().unwrap() };

        eprintln!(
            "ERROR: This should never be printed! {}",
            black_box(f32::INFINITY) - black_box(f32::INFINITY)
        );
    }
}
