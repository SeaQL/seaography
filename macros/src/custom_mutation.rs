use proc_macro2::{self, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    DataStruct, DeriveInput, Fields, FieldsNamed, Ident, ReturnType, Type, spanned::Spanned,
};

fn impl_mutation(the_struct: syn::Ident, fields: FieldsNamed) -> proc_macro2::TokenStream {
    let mut gql_fields = Vec::new();

    for field in fields.named {
        let fn_name = field.ident.as_ref().unwrap();
        let fn_name_str = fn_name.to_string();
        let Type::BareFn(func) = field.ty else {
            continue;
        };
        // println!("{:#?}", func.inputs);
        // println!("{:#?}", func.output);

        let new_field_return_ty = match &func.output {
            ReturnType::Type(_, ty) => {
                if let Type::Path(type_path) = ty.as_ref() {
                    quote! { #type_path::gql_type_ref(&CONTEXT) }
                } else {
                    return quote_spanned! {
                        func.span() => compile_error!("Unknown return type");
                    };
                }
            }
            _ => {
                return quote_spanned! {
                    func.span() => compile_error!("Please specify return type");
                };
            }
        };

        let new_field_return_value = match &func.output {
            ReturnType::Type(_, ty) => {
                if let Type::Path(type_path) = ty.as_ref() {
                    quote! { Ok(Some(#type_path::gql_field_value(result))) }
                } else {
                    return quote_spanned! {
                        func.span() => compile_error!("Unknown return type");
                    };
                }
            }
            _ => {
                return quote_spanned! {
                    func.span() => compile_error!("Please specify return type");
                };
            }
        };

        let mut new_field_args = Vec::new();
        let mut resolve_args = Vec::new();
        let mut call_args = Vec::new();

        for (i, arg) in func.inputs.into_iter().enumerate() {
            if let Type::Path(type_path) = arg.ty {
                let (arg_name, arg_name_str) = match arg.name {
                    Some(arg_name) => {
                        let arg_name_str = arg_name.0.to_string();
                        (arg_name.0, arg_name_str)
                    }
                    None => {
                        let arg_name_str = format!("arg_{i}");
                        let arg_name = Ident::new(&arg_name_str, Span::call_site());
                        (arg_name, arg_name_str)
                    }
                };
                new_field_args.push(quote! {
                    .argument(seaography::async_graphql::dynamic::InputValue::new(
                        #arg_name_str,
                        #type_path::gql_type_ref(&CONTEXT),
                    ))
                });
                resolve_args.push(quote! {
                    let #arg_name = #type_path::try_get_arg(&CONTEXT, &ctx, #arg_name_str)?;
                });
                call_args.push(arg_name);
            } else {
                return quote_spanned! {
                    arg.span() => compile_error!("Unknown argument type");
                };
            }
        }

        gql_fields.push(quote! {
            seaography::async_graphql::dynamic::Field::new(
                #fn_name_str,
                #new_field_return_ty,
                move |ctx| {
                    seaography::async_graphql::dynamic::FieldFuture::new(async move {
                        #(#resolve_args)*

                        let result = #the_struct::#fn_name(&ctx, #(#call_args),*).await?;

                        #new_field_return_value
                    })
                },
            )
            #(#new_field_args)*
        });
    }

    quote! {
        impl #the_struct {
            pub fn gql() -> std::vec::Vec<seaography::async_graphql::dynamic::Field> {
                use seaography::{AsyncGqlScalerValueType, AsyncGqlModelType};

                vec![
                    #(#gql_fields),*
                ]
            }
        }
    }
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput { ident, data, .. } = input;

    match data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => Ok(impl_mutation(ident, fields)),
        _ => Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive CustomOperation on data struct");
        }),
    }
}
