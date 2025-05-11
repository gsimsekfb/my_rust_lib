use criterion::{black_box, criterion_group, criterion_main, Criterion};

// interv & todo
// - impl fibonacci with at least 3 different approaches
// - what are the time & space complexities
// - hint: the approaches are at the end of file

// for extra bench section, run with: 
// cargo bench fib


// ------------------------------------------------------------------------



// Time Complexity: O(2ⁿ) (Exponential)
// Each call branches into two recursive calls (fib(n-1) and fib(n-2)), leading 
// to a binary tree of recursive calls.
// Example: For n = 5, the call tree evaluates fib(5), fib(4), fib(3), etc., redundantly recalculating the same values many times.
//
// Space Complexity: O(n) (Linear)
// The maximum depth of the call stack is n (e.g., fib(n) → fib(n-1) → ... → fib(0)).
// No additional space is used beyond the stack frames (no memoization or arrays). 
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // remove black_box for fns w/o params 
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


// Hint: recursive, iterative and memoization
