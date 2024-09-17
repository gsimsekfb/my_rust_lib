

// 1. Simple example
#[derive(Copy, Clone)]
enum Foo {
    One = 1,
    Two = 2
}

#[test] fn ex1_enum_to_int() {
    let one = Foo::One;
    assert_eq!(one as u32, 1);
}

#[test] fn ex2_using_enum_with_match() {
    let val = 2; // this value comes at runtime
    match val {
        val if val == Foo::One as u8 => {
            assert_eq!(val as u8, 1);
        } ,
        val if val == Foo::Two as u8 => {
            assert_eq!(val as u8, 2);
        } ,
        _ => todo!()
    }         
}

// 2. impl enum also kind of inheritance
// src: https://www.lurklurk.org/effective-rust/use-types-2.html
enum Shape {
    Rect { w: f64, h: f64 },
    Circ { r: f64 },
}

impl Shape {
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

