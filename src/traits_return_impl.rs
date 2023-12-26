
// https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#impl-trait

trait Trait { }
impl Trait for i32 { }
impl Trait for f32 { }

// Use this
// Note: This doesn't create a trait object, it's like we had written -> i32, 
// but instead, we're only mentioning the part about Trait. 
// We get static dispatch, but we can hide the real type like this.
fn foo() -> impl Trait {
    5
}
// instead of 
fn foo_() -> Box<dyn Trait> {
    Box::new(5) as Box<dyn Trait>
}

//// Real word usage:
// before
fn get_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
// after
fn get_closure_() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

#[test] 
fn ex1() {
    let xx = foo();
    let yy = foo_();
    // assert_eq!(xx, 5); // todo
}