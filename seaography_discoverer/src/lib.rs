use clap::Parser;
use sea_schema::sea_query::TableCreateStatement;
use std::collections::HashMap;

pub mod sqlite;
pub use sqlite::explore_sqlite;

pub mod mysql;
pub use mysql::explore_mysql;

pub mod postgres;
pub use postgres::explore_postgres;

pub mod error;
pub use error::{Error, Result};

pub mod utils;
pub use utils::{extract_enums, extract_relationships_meta, extract_tables_meta};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub url: String,
}

pub type TablesHashMap = HashMap<String, TableCreateStatement>;
