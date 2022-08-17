pub fn generate_entity(table_meta: &TableMeta) -> TokenStream {
    let entity_module = table_meta.snake_case_ident();
    let entity_name = table_meta.camel_case();
    let entity_filter = format!("{}Filter", table_meta.camel_case());

    let filters: Vec<TokenStream> = generate_entity_filters(table_meta);
    let getters: Vec<TokenStream> = generate_entity_getters(table_meta);
    let relations: Vec<TokenStream> = generate_entity_relations(table_meta);
    let foreign_keys: Vec<TokenStream> = generate_foreign_keys_and_loaders(table_meta);
    let recursive_filter_fn: TokenStream = generate_recursive_filter_fn(table_meta);

    let enumerations: Vec<TokenStream> = extract_enums(table_meta);

    quote! {
        use sea_orm::prelude::*;

        #(use crate::graphql::enums::#enumerations;)*

        #recursive_filter_fn

        pub use crate::orm::#entity_module::*;
        use crate::graphql::*;

        #[async_graphql::Object(name=#entity_name)]
        impl Model {
            #(#getters)*
            #(#relations)*
        }

        #[derive(async_graphql::InputObject, Debug)]
        #[graphql(name=#entity_filter)]
        pub struct Filter {
            pub or: Option<Vec<Box<Filter>>>,
            pub and: Option<Vec<Box<Filter>>>,
            #(#filters),*
        }

        #(#foreign_keys)*
    }
}

/// Used to generate filter struct fields for entity
///
/// ```
/// use quote::quote;
/// use seaography_generator::files::entity::generate_entity_filters;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = generate_entity_filters(&get_font_table()).iter().map(|t| t.to_string()).collect();
///
/// let right: Vec<_>  = vec![
///     quote!{
///         pub id: Option<TypeFilter<i32>>
///     },
///     quote!{
///         pub name: Option<TypeFilter<String>>
///     },
/// ].iter().map(|t| t.to_string()).collect();
///
/// assert_eq!(left, right)
/// ```
pub fn generate_entity_filters(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .columns
        .iter()
        .filter(|column| !matches!(column.col_type, ColumnType::Binary | ColumnType::Enum(_)))
        .map(|column: &ColumnMeta| {
            let column_name = column.snake_case_ident();
            let column_filter_type = column.get_base_type();

            quote! {
                pub #column_name: Option<TypeFilter<#column_filter_type>>
            }
        })
        .collect()
}