use rand::Rng;

fn random_num() -> u64 { rand::rng().random_range(1..=10) }

async fn sleep_ms(millis: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
}

async fn f1() -> u64 {
    let ms = random_num()*100;
    println!("11  f1 started w/ ms: {ms}");
    sleep_ms(ms).await; 
    ms
}

async fn f2() -> u64 {
    let ms = random_num()*100;
    println!("22  f2 started w/ ms: {ms}");
    sleep_ms(ms).await; 
    ms
}


#[tokio::main]
async fn main() {
    let mut cntr = 0;
    loop {
        cntr += 1;
        println!("====== loop {cntr}");
    
        tokio::select! {
            ms = f1() => { println!("11  f1() completed in {ms} ms") }
            ms = f2() => { println!("22  f2() completed in {ms} ms") }
        };
        println!("== loop {cntr} ends\n");
    }
}

/* 
Short Explanation: 
- Both f1() and f2() are started simultaneously (they are "spawned" as async tasks under the hood).
- The select! macro waits for whichever future completes first, other one dropped
- e.g. if f2 completes first, f1() will be dropped 

*/

/* 
stdout:

====== loop 1
22  f2 started w/ ms: 200
11  f1 started w/ ms: 700
22  f2() completed in 200 ms    // f2 won the race to complete first
                                // !! f1() dropped
== loop 1 ends

====== loop 2
22  f2 started w/ ms: 500
11  f1 started w/ ms: 100
11  f1() completed in 100 ms
== loop 2 ends

====== loop 3
11  f1 started w/ ms: 800
22  f2 started w/ ms: 1000
11  f1() completed in 800 ms
== loop 3 ends

====== loop 4
22  f2 started w/ ms: 700
11  f1 started w/ ms: 300
11  f1() completed in 300 ms
== loop 4 ends
*/


/* 
Step-by-Step Execution:

1. Initialization:

- The loop starts a new iteration.
- tokio::select! evaluates all futures (f1() and f2()) and begins polling them concurrently.

2. Polling:

- Both f1() and f2() are started simultaneously (they are "spawned" as async tasks under the hood).
- The select! macro waits for whichever future completes first.

3. Completion:

a) If f1() completes first:
    The x = f1() branch is taken.
    The result is bound to x.
    println!("Operation 1 completed first with: {}", x) executes.
    The other future (f2()) is dropped (its work is discarded).

b) If f2() completes first:
    The y = f2() branch is taken.
    The result is bound to y.
    println!("Operation 2 completed first with: {}", y) executes.
    The other future (f1()) is dropped.

4. Next Iteration:

The loop restarts, and select! runs again, re-evaluating f1() and f2() from scratch (since they were dropped after the previous iteration).
*/