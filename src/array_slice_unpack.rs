struct Foo { x: i32 }

fn modify_foo_array(arr: &mut [&mut Foo]) {
          // or    (arr: &mut [&mut Foo; 2]) 
    // matches/unpacks only with size 2 arrays, otherwise goes to else
    let [f1, f2] = arr else { panic!("Err: arr doesn't have two elems") };
    // or    array is fixed sized 
    // let [f1, f2] = arr;
    f1.x = 55;
}

#[test]
fn ex1_unpack_modify_array() {
    let mut f1 = Foo { x: 11 };
    let mut f2 = Foo { x: 33 };
    modify_foo_array(&mut [&mut f1, &mut f2]);
    assert_eq!(f1.x, 55);
}

#[test] #[should_panic]
fn ex2_unpack_panic_runtime() {
    let mut arr = [ &mut Foo { x: 11 }];    // arr: [&mut Foo; 1]
    modify_foo_array(&mut arr);   // panic !
    // or
    let mut f1 = Foo { x: 11 };
    let mut f2 = Foo { x: 33 };
    let mut f3 = Foo { x: 33 };
    modify_foo_array(&mut [&mut f1, &mut f2, &mut f3]);   // panic !
}
