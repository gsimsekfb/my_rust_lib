// Helper
struct Counter { count: u32 }
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//// a
// Simple, dummy representation of Iterator trait
// Also an example of a trait with an associated type
pub trait Iterator {
    type Item; // The type of the elements being iterated over aka type placeholder
    // Implementors of the Iterator trait will specify the concrete type for Item, 
    // and the next method will return an Option containing a value of that 
    // concrete type

    fn next(&mut self) -> Option<Self::Item>;
    // . . .
}

// Creating Our Own Iterator
// An iterator that will only ever count from 1 to 2
impl Iterator for Counter {
    type Item = u32;  // we can only choose what the type of Item will -
        // be once (u32), because there can only be one impl Iterator for Counter

    // the only method required to provide a definition
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 3 { Some(self.count) } 
        else { None }
    }
}

// error ^^^^^^^^^^^^^ conflicting implementation for `iterators::Counter`
// We can only choose what the type of Item will -
// be once (u32), because there can only be one impl Iterator for Counter
//
// // Creating Our Own Iterator - 2
// impl Iterator for Counter {
//     type Item = String;
//     // . . .
// }

#[test] fn iter_our_iter_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}

// b 
// For comparison:
// A hypothetical definition of the Iterator trait using generics
pub trait Iterator_<T> {
    fn next_(&mut self) -> Option<T>;
}
impl Iterator_<u32> for Counter {
    fn next_(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count < 3 { Some(self.count) } 
        else { None }
    }
}
// Diff: - we must annotate the types in each implementation
//       - it can be implemented for a type multiple times 
//         (we can also implement Iterator<String> for Counter or any other type)

#[test] fn iter_w_generic_trait_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next_(), Some(1));
    assert_eq!(counter.next_(), Some(2));
    assert_eq!(counter.next_(), None);
}