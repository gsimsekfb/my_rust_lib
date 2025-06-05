// interv-1

// Supertraits: Require a trait in another trait

// struct Point x,y i32
// trait MyDisplay which requires Display trait, with fn my_print returns
// a string ("MyDisplay") as def. impl.
// Impl MyDisplay for Point - use ToString::to_string or our custom impl
// Hint: ToString trait is automatically impl'ed for any type impls Display trait
// Impl Display for Point
// Test:
// With a Point obj, use Display, ToString and MyDisplay traits





// ==============================================================================





struct Point { x: i32, y: i32 }

// Super trait: Any type implements this Trait must implement Display first
trait MyDisplay: std::fmt::Display {
    fn my_print(&self) -> String { "MyDisplay".to_string() }
}

impl MyDisplay for Point {
    fn my_print(&self) -> String {
        // ToString trait is automatically impl'ed for any type impls Display trait
        self.to_string() // which actually uses Display::fmt impl for Point
        // or using our impl
        // self.x.to_string() + ":" + &self.y.to_string()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[test] fn ex1() {
    let p = Point { x: 1, y: 2 };
    assert_eq!( format!("{}", p), "(1,2)" ); // using std::fmt::Display impl
        // Using format!() without impl Display is error:
        // error[E0277]: `Point` doesn't implement `std::fmt::Display`
    assert_eq!(p.to_string(), "(1,2)");
    assert_eq!(p.my_print(), "(1,2)");
}
