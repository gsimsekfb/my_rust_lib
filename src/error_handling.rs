
//// Rust groups errors into two major categories: recoverable and unrecoverable
//// 1. Recoverable error
//// such as a file not found error, we most likely just want to report the 
//// problem to the user and retry the operation. 
//// 2. Unrecoverable errors:
//// are always symptoms of bugs, like trying to access a location beyond 
//// the end of an array, and so we want to immediately stop the program.
//// Src: https://doc.rust-lang.org/book/ch09-00-error-handling.html
////
//// gs2022: 
//// So, in summary, ideally (in production code), we want to recover/handle 
//// or return that error/result to caller (and not use panic or exit or expect 
//// or unwrap at all - they are all the same in the sense they stop the program)
//// for recoverable errors. For unrecoverable, we should exit/panic.

// -----------------------------------------

//// A. Returning multi-type error

/// A.1. Return multi-type error with Box<dyn Error>
use std::error::Error;
use std::net::Ipv6Addr;

// CON: We lost the type info by using Box<dyn Error> - aka type erasure
// See A.2 for better solution
fn foo() -> Result<(), Box<dyn Error>> {
    let ff = std::fs::File::open("ttt.txt")?; // Err(Os)
    let ip = "abc".parse::<Ipv6Addr>()?; // Err(AddrParseError(Ipv6))
    Ok(())
}

#[test]
fn ex_a_1_return_multi_type_error() {
    let res = foo();
    println!("--- res: {:?}", res);
        // --- res: Err(Os { 
        //    code: 2, kind: NotFound, message: "No such file or directory" })
        // --- res: Err(AddrParseError(Ipv6))
}

/// A.2. Return multi-type error with MyError (suggested solution) 
///      to retain type safety

#[derive(Debug)]
enum MyError {
    IO(std::io::Error),
    Parsing(std::net::AddrParseError),
}

impl std::error::Error for MyError { }

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Implement Display in terms of Debug
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IO(error)
    }
}

impl From<std::net::AddrParseError> for MyError {
    fn from(error: std::net::AddrParseError) -> Self {
        MyError::Parsing(error)
    }
}

fn foo2() -> Result<(), MyError> {
    let _file = std::fs::File::open("tt.txt")?;
    let _ip = "abc".parse::<std::net::Ipv6Addr>()?;
    // Or use `map_err` if we don't want to impl From trait
    // let _file = std::fs::File::open("tt.txt").map_err(MyError::IO)?;
    // let _ip = "abc".parse::<std::net::Ipv6Addr>().map_err(MyError::Parsing)?;    
    Ok(())
}

#[test]
fn ex_a_2_return_multi_type_error() {
    let res2 = foo2();
    println!("--- res2: {:?}", res2);
        // --- res: Err(Os { 
        //    code: 2, kind: NotFound, message: "No such file or directory" })
        // --- res: Err(AddrParseError(Ipv6))
}

//// B. Three equivalent error handling when panicking (w/o propagating to caller)
//// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

// Using expect - SUGGESTED OPTION
#[test] 
#[should_panic] // Comment to see the panic
fn ex_b_1() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.expect("Error xyz happened"); 
        // panics with `Error xyz happened`: emergency failure`
}

// Using match - same as `expect("...")` in ex_1:
#[test]
#[should_panic] // Comment to see the panic
fn ex_b_2() {
    let xx: Result<u32, &str> = Err("emergency failure");
    // panics with `Testing expect: emergency failure`
    let _yy = match xx {
        Ok(m) => m,
        Err(error) => panic!("Error X happened, more: {:?}", error),
    };
}

// Using unwrap - same as using an empty expect `expect("")` in ex_1:
#[test]
#[should_panic] // Comment to see the panic
fn ex_b_3() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.unwrap(); 
        // panics with `: emergency failure`
}
