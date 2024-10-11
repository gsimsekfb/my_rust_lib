// interv


// Foo with an int
// Create from Some(Foo) -> Some(Foo.x) elegantly

struct Foo { x: i32 }

#[test]
fn ex1_make_new_from_inner_value() {

    // from Option<Foo> -> Option<Foo.x>
    let foo = Some(Foo { x: 42 }); // Option<Foo>
    let foo_x = foo.map(|f| f.x);  // Option<i32>

    assert_eq!(foo_x, Some(42));
}
