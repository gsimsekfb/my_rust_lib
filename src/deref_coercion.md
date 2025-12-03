# deref coersion

In Rust, types that commonly implement `Deref` include:

1. **Smart Pointers**:
   - `Box<T>`
   - `Rc<T>`
   - `Arc<T>`
   - `Cow<'a, T>`
   - `Pin<P>`

2. **Reference Types**:
   - All shared references (`&T`) and mutable references (`&mut T`) themselves

3. **Container Types**:
   - `String` (derefs to `&str`)
   - `Vec<T>` (derefs to `&[T]`)
   - `PathBuf` (derefs to `&Path`)
   - `OsString` (derefs to `&OsStr`)

4. **Special Wrappers**:
   - `MutexGuard<T>`
   - `RwLockReadGuard<T>`
   - `RwLockWriteGuard<T>`
   - `RefCell<T>` (through `Ref<T>` and `RefMut<T>`)

The key pattern is: types that manage or wrap other data and want to provide transparent access to that underlying data through dereferencing.


5. **Examples**

```rust
// todo
struct Foo { x: i32 }
let mut foo = Box::new(Foo { x: 42 });
let ref1 = &foo.x;  // de-sugars-to/same-as `&(*foo).x` aka deref coercion
   // immut ref to x
   // &i32
 
// String to &str
fn takes_str(s: &str) {}
let string = String::from("hello");
takes_str(&string);  // Implicit: &String → &str

// Vec<T> to &[T]
fn takes_slice(s: &[i32]) {}
let vec = vec![1, 2, 3];
takes_slice(&vec);  // Implicit: &Vec<i32> → &[i32]

// Box<T> to &T
fn takes_ref(x: &i32) {}
let boxed = Box::new(5);
takes_ref(&boxed);  // Implicit: &Box<i32> → &i32

// Rc<T> to &T
use std::rc::Rc;
let rc = Rc::new(10);
takes_ref(&rc);  // Implicit: &Rc<i32> → &i32

// Method calls
let string = String::from("test");
string.len();  // Implicit: String → &str → calls &str::len()
string.starts_with("te");  // String → &str → method call
```