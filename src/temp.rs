// ----------------------
// cmds:
// tp ww
// cargo t -- --nocapture ww
//
// Template - copy and change this:
/*
#[test]
fn ex1_ww() {
    let xx = 55;
    println!("xx: {:?}", xx);
    assert_eq!(32, 32);
}
*/


#[test]
fn ex1_ww() {

    let xx = 55;
    println!("xx: {:?}", xx);
    assert_eq!(32, 32);

}


// ----------------------

// // https://doc.rust-lang.org/stable/reference/const_eval.html

// fn multiply_arrays<const N: usize>(left: [u8; N], right: [u8; N]) -> [u8; N] {
//     let mut res = [0; N];
//     for n in 0..N {
//         res[n] = left[n] * right[n];
//     }
//     res
// }


// // 
// #[test]
// fn ex1_ww() {
//     assert_eq!(multiply_arrays([1,2,3], [2,2,2]), [2, 4, 6]);
// }


// ----------------------

// ----------------------


// #[cfg(feature = "perf")]
// flat_node += now.elapsed();


// ----------------------

// Src:
// https://www.lurklurk.org/effective-rust/references.html


// struct Point { x: u32 }

// fn foo(p: &Point) {
//     println!("p.x: {:?}", p.x);
// }

// // Making foo more generic
// // This means it accepts the widest range of reference-like types
// fn foo2(p: impl AsRef<Point>) {
//     let pp = p.as_ref(); // pp: &Point
//     println!("p.x: {:?}", p.as_ref().x);
// }

// #[test]
// fn ex1_ww() {
//     let p = Point { x: 1 };
//     let ref_p = &p;
//     foo(ref_p);

//     let bp = Box::new( Point { x: 22 } );
//     foo2(&bp);
// }



// ---------------------------------------------




// ---------------------------------------------

// -------- todo
// 
// pub fn nodes_from_file<T>(file: &str) -> Vec<T> 
// where for<'a> T: Clone + Deserialize<'a> + PartialEq + Serialize {
//     let json = std::fs::read_to_string(file).unwrap();
//     let json = json.replace("formula:sum", "formula_sum")
//                    .replace("formula:custom", "formula_custom");
//     serde_json::from_str(&json).expect("Invalid JSON")
// }