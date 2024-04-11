struct Foo {
    x: i32
}

#[test]
fn ex1_option_swap_inner_value() {
    // Map Option<Foo> into Option<i32>
    let foo = Some(Foo { x: 42 }); // Option<Foo>
    let foo_x = foo.map(|f| f.x);  // Option<i32>

    assert_eq!(foo_x, Some(42));
}