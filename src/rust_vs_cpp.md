# Rust vs C++ 

### !!! Do not include long topics like smart ptrs ?? 

<table>

<tr>
<td> Topic </td> <td> Rust </td> <td> C++ </td>
</tr>

// todo: cfg.rs ifdef

// todo: algos / iter map reduce filter

// todo: refcell mutate

    let p1 = Rc::new(RefCell::new(String::from("ab"))); // Rc<Refcell<T>>
    // mutate T
    // !! p1 does not have to be "mut" to be mutated which is the point
    //    RefCell and Cell smart ptrs - aka Interior(Runtime) Mutability
    *p1.borrow_mut() = "cd".to_string();

// move semantics
let pair = (42, "abc".to_string());
let (x, s1) = &pair;     // x: &i32, s1: &String
let (x, s1) = pair;      // x: i32,  s1: String   // !! pair is moved
let (x, ref s1) = pair;  // x: i32,  s1: &String  // ref keyword

// todo: .. keyword


<!-- ----------------------------------------------------- -->
<tr>
<td> temp </td>

<td>

```rust

```

</td>

<td>
    
```cpp

```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>

<td> Ternary Operator </td>

<td>

```rust
let res = if x < 0 { '-' } else { '+' };
```

</td>
<td>
    
```cpp
char ch = x < 0 ? '-' : '+';
    // Warn: Use ?: for pure value selection, not with fn calls w/ side effects
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>

<td> Borrowing vs C++ mut/const ref </td>

<td>

```rust
// 1. Immutable borrow
fn f(num: &i32) { }     // call: f(&x);

// 2. Mutable borrow
fn change_value(num: &mut i32) { *num = 44; }
    // call:
    let mut x = 0;
    change_value(&mut x);

// 3. Move
fn move_(num: String) { }
fn move_(mut num: String) { } // move and modify in fn
    // call both:
    move_(s);
```

</td>
<td>
    
```cpp
// 1. Const reference
void f(const string& s) { s = "bb" };   // call: f(str);

// 2. Mutable reference
void changeValue(string& s) { s = "bb" };
    // call:
    string str = "undefined";
    changeValue(str);

// 3. Move
void move_(const string s) { };
void move_(string s) { s = "bb" }; // move and modify in fn
    // call:
    move_(str);
```
</td>
</tr>







<!-- ----------------------------------------------------- -->
<tr>
<td> Reference / Pointer </td>

<td>

```rust
// 1. Basics
let mut i = 42;
let ref_ = &i;      // &i32     - immut binding to immut ref
let ref_ = &mut i;  // &mut i32 - immut binding to mut ref
let pair = (42, "abc".to_string());
let (x, ref s1) = ; // x: i32, s1: &String  // ref keyword

let ptr = &i as *const i32; // syntax: &T as *const T
let ptr = &mut i *mut i32   // syntax: &T as *mut T

// deref
dbg!(ref_);
unsafe { dbg!(*ptr); }  // !! unsafe required to deref raw ptr !!

// print ptr addr
dbg!(ptr);

// 2. table
let mut x = 1;
let mut binding = &mut x;  // mut binding   to mut ref
let binding     = &mut x;  // immut binding to mut ref
let mut binding = &x;      // mut binding   to immut ref
let binding     = &x;      // immut binding to immut ref
```

</td>

<td>
    
```cpp
// 1. Basics
int i = 42;
int& ref = i; // int& - references are NOT re-bindable, unlike ptrs
ref = x;      // this is not a rebind, it will assign value `i = x`

int* ptr = &i;  // int*

// deref
cout << ref
cout << *ptr

// print ptr addr
cout << ptr

// 2. table
int* p1;              // mut ptr   to mut data
int* const p1;        // immut ptr to mut data
const int* p1;        // mut ptr   to immut data
const int* const p1;  // immut ptr to immut data
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>

<td> String </td>

<td>

```rust
let str = String::from("abc");
```

</td>
<td>
    
```cpp
string greeting = "Hello";
char str[] = "C++";
char str[4] = {'C','+','+','\0'};
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>

<td> Vector </td>

<td>

```rust
//// create
let v = vec![1,2,3];
let v: Vec<i32> = Vec::new();
let v: Vec<i32> = (0..5).collect();
let v = Vec::from([1, 2, 3]);
let v = [1, 2, 3].to_vec();
    // Slicing
    let slice = &vec[1..4];  // Elements at index 1, 2, 3
    let slice = &vec[1..=4]; // Elements at index 1, 2, 3, 4
    let slice = &vec[..3];  // First 3 elems (0,1,2)
    let slice = &vec[vec.len()-2..vec.len()]; // Last 2 elems
    let slice = &vec[2..];  // From index 2 to end
    let slice = &vec[..];   // Entire vector as slice

