//// All uses of dyn

//// Tasks
// 1.a
// trait MyTrait with fn name returns &str, def impl "MyTrait"
// struct Foo empty
// impl MyTrait for Foo, name returns "Foo"
// same things for struct Wii
// fn get_wii_or_foo_box takes bool, returns MyTrait, if true a Wii, else a Foo
//
// 1.b
// Test:
// Polymorphic vector of Foo and Wii, test name() values
// Polymorphic vector of refs to Foo and Wii, test name() values
//
// 2
// fn bar_dyn which takes polymorphic MyTrait items, calls name fn from item
// fn bar_impl which takes generic MyTrait items, calls name fn from item
// Test: Just Call these fns without asserts

// Also see
// polymorphism-generics-static-dispatch-vs-traits-dynamic-dispatch.jpg


// --------------------------------------------------------------------------


//// dyn can have 2 forms: Box<dyn Trait>, &dyn Trait or &mut dyn Trait 
// Box<dyn Trait> is owned trait obj vs others are refs hence they start w/ "&"

//// 1.a Box<dyn Trait> as return type

trait MyTrait { fn name(&self) -> &'static str { "MyTrait" } }

struct Foo {}
impl MyTrait for Foo { fn name(&self) -> &'static str { "Foo" } }

struct Wii {}
impl MyTrait for Wii { fn name(&self) -> &'static str { "Wii" } }

fn get_wii_or_foo_box(a: bool) -> Box<dyn MyTrait> {
    if a { Box::new( Wii { } ) }
    else { Box::new( Foo { } ) }
}
// Not possible with return impl, we can return only one type, see error:
//
// error[E0308]: `if` and `else` have incompatible types
// expected `Wii`, found `Foo`
// fn get_wii_or_foo(a: bool) -> impl MyTrait {
//     if a { Wii { } }
//     else { Foo { } }
// }

#[test] fn ex_1_a() {
    assert_eq!( get_wii_or_foo_box(true).name(), "Wii" );
    assert_eq!( get_wii_or_foo_box(false).name(), "Foo" );
}


//// 1.b Box<dyn Trait> vs &dyn Trait
//// storing polymorphic objects of different types

#[test] fn ex_1_b() {

    // i. Box<dyn Trait>
    let vec: Vec<Box<dyn MyTrait>> = vec![ Box::new(Foo {}), Box::new(Wii {}) ];
    // Error w/o Box 
    // error[E0277]: the size for values of type `dyn MyTrait` cannot be known
    // at compilation time
    // let vec: Vec<dyn MyTrait> = vec![ Foo {}, Wii {} ];
        // Vectors can only contain elements of a fixed size. 
        // Box struct is one such option, a smart pointer with a fixed size
    assert_eq!(vec[0].name(), "Foo");
    assert_eq!(vec[1].name(), "Wii");        

    // ii. &dyn Trait
    let foo = Foo {};
    let wii = Wii {};
    let vec: Vec<&dyn MyTrait> = vec![&foo, &wii];
        // diff: this vec does not own the objects, only refs
    assert_eq!(vec[0].name(), "Foo");
    assert_eq!(vec[1].name(), "Wii");
}



// -----------------------------------------------------------------------



//// 2. &dyn Trait (or &mut dyn Trait)
//// aka &dyn as fn param
//// also fn(impl Trait) vs fn(&dyn Trait) 

// &dyn  - runtime/dynamic polymorphism
// &impl - compile time generics
// In general same behavior - with both fns, x can access only MyTrait part of the type
fn bar_dyn (x: &dyn  MyTrait) {  x.name(); } // resolves trait methods at runtime
fn bar_impl(x: &impl MyTrait) {  x.name(); }
    // The impl version compiles to a new function for every unique 
    // function you pass in as a parameter. 
    //
    // The dyn version compiles a single function that is supplied a pointer 
    // to the function at runtime and calls it through the pointer. 
    // This makes the function harder to optimize and is generally less performant.
    //
    // Generally prefer compile-time generics if possible (&impl),
    // unless you care about the executable size.

#[test] fn ex_2() {
    let foo = Foo {};
    bar_dyn(&foo);
    bar_impl(&foo);
}
