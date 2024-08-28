
// Problem Description
// Remove Nth node of the Linked List
// 
// e.g. f(1-2-3-None, 2th node) ret 1-3-None
//
// https://www.geeksforgeeks.org/remove-nth-node-from-end-of-the-linked-list/


#[derive(Debug, Clone, PartialEq)]
struct N {
    v: i32,
    next: Option<Box<N>>
}

fn new_leaf(v: i32) -> Option<Box<N>> {
    Some(Box::new( N { v, next: None } ))
}

fn new_node(v: i32, n: Option<Box<N>>) -> Option<Box<N>> {
    Some(Box::new( N { v, next: n } ))
}


// Sol 1 - One loop using ptrs
//
// e.g. f(1-2-3-None, 2) ret 1-3-None
//
// Hint: keep track of new list's (head's beg, end and tail's beg)
//
// Time : O(n)
// Space: O(1)
//
// Debug: f(1-2-3-None, 2)
fn delete_nth_node(list: &Option<Box<N>>, index: usize) -> Option<Box<N>> {
    let mut head = None;            // t: mut Option<Box<N>> (obj, not a ptr)
    let mut head_ptr = &mut head;   // mut t: &mut Option<Box<N>>
    let mut tail_ptr = list;        // mut t: &    Option<Box<N>>
        // start with pointing to the first node

    // loop until we find (index)th node, to save next node as tail_ptr
    // also build the head and head_ptr
    for i in 1..index+1 { // deb: i 1,2 ; index 2
        // Build head nodes and advance head_ptr to finally save ptr to
        // last node (None), ir oder to later assign with tail_ptr.
        // New head will end with list's (index-1)th node + None
        if i < index {    // deb: i 1 ; index 2
            *head_ptr = new_leaf(tail_ptr.as_deref().unwrap().v); // new ctr
            head_ptr = &mut head_ptr.as_deref_mut().unwrap().next;
        }
        tail_ptr = &tail_ptr.as_deref().unwrap().next; // go to next node
        // tail ptr None means that, no more node to process, end loop
        if tail_ptr.as_deref().is_none() { break; }
    }
    // dbg!(&head_ptr);  // deb: head: 1-None, head_ptr: &None
    // dbg!(&tail_ptr);  // deb: &(3-None)

    head_ptr.clone_from(tail_ptr); // copy ctr
    // or
    // *head_ptr = tail_ptr.clone(); // copy ctr //  tail_ptr auto derefed?
        // *: deref ptr to access pointed object which is Option<Box<N>>

    head
}

#[test] fn tt() {

    // Sol-1

    // 1-2-3-None
    let list = new_node(1, new_node(2, new_node(3, None)));
    //
    let res = new_node(2, new_node(3, None));
    assert_eq!(delete_nth_node(&list, 1), res);
    //
    let res = new_node(1, new_node(3, None));
    assert_eq!(delete_nth_node(&list, 2), res);
    //
    let res = new_node(1, new_node(2, None));
    assert_eq!(delete_nth_node(&list, 3), res);
    //
    let res = &list;
    assert_eq!(&delete_nth_node(&list, 4), res);

    // 1-None
    let list = new_node(1, None);
    //
    let res = None;
    assert_eq!(delete_nth_node(&list, 1), res);

    // 1-2-None
    let list = new_node(1, new_node(2, None));
    //
    let res = new_node(2, None);
    assert_eq!(delete_nth_node(&list, 1), res);
    //
    let res = new_node(1, None);
    assert_eq!(delete_nth_node(&list, 2), res);
}
