#![feature(bench_black_box)]

use std::hint::black_box;

fn main() {
    unsafe { batman::signal() };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(1.0) / black_box(0.0)
    );
}
