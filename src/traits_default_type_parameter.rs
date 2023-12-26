use std::ops::Add;

// Original Add trait from std
pub trait Add_<Rhs = Self> { // <Rhs = Self>: default type parameter
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// 1.a. Using "default type parameter"
// Implementing the Add trait to overload the + operator for Point instances
#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test] fn ex1_a() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// 1.b. Overriding "default type parameter" also newtype pattern
#[derive(Debug, PartialEq)] struct Millimeters(u32);
#[derive(Debug, PartialEq)] struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[test] fn ex1_b() {
  assert_eq!(
      Millimeters(3) + Meters(1),
      Millimeters(1003)
  );
}