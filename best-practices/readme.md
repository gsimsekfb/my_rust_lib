
### 3.6. Understand type conversions
https://www.lurklurk.org/effective-rust/casts.html 

Rust type conversions fall into three categories:
```
manual:         user-defined type conversions provided by implementing the From and Into traits
semi-automatic: explicit casts between values using the as keyword
automatic:      implicit coercion into a new type.
```

For consistency and safety you should prefer `from / into` conversions to `as` casts, unless you understand and need the precise casting semantics (e.g for C interoperability). 
https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics

#### a) Manual conversions
```
#[derive(Debug, PartialEq)]
struct GreaterThanZero(i32);

// From std lib:
pub trait TryFrom_<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

// Our impl
impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(val: i32) -> Result<Self, Self::Error> {
        if val <= 0 {
            return Err("GreaterThanZero only accepts values greater than zero!")
        }
        Ok(GreaterThanZero(val))
    }
}

#[test] 
fn ex_1() {
    let gg = GreaterThanZero::try_from(42);
    assert_eq!(gg.unwrap(), GreaterThanZero(42));
    let gg = GreaterThanZero::try_from(-33);
    assert_eq!(gg, Err("GreaterThanZero only accepts values greater than zero!"));
}
```

#### b) Casts
https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics
```
let x: u32 = 9;
let y = x as u64;
```

#### c) Automatic conversions - coercions 
https://doc.rust-lang.org/reference/type-coercions.html

Most of the coercions involve silent conversions of pointer and reference types in ways that are sensible and convenient for the programmer, such as:
- converting a mutable reference to a non-mutable references (so you can use a &mut T as the argument to a function that takes a &T)
- converting a reference to a raw pointer (this isn't unsafe – the unsafety happens at the point where you're foolish enough to use a raw pointer)
- converting a closure that happens not to capture any variables into a bare function pointer (Item 2)
- converting an array to a slice

###  3.5 Familiarize yourself with standard traits
https://www.lurklurk.org/effective-rust/std-traits.html  

Derivable traits: 
- `Clone`: Items of this type can make a copy of themselves when asked.  
- `Copy`: If the compiler makes a bit-for-bit copy of this item's memory representation, the result is a valid new item.  
- `Default`: It's possible to make new instances of this type with sensible default values.  
- `PartialEq`: There's a partial equivalence relation for items of this type – any two items can be definitively compared, but it may not always be true that x==x.  
- `Eq`: There's an equivalence relation for items of this type: any two items can be definitively compared, and it is always true that x==x.  
- `PartialOrd`: Some items of this type can be compared and ordered.  
- `Ord`: All items of this type can be compared and ordered.  
- `Hash`: Items of this type can produce a stable hash of their contents when asked.  
- `Debug`: Items of this type can be displayed to programmers.  
- `Display`: Items of this type can be displayed to users.

Other (non-deriveable) standard traits are covered in other Items, and so are not included here. These include:

- `Fn, FnOnce and FnMut`: Items implementing this trait represent closures that can be invoked. See Item 2.
- `Error`: Items implementing this trait represent error information that can be displayed to users or programmers, and which may hold nested sub-error information. See Item 4.
- `Drop`: Items implementing this trait perform processing when they are destroyed, which is essential for RAII patterns. See Item 11.
- `From and TryFrom`: Items implementing this trait can be automatically created from items of some other type, but with a possibility of failure in the latter case. See Item 6.
- `Deref and DerefMut`: Items implementing this trait are pointer-like objects that can be dereferenced to get access to an inner item. See Item 9.
- `Iterator and friends`: Items implementing this trait represent collections that can be iterated over. See Item 10.
- `Send`: Items implementing this trait are safe to transfer between multiple threads. See Item 17.
- `Sync`: Items implementing this trait are safe to be referenced by multiple threads. See Item 17. 


###  3.1. Use the type system to express your data structures  
Src: https://www.lurklurk.org/effective-rust/use-types.html  

// a. always encode the result of an operation that might fail as a Result<T, E>  
```
    pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
        let f = std::fs::File::open("/etc/passwd")?;
    }
```

// b. Use `usize` for return type of size of collections fn.
  > standard collections return their size as a usize (from .len()), so collection indexing means that usize values are quite common – which is obviously fine from a capacity perspective, as there can't be more items in an in-memory collection than there are memory addresses on the syste  


### 3.3. When to avoid matching Option and Result  
Src: https://www.lurklurk.org/effective-rust/transform.html  

// a. When to use `if let` instead of match:  
```
    struct S {
        field: Option<i32>,
    }

    let s = S { field: Some(42) };
    match &s.field {
        Some(i) => println!("field is {}", i),
        None => {}
    }
```

For this situation, an if let expression is one line shorter and, more importantly, clearer:   
    
```
    if let Some(i) = &s.field {
        println!("field is {}", i);
    }
```

// b. When to use ? instead of match      
```
    pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
        let f = match std::fs::File::open("/etc/passwd") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
    }

    pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
        let f = std::fs::File::open("/etc/passwd")?;
    }
```

// c. Error mapping  
```
    pub fn find_user(username: &str) -> Result<UserId, String> {
        let f = match std::fs::File::open("/etc/passwd") {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("Failed to open password file: {:?}", e))
            }
        };
        // ...
    }

    pub fn find_user(username: &str) -> Result<UserId, String> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open password file: {:?}", e))?;
        // ...
    }
    // Better still, even map_err may not be necessary – if the outer error type 
    // can be created from the inner error type via an implementation of the 
    // From standard trait (Item 5), then the compiler will 'automatically' perform 
    // the conversion without the need for a call to .map_err()
```

###  3.4 Prefer idiomatic Error variants 

// a.  
Consider using David Tolnay's `anyhow` crate for error handling in applications.  


// b. Libraries versus Applications  
Code that's written for a library can't predict the environment in which the code is used, so it's preferable to emit concrete, detailed error information, and leave the caller to figure out how to use that information. This leans towards the enum-style nested errors described previously (and also avoids a dependency on anyhow in the public API of the library.  




