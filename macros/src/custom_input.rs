use proc_macro2::{self, TokenStream, Span};
use quote::{quote, quote_spanned};
use syn::{
    Token,PathArguments,
    DataStruct, DeriveInput, Fields, FieldsNamed, Type, TypePath
};

fn impl_input(the_struct: syn::Ident, fields: FieldsNamed) -> TokenStream {
    let mut input_fields = Vec::new();
    let mut field_getters = Vec::new();
    let mut field_names = Vec::new();

    for field in fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let Type::Path(TypePath {
            path: mut type_path, ..
        }) = field.ty else {
            continue;
        };
        // convert type path to turbo fish Option::<String>
        for segment in type_path.segments.iter_mut() {
            if let PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
                arguments.colon2_token = Some(Token![::](Span::call_site()));
            }
        }
        field_names.push(field_name.to_owned());
        field_getters.push(quote! {
            let #field_name = #type_path::parse_value(ctx, object.get(#field_name_str))?;
        });
        input_fields.push(quote! {
            .field(seaography::async_graphql::dynamic::InputValue::new(
                #field_name_str,
                #type_path::gql_input_type_ref(ctx),
            ))
        });
    }

    quote! {
        impl seaography::CustomInput for #the_struct {
            fn input_object(ctx: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::InputObject {
                use seaography::{GqlInputType, GqlScalarValueType};

                InputObject::new(format!("{}Input", stringify!(#the_struct)))
                #(#input_fields)*
            }
        }

        impl seaography::GqlInputType for #the_struct {
            fn gql_input_type_ref(_ctx: &'static seaography::BuilderContext) -> seaography::async_graphql::dynamic::TypeRef {
                seaography::async_graphql::dynamic::TypeRef::named_nn(format!("{}Input", stringify!(#the_struct)))
            }

            fn parse_value(
                ctx: &'static seaography::BuilderContext,
                value: Option<seaography::async_graphql::dynamic::ValueAccessor<'_>>,
            ) -> seaography::SeaResult<Self> {
                use seaography::{GqlInputType, GqlInputValue};

                let object = value.unwrap().object()?;

                #(#field_getters)*

                Ok(Self { #(#field_names),* })
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
        }) => Ok(impl_input(ident, fields).into()),
        _ => Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive CustomInput on data struct");
        }),
    }
}
