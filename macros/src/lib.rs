
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, ImplItem, ItemFn, Pat, ReturnType, Type};

#[proc_macro_attribute]
pub fn custom_mutation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);

    let fn_ident = &item_fn.sig.ident;
    let fn_name = item_fn.sig.ident.to_string();
    let return_ty = &item_fn.sig.output;
    let block = &item_fn.block;

    let new_field_return_ty = match return_ty {
        ReturnType::Type(_, ty) => {
            if let Type::Path(type_path) = ty.as_ref() {
                quote! { <#type_path as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT) }
            } else {
                panic!("No return type path found");
            }
        }
        _ => panic!("Ambiguous return type"),
    };

    let new_field_args = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let arg_name = match &*pat_type.pat {
                    Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                    _ => panic!("Expected identifier pattern"),
                };
                if let Type::Path(type_path) = pat_type.ty.as_ref() {
                    quote! {
                        .argument(async_graphql::dynamic::InputValue::new(
                            #arg_name,
                            <#type_path as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
                        ))
                    }
                } else {
                    panic!("No argument type path found: {:?}", pat_type.ty)
                }
            }
            FnArg::Receiver(_) => panic!("Receiver must be the first argument"),
        });

    let new_resolver_args = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let arg_ident = match &*pat_type.pat {
                    Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => panic!("Expected identifier pattern"),
                };
                let arg_string = arg_ident.to_string();
                if let Type::Path(type_path) = pat_type.ty.as_ref() {
                    quote! {
                        let #arg_ident = <#type_path as seaography::AsyncGqlValueType>::try_get_arg(&CONTEXT, &ctx, #arg_string)?;
                    }
                } else {
                    panic!("No argument type path found: {:?}", pat_type.ty)
                }
            }
            FnArg::Receiver(_) => panic!("Receiver must be the first argument"),
        });

    let ts = TokenStream::from(quote! {
        fn #fn_ident() -> async_graphql::dynamic::Field {
            async_graphql::dynamic::Field::new(
                #fn_name,
                #new_field_return_ty,
                move |ctx| {
                    FieldFuture::new(async move {
                        #(#new_resolver_args)*

                        let result = {
                            #block
                        };

                        Ok(Some(async_graphql::dynamic::FieldValue::value(result)))
                    })
                },
            )
            #(#new_field_args)*
        }
    });

    dbg!(ts.to_string());

    ts
}
