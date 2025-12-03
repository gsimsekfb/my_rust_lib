// interv-1 and interv-2

// 1. Create Boxed Foo obj with int x
//    - mutate x, create immut ref to inner value in different ways
//
// .. task 2 below




// -------------------------------------------------------





#![allow(clippy::toplevel_ref_arg)]

// 0. Task-1
#[test]
fn ex0_task_1() {
    struct Foo { x: i32 }
    let mut foo = Box::new(Foo { x: 42 });

    // 1. immut ref to x
    // &i32
    let ref1 = &foo.x;  // de-sugars-to/same-as `&(*foo).x` aka deref coercion
    let ref ref2 = foo.x;
        // clippy: error: `ref` on an entire `let` pattern is discouraged


    // 2. mutating x
    let ref1 = &mut foo.x;      // &mut i32
    *ref1 = 55;

    let ref mut ref2 = foo.x;   // &mut i32 
        // clippy: error: `ref` on an entire `let` pattern is discouraged
    *ref2 = 77;

    let ref3 = foo.as_mut(); // &mut Foo
    ref3.x = 66;
        // same as (*ref3).x aka deref coercion
}



// 1. Basics

#[test]
fn ex1_box() {
    // a. With non-copy type String
    let p = Box::new("ab".to_string());
    // move
    let val = *p; // String - p's box dropped and inner obj. moved
        // let val = p.deref(); // &String
        // error[E0382]: borrow of moved value: `p`

    // read
    let p = Box::new("ab".to_string());
    let immut_ref: &String = &p; // &String aka Deref coercion (auto deref)
        // same as:
        let immut_ref = &p as &String;
    let immut_ref = p.deref(); // &String
    let immut_ref = p.as_ref(); // &String
    let immut_ref = &*p; // &String  - chose this? inline with mut vers.
    assert_eq!(immut_ref, "ab");

    // mutate
    let mut p = Box::new("ab".to_string());
    // a.1 choose this ?
    *p = "ff".to_string();
    assert_eq!(p.deref(), "ff");
    // a.2 or:
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
    *p = 44; // !! p is not consumed/moved since inner type is copy type
    assert_eq!(*p, 44);
}


// interv-2
//
// 2.
// Create MyBox generic tuple struct, impl fn new and fn val
// Impl std::ops::Deref for generic MyBox and use it



// ===============================================================


// task-2
// 2. Deref Trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> Self { Self(val) }
    fn val(&self) -> &T { &self.0 }
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
    assert_eq!(*p, 42); 
        // why *p is not &i32 ? because *p is actually *(p.deref())
    assert_eq!(p.val(), &42);

    let p = MyBox("ab".to_string());
    // let val = *p;  
        // yes, deref() is not sink fn but *p is actually *(p.deref())
    // error[E0507]: cannot move out of dereference of `MyBox<String>`
    // ^^ move occurs because value has type `String`, which does not
    // implement the `Copy` trait

    let refer = p.deref(); // &String
    let refer = &*p; // &String
        // let refer = p.as_ref(); 
            // &String // error: AsRef trait not impl for String
    let refer: &String = &p; // &String
    assert_eq!(refer, "ab");
}


fn hello(name: &str) { println!("Hello, {name}!"); }

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
    let mut sp = Rc::new(3);
    if let Some(p) = Rc::get_mut(&mut sp) {
        *p = 4;
    }
        // or
        // *Rc::get_mut(&mut sp).unwrap() = 4;
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
    let sp1 = Rc::new(3);
    let sp2 = Rc::clone(&sp1);

    // p1 and p2 are raw ptrs pointing into the same mem. address
    let p1 = Rc::as_ptr(&sp1); // *const i32
    println!("p1: {p1:p}");

    let p2 = Rc::as_ptr(&sp2);
    println!("p2: {p2:p}");

    assert_eq!(p1, p2);
    // or
    assert!( Rc::ptr_eq(&sp1, &sp2) );

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
    let sp = Rc::new(RefCell::new(String::from("ab"))); // Rc<Refcell<T>>
    assert_eq!(Rc::strong_count(&sp), 1);
    assert_eq!(Rc::weak_count(&sp), 0);

    // &T
    {
        let immut_ref = &*sp.borrow();  // &String
        assert_eq!(immut_ref, "ab");
    }

    // mutate T
    // !! sp does not have to be "mut" to be mutated which is the point
    *sp.borrow_mut() = "cd".to_string();
    assert_eq!(*sp.borrow(), "cd");
    // or
    sp.replace("er".to_string());
    assert_eq!(*sp.borrow(), "er");


    //// b. ref count: 2
    let sp_clone = Rc::clone(&sp); // Rc<Refcell<i32>>
    assert_eq!(Rc::strong_count(&sp), 2);
    assert_eq!(Rc::weak_count(&sp), 0);

    // Access &T and mutate T (same as above when ref count is 1)

    //// c. Access raw ptrs to T
    // !! RefCell's as_ptr is called, not Rc's - auto deref coercion ?
    assert_eq!(sp.as_ptr(), sp_clone.as_ptr());
    let p_raw = sp.as_ptr(); // *mut i32
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
    let foo_ref = sp.as_ref().borrow(); // Ref<'_, Foo>
    assert_eq!(&foo_ref.val, &42);
    drop(foo_ref);
    // !! we must drop foo since it refers to sp which will be moved into node
    // in the next section

    // Create weak_ptr from Rc, we could do it without the Option wrapper as well
    let node = Some(sp); // Option<Rc<RefCell<Foo>>>
    let weak_ptr = node.as_ref().map(Rc::downgrade); 
        // weak_ptr: Option<Weak<RefCell<Foo>>>
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
pub trait Messenger {
    fn send(&self, msg: &str);
    fn bad_send(&self, msg: &str);
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // self.sent_messages.push(String::from(message)); // 1
            // Err: `self` is a `&` reference, so the data it refers 
            // to cannot be borrowed as mutable
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
