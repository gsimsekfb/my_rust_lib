
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
