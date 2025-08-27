
#[tokio::test(flavor = "multi_thread")]
async fn tokio_channels_mpsc() {
    use tokio::sync::mpsc;

    //// 1 producer, 1 consumer
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        for i in 0..2 {
            if let Err(err) = tx.send(i).await {
                println!("receiver dropped, err: {err}");
                return;
            }
        }
    });
    // no need to drop(tx) like the next Multiple producer section
    // because, tx is moved into the spawned task and
    // Once the task ends and tx is automatically dropped and thus channel will
    // be closed

    while let Some(i) = rx.recv().await {
        println!("got = {i}");
    }

    //// Multiple producer, 1 consumer
    let (tx, mut rx) = mpsc::channel(2);

    for i in 0..2 { // spawned producers (2)
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for j in 0..2 { // msgs (2 per producer)
                let msg = format!("Producer {} - Message {}", i+1, j+1);
                if let Err(err) = tx_clone.send(msg).await {
                    println!("receiver dropped");
                    return;
                }
            }
        });
    }

    // This tells the receiver: "No more messages coming"
    // Drop the original transmitter to allow the receiver to close
    drop(tx);
    
    // Consumer
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn tokio_channels_bounded_and_unbounded_channels() {
    use tokio::sync::mpsc;

    // 1. Bounded channel (backpressure when full)
    let (tx, mut rx) = mpsc::channel(10);
    
    tokio::spawn(async move {
        for i in 0..20 {
            // This will block when channel is full
            tx.send(i).await.unwrap();
        }
    });

    while let Some(item) = rx.recv().await {
        println!("Bounded: Received: {}", item);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // 2. Unbounded channel (no backpressure, use with caution)
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // !! no spawn, this is in-task communication
    // for i in 0..1000 {
    //     tx.send(i).unwrap(); // Doesn't block
    // }
    // drop(tx); // This tells the receiver: "No more messages coming"
    // 
    // or
    //
    // !! inter-task communication
    tokio::spawn(async move {
        for i in 0..1000 {
            tx.send(i).unwrap(); // Doesn't block
        }
    });

    while let Some(item) = rx.recv().await {
        println!("Un-Bounded: Received: {}", item);
    }
}

// ========================

#[derive(Debug)]
struct Request { id: u32, data: String }

#[derive(Debug)]
struct Response { status: u32, message: String }

use tokio::sync::oneshot;

async fn worker(rx: oneshot::Receiver<Request>) -> Response {
    Response { status: 42, message: "OK".to_string() }
}

#[tokio::test(flavor = "multi_thread")]
async fn tokio_channels_oneshot() {

    let (tx, rx) = oneshot::channel();
    let worker = tokio::spawn(worker(rx));
    let request = Request { id: 1, data: "ab".to_string() };
    let _ = tx.send(request);

    if let Ok(response) = worker.await {
        println!("-- response: {response:?}");
    }
}

use tokio::time::{timeout, Duration};

async fn worker_2() -> String {
    tokio::time::sleep(Duration::from_millis(200)).await;
    "Work completed".to_string()
}

#[tokio::test(flavor = "multi_thread")]
async fn tokio_channels_oneshot_w_timeout() {

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = worker_2().await;
        let _ = tx.send(res);
    });

    // Wait with timeout
    match timeout(Duration::from_secs(1), rx).await {
        Ok(Ok(resp)) => println!("-- resp: {resp:?}"),
        Ok(Err(err)) => println!("-- Channel closed: err: {err:?}"),
        Err(err) => println!("-- Operation timed out: err: {err:?}"),
    }
}