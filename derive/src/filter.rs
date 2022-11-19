use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

        impl seaography::EnhancedEntity for Entity {
            type Entity = Entity;
            type Filter = Filter;
            type OrderBy = OrderBy;
        }
    })
}

pub fn filter_struct(
    fields: &[IdentTypeTuple],
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, type_ident)| {
            quote! {
                #ident: Option<<#type_ident as seaography::FilterTypeTrait>::Filter>
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Internal("Invalid entity name".into())),
    };

    let filter_name = format!("{}Filter", entity_name.value().to_upper_camel_case());

    // TODO enable when async graphql support name_type for input objects
    // let type_name = quote!{
    //     impl async_graphql::TypeName for Filter {
    //         fn type_name() -> ::std::borrow::Cow<'static, str> {
    //             use seaography::heck::ToUpperCamelCase;

    //             let filter_name = format!("{}Filter", Entity::default().table_name().to_string().to_upper_camel_case());

    //             ::std::borrow::Cow::Owned(filter_name)
    //         }
    //     }
    // }

    Ok(quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Filter {
            pub or: Option<Vec<Box<Filter>>>,
            pub and: Option<Vec<Box<Filter>>>,
            #(#fields),*
        }
    })
}

pub fn order_by_struct(
    fields: &[IdentTypeTuple],
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
        _ => return Err(crate::error::Error::Internal("Invalid entity name".into())),
    };

    let filter_name = format!("{}OrderBy", entity_name.value().to_upper_camel_case());

    Ok(quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct OrderBy {
            #(#fields),*
        }
    })
}

pub fn order_by_fn(fields: &[IdentTypeTuple]) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _)| {
            let column = format_ident!("{}", ident.to_string().to_upper_camel_case());

            quote! {
                let stmt = if let Some(order_by) = self.#ident {
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
        impl seaography::EntityOrderBy<Entity> for OrderBy {
            fn order_by(&self, stmt: sea_orm::Select<Entity>) -> sea_orm::Select<Entity> {
                use sea_orm::QueryOrder;

                #(#fields)*

                stmt
            }
        }
    })
}

pub fn recursive_filter_fn(fields: &[IdentTypeTuple]) -> Result<TokenStream, crate::error::Error> {
    let columns_filters: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _)| {
            let column_name = format_ident!("{}", ident.to_string().to_snake_case());

            let column_enum_name = format_ident!("{}", ident.to_string().to_upper_camel_case());

            quote!{
                if let Some(#column_name) = &self.#column_name {
                    if let Some(eq_value) = seaography::FilterTrait::eq(#column_name) {
                        condition = condition.add(Column::#column_enum_name.eq(eq_value))
                    }

                    if let Some(ne_value) = seaography::FilterTrait::ne(#column_name) {
                        condition = condition.add(Column::#column_enum_name.ne(ne_value))
                    }

                    if let Some(gt_value) = seaography::FilterTrait::gt(#column_name) {
                        condition = condition.add(Column::#column_enum_name.gt(gt_value))
                    }

                    if let Some(gte_value) = seaography::FilterTrait::gte(#column_name) {
                        condition = condition.add(Column::#column_enum_name.gte(gte_value))
                    }

                    if let Some(lt_value) = seaography::FilterTrait::lt(#column_name) {
                        condition = condition.add(Column::#column_enum_name.lt(lt_value))
                    }

                    if let Some(lte_value) = seaography::FilterTrait::lte(#column_name) {
                        condition = condition.add(Column::#column_enum_name.lte(lte_value))
                    }

                    if let Some(is_in_value) = seaography::FilterTrait::is_in(#column_name) {
                        condition = condition.add(Column::#column_enum_name.is_in(is_in_value))
                    }

                    if let Some(is_not_in_value) = seaography::FilterTrait::is_not_in(#column_name) {
                        condition = condition.add(Column::#column_enum_name.is_not_in(is_not_in_value))
                    }

                    if let Some(is_null_value) = seaography::FilterTrait::is_null(#column_name) {
                        if is_null_value {
                            condition = condition.add(Column::#column_enum_name.is_null())
                        }
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        impl seaography::EntityFilter for Filter {
            fn filter_condition(&self) -> sea_orm::Condition {
                let mut condition = sea_orm::Condition::all();

                if let Some(or_filters) = &self.or {
                    let or_condition = or_filters
                        .iter()
                        .fold(
                            sea_orm::Condition::any(),
                            |fold_condition, filter| fold_condition.add(filter.filter_condition())
                        );
                    condition = condition.add(or_condition);
                }

                if let Some(and_filters) = &self.and {
                    let and_condition = and_filters
                        .iter()
                        .fold(
                            sea_orm::Condition::all(),
                            |fold_condition, filter| fold_condition.add(filter.filter_condition())
                        );
                    condition = condition.add(and_condition);
                }

                #(#columns_filters)*

                condition
            }
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
                    return Err(crate::error::Error::Internal(
                        "Cannot parse type brackets".into(),
                    ))
                }
            };
            match generic_arg {
                syn::GenericArgument::Type(ty) => ty.to_owned(),
                _ => return Err(crate::error::Error::Internal("Cannot parse type".into())),
            }
        }
        _ => ty,
    };

    Ok(ty)
}
