use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident};

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
        Data::Struct(data_struct) => {
            derive_custom_output_type_struct(&derive_input, data_struct, &args, name)
        }
        Data::Enum(data_enum) => derive_custom_output_type_enum(&derive_input, data_enum, name),
        Data::Union(_) => Err(Error::new(
            derive_input.ident.span(),
            "Expected a struct or enum",
        )),
    }
}

fn derive_custom_output_type_struct(
    ast: &DeriveInput,
    data: &DataStruct,
    args: &Args,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Fields::Named(named) = &data.fields else {
        return Err(Error::new(ast.ident.span(), "Expected named fields"));
    };

    let mut fields: Vec<TokenStream> = Vec::new();

    for field in named.named.iter() {
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        fields.push(quote! {
            .field(::async_graphql::dynamic::Field::new(
                stringify!(#field_ident),
                <#field_ty as ::seaography::CustomOutputType>::gql_output_type_ref(context),
                move |ctx| {
                    ::async_graphql::dynamic::FieldFuture::new(async move {
                        let obj = ::seaography::try_downcast_ref::<#orig_ident #ty_generics>(ctx.parent_value)?;
                        Ok(<#field_ty as ::seaography::CustomOutputType>::gql_field_value(
                            obj.#field_ident.clone()
                        ))
                    })
                }))
        });
    }

    let mut object_def: TokenStream = quote! {
        ::async_graphql::dynamic::Object::new(#name)
        #(#fields)*
    };

    if args.custom_fields {
        object_def = quote! {
            let mut obj = #object_def;
            for field in <Self as ::seaography::CustomFields>::to_fields(context) {
                obj = obj.field(field);
            }
            obj
        }
    }

    Ok(quote! {
        impl #impl_generics ::seaography::CustomOutputType for #orig_ident #ty_generics #where_clause {
            fn gql_output_type_ref(
                ctx: &'static ::seaography::BuilderContext,
            ) -> ::async_graphql::dynamic::TypeRef {
                ::async_graphql::dynamic::TypeRef::named_nn(#name)
            }

            fn gql_field_value(
                value: Self,
            ) -> Option<::async_graphql::dynamic::FieldValue<'static>> {
                Some(::async_graphql::dynamic::FieldValue::owned_any(value))
            }
        }

        impl #impl_generics ::seaography::CustomOutput for #orig_ident #ty_generics #where_clause {
            fn basic_object(
                context: &'static ::seaography::BuilderContext,
            ) -> ::async_graphql::dynamic::Object {
                #object_def
            }
        }
    })
}

fn derive_custom_output_type_enum(
    ast: &DeriveInput,
    data: &DataEnum,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let variants = parse_enum_variants(ast, data)?;
    match variants {
        EnumVariants::Units(variants) => derive_custom_output_type_enum_units(ast, variants, name),
        EnumVariants::Containers(variants) => {
            derive_custom_output_type_enum_containers(ast, variants, name)
        }
    }
}

fn derive_custom_output_type_enum_units(
    ast: &DeriveInput,
    variants: Vec<Ident>,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let mut variants_gql_field_value: Vec<TokenStream> = Vec::new();
    for variant_ident in variants.iter() {
        let variant_value = quote! { stringify!(#variant_ident )};

        variants_gql_field_value.push(quote! {
            Self::#variant_ident => Some(::async_graphql::dynamic::FieldValue::value(
                ::async_graphql::Value::Enum(::async_graphql::Name::new(#variant_value))
            )),
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::seaography::CustomOutputType for #orig_ident #ty_generics #where_clause {
            fn gql_output_type_ref(
                ctx: &'static ::seaography::BuilderContext,
            ) -> ::async_graphql::dynamic::TypeRef {
                ::async_graphql::dynamic::TypeRef::named_nn(#name)
            }

            fn gql_field_value(
                value: Self,
            ) -> Option<::async_graphql::dynamic::FieldValue<'static>> {
                match value {
                    #(#variants_gql_field_value)*
                }
            }
        }
    })
}

fn derive_custom_output_type_enum_containers(
    ast: &DeriveInput,
    variants: Vec<Ident>,
    _name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &ast.ident;

    let mut possible_types: Vec<TokenStream> = Vec::new();
    let mut variant_matches: Vec<TokenStream> = Vec::new();
    for variant_ident in variants.iter() {
        let variant_value = quote! { stringify!(#variant_ident )};

        possible_types.push(quote! {
            .possible_type(#variant_value)
        });

        variant_matches.push(quote! {
            #orig_ident::#variant_ident(inner) => Ok(Some(
                ::async_graphql::dynamic::FieldValue::owned_any(inner)
                    .with_type(#variant_value),
            )),
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::seaography::CustomUnion for #orig_ident #ty_generics #where_clause {
            fn to_union() -> ::async_graphql::dynamic::Union {
                ::async_graphql::dynamic::Union::new(stringify!(#orig_ident))
                    #(#possible_types)*
            }
        }

        impl #impl_generics ::seaography::ConvertOutput for #orig_ident #ty_generics #where_clause {
            fn convert_output(
                value: &::sea_orm::sea_query::Value,
            ) -> ::async_graphql::Result<Option<::async_graphql::dynamic::FieldValue<'static>>> {
                if let ::sea_orm::sea_query::value::Value::Json(opt_json) = value {
                    if let Some(json) = opt_json {
                        match ::serde_json::from_value::<#orig_ident>(*json.clone()) {
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
