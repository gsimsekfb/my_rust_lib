// interv

#[test]
fn ex1() {
    // a. copy
    let x = 5;
    let y = x; // !! copied, not moved
    println!("x: {}!", x);
        // Ok: Since because integers are simple values with a known, 
        // fixed size, and these two 5 values are pushed onto the stack

    // b. move
    let s1 = String::from("hello");
    let s2 = s1; // !! s1 moved to s2, s1 is invalid at this point
    // println!("s1: {}!", s1);
        // error[E0382]: borrow of moved value: `s1`
        //   --> src/main.rs:15:25
        //   |
        // 12 |     let s1 = String::from("hello");
        //   |         -- move occurs because `s1` has type `String`, 
        // which does not implement the `Copy` trait
        // 13 |     let s2 = s1;
        //   |              -- value moved here
        // 14 |
        // 15 |     println!("s1: {}!", s1);
        //   |                         ^^ value borrowed here after move
        //   |
        // help: consider cloning the value if the performance cost is acceptable
        //   |
        // 13 |     let s2 = s1.clone();
        //   |                ++++++++
    
    assert_eq!(32, 32);
}