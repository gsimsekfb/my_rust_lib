// Interior Mutability Pattern
//
// https://doc.rust-lang.org/std/cell/index.html
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
//
// Cell<T> and RefCell<T> provide ‘interior mutability’, in contrast 
// with typical Rust types that exhibit ‘inherited mutability’.
// Cell types come in two flavors: Cell<T> and RefCell<T>. Cell<T> 
// implements interior mutability by moving values in and out of the Cell<T>.
// To use references instead of values, one must use the RefCell<T> type,
// acquiring a write lock before mutating.

use std::cell::RefCell;

#[test] fn ex1() {
    let cc = RefCell::new(5);
    cc.replace(33);
    assert_eq!(cc, RefCell::new(33));
}