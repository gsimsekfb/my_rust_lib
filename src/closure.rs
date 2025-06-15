
// interv-1
//
// 1. How to return a closure from an fn - 2 ways
// 2. below 
// Hint at the end





// ------------------------------------------------------------





// Solution:

// static version
fn return_closure_w_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// dynamic version
fn return_closure_w_box() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[test] fn ex1() {
    assert_eq!(5, return_closure_w_impl()(4));
    assert_eq!(5, return_closure_w_box()(4));
}

// Hint: There are two ways with impl and with dyn



// ------------------------------------


// interv-1
//
// 2. implement the followings for a closure:
// - immutable burrow capture
// - mutable burrow capture
// - move capture




#[test]
pub fn examples() {
    let color = String::from("green");

    // 1. Immutable burrow capture
    // 
    // 1.a.
    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time. 
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;
    print();


    // 2. Mutable burrow capture
    // 
    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 3;
        println!("`count`: {}", count);
    }; 

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // inc closure no longer called. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 


    // 3. move capture
    // 
    // A non-copy (aka not copied automatically, but moved) type Box
    let movable = Box::new(42);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = move || {  // move is unnecessary here
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.

    // 4. move capture
    //
    // `Vec` has non-copy semantics.
    #[allow(clippy::useless_vec)]
    let vec = vec![1, 2, 3];
    let contains = |elem| vec.contains(elem);

    assert!(contains(&1));
    assert!(!contains(&4));

    println!("There're {} elements in vec", vec.len());



    // 5. When "move" word is required? 
    // A good rule of thumb: If your closure needs to:
    //
    // Be returned from a function
    // Be moved into a new thread
    // Take complete ownership of captured values
    // Store the closure in a struct that outlives the captured variables


    // ex:
    // When the closure needs to outlive the current scope:
    //
    fn create_closure() -> impl Fn(i32) -> bool {
        let vec = vec![1, 2, 3];
        move |x| vec.contains(&x)  // move is required here
    }

    // ex:
    // When working with threads:
    //
    let text = String::from("hello");
    let thread_handle = std::thread::spawn(move || {
        // `move` is required here because the closure needs to own `text`
        // since it will outlive the current scope
        println!("{}", text);
    });
  
}