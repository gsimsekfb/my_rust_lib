
// interv-1

// 1. Write a simple generic fn takes T and returns T.to_string
//   - use 3 different identical ways to write generic fn (hint: at the end)
//   - test: call fn with explicit type param and without

// see more below


// ==============================================================


//// Generics


// x 
// Generic traits - see in traits_and_generics.rs

// 1
// Generic fn

struct Gen<T> { val: T }
struct Woo {}

// not generic
fn f1(f: Gen<Woo>) { }

// not generic
fn f2(f: Gen<i32>) { }

// generic over `T`
fn f3<T>(g: &Gen<T>) -> i32 { 42 }

#[test] fn ex_1() {
    let g = Gen { val: 0 };
    assert_eq!(f3(&g), 42);
    assert_eq!(f3::<i32>(&g), 42);
}




// 2. Generic struct GenericVal with generic field val
//   - getter method val
//   - for GenericVal<i32> getter methods val
//   - test: 
//   - create a GenericVal with explicit type param and without




// Generic struct and method

struct Wii; // Concrete type `Wii`

struct GenVal<T> { gen_val: T } // Generic type `GenericVal`

impl<T> GenVal<T> { 
    fn val(&self) -> &T { &self.gen_val } 
        // !! Note: NOT a generic method, just a method of a generic struct.
        // Reason: fn val does not introduce a new type parameter.
    // fn generic_fn<U>(x: U) -> U { x }  // This is a generic function.

}

impl GenVal<i32> {
    // !!! Compiler already implements this for us due to impl GenVal<T>
    // error[E0592]: duplicate definitions with name `val`
    // fn val(&self) -> i32 { self.gen_val + 10 }
    fn val_(&self) -> i32 { self.gen_val + 10 }
}

impl GenVal<Wii> {}

#[test] fn ex_2() {
    // GenericVal<i32>
    let val = GenVal        { gen_val: 2 };
    let val = GenVal::<i32> { gen_val: 2 };
    assert_eq!(val.val(), &2);
    assert_eq!(val.val_(), 12);
}




// 3
// Generic Enum

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
    let oo = MyOption::Some(42);
    // same
    let oo = MyOption::<i32>::Some(42);

    let rr = MyResult::<i32, Error>::Ok(42);

    // assert_eq!(f1, f2);
    // assert_eq!(f1, f3); // error[E0308]: mismatched types
}



// interv-2

// 4. Generic struct Foo with types A,B and 
//    with one normal w/ A and one Phantom data member w/ B
//   - test: create Foos with same A and diff B's



/// 
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
struct Foo<A, B> { 
    x: A,
    y: PhantomData<B> 
}

#[test] fn ex_4() {
    let f1 = Foo { x: 11, y: PhantomData::<u8>};
    let f2 = Foo { x: 11, y: PhantomData::<u8>};
    let f3 = Foo { x: 33, y: PhantomData::<u32>};

    assert_eq!(f1, f2); // ok: same types
    // assert_eq!(f1, f3); // error[E0308]: mismatched types
}


// Hint: impl, <T: Trait>, where clause