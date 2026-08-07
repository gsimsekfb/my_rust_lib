
#[derive(Debug)]
struct AppState {
    bids: Vec<u32>,
    supply: u32
}

#[test]
fn foo() {
    let mut state = AppState { bids: vec![1,20,3,40], supply: 5000 };
    // assert_eq!(state.supply, 100);

    // Problem: 
    // Mutate bids vector while iterating
    for (i, bid) in &mut state.bids.iter_mut().enumerate() {
        *bid += 100;
        // if *bid > 110 { state.bids.remove(i); }
            // err: "state.bids" already mutable borrowed
        state.supply += 1;  // OK
    }
    assert_eq!(state.bids, &[101, 120, 103, 140]); // no change
    assert_eq!(state.supply, 5004);


    // Option-1: retain - *single pass, good
    let mut state = AppState { bids: vec![1,20,3,40], supply: 5000 };
    state.bids.retain_mut(|bid| {
        *bid += 100;
        if *bid > 110 { return false; } // remove bid what is > 110
        state.supply += 1;
        true // keep
    });
    assert_eq!(state.bids, &[101, 103]);
    assert_eq!(state.supply, 5002); // not 5004


    // Option-2: Reverse iterate to remove from bids vector's end 
    //           *single pass, good
    let mut state = AppState { bids: vec![1,20,3,40], supply: 5000 };

    // add 100 and remove elements bigger than 110
    for i in (0..state.bids.len()).rev() {
        state.bids[i] += 100;
        if state.bids[i] > 110 { state.bids.remove(i); continue; }
        state.supply += 1;  // OK
    }
    assert_eq!(state.bids, &[101, 103]);
    assert_eq!(state.supply, 5002);

    // Option-3: Two pass solution (separate vector), not so good
    //           !! Note: Must "reverse iter" the bids_to_remove vector
    let mut state = AppState { bids: vec![1,20,3,40], supply: 5000 };

    // add 100 and remove elements bigger than 110
    let mut bids_to_remove = vec![];
    for (i, bid) in &mut state.bids.iter_mut().enumerate() {
        *bid += 100;
        if *bid > 110 { bids_to_remove.push(i); continue; }
        state.supply += 1;  // OK
    };
    // !!! rev(): to preserve indices, we must remove from the end of vector
    for bid_index in bids_to_remove.iter().rev() { 
        state.bids.remove(*bid_index);
    };
    assert_eq!(state.bids, &[101, 103]);
    assert_eq!(state.supply, 5002);


    // Option-3: v2: one line bids_to_remove creation using filter_map. 
    //           !! Note: Must "reverse iter" the bids_to_remove vector
    let mut state = AppState { bids: vec![1,20,3,40,5], supply: 5000 };

    // add 100 and remove elements > 110
    let bids_to_remove: Vec<usize> = state.bids.iter_mut()
        .enumerate().filter_map(|(i, bid)| {
            *bid += 100;
            if *bid > 110 { Some(i) } // filters these Some()s
            else { state.supply += 1;  /* OK */ None }
                // or, instead of if else:
                // (*bid > 110).then_some(i) // when no state.supply line
    }).collect();
    // !!! rev(): to preserve indices, we must remove from the end of vector
    for bid_index in bids_to_remove.iter().rev() { 
        state.bids.remove(*bid_index); // remove 120, 140
    };
    assert_eq!(state.bids, &[101, 103, 105]);
    assert_eq!(state.supply, 5003);
}