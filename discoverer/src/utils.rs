use heck::ToUpperCamelCase;
use itertools::Itertools;
use sea_schema::sea_query::{
    ColumnDef, ForeignKeyCreateStatement, TableCreateStatement, TableForeignKey,
};
use seaography_types::{ColumnMeta, EnumMeta, RelationshipMeta, TableMeta};

use crate::{Result, TablesHashMap};

/// Used to extract relationship metadata from all tables
///
/// ```
/// use seaography_discoverer::extract_relationships_meta;
/// use seaography_discoverer::test_cfg::get_tables_hash_map;
/// use seaography_types::{ColumnMeta, ColumnType, RelationshipMeta};
///
/// let relations = vec![
///     RelationshipMeta {
///         src_table: "char".into(),
///         dst_table: "font".into(),
///         src_cols: vec![ColumnMeta {
///             col_name: "font_id".into(),
///             not_null: false,
///             col_type: ColumnType::Integer32,
///             is_primary_key: false,
///         }],
///         dst_cols: vec![ColumnMeta {
///             col_name: "id".into(),
///             not_null: true,
///             col_type: ColumnType::Integer32,
///             is_primary_key: true,
///         }]
///     }
/// ];
///
/// assert_eq!(extract_relationships_meta(&get_tables_hash_map()).unwrap(), relations)
/// ```
///
/// ```
/// use std::collections::HashMap;
/// use seaography_discoverer::{extract_relationships_meta, TablesHashMap};
///
/// let tables: TablesHashMap = HashMap::new();
///
/// assert_eq!(extract_relationships_meta(&tables).unwrap(), vec![])
/// ```
pub fn extract_relationships_meta(tables: &TablesHashMap) -> Result<Vec<RelationshipMeta>> {
    Ok(tables
        .iter()
        .map(|(table_name, table)| extract_table_relationship_meta(tables, table_name, table))
        .collect::<Result<Vec<Vec<RelationshipMeta>>>>()?
        .into_iter()
        .flatten()
        .unique()
        .collect())
}

/// Used to extract relationship meta from @table
///
/// ```
/// use seaography_discoverer::test_cfg::{get_char_table, get_tables_hash_map};
/// use seaography_discoverer::utils::extract_table_relationship_meta;
/// use seaography_types::{ColumnMeta, RelationshipMeta, ColumnType};
///
/// let left = extract_table_relationship_meta(
///     &get_tables_hash_map(),
///     &"char".into(),
///     &get_char_table()
/// ).unwrap();
///
/// let relations = vec![
///     RelationshipMeta {
///         src_table: "char".into(),
///         dst_table: "font".into(),
///         src_cols: vec![ColumnMeta {
///             col_name: "font_id".into(),
///             not_null: false,
///             col_type: ColumnType::Integer32,
///             is_primary_key: false,
///         }],
///         dst_cols: vec![ColumnMeta {
///             col_name: "id".into(),
///             not_null: true,
///             col_type: ColumnType::Integer32,
///             is_primary_key: true,
///         }]
///     }
/// ];
///
/// assert_eq!(left, relations);
/// ```
pub fn extract_table_relationship_meta(
    tables: &TablesHashMap,
    table_name: &String,
    table: &TableCreateStatement,
) -> Result<Vec<RelationshipMeta>> {
    Ok(table
        .get_foreign_key_create_stmts()
        .iter()
        .map(|fk: &ForeignKeyCreateStatement| fk.get_foreign_key())
        .map(|fk: &TableForeignKey| -> Result<Vec<RelationshipMeta>> {
            let dst_table = fk
                .get_ref_table()
                .ok_or("destination table not properly populated")?;

            let dst_table_stmt = tables
                .get(&dst_table)
                .ok_or("source table not properly populated")?;

            let src_cols: Vec<ColumnMeta> =
                extract_relationship_columns(fk.get_columns(), table.get_columns())?;

            let dst_cols: Vec<ColumnMeta> =
                extract_relationship_columns(fk.get_ref_columns(), dst_table_stmt.get_columns())?;

            if table_name.eq(&dst_table) {
                Ok(vec![
                    RelationshipMeta {
                        src_table: table_name.clone(),
                        dst_table: dst_table.clone(),
                        src_cols: src_cols.clone(),
                        dst_cols: dst_cols.clone(),
                    },
                    RelationshipMeta {
                        src_table: table_name.clone(),
                        dst_table,
                        src_cols: dst_cols,
                        dst_cols: src_cols,
                    },
                ])
            } else {
                Ok(vec![RelationshipMeta {
                    src_table: table_name.clone(),
                    dst_table,
                    src_cols,
                    dst_cols,
                }])
            }
        })
        .collect::<Result<Vec<Vec<RelationshipMeta>>>>()?
        .into_iter()
        .flatten()
        .collect())
}

