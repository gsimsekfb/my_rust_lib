
// interv-1

// - fn which accepts both sized and unsized types
// - list 3 most common unsized types
// - see also dst.rs (same topic)


// -------------------------------------------------------------------------------



// 1. Sized means the compiler knows the exact byte size at "compile time":

// i32      → always 4 bytes  ✓
// String   → always 24 bytes ✓
// str      → ??? bytes       ✗ (depends on content)
    // &str (a slice) is fat ptr (ptr + len), str is the data itself,
    // of which size is unknown at "compile time"
// [i32]    → ??? bytes       ✗ (depends on length)
    // same as &str, &[i32] (a slice) e.g. &arr[..] is fat ptr (ptr + len), 
    // arr[..] is the unsized data itself

// The references (&str, &[i32]) are always sized — they're fat pointers 
// (pointer + length). But the thing they point to (str, [i32]) is not. 
// T in ?Sized refers to the pointee, not the pointer.



// 2. ?Sized means type may be unsized, accept it
// The key ones are str, [T], and dyn Trait — these are the common unsized types
// you'd actually encounter.

// example:

// accept all (both sized and unsized) types
fn foo<T: ?Sized>(t: &T) {  }

trait MyTrait {}
impl MyTrait for i32 {}

#[test]
fn tt() {
    // Using w/ unsized types only possible with ?Sized:
    foo("hello");          // T = str        (unsized)
    foo(&[1, 2, 3][..]);   // T = [i32]      (unsized slice)
    let f1: &dyn MyTrait = &42;
    foo(&f1);     // T = dyn Display (trait object, unsized)

    // Using w/ sized types works with or without ?Sized:
    foo(&42);              // T = i32
    foo(&String::new());   // T = String
    foo(&vec![1,2,3]);     // T = Vec<i32>
}

