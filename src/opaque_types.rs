
// 1. Using private fields
mod aa {
    pub struct OpaqueType {
        hidden_data: i32 // Opaque type hence not public
            // Users of OpaqueYtpe cannot directly access hidden_data, 
            // only interact via methods.
    }

    impl OpaqueType {
        pub fn new() -> Self { Self { hidden_data: 42 }}
        pub fn val(&self) -> i32 { self.hidden_data }
    }
}

#[test]
fn ex1() {
    let obj = aa::OpaqueType::new();
    // assert_eq!(obj.hidden_data, 42); // err: private field
    assert_eq!(obj.val(), 42);
}



// 2. Using return Impl (existential types)

trait MyTrait { fn val(&self) -> i32; }

impl MyTrait for &'static str {
    fn val(&self) -> i32 { self.parse::<i32>().unwrap() }
}

fn create_opaque() -> impl MyTrait { "12" }

#[test]
fn ex2() {
    let obj = create_opaque(); // user does not know the underlying type (&str)
    assert_eq!(obj.val(), 12);
}