
// interv
// 1. multi type error w/ runtime polymorphism
// - fn foo which returns Result with any type of errors
// - in foo body, open a file and parse str to int or ip4 address
// 
// 2. multi type error w/ compile polymorphism
// - same as above but different impl



//// Rust groups errors into two major categories: recoverable and unrecoverable
//// 1. Recoverable error
//// such as a file not found error, we most likely just want to report the 
//// problem to the user and retry the operation. 
//// 2. Unrecoverable errors:
//// are always symptoms of bugs, like trying to access a location beyond 
//// the end of an array, and so we want to immediately stop the program.
//// 
//// Use Result<T, E> for recoverable errors. 
//// Use the panic! for unrecoverable errors. 
//// (panic: macro that stops program execution)
//// Src: https://doc.rust-lang.org/book/ch09-00-error-handling.html
////
//// gs2022: 
//// So, in summary, ideally (in production code), for recoverable errors, 
//// we want to recover/handle 
//// or return that error/result to caller (and not use panic or exit or expect 
//// or unwrap at all - they are all the same in the sense they stop the program)
//// For unrecoverable, we should exit/panic.

// Good srcs:
// https://doc.rust-lang.org/book/ch09-00-error-handling.html
// https://www.lurklurk.org/effective-rust/errors.html

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
    fn from(err: std::io::Error) -> Self {
        MyError::IO(err)
    }
}

impl From<std::net::AddrParseError> for MyError {
    fn from(err: std::net::AddrParseError) -> Self {
        MyError::Parsing(err)
    }
}

fn foo2() -> Result<(), MyError> {
    let _file = std::fs::File::open("tt.txt")?;
    let _ip = "abc".parse::<std::net::Ipv6Addr>()?;
    // Or use `map_err` if we don't want to impl From trait
    let _file = std::fs::File::open("tt.txt").map_err(MyError::IO)?;
    let _ip = "abc".parse::<std::net::Ipv6Addr>().map_err(MyError::Parsing)?;    
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

// Using expect or unwrap_or_else - SUGGESTED OPTION
#[test] 
#[should_panic] // Comment to see the panic
fn ex_b_1() {
    let xx: Result<u32, &str> = Err("emergency failure");
    #[allow(clippy::unnecessary_literal_unwrap)]
    let _yy = xx.expect("Error happened");
        // Error happened: emergency failure
    // or with capture variable
    let err_code = 42;
    #[allow(clippy::unnecessary_literal_unwrap)]
    let _zz = xx.unwrap_or_else(|_| panic!("Error {err_code} happened"));
        // Error 42 happened
}

// Using match - same as `expect(". . .")` in ex_1:
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

// Using unwrap - equivalent to using an empty expect `expect("")` in ex_1:
#[test]
#[should_panic] // Comment to see the panic
fn ex_b_3() {
    let xx: Result<u32, &str> = Err("emergency failure");
    #[allow(clippy::unnecessary_literal_unwrap)]
    let _yy = xx.unwrap(); 
        // panics with `: emergency failure`
}


//// skip
////
//// C. How to implement MyError a String wrapper, convertible to String, and 
//// implements Error trait

// a. String in Result<i32, String> doesn't implement Error, which we'd prefer 
// so that other areas of code can work with it
pub fn find_user_a(username: &str) -> Result<i32, String> {
    let _file = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Failed to open password file: {:?}", e))?;
    Ok(42)
}
// So, we use newtype pattern 
// (It's not possible to impl Error for String, because neither the trait 
//  nor the type belong to us - the so-called orphan rule)
//       |
//       v
#[derive(Debug)]
pub struct MyError_(String);
impl std::error::Error for MyError_ {}

// Error trait requires Debug + Display traits
impl std::fmt::Display for MyError_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// b. We want this to work: String to MyError_ auto conversion
pub fn find_user_b(username: &str) -> Result<i32, MyError_> {
    let _file = std::fs::File::open("/etc/aaa")
        .map_err(|e| format!("Failed to open file: {:?}", e))?;
    Ok(42)
}
// error:
// the trait `From<std::string::String>` is not implemented for `temp::MyError_`
//       |
//       v
impl From<std::string::String> for MyError_ {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[test]
fn ex_c_1() {
    let res = find_user_b("aaa");
    println!("-- res: {:?}", res);
    // -- res: Err(MyError_("Failed to open file: 
    // Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"))

    assert!(res.is_err());
}
