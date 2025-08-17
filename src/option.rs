// interv-1

// 1
// Foo with an int
// Create from Some(Foo) -> Some(Foo.x) elegantly

// 2
// Create num_raw Option<u8> from nested option<enum>
// - enum is simple Number enum with 1 and 2 
// - if enum is one, num_raw is Some(1), otherwise None  



// ==========================================================




struct Foo { x: i32 }

#[test]
fn ex1_make_new_from_inner_value() {
    // from Option<Foo> -> Option<Foo.x>
    let foo = Some(Foo { x: 42 }); // Option<Foo>
    let foo_x = foo.map(|f| f.x);  // Option<i32>

    assert_eq!(foo_x, Some(42));
}

enum Num { One=1, Two=2}

#[test]
fn ex2_nested_option_and_enum() {
    let inner = Num::One;
    let outer = Some(inner);

    let num_raw: Option<u8> = outer.and_then(|inner| {
        match inner {
            Num::One => Some(Num::One as u8),
            _ => None
        }
    });
    assert_eq!(num_raw, Some(1));

    //
    let inner = Num::Two;
    let outer = Some(inner);
    
    let num_raw: Option<u8> = outer.and_then(|inner| {
        match inner {
            Num::One => Some(Num::One as u8),
            _ => None
        }
    });
    assert_eq!(num_raw, None);
}
