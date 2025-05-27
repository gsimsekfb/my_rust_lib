
// Problem Description
// Count set bits in an integer
//
// e.g. 6 is 110, ret 2
//
// https://www.geeksforgeeks.org/count-set-bits-in-an-integer/?ref=lbp


// todo
// Sol 3. Lookup table
// https://www.geeksforgeeks.org/count-set-bits-in-an-integer/?ref=lbp


// Sol 1. Simple Loop
// Hint: Looping through all bits in an integer using bitwise "and" and "shift" 
// Exp: init cntr, incr cntr w/ 
// int's last bit setness using "bitwise and" op, then shift right to do
// the next bit. Do this until the int is 0.  
// Time Complexity: O(log n)  ??  n? 
// Auxiliary Space: O(1)
fn count_set_bits_loop(mut n: usize) -> usize {
    let mut cnt = 0;
    while n != 0 {
        cnt += n & 1;
        n >>= 1;
    }
    cnt
}

// Sol 2. Recursive
// Hint: Recurse and shift right until int is 0, count last bit setness
// Exp: Recurse/shift right until n is 0 which is last/exit case, return 0.
// For other cases, return sum of last bit setness and 
// previous fn return
// Time Complexity: O(log n)
// Auxiliary Space: O(log n) for recursive stack space
fn count_set_bits_recur(n: usize) -> usize {
    if n == 0 { return 0; }
    (n & 1) + count_set_bits_recur(n >> 1)
    // todo: whats wrong w/ line below, + run before & ?
    // return n & 1 + count_set_bits_recur(n >> 1);
}
// debug f(13)
// f(1101) ret 1 + 2 = 3
//   f(110) ret 0 + 2 = 2 
//     f(11) ret 1 + 1 = 2
//       f(1) ret 1 + 0 = 1
//         f(0) ret 

#[test] fn test() {
    // Sol-1
    // 110
    assert_eq!(count_set_bits_loop(6), 2);
    // 1101
    assert_eq!(count_set_bits_loop(13), 3);
    // 10011
    assert_eq!(count_set_bits_loop(19), 3);

    // Sol-2
    // 110
    assert_eq!(count_set_bits_recur(6), 2);
    // 1101
    assert_eq!(count_set_bits_recur(13), 3);
    // 10011
    assert_eq!(count_set_bits_recur(19), 3);

    // Sol-3
    // todo
}

