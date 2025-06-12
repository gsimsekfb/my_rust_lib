
// interv-1

// 
// 1. fn(list of i32)
//  - accepts arr, slice and vec of i32s
//     
// 2. fn(1: single string like types - &str, String, &String 2: suffix)
//  - returns input + suffix
//  - use it w/ all string like inputs





// -------------------------------------------------------------------






// 1. fn that accepts array, slice, vec of Strings
fn foo(nums: &[i32]) {
    println!("{nums:?}");
}
//
// Wont work w/ slice and arr
// fn foo(nums: Vec<i32>) {
//     println!("{nums:?}");
// }
//
// error
// ^^^^^ expected `&Vec<i32>`, found `&[i32]`           // err foo(slice)
// ^^^^ expected `&Vec<i32>`, found `&[{integer}; 1]`   // err foo(array)


#[test]
fn ex1_accept_arr_slice_vec() {
    let arr = [2];
    let res = foo(&arr);
    println!("\n## res: {res:?}");

    let v = vec![1,2,3,2];
    let res = foo(&v);
    println!("\n## res: {res:?}");

    let slice = &v[0..1];
    let res = foo(slice);
    println!("\n## res: {res:?}");
}


// -------------------------------------------------------------------


// 2. fn that accepts &str, String and &String
fn add_suffix(s: impl AsRef<str>, suffix: &str) -> String {
    s.as_ref().to_string() + suffix
}
fn add_suffix_2(s: impl ToString, suffix: &str) -> String {
    s.to_string() + suffix
}
// todo: 
// also use ToString, Into traits

#[test]
fn ex2_accept_str_string_like() {
    let s1 = "a".to_string();
    let s2 = "a";
    //
    let suffix = "b";
    let res = "ab";

    // accepts &String, String and &str (slice)
    assert_eq!(add_suffix(&s1, suffix), res);
    assert_eq!(add_suffix(s1, suffix), res);
    assert_eq!(add_suffix(s2, suffix), res);

    // -------- ToString
    let s1 = "a".to_string();
    let s2 = "a";
    //
    let suffix = "b";
    let res = "ab";

    // accepts &String, String and &str (slice)
    assert_eq!(add_suffix_2(&s1, suffix), res);
    assert_eq!(add_suffix_2(s1, suffix), res);
    assert_eq!(add_suffix_2(s2, suffix), res);    
}


// -------------------------------------------------------------------


