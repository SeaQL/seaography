use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::writer::EntityDefinition;

pub struct RelationDef {
    pub target: TokenStream,
    pub variant: TokenStream,
    pub related: bool,
    pub reverse: bool,
    pub self_rel: bool,
}

impl RelationDef {
    fn related(target: TokenStream) -> Self {
        Self {
            target,
            variant: quote! {},
            related: true,
            reverse: false,
            self_rel: false,
        }
    }
}

pub fn parse_entity(file_name: String, file_content: String) -> EntityDefinition {
    let name = &file_name[..file_name.len() - 3];
    let name: TokenStream = format!("crate::entities::{}", name).parse().unwrap();

    let tree = syn::parse2::<syn::File>(file_content.parse().unwrap()).unwrap();

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
                                attr.path
                                    .get_ident()
                                    .map(|i| i.to_string().eq("sea_orm"))
                                    .unwrap_or_else(|| false)
                            });
                            if let Some(attr) = attr {
                                let ident = quote::format_ident!("{}", name);

                                let attributes_string = attr.tokens.to_string();
                                let attributes_string =
                                    &attributes_string[1..attributes_string.len() - 1];

                                let attributes = attributes_string.split(",").fold(
                                    std::collections::BTreeMap::<&str, &str>::new(),
                                    |mut acc, cur| {
                                        let mut parts = cur.split("=");
                                        if parts.clone().count() == 2 {
                                            let key = parts
                                                .next()
                                                .expect("We expect to have first part")
                                                .trim();
                                            let value = parts
                                                .next()
                                                .expect("We expect to have second part")
                                                .trim();
                                            acc.insert(key, value);
                                        }

                                        acc
                                    },
                                );

                                let belongs_to = attributes.get("belongs_to");
                                let has_one = attributes.get("has_one");
                                let has_many = attributes.get("has_many");

                                let target = if let Some(v) = belongs_to {
                                    v
                                } else if let Some(v) = has_one {
                                    v
                                } else if let Some(v) = has_many {
                                    v
                                } else {
                                    panic!("Invalid relation definition")
                                };

                                let target = target.replace("super", "crate::entities");

                                let target: TokenStream =
                                    target[1..target.len() - 1].parse().unwrap();

                                let self_belongs_to =
                                    belongs_to.map_or_else(|| false, |v| v.eq(&"\"Entity\""));
                                let self_has_one =
                                    has_one.map_or_else(|| false, |v| v.eq(&"\"Entity\""));
                                let self_has_many =
                                    has_many.map_or_else(|| false, |v| v.eq(&"\"Entity\""));

                                if self_belongs_to || self_has_one || self_has_many {
                                    let normal = RelationDef {
                                        target: target.clone(),
                                        variant: quote! { #ident },
                                        related: false,
                                        reverse: false,
                                        self_rel: true,
                                    };
                                    acc.insert(name.clone(), normal);

                                    let reverse = RelationDef {
                                        target,
                                        variant: quote! { #ident },
                                        related: false,
                                        reverse: true,
                                        self_rel: true,
                                    };
                                    acc.insert(format!("{}Reverse", name), reverse);
                                } else {
                                    let normal = RelationDef {
                                        target,
                                        variant: quote! { #ident },
                                        related: false,
                                        reverse: false,
                                        self_rel: false,
                                    };
                                    acc.insert(name, normal);
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

pub struct EnumerationDefinition {
    pub name: TokenStream,
}

pub fn parse_enumerations(file_content: String) -> Vec<EnumerationDefinition> {
    let tree = syn::parse2::<syn::File>(file_content.parse().unwrap()).unwrap();

    let items: Vec<EnumerationDefinition> = tree
        .items
        .iter()
        .filter(|item| {
            match item {
                syn::Item::Enum(_) => true,
                _ => false
            }
        })
        .map(|item| {
            match item {
                syn::Item::Enum(enumeration) => {
                    EnumerationDefinition {
                        name: enumeration.ident.to_token_stream()
                    }
                },
                _ => panic!("This is unreachable.")
            }
        }).collect();

    items
}
