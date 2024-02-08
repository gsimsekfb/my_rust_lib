
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

fn variadic_generic(args: &[impl Debug]) {
    for arg in args { println!("arg: {:?}", arg); }
}

fn variadic(args: &[Color]) {
    if let Color::Num(n) = &args[0] {
        println!("n.x: {:?}", n.x);
    };
    if let Color::Txt(t) = &args[1] {
        println!("t.y: {:?}", t.y);
    };
}

#[test]
fn ex1() {
    variadic(&[ 
        Color::Num( Number{ x: 42 } ),
        Color::Txt( Text{ y:"green".to_string() } ) 
    ]);

    variadic_generic(&[1, 2, 3]);
    variadic_generic(&['a', 'b']);

    assert_eq!(32, 32);
}
