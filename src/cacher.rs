use std::collections::HashMap;

// todos: 
// - Key and Val are same types. Either combine them into one type or 
//   make Val different type.

pub struct Cacher<T, Key, Val>
    where T: Fn(Key) -> Val, 
          Key: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display,
          Val: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display
{
    function: T, // calc. fn
    results: HashMap<Key, Val>,
}

impl<Function, Key, Val> Cacher<Function, Key, Val>
    where Function: Fn(Key) -> Val, 
          Key: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display,
          Val: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Display
{
    pub fn new(function: Function) -> Cacher<Function, Key, Val> {
        Cacher {
            function,
            results: HashMap::<Key, Val>::new(),
        }
    }

    pub fn result_for(&mut self, arg: Key) -> Val {
        // better to use: self.results.entry(k).or_insert(v);
        // this is for version with prints:
        if self.results.contains_key(&arg) {
            let val = *self.results.get(&arg).unwrap();
            println!("Cacher: cashed result for {}: {}", arg, val);
            val
        } else {
            println!("Cacher: calculating for {}...", arg);
            let val = (self.function)(arg);
            self.results.insert(arg, val);
            val
        }
        // or just:
        // *self.results.entry(arg).or_insert(
        //     (self.function)(arg)
        // )
    }
}

#[test] fn call_cacher_with_same_input() {
    let mut cacher = Cacher::new(|a| a*a);
    let res1 = cacher.result_for(2);
    let res2 = cacher.result_for(2);
    assert_eq!(res1, 4);
    assert_eq!(res2, 4);
}

// #[test] fn call_cacher_with_different_inputs() {
//     let mut cacher = Cacher::new(|a| a*a);
//     let res1 = cacher.result_for(1);
//     let res2 = cacher.result_for(2);
//     assert_eq!(res1, 1);
//     assert_eq!(res2, 4);
// }