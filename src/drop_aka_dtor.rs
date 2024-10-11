// interv
// create an object, manually destroy it 


struct MySmartPtr {
    data: String,
}
impl Drop for MySmartPtr {
    fn drop(&mut self) {
        println!("Dropping MySmartPtr with data `{}`!", self.data);
    }
}

// 1. Automatic dtor with drop() of trait Drop
#[test]
fn ex1_dtor_drop() {
    let c = MySmartPtr { data: String::from("my stuff") };
    let d = MySmartPtr { data: String::from("other stuff") };
    println!("MySmartPtrs created.");
} // end of this scope: d and c dropped strictly in this order

// 2. Manual/explicit dtor with std::mem::drop
#[test]
fn ex2_mem_drop() {
    let c = MySmartPtr { data: String::from("xxx") };
    println!("MySmartPtr created.");
    std::mem::drop(c);
    println!("MySmartPtr dropped before the end of main.");
    // let xx = &c.data; // error: value moved
}