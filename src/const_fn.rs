// const fn:
// https://rust-lang.github.io/rfcs/0911-const-fn.html
// Allow marking free functions and inherent methods as const, enabling them to
// be called in constants contexts, with constant arguments.

// Constant expressions
// https://doc.rust-lang.org/reference/const_eval.html
// Constant expressions, can be evaluated at compile time

const fn add_one(n: usize) -> usize {
    n + 1
}

#[test] fn ex1() {
    const BAR: usize = add_one(5);  // constants context
    let array = [0_u8; add_one(2)]; // constants context

    assert_eq!(BAR, 6);
    assert_eq!(array, [0,0,0]);
}