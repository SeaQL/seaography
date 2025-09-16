use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Error, FnArg, Ident, ImplItem, ItemImpl, PathArguments, ReturnType, Signature, Type,
    TypeReference, spanned::Spanned,
};

pub fn expand(
    input: ItemImpl,
    annotated_item: proc_macro::TokenStream,
) -> syn::Result<TokenStream> {
    let mut impl_fields: Vec<TokenStream> = Vec::new();
    let mut field_calls: Vec<TokenStream> = Vec::new();
    for item in input.items.iter() {
        if let ImplItem::Fn(item_fn) = item {
            let field_fn_ident = format_field_name(&item_fn.sig.ident);
            impl_fields.push(
                signature_to_field(&item_fn.sig, &item_fn.sig.ident, &field_fn_ident, true)
                    .unwrap_or_else(Error::into_compile_error),
            );
            field_calls.push(quote! {
                Self::#field_fn_ident(context),
            });
        }
    }

    let annotated_item = TokenStream::from(annotated_item);
    let self_ty = &input.self_ty;
    Ok(quote! {
        #annotated_item

        impl #self_ty {
            #(#impl_fields)*
        }

        impl seaography::CustomFields for #self_ty {
            fn to_fields(context: &'static seaography::BuilderContext) -> Vec<async_graphql::dynamic::Field> {
                vec![
                    #(#field_calls)*
                ]
            }
        }

    })
}

fn format_field_name(ident: &Ident) -> Ident {
    format_ident!("_field_{}", ident)
}

fn signature_to_field(
    sig: &Signature,
    impl_fn_ident: &Ident,
    field_fn_ident: &Ident,
    is_member: bool,
) -> syn::Result<TokenStream> {
    let fn_ident: &Ident = &sig.ident;
    let field_name = fn_ident;
    let return_type: TokenStream = return_type_to_type_ref(&sig.output)?;
    let mut arguments: Vec<TokenStream> = Vec::new();
    let mut resolve_args: Vec<TokenStream> = Vec::new();

    let args: Vec<FnArg> = sig.inputs.iter().cloned().collect();
    let mut argno = 0;
    let mut have_self = false;

    if let Some(FnArg::Receiver(receiver)) = args.get(argno) {
        if receiver.reference.is_none() {
            return Err(Error::new(
                receiver.span(),
                "self argument must be a reference; use &self",
            ));
        }
        if receiver.mutability.is_some() {
            return Err(Error::new(
                receiver.span(),
                "self reference must be immutable; use &self",
            ));
        }

        have_self = true;
        argno += 1;
    }

    if let Some(arg) = args.get(argno)
        && is_context_arg(arg)
    {
        resolve_args.push(quote! {
            ctx.ctx,
        });
        argno += 1;
    }

    while argno < args.len() {
        let arg = &args[argno];
        argno += 1;

        arguments.push(fn_arg_to_field_argument(arg).unwrap_or_else(Error::into_compile_error));

        let FnArg::Typed(typed_arg) = arg else {
            return Err(Error::new(arg.span(), "arg cannot be self"));
        };
        let arg_pat = &typed_arg.pat;

        resolve_args.push(quote! {
            seaography::CustomInputType::
                parse_value(
                    context,
                    ctx.args.get(stringify!(#arg_pat))
                ).map_err(|e| seaography::SeaographyError::AsyncGraphQLError(
                    format!(
                        "Error decoding {} argument {}: {}",
                        stringify!(#field_name),
                        stringify!(#arg_pat),
                        e,
                    ).into()
                ))?,
        });
    }

    let fn_expr: TokenStream = if !is_member {
        quote! { #impl_fn_ident }
    } else if have_self {
        quote! { seaography::try_downcast_ref::<Self>(ctx.parent_value)?.#impl_fn_ident }
    } else {
        quote! { Self::#impl_fn_ident }
    };

    Ok(quote! {
        pub fn #field_fn_ident(
            context: &'static seaography::BuilderContext,
        ) -> async_graphql::dynamic::Field {
            async_graphql::dynamic::Field::new(
                stringify!(#field_name),
                #return_type,
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        Ok(seaography::CustomOutputType::gql_field_value(#fn_expr(
                            #(#resolve_args)*
                        ).await?))
                    })
                })
                #(#arguments)*
        }
    })
}

fn is_context_arg(arg: &FnArg) -> bool {
    if let FnArg::Typed(pat) = arg
        && let Type::Reference(TypeReference { elem, .. }) = &*pat.ty
        && let Type::Path(path) = elem.as_ref()
        && let Some(last_segment) = path.path.segments.last()
        && last_segment.ident == "Context"
    {
        true
    } else {
        false
    }
}

fn return_type_to_type_ref(return_type: &ReturnType) -> syn::Result<TokenStream> {
    let ReturnType::Type(_, ty) = return_type else {
        return Err(Error::new(
            return_type.span(),
            "Function must have a return type",
        ));
    };

    let Type::Path(type_path) = &**ty else {
        return Err(Error::new(ty.span(), "Expected async_graphql::Result<..>"));
    };

    let Some(last_segment) = type_path.path.segments.last() else {
        return Err(Error::new(ty.span(), "Expected async_graphql::Result<..>"));
    };

    if last_segment.ident != "Result" {
        return Err(Error::new(ty.span(), "Expected async_graphql::Result<..>"));
    }

    let PathArguments::AngleBracketed(angle_bracketed) = &last_segment.arguments else {
        return Err(Error::new(ty.span(), "Expected async_graphql::Result<..>"));
    };

    let Some(first_arg) = angle_bracketed.args.first() else {
        return Err(Error::new(ty.span(), "Expected async_graphql::Result<..>"));
    };

    Ok(quote! {
        <#first_arg as seaography::CustomOutputType>::gql_output_type_ref(context)
    })
}

fn fn_arg_to_field_argument(fn_arg: &FnArg) -> syn::Result<TokenStream> {
    match fn_arg {
        FnArg::Receiver(_) => Err(Error::new(fn_arg.span(), "arg cannot be self")),
        FnArg::Typed(typed_arg) => {
            let pat = &typed_arg.pat;
            let ty = &typed_arg.ty;

            Ok(quote! {
                .argument(
                    async_graphql::dynamic::InputValue::new(
                        stringify!(#pat),
                        <#ty as seaography::CustomInputType>::gql_input_type_ref(context),
                    )
                )
            })
        }
    }
}
