use itertools::Itertools;
use sea_schema::sea_query::{ForeignKeyCreateStatement, TableForeignKey};
use seaography_types::{
    column_meta::ColumnMeta, enum_meta::EnumMeta, relationship_meta::RelationshipMeta,
    table_meta::TableMeta,
};

use crate::{Result, TablesHashMap, Error};

pub fn extract_relationships_meta(tables: &TablesHashMap) -> Result<Vec<RelationshipMeta>> {
    tables
        .iter()
        .map(|(table_name, table)| -> Result<Vec<RelationshipMeta>> {
            table
                .get_foreign_key_create_stmts()
                .iter()
                .map(|fk: &ForeignKeyCreateStatement| fk.get_foreign_key())
                .map(|fk: &TableForeignKey| -> Result<RelationshipMeta> {
                    let dst_table = fk
                        .get_ref_table()
                        .ok_or("destination table not properly populated")?;

                    let dst_table_stmt = tables
                        .get(&dst_table)
                        .ok_or("destination table not properly populated")?;

                    let src_cols: Vec<ColumnMeta> = fk
                        .get_columns()
                        .iter()
                        .map(|col_name| -> Result<ColumnMeta> {
                            let col = table
                                .get_columns()
                                .iter()
                                .find(|col| col.get_column_name().eq(col_name))
                                .ok_or("column definition not found")?;
                            Ok(ColumnMeta::from_column_def(col))
                        })
                        .collect::<Result<Vec<_>>>()?;

                    let dst_cols: Vec<ColumnMeta> = fk
                        .get_ref_columns()
                        .iter()
                        .map(|col_name| -> Result<ColumnMeta> {
                            let col = dst_table_stmt
                                .get_columns()
                                .iter()
                                .find(|col| col.get_column_name().eq(col_name))
                                .ok_or("column definition not found")?;
                            Ok(ColumnMeta::from_column_def(col))
                        })
                        .collect::<Result<Vec<_>>>()?;

                    Ok(RelationshipMeta {
                        src_table: table_name.clone(),
                        dst_table,
                        src_cols,
                        dst_cols,
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .try_fold(
            Vec::<RelationshipMeta>::new(),
            |acc: Vec<RelationshipMeta>,
             cur: Result<Vec<RelationshipMeta>>|
             -> Result<Vec<RelationshipMeta>> { Ok([acc, cur?].concat()) },
        )
}

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

pub fn extract_enums(tables: &TablesHashMap) -> Vec<EnumMeta> {
    let enums_mixed = tables
        .into_iter()
        .map(|(_, table_create_stmt)| {
            table_create_stmt
                .get_columns()
                .iter()
                .filter(|col| match col.get_column_type().unwrap() {
                    sea_schema::sea_query::ColumnType::Enum(_, _) => true,
                    _ => false,
                })
                .map(|col| match col.get_column_type().unwrap() {
                    sea_schema::sea_query::ColumnType::Enum(name, values) => EnumMeta {
                        enum_name: name.clone(),
                        enum_values: values.clone(),
                    },
                    _ => panic!("NOT REACHABLE"),
                })
                .collect()
        })
        .fold(
            Vec::<EnumMeta>::new(),
            |acc: Vec<EnumMeta>, cur: Vec<EnumMeta>| [acc, cur].concat(),
        );

    enums_mixed
        .into_iter()
        .unique_by(|enumeration| enumeration.enum_name.clone())
        .collect_vec()
}

pub fn parse_database_url(database_url: &String) -> Result<url::Url> {
    let url = url::Url::parse(&database_url)?;

    // Make sure we have all the required url components
    //
    // Missing scheme will have been caught by the Url::parse() call
    // above
    let url_username = url.username();
    let url_host = url.host_str();

    let is_sqlite = url.scheme() == "sqlite";

    // Skip checking if it's SQLite
    if !is_sqlite {
        // Panic on any that are missing
        if url_username.is_empty() {
            return Err(Error::Error("No username was found in the database url".into()))
        }
        if url_host.is_none() {
            return Err(Error::Error("No host was found in the database url".into()))
        }
    }

    //
    // Make sure we have database name
    //
    if !is_sqlite {
        // The database name should be the first element of the path string
        //
        // Throwing an error if there is no database name since it might be
        // accepted by the database without it, while we're looking to dump
        // information from a particular database
        let database_name = url
            .path_segments()
            .ok_or(Error::Error(format!("There is no database name as part of the url path: {}", url.as_str())))?
            .next()
            .unwrap();

        // An empty string as the database name is also an error
        if database_name.is_empty() {
            return Err(Error::Error(format!("There is no database name as part of the url path: {}", url.as_str())))
        }
    }

    Ok(url)
}