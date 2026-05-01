
use std::borrow::Cow;

// Cow: copy on write
//
// src: 
// https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55
// https://dhghomon.github.io/easy_rust/Chapter_42.html

// interv-2

// 1.a For the fn below 
// [perf]
// 99.9% of the time s contains no "zz"
fn remove_zz_(s: &str) -> String {
    if s.contains("zz") { s.replace("zz", ".") } 
    else { s.to_string() }
}
//
//    - Use Cow struct which represents allocating/owned obj.
//    - return String when s has "zz"
//    - else return &str (s)
//



// -----------------------------------------------------------------


//// Minimal summary of Cow:
//// 1. A type that's either borrowed or owned.
//// 2. Only clones when mutation is needed.
//// also see examples below
////
fn remove_zz_with_cow(s: &str) -> Cow<'_, str> {
    if s.contains("zz") {
        Cow::Owned(s.to_string().replace("zz", "**")) // String
    } else { // no allocation
        Cow::Borrowed(s) // &str
    }
}

// e.g.
// this struct is auto-created by actix_web when "/allocation" query comes
struct AllocationQuery { username: String }
    //                   username: Cow<'static, str>,
    // Use Cow ? - No 
    // - Cow avoids allocation when you pass a &str, only allocates when you 
    //   pass a String.
    // - For this program, from HTTP requests — always String
    //   (deserialized from JSON/query params). So Cow brings no benefit here,
    //   String is fine.
    // - Extra Note: these will make the API more flexible:
    // - impl Into<String> — caller can pass &str or String, use w/ ctors,
    //   allocation happens inside the ctor
    // - impl AsRef<str> — caller can pass &str or String, no allocation, 
    //   just borrows



//// Most likely usage: Cow<str> or Cow<[u8]>


// 1
// the ability to represent the data as borrowed or owned is figured in
// not at compile time, but runtime

// Problem: This allocates all the time
fn remove_zz(s: &str) -> String {
    s.to_string().replace("zz", "")
}
// [perf]
// What if 99.9% of the time s contains no "zz" ?? 
// User to use to_string on return to get String
fn remove_zz_cow(s: &str) -> Cow<'_, str> {
    if s.contains("zz") {
        Cow::Owned(s.to_string().replace("zz", "**")) // String
    } else { // no allocation
        Cow::Borrowed(s) // &str
    }
}

#[test]
fn return_str_or_string() {
    let ss = "aazz";
    assert_eq!(remove_zz_cow(ss), Cow::from("aa**"));
    assert_eq!(remove_zz_cow(ss), Cow::Borrowed("aa**"));
    assert_eq!(remove_zz_cow(ss), Cow::<String>::Owned("aa**".to_string()));

    let ss = "aa";
    assert_eq!(remove_zz_cow(ss), Cow::from("aa"));
    assert_eq!(remove_zz_cow(ss), Cow::Borrowed("aa"));
    assert_eq!(remove_zz_cow(ss), Cow::<String>::Owned("aa".to_string()));
}

// 2. Copy on Write
// pass borrowed data around without cloning up until the moment when (and if)
// we need to modify it

struct LazyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> LazyBuffer<'a> {
    fn new(data: &'a[u8]) -> Self {
        Self { data: Cow::Borrowed(data) }
    }

    // todo: how this compiles ? Cow to &[u8] auto conversion ?
    fn data(&self) -> &[u8] { &self.data }

    fn append(&mut self, data: &'a[u8]) {
        self.data.to_mut().extend(data);
            // to_mut: Clones the data if it is not already owned
    }
}

#[test]
fn append() {
    let arr = [0; 3];

    // No memory copied yet
    let mut buf = LazyBuffer::new(&arr);

    assert_eq!(buf.data.as_ref(), [0; 3]);
    assert_eq!(buf.data(), [0; 3]);

    // The buf data (arr) is cloned and owned
    buf.append(&[1,2,3]);
    assert_eq!(buf.data.as_ref(), [0,0,0,1,2,3]);

    // The data (arr) is not cloned on further attempts
    buf.append(&[4, 5]);
    assert_eq!(buf.data.as_ref(), [0,0,0,1,2,3,4,5]);
}


// 3. Struct "optionally" owning the data

// We have both owned and borrowed version of the User struct. Also see ctors.
struct User<'a> {
    first_name: Cow<'a, str>,
    last_name:  Cow<'a, str>,
}

impl<'a> User<'a> {
    fn new_owned(s1: String, s2: String) -> Self {
        Self { first_name: Cow::from(s1) , last_name: Cow::from(s2) }
    }

    fn new_borrowed(s1: &'a str, s2: &'a str) -> Self {
        Self { first_name: Cow::from(s1), last_name: Cow::from(s2) }
    }

    fn first_name(&self) -> &str { &self.first_name }
}


#[test] fn in_struct() {
    let user = User::new_owned("a".to_string(), "b".to_string());
    assert_eq!(user.first_name(), "a");

    let a = "a";
    let b = "b";
    let user = User::new_borrowed(a, b);
    assert_eq!(user.first_name(), "a");
}
