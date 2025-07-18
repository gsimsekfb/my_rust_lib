/* 
Run: 
    cargo r --release --example bench
Also see benches/my_benchmark.rs 
*/

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn main() {
    let now = std::time::Instant::now();
    fibonacci(20);
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");  
}
//  Elapsed: 27.24µs
