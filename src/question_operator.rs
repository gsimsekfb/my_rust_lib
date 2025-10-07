// interv-1

// a. Write the equivalent fn w/o question mark ? operator:
//
use std::fs::File;
use std::io::{self, Read};
                            // rustlib\src\rust\library\std\src\io\error.rs:
                            // pub type Result<T> = result::Result<T, Error>;
                            //             |
fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;  // ? operator
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // ? operator
    Ok(contents)
}

// b. see question mark jpg



// Equivalent Code Without Question Mark Operator:
//
// The ? operator:
// - Unwraps (not returns) the Ok value if the result is Ok
// - "Returns" the Err early if the result is Err

fn read_file_contents_(path: &str) -> io::Result<String> {
    #[allow(clippy::question_mark)]
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),  // Early return on error
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),  // Return error (!! implicit return here)
    }
}
