pub trait Draw {
  fn draw(&self);
}

// Struct that uses trait objects - allows for MULTIPLE
// concrete types to fill in for the trait object at RUNTIME
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

// --------------------------------------------------------------------
// For comparison:
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
// -------------------------------------------------------------------


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