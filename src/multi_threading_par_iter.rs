use rayon::{prelude::*, vec};
use crate::utils::pp;
use std::collections::HashSet;
use std::sync::Mutex;

fn single_thread_exec(arr: &mut [i32]) {
    let start_time = chrono::Utc::now().time();
    arr.iter_mut().for_each(|p| *p -= 1);
    let end_time = chrono::Utc::now().time();
    let diff = (end_time - start_time).num_microseconds().unwrap();
    println!("  (b) Single thread: {} us", pp(diff));
}

// !! only diff is par_iter_mut instead of iter_mut
fn parallel_exec(arr: &mut [i32]) {
    let start_time = chrono::Utc::now().time();
    arr.par_iter_mut().for_each(|p| *p -= 1);
    let end_time = chrono::Utc::now().time();
    let diff = (end_time - start_time).num_microseconds().unwrap();
    println!("  (a) Parallel exec: {} us", pp(diff));
}

#[test] fn ex1_simple() {

    // warmup run
    const SIZE_S: usize = 5_000;
    println!("(1) Arr size: {}", pp(SIZE_S));
    let mut small_arr: [i32; SIZE_S] = [100; SIZE_S];
    parallel_exec(&mut small_arr);
    single_thread_exec(&mut small_arr);

    println!("-----");

    // second run
    const SIZE_B: usize = 5_000;
    println!("(2) Arr size: {}", pp(SIZE_B));
    let mut small_arr: [i32; SIZE_B] = [100; SIZE_B];
    parallel_exec(&mut small_arr);
    single_thread_exec(&mut small_arr);

    println!("-----");

    // third run, different size
    const SIZE_C: usize = 50_000;
    println!("(3) Arr size: {}", pp(SIZE_C));
    let mut big_arr: [i32; SIZE_C] = [100; SIZE_C];
    parallel_exec(&mut big_arr);
    single_thread_exec(&mut big_arr);

    assert_eq!(32,32);
}
//// Important: Parallel exec. is more meaningful when we have bigger arrays/tasks
//// For small arr size, single thread is faster !!
////
// StdOut:
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


// ----------------------------------------------------------


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {        
        ::core::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::core::mem::size_of::<T>(),
        )
    }
}

// See ex1 for simple example
#[test]
fn ex2_advanced_modify_ext_data() {
    const ARR_SIZE: usize = 10_000;
    // const ARR_SIZE: usize = 3;
    println!("ARRAY_SIZE: {ARR_SIZE}");

    // a.1. Modifying external data in parallel w/ Mutex
    // arr = [1,2,4,3 etc];
    let arr: [usize; ARR_SIZE] = core::array::from_fn(|e| e + 1);
    let vec: Mutex<Vec<_>> = Mutex::new(vec![]);
    let now = std::time::Instant::now();
    arr.par_iter().for_each(|item| {
        let mut v = vec.lock().unwrap(); // MutexGuard<T>
        v.push(item * 10);
    });
    println!("(a.1) Fill vec in par w/ mutex: {:.2?}", now.elapsed());
    let mut vec = vec.into_inner().unwrap();  // T
    // !!! vec will be unordered, to assert we have to sort it
    // vec: [10, 20, 30] or [30, 10, 20] etc
    vec.sort();
    assert_eq!(vec, core::array::from_fn::<_, ARR_SIZE, _>(|e| (e+1)*10));

    // a.2 Create external data in parallel w/o Mutex
    // arr = [1,2,4,3 etc];
    let arr: [usize; ARR_SIZE] = core::array::from_fn(|e| e + 1);
    let now = std::time::Instant::now();
    let mut vec: Vec<_> = arr.par_iter().map(|e| e * 10).collect();
    println!("(a.1) Fill vec in par w/o mutex: {:.2?}", now.elapsed());
    // !!! vec will be unordered, to assert we have to sort it
    // vec: [10, 20, 30] or [30, 10, 20] etc
    vec.sort();
    assert_eq!(vec, core::array::from_fn::<_, ARR_SIZE, _>(|e| (e+1)*10));
    
    // a.3: Sort and compare 2 unordered vectors
    let arr: [usize; ARR_SIZE] = core::array::from_fn(|e| (e+1) * 10);
    let now = std::time::Instant::now();
    let vec_sorted: HashSet<_> = vec.iter().collect();
    // todo: cannot call non-const fn `std::array::from_fn
    // const ARR: [usize; ARR_SIZE] = core::array::from_fn(|i| (i+1) * 10);
    let arr_sorted: HashSet<_> = arr.iter().collect();
    assert_eq!(vec_sorted, arr_sorted);
    println!("(a.2) sort & compare using HashSet: {:.2?}", now.elapsed());

    // or
    let now = std::time::Instant::now();
    use assert_unordered::assert_eq_unordered;
    assert_eq_unordered!(
        TryInto::<[usize;ARR_SIZE]>::try_into(vec).unwrap(), arr
    );
    println!("(a.3) Using assert_eq_unordered!(): {:.2?}", now.elapsed());

    // or ... 
    // arr = [1,2,4,3 etc];
    let mut arr1: [usize; ARR_SIZE] = core::array::from_fn(|i| i + 1);
    let mut arr2: [usize; ARR_SIZE] = core::array::from_fn(|i| i + 1);
    arr2.reverse();
    let now = std::time::Instant::now();
    arr1.sort();
    arr2.sort();
    let arr1_bytes: &[u8] = unsafe { any_as_u8_slice(&arr1) };
    let arr2_bytes: &[u8] = unsafe { any_as_u8_slice(&arr2) };
    assert_eq!(arr1_bytes, arr2_bytes);
    println!("(a.4) sort & compare bytes {:.2?}", now.elapsed());
        // ARRAY_SIZE: 10000
        // (a.1) Fill vec in par: 5.99ms
        // (a.2) sort & compare using HashSet: 10.30ms
        // (a.3) Using assert_eq_unordered!(): 253.97ms
        // (a.4) sort and compare bytes 175.89µs

    // b. Same op. w/o using Mutex - we have error
    // error[E0596]: cannot borrow `vv` as mutable, 
    // as it is a captured variable in a `Fn` closure
    // let mut vv: Vec<String> = vec![];
    // [1,2,3].par_iter().for_each(|item| {
    //     vv.push(item.to_string());
    // });
    // println!("vv {:?}", vv);
}
