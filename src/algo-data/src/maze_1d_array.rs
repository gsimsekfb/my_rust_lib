#![allow(clippy::needless_range_loop)]

// interv-1

// maze 1d
// - fn find_exit_route_indexes(array, start_pos) -> <indexes>
// e.g. for arr 0,1,1,1, start_pos 1 , return 2,3
// - 1d array of 0s and 1s, where 1s open cell, 0s closed
// - Use iterative version, recursive vers. is waste of time
// - also
// - fn find_shortest_exit_route_indexes(array, start_pos) -> <indexes>




// ===========================================================




// Sol-1: iterative
// returns indexes as route
fn find_exit_route_iter(arr: &[u8], start_pos: usize) -> Vec<usize> {
    let mut result = vec![];

    // go left
    for i in 0 .. start_pos {
        if arr[i] == 0 { result.clear(); break; }
        result.push(i);
        if i == start_pos-1 { return result; }
    }
    // go right
    for i in start_pos+1 .. arr.len() {
        if arr[i] == 0 { result.clear(); break; }
        result.push(i);
    }

    result
}

fn find_shortest_exit_route_iter(arr: &[u8], start_pos: usize) -> Vec<usize> {
    // go left
    let mut result_left = vec![];
    for i in 0 .. start_pos {
        if arr[i] == 0 { result_left.clear(); break; }
        result_left.push(i);
    }

    // go right
    let mut result_right = vec![];
    for i in start_pos+1 .. arr.len() {
        if arr[i] == 0 { result_right.clear(); break; }
        result_right.push(i);
    }

    if !result_left.is_empty() && result_right.is_empty() { return result_left; }
    if result_left.is_empty() && !result_right.is_empty() { return result_right; }

    if result_left.len() < result_right.len() {
        result_left 
    } else {
        result_right
    }
}

#[test]
fn test_iter() {
    let arr = [1,0,1,1,1,1];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[3,4,5]);
    assert_eq!(find_shortest_exit_route_iter(&arr, 2), &[3,4,5]);

    let arr = [1,1,0,1,1,1,1];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[0,1]);
    assert_eq!(find_shortest_exit_route_iter(&arr, 2), &[0,1]);

    let arr = [1,1,1,1];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[0,1]);
    assert_eq!(find_shortest_exit_route_iter(&arr, 2), &[3]);

    let arr = [1,1,0,1,1,1,1];
    //           ^
    assert_eq!(find_exit_route_iter(&arr, 1), &[0]);
    assert_eq!(find_shortest_exit_route_iter(&arr, 1), &[0]);

    let arr = [1,0,0,1];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[3]);

    let arr = [0,1,0,1,1];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[3,4]);

    let arr = [0,1,0,1,1];
    //         ^
    assert_eq!(find_exit_route_iter(&arr, 0), &[]);

    let arr = [0,1,0];
    //             ^
    assert_eq!(find_exit_route_iter(&arr, 2), &[]);
}

// Sol-2: Skip ? Compared to iterative version, recur sol does not make sense
// Rules:
// - start_pos assumed to be an open cell aka 1
//
fn find_exit_route_recur(arr: &[u8], start_pos: usize) -> Vec<char> {
    let mut result = vec![];
    find_exit_route_recur_impl(arr, start_pos, start_pos, &mut result);
    result.pop(); // last element is the found marker
    println!("## res: {result:?}");
    result
}

fn find_exit_route_recur_impl(
    arr: &[u8], start_pos: usize, pos: usize, result: &mut Vec<char>
) {
    println!("pos: {pos}"); // debug
    if pos == 0 || pos == arr.len()-1 {
        println!("## success: exit found !!");
        result.push('S');
        return;
    }
    if pos <= start_pos {   // going only left dir.
        if arr[pos-1] == 1 {
            result.push('L'); println!("L ");
            find_exit_route_recur_impl(arr, start_pos, pos-1, result);
        } else { // optional print
            result.clear();
            println!(" left cell dead_end");
        }
    }
    if result.last() == Some(&'S') { return; }
    if pos >= start_pos {   // going only right dir.
        if arr[pos+1] == 1 {
            result.push('R'); println!("R ");
            find_exit_route_recur_impl(arr, start_pos, pos+1, result);
        } else { // optional print
            result.clear();
            println!(" right cell dead_end");
        }
    }
    println!("end-of-pos: {pos}"); // debug
}

#[test]
fn test_recur(){
    let arr = [0,1,1,1,0,1,1,1];
    //                 ^
    assert_eq!(find_exit_route_recur(&arr, 4), &['R','R','R']);

    let arr = [1,1,1,1,0,1,1,1];
    //               ^  
    assert_eq!(find_exit_route_recur(&arr, 3), &['L','L','L']);

    let arr = [0,1,1,1];
    //             ^
    assert_eq!(find_exit_route_recur(&arr, 2), &['R']);

    let arr = [1,1,1,1];
    //           ^
    assert_eq!(find_exit_route_recur(&arr, 1), &['L']);
    
    let arr = [0,1,1,1];
    //         ^
    assert_eq!(find_exit_route_recur(&arr, 0), &[]);

    let arr = [0,1,1,1];
    //               ^
    assert_eq!(find_exit_route_recur(&arr, 3), &[]);

    let arr = [0,1,1,0,1];
    //             ^
    assert_eq!(find_exit_route_recur(&arr, 2), &[]);    

    let arr = [1,0,1,1,0];
    //             ^
    assert_eq!(find_exit_route_recur(&arr, 2), &[]);      
}