/// Used to extract relationship columns from existing table column definitions
///
/// ```
/// use seaography_discoverer::test_cfg::get_char_table;
/// use seaography_discoverer::utils::extract_relationship_columns;
/// use seaography_types::{ColumnMeta, ColumnType};
///
/// let table_cols = get_char_table().get_columns().clone();
///
/// assert_eq!(
///     extract_relationship_columns(vec!["font_id".into()], &table_cols).unwrap(),
///     vec![
///         ColumnMeta {
///             col_name: "font_id".into(),
///             not_null: false,
///             is_primary_key: false,
///             col_type: ColumnType::Integer32,
///         }
///     ]
/// );
/// ```
pub fn extract_relationship_columns(
    col_names: Vec<String>,
    table_cols: &Vec<ColumnDef>,
) -> Result<Vec<ColumnMeta>> {
    col_names
        .iter()
        .map(|col_name| -> Result<ColumnMeta> {
            let col = table_cols
                .iter()
                .find(|col| col.get_column_name().eq(col_name))
                .ok_or("column definition not found")?;
            Ok(ColumnMeta::from_column_def(col))
        })
        .into_iter()
        .collect()
}

/// Used to extract TableMeta combining TableCreateStatements and RelationshipMeta
///
/// ```
/// use std::collections::HashMap;
/// use seaography_discoverer::extract_tables_meta;
/// use seaography_discoverer::test_cfg::get_tables_hash_map;
/// use seaography_types::{ColumnMeta, ColumnType, RelationshipMeta, TableMeta};
///
/// let tables = get_tables_hash_map();
///
/// let relations = vec![
///     RelationshipMeta {
///         src_table: "char".into(),
///         dst_table: "font".into(),
///         src_cols: vec![ColumnMeta {
///             col_name: "font_id".into(),
///             not_null: false,
///             col_type: ColumnType::Integer64,
///             is_primary_key: false,
///         }],
///         dst_cols: vec![ColumnMeta {
///             col_name: "id".into(),
///             not_null: true,
///             col_type: ColumnType::Integer64,
///             is_primary_key: true,
///         }]
///     }
/// ];
///
/// let result = vec![
///     TableMeta {
///             table_name: "char".into(),
///             columns: vec![
///                 ColumnMeta {
///                     col_name: "id".into(),
///                     not_null: true,
///                     is_primary_key: true,
///                     col_type: ColumnType::Integer32,
///                 },
///                 ColumnMeta {
///                     col_name: "character".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::String,
///                 },
///                 ColumnMeta {
///                     col_name: "size_w".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer32,
///                 },
///                 ColumnMeta {
///                     col_name: "size_h".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer32,
///                 },
///                 ColumnMeta {
///                     col_name: "font_id".into(),
///                     not_null: false,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer32,
///                 },
///                 ColumnMeta {
///                     col_name: "font_size".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer32,
///                 },
///             ],
///             relations: vec![RelationshipMeta {
///                 src_table: "char".into(),
///                 dst_table: "font".into(),
///                 src_cols: vec![ColumnMeta {
///                     col_name: "font_id".into(),
///                     not_null: false,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer64,
///                 }],
///                 dst_cols: vec![ColumnMeta {
///                     col_name: "id".into(),
///                     not_null: true,
///                     is_primary_key: true,
///                     col_type: ColumnType::Integer64,
///                 }],
///             }],
///        },
///         TableMeta {
///             table_name: "font".into(),
///             columns: vec![
///                 ColumnMeta {
///                     col_name: "id".into(),
///                     not_null: true,
///                     is_primary_key: true,
///                     col_type: ColumnType::Integer32,
///                 },
///                 ColumnMeta {
///                     col_name: "name".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::String,
///                 },
///                 ColumnMeta {
///                     col_name: "language".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::Enum("LanguageEnum".into()),
///                 },
///                 ColumnMeta {
///                     col_name: "variant".into(),
///                     not_null: true,
///                     is_primary_key: false,
///                     col_type: ColumnType::Enum("VariantEnum".into()),
///                 },
///             ],
///             relations: vec![RelationshipMeta {
///                 src_table: "char".into(),
///                 dst_table: "font".into(),
///                 src_cols: vec![ColumnMeta {
///                     col_name: "font_id".into(),
///                     not_null: false,
///                     is_primary_key: false,
///                     col_type: ColumnType::Integer64,
///                 }],
///                 dst_cols: vec![ColumnMeta {
///                     col_name: "id".into(),
///                     not_null: true,
///                     is_primary_key: true,
///                     col_type: ColumnType::Integer64,
///                }],
///             }],
///         }
/// ];
///
/// assert_eq!(
///     extract_tables_meta(&tables, &relations)
///         .into_iter()
///         .map(|table| (table.table_name.clone(), table))
///         .collect::<HashMap<_,_>>(),
///     result
///         .into_iter()
///         .map(|table| (table.table_name.clone(), table))
///         .collect::<HashMap<_,_>>()
/// );
/// ```
pub fn extract_tables_meta(
    tables: &TablesHashMap,
    relationships: &Vec<RelationshipMeta>,
) -> Vec<TableMeta> {
    tables
        .into_iter()
        .map(|(table_name, table_create_stmt)| {
            let columns: Vec<ColumnMeta> = table_create_stmt
                .get_columns()
                .iter()
                .map(|col| ColumnMeta::from_column_def(col))
                .collect();

            TableMeta {
                table_name: table_name.clone(),
                columns,
                relations: relationships
                    .iter()
                    .filter(|relation| {
                        relation.src_table.eq(table_name) || relation.dst_table.eq(table_name)
                    })
                    .map(|rel| rel.clone())
                    .collect(),
            }
        })
        .collect()
}

