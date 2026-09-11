// interv-1

// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html#as-a-return-type
// https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#impl-trait

// 1
// fn get_closure which returns types which impl Fn trait that takes i32 
// and returns i32
// and in body it returns a closure that increments closure param with 1
// same with - get_closure_box with Box syntax
// Test these fns

// 2
// see below


/* what is impl Trait ?
impl Trait means "some *specific type* that implements MyTrait": When you write 
-> impl MyTrait, you're telling the compiler that the function will return 
*one specific*, concrete type that implements MyTrait. The caller doesn't know 
what that specific type is, but the function itself must consistently return 
the same type.

// e.g.
//
// Not possible with return impl, we can return only one type, see error:
//
// error[E0308]: `if` and `else` have incompatible types
// expected `Wii`, found `Foo`
//
// fn get_wii_or_foo_(cond: bool) -> impl MyTrait {
//     if cond { Wii { } }
//     else    { Foo { } }
// }
//
// !!! "return impl" is generic?
// No, only in argument position impl Trait create a generic parameter; 
// in return position it's an opaque, concrete type fixed by the 
// implementation, NOT generic.

*/


// 1
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



// interv-2

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




// ========================================================================





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
