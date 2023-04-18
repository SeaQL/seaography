use proc_macro2::Ident;
use quote::format_ident;

pub struct EntityDefinition {
    pub name: Ident,
}

pub fn parse_entity(file_name: String) -> EntityDefinition {
    let name = &file_name[..file_name.len() - 3];
    let name = format_ident!("{}", name);

    EntityDefinition { name }
}

pub struct EnumerationDefinition {
    pub name: Ident,
}

pub fn parse_enumerations(file_content: String) -> Vec<EnumerationDefinition> {
    let tree = syn::parse2::<syn::File>(file_content.parse().unwrap()).unwrap();

    let items: Vec<EnumerationDefinition> = tree
        .items
        .iter()
        .filter(|item| match item {
            syn::Item::Enum(_) => true,
            _ => false,
        })
        .map(|item| match item {
            syn::Item::Enum(enumeration) => EnumerationDefinition {
                name: enumeration.ident.clone(),
            },
            _ => panic!("This is unreachable."),
        })
        .collect();

    items
}
