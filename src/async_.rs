
// Book
// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html

// See also: src/web-server-async-and-parallel/src/main.rs

// 0. What is Async? 
// Asynchronous programming is a concurrent programming model.
// It lets you run a large number of concurrent tasks on a small number of 
// OS threads, through the async/await syntax.
// https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

// 1. Async and Await
/*
/// async 
defines an async code block or function. Specifying that it 
will return a Future, a value that will need to be .awaited 
elsewhere in order to trigger the execution of the task 
(lazy execution) and wait for the return value to be available.

/// .await
.await keyword (which must be inside an async block/function) is used 
to wait asynchronously for an async task to finish, and get the return 
value. Note that while the task itself cannot progress until the Future 
is ready, the actual OS thread can have other tasks assigned to it by the 
runtime, and so continue to do work.
*/

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 { 5 }
    // async bodies and other futures are lazy: they do nothing until they
    // are run/awaited. The most common way to run a Future is to .await it.


// 2. Async vs threads in Rust
// https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html#async-vs-threads-in-rust
// On a last note, asynchronous programming is not better than threads, 
// but different. If you don't need async for performance reasons, 
// threads can often be the simpler alternative.

// a. Thread
// downloading a web page is a small task; creating a thread for such a small
// amount of work is quite wasteful.
// fn get_two_sites() {
//     // Spawn two threads to do work.
//     let thread_one = std::thread::spawn(|| download("https://www.foo.com"));
//     let thread_two = std::thread::spawn(|| download("https://www.bar.com"));

//     // Wait for both threads to complete.
//     thread_one.join().expect("thread one panicked");
//     thread_two.join().expect("thread two panicked");
// }

// b. Async
// Here, no extra threads are created. Additionally, all function calls are 
// statically dispatched, and there are no heap allocations! 
// However, we need to write the code to be asynchronous in the first place,
// async fn get_two_sites_async() {
//     // Create two different "futures" which, when run to completion,
//     // will asynchronously download the webpages.
//     let future_one = download_async("https://www.foo.com");
//     let future_two = download_async("https://www.bar.com");

//     // Run both futures to completion at the same time.
//     std::future::join!(future_one, future_two);
// }


// 3. async await basic example
async fn learn_song() { for i in 1..=3 { println!("--- learning song... {}", i); } }

async fn sing() { for i in 1..=3 { println!("--- singing... {}", i); } }

// 3.a. await - wait/block one future
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    learn_song().await; // block  
    // learn_song(); // note: futures do nothing unless you `.await` or poll them
    sing().await;
}

async fn dance() {
    for i in 1..=3 { println!("--- dancing... {}", i); }
}

// 3.b. join - wait/block multiple futures concurrently
async fn async_main() {
    // Create futures
    let learn_and_sing = learn_and_sing();
    let dance = dance();

    // `join!` is like `.await` (which blocks/waits one future) but can wait 
    // for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(learn_and_sing, dance); // await/block multiple futures concurrently
}

#[test] fn ex1() {
    // get_two_sites();
    futures::executor::block_on(async_main());
        // `block_on` blocks the current thread until the provided future has run to
        // completion. Other executors provide more complex behavior, like scheduling
        // multiple futures onto the same thread
}

// 3.c. try_join - use it for futures which return Result
use futures::try_join;

async fn get_book() -> Result<i32, String> { /* ... */ Ok(42) }
async fn get_music() -> Result<i32, String> { /* ... */ Ok(43) }

async fn get_book_and_music() -> Result<(i32, i32), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}


// 4. tokio
// https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
// todo: downloaded image cannot be opened
async fn download() {
    let url = "https://i.imgur.com/CzXTtJV.jpg";
    let response = reqwest::get(url).await.expect("request failed");
    let mut dest = std::fs::File::create("cat.jpg")
        .expect("failed to create file");

    let content =  response.text().await.unwrap();
    std::io::copy(&mut content.as_bytes(), &mut dest)
        .expect("failed to copy content");
}

#[tokio::test]
async fn tokio_1() {
    download().await;
}