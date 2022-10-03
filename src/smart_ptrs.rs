
// 1.a Example recursive type using Box<T> (== C++ unique_ptr<const T>)
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

// 3. Rc<T>, ref counted ptr (== C++ shared_ptr<const T>)
enum List2 {
  Cons(i32, Rc<List2>),
  Nil,
}
use std::{cell::RefCell, rc::Rc};

#[test]
fn ex3_rc_ref_counted_ptr() {
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

// -------------------------------------------------------

// 4. RefCell<T> - interior mutability at runtime
//                 (disabling compile time borrow rules)
pub trait Messenger {
  fn send(&self, msg: &str);
  fn bad_send(&self, msg: &str);
}

struct MockMessenger {
  // sent_messages: Vec<String>, // 1 
  sent_messages: RefCell<Vec<String>>, // 2
}

impl MockMessenger {
  fn new() -> Self {
    // Self { sent_messages: vec![] } // 1
    Self { sent_messages: RefCell::new(vec![]) } // 2
  }
}

impl Messenger for MockMessenger {
  fn send(&self, message: &str) {
    // self.sent_messages.push(String::from(message)); // 1
      // Comp. Err: `self` is a `&` reference, so the data it refers to cannot
      //  be borrowed as mutable
    self.sent_messages.borrow_mut().push(String::from(message)); // 2
      // mutating sent_messages which is immutable at compile time 
  }

  fn bad_send(&self, message: &str) {
      let mut one_borrow = self.sent_messages.borrow_mut();
      let mut two_borrow = self.sent_messages.borrow_mut();
        // runtime err: thread 'smart_ptrs::ex4_2_refcell_bad_interior_mutability' 
        // panicked at 'already borrowed:        
      one_borrow.push(String::from(message));
      two_borrow.push(String::from(message));
    }  
}

// interior mutability
#[test]
fn ex4_1_refcell_interior_mutability() {
  let msgr = MockMessenger::new();
  msgr.send("abc");
  assert!(msgr.sent_messages.borrow().len() == 1);
}

#[test] #[should_panic(expected = "already borrowed")]
// or just: #[should_panic]
fn ex4_2_refcell_bad_interior_mutability() {
  let msgr = MockMessenger::new();
  msgr.bad_send("bad"); // runtime err: see "fn bad_send()"
  assert!(msgr.sent_messages.borrow().len() == 1);
}