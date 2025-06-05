// interv-1

// 1. Impl std::fmt::Display for Vec<String>, check the hint below after or before 
//    deciding how to impl





// =========================================================================





// Hint:
// Using the Newtype Pattern to Implement External Traits on External Types:
//
// The orphan rule that states we’re allowed to implement a trait on a type as
// long as either the trait or the type are LOCAL to our crate. It’s possible to get
// around this restriction using the newtype pattern, which involves creating
// a new type in a tuple struct.

// Src: 
// https://www.lurklurk.org/effective-rust/newtype.html




// Ex.1 - Creating a Wrapper type around Vec<String> to implement Display
//        aka by-passing the orphan rule
struct Wrapper(Vec<String>);

use std::fmt;
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test] 
fn ex_1() {
    let wrapper = Wrapper(vec![String::from("a"), String::from("b")]);
    // println!("w = {}", wrapper);
    assert_eq!(wrapper.to_string(), "[a, b]")
        // Trait ToString::to_string() is automatically implemented for any type
        // which implements the [`Display`] trait    
}



// Ex.2 - Use explicitly named types instead of lang. types when needed.
//        Both for readability and type safety.

mod foo {
    pub struct Seconds(pub i32);

    // e.g.
    // Use Seconds
    pub fn to_seconds() -> Seconds {
        Seconds(42)
    }
    // instead of i32 
    pub fn to_seconds_() -> i32 {
        66
    }
}

#[test]
fn ex_2() {
    assert_eq!(foo::to_seconds().0, 42);
    assert_eq!(foo::to_seconds_(), 66);
}
