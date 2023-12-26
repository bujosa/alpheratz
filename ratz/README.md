# Ratz (TODO)

## Description

Ratz is a simple derive macro, only for learning purposes.

## Theory of Macro

### What is a macro?

A macro is a piece of code that is inserted into your code during compilation. It is a metaprogramming technique.

### Why use a macro?

Macros are used to generate repetitive code, or to generate code based on some input.

### How to use a macro?

Macros are used by adding the `#[macro_use]` attribute to the module where the macro is defined. Then, the macro can be used in any module of the crate.

### How to define a macro?

Macros are defined using the `macro_rules!` macro. It takes a pattern and a template. The pattern is matched against the macro invocation, and the template is the code that is generated.

### Type of macros

There are two types of macros: declarative macros with `macro_rules!` and procedural macros.

#### Declarative macros

Declarative macros are defined with `macro_rules!`. They are also called "macros by example" or "macro_rules! macros". They are the simplest type of macros.

#### Procedural macros

Procedural macros are defined by a function that takes a token stream as input and returns a token stream as output. They are more powerful than declarative macros, but also more complex.

### Examples of macros

1. String Macros: These are not a specific type of macro in Rust, but you can certainly create macros that work with strings. Here's an example:
    ```rust
    macro_rules! hello {
        ($name:expr) => {
            format!("Hello, {}!", $name)
        };
    }

    fn main() {
        println!("{}", hello!("world"));
    }
    ```
2. Derive Macros: These are used to generate code based on some input. Here's an example:
    ```rust
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    fn main() {
        let person = Person {
            name: String::from("Alice"),
            age: 20,
        };

        println!("{:?}", person);
    }
    ```

3. Attribute Macros: These are used to apply metadata to some Rust construct. Here's an example:
    ```rust
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    ```

4. Function-like Macros: These are used to generate code based on some input. Here's an example:
    ```rust
    macro_rules! say_hello {
        () => {
            println!("Hello!");
        };
    }

    fn main() {
        say_hello!();
    }
    ```

    ```rust
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    fn main() {
        let v = vec![1, 2, 3];
        println!("{:?}", v);
    }
    ```

