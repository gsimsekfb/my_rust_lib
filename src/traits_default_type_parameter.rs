// interv-2

//// Use and override "default type parameter" for generic trait std::ops::Add
//// also Operator Overloading

// 1.a. Using "default type parameter"
// Impl std::ops::Add trait to overload the + operator for Point instances
//
// Point struct with x,y i32
// Impl std Add for Point
// Test: P1 + P2 = P3



// =============================================================================



#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }

// FYI: Add trait from std
pub trait std_Add_<Rhs = Self> { // <Rhs = Self>: default type parameter
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs is Self, so we impl Add<Point> here
impl std::ops::Add for Point { // 222 Add is gen trait but no need to specify type
    type Output = Self;
    fn add(self, rhs: Self) -> Self { 
        Self { x: self.x + rhs.x, y: self.y + rhs.y, }
    }
}

#[test] fn ex1_a() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}




// 1.b. Overriding "default type parameter" also newtype pattern
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
//
// Tuple structs Millimeters and Meters with u32
// Impl std Add for Millimeters 
// Test: mm + m = mm





// =============================================================================






#[derive(Debug, PartialEq)] struct Millimeters(u32);
#[derive(Debug, PartialEq)] struct Meters(u32);

// For info: Add trait from std
trait std_Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// 222 Overriding default type parameter, Meters instead of Self/Millimeters
impl std::ops::Add<Meters> for Millimeters {
    type Output = Self;
    fn add(self, rhs: Meters) -> Self {
        Self(self.0 + (rhs.0 * 1000))
    }
}

#[test] fn ex1_b() {
  assert_eq!( Millimeters(3) + Meters(1), Millimeters(1003) );
}