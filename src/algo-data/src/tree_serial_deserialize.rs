// last: 7/25

// Problem Description
// Serialize and Deserialize a Binary Tree
// e.g.
//        1 
//   21      
//     31
// 
// csv: 1 21 0 31
//
// https://www.geeksforgeeks.org/serialize-deserialize-binary-tree/

#[derive(Debug, PartialEq)]
struct N {
    v: i32,
    l: Option<Box<N>>,
    r: Option<Box<N>>
}

fn new_leaf(v: i32) -> Option<Box<N>> {
    Some(Box::new( N { v, l: None, r: None} ))
}

fn new_node(v: i32, l: Option<Box<N>>, r: Option<Box<N>>) -> Option<Box<N>> {
    Some(Box::new( N { v, l, r} ))
}

// Sol 1 
// Hint: ser fn is easier, start with it
// Hint: Use recursive fn, do sth w/ current node or its value then
// traverse/recurse on lefts and then rights. 

//        1 
//   21      
//     31
// 
// csv: 1 21 0 31
fn ser(n: &Option<Box<N>>, res: &mut Vec<i32> ) {
    if n.is_none() { res.push(0);   /*print!("0-");*/    return; }
    let v = n.as_ref().unwrap().v;
    res.push(v);    /*print!("{v}-");*/ 
    ser(&n.as_ref().unwrap().l, res);
    ser(&n.as_ref().unwrap().r, res);
}
// Debug
// f( 1, l: (21, l:null, r: 31), null )
// v = 1
// f( 21, l:null, r:31 )
//   v = 21
//   f ( null ) ret
//   f ( 31 )
//     v = 31
//     f( null ) ret
//     f( null ) ret
//   ret
// f ( null ) ret
// end


// todo
//
// fn deser_(arr: &[i32]) -> Option<Box<N>> {
//     if let Some(e) = arr.get(0) {
//         // print!("{e}-");
//         if *e == 0 { return None };
//         let mut n = new_leaf(*e);
//         n.as_deref_mut().unwrap().l = deser_(&arr[1..arr.len()]);
//         n.as_deref_mut().unwrap().r = deser_(&arr[1..arr.len()]);
//         return n;
//     }
//     None
// }

//        1 
//   21      
//     31
// 
// csv: 1 21 0 31
fn deser(iter: &mut core::slice::Iter<i32>) -> Option<Box<N>> {
    let &val = iter.next()?;
    if val == 0 { return None };

    let mut new_node = new_leaf(val);
    new_node.as_deref_mut().unwrap().l = deser(iter);
    new_node.as_deref_mut().unwrap().r = deser(iter);

    new_node
}
// Debug
// f(1)
// n.v = 1
// n.l = f(21)
//   f(21) 
//   n.v = 21
//   n.l = f(0) ret None
//   n.r = f(31)
//     f(31)
//     n.l = None
//     n.r = None 
//     ret 31, None, None
//   ret 21, None, (31, None, None)
// n.r = None
// ret v:1, l: (v:21, l:None, r:(v:31, l:None, r:None)), r:None 



#[test] fn ser_deser() {

    // Sol-1

    // --------- Deserialize 

    //        1 
    //   21      
    //     31
    // 
    // csv: 1 21 0 31
    let arr = [1, 21, 0, 31];
    let tree = deser(&mut arr.iter());
    let res = new_node(
        1,
        new_node(21, None, new_leaf(31)),
        None
    );
    assert_eq!(tree, res);

    // todo
    //
    // let tree = deser_(&arr);
    // assert_eq!(tree, res);

    //        1 
    //  21        22
    //     31
    // 
    // csv: 1 21 0 31 0 0 22
    let csv = [1,21,0,31,0,0,22];
    let tree = deser(&mut csv.iter());
    let res = new_node(
        1, 
        new_node(21, None, new_leaf(31)), 
        new_leaf(22)
    );
    assert_eq!(tree, res);

    //                  1
    //       21                   22
    //   333     334
    //               4444
    //
    // csv: 1 21 333 0 0 334 0 4444 0 0 22
    let csv = [1,21,333,0,0,334,0,4444,0,0,22];
    let tree = deser(&mut csv.iter());
    let res = new_node(
        1, 
        new_node(21, new_leaf(333), new_node(334, None, new_leaf(4444))), 
        new_leaf(22)
    );
    assert_eq!(tree, res);


    // ------- Serialize 

    //        1 
    //   21      
    //     31
    //
    // csv: 1 21 0 31
    let tree = new_node(
        1,
        new_node(21, None, new_leaf(31)),
        None
    );
    let mut res = vec![];
    ser(&tree, &mut res);
    assert_eq!(res, [1,21,0,31,0,0,0]);


    //        1 
    //  21        22
    //     31
    // 
    // csv: 1 21 0 31 0 0 22
    let tree = new_node(
        1, 
        new_node(21, None, new_leaf(31)), 
        new_leaf(22)
    );
    let mut res = vec![];
    ser(&tree, &mut res);
    assert_eq!(res, [1,21,0,31,0,0,22,0,0]);

    //                  1
    //       21                   22
    //   333     334
    //               4444
    //
    // csv: 1 21 333 0 0 334 0 4444 0 0 22
    let tree = new_node(
        1, 
        new_node(21, new_leaf(333), new_node(334, None, new_leaf(4444))), 
        new_leaf(22)
    );
    let mut res = vec![];
    ser(&tree, &mut res);
    assert_eq!(res, [1,21,333,0,0,334,0,4444,0,0,22,0,0]);

}
