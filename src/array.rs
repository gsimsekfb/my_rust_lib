// interv

// use array fn to get array elem at index i, checked


#[test]
fn ex1() {
    let arr = [1,2,3];

    let mut res = 0;
    if let Some(val) = arr.get(4) {
        res = *val;
    }
    assert_eq!(res, 0);

    if let Some(val) = arr.get(0) {
        res = *val;
    }    
    assert_eq!(res, 1);
}
