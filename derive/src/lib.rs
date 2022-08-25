use quote::ToTokens;
use syn::DeriveInput;

mod error;
mod filter;
mod relation;
mod root_query;
mod enumeration;

#[proc_macro_derive(Filter, attributes(sea_orm))]
pub fn derive_filter_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let item = match data {
        syn::Data::Struct(item) => item,
        _ => {
            return quote::quote! {
                compile_error!("Input not structure")
            }
            .into()
        }
    };

    if ident.ne("Model") {
        return quote::quote! {
            compile_error!("Struct must be SeaOrm Model structure")
        }
        .into();
    }

    let attrs = filter::SeaOrm::from_attributes(&attrs).unwrap();

    filter::filter_fn(item, attrs)
        .unwrap_or_else(|err| {
            let error = format!("{:?}", err);

            quote::quote! {
                compile_error!(#error)
            }
        })
        .into()
}

// TODO use attrs to skip relations
#[proc_macro_derive(RelationsCompact, attributes(sea_orm))]
pub fn derive_relations_compact_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse_macro_input!(input as syn::DeriveInput);

    let item = match data {
        syn::Data::Enum(item) => item,
        _ => return quote::quote! { compile_error!("Input not enumeration") }.into(),
    };

    if ident.ne("Relation") {
        return quote::quote! {
            compile_error!("Struct must be SeaOrm Relation enumeration")
        }
        .into();
    }

    let res = relation::compact_relation_fn(&item).unwrap_or_else(|err| {
        let error = format!("{:?}", err);

        quote::quote! {
            compile_error!(#error)
        }
    });

    res.into()
}

#[proc_macro_attribute]
pub fn relation(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let implementation = syn::parse_macro_input!(input as syn::Item);

    if !implementation
        .to_token_stream()
        .to_string()
        .starts_with("impl RelationTrait")
    {
        return quote::quote! {
            compile_error!("Macro should be applied on the implementation of RelationTrait trait")
        }
        .into();
    }

    let item = match implementation {
        syn::Item::Impl(implementation) => implementation,
        _ => return quote::quote! {
            compile_error!("Macro should be applied on the implementation of RelationTrait trait")
        }
        .into(),
    };

    let res = relation::expanded_relation_fn(&item).unwrap_or_else(|err| {
        let error = format!("{:?}", err);

        quote::quote! {
            compile_error!(#error)
        }
    });

    res.into()
}

#[proc_macro_derive(QueryRoot, attributes(seaography))]
pub fn derive_root_query_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    match data {
        syn::Data::Struct(_) => (),
        _ => return quote::quote! { compile_error!("Input not structure") }.into(),
    };

    let attrs: Vec<root_query::Seaography> = attrs
        .into_iter()
        .map(|attribute| root_query::Seaography::from_attributes(&vec![attribute]).unwrap())
        .collect();

    let res = root_query::root_query_fn(&ident, &attrs).unwrap_or_else(|err| {
        let error = format!("{:?}", err);

        quote::quote! {
            compile_error!(#error)
        }
    });

    res.into()
}

#[proc_macro_derive(EnumFilter, attributes())]
pub fn derive_enum_filter_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let _ = match data {
        syn::Data::Enum(enumeration) => enumeration,
        _ => return quote::quote! { compile_error!("Input not enumeration") }.into(),
    };

    enumeration::enum_filter_fn(ident).into()
}