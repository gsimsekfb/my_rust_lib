// Disables warnings in crate level
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rust_book_minigrep::{Config, run_search};

mod cacher;
mod closure;
mod doc_test;
mod error_handling;
mod iterators;
mod type_;
mod re_export;
mod smart_ptrs;

// Usage:
// cargo r cC test.txt
// CASE_INSENSITIVE=1 cargo r cC test.txt
// cargo t

fn main() {

    // 1) Minigrep ---------------------------------------------
    // // Create Config from args(filename and query)
    // let args: Vec<String> = std::env::args().collect();
	// println!("args: {:?}", args);
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     std::process::exit(1);
    // });

    // // Run search
    // if let Err(e) = run_search(config) {
    //     eprintln!("Application error: {}", e);
    //     std::process::exit(1);
    // }

    // 2) Cacher -----------------------------------------------

    // let mut expensive_fn = cacher::Cacher::new(|num: u32| {
    //     // std::thread::sleep(std::time::Duration::from_secs(2));
    //     num*num
    // });
    // println!("--- res for 12: {:?}", expensive_fn.result(12)); // calculate
    // println!("--- res for 12: {:?}", expensive_fn.result(12)); // get cashed res.
    // println!("--- res for 8: {:?}", expensive_fn.result(8)); // calcualate
    // println!("--- res for 8: {:?}", expensive_fn.result(8)); // get cached res.

    // 3) Closure ----------------------------------------------
    closure::examples();
}