//// iter
 e in &vec { dbg!(e); };      // e: &i32
for e in &mut vec { *e += 10; }; // e: &i32
for e in vec { dbg!(e); };       // e: i32, !! vec moved/consumed
for e in vec.iter().rev() { dbg!(e); }; // e: &i32

vec.iter();      // Immutable borrow iterator
vec.iter_mut();  // Mutable borrow iterator
vec.into_iter(); // Consuming/move iterator

//// read
if let Some(val) = vec.get(1) { }
let val = vec.get(5).unwrap_or(&0); // Get with default, val: &i32
let x = v[0];   // unsafe, panics out of bounds

//// write
v.push(42)
v.pop() // remove last
```

</td>
<td>
    
```cpp
vector v = {1, 2, 3};
vector<int> v = {1, 2, 3};
    // slicing, cpp20
    std::span(vec).subspan(1, 3); // Indexes 1, 2, 3
    std::span(vec).first(2);      // First 2
    std::span(vec).last(2);       // Last 2
    std::span(vec).subspan(2);    // From index 2 to end
    std::span(vec);               // Entire vector

//// iter
for(int e : vec) { } // range-based for loop - cpp11
// pre-cpp11
for (auto iter = vec.begin(); iter != vec.end(); ++iter) {
    *iter += 10;
}
for (size_t i = 0; i < vec.size(); ++i) { std::cout << vec[i] << " "; }

// ranges, cpp20
for(auto e : vec | std::views::reverse) { println(e); };
for(auto& e : vec | std::views::reverse) { e +=10; };

//// read 
try {   // safe access
    int value = vec.at(0); // OK
} catch (const out_of_range& err) {
    cerr << "err: " << err.what() << "\n";
}

vec[5] // unsafe but use if fast access needed

//// write
v.push_back(42)
v.pop() // remove last
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>

<td> todo: Iterators/Algos </td>

<td>

```rust
vec.iter();      // Immutable borrow iterator
vec.iter_mut();  // Mutable borrow iterator
vec.into_iter(); // Consuming/move iterator

let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
let filtered: Vec<_> = v.iter().filter(|&&x| x > 2).collect();

```

</td>
<td>
    
```cpp

```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> Map/HashMap </td>

<td>

```rust
//// create
// Unsorted map
let mut map = HashMap::from([ (1, "a"), (2, "b") ]);
let map = vec![ (3,"c"), (4, "d") ].into_iter().collect::<HashMap<_,_>>();
let map: HashMap<_,_> = vec![ (3,"c"), (4, "d") ].into_iter().collect();
// Sorted map (ordered by keys)
let mut bmap: BTreeMap<i32, String> = BTreeMap::new();

//// iter
for (key, val) in &map { dbg!(key, val); };
for (key, val) in &mut map { *val = "++"; };
for (key, val) in map.iter().rev() { dbg!(key); }; // !! only for BTreeMap 

//// read
// Using get - returns Option<&V>
if let Some(value) = map.get(&1) { println!("{}", value); }
let value = map[&2];    // panics if key doesn't exist

//// write
map.insert(1, "ww"); // insert or update
if let Some(value_ref) = map.get_mut(&1) { *value_ref = "--"; }
// Modify if exists, insert otherwise
map.entry(1).and_modify(|val| *val = "--").or_insert("uu");

// Remove by key - returns Option<V>
let removed = map.remove(&1);
// Remove and return key-value pair
let removed = map.remove_entry(&1); // Returns Option<(K, V)>
// Retain only matching entries
map.retain(|_k, &mut val| val == "b");
```

</td>

<td>
    
