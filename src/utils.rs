/// pretty_print: _ is the separator
pub fn pp<T: std::fmt::Display>(num: T) -> String {
    num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
       .collect::<std::result::Result<Vec<&str>, _>>().unwrap().join("_")
}

pub fn print_type_of<T>(name: &str, _: &T) {
    println!("--- type of {}: {}", name, std::any::type_name::<T>())
}
