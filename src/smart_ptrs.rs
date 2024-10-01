
// interview


// -------------------------------------------------------

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::ops::DerefMut;

// 1. Basics

#[test]
fn ex1_basics() {
    // a. With non-copy type String
    let mut p = Box::new("ab".to_string());
    // move
    // let val = *p; // value moved here
    // let val = p.deref(); // &String
        // error[E0382]: borrow of moved value: `p`

    // read
    let refer = p.deref(); // &String
    let refer = &*p;       // &String
    assert_eq!(refer, "ab");
    // mutate
    *p = "ff".to_string();
    assert_eq!(p.deref(), "ff");
    // or
    let refer = p.deref_mut(); // &mut String
    *refer = "cd".to_string();
    assert_eq!(refer, "cd");   // to &String 

    // b. With copy type i32
    let mut sp = Box::new(42);
    // read
    let refer = sp.as_ref();   // &i32
    assert_eq!(refer, &42);
    assert_eq!(*sp, 42);        // i32 - !! copied not moved
    // mutate
    *sp = 44;               
    assert_eq!(*sp, 44);    
}


// 2. Deref Trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> Self { Self(val) }
    fn val(&self) -> &T { &self.0 }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[test]
fn ex2_deref() {
    let p = MyBox::new(42);
    assert_eq!(p.val(), &42);
    assert_eq!(*p, 42); // why *p is not &i32 ? because *p is actually *(p.deref())

    let p = MyBox("ab".to_string());
    // let val = *p;
        // error[E0507]: cannot move out of dereference of `MyBox<String>`
        // ^^ move occurs because value has type `String`, which does not
        // implement the `Copy` trait

    let refer = p.deref(); // &String
    let refer = &*p;       // &String
    assert_eq!(refer, "ab");
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

#[test]
fn ex3_rc_ref_counted_ptr() {

  // 1.a mutable
  // mutating Rc (hint: it is immutable when total ref count > 1)
  // is possible ONLY when the ref count (strong + weak) is 1
  // which is not a reason to use, instead Box can be used
  let mut sp = Rc::new(3);
  *Rc::get_mut(&mut sp).unwrap() = 4;
  assert_eq!(Rc::strong_count(&sp), 1);
  assert_eq!(Rc::weak_count(&sp), 0);
  assert_eq!(*sp, 4);

  // 1.b. not mutable
  // !!! Rc is not mutable when the ref count (strong + weak) is more than 1
  let sp2 = Rc::clone(&sp);
  assert_eq!(Rc::strong_count(&sp) + Rc::weak_count(&sp), 2);
  assert_eq!(Rc::weak_count(&sp), 0);
  // !!!
  // get_mut() returns a mut ref, if there are NO other Rc or Weak pointers
  // to the same allocation.
  // Returns None otherwise, because it is not safe to mutate a shared value.
  assert!(Rc::get_mut(&mut sp).is_none());

  // 2. accessing raw ptr inside Rc
  let sp = Rc::new(3);
  let sp2 = Rc::clone(&sp);

  // p1 and p2 are raw ptrs pointing into the same mem. address
  let p1 = Rc::as_ptr(&sp); // *const i32
  println!("p1: {:p}", p1);

  let p2 = Rc::as_ptr(&sp2);
  println!("p2: {:p}", p2);

  assert_eq!(p1, p2);
  assert_eq!(unsafe { *p1 } , 3); // !! unsafe
}


#[test] fn ex_4_rc_refcell() {
  use std::rc::Rc;
  use std::cell::RefCell;

  //// a. ref count: 1
  let sp = Rc::new(RefCell::new(2)); // Rc<Refcell<i32>>
  assert_eq!(Rc::strong_count(&sp), 1);
  assert_eq!(Rc::weak_count(&sp), 0);
  
  // Access &T and mutate T - !! sp does not have to be "mut"
  let ref_cell = sp.as_ref(); // Rc<Refcell<i32>> to &RefCell<i32>
  // let refer = &*ref_cell.borrow(); // &i32
  assert_eq!(&*ref_cell.borrow(), &2);
  // let ref_cell = Rc::get_mut(&mut sp).unwrap();
      // !! No need to get mut, Refcell is interior mutable
  ref_cell.replace(4);
  assert_eq!(*ref_cell.borrow(), 4);

  //// b. ref count: 2
  let sp2 = Rc::clone(&sp); // Rc<Refcell<i32>>
  assert_eq!(Rc::strong_count(&sp), 2);
  assert_eq!(Rc::weak_count(&sp), 0);

  // Access &T and mutate T (same as above when ref count is 1)
  let ref_cell = sp.as_ref(); // Rc<Refcell<i32>> to &RefCell<i32>
  assert_eq!(&*ref_cell.borrow(), &4);
  ref_cell.replace(55);
  assert_eq!(&*ref_cell.borrow(), &55);

  //// c. Access raw ptrs to T
  // !! RefCell's as_ptr is called, not Rc's - auto deref coercion ?
  assert_eq!(sp.as_ptr(), sp2.as_ptr());
  let p_raw = sp.as_ptr(); // *mut i32
}



#[test] fn ex_5_weak_ptr() {
  use std::cell::RefCell;
  use std::rc::{Rc, Weak};

  struct Foo { val: i32 }

  let sp = Rc::new(RefCell::new(Foo { val: 42 })); // Rc<Refcell<i32>>
  assert_eq!(Rc::strong_count(&sp), 1);
  assert_eq!(Rc::weak_count(&sp), 0);
  let foo = sp.as_ref().borrow(); // Ref<'_, Val>
  assert_eq!(&foo.val, &42);
  drop(foo);
      // !! we must drop foo since it refers to sp which will be moved into node
      // in the next section

  // Create weak_ptr from Rc, we could do it without the Option wrapper as well
  let node = Some(sp); // Option<Rc<RefCell<Foo>>>
  let weak_ptr = node.as_ref().map(Rc::downgrade); // Option<Weak<RefCell<Foo>>>   
  // or
  // let weak_ptr = match node {
  //     Some(val) => Some(Rc::downgrade(val)),
  //     None => None
  // };
  let weak = weak_ptr.as_ref().unwrap();  // &Weak<RefCell<Foo>>
  assert_eq!(weak.strong_count(), 1);  // node
  assert_eq!(weak.weak_count(), 1);
  let sp2 = weak.upgrade().unwrap();      // Rc<RefCell<Foo>>
  assert_eq!(Rc::strong_count(&sp2), 2);  // node and sp2
  assert_eq!(Rc::weak_count(&sp2), 1);
}



// x. RefCell<T> - interior mutability at runtime
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
        Self {
            sent_messages: RefCell::new(vec![]),
        } // 2
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
fn ex_5_a_refcell_interior_mutability() {
    let msgr = MockMessenger::new();
    msgr.send("abc");
    assert!(msgr.sent_messages.borrow().len() == 1);
}

#[test]
#[should_panic(expected = "already borrowed")]
// or just: #[should_panic]
fn ex_5_b_refcell_bad_interior_mutability() {
    let msgr = MockMessenger::new();
    msgr.bad_send("bad"); // runtime err: see "fn bad_send()"
    assert!(msgr.sent_messages.borrow().len() == 1);
}
