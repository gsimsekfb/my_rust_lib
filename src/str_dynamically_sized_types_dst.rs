// interv
// &str vs &T - memory layout and size


//// 1. str is a special type: DST
// DST:
// https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
//
// dst: Dynamically sized types aka DSTs or unsized types these types let us 
// write code using values whose size we can know only at runtime

#[test] fn ex() {
    // Problem
    // let s1: str = "Hello there!";    // 12 bytes
    // let s2: str = "How's it going?"; // 15 bytes
        // Error: Rust needs to know how much memory to allocate for any 
        // value of a particular type, 
        // and ALL values of a type must use the SAME amount of memory.
        // error[E0277]: the size for values of type `str` cannot be known at
        // compilation time

    // Sol: use &str (a slice: ptr + len) instead of str
    let s1: &str = "Hello there!";    // Size: 16 (2 usize, ptr + len)
    let s2: &str = "How's it going?"; // Size: 16 (2 usize, ptr + len)
    assert_eq!(std::mem::size_of::<&str>(), 16); // size of &str: ptr + len   
    assert_eq!(std::mem::size_of::<&i32>(),  8); // size of  &T : ptr !!
}

// So although a &T is a single value that stores the memory address of 
// where the T is located, &str is two values: the addr of the str and its length. 
// As such, we can know the size of a &str value at compile time: 
// it’s twice the length of a usize
// That is, we always know the size of a &str
// In general, this is the way in which dynamically sized types are used in Rust:
// they have an extra bit of metadata that stores the size of the dynamic info.
// The golden rule of dynamically sized types is that we must always put values
// of dynamically sized types behind a pointer of some kind.
// Every trait is a dynamically sized type.


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
