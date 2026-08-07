`_` actually has more than two uses in Rust:

### 1. Type inference placeholder
```rust
let v: Vec<_> = some_iter.collect(); // let compiler fill in the element type
let map: HashMap<_, _> = pairs.collect();
```

### 2. Ignore in patterns
```rust
let [_, _, third, _, _, _] = arr;  // ignore specific positions
let (_, y) = some_tuple;
match val {
    Some(_) => println!("got something"),
    _ => {}   // catch-all arm
}
```

### 3. Discard a value without binding
```rust
let _ = some_result; // evaluate but drop immediately, no move
```
!! This is subtly different from `_name` — `_` **never binds** and drops the value instantly, while `_name` still binds and moves:
```rust
let _guard = mutex.lock();  // ⚠️ stays alive, dropped at end of scope
let _ = mutex.lock();       // ⚠️ dropped IMMEDIATELY, lock released at once
```

### 4. Numeric literal separator (purely visual)
```rust
let x = 1_000_000;   // same as 1000000
let b = 0b1111_0000; // binary grouping
```

---

So the common thread is: `_` means **"I don't care about this"** — whether it's a type, a value, or a binding. The compiler never warns you about unused `_` either.