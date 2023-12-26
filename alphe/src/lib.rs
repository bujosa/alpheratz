extern crate proc_macro;

use proc_macro::*;

// use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, ItemFn};

#[proc_macro_attribute]
pub fn validate_name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    // Parse the validation logic as a block of statements
    let validation: Block = syn::parse_quote!({
        let mut person_clone = person.clone();
        let person = person_clone.into_inner();
        if person.name.len() < 5 {
            return rocket::http::Status::BadRequest;
        }
        let person = rocket::serde::json::Json(person);
    });

    // Prepend the validation logic to the original function body
    let original_body = function.block;
    function.block = syn::parse_quote!({
        #validation
        #original_body
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}

#[proc_macro_attribute]
pub fn validate_lastname(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    // Parse the validation logic as a block of statements
    let validation: Block = syn::parse_quote!({
        let mut person_clone = person.clone();
        let person = person_clone.into_inner();
        if person.lastname.len() < 5 {
            return rocket::http::Status::BadRequest;
        }
        let person = rocket::serde::json::Json(person);
    });

    // Prepend the validation logic to the original function body
    let original_body = function.block;
    function.block = syn::parse_quote!({
        #validation
        #original_body
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}

#[proc_macro_attribute]
pub fn validate_email(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    // Parse the validation logic as a block of statements
    let validation: Block = syn::parse_quote!({
        let mut person_clone = person.clone();
        let person = person_clone.into_inner();
        if !person.email.contains("@") {
            return rocket::http::Status::BadRequest;
        }
        let person = rocket::serde::json::Json(person);
    });

    // Prepend the validation logic to the original function body
    let original_body = function.block;
    function.block = syn::parse_quote!({
        #validation
        #original_body
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}

#[proc_macro_attribute]
pub fn validate_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    // Parse the validation logic as a block of statements
    let validation: Block = syn::parse_quote!({
        let mut person_clone = person.clone();
        let person = person_clone.into_inner();
        let mut errors = Vec::new();
        if person.name.len() < 5 {
            errors.push(String::from("Name must be at least 5 characters long"));
        }
        if person.lastname.len() < 5 {
            errors.push(String::from("Lastname must be at least 5 characters long"));
        }
        if !person.email.contains("@") {
            errors.push(String::from("Email must contain @"));
        }
        if !errors.is_empty() {
            return Err(rocket::response::status::Custom(
                rocket::http::Status::BadRequest,
                rocket::serde::json::Json(errors),
            ));
        }
        let person = rocket::serde::json::Json(person);
    });

    // Prepend the validation logic to the original function body
    let original_body = function.block;
    function.block = syn::parse_quote!({
        #validation
        #original_body
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}

#[proc_macro_attribute]
pub fn add_updated_at_field(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    // Parse the validation logic as a block of statements
    let validation: Block = syn::parse_quote!({
        person.updated_at = Some(String::from("2021-09-01"));
    });


    // Prepend the validation logic to the original function body
    let original_body = function.block;
    function.block = syn::parse_quote!({
        #validation
        #original_body
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}
