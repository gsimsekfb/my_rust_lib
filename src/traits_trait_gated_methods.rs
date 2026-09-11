// interv-1

// generic struct Val with val T
// impl fn new, 
// impl to_str method only for Val<T>s where T impl Display trait
// struct A empty
// test: use these methods for Val i32 and Val A (expect error for A)



// =====================================================================




//// Trait-Gated Methods aka Traits to Conditionally Implement Methods
//// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

struct Val<T> { val: T }

impl<T> Val<T> {
    fn new(val: T) -> Self { Self { val } }
}

// Trait gated method:
// Enable `to_str` impl./body for a Val<T> instance, only when `T` is `Display`
impl<T: std::fmt::Display> Val<T> {
    fn to_str(&self) -> String { self.val.to_string() }
}

struct A { }

#[test] fn ex() {
    let v = Val::new(42);
    assert_eq!(v.to_str(), "42");

    let v = Val::new(A {});
    /*
    // Err since A does not impl Display, so to_str fn is gated/forbidden
    // for Val<A> instance v.
    assert_eq!(v.to_str(), "A");
        // error[E0599]: the method `to_str` exists for struct `Val<A>`, 
        // but its trait bounds were not satisfied
    */
}
