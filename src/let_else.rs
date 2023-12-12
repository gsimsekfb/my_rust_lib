use std::str::FromStr;

fn get_number(s: &str) -> u64 {
    // Using it like an "early return"
    let Ok(count) = u64::from_str(s) else {
        panic!("Can't parse integer: '{s}'");
    };
    // ...
    count
}

#[test] 
fn ex1_simple_works() {
    assert_eq!(get_number("3"), 3);
}

#[test]
#[should_panic]
fn ex2_panics() {
    assert_eq!(get_number("aa"), 3);
}