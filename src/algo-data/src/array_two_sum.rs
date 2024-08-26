
// Problem Description
// Given an array of integers and an integer sum, 
// return the two numbers (or indices) that add up to sum.
//
// e.g. let arr = [5,6,7,8], sum=13, return (6,7)
//
// https://www.geeksforgeeks.org/check-if-pair-with-given-sum-exists-in-array/
// https://medium.com/@AlexanderObregon/solving-the-two-sum-problem-on-leetcode-rust-solutions-walkthrough-26a2e8b48616

#[test] fn all_solutions() {    
    // Sol-1
    let arr = [1,2,3,4];
    assert_eq!(two_sum(&arr, 0), None);
    assert_eq!(two_sum(&arr, 7), Some((3,4)));

    // Sol-2
    let arr = [5,6,7,8];
    assert_eq!(two_sum_2(&arr, 14), Some((6,8)));

    // Sol-3
    let arr = [5,6,7,8];
    assert_eq!(two_sum_3(&arr, 14), Some((8,6)));
    let arr = [5,6,7,8,9,10];
    assert_eq!(two_sum_3(&arr, 17), Some((9,8)));
}

// Solution 1: Brute Force Approach
// Iterate over the entire array, and for each element, nested iterate through 
// the remaining elements to find a pair that adds up to the sum 
    // 1->2,3,4  - arr[0] vs arr[1,2,3]
    //   2->3,4  - arr[1] vs arr[2,3]
    //     3->4  - arr[2] vs arr[3]
// Time Complexity: O(n*n) (approximately n*(n-1)/2 comparisons)
// Space Complexity: O(1), no additional data structures and
// the space required does not depend on the size of the input array.
// e.g. let arr = [1,2,3,4]
fn two_sum(arr: &[i32], sum: i32) -> Option<(i32, i32)> {
    for i in 0..arr.len()-1 { // i: 0 .. 2
        // println!("{i}");
        for j in i+1..arr.len() {  // j: i+1 .. 3
            // println!("{i} {j}");
            if arr[i] + arr[j] == sum { return Some((arr[i], arr[j])) }
        }
    }
    None
}


// Solution 2: Two-Pass Hash Table
// Using a hash map to improve time complexity. 
// First pass, each element and its index are added to a hash map. 
// Second pass, iterate through the array again, 
// checking if (sum — element) exists in the hash map.
// Time Complexity: O(n) — The array is iterated over once, and hash map 
// operations take constant time, leading to linear time complexity.
// Space Complexity: O(n) — The hash map stores up to ’n’ elements, 
// resulting in linear space complexity.
// e.g. let arr = [5,6,7,8], sum=13, return (6,7)
fn two_sum_2(arr: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut map = std::collections::HashMap::new();
    for (i, e) in arr.iter().enumerate() {
        map.insert(e, i);
    } // map {8: 3, 7: 2, 6: 1, 5: 0}
    for (i, e) in arr.iter().enumerate() {
        if let Some(index) = map.get(&(sum-e)) {
            return Some((arr[i], arr[*index]))
        }
    }
    None
}


// Solution 3: One-Pass Hash Map
// This approach optimizes the two-pass hash map method by only iterating through 
// the array once. During this iteration, each element’s index is added to
// the hash map, and we simultaneously check if its complement already exists 
// in the hash map.
// Time Complexity: O(n) — The array is iterated over once, and hash map 
// operations take constant time, leading to linear time complexity.
// Space Complexity: O(n) — The hash map stores up to ’n’ elements, resulting 
// in linear space complexity.
// e.g. let arr = [5,6,7,8], sum=14, return (6,8)
fn two_sum_3(arr: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut map = std::collections::HashMap::new();
    for (i, e) in arr.iter().enumerate() {
        if let Some(index) = map.get(&(sum-e)) {
            return Some((arr[i], arr[*index]))
        }
        map.insert(e, i);
    } // map {8: 3, 7: 2, 6: 1, 5: 0}
    None
}
