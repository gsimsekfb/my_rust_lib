

// interv-1
 
// 1. Explain <T: 'static> vs &'static T
// - todo: use it 



// 1. T: 'static (!! tricky) vs &'static T
//
// - T: 'static
// type T contains only static data or owned data (no non-static references)
//
// Think of <T: 'static> as T contains no non-static references",
// rather than "T must live forever".
//
// A common misconception is thinking that T: 'static means T is static, must be
// valid for the entire program, but it actually means the type must not
// contain any references with lifetimes shorter than 'static.

// - &'static T
// the reference itself must live for the static lifetime


struct Foo_Static {
    x_ref: &'static str,
}

// todo
struct Foo_A<'a> {
    x_ref: &'a str
}

fn bar<T:'static>(x: &T) { }

fn bar_a<'a, T:'a>(x: &T) { }


#[test]
fn ex_1() {
    // a
    let f = Foo_Static { x_ref: "ab" };    // Ok: "ab", itself is static
    bar(&f);

    // b
    let ss = "aa".to_string();
    // let f = Foo { x_ref: &ss }; // Err
                       //    |
                       // borrowed value does not live long enough                          
                       // this usage requires that `ss` is borrowed for `'static`
    bar(&f);


    // c
    bar(&42);
    bar(&"aa".to_string());


    // todo
    // let f = Foo_A { x_ref: "ab" };
    // let res = bar(&f);

    // let ss = "aa".to_string();
    // let f = Foo_A { x_ref: &ss };
    // let res = bar_a(&f);
}