fn main() -> std::io::Result<()> {
    env_logger::init();

    // Here be dragons!
    unsafe { batman::signal()? };

    let signal = [""; 16].join(&format!("{}", f64::sqrt(50.3 - 50.0 - 0.3)));

    println!("{signal} Batman!");

    Ok(())
}
