
#[test]
fn ex1() {
    let mut cnt: i32 = 0;
    'counting_up: loop {
        println!("cnt = {cnt}");
        let mut cnt_dwn = 10;

        loop {
            println!("cnt_dwn = {cnt_dwn}");
            if cnt_dwn == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            cnt_dwn -= 1;
        }

        cnt += 1;
    }
    println!("End cnt = {cnt}");

    assert_eq!(cnt, 2);
}

// cnt = 0
// cnt_dwn = 10
// cnt_dwn = 9
// cnt = 1
// cnt_dwn = 10
// cnt_dwn = 9
// cnt = 2
// cnt_dwn = 10
// End cnt = 2
