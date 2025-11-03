// last: 11/25

// interv-2

// task:
// Create two node link with cyclic reference issue
// Find the cycle w/ 2 ways, using std lib fn and raw ptrs


// todo: memory leak in stalled tasks/threads, ways to detect and cancel them




// ========================================





use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    v: i32,
    next: Option<Rc<RefCell<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("dropping {}", self.v);
    }
}

#[test]
fn smart_ptrs_cycle() {
    //// 1. Create cycle
    let ptr1 = Rc::new(RefCell::new( Node { next: None, v: 1 } ));
    let ptr2 = Rc::new(RefCell::new( Node { next: Some(ptr1.clone()), v: 2 } ));
    ptr1.borrow_mut().next = Some( ptr2.clone() ); // Cycle! Memory never freed
        // thus, Node::drop() never called


    //// 2. Find cycle

    // a. using Rc::ptr_eq()
    assert!(Rc::ptr_eq(ptr1.borrow().next.as_ref().unwrap(), &ptr2));

    // b. Using raw ptrs
    let binding = ptr1.borrow();
    let ptr1_next = binding.next.as_ref().unwrap().borrow();
    let ptr1_next_node_addr = &*ptr1_next as *const Node; // &*ptr1_next: &Node
    let ptr2_node_addr = &*ptr2.borrow() as *const Node;
    assert_eq!(ptr1_next_node_addr, ptr2_node_addr);
}
