use heck::ToUpperCamelCase;
use quote::{quote, ToTokens};

pub fn inject_graphql(
    entities_hashmap: crate::sea_orm_codegen::EntityHashMap,
    expanded_format: bool,
) -> crate::sea_orm_codegen::EntityHashMap {
    let sea_orm_active_enums = entities_hashmap
        .get("sea_orm_active_enums.rs")
        .map(|tokens| {
            let file_parsed: syn::File = syn::parse2(tokens.clone()).unwrap();

            let items: Vec<syn::Item> = file_parsed
                .items
                .into_iter()
                .map(|item| -> syn::Item {
                    if let syn::Item::Enum(enumeration) = item {
                        let derive_attr: syn::Attribute = syn::parse_quote! {
                            #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Eq, Copy, async_graphql::Enum, seaography_derive::EnumFilter)]
                        };
                        syn::Item::Enum(
                            syn::ItemEnum {
                                attrs: [vec![derive_attr], enumeration.attrs[1..].to_vec()].concat(),
                                ..enumeration
                            }
                        )
                    } else {
                        item
                    }
                })
                .collect();

            let file_parsed = syn::File {
                items,
                ..file_parsed
            };

            file_parsed.to_token_stream()
        });

    let mut entities: crate::sea_orm_codegen::EntityHashMap = entities_hashmap
        .into_iter()
        .filter(|(name, _)| !name.eq("sea_orm_active_enums.rs"))
        .map(|(name, entity)| {
            let tree = syn::parse2::<syn::File>(entity).unwrap();

            let tree = syn::File {
                items: tree
                    .items
                    .into_iter()
                    .map(|item| match &item {
                        syn::Item::Struct(structure) if structure.ident.eq("Model") => {
                            let mut attributes = structure.attrs.clone();

                            let mut derives = attributes[0].tokens.to_string();
                            derives.truncate(derives.len() - 1);

                            attributes[0] = syn::Attribute {
                                tokens: format!(
                                    "{}, async_graphql::SimpleObject, seaography_derive::Filter)",
                                    derives
                                )
                                .parse()
                                .unwrap(),
                                ..attributes[0].clone()
                            };

                            if expanded_format {
                                let entity_name = &name[0..name.len() - 3];

                                let table_name_attr: syn::Attribute =
                                    syn::parse_quote! { #[sea_orm(table_name=#entity_name)] };

                                attributes.push(table_name_attr);
                            }

                            {
                                let complex_graphql_attr: syn::Attribute =
                                    syn::parse_quote! { #[graphql(complex)] };

                                attributes.push(complex_graphql_attr);
                            }

                            {
                                let entity_name = &name[0..name.len() - 3];

                                let name = format!("{}", entity_name.to_upper_camel_case());

                                let complex_graphql_attr: syn::Attribute =
                                    syn::parse_quote! { #[graphql(name=#name)] };

                                attributes.push(complex_graphql_attr);
                            }

                            syn::Item::Struct(syn::ItemStruct {
                                attrs: attributes,
                                ..structure.clone()
                            })
                        }
                        syn::Item::Enum(enumeration)
                            if enumeration.ident.eq("Relation") && !expanded_format =>
                        {
                            let mut attributes = enumeration.attrs.clone();

                            let mut derives = attributes[0].tokens.to_string();
                            derives.truncate(derives.len() - 1);

                            attributes[0] = syn::Attribute {
                                tokens: format!(
                                    "{}, seaography_derive::RelationsCompact)",
                                    derives
                                )
                                .parse()
                                .unwrap(),
                                ..attributes[0].clone()
                            };

                            syn::Item::Enum(syn::ItemEnum {
                                attrs: attributes,
                                ..enumeration.clone()
                            })
                        }
                        syn::Item::Impl(implementation)
                            if implementation
                                .to_token_stream()
                                .to_string()
                                .starts_with("impl RelationTrait")
                                && expanded_format =>
                        {
                            let relation_macro_attr: syn::Attribute =
                                syn::parse_quote! { #[seaography_derive::relation] };

                            syn::Item::Impl(syn::ItemImpl {
                                attrs: vec![relation_macro_attr],
                                ..implementation.clone()
                            })
                        }
                        _ => item,
                    })
                    .collect(),
                ..tree
            };

            (name, quote! { #tree })
        })
        .collect();

    if let Some(sea_orm_active_enums) = sea_orm_active_enums {
        entities.insert("sea_orm_active_enums.rs".into(), sea_orm_active_enums);
    }

    entities
}
