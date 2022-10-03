// -------------------------------------------------------

// 1. Not waiting for Spawned thread to finish

use std::thread;
use std::time::Duration;

#[test]
fn ex1_spawn_thread_no_join() {
  thread::spawn(|| {
    for i in 1..=10 {
      println!("--ST: {}", i);  // Spawned thread
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..=5 {
    println!("******** MT: {}", i); // Main thread
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
      println!("--ST: {}", i);  // Spawned thread
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..=5 {
    println!("******** MT: {}", i); // Main thread
    thread::sleep(Duration::from_millis(1));
  }

  st.join().unwrap(); // Wait until st finishes

  // No join hence not waiting for Spawned thread
  // to finish its task. Main thread will print 
  // 5 times, but spawned is not guaranteed to 
  // print 10 times.
}

// -------------------------------------------------------

// 3. Using move Closures with Threads

#[test]
fn ex3_1_spawn_thread_w_join() {
  let vv = vec![1, 2, 3];
  let st = thread::spawn(move || {
    println!("*** SP: vv is: {:?}", vv);
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
  println!("*** Rx: Got: {}", received);  
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
    println!("--- Rx got: {}", received);
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
    println!("--- Rx got: {}", received);
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