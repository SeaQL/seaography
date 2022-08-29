use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct Seaography {
    entity: syn::Lit,
}

pub fn root_query_fn(
    ident: &syn::Ident,
    attrs: &Vec<Seaography>,
) -> Result<TokenStream, crate::error::Error> {
    let paths = attrs
        .iter()
        .map(|attribute| -> Result<TokenStream, crate::error::Error> {
            if let syn::Lit::Str(item) = &attribute.entity {
                Ok(item.value().parse::<TokenStream>()?)
            } else {
                Err(crate::error::Error::Error(
                    "Unreachable parse of query entities".into(),
                ))
            }
        })
        .collect::<Result<Vec<TokenStream>, crate::error::Error>>()?;

    let names: Vec<TokenStream> = paths
        .iter()
        .map(|path| {
            let name = path.clone().into_iter().last().unwrap().to_string();
            let name = format!("Paginated{}Result", name.to_upper_camel_case());

            quote! {
                #[graphql(concrete(name = #name, params(#path::Model)))]
            }
        })
        .collect();

    let queries: Vec<TokenStream> = paths
        .iter()
        .map(|path| {
            let name = format_ident!("{}", path.clone().into_iter().last().unwrap().to_string());


            quote!{
                pub async fn #name<'a>(
                    &self,
                    ctx: &async_graphql::Context<'a>,
                    filters: Option<#path::Filter>,
                    pagination: Option<PaginationInput>,
                    order_by: Option<#path::OrderBy>,
                ) -> PaginatedResult<#path::Model> {
                    use sea_orm::prelude::*;

                    println!("filters: {:?}", filters);
                    let db: &crate::DatabaseConnection = ctx.data::<crate::DatabaseConnection>().unwrap();
                    let stmt = #path::Entity::find()
                        .filter(#path::filter_recursive(filters));

                    let stmt = #path::order_by(stmt, order_by);

                    if let Some(pagination) = pagination {
                        let paginator = stmt.paginate(db, pagination.limit);
                        let data: Vec<#path::Model> =
                            paginator.fetch_page(pagination.page).await.unwrap();
                        let pages = paginator.num_pages().await.unwrap();
                        PaginatedResult {
                            data,
                            pages,
                            current: pagination.page,
                        }
                    } else {
                        let data: Vec<#path::Model> = stmt.all(db).await.unwrap();
                        PaginatedResult {
                            data,
                            pages: 1,
                            current: 1,
                        }
                    }

                }
            }
        })
        .collect();

    Ok(quote! {
        #[derive(Debug, async_graphql::InputObject)]
        pub struct PaginationInput {
            pub limit: usize,
            pub page: usize,
        }

        #[derive(Debug, async_graphql::SimpleObject)]
        #(#names)*
        pub struct PaginatedResult<T: async_graphql::ObjectType> {
            pub data: Vec<T>,
            pub pages: usize,
            pub current: usize,
        }

        #[async_graphql::Object]
        impl #ident {
            #(#queries)*
        }
    })
}
