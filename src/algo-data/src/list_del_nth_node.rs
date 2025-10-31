// last: 9/25

// Problem Description
// Remove Nth node of the Linked List
// 
// e.g. f(1-2-3-None, 2nd node) ret 1-3-None
//
// https://www.geeksforgeeks.org/remove-nth-node-from-end-of-the-linked-list/


#[derive(Debug, PartialEq)]
struct L {
    val: u8,
    next: Option<Box<L>>
}

type List = Option<Box<L>>;

// See notes in fn
// impl Drop for L {
//     fn drop(&mut self) {
//         println!("-- dropping {}", self.val);
//     }
// }

fn new(v: u8, n: List) -> List { Some(Box::new(L { val: v, next: n })) }

fn new_leaf(v: u8) -> List { new(v, None) }

fn traverse_list_for_loop(list: &List) {
    let mut current = list;
    while let Some(node) = current {
        println!("tll -- {}", node.val);
        current = &node.next
    }
}

fn traverse_list_recur(list: &List) {
    let Some(node) = list else { return; };
    println!("tlr >> {}", node.val);
    traverse_list_recur(&node.next);
}


// Sol
// Hint at the end
//
// Time : O(n)
// Space: O(1)
//
// index: first elem's index is 1 (not 0)
// e.g. f(1-2-3-None, 2) ret 1-3-None
fn delete_nth_node(list: &mut Option<Box<L>>, index: u8) {
    if index == 1 { // edge case: delete 1th node
        if let Some(first_node) = list.take() {
            *list = first_node.next;
            println!("-- deleted node {}", first_node.val);
        }
        return;
    }
    // all other cases
    let mut current = list;
    let mut i = 1;
    while let Some(node) = current {
        // debug for: index 2, i=1, take node_1.next (node_2) out and assign node_3 instead
        if i + 1 == index && let Some(node_next) = node.next.take() { 
                                        // take node_next (node 2) out of node 1
            // node_next.next = None; // Todo: how come this is error but next line is Ok
                                        // we can take out of something from node_next but 
                                        // we cannot modify it ?
            node.next = node_next.next; // node_1.next = (2.next which is node 3)
                // Note: partial move: move node_next.next (node 3) to node_1.next
                // - node_next(node_2).next which is node_3 moved to node_1.next
                // - node_next ( node 2, Box<L>) dropped at the end of this                    
            // dbg!(&node_next);
                // Note: error[E0382]: borrow of partially moved value: `node_next`

            println!("-- deleting node {}", node_next.val);
            return;
        }
        dbg!(node.val);
        current = &mut node.next; // next node
        i += 1;
    }
}

#[test] fn tt() {
    let list = new(1, new(2, new_leaf(3)));
    traverse_list_for_loop(&list);
    // traverse_list_recur(&list);


    let mut list_1_2_3 = new(1, new(2, new_leaf(3)));
    delete_nth_node(&mut list_1_2_3, 2);
    let list_1_3 = new(1, new_leaf(3));
    assert_eq!(list_1_2_3, list_1_3);

    let mut list_1_2_3 = new(1, new(2, new_leaf(3)));
    delete_nth_node(&mut list_1_2_3, 3);
    let list_1_2 = new(1, new_leaf(2));
    assert_eq!(list_1_2_3, list_1_2);

    let mut list_1_2_3 = new(1, new(2, new_leaf(3)));
    delete_nth_node(&mut list_1_2_3, 1);
    let list_2_3 = new(2, new_leaf(3));
    assert_eq!(list_1_2_3, list_2_3);

    let mut list_1_2_3_4_5 = new(1, new(2, new(3, new(4, new_leaf(5)))));
    delete_nth_node(&mut list_1_2_3_4_5, 3);
    let list_1_2_4_5 = new(1, new(2, new(4, new_leaf(5))));
    assert_eq!(list_1_2_3_4_5, list_1_2_4_5);
    
}
