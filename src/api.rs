
// Todos
// -

// Appendix:
// 1. fn that accepts array, slice, vec
// 2. fn that accepts &str, String and &String

// -------------------------------------------------------------------


// 1. fn that accepts array, slice, vec
fn find_strs(strs: &[String], find_str: &str) -> Vec<String> {
    let mut res = vec![];
    strs.iter().for_each(|s|
        if s.contains(find_str) { res.push(s.clone()) } 
    );
    res
}
//
// Wont work
// fn find_strs(strs: &Vec<String>, find_str: &str) -> Vec<String> {
//     let mut res = vec![];
//     strs.iter().for_each(|s|
//         if s.contains(find_str) { res.push(s.clone()) } 
//     );
//     res
// }

#[test]
fn ex1_accept_arr_slice_vec() {
    let vec = vec![
        "abc".to_string(), "ccc".to_string(), "acc".to_string()
    ];
    let arr = ["abc".to_string(), "ccc".to_string(), "acc".to_string()];
    let result: Vec<_> = vec!["ccc".to_string(), "acc".to_string()];

    // array, slice and vec
    assert_eq!(find_strs(&arr, "cc"), result);
    assert_eq!(find_strs(&arr[..], "cc"), result);
    assert_eq!(find_strs(&vec, "cc"), result);
}


// -------------------------------------------------------------------


// 2. fn that accepts &str, String and &String
fn add_suffix(s: impl AsRef<str>, suffix: &str) -> String {
    s.as_ref().to_string() + suffix
}

#[test]
fn ex2_accept_str_string_like() {
    let s1 = "a".to_string();
    let s2 = "a";
    let suffix = "b";
    let res = "ab";

    // accepts &String, String and &str
    assert_eq!(add_suffix(&s1, suffix), res);
    assert_eq!(add_suffix(s1, suffix), res);
    assert_eq!(add_suffix(s2, suffix), res);
}


// -------------------------------------------------------------------

