use darling::{FromAttributes, FromMeta};
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, ext::IdentExt};

#[derive(Clone, Copy, Debug, FromMeta)]
pub(crate) enum RenameRule {
    #[darling(rename = "lowercase")]
    Lower,
    #[darling(rename = "UPPERCASE")]
    Upper,
    #[darling(rename = "PascalCase")]
    Pascal,
    #[darling(rename = "camelCase")]
    Camel,
    #[darling(rename = "snake_case")]
    Snake,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
}

impl RenameRule {
    fn apply(self, ident: &Ident) -> String {
        let name = ident.unraw().to_string();
        match self {
            Self::Lower => name.to_lowercase(),
            Self::Upper => name.to_uppercase(),
            Self::Pascal => name.to_upper_camel_case(),
            Self::Camel => name.to_lower_camel_case(),
            Self::Snake => name.to_snake_case(),
            Self::ScreamingSnake => name.to_shouty_snake_case(),
        }
    }
}

#[derive(FromAttributes)]
#[darling(attributes(seaography))]
struct FieldArgs {
    rename: Option<String>,
}

pub(crate) fn graphql_field_name(
    attrs: &[Attribute],
    ident: &Ident,
    rename_all: Option<RenameRule>,
) -> syn::Result<TokenStream> {
    let args = FieldArgs::from_attributes(attrs)?;
    let name = args
        .rename
        .or_else(|| rename_all.map(|rule| rule.apply(ident)));
    Ok(match name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#ident) },
    })
}