```cpp
//// create
std::map<int, string>           map= { {2, "bb" }, { 1, "aa"} };
std::unordered_map<int, string> map= { {2, "bb" }, { 1, "aa"} };

//// iter
for (auto const& [kk, vv] : map) { print(vv); print(" "); };
for (auto const& e : map) { print(e.first); print(e.second); };
// Reverse iteration
for (auto it = m.rbegin(); it != m.rend(); ++it) {
    cout << it->first << ": " << it->second << "\n";
}

//// read
if (auto it = map.find(20); it != map.end()) { // cpp17: if w/ initializer
    // Use: it->first, it->second
}
auto elem = map.at(13); // access with bounds checking - throws
map[13];       // !! !! force update/insert - dangerous for lookups

//// write
map[3] = "cc";          // !! force update/insert - dangerous for lookups
map.insert({3, "--"});  // no effect - inserts only if key does not exist
map.insert_or_assign(55, "ttt");    // cpp17, inserts or updates
map.emplace(4, "dd");   // Constructs in-place if key doesn't exist

map.erase(4);
map.erase(map.begin()); // by iterator
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>

<td> Array </td>

<td>

```rust
let nums = [1, 2, 3];
let zeros = [0;3];
let zeros: [i32;3] = [0;3]; 
```

</td>
<td>
    
```cpp
array<int, 3> arr = {1, 2, 3};
// avoid raw arrays:
int arr[3] = {1,2,3}; // arr:0x7fff1b72911c
size_t arr_len = sizeof(arr) / sizeof(arr[0]); // 12/4=3

array<array<int,2>,2> arr = {{ {{1,2}}, {{3,4}} }};
int arr[3][2] = { {1,2}, {3,4}, {5,6} };
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> Pair / Tuple </td>

<td>

```rust
//// Pair/Tuple (No separate pair type in Rust, all tuples )
// create
let pair                = (42, "abc".to_string());
let pair: (i32, String) = (42, "abc".to_string());

// destruct
let (x, s1) = &pair;     // x: &i32, s1: &String
// let (x, s1) = pair;      // x: i32,  s1: String   // !! pair is moved
let (x, ref s1) = pair;  // x: i32,  s1: &String  // ref keyword
let (_, s1) = &pair;     // s1: &String
let (.., last) = &pair;  // Get last element (works for longer tuples)

// access/modify
let mut pair = (42, "abc".to_string());
pair.0 = 0;
pair.1 = "www".to_string();
let (x, _) = &mut pair;     // x: &mut i32
*x = 99;
let (_, ref mut s1) = pair;  // s1: &mut String  // ref keyword
*s1 = "qq".to_string();
```

</td>

<td>
    
```cpp
//// 1. Pair 
//    Note: std::pair is essentially a std::tuple with exactly 2 elements

// create
auto pair = std::make_pair(42, "hello");
std::pair<int, string> pair = {42, "hello"};

// destruct
const auto& [i, str] = pair;   // cpp17
const auto& [_, str] = pair;   // cpp17

// access/modify
auto x = pair.first;
auto y = pair.second;

//// 2. Tuple
// create
auto tuple = std::tuple {42, 3.14, "hello"};
auto tuple = std::make_tuple(42, 3.14, "hello");
std::tuple<int, double, string> tuple = {42, 3.14, "hello"};

// destruct
auto [i, d, str] = tuple;   // cpp17

// access/modify
auto str = get<2>(tuple);
auto str = get<string>(tuple); // cpp14: Get by type, not index
get<2>(tuple) = "abc";
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> if statement </td>

<td>

```rust

```

</td>

<td>
    
```cpp
// initializer in if statement (since cpp17):
const vector v = {1,2,3};
if (auto iter = ranges::find(v, 42); iter != v.end() ) { // cpp20 ranges
    print("found elem: "); println(*iter);
}
// or
if (auto iter = find(v.cbegin(), v.cend(), 2); iter != v.end()) {
    // same
}
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>

<td> for loop </td>

<td>

```rust

for e in vec { }
for e in &vec { } // loop by reference 
    // desugars into: (if vec is mut)
    let mut iter = vec.iter_mut(); 
        // iter: scoped mutable borrow
    while let Some(e) = iter.next() {
        *e *= 2;
    }


for i in 0..=5 { }

for i in (0..6).step_by(2) { }

for (index, item) in items.iter().enumerate() {
    println!("Item {index}: {item}");
}
```

</td>
<td>
    
```cpp
// range-based for loop - cpp11
for(int e : vec) { } // see vector for pre-cpp11 options

for (int i = 0; i < 5; ++i) { }

for (int i = 0; i < 6; i+=2) { }
```
</td>
</tr>


<!-- ----------------------------------------------------- -->


<tr>

<td> while loop </td>

<td>

```rust
// e.g. iter.next(): Option<&'a mut T>, e: &mut i32
let mut iter = vec.iter_mut();
while let Some(e) = iter.next() {
    *e += 1;
}

loop {
    // body
    if condition {
        break;
    }
}
```

