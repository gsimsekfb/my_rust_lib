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
vector v = {1, 2, 3};
vector<int> v = {1, 2, 3};

// safe access: 
// (note: use vec[] if fast access needed)
try {
    int value = vec.at(0); // OK
} catch (const out_of_range& err) {
    cerr << "err: " << err.what() << "\n";
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
// a. After C++20 Concepts:
template <typename T>
concept Addable = requires(T a, T b) {
    { a + b } -> std::same_as<T>;
};

template <Addable T>
T add(T a, T b) {
    return a + b;
}

// b. Before C++20 Concepts:
template <typename T>
T add(T a, T b) {   // w/o concept, this is duck typing
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
... (see Inheritance section)
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


