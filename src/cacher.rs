use std::collections::HashMap;

// todos: 
// - yml for macos
// - Key and Val are same types. Either combine them into one type or 
//   make Val different type.

pub struct Cacher<T, Key, Val>
    where T: Fn(Key) -> Val, 
          Key: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display,
          Val: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display
{
    calculation: T, // calc. fn
    results: HashMap<Key, Val>,
}

impl<T, Key, Val> Cacher<T, Key, Val>
    where T: Fn(Key) -> Val, 
          Key: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display,
          Val: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display
{
    pub fn new(calculation: T) -> Cacher<T, Key, Val> {
        Cacher {
            calculation,
            results: HashMap::<Key, Val>::new(),
        }
    }

    pub fn result(&mut self, arg: Key) -> Val {
        if self.results.contains_key(&arg) {
            println!("Cacher: getting cashed result for {}...", arg);
            *self.results.get(&arg).unwrap()
        } else {
            println!("Cacher: calculating for {}...", arg);
            let val = (self.calculation)(arg);
            self.results.insert(arg, val);
            val
        }
        // or just:
        // *self.results.entry(arg).or_insert(
        //     (self.calculation)(arg)
        // )
    }
}

#[test]
fn call_with_different_resultss() {
    let mut cacher = Cacher::new(|a| a);
    let v1 = cacher.result(1);
    let v2 = cacher.result(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}