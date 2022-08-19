use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaAttr {
    filter_name: syn::Lit,
}

pub fn filter_fn(item: syn::DataStruct, attrs: SeaAttr) -> TokenStream {
    let fields: Vec<TokenStream> = item
        .fields
        .into_iter()
        .map(|field| (field.ident.unwrap(), field.ty))
        .map(|(ident, ty)| {
            let ty = remove_optional_from_type(ty);

            quote! {
                #ident: #ty
            }
        })
        .collect();

    let entity_name = match attrs.filter_name {
        syn::Lit::Str(name) => name,
        _ => panic!("Invalid entity name"),
    };

    let filter_name = format!("{}Filter", entity_name.value().to_upper_camel_case());

    quote! {
        #[derive(Debug, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Filter {
            #(#fields),*
        }
    }
}

pub fn remove_optional_from_type(ty: syn::Type) -> syn::Type {
    fn path_is_option(path: &syn::Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    match ty {
        syn::Type::Path(type_path)
            if type_path.qself.is_none() && path_is_option(&type_path.path) =>
        {
            let type_params = &type_path.path.segments.first().unwrap().arguments;
            let generic_arg = match type_params {
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => panic!("TODO: error handling"),
            };
            match generic_arg {
                syn::GenericArgument::Type(ty) => ty.to_owned(),
                _ => panic!("TODO: error handling"),
            }
        }
        _ => ty,
    }
}
