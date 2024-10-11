
// interv
// - create an alias for fn ptr which takes int and returns int




// 1. Function pointer example

//       syntax: a nameless fn
//                |
//                V
type fptr = fn(i32) -> i32;

fn add_one(x: i32) -> i32 { x + 1 }

fn do_twice(f: fptr, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn ex_1() {
    let res = do_twice(add_one, 5);
    assert_eq!(res, 12);
}

// 2. Function ptr vs closure

#[test]
fn ex_2() {
    // 1. using closure
    let nums = vec![1, 2, 3];
    let strs: Vec<String> = nums.iter().map(|i| i.to_string()).collect();
    assert_eq!(strs, ["1", "2", "3"]);

    // 2. using fn ptr 
    // (this is also a task in iter*algo*.rs)
    let nums = vec![1, 2, 3];
    let strs: Vec<String> = nums.iter().map(ToString::to_string).collect();
    assert_eq!(strs, ["1", "2", "3"]);
}

// 3. map called by initializer function

#[derive(Debug, PartialEq)]
enum Status {
    Value(u32),
    Stop,
}
// Without PartialEq:
// error[E0369]: binary operation `==` cannot be applied to type `Vec<Status>`

// (this is also a task in iter*algo*.rs)
#[test]
fn ex_3() {
    let res: Vec<Status> = (1u32..3)
        .map(Status::Value) // initializer function
        .collect();
    assert_eq!(res, [Status::Value(1), Status::Value(2)]);
}
