

// Official doc:
// mut can be used in several situations:
// A mutable variable, reference, or pointer

// 1. mut var
// mutable variable in the parameter list of a function.
fn add_suffix(mut ss: String) -> String {
    ss.push_str(" world");
    ss
}
#[test]
fn ex1_mut_var() {
    let s = String::from("hello");
    let s2 = add_suffix(s);
    // println!("{}", s2);
    assert_eq!(s2, "hello world");
}


// 2. mut ref
fn push_vec(v: &mut Vec<i32>) {
    v.push(42);
}

#[test]
fn ex2_mut_ref() {
    let mut vv = vec![1,2];
    push_vec(&mut vv);
    assert_eq!(vv, vec![1, 2, 42]);
}


// 3. Mut raw pointers 
// They work much like mutable references, with the added possibility of not 
// pointing to a valid object. The syntax is *mut Type.
