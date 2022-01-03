Rust notes
==================================


## From JS to Rust

### Package manager

* Cargo is NPM.
* ![crates.io] is the default package repository
* `use` key word is equivalent to import :
```rust
use web_sys::console;
```
* `pub` key is equivalent to export:
```rust
pub fn hello_world() {...}
```

* console.log
```rust
use log::Level;
use log::info;
fn main() {
    console_log::init_with_level(Level::Debug);
    debug!("It works !"); // console.log()
    info!("It works !"); // console.info()
}
```

* Iteration

```rust
let staff = [
    { name: "G", amount: 0},
    { name: "F", amount: 5}
]
let salary = 1000
staff.forEach(s => {s.amount += salary })
```

```rust
staff.iter_mut().for_each(
    | s | { employee.amount += salary; }
)
```

* Destructuring

```rust
let point = { x: 3, y: 4}
let {x,y} = point
```

```rust
let point = Point { x: 3, y: 4};
let Point {x,y} = point
```

### `match` replaces switch case

```rust
match x {
    1 => { /* do something is x == 1 */},
    3 | 4 => { /* ... */ },
    5...10 => { /* ... */ },
    _ => { /* ... */ }
}
```

## Into and unwrap

- Into is a trait generally used for data conversion conversion (struct)
- unwrap gets the value of an Option


### Into example

```rust
#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Employee {
    name: String
}

impl Into<Employee> for Person {
    fn into(self) -> Employee {
        Employee {
            name: self.name
        }
    }
}

fn main() {
    let person: Person = Person { name: "toto".to_string()};
    let employee: Employee = person.into();

    println!("{}", employee.name)
}
```

### unwrap example

```rust
fn get_even_numbers(v: Vec<i32>) -> Option<Vec<i32>> {
    let mut res: Vec<i32> = Vec::new();
    for i in v.into_iter() {
        if i % 2 == 0 {
            res.push(i.clone())
        }
    }

    if res.len() > 0 {
        Some(res)
    } else {
        None
    }
}

fn main() {
    let vec_option1 = get_even_numbers(vec![1,2,3,4]);
    println!("{:?}", vec_option1.unwrap());

    let vec_option2 = get_even_numbers(vec![1,5,6,7]);
    println!("{:?}", vec_option2.unwrap());
}
```

## LLVM
Low level virtual machine but it is not a virtual machine but a set of tools for compiling.


## Move semantics


### Affine type system

* affine resource = resource that can only be used once
* linear resource = resource that must be used once

### Moving an item in rust transfers the ownership

```rust
pub struct Foo {
    value: u8
}
fn main() {
    let foo = Foo { value: 42 };
    let bar = foo;

    println!("{}", foo.value); // error : use of moved value: `foo.value`
    println!("{}", bar.value);
}
```

Another example with functions

```rust
    // Transfer ownership to the callee
    fn do_something(foo: Foo) {}
```

```rust
    // Transfer ownership to the caller
    fn make_a_foo() -> Foo {
        Foo {
            value: 42
        }
    }
```

Move semantics is also applied in C++. But rust prevent also disables running a destructor on this variable, while in C++ it can keeps running.

Behind the scene move is a memcpy.


## Expose a wrapper function to javascript

* import wasm-bindgen in the dependencies

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen] // compiler will expose a function to JS
pub fn hello_world() {
    ...
}
```


### Kinds of procedural macros

- Custom #[derive] macros: used on struct and enum
- Attribute-like: define custom attributes usable on any item
- Function-like: look like function calls but operate on tokens speficied as argument