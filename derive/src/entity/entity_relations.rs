/// Used to generate entity relation fields
///
/// ```
/// use quote::quote;
/// use seaography_generator::files::entity::generate_entity_relations;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = generate_entity_relations(&get_font_table()).iter().map(|t| t.to_string()).collect();;
/// let right: Vec<_>  = vec![
///     quote!{
///         pub async fn font_char<'a>(&self, ctx: &async_graphql::Context<'a>) -> Vec<crate::orm::char::Model> {
///             let data_loader = ctx.data::<async_graphql::dataloader::DataLoader<OrmDataloader>>().unwrap();
///             let key = IdCharFK(self.id.clone().try_into().unwrap());
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
            let destination_table_module: TokenStream = destination_table_module.parse().unwrap();

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

                    let key = #fk_name(#(self.#key_items.clone().try_into().unwrap()),*);

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

            let field_indexes: Vec<Literal> = (0..relationship.src_cols.clone().len()).map(Literal::usize_unsuffixed).collect();

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
                            model.#name.as_ref().unwrap().clone()
                        }
                    } else if !source_optional && destination_optional {
                        quote! {
                            Some(model.#name.clone().try_into().unwrap())
                        }
                    } else {
                        quote! {
                            model.#name.clone().try_into().unwrap()
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
                                    let key = #fk_name(#(#destination_fields),*);

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