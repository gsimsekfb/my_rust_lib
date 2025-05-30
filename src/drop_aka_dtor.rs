// interv-1
// - struct Foo with data String
// - impl drop fn
// - create a Foo and manually drop it
// - hints at the end




// --------------------------------------------------------------------




struct Foo {
    data: String,
}
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping MySmartPtr with data `{}`!", self.data);
    }
}

// 1. Automatic dtor with drop() of trait Drop
#[test]
fn ex1_dtor_drop() {
    let c = Foo { data: String::from("my stuff") };
    let d = Foo { data: String::from("other stuff") };
    println!("MySmartPtrs created.");
} // end of this scope: d then c dropped strictly in this order


// 2. Manual/explicit dtor with std::mem::drop
#[test]
fn ex2_mem_drop() {
    let c = Foo { data: String::from("xxx") };
    println!("MySmartPtr created.");
    std::mem::drop(c);
    println!("MySmartPtr dropped before the end of main.");
    // let xx = &c.data; // error: value moved
}

// Hint
// - impl Drop::drop()
// - use std::mem:drop
