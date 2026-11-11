// interv-2

//// Trait with an associated type vs Generic trait
//// https://doc.rust-lang.org/rust-by-example/generics/assoc_items.html

// Task
//
// - impl (1) MY_TRAIT w/ assoc. type and (2) generic MY_GEN_TRAIT 
//   with multiple types for a simple struct with member x
// - what is the diff?
// - use/call trait fns




// ========================================================================




struct Foo { x: String } 

// 1
trait MY_TRAIT {
    type Item;
    fn foo() -> String { "default_impl".to_string() }
    fn x(&self) -> Self::Item;
}

impl MY_TRAIT for Foo {
    type Item = String;
    fn x(&self) -> Self::Item { self.x.clone() }    	
}

// !! it can NOT be implemented multiple times for a type 
//
// error[E0119]: conflicting implementations of trait `MY_TRAIT` for type `Foo`
// impl MY_TRAIT for Foo {
//     type Item = i32;
//     fn x(&self) -> Self::Item { 32 }
// }


// 2
trait MY_GEN_TRAIT<T> {
    fn x_gen(&self) -> T;
}

// !! it can be implemented multiple times for a type

impl MY_GEN_TRAIT<String> for Foo {
    fn x_gen(&self) -> String { self.x.clone() }
}

impl MY_GEN_TRAIT<i32> for Foo {
    fn x_gen(&self) -> i32 { self.x.parse::<i32>().unwrap() + 5 }
}

// ===============================================================

#[test] 
fn usage() {
    // 1. MY_TRAIT case
    let foo = Foo { x: "10".to_string() };
    assert_eq!(foo.x(), "10");

    // 2. MY_GEN_TRAIT case
    // a) compiles if there is ONE impl. for MY_GEN_TRAIT for Foo
    // assert_eq!(foo.x_gen(), "10"); 
    
    // b) if there are multi. impls
    assert_eq!(MY_GEN_TRAIT::<String>::x_gen(&foo), "10"); 
    assert_eq!(MY_GEN_TRAIT::<i32>::x_gen(&foo), 15); 

}