// Todo

// Test with
// rustdoc --test src\doc_test.rs


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = 5;
/// let res = rust_book_minigrep::add_one(x);
///
/// assert_eq!(6, res);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}