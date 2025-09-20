use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Error, Ident};

use crate::{
    Args,
    util::{EnumVariants, parse_enum_variants},
};

pub fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    let args: Args = FromDeriveInput::from_derive_input(&derive_input).unwrap();
    let orig_ident = &derive_input.ident;
    let name: TokenStream = match &args.output_type_name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#orig_ident) },
    };
    match &derive_input.data {
        Data::Enum(data_enum) => derive_custom_output_type_enum(&derive_input, data_enum, name),
        _ => Err(Error::new(derive_input.ident.span(), "Expected a enum")),
    }
}

fn derive_custom_output_type_enum(
    ast: &DeriveInput,
    data: &DataEnum,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let variants = parse_enum_variants(ast, data)?;
    match variants {
        EnumVariants::Units(_) => {
            return Err(Error::new(ast.ident.span(), "Expected a container enum"));
        }
        EnumVariants::Containers(variants) => {
            derive_convert_output_enum_containers(ast, variants, name)
        }
    }
}

fn derive_convert_output_enum_containers(
    ast: &DeriveInput,
    variants: Vec<Ident>,
    _name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let mut possible_types: Vec<TokenStream> = Vec::new();
    let mut variant_matches: Vec<TokenStream> = Vec::new();
    let mut field_value_matches: Vec<TokenStream> = Vec::new();
    for variant_ident in variants.iter() {
        let variant_value = quote! { stringify!(#variant_ident )};

        possible_types.push(quote! {
            .possible_type(#variant_value)
        });

        variant_matches.push(quote! {
            #orig_ident::#variant_ident(inner) => Ok(Some(
                async_graphql::dynamic::FieldValue::owned_any(inner)
                    .with_type(#variant_value),
            )),
        });

        field_value_matches.push(quote! {
            #orig_ident::#variant_ident(inner) => Some(
                async_graphql::dynamic::FieldValue::owned_any(inner)
                    .with_type(#variant_value),
            ),
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics seaography::ConvertOutput for #orig_ident #ty_generics #where_clause {
            fn convert_output(
                value: &sea_orm::sea_query::Value,
            ) -> async_graphql::Result<Option<async_graphql::dynamic::FieldValue<'static>>> {
                if let sea_orm::sea_query::value::Value::Json(opt_json) = value {
                    if let Some(json) = opt_json {
                        match serde_json::from_value::<#orig_ident>(*json.clone()) {
                            Ok(obj) => match obj {
                                #(#variant_matches)*
                            },
                            Err(e) => Err(e.into()),
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(format!("Expected JSON, got {:?}", value).into())
                }
            }
        }
    })
}