</td>
<td>
    
```cpp
while (cntr < 10>) {
    cout << i << " ";
    ++cntr;
}   

do {
    // body
} while (condition);
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>

<td> Enums / Match / Switch </td>

<td>

```rust

enum Color { Red, Green, Blue }

#[repr(u8)]
enum Num { One = 1, Two = 2 }

let color = Color::Red;

match color {
    Color::Red =>   println!("Red"),
    Color::Green => println!("Green"),
    _ => println!("Neither Red nor Green")
}
```

</td>
<td>
    
```cpp
// both has, by default, int underlying type
// 1
enum Num { Zero, One };
enum Num { Ten = 10, Twenty = 20 };
// 2. strongly typed scoped enum (no implicit conversion to int)
enum class Num { Zero, One };
enum class Num: uint8_t { Zero, One };

auto num = Num::Zero;

switch (num) {
    case Color::Red:
        cout << "Red\n";
        break;
    case Color::Green:
        cout << "Green\n";
        break;
    case Color::Blue:
        cout << "Blue\n";
        break;
    default:
        cout << "Error";
}
```
</td>
</tr>






<!-- ----------------------------------------------------- -->
<tr>
<td> Class / Struct </td>

<td>

```rust
#[derive(Default, Debug)]
struct Person {
    name: String,
    age: i32
}

impl Person {
    fn age(&self) -> i32 { self.age }
    fn set_age(&mut self, new_age: i32) { self.age = new_age; }
        // auto-deref for field access: 
        // self.age auto becomes (*self).age 

    // x. Class associated fn
    fn type_name() -> &'static str { "Person" }
        // call: Person::type_name();
}

let person = Person::default();
let person = Person { name: "A".to_string(), age: 42 };
```

</td>

<td>
    
```cpp
class Person {
    private:
        string name;
        int age;

    public:
        Person(string name, int age)
            : name(move(name)), age(age) {}

        int get_age();
        void set_age(int new_age);
        
        // x. Static member fn
        static string type_name();
};

int Person::get_age() { return this->age; }
void Person::set_age(int new_age) { this->age = new_age; }
string Person::type_name() { return "Person"; };

Person person;
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> Class Access Specifiers </td>

<td>

```rust
fn f()             // Fully private   
pub fn f()         // Fully public
pub(crate) fn f()  // Visible only within the current crate.

pub(super) fn f()  // Visible to the parent module.
pub(in crate::utils::math) fn f() 
    // Visible only within a specific module path
```

</td>

<td>
    
```cpp
public    - accessible by class instances
private   - not accessible by class instances and child classes
protected - not accessible by class instances,
            can be accessed in child classes
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> Constructors </td>

<td>

```rust
todo
```

</td>

<td>
    
```cpp
// All ctors & dtor
class Person {
    private:
        string name;
        int age;

    public:
        // Default constructor
        Person() : name(""), age(0) {}

        // Parameterized constructor
        Person(string name, int age)
            : name(move(name)), age(age) {}

        // dtor
        ~Person() {}
        
        // copy ctor
        Person(const Person& rhs) 
            : name(rhs.name), age(rhs.age) { println("copy ctor"); };

        // move ctor
        Person(const Person&& rhs) 
            : name(move(rhs.name)), age(rhs.age) { println("move ctor"); };

        // copy assign operator
        Person& operator=(const Person& rhs) { 
            if (this != &rhs) {
                name = rhs.name;
                age = rhs.age;
            }
            return *this;
        };

        // move assign operator
        Person& operator=(const Person&& rhs) { 
            if (this != &rhs) {
                name = move(rhs.name);
                age = rhs.age;
            }
            return *this;
        };
};
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> Closure / Lambda  </td>

<td>

```rust
let s = "aa".to_string();

// 1.a Move capture: by default implicit Move 
let closure = |x: i32| s + "bb"; // s moved here, s does not impl Copy

// 1.b. Move capture: explicit Move 
// Use when: returned from a function or moved into a new thread
let closure = move |x: i32| s + "bb";

// 2. Capture by ref 
let closure = |x: i32| (&s).to_string() + "bb";

// 3. Copy Capture 
let closure = |x: i32| s.clone() + "bb";
```

</td>

<td>
    
