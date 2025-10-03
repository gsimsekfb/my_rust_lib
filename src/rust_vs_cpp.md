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
```

</td>
<td>
    
```cpp
std::vector v = {1, 2, 3};
std::vector<int> v = {1, 2, 3};
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
```
</td>
</tr>


<!-- ----------------------------------------------------- -->
<tr>

<td> for loop </td>

<td>

```rust

for e in vec { }

for i in 0..=5 { }

for i in (0..6).step_by(2) { }

for (index, item) in items.iter().enumerate() {
    println!("Item {index}: {item}");
}
```

</td>
<td>
    
```cpp

for(int i : vec) { } 

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
    if !condition {
        break;
    }
}
```

</td>
<td>
    
```cpp
do {
    // body
} while (condition);
```
</td>
</tr>



<!-- ----------------------------------------------------- -->
<tr>

<td> Match / Switch </td>

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
enum class Color { Red, Green, Blue };
Color c = Color::Green;

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


</table>