#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(non_upper_case_globals)]

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, FnArg, Ident, ImplItem,
    ItemImpl, PathArguments, ReturnType, Signature, Type, TypeReference, parse_macro_input,
    spanned::Spanned,
};

use crate::{
    Args,
    util::{EnumVariants, parse_enum_variants},
};

pub fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    let args: Args = FromDeriveInput::from_derive_input(&derive_input).unwrap();
    let ident = &derive_input.ident;
    match &derive_input.data {
        Data::Struct(data_struct) => {
            let name: TokenStream = match &args.input_type_name {
                Some(name) => quote! { #name },
                None => {
                    let name = format!("{}Input", ident);
                    quote! { #name }
                }
            };

            derive_custom_input_type_struct(&derive_input, data_struct, name)
        }
        Data::Enum(data_enum) => {
            let name: TokenStream = match &args.input_type_name {
                Some(name) => quote! { #name },
                None => quote! { stringify!(#ident) },
            };

            derive_custom_input_type_enum(&derive_input, data_enum, name)
        }
        Data::Union(_) => Err(Error::new(
            derive_input.ident.span(),
            "Expected a struct or enum",
        )),
    }
}

fn derive_custom_input_type_struct(
    ast: &DeriveInput,
    data: &DataStruct,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let Fields::Named(named) = &data.fields else {
        return Err(Error::new(ast.ident.span(), "Expected named fields"));
    };

    let fields: Vec<Field> = named.named.clone().into_iter().collect();
    let mut resolve_args: Vec<TokenStream> = Vec::new();
    let mut dynamic_fields: Vec<TokenStream> = Vec::new();

    for field in fields.iter() {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        resolve_args.push(quote! {
            #field_ident: <#field_ty>::parse_value(
                context,
                input_object.get(stringify!(#field_ident))
            )?,
        });

        dynamic_fields.push(quote! {
            .field(async_graphql::dynamic::InputValue::new(
                stringify!(#field_ident),
                <#field_ty>::gql_input_type_ref(context),
            ))
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics seaography::CustomInputType for #orig_ident #ty_generics #where_clause {
            fn gql_input_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named_nn(#name)
            }

            fn parse_value(
                context: &'static seaography::BuilderContext,
                value: Option<async_graphql::dynamic::ValueAccessor<'_>>,
            ) -> seaography::SeaResult<Self> {
                let input = value.ok_or(
                    seaography::SeaographyError::AsyncGraphQLError("Expected a value".into())
                )?;
                let input_object = input.object()?;
                Ok(Self {
                    #(#resolve_args)*
                })
            }
        }

        impl #impl_generics seaography::CustomInputObject for #orig_ident #ty_generics #where_clause {
            fn input_object(
                context: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::InputObject {
                use seaography::CustomInputType;
                async_graphql::dynamic::InputObject::new(#name)
                #(#dynamic_fields)*
            }
        }
    })
}

fn derive_custom_input_type_enum(
    ast: &DeriveInput,
    data: &DataEnum,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let variants = parse_enum_variants(ast, data)?;
    match variants {
        EnumVariants::Units(variants) => derive_custom_input_type_enum_units(ast, variants, name),
        EnumVariants::Containers(variants) => {
            derive_custom_input_type_enum_containers(ast, variants, name)
        }
    }
}

fn derive_custom_input_type_enum_units(
    ast: &DeriveInput,
    variants: Vec<Ident>,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let mut variant_matches: Vec<TokenStream> = Vec::new();
    for variant_ident in variants.iter() {
        let variant_value = quote! { stringify!(#variant_ident)};
        variant_matches.push(quote! {
            #variant_value => Ok(Self::#variant_ident),
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics seaography::CustomInputType for #orig_ident #ty_generics #where_clause {
            fn gql_input_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named_nn(#name)
            }

            fn parse_value(
                context: &'static seaography::BuilderContext,
                value: Option<async_graphql::dynamic::ValueAccessor<'_>>,
            ) -> seaography::SeaResult<Self> {
                let value = value.ok_or(
                    seaography::SeaographyError::AsyncGraphQLError("Expected a value".into())
                )?;
                let enum_name = value.enum_name()?;
                match enum_name {
                    #(#variant_matches)*
                    _ => Err(seaography::SeaographyError::AsyncGraphQLError("Unknown enum value".into())),
                }
            }
        }
    })
}

fn derive_custom_input_type_enum_containers(
    ast: &DeriveInput,
    variants: Vec<Ident>,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let mut variant_matches: Vec<TokenStream> = Vec::new();
    let mut input_object_fields: Vec<TokenStream> = Vec::new();
    for variant_ident in variants.iter() {
        let variant_value = quote! { stringify!(#variant_ident) };

        variant_matches.push(quote! {
            if let Some(inner) = obj.get(#variant_value) {
                return #variant_ident::parse_value(context, Some(inner))
                    .map(#orig_ident::#variant_ident);
            }
        });

        input_object_fields.push(quote! {
            .field(async_graphql::dynamic::InputValue::new(
                #variant_value,
                <Option<#variant_ident>>::gql_input_type_ref(context),
            ))
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics seaography::CustomInputType for #orig_ident #ty_generics #where_clause {
            fn gql_input_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named_nn(format!("{}Input", #name))
            }

            fn parse_value(
                context: &'static seaography::BuilderContext,
                value: Option<async_graphql::dynamic::ValueAccessor<'_>>,
            ) -> seaography::SeaResult<Self> {
                let Some(value) = value else {
                    return Err(seaography::SeaographyError::AsyncGraphQLError("Value expected".into()));
                };
                let obj = value.object()?;

                #(#variant_matches)*

                Err(seaography::SeaographyError::AsyncGraphQLError(
                    format!("Unknown {} variant", stringify!(#orig_ident)).into(),
                ))
            }
        }

        impl #impl_generics seaography::CustomInputObject for #orig_ident #ty_generics #where_clause {
            fn input_object(
                context: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::InputObject {
                async_graphql::dynamic::InputObject::new(format!("{}Input", #name))
                #(#input_object_fields)*
            }
        }
    })
}
