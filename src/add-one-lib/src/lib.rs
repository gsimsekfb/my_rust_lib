

// Problem: cfg test is not accessible from other packages
// #[cfg(test)]
// pub mod foo;
//
// Sol:
// Until this issue is solved https://github.com/rust-lang/cargo/issues/8379,
// using this small hack:
// Make some test modules available for "use-foo" feature
// #usefoo - enable this line when this feature is enabled 
//           (can be enabled by client's Cargo.toml or compiler flag)
#[cfg(feature = "use-foo")]
pub mod foo;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
pub mod add_one_lib {
    use crate::add_one;

    #[test]
    fn basic_test() {
        assert_eq!(add_one(4), 5);
    }
}