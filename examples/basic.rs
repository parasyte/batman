use std::hint::black_box;

fn main() -> std::io::Result<()> {
    env_logger::init();

    // Here be dragons!
    unsafe { batman::signal()? };

    eprintln!(
        "ERROR: This should never be printed! {}",
        black_box(1.0) / black_box(0.0)
    );

    Ok(())
}
