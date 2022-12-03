
//// Dynamically sized types aka DSTs or unsized types
//// these types let us write code using values whose
//// size we can know only at runtime

/// Problem
// let s1: str = "Hello there!";    // 12 bytes
// let s2: str = "How's it going?"; // 15 bytes
      // Error: Rust needs to know how much memory to allocate for any 
      // value of a particular type, 
      // and ALL values of a type must use the SAME amount of memory.


// - Every trait is a dynamically sized type we can refer to by
//   using the name of the trait

