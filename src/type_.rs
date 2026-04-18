#![allow(private_interfaces)]

// interv-1

// 1. <skip, todo> use "type" in trait body e.g. impl MyIter::next()
// 2. Create Type Synonym Meters for i32;
// Use them in test




// =======================================================================





// 1.
// Simple, dummy representation of Iterator trait
pub trait Iterator {
    type Item;  // The type of the elements being iterated over

    fn next(&mut self) -> Option<Self::Item>;
    // . . .
}

// Simple usage of iterators
fn foo() {
  #[allow(clippy::useless_vec)]
  let v1 = vec![1, 2, 3];      // Vec<i32, Global>
  let mut v1_iter = v1.iter(); // Iter<i32>
  assert_eq!(v1_iter.next(), Some(&1));

  let xx = v1_iter.next();     // Option<&i32>
}


// 2. 
// Creating Type Synonyms with Type Aliases
type Meters = i32;

#[test] fn ex_2() {
    let x: i32 = 3;
    let y: Meters = 2;
    assert_eq!(x+y, 5);
}

// 3. Use short Result<String> instead Result<String, actix_web::Error> 

struct Error {}

// from actix_web lib:
    /// A convenience [`Result`](std::result::Result) for Actix Web operations.
    ///
    /// This type alias is generally used to avoid writing out `actix_http::Error` directly.
    /// Meaning: Let's use Result<T, E> instead of std::result::Result<T, E>, 
    /// and E type param will be actix_web::Error if user not specifies.
    pub type Result<T, E = Error> = std::result::Result<T, E>;
        // Error: actix_web::Error

// e.g.
// Result<String> is actually Result<String, actix_web::Error> here:
/*     
#[get("/")]
async fn index_2() -> Result<String> {
    let result = Err(MyError { name: "test error" });
    result.map_err(|err| error::ErrorBadRequest(err.name))
}
 */

