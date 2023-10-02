use rayon::prelude::*;
use crate::utils::pp;

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

#[test] fn ex1() {
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
//// Lesson: Parallel exec. is more meaningful when we have bigger arrays
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
