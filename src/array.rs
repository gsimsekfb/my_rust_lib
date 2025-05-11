// interv-1

// use std::array fn to get array elem at index i, checked, using if let






#[test]
fn ex1() {
    let arr = [1,2,3];

    let mut res = 0;
    if let Some(val) = arr.get(4) {
        res = *val;
    }
    assert_eq!(res, 0);

    if let Some(val) = arr.get(2) {
        res = *val;
    }    
    assert_eq!(res, 3);
}
