// Using Supertraits to Require One Trait’s Functionality Within Another Trait
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) -> String {
        let input = self.to_string();
            // Because we’ve specified that OutlinePrint requires Display trait, 
            // we can use the to_string function which must be 
            // implemented for any type that implements Display
        let len = input.len();
        let stars = "*".repeat(len);
        let res = format!("{} {} {}", stars, input, stars);
        println!("{}", res);
        res
    }
}

struct Point { x: i32, y: i32 }

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

#[test] fn ex1() {
    let point = Point { x: 3, y: 5};
    println!("p1: {}", point); // p1: (3, 5)
    let str = point.outline_print(); // ****** (3, 5) ******
    assert_eq!(str, "****** (3, 5) ******");
}