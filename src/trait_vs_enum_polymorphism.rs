// interv-2
// todo: read but skip doing ? 
// this topic almost never used or asked in interviews, also there is a 
// similar task as interv-1, no ? 

// Polymorphism using trait vs enum

// 1. Polymorphism with trait - New types and trait are decoupled
// Impl trait Shape with fn area -> i32
// Impl Shape for Square {x} and Rect {w,h} structs
// Test:
// Create vector of Shape traits - one square, one rect
// Print type of vec[0]

// 2. Polymorphism with enum - Types and enum are bound together
// Impl enum Shape_ with Square {x} and Rect {w,h} 
// Impl Shape_ fn area -> i32 
// Test:
// Create vector of Shape_ enums - one square, one rect
// Print type of vec[0]

// ----------------------------------------------------------------

// 1
trait Shape { fn area(&self) -> i32; }

struct Square { x: i32 }
impl Shape for Square {
    fn area(&self) -> i32 { self.x * self.x }
}

struct Rect { w: i32, h: i32 }
impl Shape for Rect {
    fn area(&self) -> i32 { self.w * self.h }
}

// ----------------------------------------------------------------

// 2
enum Shape_{
    Square { x: i32 },
    Rect { w: i32, h: i32},
}

impl Shape_ {
    fn area(&self) -> i32 {
        match self {
            Self::Square {x} => x * x,
            Self::Rect {w, h} => w * h,
        }
    }
}

#[test]
fn ex1() {
    // 1. trait version
    let s1: Box<dyn Shape> = Box::new(Square {x: 4});
    let traits = [
        s1, 
        Box::new(Rect {w: 5, h: 6}) as Box<dyn Shape> // s2
    ];
    crate::print_type_of("traits[0]", &traits[0]);
        // alloc::boxed::Box<dyn rust_book_minigrep::temp::Shape>
    println!("-- {}", traits[0].area());

    // 2. enum version
    let enums = [Shape_::Square { x: 8 }];
    crate::print_type_of("enums[0]", &enums[0]);
        // rust_book_minigrep::temp::Shape_
    println!("-- {}", enums[0].area());
    let xx = match enums[0] {
        Shape_::Square {x} => x,
        Shape_::Rect {w, ..} => w
    };

    assert_eq!(xx, 8);
}
// https://stackoverflow.com/questions/52240099/should-i-use-enums-or-boxed-trait-objects-to-emulate-polymorphism
// - One of the big differences between using traits and enums is their 
//   extensibility. If you make Shape an enum, then the two options are 
//   hardcoded into the type. If you want to add some third form of Shape, 
//   you'll have to modify the type itself, which will probably involve a lot 
//   of modifications to the code with uses Shape (e.g. anywhere you match on an 
//   Shape will probably need to be changed)
//
//   On the other hand, if you make Shape a trait, you can add other types of 
//   Shape by just defining a new type and writing an appropriate implementation, 
//   without modifying existing code at all. This could even be done from outside 
//   of the library, e.g. by a user.
//
// - The other important thing to consider is how much access you need to the 
//   internals of the structs. With an enum, you get full access to all the data 
//   stored within the struct. If you want to write a function which can operate 
//   on both Coordinate and Quaternion using a trait, then the only operations 
//   you will be able to perform are those described in the Shape trait 
//   (in this case Shift and Fold). For instance, giving the implementation of 
//   Shape you gave, there would be no way for you to simply retrieve the (X,Y,Z) 
//   tuple via the Shape interface. If you needed to do that at some point, you 
//   would have to add a new method.
//
// - enums can be stored directly on the stack, while a boxed trait will always 
//   require the heap. That is, enums are  cheap to create, but boxed traits are not.
//
// - an enum instance will always be as big as its biggest variant (plus a 
//   discriminant in most cases), even if you store mostly small variants. 
//   This would be a problem in a case like this:
//
//      enum Foo {
//          SmallVariant(bool),
//          BigVariant([u64; 100]),
//      }
//
//   If you were to store N instances of this type in an vector, the vector 
//   would always need N*(100*sizeof::<u64> + sizeOfDiscriminant) bytes of memory, 
//   even when the vector only contains SmallVariants.
//
//   If you were using a boxed trait, the vector would use N * sizeOfFatPointer 
//   == N * 2 * sizeof::<usize>
//   Just for completeness, if you box just the array inside BigVariant, 
//   the vector ends up the same size as the boxed trait version, but you 
//   still get the other advantages of enum aside from stack allocation