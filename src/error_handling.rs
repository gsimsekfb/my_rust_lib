/// Equivalent error handlings with panic (w/o propagating to caller)

// Using expect
// #[test] // Uncomment to see the panic
fn panic_1() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.expect("Testing expect"); 
        // panics with `Testing expect: emergency failure`
}

// -- same as `expect("...")`:

// Using match
// #[test] // Uncomment to see the panic
fn panic_2() {
    let xx: Result<u32, &str> = Err("emergency failure");

    // panics with `Testing expect: emergency failure`
    let _yy = match xx {
        Ok(m) => m,
        Err(error) => panic!("Testing expect: {:?}", error),
    };
}

// -- same as an empty expect `expect("")`:

// Using unwrap
// #[test] // Uncomment to see the panic
fn panic_3() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.unwrap(); 
        // panics with `: emergency failure`
}

//// ---------