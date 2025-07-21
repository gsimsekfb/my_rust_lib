// last: 5/25

// Problem Description
// Given an array of integers and an integer sum, 
// return the indices that add up to sum.
//
// e.g. let arr = [5,6,7,8], sum=13, return (0,3) and (1,2)
//
// https://www.geeksforgeeks.org/check-if-pair-with-given-sum-exists-in-array/
// https://medium.com/@AlexanderObregon/solving-the-two-sum-problem-on-leetcode-rust-solutions-walkthrough-26a2e8b48616

#[test]
fn all_solutions() {    
    // Sol-1
    let arr = [1,2,3,4];
    assert_eq!(two_sum(&arr, 5), &[(0,3), (1,2)]);

    // Sol-2
    let arr = [5,6,7,8];
    assert_eq!(two_sum_2(&arr, 14), &[(1,3)]);

    // Sol-3
    let arr = [5,6,7,8];
    assert_eq!(two_sum_3(&arr, 13), &[(0,3), (1,2)]);
    let arr = [5,6,7,8,9,10];
    assert_eq!(two_sum_3(&arr, 17), &[(2,5), (3,4)]);
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
fn two_sum(arr: &[i32], sum: i32) -> Vec<(usize, usize)> {
    let mut res = vec![];

    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            // println!("{},{}", arr[i], arr[j]);
            if arr[j] + arr[i] == sum {
                res.push((i,j));
            }
        }
    }
    res
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
fn two_sum_2(arr: &[i32], sum: i32) -> Vec<(usize, usize)> {
    let mut res = vec![];
    use std::collections::HashMap;
    let mut map: HashMap<&i32,usize> = HashMap::new();
    // map: 5,0 - 6,1 - 7,2 - 8,3
    for (i, e) in arr.iter().enumerate() {
        map.insert(e, i);
    }
    for (i, e) in arr.iter().enumerate() {
        if let Some(val) = map.get(&(sum-e)).filter(|val| **val > i) {
            res.push((i, *val));
        }
        // or
        //
        // // same as above w/o filter
        // if let Some(val) = map.get(&(sum-e)) {
        //     if *val > i { res.push((i, *val)); }
        // }
        //
        // // w/o if let
        // if map.contains_key(&(sum-e)) && map[&(sum-e)] > i {            
        //     res.push((i, map[&(sum-e)]));
        // }       
    }
    res
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
fn two_sum_3(arr: &[i32], sum: i32) -> Vec<(usize, usize)> {
    let mut res = vec![];
    use std::collections::HashMap;
    let mut map: HashMap<&i32,usize> = HashMap::new();
    // map: 5,0 - 6,1 ... 8,3
    for (i, e) in arr.iter().enumerate() {
        map.insert(e, i);
        if let Some(val) = map.get(&(sum-e)) {
            res.push((*val, i));
        }        
    }
    res.into_iter().rev().collect()
}
