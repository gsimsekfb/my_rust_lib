
// 1. Simple trait example
trait TShape {
    fn area(&self) -> f64;
}

struct Rect { h: f64, w: f64 }

impl TShape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

struct Circ { r: f64 }

impl TShape for Circ {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

#[test] 
fn ex1_simple_trait_example() {
    let rec = Rect { w: 4.0, h: 3.0};
    assert_eq!(rec.area(), 12.0);

    let cir = Circ { r: 3.0};
    assert_eq!(cir.area(), 28.274333882308138);
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

#[test] 
fn ex2_impl_enum_also_kind_of_inheritance() {
    let rec = Shape::Rect { w: 4.0, h: 3.0};
    assert_eq!(rec.area(), 12.0);

    let cir = Shape::Circ { r: 3.0};
    assert_eq!(cir.area(), 28.274333882308138);
}