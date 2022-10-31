pub trait Draw {
  fn draw(&self);
}

// 1.a. Struct that uses trait objects - aka DYNAMIC dispatch - 
// allows for MULTIPLE concrete types to fill in for the trait 
// object at RUNTIME
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
    // dyn: DYNAMIC dispatch to a trait object    
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
    // Performance Penalty of Trait Objects (aka Dynamic Dispatch):
    // When we use trait objects, Rust must use dynamic dispatch. The compiler
    // doesn’t know all the types that might be used with the code that is using trait
    // objects, so it doesn’t know which method implemented on which type to call.
    // Instead, at runtime, Rust uses the pointers inside the trait object to know
    // which method to call. There is a runtime cost when this lookup happens
    // that doesn’t occur with static dispatch. Dynamic dispatch also prevents the
    // compiler from choosing to inline a method’s code, which in turn prevents
    // some optimizations. However, we did get extra flexibility in the code

// 1.b. Static Dispatch
// --------------------------------------------------------------------
// For comparison: Static Dispatch
// Struct that uses a generic type parameter with trait bounds. 
// A generic type parameter can only be substituted with ONE 
// concrete type at a time,
pub struct Screen_Static_Dispatch<T: Draw> {
  pub components: Vec<T>, // aka STATIC/COMP. TIME dispatch
}
  // This restricts us to a Screen instance that has a list of components all
  // of type Button or all of type TextField. If you’ll only ever have homogeneous
  // collections, using generics and trait bounds is preferable because the 
  // definitions will be monomorphized at compile time to use the concrete types.

impl<T> Screen_Static_Dispatch<T> where T: Draw {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
    // when we use trait bounds on generics: 
    // the compiler generates nongeneric implementations
    // of functions and methods for each concrete type that we use in place
    // of a generic type parameter. The code that results from monomorphization
    // is doing static dispatch, which is when the compiler knows what 
    // method you’re calling at compile time.
// -------------------------------------------------------------------


// 1.a. DYNAMIC dispatch ...continues...
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) { println!("--- I am a Button"); }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) { println!("--- I am a SelectBox"); }
}

#[test]
fn ex1() {
  // Using trait objects to store values of different types that implement
  // the same trait
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![String::from("Yes"), String::from("No")],
      }),
      Box::new(Button { 
        width: 50, height: 10, label: String::from("OK") 
      }),
      // Box::new(String::from("Hi")), 
        // Err: the trait `Draw` is not implemented for `String`
    ],
  };
  
  screen.run();
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
