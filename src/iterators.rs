

// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item;  // The type of the elements being iterated over

    fn next(&mut self) -> Option<Self::Item>;
    // ...
}

/// Simple usages of iterators

pub fn foo() {
  let v1 = vec![1, 2, 3];      // Vec<i32, Global>
  let mut v1_iter = v1.iter(); // Iter<i32>
  assert_eq!(v1_iter.next(), Some(&1));
  let xx = v1_iter.next();     // Option<&i32>
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3]; // Vec<i32, Global>
    let mut v1_iter = v1.iter(); // Iter<i32>
    let total: i32 = v1_iter.sum(); // total: 6
    assert_eq!(total, 6);
}