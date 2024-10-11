// interv
// 
// 1. Create an *immut RefCell and modify it
// 2. Foo with a val int and cell_val Cell, modify and read cell_val



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

use std::cell::{Cell, RefCell};

#[test] fn ex1_refcell() {
    let rr = RefCell::new(5);
    rr.replace(33);
    assert_eq!(rr, RefCell::new(33));
}



struct Foo {
    regular_field: u8,
    special_field: Cell<u8>,
}

#[test] fn ex2_cell() {
    let foo = Foo {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    // ERROR: `foo` is immutable
    // foo.regular_field = 42;

    // WORKS: although `foo` is immutable, `special_field` is a `Cell`,
    // which can always be mutated
    foo.special_field.set(42);
    assert_eq!(foo.special_field.get(), 42);
}

