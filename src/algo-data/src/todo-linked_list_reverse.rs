// Problem Description
// Reverse a Linked List
// e.g. 1-2-3 -> 3-2-1
//
// https://www.geeksforgeeks.org/reverse-a-linked-list/

// Sol 1 - 
// Hint:



#[test] fn ww() {
    // Sol-1

    // Sol-2
    assert_eq!(42, 42);
}

// todo
// use std::fmt::{Formatter, Result};
// impl std::fmt::Debug for Node {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         // todo
//         Ok(())
//     }
// }

#[derive(Debug, Clone)]
struct Node {
    v: i32, 
    next: Option<Box<Node>>
}

impl Node {
    fn new_null(v: i32) -> Self {
        Self { v, next: None }
    }
    fn new(v: i32, n: Node) -> Self {
        Self { v, next: Some(Box::new(n)) }
    }
}

// todo
// Sol-1: Iter
//
// Node* reverseList(Node* head) {
//   // Initialize three pointers: curr, prev and next
//   Node *curr = head, *prev = nullptr, *next;

//   // Traverse all the nodes of Linked List
//   while (curr != nullptr) {  // c: 1-2-3 , 2-3 , 3
    
//       // Store next
//       next = curr->next;     // n: 2-3   , 3   , null 
    
//       // Reverse current node's next pointer
//       curr->next = prev;     // c: 1     , 2-1 , 3-2-1
 
//       // Move pointers one position ahead
//       prev = curr;           // p: 1     , 2-1  , 3-2-1
//       curr = next;           // c: 2-3   , 3    , null
//   }
//     // Return the head of reversed linked list
//   return prev;               // p: 3-2-1
// }

fn reverse(n: &Node ) -> Node {
    let mut curr = Some(Box::new(n.clone()));
    let mut prev = None;
    let mut next;
    while curr.is_some() {
        next = curr.clone();
        curr.as_mut().unwrap().next = prev.clone();
        prev = curr.clone();
        curr = next.clone();
    }
    return *prev.unwrap();
}
// Debug
// fn(n: 1-2-3-null)
// rev = 1-2-3
// while 
//  r=1     , rev = 2-3 ,
//  r=2-1   , rev = 3
//  r=3-2-1 ,
// ret 3-2-1-null



// todo
// Sol-2: Recursive
//
// Node* reverseList(Node* head) {
//     if (head == NULL || head->next == NULL)
//         return head;

//     // reverse the rest of linked list and put the first element at the end 
//     Node* rest = reverseList(head->next);
  
//       // Make the current head as last node of remaining linked list
//     head->next->next = head;

//       // Update next of current head to NULL
//     head->next = NULL;

//        // Return the reversed linked list
//     return rest;
// }
// Debug
// f(1-2-3)
// rest = f(2-3);
//   f(2-3)
//   rest = f(3)
//     f(3)
//     ret null
//   rest = null
//   head = 2-3-2-3
//   head = 2
//   ret 3
// rest = 3
// head = 1-2-1-2
// head = 1
// rest = 3


// ret n( 3-2-1-null )
//   fn(2-3-null)
//   head = 3-null; head.next = 2; 
//   ret (3-2-null)
//     fn(n: 3-null)
//     ret n(3-null)

fn reverse_recur(n: &Node ) -> Node {
    if n.next.is_none() { return Node::new_null(n.v); }
    let mut head = reverse_recur(&*n.next.as_ref().unwrap());
    head.next = Some(Box::new(Node::new_null(n.v)));
    return head;
}
// Debug
// fn(n: 1-2-3-null)
// head = 3-2-null; 
// ret n( 3-2-1-null )
//   fn(2-3-null)
//   head = 3-null; head.next = 2; 
//   ret (3-2-null)
//     fn(n: 3-null)
//     ret n(3-null)


fn main() {

    // 1 -> 2 -> null
    let n = Node::new(1, Node::new(2, Node::new_null(3)));
    // let n = Node::new(1, Node::new_null(2));
    // let n = Node::new_null(2);
    println!("{:?}", n);
    let n_reversed = reverse(&n);
    println!("{:?}", n_reversed);


    // rr(&mat);

    // let res = count_set_bits_recur(13);
    // println!("{res}");
    



    // println!("{}");
    // println!("xx: {:?}", xx);
}
