/// Used to generate filter function for an entity.
///
/// The filter function used to assist the generation of SQL where queries
///
/// TODO fix test
///
/// ```no_run
/// use quote::quote;
/// use seaography_generator::files::entity::generate_recursive_filter_fn;
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
///             [...]
///         }
///         condition
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
///
/// TODO support all query expressions
/// https://docs.rs/sea-query/latest/sea_query/expr/struct.Expr.html
///
pub fn generate_recursive_filter_fn(table_meta: &TableMeta) -> TokenStream {
    let columns_filters: Vec<TokenStream> = table_meta
        .columns
        .iter()
        .filter(|column| !matches!(column.col_type, ColumnType::Binary | ColumnType::Enum(_)))
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

                    if let Some(gt_value) = #column_name.gt {
                        condition = condition.add(Column::#column_enum_name.gt(gt_value))
                    }

                    if let Some(gte_value) = #column_name.gte {
                        condition = condition.add(Column::#column_enum_name.gte(gte_value))
                    }

                    if let Some(lt_value) = #column_name.lt {
                        condition = condition.add(Column::#column_enum_name.lt(lt_value))
                    }

                    if let Some(lte_value) = #column_name.lte {
                        condition = condition.add(Column::#column_enum_name.lte(lte_value))
                    }

                    if let Some(is_in_value) = #column_name.is_in {
                        condition = condition.add(Column::#column_enum_name.is_in(is_in_value))
                    }

                    if let Some(is_not_in_value) = #column_name.is_not_in {
                        condition = condition.add(Column::#column_enum_name.is_not_in(is_not_in_value))
                    }

                    if let Some(is_null_value) = #column_name.is_null {
                        if is_null_value {
                            condition = condition.add(Column::#column_enum_name.is_null())
                        }
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