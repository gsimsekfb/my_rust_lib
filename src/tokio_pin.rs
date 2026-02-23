
//// tokio::pin!
// 1. Why it's useful: 
// Futures in Rust must be pinned before they can be polled multiple times or
// used with async utilities like timeout, select, or race. 
// Pinning ensures the future won't be moved in memory while being polled.

// 2.
// - it pins futures to the stack (no heap allocation)
// - Required for operations that need &mut Future:
//    timeout
//    select!
//    race
//    Retry logic  

// 3. Ensures memory safety - future can't move while being polled
// 4. More efficient than Box::pin for local futures

use tokio::time::{sleep, timeout, Duration};

async fn fetch_data() -> String {
    sleep(Duration::from_secs(1)).await; // chg to 3 to simulate timeout 
    "data fetched".to_string() // Simulate async work
}

#[tokio::test(flavor = "multi_thread")]
async fn tokio_pin_1_w_timeout() {
    let future = fetch_data();
    
    // Pin the future so we can use it with timeout
    tokio::pin!(future);
        /* 1. Essentially expands to something like:
        let mut future = future;
        let mut future = unsafe { Pin::new_unchecked(&mut future) };
            // Now 'future' is of type Pin<&mut YourFutureType>
        */
        // 2. w/o this line:
        // error[E0277]: `{async fn body of tokio_pin::fetch_data()}` 
        // cannot be unpinned --> src\tokio_pin.rs:19:43
    
    // Apply a timeout to the pinned future
    match timeout(Duration::from_secs(2), &mut future).await {
        Ok(result) => println!("-- Ok: {result}"),
        Err(_) =>     println!("-- Err: Timeout!"),
    }
}

async fn slow_task() -> &'static str {
    sleep(Duration::from_secs(3)).await;
    "Slow task completed"
}

#[tokio::test(flavor = "multi_thread")]
async fn tokio_pin_2_future_reuse_w_select() {

    let future = slow_task();
    
    // ❌ This fails - can't use future multiple times
    // tokio::select! {
    //     result = future => { /* ... */ },
    //     result = future => { /* ... */ }, // Error: use of moved value
    // };
    
    // ✅ Solution: pin it first!
    tokio::pin!(future);

    tokio::select! {
        result = &mut future => { println!("First try: {result}") },
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            println!("Timeout, trying again...");
            // Can reuse the pinned future
            let result = (&mut future).await;
            println!("Second try: {result}");
        }
    };
}

// todo: add streams example
//
// https://tokio.rs/tokio/tutorial/streams -> search:
// A Rust value is "pinned" when it can no longer be moved in memory. A key property of a pinned value is that pointers can be taken to the pinned data and the caller can be confident the pointer stays valid. This feature is used by async/await to support borrowing data across .await points.
// 
// async fn subscribe() -> mini_redis::Result<()> {
//     let client = client::connect("127.0.0.1:6379").await?;
//     let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
//     let messages = subscriber.into_stream();

//     tokio::pin!(messages);

//     while let Some(msg) = messages.next().await {
//         println!("got = {:?}", msg);
//     }

//     Ok(())
// }
