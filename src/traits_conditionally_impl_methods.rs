// interv-1

// generic struct Val with val T
// impl fn new, 
// impl to_str method only for Val<T>s where T impl Display trait
// struct A empty
// test: use these methods for Val i32 and Val A (expect error for A)



// =====================================================================




//// Traits to Conditionally Implement Methods
//// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

struct Val<T> { val: T }

impl<T> Val<T> {
    fn new(val: T) -> Self { Self { val } }
}

// Compiler to auto-impl to_str method only for Ts that impl Display trait
impl<T: std::fmt::Display> Val<T> {
    fn to_str(&self) -> String { self.val.to_string() }
}

struct A { }

#[test] fn ex() {
    let v = Val::new(42);
    assert_eq!(v.to_str(), "42");

    let v = Val::new(A {});
    // assert_eq!(v.to_str(), "A"); // err since A does not impl Display
        // error[E0599]: the method `to_str` exists for struct `Val<A>`, 
        // but its trait bounds were not satisfied
}
