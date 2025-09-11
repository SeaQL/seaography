use proc_macro2::Span;
use syn::{
    DataEnum, DeriveInput, Error, Fields, Ident, PathArguments, Token, Type, TypePath,
    spanned::Spanned,
};

pub fn qualify_type_path(type_path: &mut TypePath) {
    // convert type path to turbo fish Option<String> -> Option::<String>
    for segment in type_path.path.segments.iter_mut() {
        if let PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
            arguments.colon2_token = Some(Token![::](Span::call_site()));
        }
    }
}

#[derive(Debug)]
pub enum EnumVariants {
    Units(Vec<Ident>),
    Containers(Vec<Ident>),
}

pub fn parse_enum_variants(ast: &DeriveInput, data: &DataEnum) -> Result<EnumVariants, Error> {
    let mut count_unit = 0;
    let mut count_container = 0;

    let mut variant_idents: Vec<Ident> = Vec::new();

    for variant in data.variants.iter() {
        variant_idents.push(variant.ident.clone());
        let variant_name = variant.ident.to_string();
        match &variant.fields {
            Fields::Named(named) => {
                return Err(Error::new(
                    named.span(),
                    "Enums with named fields are unsupported)",
                ));
            }
            Fields::Unnamed(unnamed) => {
                if unnamed.unnamed.len() != 1 {
                    return Err(Error::new(
                        unnamed.span(),
                        "Variant must have exactly 1 field",
                    ));
                }
                let inner_name: String = if let Some(first) = unnamed.unnamed.first()
                    && let Type::Path(path) = &first.ty
                    && path.path.segments.len() == 1
                    && let Some(seg) = path.path.segments.first()
                {
                    seg.ident.to_string()
                } else {
                    return Err(Error::new(
                        unnamed.span(),
                        format!(
                            "Variant {} must contain a single unnamed field of type type {}",
                            variant_name, variant_name
                        ),
                    ));
                };
                if inner_name != variant_name {
                    return Err(Error::new(
                        unnamed.span(),
                        format!(
                            "Variant {} must contain a single unnamed field of type type {}",
                            variant_name, variant_name
                        ),
                    ));
                }
                count_container += 1;
            }
            Fields::Unit => {
                count_unit += 1;
            }
        }
    }

    if count_unit == data.variants.len() {
        Ok(EnumVariants::Units(variant_idents))
    } else if count_container == data.variants.len() {
        Ok(EnumVariants::Containers(variant_idents))
    } else {
        Err(Error::new(
            ast.ident.span(),
            "enum must be all units or all containers",
        ))
    }
}
