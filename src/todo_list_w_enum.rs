// todo: move to algos
// -------------------------------------------------------
// 1.a Example recursive type using Box<T> (== C++ unique_ptr<const T>)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 1.b without Box, we would have this compiler error
/*
enum List2 {
  Cons(i32, List2),
  Nil,
}

error[E0072]: recursive type `List` has infinite size
 --> src\smart_ptrs.rs:2:1
  |
2 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
3 |   Cons(i32, List),
  |             ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`)
      to make `List` representable
  |
3 |   Cons(i32, Box<List>),
  |             ++++    +
*/

#[test]
fn ex1_usage_of_list() {
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


// 1.b. Rc<T>, ref counted ptr (== C++ shared_ptr<const T>)
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
  }
  use std::{cell::RefCell, rc::Rc};
  
  #[test]
  fn ex2_() {
    use List2::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Two lists, b and c, sharing ownership of a third list, a
    // See https://carols10cents.github.io/book/ch15-04-rc.html#using-rct-to-share-data
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // b[3, rc]
    //       \_ a [5, ..]
    //       /
    // c[4, rc]
    // a has 2 ref count, b and c both have 1
  }
  