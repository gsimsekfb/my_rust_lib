use log::{debug, error, info};

// env_logger
// https://docs.rs/env_logger/latest/env_logger/#

#[test]
fn ex1() {
    env_logger::init();
    // By order of importance:
    error!("this is printed by default");
    info!("this is info {}", "message");
    debug!("this is a debug {}", "message");
}
    // $ RUST_LOG=error cargo r
    // [2023-11-09T02:12:24Z ERROR main] this is printed by default

    // $ RUST_LOG=info cargo r
    // [2023-11-09T02:12:24Z ERROR main] this is printed by default
    // [2023-11-09T02:12:24Z INFO main] this is info

    // $ RUST_LOG=debug cargo r
    // [2023-11-09T02:12:24Z ERROR main] this is printed by default
    // [2023-11-09T02:12:24Z INFO main] this is info
    // [2023-11-09T02:12:24Z DEBUG main] this is a debug message