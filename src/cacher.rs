use std::collections::HashMap;

pub struct Cacher<T>
    where T: Fn(u32) -> u32 
{
    calculation: T, // calc. fn
    results: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            results: HashMap::<u32, u32>::new(),
        }
    }

    pub fn result(&mut self, arg: u32) -> u32 {
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
    let mut c = Cacher::new(|a| a);
    let v1 = c.result(1);
    let v2 = c.result(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}