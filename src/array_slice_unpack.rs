// interv-1

// 1
// - Foo with an int member
// - fn modify_foo_array ( arr of two (or unspecified # of) Foos )
// - Unpack arr and change the first Foo's val, panic if arr does not have 2 elems
// - Use modify_foo_array with diff sizes of arrays

// 2
// see below




//// ---------------------------------------------------------------------






struct Foo { x: i32 }

fn modify_foo_array(arr: &mut [Foo]) {
          // or    (arr: &mut [&mut Foo; 2])
          // or    <const N: usize>(arr: &mut [Foo;N])

    // matches/unpacks only with size 2 arrays, otherwise goes to else
    // key: let else
    let [f1, _] = arr else { panic!("Err: arr doesn't have two elems") };
    // or if array is fixed sized:
    // let [f1, f2] = arr;
    f1.x = 55;
}

#[test]
fn ex1_a_unpack_modify_array() {
    let mut arr = [Foo { x: 11 }, Foo { x: 22 }];
    modify_foo_array(&mut arr);
    assert_eq!(arr[0].x, 55);
}

#[test] #[should_panic]
fn ex1_b_unpack_panic_runtime() {
    let mut arr = [Foo { x: 11 }];    // arr: [&mut Foo; 1]
    modify_foo_array(&mut arr);   // panic !
    // or
    let mut arr = [Foo { x: 11 }, Foo { x: 22 }, Foo { x: 33 }];

    modify_foo_array(&mut arr);   // panic !
}





// 2. more unpack
// - clone same fn, remove panic
// - unpack change 3rd elem of size 6 arr
// - unpack change first/last elem




// -----------------------------------------------------------------



fn modify_foo_array_2(arr: &mut [Foo]) {
    let [_, _, third, _, _, _] = arr else { panic!("Err: arr doesn't have enough elems") };
    third.x = 55;

    let [first, ..] = arr else { panic!("Err: arr doesn't have enough elems") };
    first.x = 55;
    let [last, ..] = arr else { panic!("Err: arr doesn't have enough elems") };
    last.x = 55;

    let [first, .., last] = arr else { panic!("Err: arr doesn't have enough elems") };
    first.x = 55;
}

#[test]
fn ex2() {
    let mut arr = [
        Foo { x: 11 }, Foo { x: 22 }, Foo { x: 33 },
        Foo { x: 44 }, Foo { x: 55 }, Foo { x: 66 }        
    ];
    modify_foo_array_2(&mut arr);
    assert_eq!(arr[2].x, 55);
}



