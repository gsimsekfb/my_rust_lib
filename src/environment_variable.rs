// reading environment variable

// build.rs:
// println!("cargo:rustc-env=LOG_LEVEL=1");
#[test]
fn env_vars() {
    // 1. Compile time
    const LOG_LEVEL: &str = env!("LOG_LEVEL");         // eval.ed at comp. time 
    let log_level = LOG_LEVEL.parse::<i32>().unwrap(); // eval.ed at runtime
    assert_eq!(LOG_LEVEL, "1");
    assert_eq!(log_level, 1);
    
    // 2. Run time
    let log_level: i32 = 
        std::env::var("LOG_LEVEL").expect("failed to read env.var").parse().unwrap();
    assert_eq!(log_level, 1);
}