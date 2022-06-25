use std::collections::HashMap;

use clap::Parser;

use itertools::Itertools;
use seaography_types::{relationship_meta::RelationshipMeta, column_meta::ColumnMeta, table_meta::TableMeta, enum_meta::EnumMeta};

use sea_schema::{
    sea_query::{TableCreateStatement, TableForeignKey, ForeignKeyCreateStatement}
};

pub mod sqlite;
pub use sqlite::explore_sqlite;

pub mod mysql;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub url: String,
}

pub type TablesHashMap = HashMap<String, TableCreateStatement>;

pub fn extract_relationships_meta(tables: &TablesHashMap) -> Vec<RelationshipMeta> {
    tables
        .iter()
        .map(|(table_name, table)| {
            table
                .get_foreign_key_create_stmts()
                .iter()
                .map(|fk: &ForeignKeyCreateStatement| fk.get_foreign_key())
                .map(|fk: &TableForeignKey| {
                    let dst_table = fk.get_ref_table().unwrap();

                    let dst_table_stmt = tables.get(&dst_table).unwrap();

                    let src_cols: Vec<ColumnMeta> = fk
                        .get_columns()
                        .iter()
                        .map(|col_name| {
                            let col = table
                                .get_columns()
                                .iter()
                                .find(|col| col.get_column_name().eq(col_name))
                                .unwrap();
                            ColumnMeta::from_column_def(col)
                        })
                        .collect();

                    let dst_cols: Vec<ColumnMeta> = fk
                        .get_ref_columns()
                        .iter()
                        .map(|col_name| {
                            let col = dst_table_stmt
                                .get_columns()
                                .iter()
                                .find(|col| col.get_column_name().eq(col_name))
                                .unwrap();
                            ColumnMeta::from_column_def(col)
                        })
                        .collect();

                    RelationshipMeta {
                        src_table: table_name.clone(),
                        dst_table,
                        src_cols,
                        dst_cols,
                    }
                })
                .collect()
        })
        .fold(
            Vec::<RelationshipMeta>::new(),
            |acc: Vec<RelationshipMeta>, cur: Vec<RelationshipMeta>| [acc, cur].concat(),
        )
}

pub fn extract_tables_meta (tables: &TablesHashMap, relationships: &Vec<RelationshipMeta>) -> Vec<TableMeta> {
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

pub fn extract_enums (tables: &TablesHashMap) -> Vec<EnumMeta> {
    let enums_mixed = tables
        .into_iter()
        .map(|(_, table_create_stmt)| {
            table_create_stmt.get_columns().iter().filter(|col| {
                match col.get_column_type().unwrap() {
                     sea_schema::sea_query::ColumnType::Enum(_,_) => true,
                    _ => false
                }
            }).map(|col| {
                match col.get_column_type().unwrap() {
                    sea_schema::sea_query::ColumnType::Enum(name, values) => EnumMeta {
                        enum_name: name.clone(),
                        enum_values: values.clone()
                    },
                   _ => panic!("NOT REACHABLE")
               }
            }).collect()
        })
        .fold(
            Vec::<EnumMeta>::new(),
            |acc: Vec<EnumMeta>, cur: Vec<EnumMeta>| [acc, cur].concat(),
        );

    enums_mixed.into_iter().unique_by(|enumeration| enumeration.enum_name.clone()).collect_vec()
}