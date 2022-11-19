#![recursion_limit = "1024"]
use sea_orm::prelude::*;

pub mod entities;
pub mod query_root;

pub use query_root::QueryRoot;

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}
