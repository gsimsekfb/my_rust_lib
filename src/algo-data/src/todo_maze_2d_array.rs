#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::four_forward_slashes)]
#![allow(clippy::empty_line_after_doc_comments)]


// ------------------------- main

use std::vec;

#[derive(Debug, PartialEq)]
enum Dir { L, R, U, D}
 
fn find_exit_route<const N: usize>(
    arr: &[[u8;N]],
    start_pos: (usize,usize)
) -> Vec<Dir>{
    println!("---------------------------- fn()"); // debug
    let mut res = vec![];
    // for e in arr { println!("{e:?}"); }
    find_exit_route_impl(arr, start_pos, start_pos, &mut false, &mut res);
    res
}

// 5x5 array
//
// 1 0 1 1 0
// 0 1 1 1 0 
// 0 1 X 1 0
// 1 0 1 1 0
// 0 0 0 0 0
//
/*
f 3,3
    if exit_found, return
    L:  if last_dir != R
        if L is None then exit_found = true; return
        if L = 1 then result.push(pos); f 3,2        
    U:
    R:
    D:
*/


// 1 0
// 1 x
//
// f(1,1)
fn find_exit_route_impl<const N: usize>(
    arr: &[[u8;N]],
    start_pos: (usize,usize),
    pos: (usize, usize),
    exit_found: &mut bool,
    res: &mut Vec<Dir>,
) {
    println!("pos: {pos:?}"); // debug

    use Dir::*;
    if *exit_found { return; }
    // if pos.0 == 0 || pos.0 == arr.len()-1 || pos.1 == 0 || pos.1 == N-1 {
    //     return;
    // }

    // L
    println!("- L {pos:?}"); // debug
    if res.last() != Some(&R) {
        if pos.1 == 0 { *exit_found = true; return; }
        if arr[pos.0][pos.1-1] == 1 {
            res.push(L);
            if pos.1-1 == 0 { *exit_found = true; return; }
            find_exit_route_impl(arr, start_pos, (pos.0, pos.1-1), exit_found, res);
        }
    }

    if *exit_found { return; }
    // U
    println!("- U {pos:?}"); // debug
    if res.last() != Some(&D) {
        if pos.0 == 0 { *exit_found = true; return; }
        if arr[pos.0-1][pos.1] == 1 {
            res.push(U);
            if pos.0-1 == 0 { *exit_found = true; return; }
            find_exit_route_impl(arr, start_pos, (pos.0-1, pos.1), exit_found, res);
        }
    }

    if *exit_found { return; }
    // R 
    println!("- R {pos:?}"); // debug
    if res.last() != Some(&L) {
        if pos.1 == arr.len()-1 { *exit_found = true; return; }
        if arr[pos.0][pos.1+1] == 1 {
            res.push(R);
            // if pos.1+1 == arr.len()-1 { *exit_found = true; return; }
            find_exit_route_impl(arr, start_pos, (pos.0, pos.1+1), exit_found, res);
        }
    }    

    if *exit_found { return; }
    // D    
}

#[test]
fn foo() {
    use Dir::*;

    // 1 0 0 1
    // 0 x 1 1 
    // 0 1 1 1
    let arr = [[1,0,1,1],[0,1,1,1],[0,1,1,1]];
    //                      ^
    let res = find_exit_route(&arr, (1,1));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[R,R]);


    // 1 0 1 
    // 0 x 1
    // 0 1 1
    let arr = [[1,0,1],[0,1,1],[0,1,1]];
    //                    ^
    let res = find_exit_route(&arr, (1,1));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[R]);
    
    // 1 1 1 
    // 1 1 1
    // 0 1 x
    let arr = [[1,1,1],[1,1,1],[0,1,1]];
    //                              ^
    let res = find_exit_route(&arr, (2,2));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[L,U,L]);

    // 1 1 1 
    // 0 1 1
    // 0 1 x
    let arr = [[1,1,1],[0,1,1],[0,1,1]];
    //                              ^
    let res = find_exit_route(&arr, (2,2));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[L,U,U]);

    // 1 0 1 
    // 0 0 1
    // 1 1 x
    let arr = [[1,0,1],[0,0,1],[1,1,1]];
    //                              ^
    let res = find_exit_route(&arr, (2,2));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[L,L]);


    // 1 0 1 
    // 0 0 1
    // 1 0 x
    let arr = [[1,0,1],[0,0,1],[1,0,1]];
    //                              ^
    let res = find_exit_route(&arr, (2,2));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[U,U]);
    
    // 1 1 1 
    // 0 1 1
    // 1 0 x
    let arr = [[1,1,1],[0,1,1],[1,0,1]];
    //                              ^
    let res = find_exit_route(&arr, (2,2));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[U,L,U]);
    
    // 1 1
    // 0 x
    let arr = [[1,1],[0,1]];
    //                  ^
    let res = find_exit_route(&arr, (1,1));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[U]);

    // 1 0
    // 1 x
    //
    // res: L 
    let arr = [[1,0],[1,1]];
    //                  ^
    let res = find_exit_route(&arr, (1,1));
    println!("\n## res: {res:?}");
    assert_eq!(res, &[L]);
}

fn main() {
    println!("main start ---------");
    // 1 1
    // 0 x
    //
    // res: U
    let arr = [[1,1],[0,1]];
    //                  ^
    let res = find_exit_route(&arr, (1,1));
    println!("\n## res: {res:?}");


    // 1 0
    // 1 x
    //
    // res: L 
    let arr = [[1,0],[1,1]];
    //                  ^
    // let res = find_exit_route(&arr, (1,1));
    // println!("\n## res: {res:?}");


    println!("\n====== end of main =========");


    // Pinned
    //
    // println!("{xx}");
    // println!("{:?}");
    // println!("{}");
    // println!("xx: {:?}", xx);
    
    println!();
}

