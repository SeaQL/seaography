use crate::orm::sea_orm_active_enums;
use async_graphql::*;
use sea_orm::entity::prelude::*;
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Enum)]
#[graphql(remote = "sea_orm_active_enums::Rating")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Rating")]
pub enum Rating {
    #[sea_orm(string_value = "G")]
    G,
    #[sea_orm(string_value = "PG")]
    Pg,
    #[sea_orm(string_value = "PG-13")]
    Pg13,
    #[sea_orm(string_value = "R")]
    R,
    #[sea_orm(string_value = "NC-17")]
    Nc17,
}
