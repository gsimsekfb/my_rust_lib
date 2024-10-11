// interv

// struct Num with int
// struct Str with string
// enum Val which wraps Num and Str
// fn variadic which takes a slice of Vals
// use variadic w/ diff length slices

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

use std::fmt::Debug;

struct Number { x: i32 }
struct Text { y: String }

enum Color {
    Num(Number), 
    Txt(Text)
}

// not variadic, array elems have to be the same type
fn not_variadic_generic(args: &[impl Debug]) {
    for arg in args { println!("arg: {:?}", arg); }
}

// looks like same types in array but under the Enum actually different types
fn variadic(args: &[Color]) {
    if let Some(Color::Num(n)) = &args.first() {
        println!("n.x: {:?}", n.x);
    };
    if let Some(Color::Txt(t)) = &args.get(1) {
        println!("t.y: {:?}", t.y);
    };
}

#[test]
fn ex1() {
    variadic(&[ 
        Color::Num( Number{ x: 42 } ),
        Color::Txt( Text{ y:"green".to_string() } ) 
    ]);

    variadic(&[ 
        Color::Num( Number{ x: 42 } ),
    ]);    

    not_variadic_generic(&[1, 2, 3]);
    not_variadic_generic(&['a', 'b']);

    assert_eq!(32, 32);
}
