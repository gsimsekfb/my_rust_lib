
#[cfg(feature = "foo")]
const XX : i32 = 22;
#[cfg(not(feature = "foo"))]
const XX : i32 = 44;

// Using const fn instead of cfg! syntax
pub const fn feature_foo_enabled() -> bool {
    cfg!(feature = "foo")
}

// HINT:
// Tests ex0, 1 and ex2 must fail when feature foo is enabled
// cargo t -F foo cfg           // -F: --features

#[test]
fn ex_0_feature_foo_enabled() {
    assert!(!feature_foo_enabled());
}

#[test]
fn ex_1_should_fail_with_feature_foo() {
    assert_eq!(XX, 44);
}

// Using: if cfg!(feature = "foo")
#[test] fn ex_2_using_if_cfg_and_should_fail_with_feature_foo() {
    if cfg!(feature = "foo") {
        assert_eq!(42, 4);
    } else {
        assert_eq!(42, 42);
    }
}

// Will fail with 
// cargo t --features "foo aa" cfg_all
// More:
// https://doc.rust-lang.org/reference/conditional-compilation.html#conditional-compilation
#[test] fn ex_3_cfg_all() {
    #[cfg(all(feature= "foo", feature= "aa"))]
    assert_eq!(42, 43);
}