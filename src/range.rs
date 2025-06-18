
// interv-1

// 1. get first n and last n elements of an array
// 2. match int with a range



// =================================================================



//// 
#[test]
fn ex1_first_and_last_n_elements() {
    let arr = [1, 2, 3, 4, 5, 6];

    // first n elem
    let first_n = 2;
    assert_eq!([1, 2], &arr[..first_n]);       // first n element
    // same as
    assert_eq!([1, 2], &arr[0..first_n]);      // first n element

    // last n elem
    let last_n = 4;
    let len = arr.len();
    assert_eq!([3, 4, 5, 6], &arr[len-last_n..]);    // last n element
    // same as
    assert_eq!([3, 4, 5, 6], &arr[len-last_n..len]); // last n element
}


//// 2. w/ match
#[test] fn ex_2() {
    let n = 20;
    let str = match n {
        1..=20 => "<=20",
        _ => "20>"
    };
    assert_eq!(str, "<=20");
}