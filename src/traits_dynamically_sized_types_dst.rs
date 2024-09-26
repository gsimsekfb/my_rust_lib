
// - Every trait is a dynamically sized type we can refer to by
//   using the name of the trait


//// Dynamically sized types and the Sized trait
//// https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait

//// Dynamically sized types aka DSTs or unsized types
//// these types let us write code using values whose
//// size we can know only at runtime

// 1

#[test] fn ex_dst() {
    // Problem
    // let s1: str = "Hello there!";    // 12 bytes
    // let s2: str = "How's it going?"; // 15 bytes
        // Error: Rust needs to know how much memory to allocate for any 
        // value of a particular type, 
        // and ALL values of a type must use the SAME amount of memory.
        // error[E0277]: the size for values of type `str` cannot be known at
        // compilation time

    // Sol: use &str instead of str
    let s1: &str = "Hello there!";    // Size: 16 (2 usize, ptr + len)
    let s2: &str = "How's it going?"; // Size: 16 (2 usize, ptr + len)
    assert_eq!(std::mem::size_of::<&str>(), 16); // size of &str   
    assert_eq!(std::mem::size_of::<&i32>(),  8); // size of &T
}

// 2
fn generic<T>(t: T) { }
// is actually treated as though we had written this:
fn generic_<T: Sized>(t: T) {
    // Rust implicitly adds a bound on Sized to every generic function. 
}
// By default, generic functions will work only on types that have a known 
// size at compile time. 
// However, we can use the following special syntax to relax this restriction:
fn generic__<T: ?Sized>(t: &T) { // !!! changed to &T from T
    // A trait bound on ?Sized means “T may or may not be Sized” and
    // this notation overrides the default that generic types must have
    // a known size at compile time. The ?Trait syntax with this meaning is 
    // only available for Sized, not any other traits.  
    //
    // Also, we switched param from T to &T. Because the type might not be Sized,
    // we need to use it behind some kind of ptr. In this case, we chose a ref.
}
