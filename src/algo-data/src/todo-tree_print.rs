// todo

// use tree::{new_node, new_node_leaf, Node};
// use tree_max_height::max_height_2;

// use std::collections::{btree_map, BTreeMap};

// fn traverse_tree(node: &Node, h: usize, map: &mut BTreeMap<usize, Vec<i32>>) {
//     println!("{h}: {}", node.v);
//     map.insert(h, node.v);
//     if node.l.is_some() {
//         traverse_tree(node.l.as_ref().unwrap(), h+1, map);
//     }
//     if node.r.is_some() {
//         traverse_tree(node.r.as_ref().unwrap(), h+1, map);
//     }
// }

// fn save_tree(node: &Node) -> BTreeMap<usize, Vec<i32>> {
//     let mut tree = BTreeMap::new();
//     traverse_tree(&node, 1, &mut tree);
//     tree
// }


// fn print_tree(node: &Node, h: usize, max: &mut usize) {
//     println!("{h}: {}", node.v);
//     if node.l.is_some() {
//         if h+1 > *max { *max = h+1; }
//         print_tree(node.l.as_ref().unwrap(), h+1, max);
//     }
//     if node.r.is_some() {
//         if h+1 > *max { *max = h+1; }
//         print_tree(node.r.as_ref().unwrap(), h+1, max);
//     }
// }


// #[test]
// fn tt() {
//     //    1
//     //       22
//     //           333
//     let node = Node {
//         v: 1,
//         l: None,
//         r: new_node(22, None, new_node_leaf(333)),
//     };
    
// }