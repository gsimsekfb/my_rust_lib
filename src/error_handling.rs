
//// A. Return multi-type error with Box<dyn Error>
use std::error::Error;
use std::net::Ipv6Addr;

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

//// B. 3 equivalent error handling when panicking (w/o propagating to caller)
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

//// ---------
 
//// Rust groups errors into two major categories: recoverable and unrecoverable 
//// errors. For a recoverable error, such as a file not found error, 
//// we most likely just want to report the problem to the user and retry the 
//// operation. Unrecoverable errors are always symptoms of bugs, like trying 
//// to access a location beyond the end of an array, and so we want to 
//// immediately stop the program.
//// Src: https://doc.rust-lang.org/book/ch09-00-error-handling.html

//// gs2022: 
//// So, in summary, ideally (in production code), we want to recover/handle 
//// or return that error/result to caller (and not use panic or exit or expect 
//// or unwrap at all - they are all the same in the sense they stop the program)
//// for recoverable errors.