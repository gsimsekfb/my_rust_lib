// interv-1

// Trait Associated Constants
// https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants

// Trait MyTrait which has:
// const u8 
// const i32 with default value
// fn bar with no params, no return type
// Impl MyTrait for unit (empty) struct Foo, override assoc. constant w/
// default value
// Test:
// assert Foo's const u8
// assert Foo's const i32





// ========================================================================





trait MyTrait {
    const MY_CONST: u8;                 // Associated Constant
    const CONST_WITH_DEFAULT: i32 = 99; // Associated Constant w/ default value
    fn bar();
}

struct Foo;

impl MyTrait for Foo {
    const MY_CONST: u8 = 42;
    const CONST_WITH_DEFAULT: i32 = 100;
    fn bar() {}
}

#[test]
fn ex1() {
    assert_eq!(Foo::MY_CONST, 42);
    assert_eq!(Foo::CONST_WITH_DEFAULT, 100);
}
