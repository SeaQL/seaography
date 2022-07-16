use std::path::Path;
use heck::ToUpperCamelCase;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use seaography_types::{
    column_meta::ColumnMeta, column_type::ColumnType, relationship_meta::RelationshipMeta,
    table_meta::TableMeta,
};
use itertools::Itertools;

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
/// use seaography_generator::generator::entity::generate_entity_filters;
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
        .filter(|column| {
            match column.col_type {
                // TODO support enum type
                ColumnType::Binary => false,
                ColumnType::Enum(_) => false,
                _ => true
            }
        })
        .map(|column: &ColumnMeta| {
            let column_name = column.snake_case_ident();
            let column_filter_type = column.get_base_type();

            quote! {
                pub #column_name: Option<TypeFilter<#column_filter_type>>
            }
        })
        .collect()
}

/// Used to contract entity field getters
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::entity::generate_entity_getters;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = generate_entity_getters(&get_font_table()).iter().map(|t| t.to_string()).collect();;
/// let right: Vec<_>  = vec![
///     quote!{
///         pub async fn id(&self) -> &i32 {
///             &self.id
///         }
///     },
///     quote!{
///         pub async fn name(&self) -> &String {
///             &self.name
///         }
///     },
///     quote!{
///         pub async fn language(&self) -> LanguageEnum {
///             self.language.clone().map(|i| i.into())
///         }
///     },
///     quote!{
///         pub async fn variant(&self) -> VariantEnum {
///             self.variant.clone().map(|i| i.into())
///         }
///     },
/// ].iter().map(|t| t.to_string()).collect();
///
/// assert_eq!(left, right);
/// ```
pub fn generate_entity_getters(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .columns
        .iter()
        .map(|column: &ColumnMeta| {
            let column_name = column.snake_case_ident();
            let column_type = column.get_type();

            match column.col_type {
                ColumnType::Enum(_) => quote! {
                    pub async fn #column_name(&self) -> #column_type {
                        self.#column_name.clone().map(|i| i.into())
                    }
                },
                _ => quote! {
                    pub async fn #column_name(&self) -> &#column_type {
                        &self.#column_name
                    }
                }
            }
        })
        .collect()
}

/// Used to generate entity relation fields
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::entity::generate_entity_relations;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = generate_entity_relations(&get_font_table()).iter().map(|t| t.to_string()).collect();;
/// let right: Vec<_>  = vec![
///     quote!{
///         pub async fn id_char<'a>(&self, ctx: &async_graphql::Context<'a>) -> Vec<crate::orm::char::Model> {
///             let data_loader = ctx.data::<async_graphql::dataloader::DataLoader<OrmDataloader>>().unwrap();
///             let key = IdCharFK(self.id.clone());
///             let data: Option<_> = data_loader.load_one(key).await.unwrap();
///             data.unwrap_or(vec![])
///         }
///     },
/// ].iter().map(|t| t.to_string()).collect();
///
/// assert_eq!(left, right);
/// ```
///
/// TODO: write test 1-1
/// TODO: write test 1-N
/// TODO: write test N-M
pub fn generate_entity_relations(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .relations
        .iter()
        .unique_by(|relationship| relationship.retrieve_foreign_key(relationship.is_reverse(&table_meta.table_name)))
        .map(|relationship: &RelationshipMeta| {
            let reverse = relationship.is_reverse(&table_meta.table_name);

            let fk_name: TokenStream = relationship.retrieve_foreign_key(reverse).parse().unwrap();

            let source_columns = if reverse { &relationship.dst_cols } else { &relationship.src_cols };

            let destination_table_module = &relationship.snake_case(reverse);
            let relation_name: TokenStream = relationship.retrieve_name(reverse).parse().unwrap();
            let destination_table_module: TokenStream = format!("{}", destination_table_module).parse().unwrap();

            let return_type: TokenStream = if reverse {
                quote! {
                    Vec<crate::orm::#destination_table_module::Model>
                }
            } else if relationship.is_optional(reverse) {
                quote! {
                    Option<crate::orm::#destination_table_module::Model>
                }
            } else {
                quote! {
                    crate::orm::#destination_table_module::Model
                }
            };

            // TODO add filter on relation
            // filters: Option<entities::#table_filter>,

            let key_items: Vec<TokenStream> = source_columns
                .iter()
                .map(|col: &ColumnMeta| {
                    col.snake_case_ident()
                })
                .collect();

            let return_value: TokenStream = if reverse {
                quote! {
                    data.unwrap_or(vec![])
                }
            } else if relationship.is_optional(reverse) {
                quote! {
                    data
                }
            } else {
                quote! {
                    data.unwrap()
                }
            };

            quote! {
                pub async fn #relation_name<'a>(
                    &self,
                    ctx: &async_graphql::Context<'a>
                ) -> #return_type {
                    let data_loader = ctx.data::<async_graphql::dataloader::DataLoader<OrmDataloader>>().unwrap();

                    let key = #fk_name(#(self.#key_items.clone()),*);

                    let data: Option<_> = data_loader.load_one(key).await.unwrap();

                    #return_value
                }
            }
        })
        .collect()
}

