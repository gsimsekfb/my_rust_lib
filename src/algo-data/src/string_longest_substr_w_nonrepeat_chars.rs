// last: 12/25

// Problem Description
// Longest substring without repeating characters w/ space and time complex.
// e.g.
// Input: “AAA” longest substring is: “A”
// Input: “ABCBC”: “ABC”
// Input: “GEEKSFORGEEKS” longest substrings “EKSFORG” or “KSFORGE"
// 
// slightly modified version of:
// https://www.geeksforgeeks.org/length-of-the-longest-substring-without-repeating-characters/

// Hint: at the end


// todo: do the better version "sliding window" ?


// ===================================




// Sol-1: Brute Force
// e.g. "ababcdeabcd";  // ab abcde abcd  ->  res = abcde
//
// Time : O(n x 26)
// Space: O(n)
fn longest_substr_w_non_repeating_chars(word: &str) -> String {
    let mut longest = "".to_string(); // Space O(n)
    let mut temp = "".to_string();

    for ch in word.chars() {    // O(n)
        if let Some(pos) = temp.find(ch) {    // O(26)
            if temp.len() > longest.len() { longest = temp.clone() }
            temp.drain(0..pos+1); // temp is already in longest, clear it
                // e.g. first temp in this scope for "ababc" is : "ab" 
        }
        temp.push(ch);
    }

    if temp.len() > longest.len() { longest = temp }

    longest
}


#[test] fn tt() {
    // Sol-1
    let s = "ababcdeabcd";   // ab abcde abcd
    assert_eq!(longest_substr_w_non_repeating_chars(s), "abcde".to_string());
    //
    let s = "GEEKSFORGEEKS"; // GE EKSFORG EEKS
    assert_eq!(longest_substr_w_non_repeating_chars(s), "EKSFORG".to_string());
    //
    let s = "abcaba";   // abc ab a
    assert_eq!(longest_substr_w_non_repeating_chars(s), "abc".to_string());
    //
    let s = "aababc";   // a ab abc
    assert_eq!(longest_substr_w_non_repeating_chars(s), "abc".to_string());
    //
    let s = "aab";   // a ab
    assert_eq!(longest_substr_w_non_repeating_chars(s), "ab".to_string());
    //
    let s = "ab";
    assert_eq!(longest_substr_w_non_repeating_chars(s), "ab".to_string());
    //
    let s = "aaa";
    assert_eq!(longest_substr_w_non_repeating_chars(s), "a".to_string());
    //
    let s = "a";
    assert_eq!(longest_substr_w_non_repeating_chars(s), "a".to_string());

    let s1 = "ababcdeabcd";
    assert_eq!(longest_substr_w_non_repeating_chars(s1), "abcde".to_string());

    let s2 = "abcdeaf";
    assert_eq!(longest_substr_w_non_repeating_chars(s2), "bcdeaf".to_string());

    let s3 = "abba";
    assert_eq!(longest_substr_w_non_repeating_chars(s3), "ab".to_string());

    let s4 = "aaabcdeaf";
    assert_eq!(longest_substr_w_non_repeating_chars(s4), "bcdeaf".to_string());

    let s01 = "abcabcbb";
    assert_eq!(longest_substr_w_non_repeating_chars(s01), "abc".to_string());

    let s02 = "bbbbb";
    assert_eq!(longest_substr_w_non_repeating_chars(s02), "b".to_string());

    let s03 = "pwwkew";
    assert_eq!(longest_substr_w_non_repeating_chars(s03), "wke".to_string());

    let s04 = "abcdeaf";
    assert_eq!(longest_substr_w_non_repeating_chars(s04), "bcdeaf".to_string());

    let s05 = "abba";
    assert_eq!(longest_substr_w_non_repeating_chars(s05), "ab".to_string());

    let s06 = "anviaj";
    assert_eq!(longest_substr_w_non_repeating_chars(s06), "nviaj".to_string());

    let s07 = "dvdf";
    assert_eq!(longest_substr_w_non_repeating_chars(s07), "vdf".to_string());

    let s08 = "tmmzuxt";
    assert_eq!(longest_substr_w_non_repeating_chars(s08), "mzuxt".to_string());

    let s09 = "";
    assert_eq!(longest_substr_w_non_repeating_chars(s09), "".to_string());

    let s10 = "a";
    assert_eq!(longest_substr_w_non_repeating_chars(s10), "a".to_string());

    let s11 = "abcd";
    assert_eq!(longest_substr_w_non_repeating_chars(s11), "abcd".to_string());

    let s12 = "abcdaefghi";
    assert_eq!(longest_substr_w_non_repeating_chars(s12), "bcdaefghi".to_string());

    let s13 = "aaaaaaa";
    assert_eq!(longest_substr_w_non_repeating_chars(s13), "a".to_string());

    let s14 = "abcbacabc";
    assert_eq!(longest_substr_w_non_repeating_chars(s14), "abc".to_string());

}



// 1. brute force
// Hint: Keep two strings: last longest and current longest
