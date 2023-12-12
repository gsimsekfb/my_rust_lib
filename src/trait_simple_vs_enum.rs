
// 1. Simple trait example
trait TShape {
    fn area(&self) -> f64;
}

struct Rectangle { height: f64, width: f64 }

impl TShape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle { radius: f64 }

impl TShape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[test] 
fn ex1_simple_trait_example() {
    let rec = Rectangle { width: 4.0, height: 3.0};
    assert_eq!(rec.area(), 12.0);

    let cir = Circle { radius: 3.0};
    assert_eq!(cir.area(), 28.274333882308138);
}

// 2. impl enum also kind of inheritance
// src: https://www.lurklurk.org/effective-rust/use-types-2.html
enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        }
    }
}

#[test] 
fn ex2_impl_enum_also_kind_of_inheritance() {
    let rec = Shape::Rectangle { width: 4.0, height: 3.0};
    assert_eq!(rec.area(), 12.0);

    let cir = Shape::Circle { radius: 3.0};
    assert_eq!(cir.area(), 28.274333882308138);
}