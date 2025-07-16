use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, ImplItem, ItemImpl, Pat, ReturnType, Type};

#[proc_macro_attribute]
pub fn mutation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    let mut fields = Vec::new();

    for item in &mut impl_block.items {
        if let ImplItem::Fn(fn_item) = item {
            let fn_ident = &fn_item.sig.ident;
            let fn_name = fn_item.sig.ident.to_string();
            let return_ty = &fn_item.sig.output;

            let new_field_return_ty = match return_ty {
                ReturnType::Type(_, ty) => {
                    if let Type::Path(type_path) = ty.as_ref() {
                        quote! { <#type_path as seaography::GqlTypeRef>::gql_type_ref() }
                    } else {
                        panic!("No return type path found");
                    }
                }
                _ => panic!("Ambiguous return type"),
            };

            let new_field_args = fn_item
                .sig
                .inputs
                .iter()
                .skip(2)
                .map(|arg| match arg {
                    FnArg::Typed(pat_type) => {
                        let arg_name = match &*pat_type.pat {
                            Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                            _ => panic!("Expected identifier pattern"),
                        };
                        if let Type::Path(type_path) = pat_type.ty.as_ref() {
                            quote! { .argument(async_graphql::dynamic::InputValue::new(#arg_name, <#type_path as seaography::GqlTypeRef>::gql_type_ref())) }
                        } else {
                            panic!("No argument type path found: {:?}", pat_type.ty)
                        }
                    }
                    FnArg::Receiver(_) => panic!("Receiver must be the first argument"),
                });

            let new_resolver_args = fn_item
                .sig
                .inputs
                .iter()
                .skip(2)
                .map(|arg| match arg {
                    FnArg::Typed(pat_type) => {
                        let arg_name = match &*pat_type.pat {
                            Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                            _ => panic!("Expected identifier pattern"),
                        };
                        if let Type::Path(type_path) = pat_type.ty.as_ref() {
                            quote! { ctx.args.get(#arg_name).map(|v| v.deserialize::<#type_path>()).unwrap_or(Err(async_graphql::Error::new(format!("{} is required", #arg_name))))?  }
                        } else {
                            panic!("No argument type path found: {:?}", pat_type.ty)
                        }
                    }
                    FnArg::Receiver(_) => panic!("Receiver must be the first argument"),
                });

            let new_field = quote! {
                fields.push({
                    let this = this.clone();
                    async_graphql::dynamic::Field::new(
                        #fn_name,
                        #new_field_return_ty,
                        move |ctx| {
                            let this = this.clone();
                            async_graphql::dynamic::FieldFuture::new(async move {
                                use seaography::GqlFieldValue;
                                this.as_ref().#fn_ident(&ctx.ctx, #(#new_resolver_args,)*).await.map(|x| Some(x.gql_field_value()))
                            })
                        },
                    )
                    #(#new_field_args)*
                });
            };
            fields.push(new_field);
        }
    }

    let impl_item_dynamic_fields: ImplItem = parse_quote! {
        pub fn into_dynamic_fields(self) -> std::vec::Vec<async_graphql::dynamic::Field> {
            let this = std::sync::Arc::new(self);
            let mut fields = std::vec::Vec::new();
            #(#fields)*
            fields
        }
    };

    impl_block.items.push(impl_item_dynamic_fields);

    TokenStream::from(quote! {
        #impl_block
    })
}