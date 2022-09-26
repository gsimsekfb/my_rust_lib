fn main() {
    let num = 10;
    println!("{} plus one: {}!", num, add_one_lib::add_one(num));
}

// Sol-1
// #[path = "../../add-one/src/foo.rs"]
// mod foo;
// use crate::foo::foofn;

// Sol-2
// Use feature - see #usefoo

#[cfg(test)]
pub mod adder {
    #[test]
    pub fn test_add_one() {
        assert!(add_one_lib::foo::foofn() == 42);
    }
}