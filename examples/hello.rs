// Run: 
// cargo r --example hello
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello from an example!");
    println!("args: {args:?}");
}