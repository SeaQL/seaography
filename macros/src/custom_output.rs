use crate::util::qualify_type_path;
use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Attribute, DataStruct, DeriveInput, Fields, FieldsNamed, Lit, Type, meta::ParseNestedMeta,
};

fn impl_output(
    the_struct: syn::Ident,
    attrs: Vec<Attribute>,
    fields: FieldsNamed,
) -> syn::Result<TokenStream> {
    let mut output_fields = Vec::new();

    let mut prefix = "".to_owned();
    let mut suffix = "Basic".to_owned();

    for attr in attrs {
        if attr.path().is_ident("seaography") {
            attr.parse_nested_meta(|meta| {
                if let Some(value) = parse_lit_str(&meta, "prefix")? {
                    prefix = value;
                }
                if let Some(value) = parse_lit_str(&meta, "suffix")? {
                    suffix = value;
                }
                Ok(())
            })?;
        }
    }

    let object_name = format!("{prefix}{the_struct}{suffix}");

    for field in fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let Type::Path(mut type_path) = field.ty else {
            continue;
        };
        qualify_type_path(&mut type_path);
        output_fields.push(quote! {
            .field(Field::new(
                #field_name_str,
                #type_path::gql_output_type_ref(ctx),
                |ctx| {
                    let object = match ctx.parent_value.try_downcast_ref::<Self>() {
                        Ok(object) => object,
                        Err(err) => {
                            return FieldFuture::new(async move { Err::<Option<()>, _>(err) })
                        }
                    };
                    let field_value = #type_path::gql_field_value(object.#field_name.clone());

                    FieldFuture::new(async move { Ok(field_value) })
                },
            ))
        });
    }

    Ok(quote! {
        impl seaography::CustomOutput for #the_struct {
            fn basic_object(ctx: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::Object {
                use seaography::{GqlModelType, GqlModelOptionType, GqlOutputModelType, GqlScalarValueType};
                use seaography::async_graphql::dynamic::{Field, FieldFuture, Object};

                Object::new(#object_name)
                #(#output_fields)*
            }
        }

        impl seaography::GqlOutputModelType for #the_struct {
            fn gql_output_type_ref(_: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::TypeRef {
                seaography::async_graphql::dynamic::TypeRef::named_nn(#object_name)
            }
        }
    })
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident, data, attrs, ..
    } = input;

    match data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => impl_output(ident, attrs, fields),
        _ => Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive CustomOutput on data struct");
        }),
    }
}

fn parse_lit_str(meta: &ParseNestedMeta<'_>, attr: &str) -> syn::Result<Option<String>> {
    if meta.path.is_ident(attr) {
        let lit: Lit = meta.value()?.parse()?;
        if let Lit::Str(lit_str) = lit {
            Ok(Some(lit_str.value()))
        } else {
            Err(meta.error(format!("`{attr}` must be a string literal")))
        }
    } else {
        Ok(None)
    }
}
