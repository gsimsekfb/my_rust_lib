// Problem Description
// Given a boolean matrix mat[M][N] of size M X N, modify it such
// that if a matrix cell mat[i][j] is 1 then set
// all the cells of ith row and jth column as 1. 
//
// e.g.
    // -1s are markers, to be set finally
    // 0 0 0              0 -1 0              0 -1 -1
    // 0 1 1  @[1][1] -> -1  1 1  @[1][2] -> -1  1  1  
    // 0 0 0              0 -1 0              0 -1 -1
    // let mut mat = [[0,0,0],[0,1,1],[0,0,0]];
    // let res     = [[0,1,1],[1,1,1],[0,1,1]];
//
// https://www.geeksforgeeks.org/a-boolean-matrix-question/

// todo
// sol-2

// todo
// see todo below


// Sol 1 - Brute force
// Hint: Use markers to not to modify existing matrix
// Time Complexity:O(N*M*(N+M)) + O(N*M)
// Space Complexity:O(1)
//
// set_matrix rename set_rows_columns_at_set_elems
//
// v1
// fn set_matrix(m: &mut [[i32;4]]) { // v1
// v2 - generic version
// fn set_matrix<const J: usize, T>(m: &mut [[T;J]]) {
    // todo: error[E0401]: can't use generic parameters from outer item
    // https://www.reddit.com/r/rust/comments/qjm0uv/what_is_a_functional_way_to_change_one_item_in/
fn set_matrix<const J: usize>(m: &mut [[i32;J]]) { // v3
    println!("------------");
    print_matrix(&m, "before:");
    const MARKER: i32 = 7; 
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            // m[i][j] -> mark m[i][...]s and m[...][j]s
            if m[i][j] == 1 {
                for e in 0..m[i].len() {
                    if m[i][e] != 1 {
                        m[i][e] = MARKER;
                    }
                }
                for e in 0..m.len() {
                    if m[e][j] != 1 {
                        m[e][j] = MARKER;
                    }
                }
            }
        }
    }
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == MARKER {
                m[i][j] = 1;
            }
        }
    }
    print_matrix(&m, "after:");
    println!("------------");
}

#[test] fn test() {
    // Sol-1
    // -1s are markers to be set finally
    // 0 0 0              0 -1 0              0 -1 -1
    // 0 1 1  @[1][1] -> -1  1 1  @[1][2] -> -1  1  1  
    // 0 0 0              0 -1 0              0 -1 -1
    let mut mat = [[0,0,0],[0,1,1],[0,0,0]];
    let res     = [[0,1,1],[1,1,1],[0,1,1]];
    set_matrix(&mut mat);
    assert_eq!(mat, res);
    //
    let mut mat = [[0,0,0],[0,1,0],[0,0,0]];
    let res     = [[0,1,0],[1,1,1],[0,1,0]];
    set_matrix(&mut mat);
    assert_eq!(mat, res);
    //
    let mut mat = [[0,0,0,0],[0,1,0,0],[0,0,0,1]];
    let res     = [[0,1,0,1],[1,1,1,1],[1,1,1,1]];
    set_matrix(&mut mat);
    assert_eq!(mat, res);

    // Sol-2
    // todo
}


// v1
// fn print_matrix(arr: &[[i32;4]]) {
// v2 - generic
// use std::fmt::Display;
// fn print_matrix<const J: usize, T: Display>(arr: &[[T;J]], s: &str) {
// v3 - accept all sizes
fn print_matrix<const J: usize>(arr: &[[i32;J]], s: &str) {
    println!("{s}");
    for i in arr {
        for j in i {
            print!("{j} ");
        }
        println!("");
    }
    println!("");
}
