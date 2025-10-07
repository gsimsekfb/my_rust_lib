// interv-1

// Auto derive Default for Foo with x,y,z,t : u8
// Create and use Foo new fn: literal vals for x,y ; using default values for z,t
// impl Default for Foo and use it


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