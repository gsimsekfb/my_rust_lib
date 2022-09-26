fn main() {
    let num = 10;
    println!("{} plus one: {}!", num, add_one::add_one(num));
}

// Sol-1
// #[path = "../../add-one/src/foo.rs"]
// mod foo;
// use crate::foo::foofn;

// Sol-2
// Use feature - see #usefoo

#[test]
pub fn test_add_one() {
    assert!(add_one::foo::foofn() == 42);
}