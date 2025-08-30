use crate::util::qualify_type_path;
use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, Type};

fn impl_output(the_struct: syn::Ident, fields: FieldsNamed) -> TokenStream {
    let mut output_fields = Vec::new();

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

    quote! {
        impl seaography::CustomOutput for #the_struct {
            fn basic_object(ctx: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::Object {
                use seaography::{GqlOutputModelType, GqlScalarValueType};
                use seaography::async_graphql::dynamic::{Field, FieldFuture, Object};

                Object::new(format!("{}Basic", stringify!(#the_struct)))
                #(#output_fields)*
            }
        }

        impl seaography::GqlOutputModelType for #the_struct {
            fn gql_output_type_ref(_: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::TypeRef {
                seaography::async_graphql::dynamic::TypeRef::named_nn(format!("{}Basic", stringify!(#the_struct)))
            }
        }
    }
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput { ident, data, .. } = input;

    match data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => Ok(impl_output(ident, fields)),
        _ => Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive CustomOutput on data struct");
        }),
    }
}
