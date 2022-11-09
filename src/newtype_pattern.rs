use std::fmt;

// Using the Newtype Pattern to Implement External Traits on External Types:
//
// The orphan rule that states we’re allowed to implement a trait on a type as
// long as either the trait or the type are LOCAL to our crate. It’s possible to get
// around this restriction using the newtype pattern, which involves creating
// a new type in a tuple struct.

// Creating a Wrapper type around Vec<String> to implement Display
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test] fn ex_1() {
    let wrapper = Wrapper(vec![String::from("a"), String::from("b")]);
    // println!("w = {}", wrapper);
    assert_eq!(wrapper.to_string(), "[a, b]")
}