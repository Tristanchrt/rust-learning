# Rust learning

## Doc

Rust's official documentation is an invaluable resource for learning the language.
The Rust Book provides comprehensive and beginner-friendly explanations of Rust concepts.

[https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html]())

https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/getting-started.html

## Cargo CLI

> cargo --version
> cargo build
> cargo build --release
> cargo run
> cargo check
> cargo clean
> cargo test
> cargo update
> cargo bench
> cargo doc --open
> cargo new restaurant --lib

## Cargo Test

> cargo test -- --test-threads=1
> cargo test -- --show-output
> cargo test greater_than_100
> cargo test -- --ignored
> cargo test --test integration_test

## Ownership, Borrowing and Slice type

### Ownership
In Rust, ownership rules help manage memory efficiently without a garbage collector:

Ownership Rules:

- Each value in Rust has a variable that's its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Borrowing
Borrowing allows passing references to values without transferring ownership:

- References: Allow you to refer to a value without taking ownership.
- Mutable References: Enable mutable access to a value.

### Slice Type

Slices are a view into a sequence of elements in a collection:

- String Slices: Represents a portion of a String.
- Array Slices: Allow access to a portion of an array.

## Structs

### Structs in Rust allow you to create custom data types:

```rs
struct Person {
    name: String,
    age: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} is {} years old.", person1.name, person1.age);
}

let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

## Enums (Option, Match, if let)

### Option Enum

- Option: An enum type that represents either Some(T) or None, where T is the value or absence of a value, respectively.
- Used for: Handling optional values and reducing the need for null checks.

```rs
fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let target = 30;

    match find_element(&numbers, target) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}
```

### Match with Enums

- Match: Allows pattern matching against enum variants to execute code based on different conditions.

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 0
    }
}
fn main() {
    let coin = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("Value of the coin: {} cents", value);
}
```

### if let Expressions
if let: Provides a concise way to handle a specific variant of an enum.

```rs
enum Result {
    Success(i32),
    Failure(String),
}
fn main() {
    let result = Result::Success(42);

    if let Result::Success(value) = result {
        println!("Result is {}", value);
    } else {
        println!("It's a failure");
    }
}
```

## Packages, Crates and Modules

Rust's module system organizes code into reusable units:

- Packages: A Cargo feature that can contain one or more crates.
- Crates: A collection of Rust source code files.
- Modules: Used to control the organization, scope, and privacy of items like functions, structs, and enums.

```rs
// File: src/lib.rs (a part of a crate)

mod utils { // Define a module named 'utils'
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

pub mod math { // Define a public module named 'math'
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}
```

## HashMap, Vectors and String

### HashMaps in Rust store key-value pairs:

```rs
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 80);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
}
```

### Vectors are dynamically sized arrays:

```rs
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5]; // Creating a vector

    numbers.push(6); // Add an element to the vector

    println!("Third element: {}", numbers[2]); // Output: Third element: 3
}
```

### Strings in Rust represent UTF-8 encoded text:

```rs
fn main() {
    let mut s = String::from("Hello, ");
    s.push_str("world!");

    println!("{}", s); // Output: Hello, world!
}
```

## Error Handling (Panic!, Result)

### Panic!

- Panic: Represents an unrecoverable error and causes the program to stop execution.
- panic!(): Macro used to generate a panic.

```rs
fn main() {
    let divisor = 0;

    if divisor == 0 {
        panic!("Cannot divide by zero!"); // Generates a panic and terminates the program
    }

    let result = 10 / divisor;
    println!("Result: {}", result); // This line won't be reached due to panic
}
```

### Result Type

- Result: A type that represents either success (Ok) or failure (Err).
- Handling Results: Use match or combinators like unwrap() or expect() to handle Result values.

```rs
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero!"));
    }

    Ok(a / b)
}

fn main() {
    let dividend = 10;
    let divisor = 0;

    let result = divide(dividend, divisor);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
```

### Error Propagation with ?

- The ? operator can be used to propagate errors easily within functions that return Result.
- It either returns the value inside Ok or propagates Err early in the function.

```rs
fn open_file(file_name: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_name)?;
    Ok(content)
}

fn main() -> Result<(), std::io::Error> {
    let file_content = open_file("example.txt")?;
    println!("File content: {}", file_content);
    Ok(())
}
```

### Debug

```rs
#[derive(Debug)] // Debug
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // Debug
}
```
