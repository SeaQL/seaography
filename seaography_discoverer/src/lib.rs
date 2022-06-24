use std::collections::HashMap;

use clap::Parser;

use seaography_types::{relationship_meta::RelationshipMeta, column_meta::ColumnMeta, table_meta::TableMeta};
use sqlx::SqlitePool;

use sea_schema::{
    sea_query::{TableCreateStatement, TableForeignKey, ForeignKeyCreateStatement},
    sqlite::{
        def::TableDef,
        discovery::{DiscoveryResult, SchemaDiscovery},
    },
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub url: String,
}

pub type TablesHashMap = HashMap<String, TableCreateStatement>;

pub async fn explore_sqlite(url: &String) -> DiscoveryResult<TablesHashMap> {
    let connection = SqlitePool::connect(url)
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.name.clone(), table.write()))
        .collect();

    Ok(tables)
}

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