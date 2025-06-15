// rust iterators == c++ algorithms

// interv-1

// From vec 1,2,3 to vec "1", "2", "3" - manual and use std lib fn
// From 1,2,3 create an Enum vector - enum Status { Value(u32) }
// 11. Combine two vecs: a1 = vec![1, 2], a2 = vec![4, 5] - manual and std lib fn
// 22. Convert vec!['a', 'b'] to  vec![(0, 'a'), (1, 'b')] - manual and use std lib fn
// 33. 2d arr to 1d array
//
// Check others:
// https://doc.rust-lang.org/std/iter/trait.Iterator.html





// How to test this file: 
// Run: cargo t iter

// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item; // The type of the elements being iterated over aka type placeholder
    fn next(&mut self) -> Option<Self::Item>;
    // . . .
}

// =======

/// Simple uses of iterators

// INFO:
// -----------------------------------------------------
// iter()      iterates over the items by reference
// into_iter() iterates over the items, moving them into the new scope
// iter_mut()  iterates over the items, giving a mutable reference to each item


// next
#[test]
pub fn next() {
    let v1 = [1, 2, 3]; // Vec<i32, Global>
    let mut v1_iter = v1.iter(); // Iter<i32>
    assert_eq!(v1_iter.next(), Some(&1));  // v1_iter.next(): Option<&i32>
    assert_eq!(v1_iter.next(), Some(&2));
}

// sum (reduce)
#[test] fn sum() {
    let v1 = [1, 2, 3]; // Vec<i32, Global>
    let v1_iter = v1.iter(); // Iter<i32>
    let sum: i32 = v1_iter.sum(); // total: 6
    assert_eq!(sum, 6);
    // or
    let sum: i32 = v1.iter().sum(); // total: 6
    assert_eq!(sum, 6);

    // map & sum
    let sum: i32 = v1.iter().map(|e| e*10).sum(); // total: 60
    assert_eq!(sum, 60);
}

// map
#[test] fn incr_one() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        // * iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 3, 4]);
}

// map - with Trait fn ptr
#[test]
fn map_with_fn_ptr() {
    let nums = [1, 2, 3];
    let strs: Vec<String> = nums.iter().map(ToString::to_string).collect();
    // instead of 
    let _strs: Vec<String> = nums.iter().map(|i| i.to_string()).collect();

    assert_eq!(strs, ["1", "2", "3"]);
}

// map - called by initializer function
#[derive(Debug, PartialEq)]
enum Status { Value(u32), Stop }

#[test]
fn map_with_init_fn() {
    let res: Vec<Status> = (1u32..3)
        .map(Status::Value) // initializer function
        .collect();
    assert_eq!(res, [Status::Value(1), Status::Value(2)]);
}

// filter
#[test] fn filter_even() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.into_iter().filter(|e| e % 2 == 0).collect();
        // * iters are lazy evaluated, collect() forces evaluation
    assert_eq!(v2, vec![2, 4]);
}

// mixing: filter/take/map/sum
// sum the squares of the first five even items 
#[test] fn sum_squares_of_first_two_even_items() {
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

#[test]
fn chain_11() {
    let a1 = [1, 2];
    let a2 = [4, 5];
    let vec: Vec<_> = a1.iter().chain(a2.iter()).collect(); // Vec<&i32>
    assert_eq!(vec, [&1, &2, &4, &5]);
}

#[test]
fn enumerate_22() {
    let arr = ['a', 'b'];
    let vec_2: Vec<(usize, &char)> = arr.iter().enumerate().collect();
    assert_eq!(vec_2, vec![(0, &'a'), (1, &'b')]);
}

#[test]
fn find() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);
}

#[test] fn flatten_33() {
    let data = vec![vec![1, 2], vec![3, 4]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4]);
}

#[test]
fn zip() {
    let keys = ["b", "a"];
    let vals = [2, 1];
    use std::collections::BTreeMap;
    let args: BTreeMap<_,_> = keys.iter().zip(vals.iter()).collect();
    assert_eq!(args, BTreeMap::from([(&"a", &1), (&"b", &2)]));
}

#[test]
fn inspect() {
    let a = [1, 4, 2, 3];
 
    // inspect is mostly for debugging
    // adding inspect() to investigate what's happening
    let sum = a.into_iter()
        .inspect(|x| println!("--- filtering: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("--- passed filter: {x}"))
        .sum::<i32>();

    assert_eq!(sum, 6);
}



#[derive(Debug, PartialEq)]
struct Foo { x: i32 } 

#[test]
fn stop_at_error_return_result() {
    // 1. Stop iteration at "error" w/ collect() and return Result
    // a
    let arr = [ Foo { x: 2 }, Foo { x: -2 }, Foo { x: 6 } ];
    let mut itr_cnt = 0;
    let res = arr.iter().map(|foo| { // Result<Vec<&Foo>, &str>
        dbg!(foo.x); // prints: 2, -2
        itr_cnt += 1;
        if foo.x < 0 { Err("negative elem") } // some breaking condition
        else { Ok(foo) }
    }).collect::< Result<Vec<&Foo>, _> >();
    assert_eq!(itr_cnt, 2); // not 3, Foo {6} is not processed
    assert_eq!(res, Err("negative elem"));
    // b
    // Non-err case: same code, only change is all elems of arr are positive
    let arr = [ Foo { x: 2 }, Foo { x: 2 }, Foo { x: 6 } ];
    let res = arr.iter().map(|f| {
        if f.x < 0 { Err("negative elem") } // some breaking condition
        else { Ok(f) }
    }).collect::<Result<Vec<&Foo>, _>>();
    assert_eq!(res.unwrap().len(), 3);

    // 2. Stop iteration at "error" w/ "sum()" and return Result
    // a
    let arr = [ Foo { x: 2 }, Foo { x: -2 }, Foo { x: 6 } ];
    let mut itr_cnt = 0;
    let res = arr.iter().map(|f| {
        itr_cnt += 1;
        if f.x < 0 { Err("negative element") } 
        else { Ok(f.x) }
    }).sum::<Result<i32, _>>();
    assert_eq!(itr_cnt, 2); // not 3
    assert_eq!(res, Err("negative element"));
    // b
    // Non-err case: same code, only change all elems of arr are positive
    let arr = [ Foo { x: 2 }, Foo { x: 2 }, Foo { x: 6 } ];
    let res = arr.iter().map(|foo| {
        if foo.x < 0 { Err("negative element") } 
        else { Ok(foo.x) }
    }).sum::<Result<i32, _>>();
    assert_eq!(res, Ok(10));
}