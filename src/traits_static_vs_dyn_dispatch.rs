// interv-2

//// Static dispatch vs Dynamic dispatch
// 
// trait Shape, fn area returns f64
// struct Rect, h, w f64
// impl Shape for Rect
// struct Cir, radius f64
// impl Shape for Cir
// static dispatch:
// fn get_area, accepts an obj that impl Shape, returns area
// Test it
// dyn dispatch:
// fn sum_areas_dyn_a takes vector of Shapes, returns sum of areas
// fn sum_areas_static takes vector of Shapes, returns sum of areas
// fn sum_areas_dyn_b takes &vector of &Shapes, returns sum of areas
// fn sum_areas_dyn_b_sink takes vector of &Shapes, returns sum of areas
// fn sum_areas_dyn_b_generic takes &vector of &Shapes, returns sum of areas
// Test them, see usage diff

// Src:  
// https://www.mattkennedy.io/blog/rust_polymorphism/





// ---------------------------------------------------------------------




//// Static dispatch (generics) vs Dynamic dispatch (dyn)

// Performance Penalty of Trait Objects (aka Dynamic Dispatch):
    // When we use trait objects, Rust must use dynamic dispatch. The compiler
    // doesn’t know all the types that might be used with the code that is using trait
    // objects, so it doesn’t know which method implemented on which type to call.
    // Instead, at runtime, Rust uses the pointers inside the trait object to know
    // which method to call. There is a runtime cost when this lookup happens
    // that doesn’t occur with static dispatch. Dynamic dispatch also prevents the
    // compiler from choosing to inline a method’s code, which in turn prevents
    // some optimizations. However, we did get extra flexibility in the code


trait Shape { fn area(&self) -> f64; }

struct Rect { h: f64, w: f64 }
impl Shape for Rect {
    fn area(&self) -> f64 { self.w * self.h }
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

// 1. Static dispatch - generics
fn get_area<S: Shape>(shape: S) -> f64 {
    shape.area()
}

#[test] 
fn ex1_static_dispatch() {
    let rec = Rect { w: 4.0, h: 3.0};
    assert_eq!(get_area(rec), 12.0);
}



// 2. Dynamic dispatch

// a
// With trait objects multiple different shapes can 
// be contained in the vector.
fn sum_areas_dyn_a(shapes: Vec<Box<dyn Shape>>) -> f64 { // todo: remove Box?
    shapes.iter().map(|e| e.area()).sum()
    // or
    // shapes.iter().fold(0., |acc, shape| {
    //     acc + shape.area()
    // })
}
// instead of
// Static dispatch
// This only works if every element in the vector is the same Shape type
// A generic type param can only be substituted with ONE concrete type at a time
fn sum_areas_static<S: Shape>(shapes: Vec<S>) -> f64 {
    shapes.iter().map(|e| e.area()).sum()
}
// using generics and trait bounds - definitions will be monomorphized to 
// the concrete types at compile time
//
// more:
// when we use trait bounds on generics: 
// the compiler generates non-generic implementations
// of functions and methods for each concrete type that we use in place
// of a generic type parameter. The code that results from monomorphization
// is doing static dispatch, which is when the compiler knows what 
// method you’re calling at compile time.

// b 
// without Box, vector of refs
fn sum_areas_dyn_b_gen<S: Shape + ?Sized>(shapes: &[&S]) -> f64 {
    shapes.iter().map(|e| e.area()).sum()
}
fn sum_areas_dyn_b(shapes: &[&dyn Shape]) -> f64 { 
                            // clippy: &[&] instead of Vec<&S>
    shapes.iter().map(|e| e.area()).sum()
}
fn sum_areas_dyn_b_sink(shapes: Vec<&dyn Shape>) -> f64 { 
                            // cannot use &[&] instead of Vec<&S>
    shapes.iter().map(|e| e.area()).sum()
}

#[test] 
fn ex2_dyn_dispatch() {
    // a
    // dyn
    let rec: Box<dyn Shape> = Box::new(Rect { w: 4.0, h: 3.0});
    let cir: Box<dyn Shape> = Box::new(Circle { radius: 3.0});
    let vec = vec![rec, cir];
    assert_eq!(sum_areas_dyn_a(vec), 40.27433388230814);
    // static
    let rec = Rect { w: 4.0, h: 3.0 };
    let cir = Circle { radius: 3.0 };
    // sum_areas_static(vec![rec, cir]); // error
    let rec2 = Rect { w: 4.0, h: 3.0 };
    sum_areas_static(vec![rec, rec2]); // Ok

    // b
    // dyn
    let rec = Rect { w: 4.0, h: 3.0 };
    let cir = Circle { radius: 3.0 };
    let vec: Vec<&dyn Shape> = vec![&rec, &cir];
    assert_eq!(sum_areas_dyn_b(&vec), 40.27433388230814);
    assert_eq!(sum_areas_dyn_b_gen(&vec), 40.27433388230814);

    assert_eq!(sum_areas_dyn_b_sink(vec), 40.27433388230814);
}


// ----------------------------- Object Safety ---------------------------------

// You can only make object-safe traits into trait objects. Some complex rules
// govern all the properties that make a trait object safe, but in practice, only
// two rules are relevant. A trait is object safe if all the methods defined in the
// trait have the following properties:
//   • The return type isn't Self.
//   • There are no generic type parameters.
// An example of a trait whose methods are not object safe is the standard
// library’s Clone trait:
pub struct Screen2 {
    // pub components: Vec<Box<dyn Clone>>,
      // Err: Clone` cannot be made into an object
      // = note: the trait cannot be made into an object because it requires 
      // `Self: Sized`
      // = note: for a trait to be "object safe" it needs to allow building
      // a vtable to allow the call to be resolvable dynamically; for more 
      // information visit
      // <https://doc.rust-lang.org/reference/items/traits.html#object-safety>  
}

