// interv-1
// - create a mutable and immutable raw ptr to i32 (there are 2 ways to it)
// - create a ptr to i32 from address 123400
// - deref each ptrs
// - unsafe fn which derefs ptr to i32 and returns its value. Use it.



// ---------------------


// "Rules of Thumb" for using unsafe:

// 1. Can the fn crash if given "bad" input? If yes, mark the function unsafe fn.
// 2. Doing something tricky (like manual memory management) but the outside 
//    world can't possibly break it, use a safe fn with an internal unsafe block.
//
// Examples:
//
// 1. unsafe fn:
// Safety: Callers should check if the ptr p is valid
unsafe fn foo_unsafe(p: *const i32) -> i32 { *p }

//
// 2. safe fn "(but since it contains unsafe block, not the safest fn)
//    Here we guarantee safety inside the function so the caller doesn't 
//    have to worry about it
fn foo_safe(p: *const i32) -> i32 {
    if p.is_null() { return 0; /* Or handle the error */ }
    
    unsafe { *p } // We've done our checks, so we "hide" the unsafety
}



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
