// -------------------------------------------------------


use std::thread;
use std::time::Duration;

// 0. Most popular/realistic multi-threading usage 

#[test]
fn ex0_most_real_life_usage() {
    #[derive(Debug)]
    struct Foo { x: i32 }

    use std::sync::{Arc, Mutex};
    use std::{thread, vec};


    // a. Modify vector in a worker thread

    let data = Arc::new(Mutex::new(vec![Foo { x: 0 }]));

    let handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut lock = data.lock().unwrap();  // MutexGuard<'_, Vec<Foo>>
            lock[0].x = 22;
        }
    });
    let _ = handle.join();


    // - Showing how to create/solve deadlock
    {   // !!! deadlock: w/o this braces it will be deadlock,
        // because MutexGuard created by the next line &*data.lock().unwrap() 
        // will release the lock only after the end of main
        assert!(!data.is_poisoned());
        let vec = &*data.lock().unwrap();   // & Vec<Foo>
            // !!! This creates a MutexGuard that lives for the entire scope 
            // - until the end of its block
        assert_eq!(vec[0].x, 22);
    } // Lock released here, good, there will not be deadlock

    // ====

    // b. Modify vector in a second, ""panicking" worker thread hence poisoning

    let handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut lock = data.lock().unwrap();  // MutexGuard<'_, Vec<Foo>>
            lock[0].x = 33;
            panic!("-- panicked while holding the lock! \n"); // !!! lock poisoned
        }
    });
    let _result = handle.join();
    // let _ = handle.join().unwrap(); // To force main thread panic if child panics

    dbg!(&data);
        /* 
            [src\main.rs:65:5] &data = Mutex {
                data: [ Foo { x: 33 } ],
                poisoned: true,
                ..
            }
        */

    {
        assert!(data.is_poisoned()); // 3. Poisoned
        // let vec = &*data.lock().unwrap();  // & Vec<Foo>
            // if lock poisoned errors with: 
            // called `Result::unwrap()` on an `Err` value: PoisonError { .. }
        // assert_eq!(vec[0].x,33); // 
    }

    // c. Recover read/write poisoned data
    match data.lock() {
        Ok(lock) => println!("-- lock acquired normally: {:?}", *lock),
        Err(error) => {
            println!("-- lock poisoned, value: {:?}", *error.into_inner())
              // -- lock poisoned, value: [Foo { x: 33 }]
        }
    };
}


// 1. Not waiting for Spawned thread to finish

#[test]
fn ex1_spawn_thread_no_join() {
  thread::spawn(|| {
    for i in 1..=10 {
      println!("--ST: {i}");  // Spawned thread
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..=5 {
    println!("******** MT: {i}"); // Main thread
    thread::sleep(Duration::from_millis(1));
  }

  // st.join().unwrap();
    // No join hence not waiting for Spawned thread
    // to finish its task. Main thread will print 
    // 5 times, but spawned is not guaranteed to 
    // print 10 times.
}

// -------------------------------------------------------

// 2. Using join - waiting for Spawned thread to finish

#[test]
fn ex2_spawn_thread_w_join() {
  let st = thread::spawn(|| {
    for i in 1..=10 {
      println!("--ST: {i}");  // Spawned thread
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..=5 {
    println!("******** MT: {i}"); // Main thread
    thread::sleep(Duration::from_millis(1));
  }

  st.join().unwrap(); // Wait until st finishes
}

// -------------------------------------------------------

// 3. Using move Closures with Threads

#[test]
fn ex3_1_spawn_thread_w_join() {
  let vv = vec![1, 2, 3];
  let st = thread::spawn(move || {
    println!("*** SP: vv is: {vv:?}");
  });
  st.join().unwrap(); // block mt and wait for st
}

// -------------------------------------------------------

// 4. Message Passing to Transfer Data Between Threads

use std::sync::mpsc; // Multi-producer, single-consume
  // aka multiple sending ends, only one receiving end

#[test]
fn ex4_a_message_passing() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("42");
    tx.send(val).unwrap();
    // println!("val is {}", val); // err: value borrowed here after move 
  });

  let received = rx.recv().unwrap();
    // block the main thread’s execution and wait 
    // until a value is sent down the channel
  println!("*** Rx: Got: {received}");  
}


#[test]
fn ex4_b_multi_message_passing() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("st"),
      ];
      for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
  });

  for received in rx {
    println!("--- Rx got: {received}");
      // --- Rx got: hi
      // --- Rx got: from
      // --- Rx got: the
      // --- Rx got: st
  }
}

#[test]
fn ex4_c_multi_producer() {
  let (tx1, rx) = mpsc::channel();

  // --- tx2
  let tx2 = mpsc::Sender::clone(&tx1);
  thread::spawn(move || {
    let vals = vec![
        String::from("tx2-a"),
        String::from("tx2-b"),
        String::from("tx2-c"),
        String::from("tx2-d"),
      ];
      for val in vals {
        tx2.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
  });

  // --- tx1
  thread::spawn(move || {
    let vals = vec![
        String::from("tx1-xx"),
        String::from("tx1-yy"),
        String::from("tx1-zz"),
        String::from("tx1-ww"),
      ];
      for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
  });  

  for received in rx {
    println!("--- Rx got: {received}");
      // cout: different order each time:
        // --- Rx got: tx2-a
        // --- Rx got: tx1-xx
        // --- Rx got: tx2-b
        // --- Rx got: tx1-yy
        // --- Rx got: tx2-c
        // --- Rx got: tx1-zz
        // --- Rx got: tx1-ww
        // --- Rx got: tx2-d
  }
}

// -------------------------------------------------------

// 5. Using Mutex

use std::sync::{Mutex, Arc};

#[test]
fn ex5_1_mutex_usage() {
  let m = Mutex::new(5);
  {
    let mut num = m.lock().unwrap();
    *num = 6;
    assert_eq!(*num, 6);
  }
}

// Let’s try to share a value between multiple threads using Mutex<T>. We’ll
// spin up 10 threads and have them each increment a counter value by 1, so the
// counter goes from 0 to 10.
#[test]
fn ex5_2_mutex_share_multi_thread() {
  let counter = Arc::new(Mutex::new(0));
  let mut threads = vec![];
  for _ in 0..10 {
    let counter = Arc::clone(&counter);    
    let thread = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    threads.push(thread);
  }
  for thread in threads {
    thread.join().unwrap();
  }
  assert_eq!(*counter.lock().unwrap(), 10);
}