use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaOrm {
    belongs_to: Option<syn::Lit>,
    has_many: Option<syn::Lit>,
    from: Option<syn::Lit>,
    to: Option<syn::Lit>,
    on_update: Option<syn::Lit>,
    on_delete: Option<syn::Lit>,
}

pub fn compact_relation_fn(item: &syn::DataEnum) -> Result<TokenStream, crate::error::Error> {
    let relations_parameters: Vec<(String, Option<String>, Option<String>, bool)>  = item
        .variants
        .iter()
        .map(
            |variant| -> Result<(String, Option<String>, Option<String>, bool), crate::error::Error> {
                let attrs = SeaOrm::from_attributes(&variant.attrs)?;

                let belongs_to = match attrs.belongs_to {
                    Some(syn::Lit::Str(belongs_to)) => Some(belongs_to.value()),
                    _ => None,
                };

                let has_many = match attrs.has_many {
                    Some(syn::Lit::Str(has_many)) => Some(has_many.value()),
                    _ => None,
                };

                Ok((variant.ident.to_string(), belongs_to, has_many, false))
            },
        )
        .collect::<Result<Vec<_>, crate::error::Error>>()?;

    produce_relations(relations_parameters)
}

#[derive(Debug)]
struct ExpandedParams {
    variant: syn::Ident,
    relation_type: syn::Ident,
    related_type: syn::Path,
}

impl syn::parse::Parse for ExpandedParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let variant_path = input.parse::<syn::Path>()?;

        let variant = variant_path.segments[1].ident.clone();

        input.parse::<syn::token::FatArrow>()?;

        let method_path = input.parse::<syn::Path>()?;
        let relation_type = method_path.segments[1].ident.clone();

        let group;
        syn::parenthesized!(group in input);

        let related_type: syn::Path = group.parse()?;

        // Used to purge remaining buffer
        input.step(|cursor| {
            let mut rest = *cursor;

            while let Some((_, next)) = rest.token_tree() {
                rest = next;
            }

            Ok(((), rest))
        })?;

        Ok(Self {
            variant,
            relation_type,
            related_type,
        })
    }
}

pub fn expanded_relation_fn(item: &syn::ItemImpl) -> Result<TokenStream, crate::error::Error> {
    if item
        .to_token_stream()
        .to_string()
        .contains("No RelationDef")
    {
        return Ok(quote! {
            #item

            #[async_graphql::ComplexObject]
            impl Model {
            }
        });
    }

    let method_tokens = item.items[0].to_token_stream();
    let method_item: syn::ImplItemMethod = syn::parse2(method_tokens)?;

    let match_tokens = method_item.block.stmts[0].to_token_stream();
    let match_item: syn::ExprMatch = syn::parse2(match_tokens)?;

    let expanded_params: Vec<ExpandedParams> = match_item
        .arms
        .iter()
        .map(|arm| -> Result<ExpandedParams, crate::error::Error> {
            let params: ExpandedParams =
                syn::parse_str(arm.to_token_stream().to_string().as_str())?;

            Ok(params)
        })
        .collect::<Result<Vec<ExpandedParams>, crate::error::Error>>()?;

    let relations_parameters: Vec<(String, Option<String>, Option<String>, bool)> = expanded_params
        .iter()
        .map(|params| -> Result<(String, Option<String>, Option<String>, bool), crate::error::Error> {
            let belongs_to = if params.relation_type.to_string().eq("belongs_to") {
                Some(params.related_type.to_token_stream().to_string())
            } else {
                None
            };

            let has_many = if params.relation_type.to_string().ne("belongs_to") {
                Some(params.related_type.to_token_stream().to_string())
            } else {
                None
            };

            let relation_name = params.variant.to_string();

            Ok((relation_name, belongs_to, has_many, false))
        })
        .collect::<Result<Vec<_>, crate::error::Error>>()?;

    produce_relations(relations_parameters)
}

