// Also see fp image

#[tokio::test(flavor = "multi_thread")]
async fn tokio_spawn() {
    // 1. tokio::spawn()
    // a. Spawn the task (start to run it) and get its handle
    let handle = tokio::spawn(async move { // handle: JoinHandle<String>
        let data = tokio::fs::read_to_string("test.txt").await.unwrap();
        data // This value is what the JoinHandle will return
    });

    // ... !! Do other work concurrently here while the task is running...

    // Now wait for the specific task to finish and get its result.
    let result: Result<String, tokio::task::JoinError> = handle.await;

    // Handle the result
    match result {
        Ok(data) => {
            // Now you can access `data` from "outside" the task!
            println!("a. File contents: {data}");
        }
        Err(e) => {
            eprintln!("Task failed: {e}");
        }
    }

    // b. Same code simplified w/o unwrap
    let handle = tokio::spawn(async move { // handle: JoinHandle<Result<String, Error>>
        tokio::fs::read_to_string("test.txt").await // Returns a Result
    });

    if let Ok(Ok(data)) = handle.await { 
        println!("b. File contents: {data}"); // Now you have the data
    };
}


#[tokio::test(flavor = "multi_thread")]
async fn tokio_spawn_blocking() {

    // 2. tokio::spawn_blocking()
    // - Offloads a blocking or CPU-intensive synchronous operation to a 
    //   dedicated thread pool so it doesn't stall the main async runtime.
    // - opposite of spawn(), it will block the thread running on aka 
    //   will not yield control back like spawn does in awaits.
    // - so we better use continuous work that we do not want to be interrupted

    let handle = tokio::task::spawn_blocking(|| {
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    });

    if let Ok(res) = handle.await {
        println!("spawn_blocking: res: {res}");
    }
}

