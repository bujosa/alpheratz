# alpheratz
This repository implements JSON interceptors in Rust using Macros. One of the key features is the `validate_name` macro.

## validate_name Macro
The `validate_name` macro is a procedural macro that validates the name field of a JSON object. It checks if the length of the name is less than 5 characters and returns a `BadRequest` status if the condition is met.

Here is how you can use it:
```rust
#[alphe::validate_name]
#[post("/", format = "json", data = "<person>")]
fn create_person(person: Json<Person>, db: &State<Database>) -> Status {
    // Your function implementation here
}
```

In the above code, the `validate_name` macro is applied to the `create_person` function. When a POST request is made to the root ("/") route with a JSON payload, the `validate_name` macro intercepts the request, validates the name field of the JSON object, and returns a `BadRequest` status if the name is less than 5 characters long.

## Macro Implementation

The `validate_name` macro is implemented using the proc_macro_attribute attribute. It takes two arguments: `_attr` and `item. _attr` is the attribute input and `item` is the annotated item.

The macro first parses the `item` as a function. It then defines a block of validation logic that clones the `person` object, checks the length of the name, and returns a `BadRequest` status if the name is less than 5 characters long.

The macro then prepends the validation logic to the original function body and returns the modified function.

## How I create procedural macros

Here's a brief overview of the steps to create a procedural macro:
1. Create a new library crate for the macro with `cargo new --lib alphe`.
2. Navigate into the new crate directory with `cd alphe`.
3. Add the necessary dependencies for creating procedural macros to the `Cargo.toml` file. This usually includes the `syn` and `quote` crates.
4. Define your procedural macro in the `src/lib.rs` file. Procedural macros are defined as functions with the `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]` attribute.
5. Use your procedural macro in another crate by adding the macro crate as a dependency and importing the macro with `use`.