
#[tokio::test(flavor = "multi_thread")]
async fn async_closure_1() {
    // Async closure - returns Future that must be awaited

    // a. move capture: copy type
    let capture = 40; // capture variable
    let async_closure = |x: i32| async move { x * capture };
        // dbg!(capture); // Ok: capture copied
    let future = async_closure(200); // impl Future<Output = i32>
    let result = future.await;       // i32
        // dbg!(capture); // Ok

    // b. move capture: non-copy type
    let capture = String::from("ab"); // capture variable
    let async_closure = |x: String| async move { capture + &x };
        // dbg!(capture);  // err: moved value
    let future = async_closure("-de".to_string()); // impl Future<Output = String>
    let result = future.await;       // String

    // For comparison: 
    // Async fn
    async fn f1() -> u8 { 5 }
    let future = f1();      // impl Future<Output = i32>
    let res = future.await;

    assert_eq!(res, 5); 
}
