# alpheratz
This repository implements JSON interceptors in Rust using Macros. One of the key features is the `validate_name` macro.

## Macros

### `validate_name`
This macro validates that the `name` field of a `Person` is at least 5 characters long. If the validation fails, it returns a `BadRequest` status.

Here is how you can use it:
```rust
#[alphe::validate_name]
#[post("/", format = "json", data = "<person>")]
fn create_person(person: Json<Person>, db: &State<Database>) -> CustomResponse {
    // Your function implementation here
}
```

### `validate_lastname`
This macro validates that the `lastname` field of a `Person` is at least 5 characters long. If the validation fails, it returns a `BadRequest` status.

### `validate_email`
This macro validates that the `email` field of a `Person` contains an "@" character. If the validation fails, it returns a `BadRequest` status.

### `validate_fields`
This macro validates the `name`, `lastname`, and `email` fields of a `Person` using the `validate_name`, `validate_lastname`, and `validate_email` macros. If any validation fails, it returns a `BadRequest` status with a JSON array of error messages.

### `add_updated_at_field`
This macro adds an `updated_at` field to a `Person` with a value of "2021-09-01". It then returns the updated `Person`.

## validate_name Macro
The `validate_name` macro is a procedural macro that validates the name field of a JSON object. It checks if the length of the name is less than 5 characters and returns a `BadRequest` status if the condition is met.


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


## Documentation

- [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [Attribute Macros](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
- [The little book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
