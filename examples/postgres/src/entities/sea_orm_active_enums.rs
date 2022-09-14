use sea_orm::entity::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "mpaa_rating")]
pub enum MpaaRating {
    #[sea_orm(string_value = "G")]
    G,
    #[sea_orm(string_value = "NC-17")]
    Nc17,
    #[sea_orm(string_value = "PG")]
    Pg,
    #[sea_orm(string_value = "PG-13")]
    Pg13,
    #[sea_orm(string_value = "R")]
    R,
}
