#![allow(clippy::collapsible_if)]
extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, ItemImpl};

mod convert_output;
mod custom_enum;
mod custom_fields;
mod custom_input_type;
mod custom_output_type;
mod util;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(seaography))]
struct Args {
    input_type_name: Option<String>,
    output_type_name: Option<String>,
    enum_name: Option<String>,
    #[darling(default)]
    custom_fields: bool,
}

#[proc_macro_derive(CustomEnum, attributes(seaography))]
pub fn derive_custom_enum(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    match custom_enum::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(CustomInputType, attributes(seaography))]
pub fn derive_custom_input_type(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    match custom_input_type::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(CustomOutputType, attributes(seaography))]
pub fn derive_custom_output_type(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    match custom_output_type::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn CustomFields(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let derive_input: ItemImpl = syn::parse(annotated_item.clone()).unwrap();
    match custom_fields::expand(derive_input, annotated_item) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(ConvertOutput, attributes(seaography))]
pub fn derive_convert_output(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    match convert_output::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
