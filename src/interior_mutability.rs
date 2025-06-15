// Interior Mutability Pattern

// interv-1
// 
// 1. Create something immut (using stdlib) and modify its internal value
// 2. read ok, do skip
// Also see hints at the end







// --------------------------------------------------------------------








#[test]
fn ex1_cell() {
    let cell = Cell::new(5); 
        // use Cell with copy types like i32 
        // and RefCell for non-copy types like String

    cell.replace(33);
    assert_eq!(cell, Cell::new(33));
}


// 2. Foo with a val int and cell_val Cell, 
//    create a Foo, modify and read cell_val field






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

// ---------------------------------------------------------------------

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


// Hint
// 1. Create an *immut Cell/RefCell and modify it

use std::cell::{Cell, RefCell};
