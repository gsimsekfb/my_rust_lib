/// 1. Refutability
// Irrefutable: int x = 42;
// Refutable: able to be proven false

#[test]
fn ex_1() {
    let result = Some(42);

    // a. Err: Refutable pattern in local binding
    // let Some(x) = res;
    // error[E0005]: refutable pattern in local binding: `None` not covered

    // b. !!! Using if let instead of match here 
    // (Ok: Refutable pattern can be used with 'if let')
    if let Some(x) = result {
        assert_eq!(x, 42);
    }
    // instead of
    match result {
        Some(x) => assert_eq!(x, 42),
        _ => (),
    }

    // c. Irrefutable since it is always true/matches
    // if let x = 5 {
    //     println!("{}", x);
    // }
    // warning: irrefutable `if let` pattern

    // d. let else - since rust 1.66
    let x = match result {
        Some(val) => val,
        None => return,
    };
    assert_eq!(x, 42);
    // Match above, can be written:
    let Some(x) = result else {
        return;
        // error[E0308]: `else` clause of `let...else` does not diverge
        // println!("err");
        // help: try a diverging expression, e.g. `return` or `panic!()`
    };
    assert_eq!(x, 42); // !! x is still in scope here
}

/// 2. Multiple Patterns

#[test]
fn ex_2() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// 3. Matching Ranges of Values with the . . . Syntax
#[test]
fn ex_3() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        // Ranges are only allowed with numeric values or char values!
        _ => println!("something else"),
    }
}

/// 4
// 4.a. Destructuring Structs

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn ex_4_a() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);
    // or simpler
    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    // create a var with match
    let val = match p {
        Point { x, y: 0 } => 1, // x is any, y is 0
        Point { x: 0, y } => 2,
        Point { x, y } => 3, // !!! covers all cases
                             // if last line is commented,
                             // error[E0004]: non-exhaustive patterns:
                             // help: ensure that all possible cases are being handled by adding
                             // a match arm with a wildcard pattern as shown, or multiple match arms
    };
    assert_eq!(val, 2);

    // Ignoring Remaining Parts
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let p2 = Point2 { x: 42, y: 0, z: 0 };
    let val = match p2 {
        Point2 { x: 30, .. } => 11,
        Point2 { x, .. } => 22,
    };
    assert_eq!(val, 22);
}

// 4.b. Destructuring Enums

enum Message_1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn ex_4_b() {
    let msg1 = Message_1::ChangeColor(0, 160, 255);

    match msg1 {
        Message_1::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message_1::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message_1::Write(text) => println!("Text message: {}", text),
        Message_1::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// 4.c. Destructuring Nested Structs and Enums

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[test]
fn ex_4_c() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

/// 5. Match Guards
#[test]
fn ex_5() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

/// 6. @ Bindings
#[test]
fn ex_6() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 7 };
    match msg {
        // id is not in scope, hence we use id_ and @
        Message::Hello { id: id_ @ 3..=7 } => {
            println!("Found an id in a range, id: {}", id_)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
            // println!("Found an id in another range, id: {}", id)
            // Err: not found in this scope
        }
        Message::Hello { id } if id > 0 => {
            println!("Found non-zero id: {}", id)
        }
        Message::Hello { id } if (20..=40).contains(&id) => {
            println!("Found non-zero id: {}", id)
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
//// 7. w/ ranges
#[test]
fn ex_7() {
    let n = 20;
    let str = match n {
        1..=20 => "<=20",
        _ => "20>",
    };
    assert_eq!(str, "<=20");
}