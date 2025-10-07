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
        let mut iter = vec.iter_mut(); // iter: scoped mutable borrow
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
// 2. strongly typed scoped enum (no implicit conversion to int)
enum class Num { Zero, One };
enum class Num: uint8_t { Zero, One };

auto c = Num::Zero;

switch (c) {
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

let person = Person::default();

```

</td>

<td>
    
```cpp
class Person {
    string name;
    int age;
};

Person person;
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


</table>


