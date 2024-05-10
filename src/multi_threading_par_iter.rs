use rayon::{prelude::*, vec};
use crate::utils::pp;
use std::collections::HashSet;
use std::sync::Mutex;

#[test] fn ex1_adv_modify_ext_data() {
    const ARR_SIZE: usize = 10_000;
    // a. Modifying external data in parallel w/ Mutex
    // let arr = [1,2,4,3];
    let arr: [usize; ARR_SIZE] = core::array::from_fn(|i| i + 1);
    let vv: Mutex<Vec<_>> = Mutex::new(vec![]);
    let now = std::time::Instant::now();
    arr.par_iter().for_each(|item| {
        let mut vv = vv.lock().unwrap(); // MutexGuard<T>
        vv.push(item * 10);
    });
    println!("(a.1) {:.2?}", now.elapsed());
    let vv = vv.into_inner().unwrap();  // T
        // !!! vv will be unordered
    // Sort and compare unordered vectors
    let now = std::time::Instant::now();
    let vv: HashSet<_> = vv.into_iter().collect();
    let ww: [usize; ARR_SIZE] = core::array::from_fn(|i| (i+1) * 10);
    let ww: HashSet<_> = ww.into_iter().collect();
    assert_eq!(vv, ww);
    println!("(a.2) {:.2?}", now.elapsed());
    // or
    let now = std::time::Instant::now();
    use assert_unordered::assert_eq_unordered;
    assert_eq_unordered!(vv, ww);
    println!("(a.3) {:.2?}", now.elapsed());

    // b. Same op. w/o using Mutex - we have error
    // error[E0596]: cannot borrow `vv` as mutable, 
    // as it is a captured variable in a `Fn` closure
    // let mut vv: Vec<String> = vec![];
    // [1,2,3].par_iter().for_each(|item| {
    //     vv.push(item.to_string());
    // });
    // println!("vv {:?}", vv);
}

fn parallel_exec(arr: &mut [i32]) {
    let start_time = chrono::Utc::now().time();
    arr.par_iter_mut().for_each(|p| *p -= 1);
    let end_time = chrono::Utc::now().time();
    let diff = (end_time - start_time).num_microseconds().unwrap();
    println!("  (a) Parallel exec: {} us", pp(diff));
}

fn single_thread_exec(arr: &mut [i32]) {
    let start_time = chrono::Utc::now().time();
    arr.iter_mut().for_each(|p| *p -= 1);
    let end_time = chrono::Utc::now().time();
    let diff = (end_time - start_time).num_microseconds().unwrap();
    println!("  (b) Single thread: {} us", pp(diff));
}

#[test] fn ex2_simple() {
    const SIZE_S: usize = 5_000;
    println!("(1) Arr size: {}", pp(SIZE_S));
    let mut small_arr: [i32; SIZE_S] = [100; SIZE_S];
    parallel_exec(&mut small_arr);
    single_thread_exec(&mut small_arr);

    println!("-----");

    const SIZE_B: usize = 5_000;
    println!("(2) Arr size: {}", pp(SIZE_B));
    let mut small_arr: [i32; SIZE_B] = [100; SIZE_B];
    parallel_exec(&mut small_arr);
    single_thread_exec(&mut small_arr);

    println!("-----");

    const SIZE_C: usize = 50_000;
    println!("(3) Arr size: {}", pp(SIZE_C));
    let mut big_arr: [i32; SIZE_C] = [100; SIZE_C];
    parallel_exec(&mut big_arr);
    single_thread_exec(&mut big_arr);

    assert_eq!(32,32);
}
//// Important: Parallel exec. is more meaningful when we have bigger arrays/tasks
//// StdOut:
// (1) Arr size: 5_000
//   (a) Parallel exec: 636 us
//   (b) Single thread: 286 us
// -----
// (2) Arr size: 5_000
//   (a) Parallel exec: 269 us
//   (b) Single thread: 283 us
// -----
// (3) Arr size: 50_000
//   (a) Parallel exec: 771 us
//   (b) Single thread: 1_953 us
