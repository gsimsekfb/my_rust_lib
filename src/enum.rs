// interv
// enum Num with One, Two with underlying type u8


// 1 Defining enums

// 1.a. Simple examples
// enum with implicit discriminator (starts at 0)
enum Number { Zero, One, Two } // underlying values: 0, 1, 2

enum E1 {
    One = 1,  // type: E1, underlying type: isize
    Two = 2
}

#[repr(u8)]
enum E2 {
    One = 1, // type: E2, underlying type: u8
    Two = 2,
}

enum E3 {
    One(i32),
    Two(u32)
}

enum IpAddr    { V4,                 V6 }  // V4: 0, V6: 1
enum IpAddr_   { V4(String),         V6(String) }
enum IpAddr__  { V4(u8, u8, u8, u8), V6(String) }
struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddr___ { V4(Ipv4Addr),       V6(Ipv6Addr) }


// 1.b Enum w/ types embedded in its variants.
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Move has named fields, like a struct
    Write(String),
    ChangeColor(i32, i32, i32),
}
// struct equivalents
struct Quit;                       // unit struct
struct Move { x: i32, y: i32 }     // struct
struct Write(String);              // tuple struct
struct ChangeColor(i32, i32, i32); // tuple struct


#[test] fn ex_1() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 2, y: 2 };
    let m3 = Message::Write("abc".to_string());
    let m4 = Message::ChangeColor(4,5,6);

    // Things we can use instead of match

    // a. if let - for Some() case
    if let Message::Move { x, y } = m2 {
        assert_eq!(x as i32, 2);
    };
    if let Message::Write(val) = &m3 {
        assert_eq!(val, "abc");
    };
    if let Message::ChangeColor(_,y,_) = &m4 {
        assert_eq!(y, &5);
    };

    // b. let else - for None case e.g. early returns
    let Message::Write(val) = m3 else {    // let else
        panic!("empty Msg");
        // error[E0308]: `else` clause of `let...else` does not diverge
        // println!("err");
            // help: try a diverging expression, e.g. `return` or `panic!()`
    };
    assert_eq!(val, "abc"); // !! val is still valid here
}


// 2. impl Enum is same as structs
enum Ip { V4, V6 }
impl Ip { 
    fn name(&self) -> &'static str { 
        match self {
            Self::V4 => "v4",
            Self::V6 => "v6"
        }
    }
}

#[test] fn ex_2() {
    let v4 = Ip::V4;
    assert_eq!(v4.name(), "v4");
}


// 3. convert enum to T 
#[test] fn ex_cast_enum_to_int() {
    let one = E1::One;
    assert_eq!(one as u32, 1);
}

#[test] fn ex_match_enum_with_an_int() {
    let val = 2; // this value comes at runtime
    match val {
        val if val == E1::One as u8 => {
            assert_eq!(val as u8, 1);
        } ,
        val if val == E1::Two as u8 => {
            assert_eq!(val as u8, 2);
        } ,
        _ => todo!()
    }
}


// !! Duplicate: See trait_simple_vs_enum.rs as an interview task
//
// 4. impl enum also kind of inheritance
// src: https://www.lurklurk.org/effective-rust/use-types-2.html
enum Shape {
    Rect { w: f64, h: f64 },
    Circ { r: f64 },
}

impl Shape {
    // all types inherent this fn
    pub fn area(&self) -> f64 {
        match self {
            Self::Rect { w, h } => w * h,
            Self::Circ { r } => std::f64::consts::PI * r * r,
        }
    }
}

#[test] fn ex3_impl_enum_also_kind_of_inheritance() {
    let rec = Shape::Rect { w: 4.0, h: 3.0};
    assert_eq!(rec.area(), 12.0);

    let cir = Shape::Circ { r: 3.0};
    assert_eq!(cir.area(), 28.274333882308138);
}

