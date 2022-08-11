pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod add_one {
    use crate::add_one;

    #[test]
    fn basic_test() {
        assert_eq!(add_one(4), 5);
    }
}