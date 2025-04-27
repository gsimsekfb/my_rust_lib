// todo: how to run test without test mode ?

// More
// https://doc.rust-lang.org/cargo/reference/profiles.html?highlight=panic%20abort#panic

// Note: 
// Panic handling is different in test mode that's why this test cannot show
// how the Cargo.toml setting below, changes the behavior of a panicking program.
//
// Cargo.toml:
//
// [profile.dev]
// panic = 'abort'     # Abort on panic
//
// In short, 
// (a) if this program was run in dev mode (with the DEV setting above), 
// program would abort immediately after catching the "panic!("caught a panic");"
// So the match line would not be run.
// (b) w/o Cargo.toml "panic on abort" setting, the match line would be run.
//
#[test]
fn catch_unwind() {
    let res = std::panic::catch_unwind(|| {
        panic!("-- caught a panic");
    });

    match res {
        Ok(_) => println!("\n-- No panic occured \n"),
        Err(err) => println!("\n-- After catching panic: {err:?} \n")
    }
}
