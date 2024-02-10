use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::process::exit;

/*
//// Info:
// Finds and sums the time logs in this format:
// ---c413: 0.134ms

/// Usage:
cargo r --example file_sum_time_logs ./files/time_log.txt 4121
str: 4121
count: 7 times
sum: 0.94 ms
*/

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    println!(
        "Usage: cargo r --example file_sum_time_logs ./files/time_log.txt 4121"
    );
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    let find_str = &args[2];
    let mut sum = 0.0;
    let mut count = 0;

    match read_lines(file) {
        Ok(lines) => {
            for line in lines.flatten() {
                if !line.contains(find_str) { continue; }
                let v: Vec<&str> = line.split(':').collect(); 
                let num_str = v[1].trim().trim_end_matches("ms");
                sum += num_str.parse::<f64>().unwrap_or_else(
                    |_| { 
                        panic!("line: {}, num_str: {}", line, num_str)
                    }
                );
                count += 1;
            }
        },
        Err(e) => { println!("err: {}", e); exit(-1) },
    }
    println!("str: {}", find_str);
    println!("count: {} times", count);
    println!("sum: {:.2} ms", sum);
}

