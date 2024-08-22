
// 1. My generic trait that converts an object to T
trait MyInto<T> {
    fn into(self)->T;
}

impl MyInto<String> for String {
    fn into(self)->String { self }
}

impl MyInto<String> for &str {
    fn into(self)->String { self.to_string() }
}

impl MyInto<String> for MyString {
    fn into(self)->String { self.ss }
}

impl MyInto<String> for MyStruct<String> {
    fn into(self)->String { self.dd }
}

// 2. My non-generic struct which is ctorable 
// from any type which implements Into<String> e.g. String, &str
// or MyInto<String> e.g. String, &str, MyString, MyStruct<String> etc..
struct MyString { ss: String }

impl MyString {
    fn new(s: impl Into<String>) -> Self {
        Self { ss: s.into() }
    }
    fn new2(s: impl MyInto<String>) -> Self {
        Self { ss: s.into() }
    }
}

// 3. My generic struct - same as 2 but generic
struct MyStruct<T> { dd: T }
impl<T>  MyStruct<T> {
    fn new(d: impl Into<T>) -> Self {
        Self { dd: d.into() }
    }
    fn new2(d: impl MyInto<T>) -> Self {
        Self { dd: d.into() }
    }
}

#[test]
fn ex1() {
    // From MyString or MyStruct to String using MyInto trait
    let obj = MyString {ss: "aa".to_string()};
    let s: String = MyInto::into(obj);
    let obj = MyStruct {dd: "aa".to_string()};
    let s: String = MyInto::into(obj);

    // Create MyStruct with ctors
    let s = "aa";   // &str
    let obj = MyStruct::<String>::new(s);
    let s = "aa";   // &str
    let obj = MyStruct::<String>::new2(s);

    // Create MyString with ctors
    let s = "aa";   // &str
    let obj = MyString::new(s);
    let s = "bb".to_string();   // String
    let ms = MyString::new("aa".to_string());
    // 
    let s = "aa";   // &str
    let ms = MyString::new2(s);
    let s = "bb".to_string();   // String
    let ms = MyString::new2("aa".to_string());
}