```cpp
string s = "aa";

// 1. Copy capture: by default Copy 
auto lambda = [s](int x) { return s + "bb"; };

// 2. Capture by ref 
auto lambda = [&s](int x) { return s + "bb"; };

// 3. Move capture
auto f = [s = move(s)]() { return s + "bb"; };
// !! here, s is valid but has unspecified state/content,
//    usually empty but don’t expect its old contents to remain
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> Generics </td>

<td>

```rust
//// 1. Generic Fn w/ Trait bounds - these are all same:
fn foo_1<T: ToString>(arg: T) {
    println!("{}", arg.to_string());
}

fn foo_2(arg: impl ToString) {  // this is sugar for foo_1
    println!("{}", arg.to_string());
}

fn foo_3<T>(arg: T) -> i32 
where T: ToString {
    println!("{}", arg.to_string());
    42
}

//// 2. Generic class
struct Pair<T1, T2> {
    first: T1,
    second: T2
}

impl<T1: std::fmt::Display, T2> Pair<T1, T2> { // T1 is printable
    fn first(&self) -> &T1 { &self.first }
}

let pair = Pair { first: 1, second: 2 };
let pair = Pair::<u8, u8> { first: 1, second: 2 };
```

</td>

<td>
    
```cpp
//// 1. Generic fn
// a. After cpp20 Concepts:
template <typename T>
concept Addable = requires(T a, T b) {
    { a + b } -> std::same_as<T>;
};

template <Addable T>
T add(T a, T b) {
    return a + b;
}

// b. Before cpp20 Concepts:
template <typename T>
T add(T a, T b) {   // w/o concepts, this is duck typing
    return a + b; 
}

//// 2. Generic class
template <typename T1, typename T2>
class Pair {
public:
    void print_first() { cout << "first: " << this->first << endl; }
    Pair(T1 first, T2 second) : first(first), second(second) {} 

private:
    T1 first;
    T2 second;
};

auto pair = Pair(1, 2); 
Pair<int,int> pair = Pair(1, 2);
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> Error Handling </td>

<td>

```rust
let vec = vec![1,2,3];

// 1
if let Some(elem) = vec.get(10) {
    dbg!(elem);
} else { // optional 
    println!("err: index out of bounds")
}

// 2
match vec.get(10) {
    Some(elem) => { dbg!(*elem); } ,
    None => println!("err: index out of bounds")
}

// 3. Rarely used - not the same as exceptions
let res = std::panic::catch_unwind(|| {
    let elem = vec[10];
    dbg!(elem);
});

match res {
    Ok(_) => println!("No panic"),
    Err(e) => {
        println!("Caught a panic!");
        if let Some(msg) = e.downcast_ref::<&str>() {
            println!("msg: {msg}");
        }
    }
}

Cargo.toml:
[profile.dev]
panic = 'unwind' # default is 'abort'
```

</td>

<td>
    
```cpp
auto const vec = vector { 1,2,3 };

// a. try/catch basic
// E.18: Minimize the use of explicit try/catch.
//       Favor RAII and structured error handling over manual try/catch.
try {
    vec.at(10);
}
catch(std::exception const& err) { // or std::out_of_range
    cout << "err: " << err.what() << endl;
        // err: invalid vector subscript
}

// b. catch all
// only use  as a last resort, ideally with logging and re-throwing.
catch(...)
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> temp </td>

<td>

```rust

```

</td>

<td>
    
```cpp

```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> Question mark operator </td>

<td>

```rust
                // rustlib\src\rust\library\std\src\io\error.rs:
                // pub type Result<T> = result::Result<T, Error>;
                //                          |
fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;  // ? operator
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // ? operator
    Ok(contents)
}

// Equivalent Code Without Question Mark Operator:
//
// The ? operator:
// - Unwraps (not returns) the Ok value
// - "Returns" the Err early if the result is Err
fn read_file_contents_(path: &str) -> io::Result<String> {
    #[allow(clippy::question_mark)]
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),  // Early return on error
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),  // Return error (!! implicit return here)
    }
}
```

</td>

<td>
    
```cpp
N/A
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> new / delete </td>

<td>

```rust
N/A
```

</td>

<td>
    
```cpp
int* ptr = new int;
*ptr = 35;
delete ptr;

// For arrays: new[], delete[]
string* strs = new string[numStrs];
delete[] strs;
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>
<td> main fn </td>

<td>

```rust
fn main() {
    // 
}
```

</td>

<td>
    
```cpp
int main() {
  // 
  return 0;
}
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> Fn Default Parameter Value </td>

