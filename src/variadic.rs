// interv-2

// struct Num with int
// struct Text with str
// impl variadic fn accepts a collection of any type
// 1 using static dispatch
// 2 using dynamic dispatch
// test them
// Hints at the end



// ==================================================



/* 
//// C++ variadic template sum
#include <iostream>

// C++17
template<typename... Args>
auto sum(Args... args)
{
    return (args + ...);
}

int main() {
    std::cout << sum(1, 4.5) << std::endl;;	  // 5.5 !!  // diff. types
    std::cout << sum(1, 4, 6) << std::endl;;

    return 0;
} 
*/


struct Number { n: i32 }
struct Text { t: String }

enum Color {
    Num(Number), 
    Txt(Text)
}

use std::fmt::Debug;

// not variadic, array elems have to be the same type
fn not_variadic_generic(args: &[impl Debug]) {
    for arg in args { println!("arg: {arg:?}"); }
}

// 1.
// same Enum types but under the Enum actually different types
fn variadic_static(args: &[Color]) {
    if let Some(Color::Num(n)) = &args.first() {
        println!("n.x: {:?}", n.n);
    };
    if let Some(Color::Txt(t)) = &args.get(1) {
        println!("t.y: {:?}", t.t);
    };
}




// 2
// a
trait MyTrait {
    fn inner_text(&self) -> &str { "not_impl" }
        // or we could disable def. impl and return Option<> 
}
impl MyTrait for Number { }
impl MyTrait for Text { fn inner_text(&self) -> &str { &self.t }}

fn variadic_dyn(args: &[&dyn MyTrait]) {
    for arg in args {
        println!("-- {}", arg.inner_text());
    }
}


// b
trait MyTrait_2 : std::any::Any { /* fn as_any(&self) -> &dyn Any; */ }
impl MyTrait_2 for Number {}
impl MyTrait_2 for Text {}

use std::any::Any;

fn variadic_dyn_w_any(args: &[&dyn MyTrait_2]) {
    for arg in args {
        if let Some(num) = (*arg as &dyn Any).downcast_ref::<Number>() {
            println!("-- It's a Number: {}", num.n);
        }
    }
}

#[test]
fn ex1() {
    variadic_static(&[ 
        Color::Num( Number{ n: 42 } ),
        Color::Txt( Text{ t:"green".to_string() } ) 
    ]);

    variadic_static(&[ 
        Color::Num( Number{ n: 42 } ),
    ]);    

    not_variadic_generic(&[1, 2, 3]);
    not_variadic_generic(&['a', 'b']);

    let text = Text { t: "aa".to_string() };
    let text = &text as &dyn MyTrait;
    let num = Number { n: 42 };
    let num = &num as &dyn MyTrait;
    variadic_dyn( &[text, num] );

    let text = Text { t: "aa".to_string() };
    let text = &text as &dyn MyTrait_2;
    let num = Number { n: 42 };
    let num = &num as &dyn MyTrait_2;
    variadic_dyn_w_any( &[text, num] );

    assert_eq!(32, 32);
}


// Hint
// enum Val which wraps Num and Str
// fn variadic which takes a slice of Vals
// use variadic w/ diff length slices
