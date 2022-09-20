
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
fn ex1_usage_of_list() {
  use List::{Cons, Nil};
  let list = Cons(1,
    Box::new(Cons(2,
      Box::new(Cons(3,
        Box::new(Nil))))));
  // assert_eq!(*xx, 42);
}

// -------------------------------------------------------

// 2. Deref Trait 
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> std::ops::Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}

#[test]
fn ex2_deref() {
  let x = 5;
  let y = MyBox::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y); // *y is actually *(y.deref())
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

/// Implicit Deref Coercions
#[test]
fn ex2_deref_coerc() {
  let ss = MyBox::new(String::from("aaa"));
  // Rust turns &MyBox<String> into &String by calling deref. 
  // Rust calls deref again to turn the &String into &str
  hello(&ss); // with auto deref coerc.
    // same as:
    hello(&(*ss)[..]); // // w/o auto deref coerc.
}

// -------------------------------------------------------

// 3. Rc, ref counted ptr aka shared ptr
enum List2 {
  Cons(i32, Rc<List2>),
  Nil,
}
use std::rc::Rc;

#[test]
fn ex3_rc_ref_counted_ptr() {
  use List2::{Cons, Nil};
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  // Two lists, b and c, sharing ownership of a third list, a
  // See https://carols10cents.github.io/book/ch15-04-rc.html#using-rct-to-share-data
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));
}