/// Used to extract enums from all tables
///
/// ```
/// use seaography_discoverer::extract_enums;
/// use seaography_discoverer::test_cfg::{get_language_enum, get_tables_hash_map, get_variant_enum};
/// let tables = get_tables_hash_map();
///
/// assert_eq!(extract_enums(&tables), vec![get_language_enum(), get_variant_enum()]);
/// ```
pub fn extract_enums(tables: &TablesHashMap) -> Vec<EnumMeta> {
    // extract enum meta from tables, the produced result contains duplicates
    let enums_mixed = tables
        .into_iter()
        .map(|(_, table_create_stmt)| extract_table_enums(table_create_stmt))
        .fold(
            Vec::<EnumMeta>::new(),
            |acc: Vec<EnumMeta>, cur: Vec<EnumMeta>| [acc, cur].concat(),
        );

    // enums can be used in multiple tables, remove duplicates
    enums_mixed
        .into_iter()
        .unique_by(|enumeration| enumeration.enum_name.clone())
        .collect_vec()
}

/// Used to extract EnumMeta from given table_create_stmt
///
/// ```
/// use seaography_discoverer::test_cfg::{get_basic_fonts_table, get_language_enum};
/// use seaography_discoverer::utils::extract_table_enums;
/// use seaography_types::EnumMeta;
///
/// let table_create_stmt = get_basic_fonts_table();
///
/// assert_eq!(extract_table_enums(&table_create_stmt), vec![get_language_enum()]);
/// ```
///
/// ```
/// use seaography_discoverer::test_cfg::{get_complex_fonts_table, get_language_enum, get_variant_enum};
/// use seaography_discoverer::utils::extract_table_enums;
/// use seaography_types::EnumMeta;
///
/// let table_create_stmt = get_complex_fonts_table();
///
/// assert_eq!(extract_table_enums(&table_create_stmt), vec![get_language_enum(), get_variant_enum()]);
/// ```
///
/// ```
/// use seaography_discoverer::test_cfg::{get_char_table};
/// use seaography_discoverer::utils::extract_table_enums;
/// use seaography_types::EnumMeta;
///
/// let table_create_stmt = get_char_table();
///
/// assert_eq!(extract_table_enums(&table_create_stmt), vec![]);
/// ```
pub fn extract_table_enums(table_create_stmt: &TableCreateStatement) -> Vec<EnumMeta> {
    table_create_stmt
        .get_columns()
        .iter()
        .filter(|col| match col.get_column_type().unwrap() {
            sea_schema::sea_query::ColumnType::Enum(_, _) => true,
            _ => false,
        })
        .map(|col| match col.get_column_type().unwrap() {
            sea_schema::sea_query::ColumnType::Enum(name, values) => EnumMeta {
                enum_name: name.to_upper_camel_case(),
                enum_values: values.clone(),
            },
            _ => panic!("NOT REACHABLE"),
        })
        .collect()
}
