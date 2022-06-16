use rust_book_minigrep::{Config, run_search};


// Usage:
// cargo r cC test.txt
// CASE_INSENSITIVE=1 cargo r cC test.txt
// cargo t

fn main() {
    // Create Config from args(filename and query)
    let args: Vec<String> = std::env::args().collect();
	println!("args: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // Run search
    if let Err(e) = run_search(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}