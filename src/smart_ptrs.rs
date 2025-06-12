// interv-1 and 2

// 1. Create Boxed string, mutate it, create immut ref to inner value (4 ways)
// .. more below

// -------------------------------------------------------

// 1. Basics

#[test]
fn ex1_box() {
    // a. With non-copy type String
    let mut p = Box::new("ab".to_string());
    // move
    // let val = *p; // value moved here
    // let val = p.deref(); // &String
    // error[E0382]: borrow of moved value: `p`

    // read
    let immut_ref: &String = &p; // &String aka Deref coercion (auto deref)
    let immut_ref = p.deref(); // &String
    let immut_ref = p.as_ref(); // &String
    let immut_ref = &*p; // &String  - chose this? inline with mut vers.
    assert_eq!(immut_ref, "ab");
    // mutate
    *p = "ff".to_string();
    assert_eq!(p.deref(), "ff");
    // or
    let mut_ref = p.deref_mut(); // &mut String
    *mut_ref = "cd".to_string();
    assert_eq!(mut_ref, "cd"); // to &String

    // b. With copy type i32
    let mut p = Box::new(42);
    // read
    let immut_ref = p.as_ref(); // &i32
    assert_eq!(immut_ref, &42);
    assert_eq!(*p, 42); // i32 - !! copied not moved
                        // mutate
    *p = 44;
    assert_eq!(*p, 44);
}

// interv-2
// 2.
// Create MyBox generic tuple struct, impl fn new and fn val
// Impl std::ops::Deref for generic MyBox and use it

// ===============================================================

// 2. Deref Trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> Self {
        Self(val)
    }
    fn val(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn ex2_deref() {
    let p = MyBox::new(42);
    assert_eq!(*p, 42); // why *p is not &i32 ? because *p is actually *(p.deref())
    assert_eq!(p.val(), &42);

    let p = MyBox("ab".to_string());
    // let val = *p;  // yes, deref() is not sink fn but *p is actually *(p.deref())
    // error[E0507]: cannot move out of dereference of `MyBox<String>`
    // ^^ move occurs because value has type `String`, which does not
    // implement the `Copy` trait

    let refer = p.deref(); // &String
    let refer = &*p; // &String
                     // let refer = p.as_ref(); // &String // error: AsRef trait not impl for String
    let refer: &String = &p; // &String
    assert_eq!(refer, "ab");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/// Implicit Deref Coercions aka implicit forced deref
#[test]
fn ex2_deref_coerc() {
    let p = MyBox::new(String::from("aaa"));

    // &MyBox<String> derefed to &String, that's it
    //
    // From book: wrong?
    // Rust turns &MyBox<String> into &String by calling deref.
    // Rust calls deref again to turn the &String into &str ??
    hello(&p); // with auto deref coerc.
               // same as:
    hello(&(*p)[..]); // // w/o auto deref coerc.
}

// -------------------------------------------------------

// 3. Rc<T>, "immutable" ref counted ptr (== C++ shared_ptr<const T>)

#[test]
fn ex3_rc_ref_counted_ptr() {
    // 1.a mutable - ONLY when the ref count (strong + weak) is 1
    // which is not a reason to use, use Box instead
    let mut sp = Rc::new(3);
    *Rc::get_mut(&mut sp).unwrap() = 4;
    assert_eq!(Rc::strong_count(&sp), 1);
    assert_eq!(Rc::weak_count(&sp), 0);
    assert_eq!(*sp, 4);

    // 1.b. otherwise not-mutable
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
    assert_eq!(unsafe { *p1 }, 3); // !! unsafe
}

// 4. Rc<RefCell<T>>, "mutable" ref counted ptr (== C++ shared_ptr<T>)
//    aka interior mutability
//    Note: RefCell for non-copy (e.g. String) and Cell for copy types (e.g. i32)

#[test]
fn ex_4_rc_refcell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    //// a. ref count: 1
    let p1 = Rc::new(RefCell::new(String::from("ab"))); // Rc<Refcell<T>>
    assert_eq!(Rc::strong_count(&p1), 1);
    assert_eq!(Rc::weak_count(&p1), 0);

    // &T
    {
        let immut_ref = &*p1.borrow();  // &String
        assert_eq!(immut_ref, "ab");
    }

    // mutate T
    // !! sp does not have to be "mut" to be mutated which is the point
    *p1.borrow_mut() = "cd".to_string();
    assert_eq!(*p1.borrow(), "cd");
    // or
    p1.replace("er".to_string());
    assert_eq!(*p1.borrow(), "er");


    //// b. ref count: 2
    let p2 = Rc::clone(&p1); // Rc<Refcell<i32>>
    assert_eq!(Rc::strong_count(&p1), 2);
    assert_eq!(Rc::weak_count(&p1), 0);

    // Access &T and mutate T (same as above when ref count is 1)

    //// c. Access raw ptrs to T
    // !! RefCell's as_ptr is called, not Rc's - auto deref coercion ?
    assert_eq!(p1.as_ptr(), p2.as_ptr());
    let p_raw = p1.as_ptr(); // *mut i32
}

// 5. Weak Ptr

#[test]
fn ex_5_weak_ptr() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    struct Foo {
        val: i32,
    }

    let sp = Rc::new(RefCell::new(Foo { val: 42 })); // Rc<Refcell<Foo>>
    assert_eq!(Rc::strong_count(&sp), 1);
    assert_eq!(Rc::weak_count(&sp), 0);
    let foo = sp.as_ref().borrow(); // Ref<'_, Foo>
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
    let weak = weak_ptr.as_ref().unwrap(); // &Weak<RefCell<Foo>>
    assert_eq!(weak.strong_count(), 1); // node
    assert_eq!(weak.weak_count(), 1);
    let sp2 = weak.upgrade().unwrap(); // Rc<RefCell<Foo>>
    assert_eq!(Rc::strong_count(&sp2), 2); // node and sp2
    assert_eq!(Rc::weak_count(&sp2), 1);
}

// 6. RefCell<T> - interior mutability, postpone borrow rules to runtime
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
fn ex_6_a_refcell_interior_mutability() {
    let msgr = MockMessenger::new();
    msgr.send("abc");
    assert!(msgr.sent_messages.borrow().len() == 1);
}

#[test]
#[should_panic(expected = "already borrowed")]
// or just: #[should_panic]
fn ex_6_b_refcell_bad_interior_mutability() {
    let msgr = MockMessenger::new();
    msgr.bad_send("bad"); // runtime err: see "fn bad_send()"
    assert!(msgr.sent_messages.borrow().len() == 1);
}

use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
