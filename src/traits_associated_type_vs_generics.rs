// interv-2

//// Trait with an associated type vs Generic trait
//// https://doc.rust-lang.org/rust-by-example/generics/assoc_items.html

// Task A
// 1
// Struct Counter with count u32 and def ctor w/ count 0
// Trait Iter with associated type Item fn next that returns Option of Item
// and fn value_not_zero returns bool
// Impl Iter for Counter:
// fn next will increment the count from 1 to 2, returns None when 3
// fn value_not_zero will return true if count is positive
//
// Impl Iter with associated type u32 for Counter
// Impl Iter with associated type f64 for Counter
//
// 2
// below

// Helper
struct Counter { count: u32 }
impl Counter {
    fn new() -> Self { Self { count: 0 } }
}

// -------------------------------------------------------------------------

//// 1. Trait with Associated type
pub trait Iter {
    type Item; // The type of the elements being iterated over aka type placeholder
    // Implementors of this trait will specify the concrete type for Item, 

    fn next(&mut self) -> Option<Self::Item>;
    fn value_not_zero(&self) -> bool;   // does not depend on Item
}

// An iterator that will only ever count from 1 to 2
impl Iter for Counter {
    type Item = u32;  // we can only choose what the type of Item will -
        // be once (u32), because there can only be one impl Iterator for Counter

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 3 { Some(self.count) } 
        else { None }
    }

    fn value_not_zero(&self) -> bool { self.count > 0 }
}

// error[E0119]: conflicting implementations of trait `Iter` for type `Counter`
//
// We can only choose what the type of Item will -
// be once (u32), because there can only be one impl Iterator for Counter
//
// impl Iter for Counter {
//     type Item = f64;
//     fn next(&mut self) -> Option<Self::Item> { Some(42 as f64) }
//     fn value_not_zero(&self) -> bool { true }
    
// }

#[test] fn iter_our_iter_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}


// -------------------------------------------------------------------------





// 2 - For comparison: Same trait using generics
//
// Trait GenIter<T> with fn next_ that returns Option<T>
// and fn value_not_zero_ returns bool
// Impl GenIter<u32> for Counter
// Impl GenIter<f64> for Counter
// Test:
// Use Iter, call next fn 3 times
// Test:
// Use GenIter, call next_ fn 3 times




// =======================================================================



pub trait GenIter<T> {
    fn next_(&mut self) -> Option<T>;
    fn value_not_zero(&self) -> bool;
}

impl GenIter<u32> for Counter {
    fn next_(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count < 3 { Some(self.count) } 
        else { None }
    }
    fn value_not_zero(&self) -> bool { self.count > 0 }
}

// Also possible
//
// impl GenIter<f64> for Counter {
//     fn next_(&mut self) -> Option<f64> {
//         self.count += 1;
//         if self.count < 3 { Some(self.count as f64) }
//         else { None }
//     }
//     fn value_not_zero(&self) -> bool { self.count > 0 }
// }

// Diff for generic version: 
// - we must annotate the types in each implementation
// - it can be implemented for a type multiple times 


#[test] fn iter_w_generic_trait_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next_(), Some(1));
    assert_eq!(counter.next_(), Some(2));
    assert_eq!(counter.next_(), None);
}


// -------------------------------------------------------------------


// skip ?
//// Task B
//
// Non-generic:
// fn dummy that accepts objects impl Iter returns value_not_zero()
// fn first_next that accepts objects impl Iter returns next() as u32
//
// Generic:
// fn dummy_ that accepts objects impl GenIter returns value_not_zero()
// fn first_next_ that accepts objects impl GenIter returns next() as u32
// Test:
// Test all fns

// Iter
fn dummy(c: &impl Iter) -> bool { // 222: Cleaner, no need to specify Item
    c.value_not_zero()  // fn value_not_zero(&self) -> bool
}
fn first_next(c: &mut impl Iter<Item = u32>) -> u32 {
    c.next().unwrap()   // fn next(&mut self) -> Option<Self::Item>;
}
// or same w/o impl
fn first_next__<T: Iter<Item = u32>>(c: &mut T) -> u32 {
    c.next().unwrap()
}


// GenIter
fn dummy_a<U>(c: &impl GenIter<U>) -> bool { // 222: We always need to provide T
    c.value_not_zero()
}
fn dummy_b(c: &impl GenIter<u32>) -> bool { // 222: We always need to provide T
    c.value_not_zero()
}
fn first_next_<T: GenIter<u32>>(c: &mut T) -> u32 {
    c.next_().unwrap()
}


#[test] fn usage() {
    let c = Counter::new(); // count: 0
    assert!(!dummy(&c));
    assert!(!dummy_a(&c));
    assert!(!dummy_b(&c));

    let mut c = Counter::new();
    assert_eq!(first_next(&mut c), 1);
    assert_eq!(first_next_(&mut c), 2);
}