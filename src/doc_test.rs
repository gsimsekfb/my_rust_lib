// Todo

// Test with
// rustdoc --test src\doc_test.rs


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use super::add_two;
/// 
/// let x = 5;
/// let res = add_two(x);
///
/// assert_eq!(6, res);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}