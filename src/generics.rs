//// Generics


// x 
// Generic traits - see in traits_and_generics.rs

// 1
// Generic fn
struct Gen<T> { val: T }
struct Foo {}

// not generic
fn f1(f: Gen<Foo>) { }

// not generic
fn f2(f: Gen<i32>) { }

// generic over `T`.
fn f3<T>(f: Gen<T>) { }



// 2 
// Generic struct and method

struct Wii; // Concrete type `Wii`

struct GenericVal<T> { gen_val: T } // Generic type `GenericVal`

impl<T> GenericVal<T> { 
    fn gen_val(&self) -> &T { &self.gen_val } // Generic method 
}

impl GenericVal<i32> {
    // !!! Compiler already implements this for us due to impl GenVal<T>
    // error[E0592]: duplicate definitions with name `gen_val`
    // fn gen_val(&self) -> i32 { self.gen_val + 10 }
    fn val(&self) -> i32 { self.gen_val + 10 }
}

impl GenericVal<Wii> {}

#[test] fn ex_2() {
    // GenericVal<i32>
    let val = GenericVal        { gen_val: 2 };
    let val = GenericVal::<i32> { gen_val: 2 };
    assert_eq!(val.gen_val(), &2);
    assert_eq!(val.val(), 12);
}


/// 3
/// Generic Enum

enum MyOption<T> {
    Some(T),
    None
}
impl<T> MyOption<T>   { fn name()     -> &'static str { "MyOption<T>"   } }
impl    MyOption<i32> { fn name_i32() -> &'static str { "MyOption<i32>" } }
// error[E0592]: duplicate definitions with name `name`
// impl    MyOption<i32> { fn name()     -> &'static str { "MyOption<i32>" } }

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
impl<T,E> MyResult<T,E>   { fn name()     -> &'static str { "MyResult<T>"   } }
impl<E>   MyResult<i32,E> { fn name_i32() -> &'static str { "MyResult<i32>" } }


#[test] fn ex_3() {
    let e = MyOption::Some(42);
    let e = MyOption::<i32>::Some(42);

    let r = MyResult::<i32, Error>::Ok(42);

    // assert_eq!(f1, f2);
    // assert_eq!(f1, f3); // error[E0308]: mismatched types
}


/// 4
/// Phantom type parameters
/// https://doc.rust-lang.org/rust-by-example/generics/phantom.html
//
// A phantom type parameter is one that doesn't show up at runtime, but is 
// checked statically (and only) at compile time.
//
// Data types can use extra generic type parameters to act as markers or 
// to perform type checking at compile time. These extra parameters hold 
// no storage values, and have no runtime behavior.

use std::{fmt::{Display, Error}, marker::PhantomData, ops::Add};

#[derive(Debug, PartialEq)]
struct PhantomFoo<A, B> { 
    x: A,
    y: PhantomData<B> 
}

#[test] fn ex_4() {
    let f1 = PhantomFoo { x: 11, y: PhantomData::<u8>};
    let f2 = PhantomFoo { x: 11, y: PhantomData::<u8>};
    let f3 = PhantomFoo { x: 33, y: PhantomData::<u32>};
    assert_eq!(f1, f2);
    // assert_eq!(f1, f3); // error[E0308]: mismatched types
}
