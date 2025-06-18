//// All uses of dyn

// interv-1
// 1.a
// impl MyTrait with fn name returns &str, default impl. returns "MyTrait"
// impl struct Foo empty
// impl MyTrait for Foo, name returns "Foo"
// last step for struct Wii
// fn get_wii_or_foo takes bool, returns MyTrait, if true a Wii, else a Foo

// interv-1
// see 1.b below

// interv-2
// see 2 below


// Also see
// polymorphism-generics-static-dispatch-vs-traits-dynamic-dispatch.jpg





// --------------------------------------------------------------------------





//// dyn can have 2 forms: 1. Box<dyn Trait>, 2. &dyn Trait or &mut dyn Trait 
// Box<dyn Trait> is owned trait obj vs others are just refs

//// 1.a Box<dyn Trait> as return type

trait MyTrait { fn name(&self) -> &'static str { "MyTrait" } }

struct Foo {}
impl MyTrait for Foo { fn name(&self) -> &'static str { "Foo" } }

struct Wii {}
impl MyTrait for Wii { fn name(&self) -> &'static str { "Wii" } }

fn get_wii_or_foo(a: bool) -> Box<dyn MyTrait> {
    if a { Box::new( Wii { } ) }
    else { Box::new( Foo { } ) }
}
// Not possible with return impl, we can return only one type, see error:
//
// error[E0308]: `if` and `else` have incompatible types
// expected `Wii`, found `Foo`
// fn get_wii_or_foo_(a: bool) -> impl MyTrait {
//     if a { Wii { } }
//     else { Foo { } }
// }

#[test] fn ex_1_a() {
    assert_eq!( get_wii_or_foo(true).name(), "Wii" );
    assert_eq!( get_wii_or_foo(false).name(), "Foo" );
}


//
// 1.b
// Test:
// Polymorphic vector of Foo and Wii, test name() values
// Polymorphic vector of refs to Foo and Wii, test name() values
//



//// Box<dyn Trait> vs &dyn Trait
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





// 2
// fn bar accepts refs to polymorphic MyTrait items, calls name() and put 
// names into vec and returns
// copy code from task 1
// Test: call these fns with array and/or vec of Foo objs




// -----------------------------------------------------------------------




// task-2

fn bar(arr: &[&dyn MyTrait]) -> Vec<String> {
    arr.iter().map(|e| e.name().to_string()).collect()
}

#[test]
fn task_2() {
    let f = Foo {};
    let w = Wii {};
    let arr: Vec<&dyn MyTrait> = vec![&f, &w];
    // or
    let arr: Vec<_> = vec![&f as &dyn MyTrait, &w];
    // or
    let arr: [&dyn MyTrait; 2] = [&f, &w];
    assert_eq!(bar(&arr), &["Foo", "Wii"]);
}



// read

//// &dyn Trait (or &mut dyn Trait)
//// aka &dyn as fn param
//// also fn(impl Trait) vs fn(&dyn Trait) 

// &dyn  - runtime/dynamic polymorphism
// &impl - compile time generics
// In general same behavior - with both fns, x can access only MyTrait part of the type
fn bar_dyn (x: &dyn  MyTrait) { x.name(); } // resolves trait methods at runtime
fn bar_impl(x: &impl MyTrait) { x.name(); }
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
