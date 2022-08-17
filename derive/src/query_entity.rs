/// Used to generate a root query for the current table_meta
///
/// ```
/// use quote::quote;
/// use seaography_generator::files::root_node::generate_table_query;
/// use seaography_generator::test_cfg::get_char_table;
///
/// let char_table_meta = get_char_table();
///
/// let left = generate_table_query(&char_table_meta);
///
/// let right = quote!{
///     async fn char<'a>(
///         &self, ctx: &async_graphql::Context<'a>,
///         filters: Option<entities::char::Filter>,
///         pagination: Option<PaginationInput>,
///     ) -> PaginatedResult<entities::char::Model> {
///           println!("filters: {:?}", filters);
///
///           let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
///
///           let stmt = entities::char::Entity::find()
///               .filter(entities::char::filter_recursive(filters));
///
///           if let Some(pagination) = pagination {
///               let paginator = stmt
///                   .paginate(db, pagination.limit);
///
///               let data: Vec<entities::char::Model> = paginator
///                   .fetch_page(pagination.page)
///                   .await
///                   .unwrap();
///
///               let pages = paginator
///                   .num_pages()
///                   .await
///                   .unwrap();
///
///               PaginatedResult {
///                   data,
///                   pages,
///                   current: pagination.page
///               }
///           } else {
///               let data: Vec<entities::char::Model> = stmt
///                   .all(db)
///                   .await
///                   .unwrap();
///
///               PaginatedResult {
///                   data,
///                   pages: 1,
///                   current: 1
///               }
///           }
///       }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_table_query(table_meta: &TableMeta) -> TokenStream {
    let entity_module = table_meta.snake_case_ident();

    quote! {
        async fn #entity_module<'a>(
            &self, ctx: &async_graphql::Context<'a>,
            filters: Option<entities::#entity_module::Filter>,
            pagination: Option<PaginationInput>,
        ) -> PaginatedResult<entities::#entity_module::Model> {
            println!("filters: {:?}", filters);

            let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();

            let stmt = entities::#entity_module::Entity::find()
                .filter(entities::#entity_module::filter_recursive(filters));

            if let Some(pagination) = pagination {
                let paginator = stmt
                    .paginate(db, pagination.limit);

                let data: Vec<entities::#entity_module::Model> = paginator
                    .fetch_page(pagination.page)
                    .await
                    .unwrap();

                let pages = paginator
                    .num_pages()
                    .await
                    .unwrap();

                PaginatedResult {
                    data,
                    pages,
                    current: pagination.page
                }
            } else {
                let data: Vec<entities::#entity_module::Model> = stmt
                    .all(db)
                    .await
                    .unwrap();

                PaginatedResult {
                    data,
                    pages: 1,
                    current: 1
                }
            }
        }
    }
}