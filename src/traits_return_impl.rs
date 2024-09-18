
// https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#impl-trait

// Returning -> impl Trait

// 1
// fn get_closure which returns types which impl Fn trait that takes i32 and returns i32
// and returns a closure that increments the param  1
// same with - get_closure_box with Box syntax
// Test these fns

// return concrete types that don't actually have a name you could type out, 
// e.g. closure (every closure has its own type)
fn get_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
// instead of
fn get_closure_box() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
// error
// = help: every closure has a distinct type and so could not always match the caller-chosen type of parameter `T`
// fn get_closure_gen<T>() -> T where T: Fn(i32) -> i32 {
//     |x| x + 1
// }


// 2
// trait MyTrait   with fn name_ with default impl, returns &str
// trait MyTrait_2 with fn age_  with default impl, returns u32
// struct Foo with x i32, fn get_x returns x
// Impl MyTrait for Foo, empty
// Impl MyTrait_2 for Foo, empty
// fn get_foo that returns types that impl MyTrait, returns a Foo obj
// same with - fn get_foo_box with Box
// fn get_foo_gen that returns types that impl MyTrait, returns a Foo obj
//
// struct Wii with x i32
// Impl MyTrait for Wii, empty
// fn get_wii_or_foo takes a bool "a" that returns types that impl MyTrait, 
// if a true returns a Wii obj, else Foo obj - understand error
// same with - fn get_wii_or_foo_box using Box
// Get f obj from get_wii_or_foo_box. With f, call MyTrait fn

trait MyTrait   { fn name_(&self) -> &'static str { "my-trait" } }
trait MyTrait_2 { fn age_(&self) -> u32 { 42 } }

struct Foo { x: i32 }
impl Foo { fn get_x(&self) -> i32 { self.x } }
impl MyTrait for Foo { fn name_(&self) -> &'static str { "Foo"} }
impl MyTrait_2 for Foo { }

// !!! We can only access MyTrait features of Foo
// !!! We cannot access Foo data/fns or any other trait impls of Foo e.g MyTrait_2
// Note: This doesn't create a trait object, it's like we had written 
// -> Foo but only MyTrait section
// We get static dispatch, but we can hide the real type with this.
// return type: Foo as MyTrait or Foo's MyTrait section
fn get_foo() -> impl MyTrait {
    Foo { x: 42 }
}
// instead of
fn get_foo_box() -> Box<dyn MyTrait> {
    Box::new(Foo { x: 42 })
}
// get_foo is not equal to
// fn get_foo_gen<T: MyTrait>() -> T {
//     Foo { x: 42 }
// }
    // err:
    // expected type parameter `T`, found `Foo`
// 
struct Wii { x: i32 }
impl MyTrait for Wii { fn name_(&self) -> &'static str { "Wii"} }
// error[E0308]: `if` and `else` have incompatible types
// expected `Wii`, found `Foo`
// fn get_wii_or_foo(a: bool) -> impl MyTrait {
//     if a { Wii { x: 42 } }
//     else { Foo { x: 33 } }
// }
// Ok
fn get_wii_or_foo_box(a: bool) -> Box<dyn MyTrait> {
    if a { Box::new( Wii { x: 44 } ) }
    else { Box::new( Foo { x: 33 } ) }
}

#[test] fn ww() {

    // 1
    let f = get_closure();
    assert_eq!(f(1), 2);
    let f = get_closure_box();
    assert_eq!(f(1), 2);

    // 2
    // Only MyTrait is accessible
    let foo_as_my_trait = get_foo();
    assert_eq!(foo_as_my_trait.name_(), "Foo");
    //
    // No access to Foo or MyTrait_2 
    // error[E0599]: no method named `get_x` found for opaque type `impl MyTrait`
    // let xx = foo_as_my_trait.get_x();
    // let xx = foo_as_my_trait.age_(); // same error
    //
    // As expected, with Foo obj, we have access to all
    let foo = Foo { x: 10 };
    assert_eq!(foo.name_(), "Foo");
    assert_eq!(foo.get_x(), 10);
    assert_eq!(foo.age_(), 42);
    //
    let foo_as_my_trait = get_foo_box();
    assert_eq!(foo_as_my_trait.name_(), "Foo");
    //
    let wii = get_wii_or_foo_box(true); // Only MyTrait accessible thru wii
    assert_eq!(wii.name_(), "Wii");
}