<td>

```rust
n/a
```

</td>

<td>
    
```cpp
void foo(string country = "Norway") {
  cout << country << "\n";
}
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> Function Overloading </td>

<td>

```rust
n/a
```

</td>

<td>
    
```cpp
// We can overload only by parameter types, constness for member fns
int f(int x)
// float f(int x) // err 
float f(float x)
double f(double x, double y)
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> Inheritance </td>

<td>

```rust
n\a
```

</td>

<td>
    
```cpp
// !!! C++ Core Guidelines
// - For code reuse: Prefer composition over inheritance.
// - Use inheritance only when it models an ‘is-a’ relationship 
//   and when you need polymorphic behavior.

// Base class
class Animal {
protected:
    void speak() { cout << "animal" << endl; }
};

// Child class
class Cat : public Animal {
public:
    void speak_public() {
        Animal::speak();
        speak();
    }
private:
    void speak() { cout << "cat" << endl; }
};
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> 1. Polymorphism - static/compile time dispatch </td>

<td>

```rust
// a. using generics + traits
struct Cat {}
impl Animal for Cat { fn speak(&self) { println!("cat"); } }

fn speak<T: Animal>(animal: &T) { // or: fn speak(animal: impl Animal)
    animal.speak(); // resolved at compile time
}

let cat = Cat {};
speak(&cat);

// b. using enum
enum Animal { Cat, Dog }

impl Animal {
    fn speak(&self) {
        match self {
            Self::Cat => println!("cat"),
            Self::Dog => println!("dog"),
        }
    }
}

fn speak(animal: &Animal) { animal.speak() }
let cat = Animal::Cat;
speak(&cat);
```

</td>

<td>
    
```cpp
// a. using template
class Cat {
public:
    void speak() const { cout << "cat" << endl; }
};

template <typename T>
void speak(const T& animal) { animal.speak(); }

// usage:
const Cat cat;
speak(cat);

// b. using Variant (similar to Rust's enum approach)
using Animal = std::variant<Dog, Cat>;

void speak(const Animal& animal) {
    auto action = [](const auto& animal) { animal.speak(); };
    std::visit(action, animal);
}

const Cat cat;
speak(cat);

// c. Inheritance
 (see Inheritance section)
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>
<td> 2. Polymorphism - dynamic/runtime dispatch </td>

<td>

```rust
trait Animal { fn speak() { println!("animal"); } }

struct Cat {}
impl Animal for Cat { fn speak( { println!("cat"); } ) }

struct Dog {}
impl Animal for Dog { fn speak( { println!("dog"); } ) }

//// usage
let cat: Box<dyn Animal> = Cat {};
let dog: Box<dyn Animal> = Dog {};

cat.speak(); // "cat"
dog.speak(); // "dog"

fn speak(animal: &dyn Animal) { animal.speak() }
    // animal.speak() is resolved at runtime via vtable
speak(&*cat);
speak(&*dog);
```
</td>

<td>
    
```cpp
class Animal {
public:
    virtual void speak() const { cout << "animal" << endl; }
    virtual ~Animal() { cout << "Animal dtor" << endl; }
        // If you don't use a virtual dtor in a polymorphic base class,
        // you risk undefined behavior when deleting derived objects 
        // through base pointer. See below:
        //
        // Animal* a = new Cat();
        // delete a; // ❌ Only Animal::~Animal() is called,
        //           // Cat::~Cat() is skipped !!!
};

class Cat : public Animal {
public:
    void speak() const override final { cout << "cat" << endl; }
        // final: can't be overridden again
};

//// Use
/// 1. modern cpp
unique_ptr<Animal> animal = make_unique<Cat>();
// or
const unique_ptr<const Animal> animal = make_unique<const Cat>();
animal->speak(); // "cat"

/// 2. raw ptrs
const Cat cat;
const Animal* animal_ptr = &cat;
animal_ptr->speak(); // "cat"
```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> Friend fn </td>

<td>

```rust
n/a
```

</td>

<td>
    
```cpp
// def:
class Person {
    public:
        friend int get_age(const Person& person);
}

int get_age(const Person& person) {
    return person.age * 10;
}

// use:
get_age(person);

```
</td>
</tr>




<!-- ----------------------------------------------------- -->
<tr>
<td> temp </td>

<td>

```rust

```

</td>

<td>
    
```cpp

```
</td>
</tr>



</table>


