
#[derive(Copy, Clone)]
enum Foo {
    One = 1,
    Two = 2
}

#[test] fn ex1_enum_to_int() {
    let one = Foo::One;
    assert_eq!(one as u32, 1);
}

#[test] fn ex2_using_enum_with_match() {
    let val = 2; // this value comes at runtime
    match val {
        val if val == Foo::One as u8 => {
            assert_eq!(val as u8, 1);
        } ,
        val if val == Foo::Two as u8 => {
            assert_eq!(val as u8, 2);
        } ,
        _ => todo!()
    }         
}