/// Used to generate FK structs and dataloader getter functions for every relation of entity
///
/// TODO: Add tests
/// TODO: 1-1
/// TODO: 1-N
/// TODO: N-1
/// TODO: N-M
pub fn generate_foreign_keys_and_loaders(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .relations
        .iter()
        .unique_by(|relationship| relationship.retrieve_foreign_key(relationship.is_reverse(&table_meta.table_name)))
        .map(|relationship: &RelationshipMeta| {
            let reverse = relationship.is_reverse(&table_meta.table_name);

            let field_indexes: Vec<Literal> = (0..relationship.src_cols.clone().len()).map(|n| Literal::usize_unsuffixed(n)).collect();

            let destination_table_module = relationship.snake_case_ident(reverse);

            let source_columns = if reverse { &relationship.dst_cols } else { &relationship.src_cols };

            let destination_columns = if reverse { &relationship.src_cols } else { &relationship.dst_cols };
            let destination_column_names: Vec<TokenStream> = destination_columns.iter().map(|column| column.snake_case_ident()).collect();
            let destination_column_enum_names: Vec<TokenStream> = destination_columns.iter().map(|column| column.camel_case_ident()).collect();


            let fk_name: TokenStream = relationship.retrieve_foreign_key(reverse).parse().unwrap();

            let return_type: TokenStream = if reverse {
                quote! {
                    Vec<crate::orm::#destination_table_module::Model>
                }
            } else {
                quote! {
                    crate::orm::#destination_table_module::Model
                }
            };

            let source_field_types: Vec<TokenStream> = source_columns.iter().map(|column| column.get_type()).collect();

            let destination_fields: Vec<TokenStream> = destination_column_names
                .iter()
                .enumerate()
                .map(|(index, name)|{
                    let source_optional = !destination_columns[index].not_null;
                    let destination_optional = !source_columns[index].not_null;

                    if source_optional && !destination_optional {
                        quote! {
                            model.#name.unwrap()
                        }
                    } else if !source_optional && destination_optional {
                        quote! {
                            Some(model.#name)
                        }
                    } else {
                        quote! {
                            model.#name
                        }
                    }
                })
                .collect();

            let prepare_dependencies = if reverse {
                quote! {
                    use itertools::Itertools;
                }
            } else {
                quote! {

                }
            };

            let prepare_step = if reverse {
                quote! {
                    .into_group_map()
                }
            } else {
                quote!{
                    .collect()
                }
            };

            quote! {
                #[derive(Clone, Eq, PartialEq, Hash, Debug)]
                pub struct #fk_name(#(#source_field_types),*);

                #[async_trait::async_trait]
                impl async_graphql::dataloader::Loader<#fk_name> for OrmDataloader {
                    type Value = #return_type;
                    type Error = std::sync::Arc<sea_orm::error::DbErr>;

                    async fn load(&self, keys: &[#fk_name]) -> Result<std::collections::HashMap<#fk_name, Self::Value>, Self::Error> {
                        let filter = sea_orm::Condition::all()
                            .add(
                                sea_orm::sea_query::SimpleExpr::Binary(
                                    Box::new(
                                        sea_orm::sea_query::SimpleExpr::Tuple(vec![
                                            #(sea_orm::sea_query::Expr::col(crate::orm::#destination_table_module::Column::#destination_column_enum_names.as_column_ref()).into_simple_expr()),*
                                        ])
                                    ),
                                    sea_orm::sea_query::BinOper::In,
                                    Box::new(
                                        sea_orm::sea_query::SimpleExpr::Tuple(
                                            keys
                                                .iter()
                                                .map(|tuple|
                                                    sea_orm::sea_query::SimpleExpr::Values(vec![#(tuple.#field_indexes.clone().into()),*])
                                                )
                                                .collect()
                                        )
                                    )
                                )
                            );

                        #prepare_dependencies

                        Ok(
                            crate::orm::#destination_table_module::Entity::find()
                                .filter(filter)
                                .all(&self.db)
                                .await?
                                .into_iter()
                                .map(|model| {
                                    let key = #fk_name(#(#destination_fields.clone()),*);

                                    (key, model)
                                })
                                #prepare_step
                        )
                    }
                }
            }
        })
        .collect()
}

/// Used to generate filter function for an entity.
///
/// The filter function used to assist the generation of SQL where queries
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::entity::generate_recursive_filter_fn;
/// use seaography_generator::test_cfg::get_char_table;
///
/// let left = generate_recursive_filter_fn(&get_char_table());
///
/// let right = quote!{
///     pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
///         let mut condition = sea_orm::Condition::all();
///         if let Some(current_filter) = root_filter {
///             if let Some(or_filters) = current_filter.or {
///                 let or_condition = or_filters
///                     .into_iter()
///                     .fold(sea_orm::Condition::any(), |fold_condition, filter|
///                         fold_condition.add(filter_recursive(Some(*filter))));
///                 condition = condition.add(or_condition);
///             }
///             if let Some(and_filters) = current_filter.and {
///                 let and_condition = and_filters
///                     .into_iter()
///                     .fold(sea_orm::Condition::all(), |fold_condition, filter|
///                         fold_condition.add(filter_recursive(Some(*filter)))
///                     );
///                 condition = condition.add(and_condition);
///             }
///             if let Some(id) = current_filter.id {
///                 if let Some(eq_value) = id.eq {
///                     condition = condition.add(Column::Id.eq(eq_value))
///                 }
///                 if let Some(ne_value) = id.ne {
///                     condition = condition.add(Column::Id.ne(ne_value))
///                 }
///             }
///             if let Some(character) = current_filter.character {
///                 if let Some(eq_value) = character.eq {
///                     condition = condition.add(Column::Character.eq(eq_value))
///                 }
///                 if let Some(ne_value) = character.ne {
///                     condition = condition.add(Column::Character.ne(ne_value))
///                 }
///             }
///             if let Some(size_w) = current_filter.size_w {
///                 if let Some(eq_value) = size_w.eq {
///                     condition = condition.add(Column::SizeW.eq(eq_value))
///                 }
///                 if let Some(ne_value) = size_w.ne {
///                     condition = condition.add(Column::SizeW.ne(ne_value))
///                 }
///             }
///             if let Some(size_h) = current_filter.size_h {
///                 if let Some(eq_value) = size_h.eq {
///                     condition = condition.add(Column::SizeH.eq(eq_value))
///                 }
///                 if let Some(ne_value) = size_h.ne {
///                     condition = condition.add(Column::SizeH.ne(ne_value))
///                 }
///             }
///             if let Some(font_id) = current_filter.font_id {
///                 if let Some(eq_value) = font_id.eq {
///                     condition = condition.add(Column::FontId.eq(eq_value))
///                 }
///                 if let Some(ne_value) = font_id.ne {
///                     condition = condition.add(Column::FontId.ne(ne_value))
///                 }
///             }
///             if let Some(font_size) = current_filter.font_size {
///                 if let Some(eq_value) = font_size.eq {
///                     condition = condition.add(Column::FontSize.eq(eq_value))
///                 }
///                 if let Some(ne_value) = font_size.ne {
///                     condition = condition.add(Column::FontSize.ne(ne_value))
///                 }
///             }
///         }
///         condition
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_recursive_filter_fn(table_meta: &TableMeta) -> TokenStream {
    let columns_filters: Vec<TokenStream> = table_meta
        .columns
        .iter()
        .filter(|column| {
            match column.col_type {
                // TODO support enum type
                ColumnType::Binary => false,
                ColumnType::Enum(_) => false,
                _ => true
            }
        })
        .map(|column: &ColumnMeta| {
            let column_name = column.snake_case_ident();
            let column_enum_name = column.camel_case_ident();

            quote! {
                if let Some(#column_name) = current_filter.#column_name {
                    if let Some(eq_value) = #column_name.eq {
                        condition = condition.add(Column::#column_enum_name.eq(eq_value))
                    }

                    if let Some(ne_value) = #column_name.ne {
                        condition = condition.add(Column::#column_enum_name.ne(ne_value))
                    }
                }
            }
        })
        .collect();

    quote! {
        pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();

            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::any(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(or_condition);
                }

                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::all(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(and_condition);
                }

                #(#columns_filters)*
            }

            condition
        }
    }
}

/// Used to extract enumerations from table meta
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::entity::extract_enums;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = extract_enums(&get_font_table()).iter().map(|t| t.to_string()).collect();
///
/// let right: Vec<_> = vec![
///     quote! {
///         LanguageEnum
///     },
///     quote! {
///         VariantEnum
///     },
///
/// ].iter().map(|t| t.to_string()).collect();
///
/// assert_eq!(left, right);
/// ```
pub fn extract_enums(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .columns
        .iter()
        .filter(|col| matches!(col.col_type, ColumnType::Enum(_)))
        .map(|col| {
            if let ColumnType::Enum(name) = &col.col_type {
                name.to_upper_camel_case().parse().unwrap()
            } else {
                panic!("UNREACHABLE")
            }
        })
        .collect()
}

pub fn write_graphql_entity<P: AsRef<Path>>(
    path: &P,
    table_meta: &TableMeta,
) -> std::io::Result<()> {
    let file_name = path
        .as_ref()
        .join(format!("{}.rs", table_meta.snake_case()));

    let data = generate_entity(table_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
