fn add_const() -> i32 {
    1
}

pub fn add_one(x: i32) -> i32 {
    x + add_const()
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn add_const_should_be_1() {
        assert_eq!(1, add_const());
    }
}