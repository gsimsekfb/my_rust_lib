
// 1.a Example recursive type using Box
enum List {
  Cons(i32, Box<List>),
  Nil,
}

// 1.b without Box, we would have this compiler error
/*
enum List2 {
  Cons(i32, List),
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
fn ex1_deref_smart_ptr() {
  let xx = Box::new(42);
  assert_eq!(*xx, 42);
}

#[test]
fn ex2_usage_of_list() {
  use List::{Cons, Nil};
  let list = Cons(1,
    Box::new(Cons(2,
      Box::new(Cons(3,
        Box::new(Nil))))));
  // assert_eq!(*xx, 42);
}