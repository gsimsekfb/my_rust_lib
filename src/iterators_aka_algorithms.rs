// rust iterators == similar to c++ algorithm

// How to test this file: 
// Run: cargo t iter

// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item; // The type of the elements being iterated over aka type placeholder
    fn next(&mut self) -> Option<Self::Item>;
    // ...
}

// =======

/// Simple uses of iterators

// next
pub fn foo() {
    let v1 = vec![1, 2, 3]; // Vec<i32, Global>
    let mut v1_iter = v1.iter(); // Iter<i32>
    assert_eq!(v1_iter.next(), Some(&1));
    let xx = v1_iter.next(); // Option<&i32>
}

// reduce
#[test] fn iter_sum() {
    let v1 = vec![1, 2, 3]; // Vec<i32, Global>
    let v1_iter = v1.iter(); // Iter<i32>
    let total: i32 = v1_iter.sum(); // total: 6
    assert_eq!(total, 6);
    // or
    let total: i32 = v1.iter().sum(); // total: 6
    assert_eq!(total, 6);
}

// map
#[test] fn iter_inc_one() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        // * iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 3, 4]);
}

// filter
#[test] fn iter_filter_even() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.into_iter().filter(|e| e % 2 == 0).collect();
        // * iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 4]);
}

// mixing: filter/take/map/sum
// sum the squares of the first five even items 
#[test] fn iter_sum_squares_of_first_two_even_items() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let even_sum_squares: i32 = v1
        .iter()
        .filter(|x| *x % 2 == 0)
        .take(2)
        .map(|x| x * x)
        .sum();
        // 2*2 + 4*4 = 20
    assert_eq!(even_sum_squares, 20);
}

#[test] fn iter_chain() {
    let a1 = vec![1, 2];
    let a2 = vec![4, 5];
    let vec: Vec<_> = a1.iter().chain(a2.iter()).collect();
    assert_eq!(vec, vec![&1, &2, &4, &5]);
}

#[test] fn iter_enumerate() {
    let aa = vec!['a', 'b'];
    let iter = aa.iter().enumerate();
    let vec: Vec<_> = iter.collect();
    assert_eq!(vec, vec![(0, &'a'), (1, &'b')]);
    // or
    // assert_eq!(iter.next(), Some((0, &'a')));
}

#[test] fn iter_find() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);
}

#[test] fn iter_flatten() {
    let data = vec![vec![1, 2], vec![3, 4]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4]);
}
