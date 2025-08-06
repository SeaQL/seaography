extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

mod custom_input;
mod custom_operation;
mod util;

#[proc_macro_derive(CustomOperation, attributes(seaography))]
pub fn custom_operation(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    match custom_operation::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(CustomInput, attributes(seaography))]
pub fn custom_input(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    match custom_input::expand(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
