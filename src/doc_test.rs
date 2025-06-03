// Todo

// Test with
// rustdoc --test src\doc_test.rs


/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// use my_rust_lib::add_two;
/// 
/// let x = 5;
/// let res = add_two(x);
///
/// assert_eq!(7, res);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}