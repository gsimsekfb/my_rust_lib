// interv-2

// Prob:
// Find the Maximum Depth or Height of given Binary Tree
// Note: The height of an empty tree is 0 and 
// the height of a tree with single node is 1.
// https://www.geeksforgeeks.org/find-the-maximum-depth-or-height-of-a-tree/

// Other solutions
// https://www.geeksforgeeks.org/find-the-maximum-depth-or-height-of-a-tree/
// https://medium.com/go-rust/rust-day-10-lc-maximum-depth-of-binary-tree-872b39537716


#[derive(Debug)]
pub struct Node {
    pub v: i32,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>
}

pub fn new_node(v: i32, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { v, l, r }))
}

pub fn new_node_leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { v, l: None, r: None }))
}

// 2nd attempt
//
// ?
// Time Complexity: O(N)
// Auxiliary Space: O(N)
pub fn max_height_2(node: &Option<Box<Node>>) -> usize {
    match node {
        Some(n) => {
            let left_h = max_height_2(&n.l) + 1;
            let left_r = max_height_2(&n.r) + 1;
            std::cmp::max(left_h, left_r)
        }
        _ => 0
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
fn he(t: &Option<Box<Node>>) -> usize {
    if t.is_none() { return 0 };
    let l_h = he(&t.as_ref().unwrap().l) + 1;
    let r_h = he(&t.as_ref().unwrap().r) + 1;
    std::cmp::max(l_h, r_h)
}


// 1st attempt
fn max_height(node: &Node) -> usize {
    let mut h: usize = 0;
    max_height_impl(node, h + 1, &mut h);
    h
}

fn max_height_impl(node: &Node, h: usize, max_height: &mut usize) {
    // println!("{h}: {}", node.v);
    if h > *max_height { *max_height = h; }
    if node.l.is_some() {
        if h+1 > *max_height { *max_height = h+1; }
        max_height_impl(node.l.as_ref().unwrap(), h+1, max_height);
    }
    if node.r.is_some() {
        if h+1 > *max_height { *max_height = h+1; }
        max_height_impl(node.r.as_ref().unwrap(), h+1, max_height);
    }
}

#[test]
fn t1() {
    //  - empty tree
    assert_eq!(max_height_2(&None), 0);

    //  1
    let node = new_node_leaf(1);
    assert_eq!(max_height(node.as_deref().unwrap()), 1);
    assert_eq!(max_height_2(&node), 1);

    //    1
    // 22
    let node = Node { v: 1,  l: new_node_leaf(22), r: None };
    assert_eq!(max_height(&node), 2);
    assert_eq!(max_height_2( &Some(Box::new(node)) ), 2);

    //    1
    //       33
    let node = Node { v: 1, l: None, r: new_node_leaf(33) };
    assert_eq!(max_height(&node), 2);
    assert_eq!(max_height_2( &Some(Box::new(node)) ), 2);

    //    1
    //       22
    //           333
    let node = Node {
        v: 1,
        l: None,
        r: new_node(22, None, new_node_leaf(333)),
    };
    assert_eq!(max_height(&node), 3);
    assert_eq!(max_height_2( &Some(Box::new(node)) ), 3);

    //             1
    //       22          33
    //   333
    //
    let node = Node {
        v: 1, 
        l: new_node(22, new_node_leaf(33), None),
        r: new_node_leaf(333)
    };
    assert_eq!(max_height(&node), 3);
    assert_eq!(max_height_2( &Some(Box::new(node)) ), 3);

    //                  1
    //       11                   22
    //   111     222
    //               4444
    //
    let node = Node {
        v: 1, 
        l: new_node(
            11, 
            new_node_leaf(111), 
            new_node(222, None, new_node_leaf(4444))
        ),
        r: new_node_leaf(22)
    };
    assert_eq!(max_height(&node), 4);
    assert_eq!(max_height_2( &Some(Box::new(node)) ), 4);

}