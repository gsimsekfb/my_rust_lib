use std::sync::atomic::{AtomicU64, Ordering};

// The core patterns are: CAS loop, fetch-and-modify, and read-copy-update (RCU) 

#[test]
fn ex1_() {

    /// Problem-1: When the store value depends on load value without extra
    /// condition or branching (if there is, this is Problem-2 below).    
    /// in short when two atomic ops are NOT independent from each other thus 
    /// we will have race condition
    /// the solution is they must be done in one atomic step
    let counter = AtomicU64::new(5);
    let val = counter.load(Ordering::Relaxed);  // thread A reads 5
    // thread B also reads 5 here
    counter.store(val + 1, Ordering::Relaxed);  // both write 6, not 7

    /// Solution: Fetch and modify pattern
    /// Use fetch_add/fetch_sub/fetch_or etc. when new value = 
    /// simple operation on old value
    let counter = AtomicU64::new(5);
    let old_val = counter.fetch_add(1, Ordering::Relaxed); 
        // atomic read+write in one op
        // old_val is the value before increment, guaranteed correct
    assert_eq!(old_val, 5);
    assert_eq!(counter.load(Ordering::Relaxed), 6);


    /// Problem-2: Same race condition problem as Problem-1 
    /// Plus, now we have also - the new value depends on a branch or
    /// condition which is the REASON to use CAS loop pattern.
    let counter = AtomicU64::new(5);
    let current = counter.load(Ordering::Relaxed);
    if current < 100 {  // thread A checks: 99 < 100, ok
        // thread B also checks: 99 < 100, ok
        counter.store(current + 1, Ordering::Relaxed); 
            // both threads increment, oversold 101st seat
    }

    /// Solution: Load and CAS Loop pattern
    /// !! instead of blocking other threads like a mutex, CAS lets all threads
    ///    try simultaneously and only one succeeds, while the others retry 
    ///    in the loop.
    let counter = AtomicU64::new(5);
    // Step-1: Load
    let mut current = counter.load(Ordering::Acquire);
        // !! no code here, the pattern is load and loop
    loop { // threads loop here until they get OK from compare_exchange()

        // Step-2
        // We need to do things which depend on the value of current
        // variable
        // e.g. deciding the new value of counter:
        let new_val = if current > 0 { current + 1 } else { 0 };
    
        // Step-3: "Try" to update counter
        // !! compare_exchange: stores a value into the atomic integer if the
        //    current value is the same as the value of `current` param.
        match counter.compare_exchange(
            current, new_val, Ordering::AcqRel, Ordering::Acquire
        ) {
            Ok(prev_val) => { // we won, value updated
                dbg!(current);  // 5 (mut)
                dbg!(prev_val); // 5
                dbg!(new_val);  // 6
                break
            }
            // other thread changed it, retry with new `current` value
            Err(new_current) => { // new_current_val_other_thread_wrote 
                dbg!(new_current);
                current = new_current; 
            }
        }
    }
    assert_eq!(counter.load(Ordering::Relaxed), 6);



    /// Problem-3: 
    /// Need to update a complex data structure (like a config) that many
    /// threads read frequently, without blocking readers.
    /// 
    /// When to use:
    /// read-heavy shared data that rarely changes (config, routing tables, 
    /// feature flags) — readers never block, writer pays the copy cost.

    // Solution: read-copy-update (RCU) using ArcSwap pattern
    struct Config { workers: u8, x: u8 }
    let config = arc_swap::ArcSwap::from_pointee(Config { workers: 2, x: 42 });

    // readers — zero locking
    let current = config.load();
    assert_eq!(config.load().workers, 2);

    // writer — copy, modify, swap
    // !! This pattern assumes there's only one writer (e.g. a config reload
    //    triggered by a signal or admin endpoint) and losing a concurrent 
    //    write is either impossible or acceptable.
    let old = config.load_full();
    let new_config = Config { workers: 8, ..*old };
    config.store(std::sync::Arc::new(new_config));

        // load() vs load_full()
        // - `load` returns a guard that borrows the current value (cheap, 
        //    no clone), 
        // - `load_full` returns an owned `Arc` (clones the pointer) 
        // - use `load` for readers, `load_full` for writers who need to
        //   copy-modify-swap.
    
    assert_eq!(config.load().workers, 8);

}

// any other important/popular lock free programming patterns ? 
/*  
Two more worth knowing: **hazard pointers** (safe memory reclamation for lock-free data structures) and **epoch-based reclamation** (used by `crossbeam` and `DashMap` internally) — but in practice for Rust you "rarely" implement these yourself, you just use `crossbeam` or `DashMap` which handle it internally.
*/