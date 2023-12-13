use std::str::FromStr;

fn process_str(s: &str) -> u64 {
    // Using it like an "early return"
    let Ok(count) = u64::from_str(s) else {
        panic!("Can't parse integer: '{s}'");
    };
    // instead of
    let res = u64::from_str(s);
    if res.is_err() {
        return 42
    }
    let count = res.unwrap();
    // ... do_something_with(count)
    count
}

#[test] 
fn ex1_simple_works() {
    assert_eq!(process_str("3"), 3);
}

#[test]
#[should_panic]
fn ex2_panics() {
    assert_eq!(process_str("aa"), 3);
}