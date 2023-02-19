use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sea_orm_codegen::OutputFile;

use crate::writer::EntityDefinition;

pub struct RelationDef {
    pub path: TokenStream,
    pub variant: TokenStream,
    pub related: bool,
    pub reverse: bool,
}

impl RelationDef {
    fn related(path: TokenStream) -> Self {
        Self {
            path,
            variant: quote!{},
            related: true,
            reverse: false
        }
    }
    fn relation(path: TokenStream, variant: TokenStream, reverse: bool) -> Self {
        Self {
            path,
            variant,
            related: false,
            reverse
        }
    }
}

pub fn parse_entity(file: &OutputFile) -> EntityDefinition {
    let name = &file.name.as_str()[..file.name.len() - 3];
    let name: TokenStream = format!("crate::entities::{}", name).parse().unwrap();

    let tree = syn::parse2::<syn::File>(file.content.parse().unwrap()).unwrap();

    let relations: BTreeMap<String, RelationDef> =
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

                            let via_method = implementation.items.iter().find(|item| match item {
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
                                let ident: String = ident
                                    [12..ident.chars().position(|c| c == '.').unwrap() - 1]
                                    .into();
                                ident.split("::").last().unwrap().trim().into()
                            } else {
                                panic!("We expect to_method variable to be Method type")
                            };

                            if let Some(_) = via_method {
                                acc.insert(name, RelationDef::related(path));
                            }
                        }
                    }
                    acc
                }
                syn::Item::Enum(enumeration) => {
                    if enumeration.ident.to_string().eq("Relation") {
                        enumeration.variants.iter().for_each(|variant| {
                            let name = variant.ident.to_string();
                            let attr = variant.attrs.iter().find(|attr| {
                                attr
                                    .path
                                    .get_ident()
                                    .map(|i| i.to_string().eq("sea_orm")).unwrap_or_else(|| false)
                            });
                            if let Some(attr) = attr {
                                let ident = quote::format_ident!("{}", name);
                                attr.tokens.clone().into_iter().for_each(|tok| println!("{:?}", tok));
                                if attr.tokens.to_string().contains("belongs_to = \"Entity\"") {
                                    acc.insert(name.clone(), RelationDef { path: quote!{ #ident }, related: false, reverse: false, variant: quote!{ ident } });
                                    acc.insert(format!("{}Reverse", name), RelationDef { path: quote!{ #ident }, related: false, reverse: false, variant: quote!{ ident } });
                                } else {
                                    acc.insert(name, RelationDef { path: quote!{ #ident }, related: false, reverse: false, variant: quote!{ ident } });
                                }
                            }
                        });
                        acc
                    } else {
                        acc
                    }
                }
                _ => acc,
            });

    EntityDefinition { name, relations }
}
