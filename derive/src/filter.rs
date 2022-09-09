use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaOrm {
    table_name: Option<syn::Lit>,
}

pub type IdentTypeTuple = (syn::Ident, syn::Type);

// TODO skip ignored fields
pub fn filter_fn(item: syn::DataStruct, attrs: SeaOrm) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<IdentTypeTuple> = item
        .fields
        .into_iter()
        .map(|field| {
            (
                field.ident.unwrap(),
                remove_optional_from_type(field.ty).unwrap(),
            )
        })
        .collect();

    let filter_struct = filter_struct(&fields, &attrs)?;

    let recursive_filter_fn = recursive_filter_fn(&fields)?;

    let order_by_struct = order_by_struct(&fields, &attrs)?;

    let order_by_fn = order_by_fn(&fields)?;

    Ok(quote! {
        #filter_struct

        #recursive_filter_fn

        #order_by_struct

        #order_by_fn
    })
}

pub fn filter_struct(
    fields: &Vec<IdentTypeTuple>,
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, ty)| {
            let type_literal = ty.to_token_stream().to_string();

            let default_filters = vec![
                "String",
                "i8",
                "i16",
                "i32",
                "i64",
                "u8",
                "u16",
                "u32",
                "u64",
                "f32",
                "f64",
                "Date",
                "DateTime",
                "DateTimeUtc",
                "Decimal",
                "BinaryVector",
                "bool",
            ];

            let filter_item = if default_filters.contains(&type_literal.as_str())
                || type_literal.starts_with("Vec")
            {
                quote! {
                    seaography::TypeFilter<#ty>
                }
            } else {
                let ident = format_ident!("{}EnumFilter", type_literal);
                quote! {
                    crate::entities::sea_orm_active_enums::#ident
                }
            };

            quote! {
                #ident: Option<#filter_item>
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Error("Invalid entity name".into())),
    };

    let filter_name = format!("{}Filter", entity_name.value().to_upper_camel_case());

    // TODO enable when async graphql support name_type for input objects
    // let type_name = quote!{
    //     impl async_graphql::TypeName for Filter {
    //         fn type_name() -> ::std::borrow::Cow<'static, str> {
    //             use heck::ToUpperCamelCase;

    //             let filter_name = format!("{}Filter", Entity::default().table_name().to_string().to_upper_camel_case());

    //             ::std::borrow::Cow::Owned(filter_name)
    //         }
    //     }
    // }

    Ok(quote! {
        #[derive(Debug, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Filter {
            pub or: Option<Vec<Box<Filter>>>,
            pub and: Option<Vec<Box<Filter>>>,
            #(#fields),*
        }
    })
}

pub fn order_by_struct(
    fields: &Vec<IdentTypeTuple>,
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _)| {
            quote! {
                #ident: Option<seaography::OrderByEnum>
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Error("Invalid entity name".into())),
    };

    let filter_name = format!("{}OrderBy", entity_name.value().to_upper_camel_case());

    Ok(quote! {
        #[derive(Debug, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct OrderBy {
            #(#fields),*
        }
    })
}

pub fn order_by_fn(fields: &Vec<IdentTypeTuple>) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _)| {
            let column = format_ident!("{}", ident.to_string().to_upper_camel_case());

            quote! {
                let stmt = if let Some(order_by) = order_by_struct.#ident {
                    match order_by {
                        seaography::OrderByEnum::Asc => stmt.order_by(Column::#column, sea_orm::query::Order::Asc),
                        seaography::OrderByEnum::Desc => stmt.order_by(Column::#column, sea_orm::query::Order::Desc),
                    }
                } else {
                    stmt
                };
            }
        })
        .collect();

    Ok(quote! {
        pub fn order_by(stmt: sea_orm::Select<Entity>, order_by_struct: Option<OrderBy>) -> sea_orm::Select<Entity> {
            use sea_orm::QueryOrder;

            if let Some(order_by_struct) = order_by_struct {
                #(#fields)*
                stmt
            } else {
                stmt
            }
        }
    })
}

pub fn recursive_filter_fn(
    fields: &Vec<IdentTypeTuple>,
) -> Result<TokenStream, crate::error::Error> {
    let columns_filters: Vec<TokenStream> = fields
        .iter()
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

    Ok(quote! {
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
    })
}

pub fn remove_optional_from_type(ty: syn::Type) -> Result<syn::Type, crate::error::Error> {
    fn path_is_option(path: &syn::Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    let ty = match ty {
        syn::Type::Path(type_path)
            if type_path.qself.is_none() && path_is_option(&type_path.path) =>
        {
            let type_params = &type_path.path.segments.first().unwrap().arguments;
            let generic_arg = match type_params {
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => {
                    return Err(crate::error::Error::Error(
                        "Cannot parse type brackets".into(),
                    ))
                }
            };
            match generic_arg {
                syn::GenericArgument::Type(ty) => ty.to_owned(),
                _ => return Err(crate::error::Error::Error("Cannot parse type".into())),
            }
        }
        _ => ty,
    };

    Ok(ty)
}
