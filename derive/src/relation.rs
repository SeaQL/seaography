use heck::ToUpperCamelCase;
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
    let (loaders, functions): (Vec<_>, Vec<_>) = item
        .variants
        .iter()
        .map(
            |variant| -> Result<(TokenStream, TokenStream), crate::error::Error> {
                let attrs = SeaOrm::from_attributes(&variant.attrs)?;

                let belongs_to = match attrs.belongs_to {
                    Some(syn::Lit::Str(belongs_to)) => Some(belongs_to.value()),
                    _ => None,
                };

                let has_many = match attrs.has_many {
                    Some(syn::Lit::Str(has_many)) => Some(has_many.value()),
                    _ => None,
                };

                relation_fn(variant.ident.to_string(), belongs_to, has_many)
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

    let (loaders, functions): (Vec<_>, Vec<_>) = expanded_params
        .iter()
        .map(
            |params| -> Result<(TokenStream, TokenStream), crate::error::Error> {
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

                relation_fn(params.variant.to_string(), belongs_to, has_many)
            },
        )
        .collect::<Result<Vec<_>, crate::error::Error>>()?
        .into_iter()
        .map(|(loader, func)| (loader, func))
        .unzip();

    Ok(quote! {
        #item

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
) -> Result<(TokenStream, TokenStream), crate::error::Error> {
    let relation_ident = format_ident!("{}", relation_name.to_upper_camel_case());

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
        quote!{ self }
    };

    let (return_type, extra_imports, map_method) = if has_many.is_some() {
        (
            quote! { Vec<#path::Model> },
            quote! { use seaography::itertools::Itertools; },
            quote! { .into_group_map() },
        )
    } else if belongs_to.is_some() {
        (quote! { #path::Model }, quote! {}, quote! { .collect() })
    } else {
        return Err(crate::error::Error::Internal(
            "Cannot map relation: neither one-many or many-one".into(),
        ));
    };

    let relation_enum = quote! {Relation::#relation_ident};
    let foreign_key_name = format_ident!("{}FK", relation_ident).to_token_stream();

    Ok((
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #foreign_key_name(pub seaography::RelationKeyStruct<Option<#path::Filter>, Option<#path::OrderBy>>);

            #[async_trait::async_trait]
            impl async_graphql::dataloader::Loader<#foreign_key_name> for crate::OrmDataloader {
                type Value = #return_type;
                type Error = std::sync::Arc<sea_orm::error::DbErr>;

                async fn load(
                    &self,
                    keys: &[#foreign_key_name],
                ) -> Result<std::collections::HashMap<#foreign_key_name, Self::Value>, Self::Error> {
                    use seaography::heck::ToSnakeCase;
                    use ::std::str::FromStr;

                    let keys: Vec<_> = keys
                        .into_iter()
                        .map(|key| key.0.to_owned())
                        .collect();

                    #extra_imports

                    let data: std::collections::HashMap<#foreign_key_name, Self::Value> = seaography
                        ::fetch_relation_data::<#path::Entity, #path::Filter, #path::OrderBy>(
                            keys,
                            #relation_enum.def(),
                            &self.db,
                        ).await?
                        .into_iter()
                        .map(|(key, model)| (#foreign_key_name(key), model))
                        #map_method;


                    Ok(data)
                }
            }
        },
        quote! {
            pub async fn #relation_ident<'a>(
                &self,
                ctx: &async_graphql::Context<'a>,
            ) -> Option<#return_type> {
                use seaography::heck::ToSnakeCase;
                use ::std::str::FromStr;

                let data_loader = ctx
                    .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
                    .unwrap();

                let from_column: Column = Column::from_str(
                    #relation_enum
                        .def()
                        .from_col
                        .to_string()
                        .to_snake_case()
                        .as_str()
                ).unwrap();

                let key = #foreign_key_name(seaography::RelationKeyStruct(self.get(from_column), None, None));

                let data: Option<_> = data_loader.load_one(key).await.unwrap();

                data
            }
        },
    ))
}
