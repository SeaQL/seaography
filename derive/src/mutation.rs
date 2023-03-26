use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;

#[derive(Debug, Eq, PartialEq, bae::FromAttributes, Clone)]
pub struct Seaography {
    entity: Option<syn::Lit>,
    object_config: Option<syn::Expr>,
    not_mutable: Option<syn::Lit>,
}

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaographyMutation {
    table_name: Option<syn::Lit>,
    mutation_root: Option<syn::Lit>,
    skip: Option<syn::Lit>
}

// copy of filter::SeaOrm to make table_name public
#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaOrm {
    table_name: Option<syn::Lit>,
}

// bool is meant as "skip", so this entry cant be created/updated/deleted
pub type IdentTypeTuple = (syn::Ident, syn::Type, bool);

pub fn mutation_fn(item: syn::DataStruct, attrs_seaorm: SeaOrm) -> Result<TokenStream, crate::error::Error> {

    let fields: Vec<IdentTypeTuple> = item
        .fields
        .into_iter()
        .map(|field| {
            (
                field.ident.unwrap(),
                field.ty,
                field.attrs
                    .into_iter()
                    .any(|attr| {
                        if let Ok(c) = SeaographyMutation::from_attributes(&[attr]) {
                            c.skip.is_some()
                        } else {
                            false
                        }
                    })
            )
        })
        .collect();

    dbg!(fields.clone());


    let name = if let syn::Lit::Str(item) = attrs_seaorm.table_name.as_ref().unwrap() {
        Ok(item.value().parse::<TokenStream>()?)
    } else {
        Err(crate::error::Error::Internal(
            "Unreachable parse of query entities".into(),
        ))
    }?;

    let create_mutation_query = create_mutation(&name, &fields);
    let delete_mutation_query = delete_mutation(&name);
    let update_mutation_query = update_mutation(&name, &fields);

    let struct_name = Ident::new(&format!("{}Mutation", &name), Span::call_site());

    Ok(quote! {

        #[derive(Default)]
        pub struct #struct_name;

        #[async_graphql::Object]
        impl #struct_name {
            #create_mutation_query
            #delete_mutation_query
            #update_mutation_query
        }
    })

}

pub fn create_mutation(name: &TokenStream, fields: &[IdentTypeTuple]) -> TokenStream {

    let variables: Vec<TokenStream> = fields
        .iter()
        .filter(|(i, _, skip)| { dbg!(&*i); dbg!(&*skip); !*skip })
        .map(|(i, tp, _)| {
            quote! {
                #i: #tp,
            }
        })
        .collect();


    let variables_set: Vec<TokenStream> = fields
        .iter()
        .filter(|(_, _, i)| { !*i })
        .map(|(i, _, _)| {
            quote! {
                #i: Set(#i),
            }
        })
        .collect();

    let fn_name = Ident::new(&format!("create_{}", *name), Span::call_site());

    quote! {

        pub async fn #fn_name<'a>(
            &self,
            ctx: &async_graphql::Context<'a>,
            #(#variables)*
        ) -> async_graphql::Result<bool> {

            use async_graphql::*;
            use sea_orm::prelude::*;
            use sea_orm::{NotSet, Set, IntoActiveModel, ActiveModelTrait, ModelTrait, EntityTrait, QueryFilter, ColumnTrait};

            let db: &crate::DatabaseConnection = ctx.data::<crate::DatabaseConnection>().unwrap();
            
            let c = super::#name::ActiveModel {
                #(#variables_set)*
                ..Default::default()
            };

            let res = c.insert(db).await?;

            Ok(true)
        }
    }
}

pub fn delete_mutation(name: &TokenStream) -> TokenStream {

    let fn_name = Ident::new(&format!("delete_{}", *name), Span::call_site());

    quote! {

        pub async fn #fn_name<'a>(
            &self,
            ctx: &async_graphql::Context<'a>,
            filters: super::#name::Filter,
        ) -> async_graphql::Result<bool> {
            use async_graphql::*;
            use sea_orm::prelude::*;
            use sea_orm::{EntityTrait};
            use seaography::{EntityOrderBy, EntityFilter};

            let db: &crate::DatabaseConnection = ctx.data::<crate::DatabaseConnection>().unwrap();
            let res = super::#name::Entity::delete_many()
                .filter(filters.filter_condition())
                .exec(db)
                .await?;

            Ok(true)
        }
    }
}




pub fn update_mutation(name: &TokenStream, fields: &[IdentTypeTuple]) -> TokenStream {

    let variables: Vec<TokenStream> = fields
        .iter()
        .filter(|(i, _, skip)| { dbg!(&*i); dbg!(&*skip); !*skip })
        .map(|(i, tp, _)| {
            quote! {
                #i: Option<#tp>,
            }
        })
        .collect();


    let variables_set: Vec<TokenStream> = fields
        .iter()
        .filter(|(_, _, i)| { !*i })
        .map(|(i, _, _)| {
            quote! {
                if let v = #i.unwrap() {
                    c.#i = Set(v);
                }
            }
        })
        .collect();

    let fn_name = Ident::new(&format!("update_{}", *name), Span::call_site());

    quote! {

        pub async fn #fn_name<'a>(
            &self,
            ctx: &async_graphql::Context<'a>,
            filters: super::#name::Filter,
            #(#variables)*
        ) -> async_graphql::Result<bool> {

            use async_graphql::*;
            use sea_orm::prelude::*;
            use sea_orm::{NotSet, Set, IntoActiveModel, ActiveModelTrait, ModelTrait, EntityTrait, QueryFilter, ColumnTrait};
            use seaography::EntityFilter;

            let db: &crate::DatabaseConnection = ctx.data::<crate::DatabaseConnection>().unwrap();
            
            let mut c: super::#name::ActiveModel =Default::default();

            #(#variables_set)*

            let res = super::#name::Entity::update_many()
                .set(c)
                .filter(filters.filter_condition())
                .exec(db)
                .await?;

            Ok(true)
        }
    }
}
