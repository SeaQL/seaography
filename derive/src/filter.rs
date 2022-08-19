use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaAttr {
    filter_name: syn::Lit,
}

pub type IdentTypeTuple = (syn::Ident, syn::Type);

// TODO skip ignored fields
pub fn filter_fn(item: syn::DataStruct, attrs: SeaAttr) -> TokenStream {
    let fields: Vec<IdentTypeTuple> = item
        .fields
        .into_iter()
        .map(|field| {
            (field.ident.unwrap(), remove_optional_from_type(field.ty))
        })
        .collect();

    let filter_struct = filter_struct(&fields, &attrs);


    let recursive_filter_fn = recursive_filter_fn(&fields);

    quote! {
        #filter_struct

        #recursive_filter_fn
    }
}

pub fn filter_struct(fields: &Vec<IdentTypeTuple>, attrs: &SeaAttr) -> TokenStream {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, ty)| {
            quote! {
                #ident: Option<crate::TypeFilter<#ty>>
            }
        })
        .collect();

    let entity_name = match &attrs.filter_name {
        syn::Lit::Str(name) => name,
        _ => panic!("Invalid entity name"),
    };

    let filter_name = format!("{}Filter", entity_name.value().to_upper_camel_case());

    quote! {
        #[derive(Debug, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Filter {
            pub or: Option<Vec<Box<Filter>>>,
            pub and: Option<Vec<Box<Filter>>>,
            #(#fields),*
        }
    }
}

pub fn recursive_filter_fn(fields: &Vec<IdentTypeTuple>) -> TokenStream {
    let columns_filters: Vec<TokenStream> = fields
        .iter()
        .filter(|(_, ty)| {
            // TODO skip binary and enum
            true
        })
        .map(|(ident, _)| {
            let column_name = format_ident!("{}", ident.to_string().to_snake_case());

            let column_enum_name = format_ident!("{}", ident.to_string().to_upper_camel_case());

            quote!{
                if let Some(#column_name) = current_filter.#column_name {
                    if let Some(eq_value) = #column_name.eq {
                        condition = condition.add(Column::#column_enum_name.eq(eq_value))
                    }

                    if let Some(ne_value) = #column_name.ne {
                        condition = condition.add(Column::#column_enum_name.ne(ne_value))
                    }

                    if let Some(gt_value) = #column_name.gt {
                        condition = condition.add(Column::#column_enum_name.gt(gt_value))
                    }

                    if let Some(gte_value) = #column_name.gte {
                        condition = condition.add(Column::#column_enum_name.gte(gte_value))
                    }

                    if let Some(lt_value) = #column_name.lt {
                        condition = condition.add(Column::#column_enum_name.lt(lt_value))
                    }

                    if let Some(lte_value) = #column_name.lte {
                        condition = condition.add(Column::#column_enum_name.lte(lte_value))
                    }

                    if let Some(is_in_value) = #column_name.is_in {
                        condition = condition.add(Column::#column_enum_name.is_in(is_in_value))
                    }

                    if let Some(is_not_in_value) = #column_name.is_not_in {
                        condition = condition.add(Column::#column_enum_name.is_not_in(is_not_in_value))
                    }

                    if let Some(is_null_value) = #column_name.is_null {
                        if is_null_value {
                            condition = condition.add(Column::#column_enum_name.is_null())
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();

            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::any(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(or_condition);
                }

                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::all(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(and_condition);
                }

                #(#columns_filters)*
            }

            condition
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
