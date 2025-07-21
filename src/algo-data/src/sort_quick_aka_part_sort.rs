
// skip: too hard, no need to re-invent the wheel

// interv-3

#[test] fn sol_1() {

    // Sol-1
    let mut arr = [4,3];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    assert_eq!(arr, [3,4]);

    let mut arr = [3, 4];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    assert_eq!(arr, [3,4]);
    
    let mut arr = [2,4,5,1,3];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    assert_eq!(arr, [1,2,3,4,5]);

    let mut arr = [6,4,5,1,3];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    assert_eq!(arr, [1,3,4,5,6]);

    let mut arr = [8,2,4,5,1,3,9];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    assert_eq!(arr, [1,2,3,4,5,8,9]);
}

// Problem Description
// QuickSort aka partition sort
// 
// e.g. [2 4 5 1 3], 0, 4 -> ret [1 2 3 4 5]
//
// https://www.geeksforgeeks.org/cpp-program-for-quicksort/

// Sol 1 - Divide and conquer
//
// Hint: Partition sort arr first, then recursly do left then right side of arr
//
// Time : O(NlogN) on average and O(N^2) in worst case
// Space: O(logN) on average and O(N) in worst case
//
// Partition arr, find part_index, partition left and then right of 
// this index. Do this recursively
fn quick_sort(arr: &mut [i32], beg: usize, end: usize) {
    if beg >= end { return };
    let part_index = partition(arr, beg, end);
    if part_index == 0 { return };  // part_index(usize)-1 might overflow
    quick_sort(arr, beg, part_index-1); // left part
    quick_sort(arr, part_index+1, end); // right part
}
// Debug
// f(arr: [2 4 5 1 3], 0, 4)-> ret 2 1 3 4 5
//   len = 5
//   arr [2 1 3 4 5],  pi 2
//   f(arr , 0, 1)   // l: [2 1]
//     arr [1 2], pi 0
//       f(arr, 0, -1) ret 
//   f(arr, 3, 4)   // r: [4 5]
//     arr [4 5] , pi 4
//       f(arr, 5, 4) 

// [2 4 5 1 3], 0, 4 -> ret [2 1 3 4 5], 2
fn partition(arr: &mut [i32], beg: usize, end: usize) -> usize {
    let pivot = arr[end]; // deb: 5 (last elem)
    let mut part_index = beg;
    for i in beg..end { // [0..3] // loop except last elem aka pivot
        if arr[i] < pivot {
            arr.swap(i, part_index); // or use "temp" instead of swap
            part_index += 1;
        }
    }
    arr.swap(part_index, end);
    part_index
}
// Algo:
// Put elems smaller than pivot on the left side and 
// bigger on the right side of part_index
// e.g. 2 4 5 1 3
// ret  2 1 3 4 5, part_index=2
//          
// Psuedo-code:
// Select pivot as last elem
// Start part_index with 0
// Loop array except pivot (last elem)
//    if elem at i < elem at part_index 
//    // todo
// 
// loop [0..len-1)
// 	if arr[ i ] < pivot 	
// 		swap( arr[ i ] , arr [ part_index ]
// 		++ part_index	
// swap( arr[ part_index ], pivot aka arr[ len-1 ] )
//
// Debug:		
// f ( 2 4 5 1 3 )-> ret 2 1 3 4 5
// > loop
// 	-----------------------------------
// 		i 0		     
// 		|
// 	arr 2 4 5 1 3 		-  pivot 3
// 		|
// 		part_index 0
// 	-----------------------------------
// 		  i 1
// 		  |
// 	arr 2 4 5 1 3 		-  pivot 3
// 		  |
// 		  part_index 1
// 	-----------------------------------
// 		    i 2
// 		    |
// 	arr 2 4 5 1 3 		-  pivot 3
// 		  |
// 		  part_index 1
// 	-----------------------------------
// 		      i 3
// 		      |
// 	arr 2 4 5 1 3 		-  pivot 3
// 		  |
// 		  part_index 1
// 	-----------------------------------
// 	end of loop
//  after loop
// 	-----------------------------------
// 	arr 2 1 5 4 3 		-  pivot 3
// 		    |
// 		    part_index 2
// 	-----------------------------------
//  // Final swap
// > swap( part_index, pivot )
// 	arr 2 1 3 4 5  		-  pivot 3