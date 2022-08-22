use syn::DeriveInput;

mod filter;
mod relation;
mod error;

#[proc_macro_derive(Filter, attributes(sea_orm))]
pub fn derive_filter_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let item = match data {
        syn::Data::Struct(item) => item,
        _ => return quote::quote! {
            compile_error!("Input not structure")
        }.into(),
    };

    if ident.ne("Model") {
        return quote::quote! {
            compile_error!("Struct must be SeaOrm Model structure")
        }.into()
    }

    let attrs = filter::SeaOrm::from_attributes(&attrs).unwrap();

    filter::filter_fn(item, attrs).unwrap_or_else(|err| {
        let error = format!("{:?}", err);

        quote::quote!{
            compile_error!(#error)
        }
    }).into()
}

#[proc_macro_derive(RelationsCompact, attributes(sea_orm))]
pub fn derive_relations_compact_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let item = match data {
        syn::Data::Enum(item) => item,
        _ => return quote::quote!{ compile_error!("Input not enumeration") }.into(),
    };

    if ident.ne("Relation") {
        return quote::quote!{
            compile_error!("Struct must be SeaOrm Relation enumeration")
        }.into()
    }

    let res = relation::compact_relation_fn(&item).unwrap_or_else(|err| {
        let error = format!("{:?}", err);

        quote::quote!{
            compile_error!(#error)
        }
    });

    println!("{}", res.to_string());

    res.into()
}

// TODO attribute to change relation name
// #[proc_macro_attribute]
// pub fn relation(
//     _attrs: proc_macro::TokenStream,
//     input: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     let implementation = syn::parse_macro_input!(input as syn::Item);

//     if !implementation
//         .to_token_stream()
//         .to_string()
//         .starts_with("impl Related")
//     {
//         compile_error!("Macro should be applied on the implementation of Related trait");
//     }

//     let item = match implementation {
//         syn::Item::Impl(implementation) => implementation,
//         _ => compile_error!("Macro should be applied on the implementation of Related trait")
//     };

//     relation::relation_fn(&item).into()
// }
