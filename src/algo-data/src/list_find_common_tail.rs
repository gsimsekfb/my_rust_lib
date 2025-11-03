// Last: 11.25

// Task:
// Given two linked lists, determine if they share a common tail 
// (i.e., they converge at some node and share all subsequent nodes). 
// If they do, return the node where they merge. If not, return null
// Report time and space complexity


// todo: compare w/ cpp impl






// -----------------------------------------------------------------------







use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct List {
    val: i32,
    next: Option<Rc<List>>
}

type NodePtr = Option<Rc<List>>;

impl List {
    fn new(val: i32, next: NodePtr) -> NodePtr {
        Some(Rc::new(List { val, next }))
    }

    fn new_leaf(val: i32) -> NodePtr {
        Some(Rc::new(List { val, next: None }))
    }
}

fn print_list(list: &NodePtr) {
    let Some(node) = list else { println!("None"); return; };
    print!("{} ", node.val);
    let mut current = &node.next;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next;
    }
    println!();
}

fn print_lists(list1: &NodePtr, list2: &NodePtr) {
    print!("l1: "); print_list(list1);
    print!("l2: "); print_list(list2);
}

fn find_len_and_last_node(mut list: &NodePtr) -> (usize, &NodePtr) {
    let mut len = 0_usize;
    while let Some(node) = list {
        len += 1;
        if node.next.is_none() { break; }
        list = &node.next;
    }
    (len, list)
}

fn advance(mut list: &NodePtr, mut steps: usize) -> &NodePtr {
    while let Some(node) = list && steps > 0 {
        list = &node.next;       
        steps -= 1;
    }
    list
}

// O(n+m) time, O(1) space
fn find_common_tail<'a>(
    mut list1: &'a NodePtr,
    mut list2: &'a NodePtr
) -> &'a NodePtr {
    // Step-1: find lens and last nodes. 
    let (len1, last_node_1) = find_len_and_last_node(list1);
    let (len2, last_node_2) = find_len_and_last_node(list2);

    if last_node_1 != last_node_2 { 
        println!("# early return: last nodes differ, exiting..."); 
        return &None
    };

    // Step-2: Make them same len
    if len1 > len2 { list1 = advance(list1, len1-len2); } 
    else {           list2 = advance(list2, len2-len1); }

    // Step-3: Parallel search as they have same len now
    while let Some(node1) = list1 {
        let node2 = list2.as_ref().unwrap();

        if Rc::ptr_eq(node1, node2) { return list1; };
        // or
            // let ptr1 = node1.as_ref() as *const List;
            // let ptr2 = node2.as_ref() as *const List;
            // if ptr1 == ptr2 { return list1; };

        list2 = &list2.as_ref().unwrap().next;
        list1 = &node1.next;
    }

    &None
}

#[test]
fn tests() {
    // l1: 1 3
    // l2: 1 2 3
    let tail = List::new_leaf(3);
    let list1 = List::new(1, tail.clone());
    let list2 = List::new(1, List::new(2, tail));
    print_lists(&list1, &list2);
    let res = find_common_tail(&list1, &list2);
    print!("res: "); print_list(res);
    assert_eq!(res, &List::new_leaf(3));

    println!("--------------");

    // l1: 1 2
    // l2: 1 2 3
    let list1 = List::new(1, List::new_leaf(2));
    let list2 = List::new(1, List::new(2, List::new_leaf(3)));
    print_lists(&list1, &list2);
    let res = find_common_tail(&list1, &list2);
    print!("res: "); print_list(res);
    assert_eq!(res, &None);

    println!("--------------");

    // l1: 1 2 3
    // l2: 1 2 3
    let tail = List::new(1, List::new(2, List::new_leaf(3)));
    let list1 = tail.clone();
    let list2 = tail.clone();
    print_lists(&list1, &list2);
    let res = find_common_tail(&list1, &list2);
    print!("res: "); print_list(res);
    assert_eq!(res, &list1);


    println!("--------------");

    // l1: 3 2 1
    // l2: 1
    let tail = List::new_leaf(1);
    let list1 = List::new(3, List::new(2, tail.clone()));
    let list2 = tail;
    print_lists(&list1, &list2);
    let res = find_common_tail(&list1, &list2);
    print!("res: "); print_list(res);
    assert_eq!(res, &list2);

}
