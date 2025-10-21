// interv-1
// - create a mutable and immutable raw ptr to i32 (there are 2 ways to it)
// - create a ptr to i32 from address 123400
// - deref each ptrs
// - unsafe fn which derefs ptr to i32 and returns its value. Use it.



// ---------------------



// https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html
//
//// 5 Unsafe Superpowers
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of a union
// Note: Use Miri to Check Unsafe Code - Miri is the official Rust tool
//         for detecting undefined behavior at runtime

#[test] 
fn unsafe_() {
    // 1. Dereference a raw pointer
    let mut num = 5;

    let ptr1 = num as *const i32; // *const i32
    let ptr1 = &raw const num;    // *const i32

    let ptr2 = num as *mut i32;   // *mut i32
    let ptr2 = &raw mut num;      // *mut i32

    let addr = 0x123400usize;
    let ptr = addr as *const i32; // *const i32 

    unsafe { 
        println!("*ptr1: {}", *ptr1);   // ok
        println!("*ptr2: {}", *ptr2);   // ok
        // println!("*ptr: {}", *ptr)cc;     // ok but possibly runtime panic
        assert_eq!(*ptr1, 5);
    }

    // 2. Call an unsafe function or method
    // rustc: an unsafe fn restricts its caller, but its body is safe by default
    unsafe fn unsafe_foo(ptr: *const i32) -> i32 {
        unsafe { *ptr }
    }
        // safe version (note: null check may not be enough):    
        // fn foo(ptr: *const i32) -> i32 {
        //     assert!(!ptr.is_null()); // safety check
        //     unsafe { *ptr }
        // }
    
    unsafe { unsafe_foo(ptr1); }
    unsafe { unsafe_foo(ptr1); } 
        // ok: Raw pointer does not impl Rust’s ownership/move semantics
        //     so it is copied here
}
