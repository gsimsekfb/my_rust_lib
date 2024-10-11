// interv
// test - assign a "loop" into res, increment cntr inside it,
// when the cntr is 10, finish loop and return cntr  



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