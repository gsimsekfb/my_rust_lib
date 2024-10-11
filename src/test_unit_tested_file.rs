
// Unit Tests
// The purpose of unit tests is to test each unit of code in isolation from 
// the rest of the code to quickly pinpoint where code is and isn’t working 
// as expected. You’ll put unit tests in the src directory in each file with 
// the code that they’re testing. 
// Src: 
// https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests
    
fn add_const() -> i32 {
    1
}

pub fn add_one(x: i32) -> i32 {
    x + add_const()
}

#[cfg(test)] // the code only gets built in test configurations
mod unit_tests {
    use super::*;

    #[test]
    fn add_const_should_be_1() {
        assert_eq!(1, add_const());
    }
}
