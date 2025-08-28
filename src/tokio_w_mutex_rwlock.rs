use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex, RwLock}};

#[tokio::test(flavor = "multi_thread")]
async fn tokio_mutex() {

    // 1. Mutex
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let peers_clone = peers.clone();
    let task_1 = tokio::spawn(async move {
        println!("-- task_1 running");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        let mut peers = peers_clone.lock().unwrap();
        peers.insert("aa", 11);
        println!("-- task_2 finished");
    });
    let _ = task_1.await; // !! this is a blocking wait for this/main task 
                          // but not blocking for rt/program.
                          // rt can work on other tasks scheduled outside of 
                          // this/main task

    // 2. RwLock
    let list = Arc::new(RwLock::new(HashMap::new()));
    let list_clone = list.clone();
    let task_2 = tokio::spawn(async move {
        println!("-- task_2 running");
        let mut list = list_clone.write().unwrap();
        list.insert("ww", 99);
        println!("-- task_2 finished");
    });
    let _ = task_2.await;


    // Mutex
    {
        let peers = peers.lock();
        dbg!(&peers);
    }

    // RwLock
    {
        let list = list.read().unwrap();
        dbg!(&list);
    }
}
