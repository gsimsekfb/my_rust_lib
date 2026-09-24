// interv-2
// difficulty: easy
// last: 9/26

// Prob:
// Find the Maximum Depth or Height of given Binary Tree
// Note: The height of an empty tree is 0 and 
// the height of a tree with single node is 1.
// https://www.geeksforgeeks.org/find-the-maximum-depth-or-height-of-a-tree/

// Other solutions
// https://www.geeksforgeeks.org/find-the-maximum-depth-or-height-of-a-tree/
// https://medium.com/go-rust/rust-day-10-lc-maximum-depth-of-binary-tree-872b39537716

use std::collections::VecDeque;


#[derive(Debug)]
pub struct Node {
    pub v: i32,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>
}

type Tree = Option<Box<Node>>;

impl Node {
    fn new_leaf(v: i32) -> Tree {
        Self::new(v, None, None)
    }

    fn new(v: i32, l: Tree, r: Tree) -> Tree {
        Some(Box::new( Self { v, l , r } ) )
    }
}

// Iterative BFS version
// 
// !! This version avoids recursion-depth/stack-overflow risk of recursive ver.
//
// - This iterative *BFS version works level by level, 
//   unlike DFS/recursive version which goes deep and then backtracks.
// - *Breadth-first search (BFS) is a graph and tree traversal algorithm 
//   that explores nodes level by level, visiting all neighbors of a node 
//   before moving deeper.
//
//        1        ← level 1
//       / \
//      2   3      ← level 2
//     / \
//    4   5        ← level 3
//
// queue = [1]
// height = 0
// 
// while queue is not empty:
//   >> process queue = [1] // all nodes at this level
//   new queue = [2, 3]     // all nodes at next level
//   height = 1
//   
//   >> process queue = [2, 3]
//   new queue = [4, 5]
//   height = 2
//   
//   >> process queue = [4, 5]
//   new queue = []  // signal to finish the fn
//   height = 3
// 
// just before processing the queue for a level : [2,3]  
// just after processing the queue for a level : [4,5]  
//
// Time Complexity: O(n)
// Space: due to queue having all nodes of a level
//   - Normally: O(w) - w, width of the tree
//   - Worst case: O(n) - at some level, the queue can contain almost all 
//     n nodes, so O(w) → O(n) worst-case space.
fn height_iter(root: &Tree) -> usize {

    if root.is_none() { return 0 };

    // contains total nodes in a level - refilled after each while
    let mut queue = VecDeque::from([root.as_ref().unwrap()]);

    let mut height = 0;

    // process each level - until no queue/children left for a level
    while !queue.is_empty() {
        // process each node in this level, 
        // add (children of each node) into queue for next level processing
        // queue - before for: all nodes in this level 
        //       - after for : all children nodes, next level
        for _ in 0..queue.len() { // Ok: queue.len() evaluated once up front 
                                  // to produce a Range
            // process a node and the queue
            // remove node to be processed now, add children to be processed
            // in following while loop                                 
            let node = queue.pop_front().unwrap(); // ! from front

            if let Some(left) = &node.l {
                queue.push_back(left);                // ! into back
            }
            if let Some(right) = &node.r {
                queue.push_back(right);                 // ! into back
            }
        }

        height += 1;
    }

    height
}

// Recursive version
//
// !! This version has recursion-depth/stack-overflow risk.
// 
// Time Complexity: O(n)
// Space: due to recursion
//   - Normally: O(h) - h, height of the tree ( or O(log n) which is == O(h) )
//   - Worst case: O(n) - due to recursion stack; for a skewed tree
//     (e.g. all nodes on the right side) 
pub fn height(tree: &Tree) -> usize {
    match tree {
        Some(n) => height(&n.l).max(height(&n.r)) + 1,
        None => 0
    }
}
// Manual Debug
//
//     1
//  2
/* 
> f(root: 1)
  lh = f(root.l: 2) + 1
    > f(root.l: 2 )
      lh = f(root.l.l: None) + 1
        > f(root.l.l: None)
          0
      lh = 1    
      rh = f(root.l.r: None) + 1
        > f(root.l.r: None)
      rh = 1
      return max: 1
  lh = 2
  rh = f(root.r: None) + 1
    > f(root.r: None)
      0
  rh = 1
  return max: 2
*/
// or less idiomatic version:
fn he(t: &Tree) -> usize {
    if t.is_none() { return 0 };
    let l_h = he(&t.as_ref().unwrap().l) + 1;
    let r_h = he(&t.as_ref().unwrap().r) + 1;
    std::cmp::max(l_h, r_h)
}

#[test]
fn t1() {
    //  - empty tree
    assert_eq!(height(&None), 0);
    assert_eq!(height_iter(&None), 0);

    //  1
    let node = Node::new_leaf(1);
    assert_eq!(height(&node), 1);
    assert_eq!(height_iter(&node), 1);

    //     1
    //   1 
    let node = Node::new(1, node, None);
    assert_eq!(height(&node), 2);
    assert_eq!(height_iter(&node), 2);

    //    1
    //       33
    let t = Node::new(1, None, Node::new_leaf(33));
    assert_eq!(height( &t ), 2);
    assert_eq!(height_iter(&t), 2);

    //    1
    //       22
    //           333
    let node = Node::new(
        1,
        None,
        Node::new(22, None, Node::new_leaf(333)),
    );
    assert_eq!(height( &node ), 3);
    assert_eq!(height_iter(&node), 3);

    //             1
    //       22          33
    //   333
    //
    let node = Node::new(
        1, 
        Node::new(22, Node::new_leaf(33), None),
        Node::new_leaf(333)
    );
    assert_eq!(height(&node), 3);
    assert_eq!(height_iter(&node), 3);

    //                  1
    //       11                   22
    //   111     222
    //               4444
    //
    let node = Node::new (
        1, 
        Node::new(
            11, 
            Node::new_leaf(111), 
            Node::new(222, None, Node::new_leaf(4444))
        ),
        Node::new_leaf(22)
    );
    assert_eq!(height(&node), 4);
    assert_eq!(height_iter(&node), 4);

    //     1
    //    / \
    //   2   3
    //      / \
    //     4   5
    let t = Node::new(
        1,
        Node::new_leaf(2),
        Node::new(
            3,
            Node::new_leaf(4),
            Node::new_leaf(5),
        ),
    );
    assert_eq!(height(&t), 3);
    assert_eq!(height_iter(&t), 3);


    //     1
    //    / \
    //   2   3
    //        \
    //         4
    //          \
    //           5
    let t = Node::new(
        1,
        Node::new_leaf(2),
        Node::new(
            3,
            None,
            Node::new(
                4,
                None,
                Node::new_leaf(5),
            ),
        ),
    );
    assert_eq!(height(&t), 4);
    assert_eq!(height_iter(&t), 4);


    // --------- AI

    // Empty tree
    assert_eq!(height(&None), 0);
    assert_eq!(height_iter(&None), 0);

    // Balanced tree:
    //
    //        1
    //       / \
    //      2   3
    //     / \
    //    4   5

    let t = Node::new(
        1,
        Node::new(
            2,
            Node::new_leaf(4),
            Node::new_leaf(5),
        ),
        Node::new_leaf(3),
    );

    assert_eq!(height(&t), 3);
    assert_eq!(height_iter(&t), 3);

}