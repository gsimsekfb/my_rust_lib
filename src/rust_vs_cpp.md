# Rust vs C++

<table>

<tr>
<td> Topic </td> <td> Rust </td> <td> C++ </td>
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
let i = 42;
let ref_ = &i;
let ptr = &i as *const i32;

// deref
dbg!(ref_);
unsafe { dbg!(*ptr); }

// print ptr addr
dbg!(ptr);
```

</td>

<td>
    
```cpp
int i = 42;
int& ref = i;
int* ptr = &i;

// deref
cout << ref
cout << *ptr

// print ptr addr
cout << ptr
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
let v = vec![1,2,3];
let v: Vec<i32> = Vec::new();

// access
if let Some(value) = vec.get(1) { }

// add element
v.push(42)
```

</td>
<td>
    
```cpp
std::vector v = {1, 2, 3};
std::vector<int> v = {1, 2, 3};

// safe access: 
// (note: use vec[] if fast access needed)
try {
    int value = vec.at(0); // OK
} catch (const std::out_of_range& err) {
    std::cerr << "err: " << err.what() << "\n";
}

// add element
v.push_back(42)
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
std::array<int, 3> arr = {1, 2, 3};
// avoid raw arrays:
int arr[3] = {1,2,3}; // arr:0x7fff1b72911c
size_t arr_len = sizeof(arr) / sizeof(arr[0]); // 12/4=3

std::array<std::array<int,2>,2> arr = {{ {{1,2}}, {{3,4}} }};
int arr[3][2] = { {1,2}, {3,4}, {5,6} };
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
    // desugars into:
    {
        let mut iter = vec.iter_mut(); 
            // iter: scoped mutable borrow
        while let Some(e) = iter.next() {
            *e *= 2;
        }
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

for(int e : vec) { } 

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
    std::cout << i << " ";
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
        std::cout << "Red\n";
        break;
    case Color::Green:
        std::cout << "Green\n";
        break;
    case Color::Blue:
        std::cout << "Blue\n";
        break;
    default:
        std::cout << "Error";
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
public - members are accessible from outside the class
private - members cannot be accessed from outside the class
protected - members cannot be accessed from outside the class, 
            however, they can be accessed in inherited classes.
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
    // ...
}
```

</td>

<td>
    
```cpp
int main() {
  // ...
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
// We can overload only by parameter types
int f(int x)
// float f(int x) // err 
float f(float x)
double f(double x, double y)
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


