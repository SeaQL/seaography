use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sea_orm_codegen::OutputFile;

use crate::writer::EntityDefinition;

pub fn parse_entity(file: &OutputFile) -> EntityDefinition {
    let name = &file.name.as_str()[..file.name.len() - 3];
    let name: TokenStream = format!("crate::entities::{}", name).parse().unwrap();

    let tree = syn::parse2::<syn::File>(file.content.parse().unwrap()).unwrap();

    let relations: BTreeMap<String, TokenStream> =
        tree.items
            .iter()
            .fold(BTreeMap::new(), |mut acc, cur| match cur {
                syn::Item::Impl(implementation) => {
                    if let Some((_bang, path, _for)) = &implementation.trait_ {
                        let path = path.to_token_stream().to_string();
                        if path.starts_with("Related") {
                            let path: TokenStream = path[18..path.len() - 1].parse().unwrap();
                            let path = quote! { crate::entities::#path };

                            let to_method = implementation
                                .items
                                .iter()
                                .find(|item| match item {
                                    syn::ImplItem::Method(method) => method
                                        .sig
                                        .to_token_stream()
                                        .to_string()
                                        .starts_with("fn to ()"),
                                    _ => false,
                                })
                                .expect("We expect Related to have `to` method");

                            let via_method = implementation
                                .items
                                .iter()
                                .find(|item| match item {
                                    syn::ImplItem::Method(method) => method
                                        .sig
                                        .to_token_stream()
                                        .to_string()
                                        .starts_with("fn via ()"),
                                    _ => false,
                                });

                            let name: String = if let syn::ImplItem::Method(method) = to_method {
                                let ident =
                                    (&method.block.stmts[0]).into_token_stream().to_string();
                                let ident: String = ident[12..ident.chars().position(|c| c == '.').unwrap() - 1].into();
                                ident.split("::").last().unwrap().trim().into()
                            } else {
                                panic!("We expect to_method variable to be Method type")
                            };

                            if let Some(_) = via_method {
                                acc.insert(name, path);
                            }
                        }
                    }
                    acc
                }
                _ => acc,
            });

    EntityDefinition { name, relations }
}
