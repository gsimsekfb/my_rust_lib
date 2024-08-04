#[test]
fn ex1_first_last_n_elements() {
    let vec = vec![1, 2, 3, 4, 5, 6];

    let first_n = 2;
    assert_eq!([1, 2], &vec[..first_n]);       // first n element

    let last_n = 4;
    let len = vec.len();
    assert_eq!([3, 4, 5, 6], &vec[len-last_n..]);    // last n element
    assert_eq!([3, 4, 5, 6], &vec[len-last_n..len]); // last n element
}