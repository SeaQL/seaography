use itertools::Itertools;
use quote::{quote, ToTokens, format_ident};

pub fn inject_graphql(
    entities_hashmap: crate::sea_orm_codegen::EntityHashMap,
) -> crate::sea_orm_codegen::EntityHashMap {
    let sea_orm_active_enums = entities_hashmap
        .get("sea_orm_active_enums.rs")
        .map(|enums| {
            // println!("{:?}", enums);
            quote! {}
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
                        syn::Item::Struct(structure) => {
                            if structure.ident.eq("Model") {
                                let mut attributes = structure.attrs.clone();

                                let mut derives = attributes[0].tokens.to_string();
                                derives.truncate(derives.len() - 1);

                                attributes[0] = syn::Attribute{
                                    tokens: format!("{}, async_graphql::SimpleObject)", derives).parse().unwrap(),
                                    ..attributes[0].clone()
                                };

                                println!("{:?}", attributes[0].tokens.to_string());

                                syn::Item::Struct(
                                    syn::ItemStruct{
                                        attrs: attributes,
                                        ..structure.clone()
                                    }
                                )
                            } else {
                                item
                            }
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
