use async_graphql::Enum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::hash::Hash;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumIter,
    DeriveActiveEnum,
    Hash,
    Copy,
    Serialize,
    Deserialize,
    Type,
    Enum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "permission")]
pub enum Permission {
    #[sea_orm(string_value = "read")]
    Read,
    #[sea_orm(string_value = "write")]
    Write,
    #[sea_orm(string_value = "admin")]
    Admin,
}

impl Permission {
    pub fn includes(&self, other: Permission) -> bool {
        match (self, other) {
            (Permission::Admin, _)
            | (Permission::Write, Permission::Write)
            | (Permission::Write, Permission::Read)
            | (Permission::Read, Permission::Read) => true,
            (_, _) => false,
        }
    }
}

seaography::impl_custom_type_for_enum!(Permission);