pub fn produce_relations(
    relations_parameters: Vec<(String, Option<String>, Option<String>, bool)>,
) -> Result<TokenStream, crate::error::Error> {
    let relations_copy = relations_parameters.clone();

    let reverse_self_references_parameters = relations_copy
        .into_iter()
        .filter(|(_, belongs_to, has_one, _)| {
            belongs_to.eq(&Some("Entity".into())) || has_one.eq(&Some("Entity".into()))
        })
        .map(|(relation_name, belongs_to, has_many, _)| {
            (relation_name, has_many, belongs_to, true)
        });

    let (loaders, functions): (Vec<_>, Vec<_>) = relations_parameters
        .into_iter()
        .chain(reverse_self_references_parameters)
        .map(
            |(relation_name, belongs_to, has_many, reverse)| -> Result<(TokenStream, TokenStream), crate::error::Error> {
                relation_fn(relation_name, belongs_to, has_many, reverse)
            },
        )
        .collect::<Result<Vec<_>, crate::error::Error>>()?
        .into_iter()
        .map(|(loader, func)| (loader, func))
        .unzip();

    Ok(quote! {
        #(#loaders)*

        #[async_graphql::ComplexObject]
        impl Model {
            #(#functions)*
        }
    })
}

pub fn relation_fn(
    relation_name: String,
    belongs_to: Option<String>,
    has_many: Option<String>,
    reverse: bool,
) -> Result<(TokenStream, TokenStream), crate::error::Error> {
    let relation_ident = format_ident!("{}", relation_name.to_upper_camel_case());

    let relation_name = if reverse {
        format_ident!("{}Reverse", relation_name.to_upper_camel_case())
    } else {
        format_ident!("{}", relation_name.to_upper_camel_case())
    };

    let (reverse, column_type) = if reverse {
        (quote! { true }, quote! { to_col })
    } else {
        (quote! { false }, quote! { from_col })
    };

    let target_path = if let Some(target_path) = &has_many {
        target_path
    } else if let Some(target_path) = &belongs_to {
        target_path
    } else {
        return Err(crate::error::Error::Internal(
            "Cannot map relation: neither one-many or many-one".into(),
        ));
    };

    let path: TokenStream = if target_path.ne("Entity") {
        target_path.as_str()[..target_path.len() - 8]
            .parse()
            .unwrap()
    } else {
        quote! { self }
    };

    let relation_enum = quote! {Relation::#relation_ident};
    let foreign_key_name = format_ident!("{}FK", relation_name).to_token_stream();

    if has_many.is_some() && belongs_to.is_some() {
        return Err(crate::error::Error::Internal(
            "Cannot map relation: cannot be both one-many and many-one".into(),
        ))
    }

    let (global_scope, object_scope) = if has_many.is_some() {
        (
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #foreign_key_name(pub seaography::RelationKeyStruct<#path::Entity>);

                #[async_trait::async_trait]
                impl async_graphql::dataloader::Loader<#foreign_key_name> for crate::OrmDataloader {
                    type Value = Vec<#path::Model>;
                    type Error = std::sync::Arc<sea_orm::error::DbErr>;

                    async fn load(
                        &self,
                        keys: &[#foreign_key_name],
                    ) -> Result<std::collections::HashMap<#foreign_key_name, Self::Value>, Self::Error> {
                        let keys: Vec<_> = keys
                            .into_iter()
                            .map(|key| key.0.to_owned())
                            .collect();

                        use seaography::itertools::Itertools;

                        let data: std::collections::HashMap<#foreign_key_name, Self::Value> = seaography
                            ::fetch_relation_data::<#path::Entity>(
                                keys,
                                #relation_enum.def(),
                                #reverse,
                                &self.db,
                            ).await?
                            .into_iter()
                            .map(|(key, model)| (#foreign_key_name(key), model))
                            .into_group_map();


                        Ok(data)
                    }
                }
            },
            quote! {
                pub async fn #relation_name<'a>(
                    &self,
                    ctx: &async_graphql::Context<'a>,
                    filters: Option<#path::Filter>,
                    pagination: Option<seaography::Pagination>,
                    order_by: Option<#path::OrderBy>,
                ) -> async_graphql::types::connection::Connection<String, #path::Model, seaography::ExtraPaginationFields, async_graphql::types::connection::EmptyFields> {
                    use seaography::heck::ToSnakeCase;
                    use ::std::str::FromStr;

                    let data_loader = ctx
                        .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
                        .unwrap();

                    let from_column: Column = Column::from_str(
                        #relation_enum
                            .def()
                            .#column_type
                            .to_string()
                            .to_snake_case()
                            .as_str()
                    ).unwrap();

                    let key = #foreign_key_name(seaography::RelationKeyStruct(self.get(from_column), filters, order_by));

                    let nodes: Vec<#path::Model> = data_loader
                        .load_one(key)
                        .await
                        .unwrap()
                        .unwrap();

                    if let Some(pagination) = pagination {
                        return match pagination {
                            seaography::Pagination::Pages(pagination) => {
                                let nodes_size = nodes.len();

                                let nodes = nodes
                                    .into_iter()
                                    .skip(pagination.page * pagination.limit)
                                    .take(pagination.limit)
                                    .collect();

                                let has_previous_page = pagination.page * pagination.limit > 0 && nodes_size != 0;
                                let has_next_page = ((nodes_size / pagination.limit) as i64) - (pagination.page as i64) - 1 > 0;
                                let pages: usize = nodes_size / pagination.limit;
                                let current = pagination.page;

                                seaography::data_to_connection::<#path::Entity>(
                                    nodes,
                                    has_previous_page,
                                    has_next_page,
                                    Some(pages),
                                    Some(current)
                                )
                            },
                            seaography::Pagination::Cursor(cursor) => {
                                // TODO fix cursor related query pagination
                                seaography::data_to_connection::<#path::Entity>(
                                    nodes,
                                    false,
                                    false,
                                    Some(1),
                                    Some(1)
                                )
                            }
                        }
                    }

                    seaography::data_to_connection::<#path::Entity>(
                        nodes,
                        false,
                        false,
                        Some(1),
                        Some(1)
                    )
                }
            },
        )
    } else if belongs_to.is_some() {
        (
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #foreign_key_name(pub seaography::RelationKeyStruct<#path::Entity>);

                #[async_trait::async_trait]
                impl async_graphql::dataloader::Loader<#foreign_key_name> for crate::OrmDataloader {
                    type Value = #path::Model;
                    type Error = std::sync::Arc<sea_orm::error::DbErr>;

                    async fn load(
                        &self,
                        keys: &[#foreign_key_name],
                    ) -> Result<std::collections::HashMap<#foreign_key_name, Self::Value>, Self::Error> {
                        let keys: Vec<_> = keys
                            .into_iter()
                            .map(|key| key.0.to_owned())
                            .collect();

                        let data: std::collections::HashMap<#foreign_key_name, Self::Value> = seaography
                            ::fetch_relation_data::<#path::Entity>(
                                keys,
                                #relation_enum.def(),
                                #reverse,
                                &self.db,
                            ).await?
                            .into_iter()
                            .map(|(key, model)| (#foreign_key_name(key), model))
                            .collect();


                        Ok(data)
                    }
                }
            },
            quote! {
                pub async fn #relation_name<'a>(
                    &self,
                    ctx: &async_graphql::Context<'a>,
                ) -> Option<#path::Model> {
                    use seaography::heck::ToSnakeCase;
                    use ::std::str::FromStr;

                    let data_loader = ctx
                        .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
                        .unwrap();

                    let from_column: Column = Column::from_str(
                        #relation_enum
                            .def()
                            .#column_type
                            .to_string()
                            .to_snake_case()
                            .as_str()
                    ).unwrap();

                    let key = #foreign_key_name(seaography::RelationKeyStruct(self.get(from_column), None, None));

                    let data: Option<_> = data_loader.load_one(key).await.unwrap();

                    data
                }
            },
        )
    } else {
        return Err(crate::error::Error::Internal(
            "Cannot map relation: neither one-many or many-one".into(),
        ))
    };

    Ok((global_scope, object_scope))
}
