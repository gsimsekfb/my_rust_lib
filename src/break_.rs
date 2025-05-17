// interv-1
// test - increment an outside defined cntr inside a loop
// assign the loop into a var `res`
// when the cntr is 10, finish loop and return cntr  
// assert res, 10






#[test]
fn ex1_break_as_return() {
    let mut cntr = 0;

    let res = loop {
        cntr += 1;

        if cntr == 10 {
            break cntr;
        }
    };

    assert_eq!(res, 10);
}