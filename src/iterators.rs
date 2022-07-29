// How to test this file? 
// Run: cargo t iter

// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item; // The type of the elements being iterated over

    fn next(&mut self) -> Option<Self::Item>;
    // ...
}

/// Simple usages of iterators

pub fn foo() {
    let v1 = vec![1, 2, 3]; // Vec<i32, Global>
    let mut v1_iter = v1.iter(); // Iter<i32>
    assert_eq!(v1_iter.next(), Some(&1));
    let xx = v1_iter.next(); // Option<&i32>
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3]; // Vec<i32, Global>
    let v1_iter = v1.iter(); // Iter<i32>
    let total: i32 = v1_iter.sum(); // total: 6
    assert_eq!(total, 6);
    // or
    let total: i32 = v1.iter().sum(); // total: 6
    assert_eq!(total, 6);
}

#[test]
fn iter_inc_one() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iter_filter_even() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.into_iter().filter(|e| e % 2 == 0).collect();
    // iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 4]);
}

// ---------

/// Creating Our Own Iterator

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// An iterator that will only ever count from 1 to 5
impl Iterator for Counter {
    type Item = u32;

    // the only method required to provide a definition
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn iter_our_iter_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}