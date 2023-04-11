
#[cfg(feature = "foo")]
const XX : i32 = 22;
#[cfg(feature = "abc")]
const XX : i32 = 33;
#[cfg(not(feature = "foo"))]
const XX : i32 = 44;

// HINT:
// Both tests must fail when foo used like this:
// cargo t cfg --features foo

#[test] fn ex_1_should_fail_with_foo() {
    assert_eq!(XX, 44);
}
// Using: if cfg!(feature = "foo")
#[test] fn ex_2_using_if_cfg_and_should_fail_with_foo() {
    if cfg!(feature = "foo") {
        assert_eq!(42, 4);
    } else {
        assert_eq!(42, 42);
    }
}