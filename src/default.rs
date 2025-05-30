// interv-1
// Derive Default for Foo x,y,z,t : u8
// Create a Foo: literal vals for x,y ; using default values for y,z


// ---------------------------------------


#[derive(Debug, Default, PartialEq)]
struct Foo {
    x: i32,
    // d: chrono::Date<chrono::Utc>,
        // If a field is no Default, Foo definition itself will cause error.
        // err: the trait `Default` is not implemented for `Date<Utc>
    s: String,
    w: Option<bool>,
    y: i32,
}

#[test]
fn ex1() {
    let foo = Foo { x: 42, y: 33, ..Default::default() };
    assert_eq!(foo, Foo { x: 42, s: "".to_string(), w: None, y: 33 });

    let foo: Foo = Default::default();
    assert_eq!(foo, Foo { x: 0, s: "".to_string(), w: None, y: 0 });
}