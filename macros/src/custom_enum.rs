use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error};

use crate::Args;

pub fn expand(ast: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(data) = &ast.data else {
        return Err(Error::new(ast.ident.span(), "Expected a struct or enum"));
    };

    let args: Args = FromDeriveInput::from_derive_input(&ast).unwrap();
    let orig_ident = &ast.ident;
    let name: TokenStream = match &args.enum_name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#orig_ident) },
    };

    let mut enum_variants: Vec<TokenStream> = Vec::new();
    for variant in data.variants.iter() {
        let variant_ident = &variant.ident;
        let variant_value = quote! { stringify!(#variant_ident )};

        enum_variants.push(quote! {
            .item(#variant_value)
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics seaography::CustomEnum for #orig_ident  #ty_generics #where_clause {
            fn to_enum() -> async_graphql::dynamic::Enum {
                async_graphql::dynamic::Enum::new(#name)
                #(#enum_variants)*
            }
        }
    })
}
