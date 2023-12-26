// https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants

trait MyTrait {
    const MY_CONST: u8; // Associated Constant
    fn bar();
}

struct Foo;

impl MyTrait for Foo {
    const MY_CONST: u8 = 42;
    fn bar() {}
}

#[test]
fn ex1() {
    assert_eq!(Foo::MY_CONST, 42);
}
