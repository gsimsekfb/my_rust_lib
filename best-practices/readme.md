
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
    
