// Disables warnings in crate level
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod mini_grep;
use mini_grep::{SearchConfig, run_search};
mod cacher;

// todos: 
// - yml for macos

fn main() {

    // 0) Use runtime env. vars. See [env] in Cargo.toml
    let xx: i32 = env!("XX").parse().unwrap();
	println!("xx: {xx:?}");   // 42


    // 1) Minigrep ---------------------------------------------
    // Usage:
        // cargo r search-str file.txt
        // cargo r cC test.txt
        // CASE_INSENSITIVE=1 cargo r cC test.txt
        //   Windows: cmd /c "set CASE_INSENSITIVE=1 && cargo r cC test.txt"          
        // cargo t

    // Create Config from args (get filename and search-str )
    println!("\n1. MiniGrep \n-------------");
    let args: Vec<String> = std::env::args().collect();
	println!("args: {args:?}");
    if args.len() == 3 {
        let config = SearchConfig::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            eprintln!("HINT: Try \"cargo r cC test.txt\"");
            std::process::exit(1);
        });
        // Run search
        if let Err(e) = run_search(config) {
            eprintln!("Application error: {e}");
            std::process::exit(1);
        }
        return;
    } else { println!("HINT: Try \"cargo r cC test.txt\""); }

    // 2) Cacher -----------------------------------------------

    let mut expensive_fn = cacher::Cacher::new(|num: u32| {
        // std::thread::sleep(std::time::Duration::from_secs(2));
        num*num
    });
    println!("\n2. Cacher \n-------------");
    println!("--- res for 12: {:?}", expensive_fn.result_for(12)); // calculate
    println!("--- res for 12: {:?}", expensive_fn.result_for(12)); // get cashed res.
    println!("--- res for 8: {:?}", expensive_fn.result_for(8)); // calculate
    println!("--- res for 8: {:?}", expensive_fn.result_for(8)); // get cached res.
}