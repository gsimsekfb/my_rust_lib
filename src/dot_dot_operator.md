`..` has several distinct uses in Rust:

### 1. Ignore remaining elements in patterns (your example)
```rust
let [first, .., last] = arr;         // ignore everything in between
let [first, ..] = arr;               // ignore everything after
let (first, .., last) = some_tuple;
match some_tuple {
    (1, .., last) => println!("{last}"),
}
```

### 2. Struct update syntax
```rust
let config = Config {
    debug: true,
    ..Config::default()  // fill remaining fields from default
};
```

### 3. Range (exclusive end)
```rust
for i in 0..10 {}        // 0 to 9
let s = &arr[1..3];      // slice
```

### 4. `..=` inclusive range
```rust
for i in 0..=10 {}       // 0 to 10
match x {
    1..=5 => println!("small"),
}
```

### 5. Full range (unbounded)
```rust
let s = &arr[..];   // entire slice, equivalent to &arr
let s = &arr[2..];  // from index 2 to end
let s = &arr[..3];  // from start to index 2
```

---

The common thread is **"the rest"** — whether it's the rest of a range, the rest of a pattern, or the rest of a struct's fields. The range meaning is the most familiar, but the "skip everything else" meaning in patterns is equally common in production code.