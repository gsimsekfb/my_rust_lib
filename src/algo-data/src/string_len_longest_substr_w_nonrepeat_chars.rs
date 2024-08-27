// Problem Description
// Length of the longest substring without repeating characters
// e.g.
// Input: “ABCBC” Output: 3, longest substring is “ABC”
// Input: “AAA” Output: 1,   longest substring is “A”
// Input: “GEEKSFORGEEKS” Output: 7, longest substrings “EKSFORG” and “KSFORGE"
// 
// https://www.geeksforgeeks.org/length-of-the-longest-substring-without-repeating-characters/


// Sol-1: Brute Force
// e.g. "ababcdeabcd";  // ab abcde abcd  ->  res = abcde
//
// Hint: Keep two strings one with the last longest str, other current longest
// Time : O(n^2)
// Space: O(n)
fn longest_substr_w_non_repeating_chars(s: &str) -> (usize, String) {
    let mut res = String::new();  // last saved longest str
    let mut temp = String::new(); // current longest str
    for c in s.chars() {
        let mut found = false;
        for i in 0..temp.len() {
            if temp.chars().nth(i).unwrap() == c { 
                found = true;
                temp.clone_from(&c.to_string()); // restart temp
                break;
            };
        }
        if !found { temp.push(c) };
        if temp.len() > res.len() { res.clone_from(&temp); }
    }
    (res.len(), res)
}


#[test] fn tt() {

    // Sol-1
    let s = "ababcdeabcd";   // ab abcde abcd
    assert_eq!(longest_substr_w_non_repeating_chars(s), (5, "abcde".to_string()));
    //
    let s = "GEEKSFORGEEKS"; // GE EKSFORG EEKS
    assert_eq!(longest_substr_w_non_repeating_chars(s), (7, "EKSFORG".to_string()));
    //
    let s = "abcaba";   // abc ab a
    assert_eq!(longest_substr_w_non_repeating_chars(s), (3, "abc".to_string()));
    //
    let s = "aababc";   // a ab abc
    assert_eq!(longest_substr_w_non_repeating_chars(s), (3, "abc".to_string()));
    //
    let s = "aab";   // a ab
    assert_eq!(longest_substr_w_non_repeating_chars(s), (2, "ab".to_string()));
    //
    let s = "ab";
    assert_eq!(longest_substr_w_non_repeating_chars(s), (2, "ab".to_string()));
    //
    let s = "aaa";
    assert_eq!(longest_substr_w_non_repeating_chars(s), (1, "a".to_string()));
    //
    let s = "a";
    assert_eq!(longest_substr_w_non_repeating_chars(s), (1, "a".to_string()));
}
