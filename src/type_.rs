
// interv-1

// 1. use "type" in trait body e.g. impl MyIter::next()
// 2. Create Type Synonym Meters for i32;
// Use them in test




// =======================================================================





// 1.
// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item;  // The type of the elements being iterated over

    fn next(&mut self) -> Option<Self::Item>;
    // . . .
}

// Simple usage of iterators
fn foo() {
  #[allow(clippy::useless_vec)]
  let v1 = vec![1, 2, 3];      // Vec<i32, Global>
  let mut v1_iter = v1.iter(); // Iter<i32>
  assert_eq!(v1_iter.next(), Some(&1));

  let xx = v1_iter.next();     // Option<&i32>
}


// 2. 
// Creating Type Synonyms with Type Aliases
type Meters = i32;

#[test] fn ex_2() {
    let x: i32 = 3;
    let y: Meters = 2;
    assert_eq!(x+y, 5);
}
